// ============================================================================
// Module:       progress
// Description:  M3 linear and circular progress indicators, flat and wavy.
//
// Dependencies: egui; crate::{tokens, material_theme}
// ============================================================================

//! Material 3 progress indicators: [`MaterialProgress`], linear or
//! circular, determinate or indeterminate, flat or wavy.
//!
//! # Tokens
//!
//! From Compose (androidx-main):
//!
//! - `ProgressIndicatorTokens`: active indicator and stop `Primary`, track
//!   `SecondaryContainer`, `CornerFull` (round caps);
//! - `LinearProgressIndicatorTokens`: 4 dp thick, `LargeHeight` 10 dp (the
//!   wavy container), 4 dp stop, 4 dp track gap (`TrackActiveSpace`), wave
//!   amplitude 3 dp and wavelength 40 dp (20 dp indeterminate);
//! - `CircularProgressIndicatorTokens`: 40 dp (`Size`), 48 dp (`LargeSize`,
//!   the wavy container), 4 dp thick, 4 dp gap, wave amplitude 1.6 dp and
//!   wavelength 15 dp;
//! - `ProgressIndicator.kt`: 240 dp default linear width; the indeterminate
//!   timings (linear: 1750 ms cycle, lines at 0/250/650/900 ms for
//!   1000/1000/850/850 ms with `EasingEmphasizedAccelerate`; circular: a
//!   6000 ms cycle with a 1080 degree global rotation, a quarter-turn jump
//!   every 1500 ms and the sweep going 10% to 87% and back); the
//!   indeterminate circular track is transparent;
//! - `WavyProgressIndicator.kt`: the amplitude is 0 at or below 10% and at
//!   or above 95% progress, eased over 500 ms, and the wave moves one
//!   wavelength per second.
//!
//! Animation time is read as `f64` from `ui.input(|i| i.time)` and reduced
//! modulo the cycle before it is narrowed, so long uptimes keep precision.

use std::f32::consts::PI;

use egui::epaint::{PathShape, PathStroke};
use egui::{Color32, CornerRadius, Pos2, Rect, Response, Sense, Ui, Vec2, Widget, WidgetInfo};

use crate::tokens::{ColorRole, CubicBezier, LayoutDirection};
use crate::MaterialTheme;

/// `LinearProgressIndicatorTokens.Height`: 4 dp.
pub const LINEAR_HEIGHT: f32 = 4.0;
/// `LinearProgressIndicatorTokens.LargeHeight`: 10 dp.
pub const LINEAR_LARGE_HEIGHT: f32 = 10.0;
/// `ProgressIndicator.kt` `LinearIndicatorWidth`: 240 dp.
pub const LINEAR_WIDTH: f32 = 240.0;
/// `CircularProgressIndicatorTokens.Size`: 40 dp.
pub const CIRCULAR_SIZE: f32 = 40.0;
/// `CircularProgressIndicatorTokens.LargeSize`: 48 dp.
pub const CIRCULAR_LARGE_SIZE: f32 = 48.0;
/// `ActiveThickness` / `TrackThickness`: 4 dp.
pub const THICKNESS: f32 = 4.0;
/// `TrackActiveSpace` (`MeasurementTokens.Space50`): 4 dp.
pub const TRACK_GAP: f32 = 4.0;
/// `LinearProgressIndicatorTokens.StopSize`: 4 dp.
pub const STOP_SIZE: f32 = 4.0;
/// `ProgressIndicator.kt` `StopIndicatorTrailingSpace`: at most 6 dp.
const STOP_MAX_OFFSET: f32 = 6.0;
/// `LinearProgressIndicatorTokens.ActiveWaveAmplitude`: 3 dp.
pub const LINEAR_WAVE_AMPLITUDE: f32 = 3.0;
/// `LinearProgressIndicatorTokens.ActiveWaveWavelength`: 40 dp.
pub const LINEAR_WAVELENGTH: f32 = 40.0;
/// `LinearProgressIndicatorTokens.IndeterminateActiveWaveWavelength`: 20 dp.
pub const LINEAR_INDETERMINATE_WAVELENGTH: f32 = 20.0;
/// `CircularProgressIndicatorTokens.ActiveWaveAmplitude`: 1.6 dp.
pub const CIRCULAR_WAVE_AMPLITUDE: f32 = 1.6;
/// `CircularProgressIndicatorTokens.ActiveWaveWavelength`: 15 dp.
pub const CIRCULAR_WAVELENGTH: f32 = 15.0;

