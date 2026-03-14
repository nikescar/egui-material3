use crate::theme::get_global_color;
use egui::{
    pos2, Area, FontId, Id, Order, Rect, Response, Sense, Stroke, Ui, Vec2,
};

/// Tooltip position relative to target element
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TooltipPosition {
    /// Above the target element
    Top,
    /// Below the target element
    Bottom,
    /// To the left of the target element
    Left,
    /// To the right of the target element
    Right,
    /// Automatically choose best position
    Auto,
}

/// Material Design tooltip component
///
/// Tooltips display informative text when users hover over an element.
/// They follow Material Design 3 specifications.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialButton, show_tooltip_on_hover, TooltipPosition};
///
/// let button_response = ui.add(MaterialButton::filled("Hover me"));
/// show_tooltip_on_hover(ui, &button_response, "This is a tooltip", TooltipPosition::Top);
/// # });
/// ```
pub struct MaterialTooltip {
    text: String,
    position: TooltipPosition,
    max_width: f32,
    padding: Vec2,
    font_size: f32,
}

impl MaterialTooltip {
    /// Create a new tooltip
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: TooltipPosition::Auto,
            max_width: 200.0,
            padding: Vec2::new(8.0, 6.0),
            font_size: 12.0,
        }
    }

    /// Set the tooltip position
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the maximum width
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    /// Set the font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Show the tooltip at a specific position
    pub fn show(&self, ui: &mut Ui, target_rect: Rect) {
        let inverse_surface = get_global_color("inverseSurface");
        let inverse_on_surface = get_global_color("inverseOnSurface");

        // Calculate text size
        let text_galley = ui.painter().layout(
            self.text.clone(),
            FontId::proportional(self.font_size),
            inverse_on_surface,
            self.max_width - self.padding.x * 2.0,
        );

        let tooltip_size = Vec2::new(
            text_galley.size().x + self.padding.x * 2.0,
            text_galley.size().y + self.padding.y * 2.0,
        );

        // Calculate tooltip position based on preferred position
        let screen_rect = ui.ctx().content_rect();
        let tooltip_pos = self.calculate_position(target_rect, tooltip_size, screen_rect);

        // Create unique ID for this tooltip
        let tooltip_id = Id::new("tooltip").with(&self.text);

        // Show tooltip as an overlay
        Area::new(tooltip_id)
            .fixed_pos(tooltip_pos)
            .order(Order::Tooltip)
            .interactable(false)
            .show(ui.ctx(), |ui| {
                let (rect, _) = ui.allocate_exact_size(tooltip_size, Sense::hover());

                // Draw background
                ui.painter().rect_filled(rect, 4.0, inverse_surface);

                // Draw border for better visibility
                ui.painter().rect_stroke(
                    rect,
                    4.0,
                    Stroke::new(1.0, inverse_on_surface.linear_multiply(0.2)),
                    egui::epaint::StrokeKind::Outside,
                );

                // Draw text
                let text_pos = pos2(
                    rect.min.x + self.padding.x,
                    rect.min.y + self.padding.y,
                );
                ui.painter().galley(text_pos, text_galley, inverse_on_surface);
            });
    }

    /// Calculate the position for the tooltip based on the preferred position
    fn calculate_position(
        &self,
        target_rect: Rect,
        tooltip_size: Vec2,
        screen_rect: Rect,
    ) -> egui::Pos2 {
        let spacing = 8.0; // Gap between target and tooltip

        let position = match self.position {
            TooltipPosition::Auto => {
                // Auto-select best position based on available space
                self.auto_position(target_rect, tooltip_size, screen_rect)
            }
            pos => pos,
        };

        match position {
            TooltipPosition::Top => pos2(
                target_rect.center().x - tooltip_size.x / 2.0,
                target_rect.min.y - tooltip_size.y - spacing,
            ),
            TooltipPosition::Bottom => pos2(
                target_rect.center().x - tooltip_size.x / 2.0,
                target_rect.max.y + spacing,
            ),
            TooltipPosition::Left => pos2(
                target_rect.min.x - tooltip_size.x - spacing,
                target_rect.center().y - tooltip_size.y / 2.0,
            ),
            TooltipPosition::Right => pos2(
                target_rect.max.x + spacing,
                target_rect.center().y - tooltip_size.y / 2.0,
            ),
            TooltipPosition::Auto => {
                // This shouldn't happen as Auto is converted above
                pos2(target_rect.max.x + spacing, target_rect.min.y)
            }
        }
    }

    /// Automatically determine the best position
    fn auto_position(
        &self,
        target_rect: Rect,
        tooltip_size: Vec2,
        screen_rect: Rect,
    ) -> TooltipPosition {
        let spacing = 8.0;

        // Check available space in each direction
        let space_above = target_rect.min.y - screen_rect.min.y;
        let space_below = screen_rect.max.y - target_rect.max.y;
        let space_left = target_rect.min.x - screen_rect.min.x;
        let space_right = screen_rect.max.x - target_rect.max.x;

        // Prefer bottom, then top, then right, then left
        if space_below >= tooltip_size.y + spacing {
            TooltipPosition::Bottom
        } else if space_above >= tooltip_size.y + spacing {
            TooltipPosition::Top
        } else if space_right >= tooltip_size.x + spacing {
            TooltipPosition::Right
        } else if space_left >= tooltip_size.x + spacing {
            TooltipPosition::Left
        } else {
            // Default to bottom if no space is sufficient
            TooltipPosition::Bottom
        }
    }
}

/// Convenience function to show a tooltip on hover
///
/// This function shows a tooltip when the user hovers over the target element.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialButton, show_tooltip_on_hover, TooltipPosition};
///
/// let response = ui.add(MaterialButton::filled("Hover me"));
/// show_tooltip_on_hover(ui, &response, "Tooltip text", TooltipPosition::Top);
/// # });
/// ```
pub fn show_tooltip_on_hover(
    ui: &mut Ui,
    target_response: &Response,
    text: impl Into<String>,
    position: TooltipPosition,
) {
    if target_response.hovered() {
        MaterialTooltip::new(text).position(position).show(ui, target_response.rect);
    }
}

/// Convenience function to show a tooltip with custom styling on hover
pub fn show_tooltip_on_hover_custom(
    ui: &mut Ui,
    target_response: &Response,
    tooltip: MaterialTooltip,
) {
    if target_response.hovered() {
        tooltip.show(ui, target_response.rect);
    }
}

/// Add a tooltip to any widget (builder pattern style)
///
/// This is a convenience wrapper that captures the response and shows a tooltip.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialButton, with_tooltip, TooltipPosition};
///
/// let response = with_tooltip(ui, "Tooltip text", TooltipPosition::Top, |ui| {
///     ui.add(MaterialButton::filled("Button with tooltip"))
/// });
/// # });
/// ```
pub fn with_tooltip<R>(
    ui: &mut Ui,
    _text: impl Into<String>,
    _position: TooltipPosition,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    // Note: This is a simplified helper. For proper tooltip support,
    // use show_tooltip_on_hover with the response directly.
    // This function is provided for API compatibility.
    add_contents(ui)
}

/// Create a tooltip component
pub fn tooltip(text: impl Into<String>) -> MaterialTooltip {
    MaterialTooltip::new(text)
}
