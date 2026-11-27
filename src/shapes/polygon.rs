// ============================================================================
// Module:       shapes::polygon
// Description:  RoundedPolygon: polygons with rounded and smoothed corners.
//
// Dependencies: egui (emath); super::cubic
// ============================================================================

//! [`RoundedPolygon`], [`CornerRounding`] and [`Feature`], ported from
//! `androidx.graphics.shapes` (`RoundedPolygon.kt`, `Features.kt`,
//! `CornerRounding.kt`, `Shapes.kt`; Apache-2.0).
//!
//! Where the Kotlin original throws `IllegalArgumentException`, these
//! constructors return [`ShapeError`].

use egui::{vec2, Rect, Vec2};

use super::cubic::{convex, direction, rotate90, Cubic, DISTANCE_EPSILON};

/// Why a shape could not be built.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeError {
    /// A polygon needs at least three vertices.
    TooFewVertices,
    /// `per_vertex_rounding` must have one entry per vertex.
    RoundingCountMismatch,
    /// Radii must be positive, and a star's inner radius below its outer one.
    InvalidRadius,
    /// The outline's segments do not join end to end.
    NotContiguous,
    /// The outline has no length, so it cannot be measured for morphing.
    ZeroLength,
    /// Two shapes' outlines could not be matched for morphing.
    Unmatched,
}

impl std::fmt::Display for ShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            ShapeError::TooFewVertices => "polygons must have at least 3 vertices",
            ShapeError::RoundingCountMismatch => {
                "per-vertex rounding must have one entry per vertex"
            }
            ShapeError::InvalidRadius => "invalid radius",
            ShapeError::NotContiguous => "outline segments do not join end to end",
            ShapeError::ZeroLength => "outline has no length",
            ShapeError::Unmatched => "outlines could not be matched",
        };
        f.write_str(text)
    }
}

impl std::error::Error for ShapeError {}

/// How a corner is rounded: a radius, and how much of the edge next to it
/// the curve may use to ease in (`smoothing`, 0..=1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CornerRounding {
    /// Radius of the circular part of the corner.
    pub radius: f32,
    /// 0 is a plain circular arc; 1 lets the curve use as much of the
    /// adjacent edges again for a smooth entry.
    pub smoothing: f32,
}

impl CornerRounding {
    /// A sharp corner.
    pub const UNROUNDED: Self = Self::new(0.0, 0.0);

    /// A rounding of `radius` with `smoothing`.
    pub const fn new(radius: f32, smoothing: f32) -> Self {
        Self { radius, smoothing }
    }

    /// A circular rounding of `radius`.
    pub const fn radius(radius: f32) -> Self {
        Self::new(radius, 0.0)
    }
}

impl Default for CornerRounding {
    fn default() -> Self {
        Self::UNROUNDED
    }
}

/// What a [`Feature`] is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeatureKind {
    /// A straight-ish edge; morphing does not try to keep it.
    Edge,
    /// A corner; morphing maps corners of one shape onto corners of the other.
    Corner {
        /// Convex (bulging out) or concave.
        convex: bool,
    },
}

/// A run of cubics that forms one edge or one corner of a polygon.
#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    /// The segments, end to end.
    pub cubics: Vec<Cubic>,
    /// Edge or corner.
    pub kind: FeatureKind,
}

impl Feature {
    fn transformed(&self, f: &impl Fn(Vec2) -> Vec2) -> Self {
        Self {
            cubics: self.cubics.iter().map(|c| c.transformed(f)).collect(),
            kind: self.kind,
        }
    }

    /// Whether this is a corner.
    pub fn is_corner(&self) -> bool {
        match self.kind {
            FeatureKind::Corner { .. } => true,
            FeatureKind::Edge => false,
        }
    }
}

/// A closed outline made of rounded corners and the edges between them.
#[derive(Clone, Debug, PartialEq)]
pub struct RoundedPolygon {
    features: Vec<Feature>,
    center: Vec2,
    cubics: Vec<Cubic>,
}

