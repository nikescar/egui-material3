use egui::{
    ecolor::Color32, 
    Rect, Response, Sense, Ui, Vec2, Widget,
};
use crate::theme::get_global_color;

/// Material Design divider component.
///
/// A divider is a thin line that groups content in lists and containers.
/// Dividers can reinforce tapability, such as when used to separate
/// list items or define tappable regions in an accordion.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Basic divider
/// ui.add(MaterialDivider::new());
///
/// // Inset divider
/// ui.add(MaterialDivider::new().inset(true));
///
/// // Inset start divider
/// ui.add(MaterialDivider::new().inset_start(true));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialDivider {
    inset: bool,
    inset_start: bool,
    inset_end: bool,
}

impl MaterialDivider {
    /// Create a new material divider.
    pub fn new() -> Self {
        Self {
            inset: false,
            inset_start: false,
            inset_end: false,
        }
    }

    /// Indents the divider with equal padding on both sides.
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Indents the divider with padding on the leading side.
    pub fn inset_start(mut self, inset_start: bool) -> Self {
        self.inset_start = inset_start;
        self
    }

    /// Indents the divider with padding on the trailing side.
    pub fn inset_end(mut self, inset_end: bool) -> Self {
        self.inset_end = inset_end;
        self
    }
}

impl Default for MaterialDivider {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialDivider {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialDivider {
            inset,
            inset_start,
            inset_end,
        } = self;

        // Material Design divider color - outline variant
        let divider_color = get_global_color("outlineVariant");
        
        // Material Design divider thickness
        let thickness = 1.0;
        
        // Material Design inset values (16dp)
        let inset_value = 16.0;
        
        // Calculate padding
        let left_padding = if inset || inset_start { inset_value } else { 0.0 };
        let right_padding = if inset || inset_end { inset_value } else { 0.0 };
        
        // Desired size for the divider
        let desired_size = Vec2::new(
            ui.available_width() - left_padding - right_padding,
            thickness
        );
        
        let response = ui.allocate_response(
            Vec2::new(ui.available_width(), thickness), 
            Sense::hover()
        );
        let rect = response.rect;
        
        if ui.is_rect_visible(rect) {
            // Create the actual divider rect with insets
            let divider_rect = Rect::from_min_size(
                rect.min + Vec2::new(left_padding, 0.0),
                desired_size
            );
            
            // Draw the divider line
            ui.painter().rect_filled(
                divider_rect,
                0.0,
                divider_color,
            );
        }

        response
    }
}