/// `LinearAnimationDuration`: 1750 ms.
const LINEAR_CYCLE_MS: f64 = 1750.0;
/// (delay, duration) in ms of first head, first tail, second head and
/// second tail (`FirstLineHeadDelay` .. `SecondLineTailDuration`).
const LINEAR_LINES: [(f32, f32); 4] = [
    (0.0, 1000.0),
    (250.0, 1000.0),
    (650.0, 850.0),
    (900.0, 850.0),
];
/// `CircularAnimationProgressDuration`: 6000 ms.
const CIRCULAR_CYCLE_MS: f64 = 6000.0;
/// `CircularAnimationAdditionalRotationDelay`: 1500 ms.
const CIRCULAR_ROTATION_DELAY_MS: f32 = 1500.0;
/// `CircularAnimationAdditionalRotationDuration`: 300 ms.
const CIRCULAR_ROTATION_DURATION_MS: f32 = 300.0;
/// `CircularGlobalRotationDegreesTarget`.
const CIRCULAR_GLOBAL_ROTATION: f32 = 1080.0;
/// `CircularIndeterminateMinProgress` / `MaxProgress`.
const CIRCULAR_MIN_PROGRESS: f32 = 0.1;
const CIRCULAR_MAX_PROGRESS: f32 = 0.87;
/// `DurationLong2`: the wavy amplitude change, 500 ms.
const AMPLITUDE_ANIMATION_SECONDS: f32 = 0.5;

/// Material Design progress indicator variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressVariant {
    /// Linear progress bar - horizontal bar showing progress.
    Linear,
    /// Circular progress indicator - circular arc showing progress.
    Circular,
}

/// A Material 3 progress indicator.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::MaterialProgress;
/// ui.add(MaterialProgress::linear().value(0.65));
/// ui.add(MaterialProgress::circular().value(0.8));
/// ui.add(MaterialProgress::linear().indeterminate(true));
/// ui.add(MaterialProgress::linear().wavy(true).value(0.4));
/// ui.add(MaterialProgress::circular().wavy(true).indeterminate(true));
/// ui.add(MaterialProgress::linear().value(0.3).buffer(0.6));
/// # });
/// ```
#[must_use = "add it with `ui.add(...)`"]
pub struct MaterialProgress {
    variant: ProgressVariant,
    value: f32,
    max: f32,
    buffer: Option<f32>,
    indeterminate: bool,
    four_color_enabled: bool,
    size: Option<Vec2>,
    active_color: Option<Color32>,
    track_color: Option<Color32>,
    buffer_color: Option<Color32>,
    border_radius: Option<f32>,
    stroke_width: Option<f32>,
    track_gap: Option<f32>,
    stop_indicator_radius: Option<f32>,
    stop_indicator_color: Option<Color32>,
    wavy: bool,
    large: bool,
    amplitude: Option<f32>,
    wavelength: Option<f32>,
    label: Option<String>,
}

impl MaterialProgress {
    /// A progress indicator of `variant`.
    pub fn new(variant: ProgressVariant) -> Self {
        Self {
            variant,
            value: 0.0,
            max: 1.0,
            buffer: None,
            indeterminate: false,
            four_color_enabled: false,
            size: None,
            active_color: None,
            track_color: None,
            buffer_color: None,
            border_radius: None,
            stroke_width: None,
            track_gap: None,
            stop_indicator_radius: None,
            stop_indicator_color: None,
            wavy: false,
            large: false,
            amplitude: None,
            wavelength: None,
            label: None,
        }
    }

