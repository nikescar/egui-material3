// ============================================================================
// Module:       shapes::cubic
// Description:  Cubic Bezier segments, the building block of every shape.
//
// Dependencies: egui (emath)
// ============================================================================

//! [`Cubic`]: one cubic Bezier segment, ported from `androidx.graphics.shapes`
//! (`Cubic.kt`, `Utils.kt`, `Point.kt`; Apache-2.0).
//!
//! Points are [`egui::Vec2`] in the shape's own unit space; nothing here
//! knows about screen coordinates.

use egui::{vec2, Rect, Vec2};

/// Distances below this are treated as zero (`Utils.kt` `DistanceEpsilon`).
pub(crate) const DISTANCE_EPSILON: f32 = 1e-4;
/// Angles (and outline progress) below this are equal (`AngleEpsilon`).
pub(crate) const ANGLE_EPSILON: f32 = 1e-6;

/// `Point.rotate90()`: a quarter turn, `(-y, x)`.
///
/// Not `Vec2::rot90`, which turns the other way.
pub(crate) fn rotate90(p: Vec2) -> Vec2 {
    vec2(-p.y, p.x)
}

/// `Point.clockwise()`: the 2D cross product is positive.
pub(crate) fn clockwise(a: Vec2, b: Vec2) -> bool {
    a.x * b.y - a.y * b.x > 0.0
}

/// `convex()`: whether the turn at `current` is convex for a clockwise
/// (in y-down space) outline.
pub(crate) fn convex(previous: Vec2, current: Vec2, next: Vec2) -> bool {
    clockwise(current - previous, next - current)
}

/// `directionVector()`: `p` scaled to unit length, or `None` for a zero
/// vector (where Kotlin throws).
pub(crate) fn direction(p: Vec2) -> Option<Vec2> {
    let d = p.length();
    (d > 0.0).then(|| p / d)
}

