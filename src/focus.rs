use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget,
};

/// Material Design focus ring component.
///
/// A focus ring provides a visual indicator for keyboard navigation and accessibility.
/// It appears around interactive elements when they receive keyboard focus.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Add a focus ring to a button
/// let button_response = ui.button("Focusable Button");
/// ui.add(MaterialFocusRing::new()
///     .visible(button_response.has_focus())
///     .target_rect(button_response.rect));
///
/// // Focus ring with inward animation
/// ui.add(MaterialFocusRing::new()
///     .visible(true)
///     .inward(true)
///     .target_rect(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(64.0, 64.0))));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialFocusRing {
    visible: bool,
    inward: bool,
    target_rect: Option<Rect>,
    corner_radius: CornerRadius,
    color: Option<Color32>,
    width: f32,
    offset: f32,
}

impl MaterialFocusRing {
    /// Create a new material focus ring.
    pub fn new() -> Self {
        Self {
            visible: false,
            inward: false,
            target_rect: None,
            corner_radius: CornerRadius::from(16),
            color: None,
            width: 2.0,
            offset: 2.0,
        }
    }

    /// Set the visibility of the focus ring.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Make the focus ring animate inwards instead of outwards.
    pub fn inward(mut self, inward: bool) -> Self {
        self.inward = inward;
        self
    }

    /// Set the target rectangle that the focus ring should surround.
    pub fn target_rect(mut self, rect: Rect) -> Self {
        self.target_rect = Some(rect);
        self
    }

    /// Set the corner radius of the focus ring.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set the color of the focus ring.
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the width of the focus ring stroke.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the offset from the target element.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Attach the focus ring to a response, automatically setting visibility based on focus state.
    pub fn attach_to_response(mut self, response: &Response) -> Self {
        self.visible = response.has_focus();
        self.target_rect = Some(response.rect);
        self
    }
}

impl Default for MaterialFocusRing {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialFocusRing {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialFocusRing {
            visible,
            inward,
            target_rect,
            corner_radius,
            color,
            width,
            offset,
        } = self;

        // Material Design focus ring color
        let focus_color = color.unwrap_or_else(|| Color32::from_rgb(103, 80, 164)); // md-sys-color-primary

        let rect = if let Some(target) = target_rect {
            if inward {
                // Inward focus ring shrinks the ring inside the target
                target.shrink(offset)
            } else {
                // Outward focus ring expands outside the target
                target.expand(offset)
            }
        } else {
            // If no target rect is provided, use available space
            let size = Vec2::splat(64.0); // Default size matching Material Web stories
            Rect::from_min_size(ui.next_widget_position(), size)
        };

        let response = ui.allocate_response(rect.size(), Sense::hover());
        let response_rect = response.rect;

        if visible && ui.is_rect_visible(rect) {
            // Draw the focus ring
            ui.painter().rect_stroke(
                rect,
                corner_radius,
                Stroke::new(width, focus_color),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Set aria-hidden attribute equivalent behavior by not including in accessibility
        // In egui, this is done by not adding any widget info for screen readers
        response
    }
}

/// Helper function to add a focus ring to any response.
/// This is a convenience function for quickly adding focus rings to existing widgets.
pub fn add_focus_ring_to_response(response: &Response, ui: &mut Ui, inward: bool) {
    if response.has_focus() {
        ui.add(MaterialFocusRing::new()
            .visible(true)
            .inward(inward)
            .target_rect(response.rect));
    }
}

/// Helper function to add a focus ring that's always visible for demonstration purposes.
pub fn demo_focus_ring(ui: &mut Ui, rect: Rect, inward: bool) -> Response {
    ui.add(MaterialFocusRing::new()
        .visible(true)
        .inward(inward)
        .target_rect(rect))
}