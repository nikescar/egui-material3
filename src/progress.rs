use crate::get_global_color;
use egui::{Color32, CornerRadius, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use std::f32::consts::PI;

// Animation duration constants (from Flutter reference)
const INDETERMINATE_LINEAR_DURATION_MS: f32 = 1800.0;
const INDETERMINATE_CIRCULAR_DURATION_MS: f32 = 1333.0 * 2.222;

// Track gap ramp-down threshold: below this progress value, the gap is
// scaled proportionally to prevent it from appearing abruptly at 0%.
const TRACK_GAP_RAMP_DOWN_THRESHOLD: f32 = 0.01;

/// Material Design progress indicator variants
#[derive(Clone, Copy, PartialEq)]
pub enum ProgressVariant {
    /// Linear progress bar - horizontal bar showing progress
    Linear,
    /// Circular progress indicator - circular arc showing progress
    Circular,
}

/// Material Design progress indicator component
///
/// Progress indicators inform users about the status of ongoing processes, such as
/// loading an app, submitting a form, or saving updates. They communicate an app's
/// state and indicate available actions.
///
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// // Linear progress with value
/// ui.add(MaterialProgress::linear()
///     .value(0.65)
///     .size(Vec2::new(300.0, 6.0)));
///
/// // Circular progress with value
/// ui.add(MaterialProgress::circular()
///     .value(0.8)
///     .size(Vec2::splat(64.0)));
///
/// // Indeterminate linear progress (loading)
/// ui.add(MaterialProgress::linear()
///     .indeterminate(true));
///
/// // Buffered linear progress (like video loading)
/// ui.add(MaterialProgress::linear()
///     .value(0.3)
///     .buffer(0.6));
///
/// // Customized colors and style
/// ui.add(MaterialProgress::linear()
///     .value(0.5)
///     .active_color(Color32::RED)
///     .track_color(Color32::LIGHT_GRAY)
///     .track_gap(4.0)
///     .stop_indicator_radius(2.0));
/// # });
/// ```
///
/// ## Material Design Spec
/// - Linear: 4dp height (default), variable width
/// - Circular: 48dp diameter (default), 4dp stroke width
/// - Colors: Primary color for progress, secondaryContainer for track
/// - Animation: Smooth transitions, indeterminate animations
/// - Corner radius: pill-shaped for linear progress
/// - Track gap: 4dp between indicator and track (M3 2024)
/// - Stop indicator: 2dp radius dot at track end (linear determinate)
pub struct MaterialProgress {
    /// Type of progress indicator (linear or circular)
    variant: ProgressVariant,
    /// Current progress value (0.0 to max)
    value: f32,
    /// Maximum value for progress calculation
    max: f32,
    /// Optional buffer value for buffered progress (e.g., video loading)
    buffer: Option<f32>,
    /// Whether to show indeterminate progress animation
    indeterminate: bool,
    /// Whether to use four-color animation for indeterminate progress
    four_color_enabled: bool,
    /// Size of the progress indicator
    size: Vec2,
    /// Custom indicator/active color (default: primary)
    active_color: Option<Color32>,
    /// Custom track color (default: secondaryContainer)
    track_color: Option<Color32>,
    /// Custom buffer color (default: primaryContainer)
    buffer_color: Option<Color32>,
    /// Corner radius for linear progress (default: height / 2.0)
    border_radius: Option<f32>,
    /// Stroke width for circular progress (default: 4.0)
    stroke_width: Option<f32>,
    /// Gap between indicator and track (default: 4.0)
    track_gap: Option<f32>,
    /// Radius of the stop indicator dot at track end (default: 2.0, set 0 to hide)
    stop_indicator_radius: Option<f32>,
    /// Color of the stop indicator dot (default: primary)
    stop_indicator_color: Option<Color32>,
}

impl MaterialProgress {
    /// Create a new progress indicator with the specified variant
    pub fn new(variant: ProgressVariant) -> Self {
        Self {
            variant,
            value: 0.0,
            max: 1.0,
            buffer: None,
            indeterminate: false,
            four_color_enabled: false,
            size: match variant {
                ProgressVariant::Linear => Vec2::new(200.0, 4.0),
                ProgressVariant::Circular => Vec2::splat(48.0),
            },
            active_color: None,
            track_color: None,
            buffer_color: None,
            border_radius: None,
            stroke_width: None,
            track_gap: None,
            stop_indicator_radius: None,
            stop_indicator_color: None,
        }
    }

    /// Create a linear progress bar
    pub fn linear() -> Self {
        Self::new(ProgressVariant::Linear)
    }

    /// Create a circular progress indicator
    pub fn circular() -> Self {
        Self::new(ProgressVariant::Circular)
    }

    /// Set the current progress value (clamped between 0.0 and max)
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, self.max);
        self
    }

    /// Set the maximum value for progress calculation (default: 1.0)
    pub fn max(mut self, max: f32) -> Self {
        self.max = max.max(0.001);
        self.value = self.value.clamp(0.0, self.max);
        self
    }

    /// Set the buffer value for buffered progress (e.g., video buffering)
    pub fn buffer(mut self, buffer: f32) -> Self {
        self.buffer = Some(buffer.clamp(0.0, self.max));
        self
    }

    /// Enable or disable indeterminate progress animation
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Enable or disable four-color animation for indeterminate progress
    pub fn four_color_enabled(mut self, enabled: bool) -> Self {
        self.four_color_enabled = enabled;
        self
    }

    /// Set the size of the progress indicator
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    /// Set the width of the progress indicator
    pub fn width(mut self, width: f32) -> Self {
        self.size.x = width;
        self
    }

    /// Set the height of the progress indicator
    pub fn height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }

    /// Set the indicator/active color (default: theme primary)
    pub fn active_color(mut self, color: Color32) -> Self {
        self.active_color = Some(color);
        self
    }

    /// Set the track background color (default: theme secondaryContainer)
    pub fn track_color(mut self, color: Color32) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Set the buffer indicator color (default: theme primaryContainer)
    pub fn buffer_color(mut self, color: Color32) -> Self {
        self.buffer_color = Some(color);
        self
    }

    /// Set the corner radius for linear progress (default: height / 2.0 for pill shape)
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Set the stroke width for circular progress (default: 4.0)
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = Some(width);
        self
    }

    /// Set the gap between indicator and track (default: 4.0, set 0 to hide)
    pub fn track_gap(mut self, gap: f32) -> Self {
        self.track_gap = Some(gap);
        self
    }

    /// Set the stop indicator dot radius for linear determinate (default: 2.0, set 0 to hide)
    pub fn stop_indicator_radius(mut self, radius: f32) -> Self {
        self.stop_indicator_radius = Some(radius);
        self
    }

    /// Set the stop indicator dot color (default: theme primary)
    pub fn stop_indicator_color(mut self, color: Color32) -> Self {
        self.stop_indicator_color = Some(color);
        self
    }

    /// Enable or disable four-color animation (deprecated, use four_color_enabled)
    #[deprecated(note = "Use four_color_enabled() instead")]
    pub fn four_color(mut self, enabled: bool) -> Self {
        self.four_color_enabled = enabled;
        self
    }
}

