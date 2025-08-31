use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use crate::get_global_color;

/// Material Design checkbox component following Material Design 3 specifications
/// 
/// Provides a checkbox with three states: checked, unchecked, and indeterminate.
/// Follows Material Design guidelines for colors, sizing, and interaction states.
/// 
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut checked = false;
/// 
/// // Basic checkbox
/// ui.add(MaterialCheckbox::new(&mut checked, "Accept terms"));
/// 
/// // Checkbox with indeterminate state
/// let mut partial_checked = false;
/// ui.add(MaterialCheckbox::new(&mut partial_checked, "Select all")
///     .indeterminate(true));
/// 
/// // Disabled checkbox
/// let mut disabled_checked = false;  
/// ui.add(MaterialCheckbox::new(&mut disabled_checked, "Disabled option")
///     .enabled(false));
/// # });
/// ```
/// 
/// ## Material Design Spec
/// - Size: 18x18dp checkbox with 40x40dp touch target
/// - Colors: Primary color when checked, outline when unchecked
/// - Animation: 150ms cubic-bezier transition
/// - States: Normal, hover, focus, pressed, disabled
pub struct MaterialCheckbox<'a> {
    /// Mutable reference to the checked state
    checked: &'a mut bool,
    /// Text label displayed next to the checkbox
    text: String,
    /// Whether the checkbox is in indeterminate state (partially checked)
    indeterminate: bool,
    /// Whether the checkbox is interactive (enabled/disabled)
    enabled: bool,
}

impl<'a> MaterialCheckbox<'a> {
    /// Create a new Material Design checkbox
    /// 
    /// ## Parameters
    /// - `checked`: Mutable reference to boolean state
    /// - `text`: Label text displayed next to checkbox
    /// 
    /// ## Returns
    /// A new MaterialCheckbox instance with default settings
    pub fn new(checked: &'a mut bool, text: impl Into<String>) -> Self {
        Self {
            checked,
            text: text.into(),
            indeterminate: false,
            enabled: true,
        }
    }

    /// Set the indeterminate state of the checkbox
    /// 
    /// Indeterminate checkboxes are used when the checkbox represents
    /// a collection of items where some, but not all, are selected.
    /// 
    /// ## Parameters  
    /// - `indeterminate`: True for indeterminate state, false for normal
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Set whether the checkbox is enabled or disabled
    /// 
    /// Disabled checkboxes cannot be interacted with and are visually dimmed.
    /// 
    /// ## Parameters
    /// - `enabled`: True for interactive, false for disabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for MaterialCheckbox<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(
            ui.available_width().min(300.0),
            24.0,
        );

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() && self.enabled {
            if self.indeterminate {
                *self.checked = true;
            } else {
                *self.checked = !*self.checked;
            }
            response.mark_changed();
        }

        let _visuals = ui.style().interact(&response);
        let checkbox_size = 18.0;
        let checkbox_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - checkbox_size / 2.0),
            Vec2::splat(checkbox_size),
        );

        // Material Design colors
        let primary_color = get_global_color("primary");
        let on_surface = get_global_color("onSurface");
        let surface_variant = get_global_color("surfaceVariant");
        let outline = get_global_color("outline");

        let (bg_color, border_color, check_color) = if !self.enabled {
            // Material Design disabled state: onSurface with 38% opacity
            let disabled_color = on_surface.gamma_multiply(0.38);
            (
                Color32::TRANSPARENT,
                disabled_color,
                disabled_color,
            )
        } else if *self.checked || self.indeterminate {
            (primary_color, primary_color, get_global_color("onPrimary"))
        } else if response.hovered() {
            (surface_variant, outline, on_surface)
        } else {
            (Color32::TRANSPARENT, outline, on_surface)
        };

        // Draw checkbox background
        ui.painter().rect_filled(
            checkbox_rect,
            2.0,
            bg_color,
        );

        // Draw checkbox border
        ui.painter().rect_stroke(
            checkbox_rect,
            2.0,
            Stroke::new(2.0, border_color),
            egui::epaint::StrokeKind::Outside,
        );

        // Draw checkmark or indeterminate mark
        if *self.checked && !self.indeterminate {
            // Draw checkmark
            let center = checkbox_rect.center();
            let checkmark_size = checkbox_size * 0.6;
            
            let start = Pos2::new(
                center.x - checkmark_size * 0.3,
                center.y,
            );
            let middle = Pos2::new(
                center.x - checkmark_size * 0.1,
                center.y + checkmark_size * 0.2,
            );
            let end = Pos2::new(
                center.x + checkmark_size * 0.3,
                center.y - checkmark_size * 0.2,
            );

            ui.painter().line_segment([start, middle], Stroke::new(2.0, check_color));
            ui.painter().line_segment([middle, end], Stroke::new(2.0, check_color));
        } else if self.indeterminate {
            // Draw indeterminate mark (horizontal line)
            let center = checkbox_rect.center();
            let line_width = checkbox_size * 0.5;
            let start = Pos2::new(center.x - line_width / 2.0, center.y);
            let end = Pos2::new(center.x + line_width / 2.0, center.y);
            
            ui.painter().line_segment([start, end], Stroke::new(2.0, check_color));
        }

        // Draw label text
        if !self.text.is_empty() {
            let text_pos = Pos2::new(
                checkbox_rect.max.x + 8.0,
                rect.center().y,
            );
            
            let text_color = if self.enabled { 
                on_surface 
            } else { 
                on_surface.gamma_multiply(0.38)
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                &self.text,
                egui::FontId::default(),
                text_color,
            );
        }

        // Add ripple effect on hover/click
        if response.hovered() && self.enabled {
            let ripple_rect = Rect::from_center_size(checkbox_rect.center(), Vec2::splat(40.0));
            let ripple_color = if *self.checked || self.indeterminate {
                Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 20)
            } else {
                Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20)
            };
            
            ui.painter().circle_filled(
                ripple_rect.center(),
                ripple_rect.width() / 2.0,
                ripple_color,
            );
        }

        response
    }
}

pub fn checkbox(checked: &mut bool, text: impl Into<String>) -> MaterialCheckbox<'_> {
    MaterialCheckbox::new(checked, text)
}