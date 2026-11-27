// ============================================================================
// Module:       shapes
// Description:  Rounded polygons, morphing and the Material 3 shape library.
//
// Dependencies: egui (emath, epaint)
// ============================================================================

//! The Material 3 Expressive shape library: [`RoundedPolygon`], [`Morph`]
//! and the 35 named [`MaterialShape`]s.
//!
//! The geometry is a port of `androidx.graphics.shapes` (`graphics-shapes`
//! on `androidx-main`) and the shape table of Compose's `MaterialShapes.kt`,
//! both Copyright The Android Open Source Project, licensed under the
//! Apache License 2.0. The port keeps the algorithms and constants; Kotlin
//! exceptions become [`ShapeError`].
//!
//! Shapes live in their own unit space. [`fill`] paints an outline onto an
//! egui [`egui::Painter`] through a mapping into screen space:
//!
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::shapes::{fill, MaterialShape};
//! let rect = egui::Rect::from_min_size(ui.min_rect().min, egui::vec2(48.0, 48.0));
//! let heart = MaterialShape::Heart.polygon();
//! fill(
//!     ui.painter(),
//!     heart.cubics(),
//!     heart.center(),
//!     |p| rect.min + p * rect.width(),
//!     egui::Color32::RED,
//! );
//! # });
//! ```

mod cubic;
mod material;
mod morph;
mod polygon;

pub use cubic::Cubic;
pub use material::MaterialShape;
pub use morph::Morph;
pub use polygon::{CornerRounding, Feature, FeatureKind, RoundedPolygon, ShapeError};

use egui::epaint::{Mesh, PathShape, PathStroke, Vertex, WHITE_UV};
use egui::{Color32, Painter, Pos2, Vec2};

/// Line segments per cubic when flattening an outline for painting.
const SEGMENTS_PER_CUBIC: usize = 8;

/// The outline as a closed polyline: each cubic flattened into
/// `segments_per_cubic` pieces. The first point is not repeated at the end.
pub fn outline_points(cubics: &[Cubic], segments_per_cubic: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(cubics.len() * segments_per_cubic.max(1));
    if let Some(first) = cubics.first() {
        points.push(first.anchor0);
    }
    for cubic in cubics {
        cubic.flatten_into(segments_per_cubic, &mut points);
    }
    // The last point closes onto the first.
    points.pop();
    points
}

fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

/// Whether every point of the closed polyline `points` is visible from
/// `center` (the polygon is star-shaped around it), so a triangle fan from
/// `center` covers it exactly.
pub fn is_star_shaped(points: &[Vec2], center: Vec2) -> bool {
    let mut positive = false;
    let mut negative = false;
    let n = points.len();
    for (i, p) in points.iter().enumerate() {
        let Some(q) = points.get((i + 1) % n) else {
            continue;
        };
        let c = cross(*p - center, *q - center);
        // Tolerate collinear (zero-area) wedges from flattening.
        let tolerance = 1e-6 * ((*p - center).length() * (*q - center).length()).max(1e-12);
        if c > tolerance {
            positive = true;
        } else if c < -tolerance {
            negative = true;
        }
    }
    !(positive && negative)
}

/// Twice the signed area of the closed polyline (shoelace formula).
pub(crate) fn signed_area2(points: &[Vec2]) -> f32 {
    let n = points.len();
    points
        .iter()
        .enumerate()
        .map(|(i, p)| points.get((i + 1) % n).map_or(0.0, |q| cross(*p, *q)))
        .sum()
}

/// Triangles covering the closed polyline `points`, as index triples into
/// `points`, with index `points.len()` standing for `center`. A fan from
/// `center` when the polygon is star-shaped around it, ear clipping when not.
pub fn triangulate(points: &[Vec2], center: Vec2) -> Vec<[u32; 3]> {
    let n = points.len();
    if n < 3 {
        return Vec::new();
    }
    if is_star_shaped(points, center) {
        let c = n as u32;
        return (0..n as u32).map(|i| [c, i, (i + 1) % n as u32]).collect();
    }
    ear_clip(points)
}

