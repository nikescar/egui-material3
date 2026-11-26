//! Motion: durations, easing curves and springs.
//!
//! Durations and easings from Compose's `MotionTokens.kt`; springs from
//! `StandardMotionTokens.kt` and `ExpressiveMotionTokens.kt` (the two
//! motion schemes of `MotionSchemeKeyTokens`). The spring maths is
//! Compose's `SpringSimulation` (unit mass, closed-form solution), and
//! the easing is Compose's `CubicBezierEasing`.

use std::time::Duration;

/// `MotionTokens.DurationShort1`: 50 ms.
pub const DURATION_SHORT1: Duration = Duration::from_millis(50);
/// `MotionTokens.DurationShort2`: 100 ms.
pub const DURATION_SHORT2: Duration = Duration::from_millis(100);
/// `MotionTokens.DurationShort3`: 150 ms.
pub const DURATION_SHORT3: Duration = Duration::from_millis(150);
/// `MotionTokens.DurationShort4`: 200 ms.
pub const DURATION_SHORT4: Duration = Duration::from_millis(200);
/// `MotionTokens.DurationMedium1`: 250 ms.
pub const DURATION_MEDIUM1: Duration = Duration::from_millis(250);
/// `MotionTokens.DurationMedium2`: 300 ms.
pub const DURATION_MEDIUM2: Duration = Duration::from_millis(300);
/// `MotionTokens.DurationMedium3`: 350 ms.
pub const DURATION_MEDIUM3: Duration = Duration::from_millis(350);
/// `MotionTokens.DurationMedium4`: 400 ms.
pub const DURATION_MEDIUM4: Duration = Duration::from_millis(400);
/// `MotionTokens.DurationLong1`: 450 ms.
pub const DURATION_LONG1: Duration = Duration::from_millis(450);
/// `MotionTokens.DurationLong2`: 500 ms.
pub const DURATION_LONG2: Duration = Duration::from_millis(500);
/// `MotionTokens.DurationLong3`: 550 ms.
pub const DURATION_LONG3: Duration = Duration::from_millis(550);
/// `MotionTokens.DurationLong4`: 600 ms.
pub const DURATION_LONG4: Duration = Duration::from_millis(600);
/// `MotionTokens.DurationExtraLong1`: 700 ms.
pub const DURATION_EXTRA_LONG1: Duration = Duration::from_millis(700);
/// `MotionTokens.DurationExtraLong2`: 800 ms.
pub const DURATION_EXTRA_LONG2: Duration = Duration::from_millis(800);
/// `MotionTokens.DurationExtraLong3`: 900 ms.
pub const DURATION_EXTRA_LONG3: Duration = Duration::from_millis(900);
/// `MotionTokens.DurationExtraLong4`: 1000 ms.
pub const DURATION_EXTRA_LONG4: Duration = Duration::from_millis(1000);

/// A CSS-style cubic Bézier easing curve from (0, 0) to (1, 1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBezier {
    /// First control point x.
    pub x1: f32,
    /// First control point y.
    pub y1: f32,
    /// Second control point x.
    pub x2: f32,
    /// Second control point y.
    pub y2: f32,
}

