// ============================================================================
// Module:       shapes::morph
// Description:  Morph: a smooth transition between two RoundedPolygons.
//
// Dependencies: egui (emath); super::{cubic, polygon}
// ============================================================================

//! [`Morph`], ported from `androidx.graphics.shapes` (`Morph.kt`,
//! `PolygonMeasure.kt`, `FeatureMapping.kt`, `FloatMapping.kt`;
//! Apache-2.0).
//!
//! Both outlines are measured by length, their corners are paired up by
//! proximity, and the outlines are cut so that every cubic of one shape has
//! a partner in the other. Interpolating the pairs gives the in-between
//! shapes.

use egui::Rect;

use super::cubic::{positive_modulo, Cubic, ANGLE_EPSILON, DISTANCE_EPSILON};
use super::polygon::{Feature, FeatureKind, RoundedPolygon, ShapeError};

/// A transition between two shapes; [`Morph::as_cubics`] gives the outline
/// at any progress.
#[derive(Clone, Debug, PartialEq)]
pub struct Morph {
    start: RoundedPolygon,
    end: RoundedPolygon,
    pairs: Vec<(Cubic, Cubic)>,
}

impl Morph {
    /// Matches `start` against `end`.
    pub fn new(start: RoundedPolygon, end: RoundedPolygon) -> Result<Self, ShapeError> {
        let pairs = match_polygons(&start, &end)?;
        Ok(Self { start, end, pairs })
    }

    /// The shape it starts from.
    pub fn start(&self) -> &RoundedPolygon {
        &self.start
    }

    /// The shape it ends at.
    pub fn end(&self) -> &RoundedPolygon {
        &self.end
    }

    /// The outline at `progress` (0 is `start`, 1 is `end`; values outside
    /// extrapolate, which springs use for overshoot), closed end to end.
    pub fn as_cubics(&self, progress: f32) -> Vec<Cubic> {
        let mut out: Vec<Cubic> = self
            .pairs
            .iter()
            .map(|(a, b)| Cubic::lerp(a, b, progress))
            .collect();
        let first = out.first().map(|c| c.anchor0);
        if let (Some(last), Some(first)) = (out.last_mut(), first) {
            last.anchor1 = first;
        }
        out
    }

    /// The bounds covering both shapes.
    pub fn bounds(&self, approximate: bool) -> Rect {
        self.start
            .bounds(approximate)
            .union(self.end.bounds(approximate))
    }

    /// The square holding both shapes at any rotation.
    pub fn max_bounds(&self) -> Rect {
        self.start.max_bounds().union(self.end.max_bounds())
    }
}

// ---------------------------------------------------------------------------
// Measuring (PolygonMeasure.kt)

/// A cubic and the stretch of outline progress (0..=1) it covers.
#[derive(Clone, Copy, Debug)]
struct MeasuredCubic {
    cubic: Cubic,
    start: f32,
    end: f32,
    size: f32,
}

impl MeasuredCubic {
    fn new(cubic: Cubic, start: f32, end: f32) -> Self {
        Self {
            cubic,
            start,
            end: end.max(start),
            size: measure_cubic(&cubic),
        }
    }

    fn cut_at_progress(&self, cut: f32) -> (MeasuredCubic, MeasuredCubic) {
        let bounded = cut.clamp(self.start, self.end);
        let span = self.end - self.start;
        let relative = if span > 0.0 {
            (bounded - self.start) / span
        } else {
            0.0
        };
        let t = find_cubic_cut_point(&self.cubic, relative * self.size).clamp(0.0, 1.0);
        let (c1, c2) = self.cubic.split(t);
        (
            MeasuredCubic::new(c1, self.start, bounded),
            MeasuredCubic::new(c2, bounded, self.end),
        )
    }
}

/// A corner and where along the outline (0..1) it sits.
#[derive(Clone, Copy, Debug)]
struct ProgressableFeature {
    progress: f32,
    convex: bool,
    representative: egui::Vec2,
}

#[derive(Clone, Debug)]
struct MeasuredPolygon {
    cubics: Vec<MeasuredCubic>,
    features: Vec<ProgressableFeature>,
}

impl MeasuredPolygon {
    fn new(
        features: Vec<ProgressableFeature>,
        cubics: &[Cubic],
        outline_progress: &[f32],
    ) -> Result<Self, ShapeError> {
        let mut measured = Vec::with_capacity(cubics.len());
        let mut start = 0.0;
        for (cubic, window) in cubics.iter().zip(outline_progress.windows(2)) {
            let [from, to] = window else {
                continue;
            };
            if to - from > DISTANCE_EPSILON {
                measured.push(MeasuredCubic::new(*cubic, start, *to));
                start = *to;
            }
        }
        let last = measured.last_mut().ok_or(ShapeError::ZeroLength)?;
        last.end = 1.0;
        Ok(Self {
            cubics: measured,
            features,
        })
    }