impl RoundedPolygon {
    /// An empty polygon: a single point at `center`.
    pub fn empty(center: Vec2) -> Self {
        Self {
            features: Vec::new(),
            center,
            cubics: vec![Cubic::empty(center)],
        }
    }

    /// A polygon from its features, which must join end to end.
    pub fn from_features(features: Vec<Feature>, center: Vec2) -> Result<Self, ShapeError> {
        let cubics = build_cubics(&features, center);
        // The outline must be contiguous.
        let mut prev = cubics.last().copied().unwrap_or(Cubic::empty(center));
        for cubic in &cubics {
            if (cubic.anchor0.x - prev.anchor1.x).abs() > DISTANCE_EPSILON
                || (cubic.anchor0.y - prev.anchor1.y).abs() > DISTANCE_EPSILON
            {
                return Err(ShapeError::NotContiguous);
            }
            prev = *cubic;
        }
        Ok(Self {
            features,
            center,
            cubics,
        })
    }

    /// A polygon through `vertices` (in order), each corner rounded by
    /// `per_vertex_rounding` (one per vertex) or else by `rounding`. The
    /// centre defaults to the vertices' average.
    pub fn from_vertices(
        vertices: &[Vec2],
        rounding: CornerRounding,
        per_vertex_rounding: Option<&[CornerRounding]>,
        center: Option<Vec2>,
    ) -> Result<Self, ShapeError> {
        let n = vertices.len();
        if n < 3 {
            return Err(ShapeError::TooFewVertices);
        }
        if per_vertex_rounding.is_some_and(|r| r.len() != n) {
            return Err(ShapeError::RoundingCountMismatch);
        }
        let vertex = |i: usize| vertices.get(i % n).copied().unwrap_or_default();
        let rounding_at = |i: usize| {
            per_vertex_rounding
                .and_then(|r| r.get(i).copied())
                .unwrap_or(rounding)
        };
        let corners: Vec<RoundedCorner> = (0..n)
            .map(|i| {
                RoundedCorner::new(vertex(i + n - 1), vertex(i), vertex(i + 1), rounding_at(i))
            })
            .collect();
        let corner = |i: usize| corners.get(i % n);

        // For each side (corner i to corner i+1): how much of the round cut
        // and of the smoothing cut fits.
        let cut_adjusts: Vec<(f32, f32)> = (0..n)
            .map(|i| {
                let (Some(a), Some(b)) = (corner(i), corner(i + 1)) else {
                    return (0.0, 0.0);
                };
                let expected_round_cut = a.expected_round_cut + b.expected_round_cut;
                let expected_cut = a.expected_cut() + b.expected_cut();
                let side = (vertex(i) - vertex(i + 1)).length();
                if expected_round_cut > side {
                    (side / expected_round_cut, 0.0)
                } else if expected_cut > side {
                    (
                        1.0,
                        (side - expected_round_cut) / (expected_cut - expected_round_cut),
                    )
                } else {
                    (1.0, 1.0)
                }
            })
            .collect();

        let mut corner_cubics: Vec<Vec<Cubic>> = Vec::with_capacity(n);
        for (i, c) in corners.iter().enumerate() {
            let allowed = |delta: usize| {
                let (round_ratio, cut_ratio) = cut_adjusts
                    .get((i + n - 1 + delta) % n)
                    .copied()
                    .unwrap_or((0.0, 0.0));
                c.expected_round_cut * round_ratio
                    + (c.expected_cut() - c.expected_round_cut) * cut_ratio
            };
            corner_cubics.push(c.cubics(allowed(0), allowed(1)));
        }

        let mut features = Vec::with_capacity(2 * n);
        for i in 0..n {
            let cubics = corner_cubics.get(i).cloned().unwrap_or_default();
            let next = corner_cubics.get((i + 1) % n);
            let is_convex = convex(vertex(i + n - 1), vertex(i), vertex(i + 1));
            let edge_start = cubics.last().map_or(vertex(i), |c| c.anchor1);
            let edge_end = next
                .and_then(|c| c.first())
                .map_or(vertex(i + 1), |c| c.anchor0);
            features.push(Feature {
                cubics,
                kind: FeatureKind::Corner { convex: is_convex },
            });
            features.push(Feature {
                cubics: vec![Cubic::straight_line(edge_start, edge_end)],
                kind: FeatureKind::Edge,
            });
        }
        let center = center
            .unwrap_or_else(|| vertices.iter().fold(Vec2::ZERO, |sum, v| sum + *v) / n as f32);
        Self::from_features(features, center)
    }

