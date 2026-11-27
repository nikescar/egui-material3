// ============================================================================
// Module:       loading_indicator
// Description:  The M3 Expressive loading indicator: a morphing shape.
//
// Dependencies: egui; crate::{shapes, tokens, material_theme}
// ============================================================================

//! [`LoadingIndicator`]: a shape that morphs through a sequence of
//! [`MaterialShape`]s while it spins, contained (on a `PrimaryContainer`
//! circle) or not.
//!
//! Values from Compose: `LoadingIndicatorTokens.kt` (48 dp container, 38 dp
//! active indicator, `Primary`, contained `OnPrimaryContainer` on
//! `PrimaryContainer`, `CornerFull` container) and `LoadingIndicator.kt`
//! (the shape sequences, a morph every 650 ms driven by a
//! `spring(dampingRatio = 0.6, stiffness = 200)`, a quarter turn per morph,
//! a 4666 ms linear global rotation; determinate: circle to soft burst,
//! turning counter-clockwise by half a turn).
//!
//! The animation is a pure function of `ui.input(|i| i.time)`, so a frame at
//! a given time is reproducible ([`LoadingIndicator::frame_at`]).
//!
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::LoadingIndicator;
//! ui.add(LoadingIndicator::new());
//! ui.add(LoadingIndicator::contained().progress(0.4));
//! # });
//! ```

use std::sync::OnceLock;

use egui::{Color32, Rect, Response, Sense, Ui, Vec2, Widget, WidgetInfo};

use crate::shapes::{fill, MaterialShape, Morph, RoundedPolygon};
use crate::tokens::{ColorRole, SpringSpec};
use crate::MaterialTheme;

/// `LoadingIndicatorTokens.ContainerWidth` / `ContainerHeight`: 48 dp.
pub const CONTAINER_SIZE: f32 = 48.0;
/// `LoadingIndicatorTokens.ActiveSize`: 38 dp.
pub const ACTIVE_SIZE: f32 = 38.0;

/// `LoadingIndicatorDefaults.IndeterminateIndicatorPolygons`, in order.
pub const INDETERMINATE_SEQUENCE: [MaterialShape; 7] = [
    MaterialShape::SoftBurst,
    MaterialShape::Cookie9Sided,
    MaterialShape::Pentagon,
    MaterialShape::Pill,
    MaterialShape::Sunny,
    MaterialShape::Cookie4Sided,
    MaterialShape::Oval,
];

/// `MorphIntervalMillis`: a new morph starts every 650 ms.
const MORPH_INTERVAL: f64 = 0.650;
/// `GlobalRotationDurationMillis`: one full turn in 4666 ms.
const GLOBAL_ROTATION_DURATION: f64 = 4.666;
/// The morph spring of `LoadingIndicatorImpl`.
const MORPH_SPRING: SpringSpec = SpringSpec::new(0.6, 200.0);
/// `QuarterRotation`.
const QUARTER_ROTATION: f64 = 90.0;

/// Where an indeterminate indicator is at one instant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndicatorFrame {
    /// Index into [`INDETERMINATE_SEQUENCE`] of the shape being morphed from
    /// (the morph goes to the next one, wrapping around).
    pub morph_index: usize,
    /// Morph progress: 0 at the start shape, 1 at the next; the spring
    /// overshoots past 1 briefly.
    pub progress: f32,
    /// Clockwise rotation in degrees, 0..360.
    pub rotation_degrees: f32,
}

/// Whether the indicator sits on a container.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Style {
    #[default]
    Plain,
    Contained,
}

/// The M3 Expressive loading indicator.
#[derive(Clone, Debug, Default)]
#[must_use = "add it with `ui.add(...)`"]
pub struct LoadingIndicator {
    style: Style,
    progress: Option<f32>,
    color: Option<Color32>,
    container_color: Option<Color32>,
    label: Option<String>,
}

impl LoadingIndicator {
    /// An uncontained, indeterminate indicator in `Primary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// A contained indicator: `OnPrimaryContainer` on a `PrimaryContainer`
    /// circle.
    pub fn contained() -> Self {
        Self {
            style: Style::Contained,
            ..Self::default()
        }
    }