    fn measure(polygon: &RoundedPolygon) -> Result<Self, ShapeError> {
        let mut cubics = Vec::new();
        let mut corner_at: Vec<(&Feature, usize)> = Vec::new();
        for feature in polygon.features() {
            for (i, cubic) in feature.cubics.iter().enumerate() {
                if feature.is_corner() && i == feature.cubics.len() / 2 {
                    corner_at.push((feature, cubics.len()));
                }
                cubics.push(*cubic);
            }
        }
        let mut measures = Vec::with_capacity(cubics.len() + 1);
        let mut total = 0.0_f32;
        measures.push(0.0);
        for cubic in &cubics {
            total += measure_cubic(cubic);
            measures.push(total);
        }
        if total <= 0.0 || !total.is_finite() {
            return Err(ShapeError::ZeroLength);
        }
        let outline: Vec<f32> = measures.iter().map(|m| m / total).collect();
        let features = corner_at
            .iter()
            .filter_map(|(feature, ix)| {
                let from = outline.get(*ix)?;
                let to = outline.get(ix + 1)?;
                let convex = match feature.kind {
                    FeatureKind::Corner { convex } => convex,
                    FeatureKind::Edge => true,
                };
                Some(ProgressableFeature {
                    progress: positive_modulo((from + to) / 2.0, 1.0),
                    convex,
                    representative: representative_point(feature),
                })
            })
            .collect();
        Self::new(features, &cubics, &outline)
    }

    /// The same outline starting at `cut` instead of 0.
    fn cut_and_shift(&self, cut: f32) -> Result<Self, ShapeError> {
        if cut < DISTANCE_EPSILON {
            return Ok(self.clone());
        }
        let n = self.cubics.len();
        let target_index = self
            .cubics
            .iter()
            .position(|c| (c.start..=c.end).contains(&cut))
            .ok_or(ShapeError::Unmatched)?;
        let target = self.cubics.get(target_index).ok_or(ShapeError::Unmatched)?;
        let (b1, b2) = target.cut_at_progress(cut);
        let mut cubics = vec![b2.cubic];
        for i in 1..n {
            if let Some(c) = self.cubics.get((i + target_index) % n) {
                cubics.push(c.cubic);
            }
        }
        cubics.push(b1.cubic);
        let mut outline = Vec::with_capacity(n + 2);
        outline.push(0.0);
        for index in 1..=n {
            let c = self
                .cubics
                .get((target_index + index - 1) % n)
                .ok_or(ShapeError::Unmatched)?;
            outline.push(positive_modulo(c.end - cut, 1.0));
        }
        outline.push(1.0);
        let features = self
            .features
            .iter()
            .map(|f| ProgressableFeature {
                progress: positive_modulo(f.progress - cut, 1.0),
                ..*f
            })
            .collect();
        Self::new(features, &cubics, &outline)
    }
}

/// `LengthMeasurer`: the cubic's length, approximated by three chords.
fn measure_cubic(cubic: &Cubic) -> f32 {
    closest_progress_to(cubic, f32::INFINITY).1
}

/// `LengthMeasurer.findCubicCutPoint`: the parameter at length `m`.
fn find_cubic_cut_point(cubic: &Cubic, m: f32) -> f32 {
    closest_progress_to(cubic, m).0
}

fn closest_progress_to(cubic: &Cubic, threshold: f32) -> (f32, f32) {
    const SEGMENTS: usize = 3;
    let mut total = 0.0;
    let mut remainder = threshold;
    let mut prev = cubic.anchor0;
    for i in 1..=SEGMENTS {
        let progress = i as f32 / SEGMENTS as f32;
        let point = cubic.point_on_curve(progress);
        let segment = (point - prev).length();
        if segment >= remainder {
            return (
                progress - (1.0 - remainder / segment) / SEGMENTS as f32,
                threshold,
            );
        }
        remainder -= segment;
        total += segment;
        prev = point;
    }
    (1.0, total)
}

fn representative_point(feature: &Feature) -> egui::Vec2 {
    match (feature.cubics.first(), feature.cubics.last()) {
        (Some(first), Some(last)) => (first.anchor0 + last.anchor1) / 2.0,
        _ => egui::Vec2::ZERO,
    }
}

// ---------------------------------------------------------------------------
// Mapping progress between the two outlines (FeatureMapping.kt,
// FloatMapping.kt)