    /// A linear progress bar.
    pub fn linear() -> Self {
        Self::new(ProgressVariant::Linear)
    }

    /// A circular progress indicator.
    pub fn circular() -> Self {
        Self::new(ProgressVariant::Circular)
    }

    /// The current value (clamped to 0..=max).
    pub fn value(mut self, value: f32) -> Self {
        self.value = if value.is_nan() {
            0.0
        } else {
            value.clamp(0.0, self.max)
        };
        self
    }

    /// The value that means complete (default 1).
    pub fn max(mut self, max: f32) -> Self {
        self.max = if max.is_nan() { 1.0 } else { max.max(0.001) };
        self.value = self.value.clamp(0.0, self.max);
        self
    }

    /// A buffered value (e.g. video buffering), painted over the track.
    pub fn buffer(mut self, buffer: f32) -> Self {
        self.buffer = Some(buffer.clamp(0.0, self.max));
        self
    }

    /// Indeterminate: animates without a value.
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Cycles the indeterminate indicator through primary, primary
    /// container, tertiary and tertiary container.
    pub fn four_color_enabled(mut self, enabled: bool) -> Self {
        self.four_color_enabled = enabled;
        self
    }

    /// Overrides the size.
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    /// Overrides the width.
    pub fn width(mut self, width: f32) -> Self {
        let mut size = self.resolved_size();
        size.x = width;
        self.size = Some(size);
        self
    }

    /// Overrides the height.
    pub fn height(mut self, height: f32) -> Self {
        let mut size = self.resolved_size();
        size.y = height;
        self.size = Some(size);
        self
    }

    /// The indicator colour (default `Primary`).
    pub fn active_color(mut self, color: Color32) -> Self {
        self.active_color = Some(color);
        self
    }

    /// The track colour (default `SecondaryContainer`).
    pub fn track_color(mut self, color: Color32) -> Self {
        self.track_color = Some(color);
        self
    }

    /// The buffer colour (default `PrimaryContainer`).
    pub fn buffer_color(mut self, color: Color32) -> Self {
        self.buffer_color = Some(color);
        self
    }

    /// Corner radius of a flat linear bar (default fully round).
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Stroke width of a circular indicator or a wavy line (default 4).
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = Some(width);
        self
    }

    /// Gap between the indicator and the track (default 4, 0 hides it).
    pub fn track_gap(mut self, gap: f32) -> Self {
        self.track_gap = Some(gap);
        self
    }

    /// Radius of the stop dot at the end of a determinate linear track
    /// (default 2, 0 hides it).
    pub fn stop_indicator_radius(mut self, radius: f32) -> Self {
        self.stop_indicator_radius = Some(radius);
        self
    }

    /// Colour of the stop dot (default: the indicator colour).
    pub fn stop_indicator_color(mut self, color: Color32) -> Self {
        self.stop_indicator_color = Some(color);
        self
    }

    /// Enables or disables four-color animation.
    #[deprecated(note = "Use four_color_enabled() instead")]
    pub fn four_color(mut self, enabled: bool) -> Self {
        self.four_color_enabled = enabled;
        self
    }

    /// The Expressive wavy indicator: the active part is drawn as a wave.
    pub fn wavy(mut self, wavy: bool) -> Self {
        self.wavy = wavy;
        self
    }

    /// The large size: a 10 dp linear bar, a 48 dp circle.
    pub fn large(mut self, large: bool) -> Self {
        self.large = large;
        self
    }

    /// Wave amplitude in points (defaults: 3 linear, 1.6 circular).
    pub fn amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = Some(amplitude);
        self
    }

    /// Wavelength in points (defaults: 40 linear, 20 linear indeterminate,
    /// 15 circular).
    pub fn wavelength(mut self, wavelength: f32) -> Self {
        self.wavelength = Some(wavelength);
        self
    }

    /// The accessible label (default "Progress").
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    fn resolved_size(&self) -> Vec2 {
        if let Some(size) = self.size {
            return size;
        }
        match self.variant {
            ProgressVariant::Linear => Vec2::new(
                LINEAR_WIDTH,
                if self.wavy || self.large {
                    LINEAR_LARGE_HEIGHT
                } else {
                    LINEAR_HEIGHT
                },
            ),
            ProgressVariant::Circular => Vec2::splat(if self.wavy || self.large {
                CIRCULAR_LARGE_SIZE
            } else {
                CIRCULAR_SIZE
            }),
        }
    }

    fn fraction(&self) -> f32 {
        (self.value / self.max).clamp(0.0, 1.0)
    }
}