    /// Makes it determinate at `progress` (0..=1): the shape morphs from a
    /// circle to a soft burst as progress grows.
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(if progress.is_nan() {
            0.0
        } else {
            progress.clamp(0.0, 1.0)
        });
        self
    }

    /// Overrides the indicator colour.
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the container colour (contained style only).
    pub fn container_color(mut self, color: Color32) -> Self {
        self.container_color = Some(color);
        self
    }

    /// The accessible label (default "Loading").
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// The indeterminate animation at `time` seconds: which morph, how far
    /// along, and the rotation. Pure, so frames are reproducible.
    pub fn frame_at(time: f64) -> IndicatorFrame {
        let time = time.max(0.0);
        let cycle = (time / MORPH_INTERVAL).floor();
        let local = (time - cycle * MORPH_INTERVAL) as f32;
        let progress = MORPH_SPRING.progress(local);
        let cycle_index = cycle as u64;
        let morph_index = (cycle_index % INDETERMINATE_SEQUENCE.len() as u64) as usize;
        // The target angle starts at a quarter turn and gains one per morph.
        let target = ((cycle_index % 4) as f64 + 1.0) * QUARTER_ROTATION;
        let global = (time % GLOBAL_ROTATION_DURATION) / GLOBAL_ROTATION_DURATION * 360.0;
        let rotation = (f64::from(progress) * QUARTER_ROTATION + target + global) % 360.0;
        IndicatorFrame {
            morph_index,
            progress,
            rotation_degrees: rotation as f32,
        }
    }
}

/// The circular sequence of morphs between the indeterminate shapes.
fn indeterminate_morphs() -> &'static [Option<Morph>] {
    static MORPHS: OnceLock<Vec<Option<Morph>>> = OnceLock::new();
    MORPHS.get_or_init(|| {
        let n = INDETERMINATE_SEQUENCE.len();
        (0..n)
            .map(|i| {
                let from = INDETERMINATE_SEQUENCE.get(i)?.polygon().clone();
                let to = INDETERMINATE_SEQUENCE.get((i + 1) % n)?.polygon().clone();
                Morph::new(from, to).ok()
            })
            .collect()
    })
}

/// `DeterminateIndicatorPolygons`: a circle turned by 18 degrees (to line
/// up with the burst's points) morphing into a soft burst.
fn determinate_morph() -> Option<&'static Morph> {
    static MORPH: OnceLock<Option<Morph>> = OnceLock::new();
    MORPH
        .get_or_init(|| {
            let circle = MaterialShape::Circle
                .polygon()
                .rotated(360.0 / 20.0)
                .ok()?
                .normalized()
                .ok()?;
            Morph::new(circle, MaterialShape::SoftBurst.polygon().clone()).ok()
        })
        .as_ref()
}

/// `calculateScaleFactor`: shrinks the shapes so none clips while rotating.
fn scale_factor(polygons: &[&RoundedPolygon]) -> f32 {
    polygons.iter().fold(1.0_f32, |scale, p| {
        let bounds = p.bounds(true);
        let max = p.max_bounds();
        let sx = bounds.width() / max.width().max(f32::EPSILON);
        let sy = bounds.height() / max.height().max(f32::EPSILON);
        scale.min(sx.max(sy))
    })
}

fn indeterminate_scale() -> f32 {
    static SCALE: OnceLock<f32> = OnceLock::new();
    *SCALE.get_or_init(|| {
        let polygons: Vec<&RoundedPolygon> =
            INDETERMINATE_SEQUENCE.iter().map(|s| s.polygon()).collect();
        scale_factor(&polygons)
    })
}

fn determinate_scale() -> f32 {
    static SCALE: OnceLock<f32> = OnceLock::new();
    *SCALE.get_or_init(|| match determinate_morph() {
        Some(m) => scale_factor(&[m.start(), m.end()]),
        None => 1.0,
    })
}