    /// A regular polygon of `num_vertices` on a circle of `radius`, with its
    /// first vertex at angle 0 (pointing along +x).
    pub fn regular(
        num_vertices: usize,
        radius: f32,
        center: Vec2,
        rounding: CornerRounding,
        per_vertex_rounding: Option<&[CornerRounding]>,
    ) -> Result<Self, ShapeError> {
        let vertices: Vec<Vec2> = (0..num_vertices)
            .map(|i| {
                let angle = std::f32::consts::PI / num_vertices as f32 * 2.0 * i as f32;
                Vec2::angled(angle) * radius + center
            })
            .collect();
        Self::from_vertices(&vertices, rounding, per_vertex_rounding, Some(center))
    }

    /// A circle of `radius`, approximated by a fully rounded regular polygon
    /// of `num_vertices` (`RoundedPolygon.circle`, default 8).
    pub fn circle(num_vertices: usize, radius: f32, center: Vec2) -> Result<Self, ShapeError> {
        if num_vertices < 3 {
            return Err(ShapeError::TooFewVertices);
        }
        let theta = std::f32::consts::PI / num_vertices as f32;
        Self::regular(
            num_vertices,
            radius / theta.cos(),
            center,
            CornerRounding::radius(radius),
            None,
        )
    }

    /// A `width` by `height` rectangle around `center`.
    pub fn rectangle(
        width: f32,
        height: f32,
        rounding: CornerRounding,
        per_vertex_rounding: Option<&[CornerRounding]>,
        center: Vec2,
    ) -> Result<Self, ShapeError> {
        let (left, top) = (center.x - width / 2.0, center.y - height / 2.0);
        let (right, bottom) = (center.x + width / 2.0, center.y + height / 2.0);
        Self::from_vertices(
            &[
                vec2(right, bottom),
                vec2(left, bottom),
                vec2(left, top),
                vec2(right, top),
            ],
            rounding,
            per_vertex_rounding,
            Some(center),
        )
    }

    /// A star with `num_vertices_per_radius` points on `radius` and as many
    /// on `inner_radius` between them. `inner_rounding`, when set, rounds the
    /// inner vertices differently from the outer ones.
    pub fn star(
        num_vertices_per_radius: usize,
        radius: f32,
        inner_radius: f32,
        rounding: CornerRounding,
        inner_rounding: Option<CornerRounding>,
        center: Vec2,
    ) -> Result<Self, ShapeError> {
        if radius <= 0.0 || inner_radius <= 0.0 || inner_radius >= radius {
            return Err(ShapeError::InvalidRadius);
        }
        let n = num_vertices_per_radius;
        let step = std::f32::consts::PI / n as f32;
        let mut vertices = Vec::with_capacity(2 * n);
        for i in 0..n {
            vertices.push(Vec2::angled(step * 2.0 * i as f32) * radius + center);
            vertices.push(Vec2::angled(step * (2 * i + 1) as f32) * inner_radius + center);
        }
        let per_vertex: Option<Vec<CornerRounding>> =
            inner_rounding.map(|inner| (0..n).flat_map(|_| [rounding, inner]).collect());
        Self::from_vertices(&vertices, rounding, per_vertex.as_deref(), Some(center))
    }