impl Widget for MaterialProgress {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());

        match self.variant {
            ProgressVariant::Linear => self.render_linear(ui, rect),
            ProgressVariant::Circular => self.render_circular(ui, rect),
        }

        response
    }
}

// --- Cubic bezier interpolation for animation curves ---

/// Evaluate a cubic bezier curve at parameter t.
/// Control points: (0,0), (x1,y1), (x2,y2), (1,1)
fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    // Use Newton's method to find the t parameter for the given x value
    // then evaluate y at that t.
    // For animation curves, input t is the x-axis (time fraction).
    let mut guess = t;
    for _ in 0..8 {
        let x = cubic_eval(x1, x2, guess) - t;
        if x.abs() < 1e-6 {
            break;
        }
        let dx = cubic_eval_derivative(x1, x2, guess);
        if dx.abs() < 1e-6 {
            break;
        }
        guess -= x / dx;
        guess = guess.clamp(0.0, 1.0);
    }
    cubic_eval(y1, y2, guess)
}

fn cubic_eval(a: f32, b: f32, t: f32) -> f32 {
    // Cubic bezier with points 0, a, b, 1
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    3.0 * mt2 * t * a + 3.0 * mt * t2 * b + t3
}

fn cubic_eval_derivative(a: f32, b: f32, t: f32) -> f32 {
    let mt = 1.0 - t;
    3.0 * mt * mt * a + 6.0 * mt * t * (b - a) + 3.0 * t * t * (1.0 - b)
}

/// Interval transform: maps t from [begin..end] to [0..1], clamped.
fn interval(t: f32, begin: f32, end: f32) -> f32 {
    ((t - begin) / (end - begin)).clamp(0.0, 1.0)
}

// --- Linear indeterminate animation curves (from Flutter) ---
// Duration: 1800ms