fn progress_in_range(progress: f32, from: f32, to: f32) -> bool {
    if to >= from {
        (from..=to).contains(&progress)
    } else {
        progress >= from || progress <= to
    }
}

fn progress_distance(p1: f32, p2: f32) -> f32 {
    let d = (p1 - p2).abs();
    d.min(1.0 - d)
}

/// A piecewise-linear, wrapping map of outline progress between shapes.
#[derive(Clone, Debug)]
struct DoubleMapper {
    source: Vec<f32>,
    target: Vec<f32>,
}

impl DoubleMapper {
    fn new(mappings: &[(f32, f32)]) -> Result<Self, ShapeError> {
        let source: Vec<f32> = mappings.iter().map(|m| m.0).collect();
        let target: Vec<f32> = mappings.iter().map(|m| m.1).collect();
        validate_progress(&source)?;
        validate_progress(&target)?;
        Ok(Self { source, target })
    }

    fn map(&self, x: f32) -> f32 {
        linear_map(&self.source, &self.target, x)
    }

    fn map_back(&self, x: f32) -> f32 {
        linear_map(&self.target, &self.source, x)
    }
}

fn validate_progress(p: &[f32]) -> Result<(), ShapeError> {
    let Some(mut prev) = p.last().copied() else {
        return Err(ShapeError::Unmatched);
    };
    let mut wraps = 0;
    for &curr in p {
        if !(0.0..1.0).contains(&curr) || progress_distance(curr, prev) <= DISTANCE_EPSILON {
            return Err(ShapeError::Unmatched);
        }
        if curr < prev {
            wraps += 1;
            if wraps > 1 {
                return Err(ShapeError::Unmatched);
            }
        }
        prev = curr;
    }
    Ok(())
}

fn linear_map(xs: &[f32], ys: &[f32], x: f32) -> f32 {
    let n = xs.len();
    if n == 0 {
        return x;
    }
    let at = |v: &[f32], i: usize| v.get(i % n).copied().unwrap_or(0.0);
    let start = (0..n)
        .find(|&i| progress_in_range(x, at(xs, i), at(xs, i + 1)))
        .unwrap_or(0);
    let end = start + 1;
    let size_x = positive_modulo(at(xs, end) - at(xs, start), 1.0);
    let size_y = positive_modulo(at(ys, end) - at(ys, start), 1.0);
    let position = if size_x < 0.001 {
        0.5
    } else {
        positive_modulo(x - at(xs, start), 1.0) / size_x
    };
    positive_modulo(at(ys, start) + size_y * position, 1.0)
}

/// `featureMapper`: pairs corners of the two shapes, closest first, keeping
/// the pairing order-preserving around the outline.
fn feature_mapper(
    features1: &[ProgressableFeature],
    features2: &[ProgressableFeature],
) -> Result<DoubleMapper, ShapeError> {
    let mut candidates: Vec<(f32, usize, usize)> = Vec::new();
    for (i1, f1) in features1.iter().enumerate() {
        for (i2, f2) in features2.iter().enumerate() {
            if f1.convex == f2.convex {
                let d = (f1.representative - f2.representative).length_sq();
                candidates.push((d, i1, i2));
            }
        }
    }
    candidates.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mapping: Vec<(f32, f32)> = match candidates.as_slice() {
        [] => vec![(0.0, 0.0), (0.5, 0.5)],
        [(_, i1, i2)] => {
            let p1 = features1.get(*i1).map_or(0.0, |f| f.progress);
            let p2 = features2.get(*i2).map_or(0.0, |f| f.progress);
            vec![(p1, p2), ((p1 + 0.5) % 1.0, (p2 + 0.5) % 1.0)]
        }
        _ => {
            let mut helper = MappingHelper::default();
            for (_, i1, i2) in &candidates {
                if let (Some(f1), Some(f2)) = (features1.get(*i1), features2.get(*i2)) {
                    helper.add(*i1, f1.progress, *i2, f2.progress);
                }
            }
            helper.mapping
        }
    };
    DoubleMapper::new(&mapping)
}

#[derive(Default)]
struct MappingHelper {
    mapping: Vec<(f32, f32)>,
    used1: Vec<usize>,
    used2: Vec<usize>,
}