impl CubicBezier {
    /// A curve with control points (x1, y1) and (x2, y2).
    pub const fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// `MotionTokens.EasingEmphasizedCubicBezier`.
    pub const EMPHASIZED: Self = Self::new(0.2, 0.0, 0.0, 1.0);
    /// `MotionTokens.EasingEmphasizedDecelerateCubicBezier`.
    pub const EMPHASIZED_DECELERATE: Self = Self::new(0.05, 0.7, 0.1, 1.0);
    /// `MotionTokens.EasingEmphasizedAccelerateCubicBezier`.
    pub const EMPHASIZED_ACCELERATE: Self = Self::new(0.3, 0.0, 0.8, 0.15);
    /// `MotionTokens.EasingStandardCubicBezier`.
    pub const STANDARD: Self = Self::new(0.2, 0.0, 0.0, 1.0);
    /// `MotionTokens.EasingStandardDecelerateCubicBezier`.
    pub const STANDARD_DECELERATE: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    /// `MotionTokens.EasingStandardAccelerateCubicBezier`.
    pub const STANDARD_ACCELERATE: Self = Self::new(0.3, 0.0, 1.0, 1.0);
    /// `MotionTokens.EasingLegacyCubicBezier`.
    pub const LEGACY: Self = Self::new(0.4, 0.0, 0.2, 1.0);
    /// `MotionTokens.EasingLegacyDecelerateCubicBezier`.
    pub const LEGACY_DECELERATE: Self = Self::new(0.0, 0.0, 0.2, 1.0);
    /// `MotionTokens.EasingLegacyAccelerateCubicBezier`.
    pub const LEGACY_ACCELERATE: Self = Self::new(0.4, 0.0, 1.0, 1.0);
    /// `MotionTokens.EasingLinearCubicBezier`.
    pub const LINEAR: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    fn bezier(t: f32, p1: f32, p2: f32) -> f32 {
        // B(t) for control values 0, p1, p2, 1.
        let u = 1.0 - t;
        3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t
    }

    /// The eased progress for linear progress `fraction` in 0..=1.
    pub fn transform(&self, fraction: f32) -> f32 {
        if fraction <= 0.0 {
            return 0.0;
        }
        if fraction >= 1.0 {
            return 1.0;
        }
        // x(t) is monotonic for x1, x2 in 0..=1: bisect for x(t) = fraction.
        let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
        let mut t = fraction;
        for _ in 0..32 {
            let x = Self::bezier(t, self.x1, self.x2);
            if (x - fraction).abs() < 1e-6 {
                break;
            }
            if x < fraction {
                lo = t;
            } else {
                hi = t;
            }
            t = (lo + hi) / 2.0;
        }
        Self::bezier(t, self.y1, self.y2)
    }
}

/// A spring: damping ratio and stiffness, at unit mass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringSpec {
    /// Damping ratio: 1 is critically damped, below 1 bounces.
    pub damping_ratio: f32,
    /// Stiffness (N/m at unit mass).
    pub stiffness: f32,
}

impl SpringSpec {
    /// A spring with `damping_ratio` and `stiffness`.
    pub const fn new(damping_ratio: f32, stiffness: f32) -> Self {
        Self {
            damping_ratio,
            stiffness,
        }
    }

    /// Displacement from the target and velocity after `t` seconds, starting
    /// at displacement `x0` with velocity `v0` (Compose's
    /// `SpringSimulation.updateValues`).
    pub fn displacement(&self, x0: f32, v0: f32, t: f32) -> (f32, f32) {
        let x0 = f64::from(x0);
        let v0 = f64::from(v0);
        let t = f64::from(t);
        let z = f64::from(self.damping_ratio);
        let w0 = f64::from(self.stiffness).max(0.0).sqrt();
        let (x, v) = if z > 1.0 {
            let root = w0 * (z * z - 1.0).sqrt();
            let gp = -z * w0 + root;
            let gm = -z * w0 - root;
            let b = (gm * x0 - v0) / (gm - gp);
            let a = x0 - b;
            let (ea, eb) = ((gm * t).exp(), (gp * t).exp());
            (a * ea + b * eb, a * gm * ea + b * gp * eb)
        } else if (z - 1.0).abs() < f64::EPSILON {
            let a = x0;
            let b = v0 + w0 * x0;
            let e = (-w0 * t).exp();
            let x = (a + b * t) * e;
            (x, b * e - w0 * x)
        } else {
            let wd = w0 * (1.0 - z * z).sqrt();
            let c = x0;
            let s = (z * w0 * x0 + v0) / wd;
            let e = (-z * w0 * t).exp();
            let (sin, cos) = (wd * t).sin_cos();
            let x = e * (c * cos + s * sin);
            let v = -z * w0 * x + e * (-c * wd * sin + s * wd * cos);
            (x, v)
        };
        (x as f32, v as f32)
    }