fn line1_head(t: f32) -> f32 {
    let local_t = interval(t, 0.0, 750.0 / INDETERMINATE_LINEAR_DURATION_MS);
    cubic_bezier(0.2, 0.0, 0.8, 1.0, local_t)
}

fn line1_tail(t: f32) -> f32 {
    let local_t = interval(t, 333.0 / INDETERMINATE_LINEAR_DURATION_MS, 1083.0 / INDETERMINATE_LINEAR_DURATION_MS);
    cubic_bezier(0.4, 0.0, 1.0, 1.0, local_t)
}

fn line2_head(t: f32) -> f32 {
    let local_t = interval(t, 1000.0 / INDETERMINATE_LINEAR_DURATION_MS, 1567.0 / INDETERMINATE_LINEAR_DURATION_MS);
    cubic_bezier(0.0, 0.0, 0.65, 1.0, local_t)
}

fn line2_tail(t: f32) -> f32 {
    let local_t = interval(t, 1267.0 / INDETERMINATE_LINEAR_DURATION_MS, 1800.0 / INDETERMINATE_LINEAR_DURATION_MS);
    cubic_bezier(0.10, 0.0, 0.45, 1.0, local_t)
}

// --- Circular indeterminate animation (from Flutter) ---

const CIRCULAR_PATH_COUNT: f32 = 3.0;
const CIRCULAR_ROTATION_COUNT: f32 = CIRCULAR_PATH_COUNT * 5.0 / 6.0;

fn sawtooth(t: f32, count: f32) -> f32 {
    (t * count).fract()
}

fn circular_head_value(t: f32) -> f32 {
    let st = sawtooth(t, CIRCULAR_PATH_COUNT);
    interval(st, 0.0, 0.5)
}

fn circular_tail_value(t: f32) -> f32 {
    let st = sawtooth(t, CIRCULAR_PATH_COUNT);
    interval(st, 0.5, 1.0)
}

fn circular_offset_value(t: f32) -> f32 {
    sawtooth(t, CIRCULAR_PATH_COUNT)
}

fn circular_rotation_value(t: f32) -> f32 {
    sawtooth(t, CIRCULAR_ROTATION_COUNT)
}

impl MaterialProgress {
    /// Resolve colors with fallback to theme defaults
    fn resolve_active_color(&self) -> Color32 {
        self.active_color.unwrap_or_else(|| get_global_color("primary"))
    }

    fn resolve_track_color(&self) -> Color32 {
        self.track_color.unwrap_or_else(|| get_global_color("secondaryContainer"))
    }

    fn resolve_buffer_color(&self) -> Color32 {
        self.buffer_color.unwrap_or_else(|| get_global_color("primaryContainer"))
    }

    fn resolve_stop_indicator_color(&self) -> Color32 {
        self.stop_indicator_color.unwrap_or_else(|| get_global_color("primary"))
    }

    fn resolve_border_radius(&self, rect_height: f32) -> f32 {
        self.border_radius.unwrap_or(rect_height / 2.0)
    }

    fn resolve_stroke_width(&self) -> f32 {
        self.stroke_width.unwrap_or(4.0)
    }

    fn resolve_track_gap(&self) -> f32 {
        self.track_gap.unwrap_or(4.0)
    }

    fn resolve_stop_indicator_radius(&self, rect_height: f32) -> f32 {
        let r = self.stop_indicator_radius.unwrap_or(2.0);
        r.min(rect_height / 2.0)
    }

    /// Get effective track gap fraction scaled proportionally near 0%.
    fn effective_track_gap_fraction(current_value: f32, track_gap_fraction: f32) -> f32 {
        track_gap_fraction
            * current_value.clamp(0.0, TRACK_GAP_RAMP_DOWN_THRESHOLD)
            / TRACK_GAP_RAMP_DOWN_THRESHOLD
    }

    /// Get the four-color cycle color based on animation time
    fn get_four_color(&self, time: f32) -> Color32 {
        let colors = [
            get_global_color("primary"),
            get_global_color("primaryContainer"),
            get_global_color("tertiary"),
            get_global_color("tertiaryContainer"),
        ];
        let cycle = (time * 0.5) as usize % 4; // Change color roughly every 2 seconds
        colors[cycle]
    }

