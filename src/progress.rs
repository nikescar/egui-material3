use crate::get_global_color;
use eframe::egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use std::f32::consts::PI;

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
/// # });
/// ```
///
/// ## Material Design Spec
/// - Linear: 4dp height (default), variable width
/// - Circular: 48dp diameter (default), 4dp stroke width
/// - Colors: Primary color for progress, surfaceVariant for track
/// - Animation: Smooth transitions, indeterminate animations
/// - Corner radius: 2dp for linear progress
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
}

impl MaterialProgress {
    /// Create a new progress indicator with the specified variant
    ///
    /// ## Parameters
    /// - `variant`: Whether to create a linear or circular progress indicator
    ///
    /// ## Returns
    /// A new MaterialProgress instance with default settings
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
        }
    }

    /// Create a linear progress bar
    ///
    /// Linear progress indicators display progress along a horizontal line.
    /// Best for showing progress of tasks with known duration.
    ///
    /// ## Material Design Usage
    /// - File downloads/uploads
    /// - Form submission progress  
    /// - Loading content with known steps
    pub fn linear() -> Self {
        Self::new(ProgressVariant::Linear)
    }

    /// Create a circular progress indicator
    ///
    /// Circular progress indicators display progress along a circular path.
    /// Best for compact spaces or indeterminate progress.
    ///
    /// ## Material Design Usage
    /// - Loading states in buttons or cards
    /// - Refreshing content
    /// - Background processing
    pub fn circular() -> Self {
        Self::new(ProgressVariant::Circular)
    }

    /// Set the current progress value
    ///
    /// ## Parameters
    /// - `value`: Progress value (will be clamped between 0.0 and max)
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, self.max);
        self
    }

    /// Set the maximum value for progress calculation
    ///
    /// The progress percentage will be calculated as value/max.
    ///
    /// ## Parameters
    /// - `max`: Maximum value (default is 1.0 for 0-100% range)
    pub fn max(mut self, max: f32) -> Self {
        self.max = max.max(0.001); // Prevent division by zero
        self.value = self.value.clamp(0.0, self.max);
        self
    }

    /// Set the buffer value for buffered progress
    ///
    /// Buffered progress shows an additional value indicating estimated completion.
    /// Useful for tasks like video buffering where loading status is variable.
    ///
    /// ## Parameters
    /// - `buffer`: Buffer value (will be clamped between 0.0 and max)
    pub fn buffer(mut self, buffer: f32) -> Self {
        self.buffer = Some(buffer.clamp(0.0, self.max));
        self
    }

    /// Enable or disable indeterminate progress animation
    ///
    /// Indeterminate progress is used when the actual progress is unknown,
    /// such as during loading states. It shows a looping animation to indicate
    /// activity.
    ///
    /// ## Parameters
    /// - `indeterminate`: true to enable indeterminate mode, false to disable
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Enable or disable four-color animation for indeterminate progress
    ///
    /// Four-color animation provides a more dynamic indeterminate animation
    /// using four distinct colors. This can be visually appealing but may
    /// impact performance due to increased draw calls.
    ///
    /// ## Parameters
    /// - `enabled`: true to enable four-color animation, false to disable
    pub fn four_color_enabled(mut self, enabled: bool) -> Self {
        self.four_color_enabled = enabled;
        self
    }

    /// Set the size of the progress indicator
    ///
    /// ## Parameters
    /// - `size`: Desired size (width, height) of the progress indicator
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    /// Set the width of the progress indicator (for linear variant)
    ///
    /// ## Parameters
    /// - `width`: Desired width of the progress indicator
    pub fn width(mut self, width: f32) -> Self {
        self.size.x = width;
        self
    }

    /// Set the height of the progress indicator (for circular variant)
    ///
    /// ## Parameters
    /// - `height`: Desired height of the progress indicator
    pub fn height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }

    /// Enable or disable four-color animation (deprecated, use four_color_enabled)
    ///
    /// ## Parameters
    /// - `enabled`: true to enable four-color animation, false to disable
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

impl MaterialProgress {
    fn render_linear(&self, ui: &mut Ui, rect: Rect) {
        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface_variant = get_global_color("surfaceVariant");
        let primary_container = get_global_color("primaryContainer");

        // Draw track background
        ui.painter()
            .rect_filled(rect, rect.height() / 2.0, surface_variant);

        if self.indeterminate {
            // Indeterminate animation - simplified for egui
            let time = ui.input(|i| i.time) as f32;
            let progress = ((time * 2.0).sin() + 1.0) / 2.0; // Oscillate between 0 and 1

            let bar_width = rect.width() * 0.3; // 30% of total width
            let start_x = rect.min.x + (rect.width() - bar_width) * progress;

            let bar_rect = Rect::from_min_size(
                Pos2::new(start_x, rect.min.y),
                Vec2::new(bar_width, rect.height()),
            );

            ui.painter()
                .rect_filled(bar_rect, rect.height() / 2.0, primary_color);

            // Request repaint for animation
            ui.ctx().request_repaint();
        } else {
            // Draw buffer if present
            if let Some(buffer) = self.buffer {
                let buffer_progress = (buffer / self.max).clamp(0.0, 1.0);
                let buffer_width = rect.width() * buffer_progress;

                if buffer_width > 0.0 {
                    let buffer_rect =
                        Rect::from_min_size(rect.min, Vec2::new(buffer_width, rect.height()));

                    ui.painter()
                        .rect_filled(buffer_rect, rect.height() / 2.0, primary_container);
                }
            }

            // Draw progress bar
            let progress = (self.value / self.max).clamp(0.0, 1.0);
            let progress_width = rect.width() * progress;

            if progress_width > 0.0 {
                let progress_rect =
                    Rect::from_min_size(rect.min, Vec2::new(progress_width, rect.height()));

                ui.painter()
                    .rect_filled(progress_rect, rect.height() / 2.0, primary_color);
            }
        }
    }

    fn render_circular(&self, ui: &mut Ui, rect: Rect) {
        let center = rect.center();
        let radius = (rect.width().min(rect.height()) / 2.0) - 4.0;
        let stroke_width = 4.0;

        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface_variant = get_global_color("surfaceVariant");

        // Draw track circle
        ui.painter()
            .circle_stroke(center, radius, Stroke::new(stroke_width, surface_variant));

        if self.indeterminate {
            // Indeterminate animation - spinning arc
            let time = ui.input(|i| i.time) as f32;
            let rotation = time * 2.0; // Rotation speed
            let arc_length = PI; // Half circle arc

            self.draw_arc(
                ui,
                center,
                radius,
                rotation,
                rotation + arc_length,
                stroke_width,
                primary_color,
            );

            // Request repaint for animation
            ui.ctx().request_repaint();
        } else {
            // Draw progress arc
            let progress = (self.value / self.max).clamp(0.0, 1.0);
            let arc_length = 2.0 * PI * progress;

            if progress > 0.0 {
                self.draw_arc(
                    ui,
                    center,
                    radius,
                    -PI / 2.0, // Start at top
                    -PI / 2.0 + arc_length,
                    stroke_width,
                    primary_color,
                );
            }
        }
    }

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
        let segments = 32;
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