impl Widget for MaterialProgress {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.resolved_size(), Sense::hover());
        let label = self.label.clone().unwrap_or_else(|| "Progress".to_owned());
        let value = (!self.indeterminate).then(|| f64::from(self.fraction()));
        response.widget_info(move || {
            let mut info = WidgetInfo::labeled(egui::Role::ProgressIndicator, true, label.clone());
            info.value = value;
            info
        });
        if !ui.is_rect_visible(rect) {
            return response;
        }
        let theme = MaterialTheme::of(ui.ctx());
        let time = ui.input(|i| i.time);
        let colors = Colors::resolve(&self, &theme, time);
        // The wavy amplitude eases in and out with progress.
        let amplitude_target = if !self.wavy {
            0.0
        } else if self.indeterminate {
            1.0
        } else {
            wavy_amplitude_fraction(self.fraction())
        };
        let amplitude = ui.ctx().animate_value_with_time(
            response.id.with("wave_amplitude"),
            amplitude_target,
            AMPLITUDE_ANIMATION_SECONDS,
        );
        let rtl = match theme.layout_direction {
            LayoutDirection::Rtl => true,
            LayoutDirection::Ltr => false,
        };
        let painter = Paint {
            ui,
            rect,
            rtl,
            colors,
        };
        match self.variant {
            ProgressVariant::Linear => self.paint_linear(&painter, time, amplitude),
            ProgressVariant::Circular => self.paint_circular(&painter, time, amplitude),
        }
        if self.indeterminate || self.wavy {
            ui.ctx().request_repaint();
        }
        response
    }
}

/// `WavyProgressIndicatorDefaults.indicatorAmplitude`: full amplitude
/// between 10% and 95% progress, none outside.
pub fn wavy_amplitude_fraction(progress: f32) -> f32 {
    if progress <= 0.1 || progress >= 0.95 {
        0.0
    } else {
        1.0
    }
}

#[derive(Clone, Copy, Debug)]
struct Colors {
    active: Color32,
    track: Color32,
    buffer: Color32,
    stop: Color32,
}

impl Colors {
    fn resolve(p: &MaterialProgress, theme: &MaterialTheme, time: f64) -> Self {
        let active = match (p.four_color_enabled && p.indeterminate, p.active_color) {
            (true, _) => {
                // Roughly every two seconds.
                let roles = [
                    ColorRole::Primary,
                    ColorRole::PrimaryContainer,
                    ColorRole::Tertiary,
                    ColorRole::TertiaryContainer,
                ];
                let cycle = ((time * 0.5).max(0.0) as u64 % 4) as usize;
                theme.color(roles.get(cycle).copied().unwrap_or(ColorRole::Primary))
            }
            (false, Some(color)) => color,
            (false, None) => theme.color(ColorRole::Primary),
        };
        Self {
            active,
            track: p
                .track_color
                .unwrap_or_else(|| theme.color(ColorRole::SecondaryContainer)),
            buffer: p
                .buffer_color
                .unwrap_or_else(|| theme.color(ColorRole::PrimaryContainer)),
            stop: p.stop_indicator_color.unwrap_or(active),
        }
    }
}