    /// The features (corners and edges), in outline order.
    pub fn features(&self) -> &[Feature] {
        &self.features
    }

    /// The outline as cubics, joined end to end and closed.
    pub fn cubics(&self) -> &[Cubic] {
        &self.cubics
    }

    /// The centre the shape was built around.
    pub fn center(&self) -> Vec2 {
        self.center
    }

    /// Every point (and the centre) mapped through `f`.
    pub fn transformed(&self, f: &impl Fn(Vec2) -> Vec2) -> Result<Self, ShapeError> {
        Self::from_features(
            self.features
                .iter()
                .map(|feat| feat.transformed(f))
                .collect(),
            f(self.center),
        )
    }

    /// Rotated by `degrees` around the origin (y-down, so positive is
    /// clockwise on screen, like Compose's `Matrix.rotateZ`).
    pub fn rotated(&self, degrees: f32) -> Result<Self, ShapeError> {
        let (sin, cos) = degrees.to_radians().sin_cos();
        self.transformed(&|p: Vec2| vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos))
    }

    /// Scaled by `sx`, `sy` around the origin.
    pub fn scaled(&self, sx: f32, sy: f32) -> Result<Self, ShapeError> {
        self.transformed(&|p: Vec2| vec2(p.x * sx, p.y * sy))
    }

    /// Moved and scaled to fit the unit square, centred in it.
    pub fn normalized(&self) -> Result<Self, ShapeError> {
        let bounds = self.bounds(true);
        let side = bounds.width().max(bounds.height());
        if side <= 0.0 {
            return Err(ShapeError::ZeroLength);
        }
        let offset = vec2(
            (side - bounds.width()) / 2.0 - bounds.min.x,
            (side - bounds.height()) / 2.0 - bounds.min.y,
        );
        self.transformed(&|p: Vec2| (p + offset) / side)
    }

    /// The bounding box; `approximate` uses the control points.
    pub fn bounds(&self, approximate: bool) -> Rect {
        self.cubics
            .iter()
            .map(|c| c.bounds(approximate))
            .reduce(|a, b| a.union(b))
            .unwrap_or(Rect::from_min_max(
                self.center.to_pos2(),
                self.center.to_pos2(),
            ))
    }

    /// The square around the centre that holds the shape at any rotation.
    pub fn max_bounds(&self) -> Rect {
        let max_dist_sq = self
            .cubics
            .iter()
            .map(|c| {
                let anchor = (c.anchor0 - self.center).length_sq();
                let middle = (c.point_on_curve(0.5) - self.center).length_sq();
                anchor.max(middle)
            })
            .fold(0.0_f32, f32::max);
        Rect::from_center_size(self.center.to_pos2(), Vec2::splat(2.0 * max_dist_sq.sqrt()))
    }
}

/// `RoundedPolygon.cubics`: the features' cubics, with zero-length ones
/// dropped, the first feature's middle cubic split so the outline starts
/// mid-corner, and the last anchor snapped onto the first.
fn build_cubics(features: &[Feature], center: Vec2) -> Vec<Cubic> {
    let mut split_start: Option<Vec<Cubic>> = None;
    let mut split_end: Option<Vec<Cubic>> = None;
    if let Some([c0, c1, c2]) = features.first().map(|f| f.cubics.as_slice()) {
        let (start, end) = c1.split(0.5);
        split_start = Some(vec![*c0, start]);
        split_end = Some(vec![end, *c2]);
    }
    let mut out = Vec::new();
    let mut first: Option<Cubic> = None;
    let mut last: Option<Cubic> = None;
    let mut push_all = |cubics: &[Cubic]| {
        for cubic in cubics {
            if !cubic.zero_length() {
                if let Some(l) = last {
                    out.push(l);
                }
                last = Some(*cubic);
                if first.is_none() {
                    first = Some(*cubic);
                }
            } else if let Some(l) = last.as_mut() {
                // Keep the latest anchor so several dropped curves in a row
                // cannot open a gap.
                l.anchor1 = cubic.anchor1;
            }
        }
    };
    for (i, feature) in features.iter().enumerate() {
        match (i, &split_end) {
            (0, Some(end)) => push_all(end),
            _ => push_all(&feature.cubics),
        }
    }
    if let Some(start) = &split_start {
        push_all(start);
    }
    match (last, first) {
        (Some(l), Some(f)) => out.push(Cubic::new(l.anchor0, l.control0, l.control1, f.anchor0)),
        _ => out.push(Cubic::empty(center)),
    }
    out
}