/// Paints `morph` at `progress` in `rect`, scaled by `scale` of the square's
/// side, centred, and turned `rotation_degrees` clockwise.
fn paint_morph(
    ui: &Ui,
    rect: Rect,
    morph: &Morph,
    progress: f32,
    scale: f32,
    rotation_degrees: f32,
    color: Color32,
) {
    let cubics = morph.as_cubics(progress);
    let bounds = cubics
        .iter()
        .map(|c| c.bounds(true))
        .reduce(|a, b| a.union(b))
        .unwrap_or(Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::ZERO));
    let mid = bounds.center().to_vec2();
    let center =
        morph.start().center() + (morph.end().center() - morph.start().center()) * progress;
    let side = rect.width().min(rect.height()) * scale;
    let (sin, cos) = rotation_degrees.to_radians().sin_cos();
    let origin = rect.center();
    fill(
        ui.painter(),
        &cubics,
        center,
        |p: Vec2| {
            let v = (p - mid) * side;
            origin + egui::vec2(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
        },
        color,
    );
}

impl Widget for LoadingIndicator {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(CONTAINER_SIZE), Sense::hover());
        let label = self.label.clone().unwrap_or_else(|| "Loading".to_owned());
        let value = self.progress.map(f64::from);
        response.widget_info(move || {
            let mut info = WidgetInfo::labeled(egui::Role::ProgressIndicator, true, label.clone());
            info.value = value;
            info
        });
        if !ui.is_rect_visible(rect) {
            return response;
        }
        let theme = MaterialTheme::of(ui.ctx());
        let (indicator_role, container) = match self.style {
            Style::Plain => (ColorRole::Primary, None),
            Style::Contained => (
                ColorRole::OnPrimaryContainer,
                Some(
                    self.container_color
                        .unwrap_or_else(|| theme.color(ColorRole::PrimaryContainer)),
                ),
            ),
        };
        let color = self.color.unwrap_or_else(|| theme.color(indicator_role));
        if let Some(container) = container {
            // `ContainerShape = CornerFull`.
            ui.painter()
                .rect_filled(rect, CONTAINER_SIZE / 2.0, container);
        }
        // `ActiveIndicatorScale`: 38 / 48.
        let active_scale = ACTIVE_SIZE / CONTAINER_SIZE;
        match self.progress {
            Some(progress) => {
                if let Some(morph) = determinate_morph() {
                    // One morph; turns counter-clockwise by half a turn.
                    paint_morph(
                        ui,
                        rect,
                        morph,
                        progress,
                        determinate_scale() * active_scale,
                        -progress * 180.0,
                        color,
                    );
                }
            }
            None => {
                let frame = Self::frame_at(ui.input(|i| i.time));
                if let Some(Some(morph)) = indeterminate_morphs().get(frame.morph_index) {
                    paint_morph(
                        ui,
                        rect,
                        morph,
                        frame.progress,
                        indeterminate_scale() * active_scale,
                        frame.rotation_degrees,
                        color,
                    );
                }
                ui.ctx().request_repaint();
            }
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_morph_in_the_sequence_builds() {
        assert!(
            indeterminate_morphs().iter().all(Option::is_some),
            "all seven indeterminate morphs build"
        );
        assert!(
            determinate_morph().is_some(),
            "the determinate morph builds"
        );
    }

    #[test]
    fn frames_follow_the_compose_sequence() {
        // A new morph every 650 ms, through the seven shapes and around.
        for (k, expected) in (0..10).map(|k| (k, k % 7)) {
            let frame = LoadingIndicator::frame_at(k as f64 * 0.65 + 0.001);
            assert_eq!(frame.morph_index, expected, "morph at cycle {k}");
            assert!(frame.progress < 0.05, "a morph starts near 0 at cycle {k}");
        }
        let settled = LoadingIndicator::frame_at(0.64);
        assert!(
            (settled.progress - 1.0).abs() < 0.01,
            "the spring has settled before the next morph, got {}",
            settled.progress
        );
        let overshoot = (1..64)
            .map(|i| LoadingIndicator::frame_at(f64::from(i) * 0.01).progress)
            .fold(0.0_f32, f32::max);
        assert!(overshoot > 1.0, "the 0.6-damped spring overshoots");
    }

    #[test]
    fn rotation_is_continuous_across_a_morph_boundary() {
        let before = LoadingIndicator::frame_at(0.6499).rotation_degrees;
        let after = LoadingIndicator::frame_at(0.6501).rotation_degrees;
        let delta = (after - before).rem_euclid(360.0);
        assert!(delta < 1.0, "no jump at the boundary: {before} -> {after}");
    }
}