/// `positiveModulo()`.
pub(crate) fn positive_modulo(num: f32, modulus: f32) -> f32 {
    (num % modulus + modulus) % modulus
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

fn lerp_point(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    vec2(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
}

/// One cubic Bezier segment: two anchors and two control points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cubic {
    /// Start point.
    pub anchor0: Vec2,
    /// First control point.
    pub control0: Vec2,
    /// Second control point.
    pub control1: Vec2,
    /// End point.
    pub anchor1: Vec2,
}

impl Cubic {
    /// A cubic from its four points.
    pub const fn new(anchor0: Vec2, control0: Vec2, control1: Vec2, anchor1: Vec2) -> Self {
        Self {
            anchor0,
            control0,
            control1,
            anchor1,
        }
    }

    /// A straight line from `p0` to `p1`, with the controls at the thirds.
    pub fn straight_line(p0: Vec2, p1: Vec2) -> Self {
        Self::new(
            p0,
            lerp_point(p0, p1, 1.0 / 3.0),
            lerp_point(p0, p1, 2.0 / 3.0),
            p1,
        )
    }

    /// A circular arc around `center` from `p0` to `p1`, the short way.
    pub fn circular_arc(center: Vec2, p0: Vec2, p1: Vec2) -> Self {
        let (Some(p0d), Some(p1d)) = (direction(p0 - center), direction(p1 - center)) else {
            return Self::straight_line(p0, p1);
        };
        let rotated_p0 = rotate90(p0d);
        let rotated_p1 = rotate90(p1d);
        let is_clockwise = rotated_p0.dot(p1 - center) >= 0.0;
        let cosa = p0d.dot(p1d);
        if cosa > 0.999 {
            return Self::straight_line(p0, p1);
        }
        let k = (p0 - center).length() * 4.0 / 3.0
            * ((2.0 * (1.0 - cosa)).sqrt() - (1.0 - cosa * cosa).sqrt())
            / (1.0 - cosa)
            * if is_clockwise { 1.0 } else { -1.0 };
        Self::new(p0, p0 + rotated_p0 * k, p1 - rotated_p1 * k, p1)
    }

    /// A zero-length cubic at `p`.
    pub const fn empty(p: Vec2) -> Self {
        Self::new(p, p, p, p)
    }

    /// The point at parameter `t` (0..=1).
    pub fn point_on_curve(&self, t: f32) -> Vec2 {
        let u = 1.0 - t;
        self.anchor0 * (u * u * u)
            + self.control0 * (3.0 * t * u * u)
            + self.control1 * (3.0 * t * t * u)
            + self.anchor1 * (t * t * t)
    }

    /// Whether the anchors coincide.
    pub fn zero_length(&self) -> bool {
        (self.anchor0.x - self.anchor1.x).abs() < DISTANCE_EPSILON
            && (self.anchor0.y - self.anchor1.y).abs() < DISTANCE_EPSILON
    }

    /// The bounds; `approximate` uses the control points (cheaper, larger).
    pub fn bounds(&self, approximate: bool) -> Rect {
        if self.zero_length() {
            return Rect::from_min_max(self.anchor0.to_pos2(), self.anchor0.to_pos2());
        }
        let mut min = self.anchor0.min(self.anchor1);
        let mut max = self.anchor0.max(self.anchor1);
        if approximate {
            min = min.min(self.control0.min(self.control1));
            max = max.max(self.control0.max(self.control1));
            return Rect::from_min_max(min.to_pos2(), max.to_pos2());
        }
        // Extrema of each axis: roots of the derivative's quadratic.
        let axis = |a0: f32, c0: f32, c1: f32, a1: f32| -> [Option<f32>; 2] {
            let a = -a0 + 3.0 * c0 - 3.0 * c1 + a1;
            let b = 2.0 * a0 - 4.0 * c0 + 2.0 * c1;
            let c = -a0 + c0;
            if a.abs() < DISTANCE_EPSILON {
                if b != 0.0 {
                    [Some(2.0 * c / (-2.0 * b)), None]
                } else {
                    [None, None]
                }
            } else {
                let s = b * b - 4.0 * a * c;
                if s >= 0.0 {
                    [
                        Some((-b + s.sqrt()) / (2.0 * a)),
                        Some((-b - s.sqrt()) / (2.0 * a)),
                    ]
                } else {
                    [None, None]
                }
            }
        };
        let in_range = |t: &f32| (0.0..=1.0).contains(t);
        for t in axis(
            self.anchor0.x,
            self.control0.x,
            self.control1.x,
            self.anchor1.x,
        )
        .into_iter()
        .flatten()
        .filter(in_range)
        {
            let x = self.point_on_curve(t).x;
            min.x = min.x.min(x);
            max.x = max.x.max(x);
        }
        for t in axis(
            self.anchor0.y,
            self.control0.y,
            self.control1.y,
            self.anchor1.y,
        )
        .into_iter()
        .flatten()
        .filter(in_range)
        {
            let y = self.point_on_curve(t).y;
            min.y = min.y.min(y);
            max.y = max.y.max(y);
        }
        Rect::from_min_max(min.to_pos2(), max.to_pos2())
    }

    /// Splits at parameter `t` into the parts before and after it.
    pub fn split(&self, t: f32) -> (Cubic, Cubic) {
        let u = 1.0 - t;
        let p = self.point_on_curve(t);
        (
            Cubic::new(
                self.anchor0,
                self.anchor0 * u + self.control0 * t,
                self.anchor0 * (u * u) + self.control0 * (2.0 * u * t) + self.control1 * (t * t),
                p,
            ),
            Cubic::new(
                p,
                self.control0 * (u * u) + self.control1 * (2.0 * u * t) + self.anchor1 * (t * t),
                self.control1 * u + self.anchor1 * t,
                self.anchor1,
            ),
        )
    }

    /// The same curve, traversed the other way.
    pub fn reverse(&self) -> Cubic {
        Cubic::new(self.anchor1, self.control1, self.control0, self.anchor0)
    }

    /// Every point mapped through `f`.
    pub fn transformed(&self, f: &impl Fn(Vec2) -> Vec2) -> Cubic {
        Cubic::new(
            f(self.anchor0),
            f(self.control0),
            f(self.control1),
            f(self.anchor1),
        )
    }

    /// Point-wise interpolation between `a` and `b`.
    pub fn lerp(a: &Cubic, b: &Cubic, t: f32) -> Cubic {
        Cubic::new(
            lerp_point(a.anchor0, b.anchor0, t),
            lerp_point(a.control0, b.control0, t),
            lerp_point(a.control1, b.control1, t),
            lerp_point(a.anchor1, b.anchor1, t),
        )
    }

    /// Flattens the curve into `segments` points after `anchor0`.
    pub(crate) fn flatten_into(&self, segments: usize, out: &mut Vec<Vec2>) {
        let n = segments.max(1);
        for i in 1..=n {
            out.push(self.point_on_curve(i as f32 / n as f32));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_halves_meet_on_the_curve() {
        let c = Cubic::new(
            vec2(0.0, 0.0),
            vec2(1.0, 2.0),
            vec2(3.0, 2.0),
            vec2(4.0, 0.0),
        );
        let (a, b) = c.split(0.3);
        assert!(
            (a.anchor1 - c.point_on_curve(0.3)).length() < 1e-5,
            "first half ends on the curve"
        );
        assert_eq!(a.anchor1, b.anchor0, "halves meet");
        assert!(
            (a.point_on_curve(0.5) - c.point_on_curve(0.15)).length() < 1e-5,
            "first half reparametrises the curve"
        );
    }

    #[test]
    fn exact_bounds_include_the_peak() {
        let c = Cubic::new(
            vec2(0.0, 0.0),
            vec2(0.0, 4.0),
            vec2(4.0, 4.0),
            vec2(4.0, 0.0),
        );
        let exact = c.bounds(false);
        assert!(
            (exact.max.y - 3.0).abs() < 1e-5,
            "peak at y = 3, got {exact:?}"
        );
        assert!(
            (c.bounds(true).max.y - 4.0).abs() < 1e-5,
            "approximate bounds use the controls"
        );
    }

    #[test]
    fn quarter_arc_stays_near_the_circle() {
        let arc = Cubic::circular_arc(Vec2::ZERO, vec2(1.0, 0.0), vec2(0.0, 1.0));
        for i in 0..=10 {
            let r = arc.point_on_curve(i as f32 / 10.0).length();
            assert!((r - 1.0).abs() < 1e-3, "radius {r} at sample {i}");
        }
    }
}