/// One corner being rounded (`RoundedCorner` in `RoundedPolygon.kt`).
struct RoundedCorner {
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    d1: Vec2,
    d2: Vec2,
    corner_radius: f32,
    smoothing: f32,
    expected_round_cut: f32,
}

impl RoundedCorner {
    fn new(p0: Vec2, p1: Vec2, p2: Vec2, rounding: CornerRounding) -> Self {
        let v01 = p0 - p1;
        let v21 = p2 - p1;
        let (d01, d21) = (v01.length(), v21.length());
        if d01 > 0.0 && d21 > 0.0 {
            let d1 = v01 / d01;
            let d2 = v21 / d21;
            let cos_angle = d1.dot(d2);
            let sin_angle = (1.0 - cos_angle * cos_angle).max(0.0).sqrt();
            let expected_round_cut = if sin_angle > 1e-3 {
                rounding.radius * (cos_angle + 1.0) / sin_angle
            } else {
                0.0
            };
            Self {
                p0,
                p1,
                p2,
                d1,
                d2,
                corner_radius: rounding.radius,
                smoothing: rounding.smoothing,
                expected_round_cut,
            }
        } else {
            Self {
                p0,
                p1,
                p2,
                d1: Vec2::ZERO,
                d2: Vec2::ZERO,
                corner_radius: 0.0,
                smoothing: 0.0,
                expected_round_cut: 0.0,
            }
        }
    }

    fn expected_cut(&self) -> f32 {
        (1.0 + self.smoothing) * self.expected_round_cut
    }

    fn cubics(&self, allowed_cut0: f32, allowed_cut1: f32) -> Vec<Cubic> {
        let allowed_cut = allowed_cut0.min(allowed_cut1);
        if self.expected_round_cut < DISTANCE_EPSILON
            || allowed_cut < DISTANCE_EPSILON
            || self.corner_radius < DISTANCE_EPSILON
        {
            return vec![Cubic::straight_line(self.p1, self.p1)];
        }
        let actual_round_cut = allowed_cut.min(self.expected_round_cut);
        let smoothing0 = self.actual_smoothing(allowed_cut0);
        let smoothing1 = self.actual_smoothing(allowed_cut1);
        let actual_r = self.corner_radius * actual_round_cut / self.expected_round_cut;
        let center_distance = (actual_r * actual_r + actual_round_cut * actual_round_cut).sqrt();
        let Some(bisector) = direction((self.d1 + self.d2) / 2.0) else {
            return vec![Cubic::straight_line(self.p1, self.p1)];
        };
        let center = self.p1 + bisector * center_distance;
        let circle0 = self.p1 + self.d1 * actual_round_cut;
        let circle2 = self.p1 + self.d2 * actual_round_cut;
        let flank = FlankInput {
            actual_round_cut,
            corner: self.p1,
            circle_center: center,
            actual_r,
        };
        let flanking0 = flank.curve(smoothing0, self.p0, circle0, circle2);
        let flanking2 = flank.curve(smoothing1, self.p2, circle2, circle0).reverse();
        vec![
            flanking0,
            Cubic::circular_arc(center, flanking0.anchor1, flanking2.anchor0),
            flanking2,
        ]
    }