    fn render_linear(&self, ui: &mut Ui, rect: Rect) {
        let active_color = if self.four_color_enabled && self.indeterminate {
            let time = ui.input(|i| i.time) as f32;
            self.get_four_color(time)
        } else {
            self.resolve_active_color()
        };
        let track_color = self.resolve_track_color();
        let buffer_color = self.resolve_buffer_color();
        let border_radius = self.resolve_border_radius(rect.height());
        let rounding = CornerRadius::same(border_radius as u8);
        let track_gap = self.resolve_track_gap();
        let track_gap_fraction = track_gap / rect.width();

        if self.indeterminate {
            // Flutter-style dual-bar indeterminate animation
            let time = ui.input(|i| i.time) as f32;
            let cycle_duration = INDETERMINATE_LINEAR_DURATION_MS / 1000.0;
            let animation_value = ((time % cycle_duration) / cycle_duration).clamp(0.0, 1.0);

            let first_line_head = line1_head(animation_value);
            let first_line_tail = line1_tail(animation_value);
            let second_line_head = line2_head(animation_value);
            let second_line_tail = line2_tail(animation_value);

            // Draw track before line 1 (right side of line 1)
            if first_line_head < 1.0 - track_gap_fraction {
                let track_start = if first_line_head > 0.0 {
                    first_line_head + Self::effective_track_gap_fraction(first_line_head, track_gap_fraction)
                } else {
                    0.0
                };
                self.draw_linear_segment(ui, rect, track_start, 1.0, track_color, rounding);
            }

            // Draw line 1
            if first_line_head - first_line_tail > 0.0 {
                self.draw_linear_segment(ui, rect, first_line_tail, first_line_head, active_color, rounding);
            }

            // Draw track between line 1 and line 2
            if first_line_tail > track_gap_fraction {
                let track_start = if second_line_head > 0.0 {
                    second_line_head + Self::effective_track_gap_fraction(second_line_head, track_gap_fraction)
                } else {
                    0.0
                };
                let track_end = if first_line_tail < 1.0 {
                    first_line_tail - Self::effective_track_gap_fraction(1.0 - first_line_tail, track_gap_fraction)
                } else {
                    1.0
                };
                if track_end > track_start {
                    self.draw_linear_segment(ui, rect, track_start, track_end, track_color, rounding);
                }
            }

            // Draw line 2
            if second_line_head - second_line_tail > 0.0 {
                self.draw_linear_segment(ui, rect, second_line_tail, second_line_head, active_color, rounding);
            }

            // Draw track after line 2 (left side of line 2)
            if second_line_tail > track_gap_fraction {
                let track_end = if second_line_tail < 1.0 {
                    second_line_tail - Self::effective_track_gap_fraction(1.0 - second_line_tail, track_gap_fraction)
                } else {
                    1.0
                };
                self.draw_linear_segment(ui, rect, 0.0, track_end, track_color, rounding);
            }

            // If both lines haven't started yet, draw full track
            if first_line_head <= 0.0 && second_line_head <= 0.0 {
                self.draw_linear_segment(ui, rect, 0.0, 1.0, track_color, rounding);
            }

            ui.ctx().request_repaint();
        } else {
            // Determinate progress
            let progress = (self.value / self.max).clamp(0.0, 1.0);

            // Draw track with gap
            let track_start = if track_gap_fraction > 0.0 && progress > 0.0 {
                progress + Self::effective_track_gap_fraction(progress, track_gap_fraction)
            } else {
                0.0
            };
            if track_start < 1.0 {
                self.draw_linear_segment(ui, rect, track_start, 1.0, track_color, rounding);
            }

            // Draw stop indicator at the end of the track
            let stop_radius = self.resolve_stop_indicator_radius(rect.height());
            if stop_radius > 0.0 {
                let stop_color = self.resolve_stop_indicator_color();
                let max_radius = rect.height() / 2.0;
                let center = Pos2::new(
                    rect.max.x - max_radius,
                    rect.min.y + max_radius,
                );
                ui.painter().circle_filled(center, stop_radius, stop_color);
            }

            // Draw buffer if present
            if let Some(buffer) = self.buffer {
                let buffer_progress = (buffer / self.max).clamp(0.0, 1.0);
                if buffer_progress > progress {
                    let buffer_start = if track_gap_fraction > 0.0 && progress > 0.0 {
                        progress + Self::effective_track_gap_fraction(progress, track_gap_fraction)
                    } else {
                        progress
                    };
                    if buffer_progress > buffer_start {
                        self.draw_linear_segment(ui, rect, buffer_start, buffer_progress, buffer_color, rounding);
                    }
                }
            }

            // Draw progress bar
            if progress > 0.0 {
                self.draw_linear_segment(ui, rect, 0.0, progress, active_color, rounding);
            }
        }
    }