fn ear_clip(points: &[Vec2]) -> Vec<[u32; 3]> {
    let mut remaining: Vec<usize> = (0..points.len()).collect();
    if signed_area2(points) < 0.0 {
        remaining.reverse();
    }
    let at = |i: usize| points.get(i).copied().unwrap_or_default();
    let mut triangles = Vec::with_capacity(points.len().saturating_sub(2));
    let mut guard = 0;
    while remaining.len() > 3 && guard < remaining.len() {
        let m = remaining.len();
        let mut clipped = false;
        for k in 0..m {
            let (Some(&ia), Some(&ib), Some(&ic)) = (
                remaining.get((k + m - 1) % m),
                remaining.get(k),
                remaining.get((k + 1) % m),
            ) else {
                continue;
            };
            let (a, b, c) = (at(ia), at(ib), at(ic));
            let turn = cross(b - a, c - b);
            if turn < 0.0 {
                continue; // reflex
            }
            let contains_other = remaining.iter().any(|&j| {
                if j == ia || j == ib || j == ic {
                    return false;
                }
                let p = at(j);
                cross(b - a, p - a) > 0.0 && cross(c - b, p - b) > 0.0 && cross(a - c, p - c) > 0.0
            });
            if contains_other {
                continue;
            }
            if turn > 0.0 {
                triangles.push([ia as u32, ib as u32, ic as u32]);
            }
            remaining.remove(k);
            clipped = true;
            break;
        }
        guard = if clipped { 0 } else { guard + 1 };
    }
    if let [a, b, c] = remaining.as_slice() {
        triangles.push([*a as u32, *b as u32, *c as u32]);
    }
    triangles
}

/// Paints the outline `cubics` filled with `color`, mapping shape space to
/// screen space with `to_screen`. `center` must be inside the shape (a
/// polygon's [`RoundedPolygon::center`]); it seeds the triangulation.
///
/// The fill is a mesh, edged with a hairline of the same colour so the
/// outline is anti-aliased like egui's own shapes.
pub fn fill(
    painter: &Painter,
    cubics: &[Cubic],
    center: Vec2,
    to_screen: impl Fn(Vec2) -> Pos2,
    color: Color32,
) {
    let points = outline_points(cubics, SEGMENTS_PER_CUBIC);
    let triangles = triangulate(&points, center);
    if triangles.is_empty() {
        return;
    }
    let mut screen: Vec<Pos2> = points.iter().map(|p| to_screen(*p)).collect();
    let mut mesh = Mesh::default();
    for pos in screen
        .iter()
        .copied()
        .chain(std::iter::once(to_screen(center)))
    {
        mesh.vertices.push(Vertex {
            pos,
            uv: WHITE_UV,
            color,
        });
    }
    for [a, b, c] in triangles {
        mesh.add_triangle(a, b, c);
    }
    painter.add(mesh);
    screen.shrink_to_fit();
    painter.add(PathShape {
        points: screen,
        closed: true,
        fill: Color32::TRANSPARENT,
        stroke: PathStroke::new(1.0, color),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn area_of(points: &[Vec2], center: Vec2, triangles: &[[u32; 3]]) -> f32 {
        let at = |i: u32| points.get(i as usize).copied().unwrap_or(center);
        triangles
            .iter()
            .map(|[a, b, c]| cross(at(*b) - at(*a), at(*c) - at(*a)).abs() / 2.0)
            .sum()
    }

    #[test]
    fn every_material_shape_builds() {
        for shape in MaterialShape::ALL {
            let built = shape.build().and_then(|p| p.normalized());
            assert!(built.is_ok(), "{} failed: {built:?}", shape.name());
        }
        assert_eq!(MaterialShape::ALL.len(), 35, "the 35 named shapes");
    }

    #[test]
    fn triangulation_covers_exactly_the_shape() {
        for shape in MaterialShape::ALL {
            let polygon = shape.polygon();
            let points = outline_points(polygon.cubics(), SEGMENTS_PER_CUBIC);
            let triangles = triangulate(&points, polygon.center());
            let expected = signed_area2(&points).abs() / 2.0;
            let covered = area_of(&points, polygon.center(), &triangles);
            assert!(
                (covered - expected).abs() < 1e-3 * expected.max(1e-3),
                "{}: triangles cover {covered}, shape is {expected}",
                shape.name()
            );
        }
    }

    #[test]
    fn shapes_are_star_shaped_around_their_centre() {
        // PixelTriangle's staircase risers face away from its centre, so a
        // fan would overdraw; `triangulate` ear-clips it instead (covered by
        // `triangulation_covers_exactly_the_shape`).
        const NOT_STAR_SHAPED: [MaterialShape; 1] = [MaterialShape::PixelTriangle];
        for shape in MaterialShape::ALL {
            let polygon = shape.polygon();
            let points = outline_points(polygon.cubics(), SEGMENTS_PER_CUBIC);
            assert_eq!(
                is_star_shaped(&points, polygon.center()),
                !NOT_STAR_SHAPED.contains(&shape),
                "{}: star-shaped around its centre",
                shape.name()
            );
        }
    }

    #[test]
    fn ear_clipping_handles_a_non_star_polygon() {
        // A U shape: not star-shaped around its centroid.
        let u = [
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(3.0, 3.0),
            Vec2::new(2.0, 3.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 3.0),
            Vec2::new(0.0, 3.0),
        ];
        let center = Vec2::new(1.5, 2.5);
        assert!(!is_star_shaped(&u, center), "U is not star-shaped there");
        let triangles = triangulate(&u, center);
        assert!(
            (area_of(&u, center, &triangles) - 7.0).abs() < 1e-4,
            "U has area 7"
        );
    }
}