    fn actual_smoothing(&self, allowed_cut: f32) -> f32 {
        if allowed_cut > self.expected_cut() {
            self.smoothing
        } else if allowed_cut > self.expected_round_cut {
            self.smoothing * (allowed_cut - self.expected_round_cut)
                / (self.expected_cut() - self.expected_round_cut)
        } else {
            0.0
        }
    }
}

/// The parts of `computeFlankingCurve`'s arguments shared by both flanks.
struct FlankInput {
    actual_round_cut: f32,
    corner: Vec2,
    circle_center: Vec2,
    actual_r: f32,
}

impl FlankInput {
    fn curve(
        &self,
        smoothing: f32,
        side_start: Vec2,
        circle_intersection: Vec2,
        other_circle_intersection: Vec2,
    ) -> Cubic {
        let side_direction = direction(side_start - self.corner).unwrap_or_default();
        let curve_start = self.corner + side_direction * self.actual_round_cut * (1.0 + smoothing);
        let mid = (circle_intersection + other_circle_intersection) / 2.0;
        let p = circle_intersection + (mid - circle_intersection) * smoothing;
        let curve_end = self.circle_center
            + direction(p - self.circle_center).unwrap_or_default() * self.actual_r;
        let circle_tangent = rotate90(curve_end - self.circle_center);
        let anchor_end = line_intersection(side_start, side_direction, curve_end, circle_tangent)
            .unwrap_or(circle_intersection);
        let anchor_start = (curve_start + anchor_end * 2.0) / 3.0;
        Cubic::new(curve_start, anchor_start, anchor_end, curve_end)
    }
}

fn line_intersection(p0: Vec2, d0: Vec2, p1: Vec2, d1: Vec2) -> Option<Vec2> {
    let rotated_d1 = rotate90(d1);
    let den = d0.dot(rotated_d1);
    if den.abs() < DISTANCE_EPSILON {
        return None;
    }
    let num = (p1 - p0).dot(rotated_d1);
    if den.abs() < DISTANCE_EPSILON * num.abs() {
        return None;
    }
    Some(p0 + d0 * (num / den))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_few_vertices_is_an_error() {
        let r = RoundedPolygon::from_vertices(
            &[Vec2::ZERO, Vec2::X],
            CornerRounding::UNROUNDED,
            None,
            None,
        );
        assert_eq!(r.err(), Some(ShapeError::TooFewVertices), "two vertices");
    }

    #[test]
    fn unrounded_square_has_its_corners() -> Result<(), ShapeError> {
        let square =
            RoundedPolygon::rectangle(2.0, 2.0, CornerRounding::UNROUNDED, None, Vec2::ZERO)?;
        let b = square.bounds(false);
        assert!(
            (b.min.x + 1.0).abs() < 1e-5 && (b.max.y - 1.0).abs() < 1e-5,
            "{b:?}"
        );
        assert_eq!(square.features().len(), 8, "four corners, four edges");
        Ok(())
    }

    #[test]
    fn circle_points_lie_on_the_radius() -> Result<(), ShapeError> {
        let circle = RoundedPolygon::circle(8, 1.0, Vec2::ZERO)?;
        for cubic in circle.cubics() {
            for i in 0..=4 {
                let r = cubic.point_on_curve(i as f32 / 4.0).length();
                assert!((r - 1.0).abs() < 2e-3, "radius {r}");
            }
        }
        Ok(())
    }

    #[test]
    fn normalized_fits_the_unit_square() -> Result<(), ShapeError> {
        let star = RoundedPolygon::star(
            5,
            3.0,
            1.0,
            CornerRounding::radius(0.2),
            None,
            vec2(4.0, -2.0),
        )?
        .normalized()?;
        let b = star.bounds(true);
        assert!(
            b.min.x >= -1e-5 && b.min.y >= -1e-5 && b.max.x <= 1.0 + 1e-5 && b.max.y <= 1.0 + 1e-5,
            "{b:?}"
        );
        assert!(
            (b.width().max(b.height()) - 1.0).abs() < 1e-4,
            "fills one side"
        );
        Ok(())
    }
}