/// Where and with what an indicator is painted.
struct Paint<'u> {
    ui: &'u Ui,
    rect: Rect,
    rtl: bool,
    colors: Colors,
}

impl Paint<'_> {
    /// Screen x of a fraction of the width.
    fn x(&self, fraction: f32) -> f32 {
        if self.rtl {
            self.rect.max.x - fraction * self.rect.width()
        } else {
            self.rect.min.x + fraction * self.rect.width()
        }
    }

    /// A flat bar from `from` to `to` (fractions) with round caps clamped
    /// inside the rect (`drawLinearIndicator`).
    fn bar(&self, from: f32, to: f32, thickness: f32, radius: Option<f32>, color: Color32) {
        if (to - from).abs() <= 0.0 {
            return;
        }
        let w = self.rect.width();
        let cap = thickness / 2.0;
        let (lo, hi) = (from.min(to) * w, from.max(to) * w);
        let (lo, hi) = if thickness > w {
            (lo, hi)
        } else {
            (lo.clamp(cap, w - cap) - cap, hi.clamp(cap, w - cap) + cap)
        };
        let (x0, x1) = (self.x(lo / w.max(1.0)), self.x(hi / w.max(1.0)));
        let cy = self.rect.center().y;
        let bar = Rect::from_x_y_ranges(x0.min(x1)..=x0.max(x1), (cy - cap)..=(cy + cap));
        let r = radius.unwrap_or(cap);
        self.ui.painter().rect_filled(
            bar,
            CornerRadius::same(r.clamp(0.0, 255.0).round() as u8),
            color,
        );
    }

    /// A wavy line from `from` to `to` (fractions), round-capped.
    #[expect(clippy::too_many_arguments, reason = "private painting helper")]
    fn wave(
        &self,
        from: f32,
        to: f32,
        thickness: f32,
        amplitude: f32,
        wavelength: f32,
        phase: f32,
        color: Color32,
    ) {
        if (to - from).abs() <= 0.0 {
            return;
        }
        let w = self.rect.width();
        let cap = thickness / 2.0;
        let lo = (from.min(to) * w).clamp(cap, (w - cap).max(cap));
        let hi = (from.max(to) * w).clamp(cap, (w - cap).max(cap));
        let cy = self.rect.center().y;
        let steps = (((hi - lo) / 2.0).ceil() as usize).clamp(1, 2000);
        let points: Vec<Pos2> = (0..=steps)
            .map(|i| {
                let d = lo + (hi - lo) * i as f32 / steps as f32;
                let y = cy + amplitude * (2.0 * PI * (d - phase) / wavelength.max(1.0)).sin();
                Pos2::new(self.x(d / w.max(1.0)), y)
            })
            .collect();
        stroke_round(self.ui, points, thickness, color);
    }
}

/// A polyline stroke with round caps.
fn stroke_round(ui: &Ui, points: Vec<Pos2>, thickness: f32, color: Color32) {
    let (Some(first), Some(last)) = (points.first().copied(), points.last().copied()) else {
        return;
    };
    let painter = ui.painter();
    painter.circle_filled(first, thickness / 2.0, color);
    painter.circle_filled(last, thickness / 2.0, color);
    if points.len() >= 2 {
        painter.add(PathShape {
            points,
            closed: false,
            fill: Color32::TRANSPARENT,
            stroke: PathStroke::new(thickness, color),
        });
    }
}