    fn draw_linear_segment(
        &self,
        ui: &mut Ui,
        rect: Rect,
        start_fraction: f32,
        end_fraction: f32,
        color: Color32,
        rounding: CornerRadius,
    ) {
        if end_fraction - start_fraction <= 0.0 {
            return;
        }

        let left = rect.min.x + start_fraction * rect.width();
        let right = rect.min.x + end_fraction * rect.width();
        let segment_rect = Rect::from_min_max(
            Pos2::new(left, rect.min.y),
            Pos2::new(right, rect.max.y),
        );

        ui.painter().rect_filled(segment_rect, rounding, color);
    }

    fn render_circular(&self, ui: &mut Ui, rect: Rect) {
        let stroke_width = self.resolve_stroke_width();
        let center = rect.center();
        let radius = (rect.width().min(rect.height()) / 2.0) - stroke_width / 2.0;
        let track_color = self.resolve_track_color();
        let track_gap = self.resolve_track_gap();

        if self.indeterminate {
            let time = ui.input(|i| i.time) as f32;
            let cycle_duration = INDETERMINATE_CIRCULAR_DURATION_MS / 1000.0;
            let animation_value = ((time % cycle_duration) / cycle_duration).clamp(0.0, 1.0);

            let head_value = circular_head_value(animation_value);
            let tail_value = circular_tail_value(animation_value);
            let offset_value = circular_offset_value(animation_value);
            let rotation_value = circular_rotation_value(animation_value);

            // Draw track (full circle, no gap for indeterminate)
            ui.painter().circle_stroke(center, radius, Stroke::new(stroke_width, track_color));

            // Calculate arc start and sweep (from Flutter reference)
            let arc_start = -PI / 2.0
                + tail_value * 3.0 / 2.0 * PI
                + rotation_value * PI * 2.0
                + offset_value * 0.5 * PI;
            let arc_sweep = (head_value * 3.0 / 2.0 * PI - tail_value * 3.0 / 2.0 * PI).max(0.001);

            let active_color = if self.four_color_enabled {
                self.get_four_color(time)
            } else {
                self.resolve_active_color()
            };

            self.draw_arc(
                ui,
                center,
                radius,
                arc_start,
                arc_start + arc_sweep,
                stroke_width,
                active_color,
            );

            ui.ctx().request_repaint();
        } else {
            let progress = (self.value / self.max).clamp(0.0, 1.0);
            let active_color = self.resolve_active_color();

            let epsilon = 0.001;
            let two_pi = 2.0 * PI;

            if track_gap > 0.0 && progress > epsilon {
                // Draw track with gap (from Flutter reference)
                let arc_radius = radius;
                let stroke_radius = stroke_width / arc_radius;
                let gap_radius = track_gap / arc_radius;
                let start_gap = stroke_radius + gap_radius;
                let end_gap = if progress < epsilon { start_gap } else { start_gap * 2.0 };
                let track_start = -PI / 2.0 + start_gap;
                let track_sweep = (two_pi - progress.clamp(0.0, 1.0) * two_pi - end_gap).max(0.0);

                if track_sweep > 0.0 {
                    // Draw the track arc on the opposite side (flipped)
                    let flipped_start = PI - track_start;
                    self.draw_arc(
                        ui,
                        center,
                        radius,
                        flipped_start,
                        flipped_start - track_sweep,
                        stroke_width,
                        track_color,
                    );
                }
            } else {
                // Full track circle (no gap)
                ui.painter().circle_stroke(center, radius, Stroke::new(stroke_width, track_color));
            }

            // Draw progress arc
            if progress > 0.0 {
                let arc_length = two_pi * progress - epsilon;
                self.draw_arc(
                    ui,
                    center,
                    radius,
                    -PI / 2.0,
                    -PI / 2.0 + arc_length,
                    stroke_width,
                    active_color,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_arc(
        &self,
        ui: &mut Ui,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        stroke_width: f32,
        color: Color32,
    ) {
        let segments = 48;
        let angle_step = (end_angle - start_angle) / segments as f32;

        for i in 0..segments {
            let angle1 = start_angle + i as f32 * angle_step;
            let angle2 = start_angle + (i + 1) as f32 * angle_step;

            let point1 = Pos2::new(
                center.x + radius * angle1.cos(),
                center.y + radius * angle1.sin(),
            );
            let point2 = Pos2::new(
                center.x + radius * angle2.cos(),
                center.y + radius * angle2.sin(),
            );

            ui.painter()
                .line_segment([point1, point2], Stroke::new(stroke_width, color));
        }
    }
}

pub fn linear_progress() -> MaterialProgress {
    MaterialProgress::linear()
}

pub fn circular_progress() -> MaterialProgress {
    MaterialProgress::circular()
}
