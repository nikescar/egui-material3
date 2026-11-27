// ============================================================================
// Module:       shapes::material
// Description:  The 35 named Material 3 Expressive shapes.
//
// Dependencies: egui (emath); super::polygon
// ============================================================================

//! [`MaterialShape`]: the named shapes of the M3 Expressive shape library,
//! with the vertex and rounding values of Compose's `MaterialShapes.kt`
//! (`androidx.compose.material3`, Apache-2.0). Every shape is normalized to
//! the unit square and built once.

use std::sync::OnceLock;

use egui::{vec2, Vec2};

use super::polygon::{CornerRounding, RoundedPolygon, ShapeError};

macro_rules! material_shapes {
    ($($(#[$doc:meta])* $variant:ident),* $(,)?) => {
        /// One of the named Material 3 Expressive shapes.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum MaterialShape {
            $($(#[$doc])* $variant,)*
        }

        impl MaterialShape {
            /// Every shape, in `MaterialShapes.kt` order.
            pub const ALL: [MaterialShape; material_shapes!(@count $($variant)*)] =
                [$(MaterialShape::$variant,)*];

            /// The shape's name, as Compose spells it.
            pub const fn name(self) -> &'static str {
                match self {
                    $(MaterialShape::$variant => stringify!($variant),)*
                }
            }
        }
    };
    (@count $($x:ident)*) => { <[()]>::len(&[$(material_shapes!(@unit $x)),*]) };
    (@unit $x:ident) => { () };
}

material_shapes! {
    /// A circle.
    Circle,
    /// A rounded square.
    Square,
    /// A slanted rounded square.
    Slanted,
    /// An arch: round top, squarer bottom.
    Arch,
    /// A quarter-circle fan.
    Fan,
    /// An arrowhead.
    Arrow,
    /// A semicircle.
    SemiCircle,
    /// A tilted oval.
    Oval,
    /// A tilted pill.
    Pill,
    /// A rounded triangle.
    Triangle,
    /// A rounded diamond.
    Diamond,
    /// A clam shell.
    ClamShell,
    /// A rounded pentagon.
    Pentagon,
    /// A gem.
    Gem,
    /// An 8-pointed sun.
    Sunny,
    /// A spikier 8-pointed sun.
    VerySunny,
    /// A 4-sided cookie.
    Cookie4Sided,
    /// A 6-sided cookie.
    Cookie6Sided,
    /// A 7-sided cookie.
    Cookie7Sided,
    /// A 9-sided cookie.
    Cookie9Sided,
    /// A 12-sided cookie.
    Cookie12Sided,
    /// A ghost-like shape.
    Ghostish,
    /// A 4-leaf clover.
    Clover4Leaf,
    /// An 8-leaf clover.
    Clover8Leaf,
    /// A 12-pointed burst.
    Burst,
    /// A soft 10-pointed burst.
    SoftBurst,
    /// A 15-pointed boom.
    Boom,
    /// A soft 16-pointed boom.
    SoftBoom,
    /// An 8-petal flower.
    Flower,
    /// A puffy cloud.
    Puffy,
    /// A puffy diamond.
    PuffyDiamond,
    /// A pixelated circle.
    PixelCircle,
    /// A pixelated triangle.
    PixelTriangle,
    /// A bun.
    Bun,
    /// A heart.
    Heart,
}

const SHAPE_COUNT: usize = MaterialShape::ALL.len();

impl MaterialShape {
    /// The shape, normalized to the unit square. Built on first use.
    ///
    /// Every named shape builds (the tests check it); should one ever fail,
    /// an empty polygon ([`RoundedPolygon::empty`]) is returned instead of
    /// panicking.
    pub fn polygon(self) -> &'static RoundedPolygon {
        static CACHE: [OnceLock<Option<RoundedPolygon>>; SHAPE_COUNT] =
            [const { OnceLock::new() }; SHAPE_COUNT];
        static EMPTY: OnceLock<RoundedPolygon> = OnceLock::new();
        let index = self as usize;
        CACHE
            .get(index)
            .and_then(|cell| {
                cell.get_or_init(|| self.build().and_then(|p| p.normalized()).ok())
                    .as_ref()
            })
            .unwrap_or_else(|| EMPTY.get_or_init(|| RoundedPolygon::empty(Vec2::splat(0.5))))
    }

    /// Builds the shape (not yet normalized), as `MaterialShapes.kt` does.
    pub fn build(self) -> Result<RoundedPolygon, ShapeError> {
        let r15 = CornerRounding::radius(0.15);
        let r20 = CornerRounding::radius(0.2);
        let r30 = CornerRounding::radius(0.3);
        let r50 = CornerRounding::radius(0.5);
        let r100 = CornerRounding::radius(1.0);
        let unrounded = CornerRounding::UNROUNDED;
        let pr = |x: f32, y: f32, r: f32| PointNRound::new(x, y, CornerRounding::radius(r));
        let ps = |x: f32, y: f32, r: f32, s: f32| PointNRound::new(x, y, CornerRounding::new(r, s));
        let p = |x: f32, y: f32| PointNRound::new(x, y, unrounded);
        match self {
            MaterialShape::Circle => RoundedPolygon::circle(10, 1.0, Vec2::ZERO),
            MaterialShape::Square => RoundedPolygon::rectangle(1.0, 1.0, r30, None, Vec2::ZERO),
            MaterialShape::Slanted => custom(
                &[
                    ps(0.926, 0.970, 0.189, 0.811),
                    ps(-0.021, 0.967, 0.187, 0.057),
                ],
                2,
                false,
            ),
            MaterialShape::Arch => RoundedPolygon::regular(
                4,
                1.0,
                Vec2::ZERO,
                unrounded,
                Some(&[r100, r100, r20, r20]),
            )?
            .rotated(-135.0),
            MaterialShape::Fan => custom(
                &[
                    ps(1.004, 1.000, 0.148, 0.417),
                    pr(0.000, 1.000, 0.151),
                    pr(0.000, -0.003, 0.148),
                    pr(0.978, 0.020, 0.803),
                ],
                1,
                false,
            ),
            MaterialShape::Arrow => custom(
                &[
                    pr(0.500, 0.892, 0.313),
                    pr(-0.216, 1.050, 0.207),
                    ps(0.499, -0.160, 0.215, 1.000),
                    pr(1.225, 1.060, 0.211),
                ],
                1,
                false,
            ),
            MaterialShape::SemiCircle => RoundedPolygon::rectangle(
                1.6,
                1.0,
                unrounded,
                Some(&[r20, r20, r100, r100]),
                Vec2::ZERO,
            ),
            MaterialShape::Oval => RoundedPolygon::circle(8, 1.0, Vec2::ZERO)?
                .scaled(1.0, 0.64)?
                .rotated(-45.0),
            MaterialShape::Pill => custom(
                &[
                    pr(0.961, 0.039, 0.426),
                    p(1.001, 0.428),
                    pr(1.000, 0.609, 1.000),
                ],
                2,
                true,
            ),
            MaterialShape::Triangle => {
                RoundedPolygon::regular(3, 1.0, Vec2::ZERO, r20, None)?.rotated(-90.0)
            }
            MaterialShape::Diamond => custom(
                &[ps(0.500, 1.096, 0.151, 0.524), pr(0.040, 0.500, 0.159)],
                2,
                false,
            ),
            MaterialShape::ClamShell => custom(
                &[
                    pr(0.171, 0.841, 0.159),
                    pr(-0.020, 0.500, 0.140),
                    pr(0.170, 0.159, 0.159),
                ],
                2,
                false,
            ),
            MaterialShape::Pentagon => custom(
                &[
                    pr(0.500, -0.009, 0.172),
                    pr(1.030, 0.365, 0.164),
                    pr(0.828, 0.970, 0.169),
                ],
                1,
                true,
            ),
            MaterialShape::Gem => custom(
                &[
                    ps(0.499, 1.023, 0.241, 0.778),
                    pr(-0.005, 0.792, 0.208),
                    pr(0.073, 0.258, 0.228),
                    pr(0.433, -0.000, 0.491),
                ],
                1,
                true,
            ),
            MaterialShape::Sunny => RoundedPolygon::star(8, 1.0, 0.8, r15, None, Vec2::ZERO),
            MaterialShape::VerySunny => custom(
                &[pr(0.500, 1.080, 0.085), pr(0.358, 0.843, 0.085)],
                8,
                false,
            ),
            MaterialShape::Cookie4Sided => custom(
                &[pr(1.237, 1.236, 0.258), pr(0.500, 0.918, 0.233)],
                4,
                false,
            ),
            MaterialShape::Cookie6Sided => custom(
                &[pr(0.723, 0.884, 0.394), pr(0.500, 1.099, 0.398)],
                6,
                false,
            ),
            MaterialShape::Cookie7Sided => {
                RoundedPolygon::star(7, 1.0, 0.75, r50, None, Vec2::ZERO)?.rotated(-90.0)
            }
            MaterialShape::Cookie9Sided => {
                RoundedPolygon::star(9, 1.0, 0.8, r50, None, Vec2::ZERO)?.rotated(-90.0)
            }
            MaterialShape::Cookie12Sided => {
                RoundedPolygon::star(12, 1.0, 0.8, r50, None, Vec2::ZERO)?.rotated(-90.0)
            }
            MaterialShape::Ghostish => custom(
                &[
                    pr(0.500, 0.0, 1.000),
                    pr(1.0, 0.0, 1.000),
                    ps(1.0, 1.140, 0.254, 0.106),
                    pr(0.575, 0.906, 0.253),
                ],
                1,
                true,
            ),
            MaterialShape::Clover4Leaf => {
                custom(&[p(0.500, 0.074), pr(0.725, -0.099, 0.476)], 4, true)
            }
            MaterialShape::Clover8Leaf => {
                custom(&[p(0.500, 0.036), pr(0.758, -0.101, 0.209)], 8, false)
            }
            MaterialShape::Burst => custom(
                &[pr(0.500, -0.006, 0.006), pr(0.592, 0.158, 0.006)],
                12,
                false,
            ),
            MaterialShape::SoftBurst => custom(
                &[pr(0.193, 0.277, 0.053), pr(0.176, 0.055, 0.053)],
                10,
                false,
            ),
            MaterialShape::Boom => custom(
                &[pr(0.457, 0.296, 0.007), pr(0.500, -0.051, 0.007)],
                15,
                false,
            ),
            MaterialShape::SoftBoom => custom(
                &[
                    p(0.733, 0.454),
                    pr(0.839, 0.437, 0.532),
                    ps(0.949, 0.449, 0.439, 1.000),
                    pr(0.998, 0.478, 0.174),
                ],
                16,
                true,
            ),
            MaterialShape::Flower => custom(
                &[
                    p(0.370, 0.187),
                    pr(0.416, 0.049, 0.381),
                    pr(0.479, 0.001, 0.095),
                ],
                8,
                true,
            ),
            MaterialShape::Puffy => custom(
                &[
                    p(0.500, 0.053),
                    pr(0.545, -0.040, 0.405),
                    pr(0.670, -0.035, 0.426),
                    pr(0.717, 0.066, 0.574),
                    p(0.722, 0.128),
                    pr(0.777, 0.002, 0.360),
                    pr(0.914, 0.149, 0.660),
                    pr(0.926, 0.289, 0.660),
                    p(0.881, 0.346),
                    pr(0.940, 0.344, 0.126),
                    pr(1.003, 0.437, 0.255),
                ],
                2,
                true,
            )?
            .scaled(1.0, 0.742),
            MaterialShape::PuffyDiamond => custom(
                &[
                    pr(0.870, 0.130, 0.146),
                    p(0.818, 0.357),
                    pr(1.000, 0.332, 0.853),
                ],
                4,
                true,
            ),
            MaterialShape::PixelCircle => custom(
                &[
                    p(0.500, 0.000),
                    p(0.704, 0.000),
                    p(0.704, 0.065),
                    p(0.843, 0.065),
                    p(0.843, 0.148),
                    p(0.926, 0.148),
                    p(0.926, 0.296),
                    p(1.000, 0.296),
                ],
                2,
                true,
            ),
            MaterialShape::PixelTriangle => custom(
                &[
                    p(0.110, 0.500),
                    p(0.113, 0.000),
                    p(0.287, 0.000),
                    p(0.287, 0.087),
                    p(0.421, 0.087),
                    p(0.421, 0.170),
                    p(0.560, 0.170),
                    p(0.560, 0.265),
                    p(0.674, 0.265),
                    p(0.675, 0.344),
                    p(0.789, 0.344),
                    p(0.789, 0.439),
                    p(0.888, 0.439),
                ],
                1,
                true,
            ),
            MaterialShape::Bun => custom(
                &[
                    p(0.796, 0.500),
                    pr(0.853, 0.518, 1.0),
                    pr(0.992, 0.631, 1.0),
                    pr(0.968, 1.000, 1.0),
                ],
                2,
                true,
            ),
            MaterialShape::Heart => custom(
                &[
                    pr(0.500, 0.268, 0.016),
                    pr(0.792, -0.066, 0.958),
                    pr(1.064, 0.276, 1.000),
                    pr(0.501, 0.946, 0.129),
                ],
                1,
                true,
            ),
        }
    }
}

/// A vertex and its rounding (`PointNRound`).
#[derive(Clone, Copy, Debug)]
struct PointNRound {
    o: Vec2,
    r: CornerRounding,
}

impl PointNRound {
    fn new(x: f32, y: f32, r: CornerRounding) -> Self {
        Self { o: vec2(x, y), r }
    }
}

fn rotate_degrees(p: Vec2, angle: f32, center: Vec2) -> Vec2 {
    let (sin, cos) = angle.to_radians().sin_cos();
    let off = p - center;
    vec2(off.x * cos - off.y * sin, off.x * sin + off.y * cos) + center
}

/// `doRepeat`: `points` repeated `reps` times around `center`, every other
/// copy mirrored when `mirroring`.
fn do_repeat(
    points: &[PointNRound],
    reps: usize,
    center: Vec2,
    mirroring: bool,
) -> Vec<PointNRound> {
    let np = points.len();
    if mirroring {
        let angle_of = |p: &PointNRound| {
            let d = p.o - center;
            d.y.atan2(d.x).to_degrees()
        };
        let angles: Vec<f32> = points.iter().map(angle_of).collect();
        let distances: Vec<f32> = points.iter().map(|p| (p.o - center).length()).collect();
        let angle0 = angles.first().copied().unwrap_or(0.0);
        let actual_reps = reps * 2;
        let section = 360.0 / actual_reps as f32;
        let mut out = Vec::with_capacity(actual_reps * np);
        for it in 0..actual_reps {
            for index in 0..np {
                let i = if it % 2 == 0 { index } else { np - 1 - index };
                if i > 0 || it % 2 == 0 {
                    let (Some(angle), Some(distance), Some(point)) =
                        (angles.get(i), distances.get(i), points.get(i))
                    else {
                        continue;
                    };
                    let a = section * it as f32
                        + if it % 2 == 0 {
                            *angle
                        } else {
                            section - angle + 2.0 * angle0
                        };
                    let final_point = Vec2::angled(a.to_radians()) * *distance + center;
                    out.push(PointNRound {
                        o: final_point,
                        r: point.r,
                    });
                }
            }
        }
        out
    } else {
        (0..np * reps)
            .filter_map(|it| {
                let point = points.get(it % np)?;
                let angle = (it / np) as f32 * 360.0 / reps as f32;
                Some(PointNRound {
                    o: rotate_degrees(point.o, angle, center),
                    r: point.r,
                })
            })
            .collect()
    }
}

/// `customPolygon`, centred on (0.5, 0.5).
fn custom(
    points: &[PointNRound],
    reps: usize,
    mirroring: bool,
) -> Result<RoundedPolygon, ShapeError> {
    let center = Vec2::splat(0.5);
    let actual = do_repeat(points, reps, center, mirroring);
    let vertices: Vec<Vec2> = actual.iter().map(|p| p.o).collect();
    let roundings: Vec<CornerRounding> = actual.iter().map(|p| p.r).collect();
    RoundedPolygon::from_vertices(
        &vertices,
        CornerRounding::UNROUNDED,
        Some(&roundings),
        Some(center),
    )
}