/// A circular arc (degrees, 0 = 3 o'clock, clockwise) with round caps,
/// optionally wavy: `wave` is (amplitude, number of waves around, phase in
/// radians).
#[expect(clippy::too_many_arguments, reason = "private painting helper")]
fn arc(
    ui: &Ui,
    center: Pos2,
    radius: f32,
    start_deg: f32,
    sweep_deg: f32,
    thickness: f32,
    wave: Option<(f32, f32, f32)>,
    color: Color32,
) {
    if sweep_deg <= 0.0 {
        return;
    }
    let steps = ((sweep_deg / 3.0).ceil() as usize).clamp(2, 240);
    let points: Vec<Pos2> = (0..=steps)
        .map(|i| {
            let angle = (start_deg + sweep_deg * i as f32 / steps as f32).to_radians();
            let r = match wave {
                Some((amplitude, waves, phase)) => {
                    radius + amplitude * (waves * angle + phase).sin()
                }
                None => radius,
            };
            center + Vec2::angled(angle) * r
        })
        .collect();
    stroke_round(ui, points, thickness, color);
}

/// A keyframed 0 to 1 line position: 0 until `delay`, eased over
/// `duration`, then 1.
fn line_position(t_ms: f32, delay: f32, duration: f32) -> f32 {
    if t_ms <= delay {
        0.0
    } else if t_ms >= delay + duration {
        1.0
    } else {
        CubicBezier::EMPHASIZED_ACCELERATE.transform((t_ms - delay) / duration)
    }
}

/// The four indeterminate linear positions (first head, first tail, second
/// head, second tail) at `time` seconds.
pub(crate) fn linear_indeterminate(time: f64) -> [f32; 4] {
    let t = ((time.max(0.0) * 1000.0) % LINEAR_CYCLE_MS) as f32;
    LINEAR_LINES.map(|(delay, duration)| line_position(t, delay, duration))
}

/// The indeterminate circular rotation (degrees) and sweep (fraction of a
/// turn) at `time` seconds.
pub(crate) fn circular_indeterminate(time: f64) -> (f32, f32) {
    let t = ((time.max(0.0) * 1000.0) % CIRCULAR_CYCLE_MS) as f32;
    let cycle = CIRCULAR_CYCLE_MS as f32;
    let global = CIRCULAR_GLOBAL_ROTATION * t / cycle;
    // A linear quarter turn over 300 ms at the start of each 1500 ms.
    let quarter = (t / CIRCULAR_ROTATION_DELAY_MS).floor();
    let within = t - quarter * CIRCULAR_ROTATION_DELAY_MS;
    let additional = 90.0 * quarter + 90.0 * (within / CIRCULAR_ROTATION_DURATION_MS).min(1.0);
    let half = cycle / 2.0;
    let sweep = if t <= half {
        CIRCULAR_MIN_PROGRESS + (CIRCULAR_MAX_PROGRESS - CIRCULAR_MIN_PROGRESS) * t / half
    } else {
        let eased = CubicBezier::STANDARD.transform((t - half) / half);
        CIRCULAR_MAX_PROGRESS + (CIRCULAR_MIN_PROGRESS - CIRCULAR_MAX_PROGRESS) * eased
    };
    ((global + additional.min(360.0)) % 360.0, sweep)
}

/// The wave's phase in points: one wavelength per second.
fn wave_phase(time: f64, wavelength: f32) -> f32 {
    (time.max(0.0).fract() as f32) * wavelength
}