    /// Progress from 0 towards 1 after `t` seconds, starting at rest.
    pub fn progress(&self, t: f32) -> f32 {
        1.0 - self.displacement(1.0, 0.0, t).0
    }
}

/// How fast a spring is, within a motion scheme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SpringSpeed {
    /// Small components (switches, buttons).
    Fast,
    /// Partial-screen transitions (sheets, drawers).
    #[default]
    Default,
    /// Full-screen transitions.
    Slow,
}

/// The two Material 3 motion schemes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MotionScheme {
    /// `StandardMotionTokens`: functional, little bounce.
    #[default]
    Standard,
    /// `ExpressiveMotionTokens`: springier spatial motion.
    Expressive,
}

impl MotionScheme {
    /// Spring for movement, size and shape (`Spring<Speed>Spatial*`).
    pub const fn spatial(self, speed: SpringSpeed) -> SpringSpec {
        match (self, speed) {
            (MotionScheme::Standard, SpringSpeed::Fast) => SpringSpec::new(0.9, 1400.0),
            (MotionScheme::Standard, SpringSpeed::Default) => SpringSpec::new(0.9, 700.0),
            (MotionScheme::Standard, SpringSpeed::Slow) => SpringSpec::new(0.9, 300.0),
            (MotionScheme::Expressive, SpringSpeed::Fast) => SpringSpec::new(0.6, 800.0),
            (MotionScheme::Expressive, SpringSpeed::Default) => SpringSpec::new(0.8, 380.0),
            (MotionScheme::Expressive, SpringSpeed::Slow) => SpringSpec::new(0.8, 200.0),
        }
    }

    /// Spring for colour and opacity (`Spring<Speed>Effects*`; the same in
    /// both schemes).
    pub const fn effects(self, speed: SpringSpeed) -> SpringSpec {
        match speed {
            SpringSpeed::Fast => SpringSpec::new(1.0, 3800.0),
            SpringSpeed::Default => SpringSpec::new(1.0, 1600.0),
            SpringSpeed::Slow => SpringSpec::new(1.0, 800.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SpringState {
    target: f32,
    start_time: f64,
    x0: f32,
    v0: f32,
}

/// Animates towards `target` with `spring`, keyed by `id`.
///
/// The first call snaps to `target`. When `target` changes, the spring
/// restarts from the current value and velocity, so motion stays smooth
/// when interrupted. Repaints are requested until it settles.
pub fn animate_value_with_spring(
    ctx: &egui::Context,
    id: egui::Id,
    target: f32,
    spring: SpringSpec,
) -> f32 {
    let now = ctx.input(|i| i.time);
    let state = ctx.data(|d| d.get_temp::<SpringState>(id));
    let (state, value, velocity) = match state {
        None => {
            let s = SpringState {
                target,
                start_time: now,
                x0: 0.0,
                v0: 0.0,
            };
            (s, target, 0.0)
        }
        Some(s) => {
            let t = (now - s.start_time).max(0.0) as f32;
            let (x, v) = spring.displacement(s.x0, s.v0, t);
            let value = s.target + x;
            if (s.target - target).abs() > f32::EPSILON {
                let s = SpringState {
                    target,
                    start_time: now,
                    x0: value - target,
                    v0: v,
                };
                (s, value, v)
            } else {
                (s, value, v)
            }
        }
    };
    let settled = (value - target).abs() < 1e-3 && velocity.abs() < 1e-2;
    let (state, value) = if settled {
        (
            SpringState {
                target,
                start_time: now,
                x0: 0.0,
                v0: 0.0,
            },
            target,
        )
    } else {
        ctx.request_repaint();
        (state, value)
    };
    ctx.data_mut(|d| d.insert_temp(id, state));
    value
}