impl MappingHelper {
    fn add(&mut self, i1: usize, p1: f32, i2: usize, p2: f32) {
        if self.used1.contains(&i1) || self.used2.contains(&i2) {
            return;
        }
        let insertion = match self.mapping.binary_search_by(|m| m.0.total_cmp(&p1)) {
            // Two corners at the same progress: keep the first.
            Ok(_) => return,
            Err(insertion) => insertion,
        };
        let n = self.mapping.len();
        if n >= 1 {
            let before = self.mapping.get((insertion + n - 1) % n).copied();
            let after = self.mapping.get(insertion % n).copied();
            if let (Some((before1, before2)), Some((after1, after2))) = (before, after) {
                if progress_distance(p1, before1) < DISTANCE_EPSILON
                    || progress_distance(p1, after1) < DISTANCE_EPSILON
                    || progress_distance(p2, before2) < DISTANCE_EPSILON
                    || progress_distance(p2, after2) < DISTANCE_EPSILON
                {
                    return;
                }
                if n > 1 && !progress_in_range(p2, before2, after2) {
                    return;
                }
            }
        }
        self.mapping.insert(insertion, (p1, p2));
        self.used1.push(i1);
        self.used2.push(i2);
    }
}

// ---------------------------------------------------------------------------
// Matching (Morph.match)

fn match_polygons(
    p1: &RoundedPolygon,
    p2: &RoundedPolygon,
) -> Result<Vec<(Cubic, Cubic)>, ShapeError> {
    let measured1 = MeasuredPolygon::measure(p1)?;
    let measured2 = MeasuredPolygon::measure(p2)?;
    let mapper = feature_mapper(&measured1.features, &measured2.features)?;
    let cut_point = mapper.map(0.0);
    let bs1 = measured1.cubics;
    let bs2 = measured2.cut_and_shift(cut_point)?.cubics;

    let mut ret = Vec::new();
    let (mut i1, mut i2) = (0, 0);
    let mut b1 = bs1.get(i1).copied();
    i1 += 1;
    let mut b2 = bs2.get(i2).copied();
    i2 += 1;
    while let (Some(c1), Some(c2)) = (b1, b2) {
        let b1a = if i1 == bs1.len() { 1.0 } else { c1.end };
        let b2a = if i2 == bs2.len() {
            1.0
        } else {
            mapper.map_back(positive_modulo(c2.end + cut_point, 1.0))
        };
        let minb = b1a.min(b2a);
        let (seg1, next1) = if b1a > minb + ANGLE_EPSILON {
            let (a, b) = c1.cut_at_progress(minb);
            (a, Some(b))
        } else {
            let next = bs1.get(i1).copied();
            i1 += 1;
            (c1, next)
        };
        let (seg2, next2) = if b2a > minb + ANGLE_EPSILON {
            let (a, b) = c2.cut_at_progress(positive_modulo(mapper.map(minb) - cut_point, 1.0));
            (a, Some(b))
        } else {
            let next = bs2.get(i2).copied();
            i2 += 1;
            (c2, next)
        };
        ret.push((seg1.cubic, seg2.cubic));
        b1 = next1;
        b2 = next2;
        if ret.len() > 4 * (bs1.len() + bs2.len()) + 8 {
            // Cannot happen for valid input; refuse rather than loop.
            return Err(ShapeError::Unmatched);
        }
    }
    if b1.is_some() || b2.is_some() {
        return Err(ShapeError::Unmatched);
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::super::polygon::CornerRounding;
    use super::*;
    use egui::Vec2;

    #[test]
    fn morph_ends_match_the_shapes() -> Result<(), ShapeError> {
        let square = RoundedPolygon::rectangle(
            1.0,
            1.0,
            CornerRounding::radius(0.1),
            None,
            Vec2::splat(0.5),
        )?;
        let star = RoundedPolygon::star(
            5,
            0.5,
            0.25,
            CornerRounding::radius(0.05),
            None,
            Vec2::splat(0.5),
        )?;
        let morph = Morph::new(square.clone(), star.clone())?;
        let close = |a: Rect, b: Rect| (a.min - b.min).length() + (a.max - b.max).length() < 1e-3;
        let bounds = |cubics: &[Cubic]| {
            cubics
                .iter()
                .map(|c| c.bounds(false))
                .reduce(|a, b| a.union(b))
                .unwrap_or(Rect::NOTHING)
        };
        assert!(
            close(bounds(&morph.as_cubics(0.0)), square.bounds(false)),
            "progress 0 is the square"
        );
        assert!(
            close(bounds(&morph.as_cubics(1.0)), star.bounds(false)),
            "progress 1 is the star"
        );
        let mid = morph.as_cubics(0.5);
        let joined = mid.windows(2).all(|w| match w {
            [a, b] => (a.anchor1 - b.anchor0).length() < 1e-3,
            _ => true,
        });
        assert!(joined, "the in-between outline is contiguous");
        Ok(())
    }
}