impl MaterialProgress {
    fn paint_linear(&self, p: &Paint<'_>, time: f64, amplitude_fraction: f32) {
        let w = p.rect.width().max(1.0);
        let h = p.rect.height();
        let thickness = if self.wavy {
            self.stroke_width.unwrap_or(THICKNESS).min(h)
        } else {
            h
        };
        let gap = self.track_gap.unwrap_or(TRACK_GAP);
        // Round caps eat into the gap (`adjustedGapSize`).
        let gap_fraction = if gap > 0.0 {
            (gap + thickness) / w
        } else {
            0.0
        };
        // Full amplitude fills the height: (10 - 4) / 2 = 3 dp, the token.
        let amplitude =
            amplitude_fraction * self.amplitude.unwrap_or(((h - thickness) / 2.0).max(0.0));
        let wavelength = self.wavelength.unwrap_or(if self.indeterminate {
            LINEAR_INDETERMINATE_WAVELENGTH
        } else {
            LINEAR_WAVELENGTH
        });
        let phase = wave_phase(time, wavelength);
        let radius = self.border_radius;
        let active = |from: f32, to: f32| {
            if self.wavy && amplitude > 0.0 {
                p.wave(
                    from,
                    to,
                    thickness,
                    amplitude,
                    wavelength,
                    phase,
                    p.colors.active,
                );
            } else {
                p.bar(from, to, thickness, radius, p.colors.active);
            }
        };
        let track = |from: f32, to: f32| p.bar(from, to, thickness, radius, p.colors.track);

        if self.indeterminate {
            let [head1, tail1, head2, tail2] = linear_indeterminate(time);
            if head1 < 1.0 - gap_fraction {
                let start = if head1 > 0.0 {
                    head1 + gap_fraction
                } else {
                    0.0
                };
                track(start, 1.0);
            }
            if head1 - tail1 > 0.0 {
                active(tail1, head1);
            }
            if tail1 > gap_fraction {
                let start = if head2 > 0.0 {
                    head2 + gap_fraction
                } else {
                    0.0
                };
                let end = if tail1 < 1.0 {
                    tail1 - gap_fraction
                } else {
                    1.0
                };
                if end > start {
                    track(start, end);
                }
            }
            if head2 - tail2 > 0.0 {
                active(tail2, head2);
            }
            if tail2 > gap_fraction {
                let end = if tail2 < 1.0 {
                    tail2 - gap_fraction
                } else {
                    1.0
                };
                track(0.0, end);
            }
            return;
        }

        let progress = self.fraction();
        let track_start = progress + progress.min(gap_fraction);
        if track_start <= 1.0 {
            track(track_start, 1.0);
        }
        if let Some(buffer) = self.buffer {
            let buffer = (buffer / self.max).clamp(0.0, 1.0);
            if buffer > track_start {
                p.bar(track_start, buffer, thickness, radius, p.colors.buffer);
            }
        }
        active(0.0, progress);
        // `drawStopIndicator`: a dot at the end of the track.
        let stop_size = self
            .stop_indicator_radius
            .map_or(STOP_SIZE, |r| 2.0 * r)
            .min(thickness);
        if stop_size > 0.0 {
            let offset = ((thickness - stop_size) / 2.0).min(STOP_MAX_OFFSET);
            let d = w - stop_size / 2.0 - offset;
            let center = Pos2::new(p.x(d / w), p.rect.center().y);
            p.ui.painter()
                .circle_filled(center, stop_size / 2.0, p.colors.stop);
        }
    }

    fn paint_circular(&self, p: &Paint<'_>, time: f64, amplitude_fraction: f32) {
        let thickness = self.stroke_width.unwrap_or(THICKNESS);
        let side = p.rect.width().min(p.rect.height());
        let amplitude = amplitude_fraction * self.amplitude.unwrap_or(CIRCULAR_WAVE_AMPLITUDE);
        let max_amplitude = if self.wavy {
            self.amplitude.unwrap_or(CIRCULAR_WAVE_AMPLITUDE)
        } else {
            0.0
        };
        let radius = (side - thickness) / 2.0 - max_amplitude;
        if radius <= 0.0 {
            return;
        }
        let center = p.rect.center();
        let gap = self.track_gap.unwrap_or(TRACK_GAP);
        // `gapSizeSweep`, with the round caps added to the gap.
        let gap_sweep = if gap > 0.0 {
            (gap + thickness) / (PI * side) * 360.0
        } else {
            0.0
        };
        let wavelength = self.wavelength.unwrap_or(CIRCULAR_WAVELENGTH);
        let waves = (2.0 * PI * radius / wavelength.max(1.0)).round().max(1.0);
        let wave_phase_radians = -2.0 * PI * (time.max(0.0).fract() as f32);
        let wave = (self.wavy && amplitude > 0.0).then_some((amplitude, waves, wave_phase_radians));

        let (start, sweep, track_color) = if self.indeterminate {
            let (rotation, sweep) = circular_indeterminate(time);
            // The indeterminate track is transparent unless set.
            (rotation, sweep * 360.0, self.track_color)
        } else {
            // Starts at 12 o'clock.
            (270.0, self.fraction() * 360.0, Some(p.colors.track))
        };
        if let Some(track_color) = track_color {
            let track_start = start + sweep + sweep.min(gap_sweep);
            let track_sweep = 360.0 - sweep - sweep.min(gap_sweep) * 2.0;
            arc(
                p.ui,
                center,
                radius,
                track_start,
                track_sweep,
                thickness,
                None,
                track_color,
            );
        }
        arc(
            p.ui,
            center,
            radius,
            start,
            sweep,
            thickness,
            wave,
            p.colors.active,
        );
    }
}

/// Shorthand for [`MaterialProgress::linear`].
pub fn linear_progress() -> MaterialProgress {
    MaterialProgress::linear()
}

/// Shorthand for [`MaterialProgress::circular`].
pub fn circular_progress() -> MaterialProgress {
    MaterialProgress::circular()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_lines_follow_the_compose_keyframes() {
        let at = |ms: f64| linear_indeterminate(ms / 1000.0);
        assert_eq!(at(0.0), [0.0, 0.0, 0.0, 0.0], "all lines start at 0");
        let [head1, tail1, head2, tail2] = at(500.0);
        assert!(head1 > 0.0 && head1 < 1.0, "first head moving at 500 ms");
        assert!(tail1 > 0.0 && tail1 < head1, "first tail behind it");
        assert_eq!((head2, tail2), (0.0, 0.0), "second line not started");
        let [h1, t1, h2, t2] = at(1600.0);
        assert_eq!(
            (h1, t1, h2),
            (1.0, 1.0, 1.0),
            "done by 1600 ms but the last tail"
        );
        assert!(t2 > 0.0 && t2 < 1.0, "second tail still moving");
        assert_eq!(at(1750.0), at(0.0), "repeats every 1750 ms");
    }

    #[test]
    fn long_uptimes_keep_precision() {
        // About twelve days of uptime: f32 seconds would only resolve 0.125 s.
        let base = 1_050_000.0; // 600 000 linear cycles of 1.75 s
        assert_eq!(
            linear_indeterminate(base + 0.5),
            linear_indeterminate(0.5),
            "same phase after a whole number of cycles"
        );
    }

    #[test]
    fn circular_sweep_breathes_between_the_compose_limits() {
        let (_, start) = circular_indeterminate(0.0);
        let (_, peak) = circular_indeterminate(3.0);
        assert!(
            (start - CIRCULAR_MIN_PROGRESS).abs() < 1e-5,
            "starts at 10%"
        );
        assert!((peak - CIRCULAR_MAX_PROGRESS).abs() < 1e-5, "peaks at 87%");
        let (r0, _) = circular_indeterminate(0.0);
        let (r1, _) = circular_indeterminate(0.3);
        // 0.3 s: 1080 * 0.05 = 54 degrees global plus the 90 degree jump.
        assert!(
            (r1 - r0 - 144.0).abs() < 1e-3,
            "rotation after 300 ms: {r1}"
        );
    }

    #[test]
    fn amplitude_is_off_near_the_ends() {
        assert_eq!(wavy_amplitude_fraction(0.05), 0.0, "off at 5%");
        assert_eq!(wavy_amplitude_fraction(0.5), 1.0, "on at 50%");
        assert_eq!(wavy_amplitude_fraction(0.97), 0.0, "off at 97%");
    }
}
