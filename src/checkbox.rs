use crate::get_global_color;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

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
/// - States: Normal, hover, focus, pressed, disabled, error
pub struct MaterialCheckbox<'a> {
    /// Mutable reference to the checked state
    checked: &'a mut bool,
    /// Text label displayed next to the checkbox
    text: String,
    /// Whether the checkbox is in indeterminate state (partially checked)
    indeterminate: bool,
    /// Whether the checkbox is interactive (enabled/disabled)
    enabled: bool,
    /// Whether to show error state styling
    is_error: bool,
    /// Custom check mark color (overrides theme)
    check_color: Option<Color32>,
    /// Custom fill color when checked (overrides theme)
    fill_color: Option<Color32>,
    /// Custom border width (default: 2.0)
    border_width: f32,
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
            is_error: false,
            check_color: None,
            fill_color: None,
            border_width: 2.0,
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

    /// Set whether the checkbox should display in error state
    ///
    /// Error state checkboxes use error color from the theme to indicate
    /// validation failure or invalid selection.
    ///
    /// ## Parameters
    /// - `is_error`: True for error state styling
    pub fn is_error(mut self, is_error: bool) -> Self {
        self.is_error = is_error;
        self
    }

    /// Set custom check mark color
    ///
    /// Overrides the default check mark color from the theme.
    ///
    /// ## Parameters
    /// - `color`: Custom color for the check mark
    pub fn check_color(mut self, color: Color32) -> Self {
        self.check_color = Some(color);
        self
    }

    /// Set custom fill color when checked
    ///
    /// Overrides the default fill color from the theme.
    ///
    /// ## Parameters
    /// - `color`: Custom fill color when checkbox is checked
    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set custom border width
    ///
    /// ## Parameters
    /// - `width`: Border width in pixels (default: 2.0)
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }
}

impl<'a> Widget for MaterialCheckbox<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(ui.available_width().min(300.0), 24.0);

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() && self.enabled {
            if self.indeterminate {
                *self.checked = true;
            } else {
                *self.checked = !*self.checked;
            }
            response.mark_changed();
        }

        let visuals = ui.style().interact(&response);
        let checkbox_size = 18.0;
        let checkbox_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - checkbox_size / 2.0),
            Vec2::splat(checkbox_size),
        );

        // Material Design colors
        let primary_color = self.fill_color.unwrap_or_else(|| get_global_color("primary"));
        let error_color = get_global_color("error");
        let on_error = get_global_color("onError");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let surface_variant = get_global_color("surfaceVariant");
        let outline = get_global_color("outline");
        let on_primary = self.check_color.unwrap_or_else(|| get_global_color("onPrimary"));

        // Determine colors based on state
        let (bg_color, border_color, check_color, border_width) = if !self.enabled {
            // Material Design disabled state: onSurface with 38% opacity
            let disabled_color = on_surface.gamma_multiply(0.38);
            if *self.checked || self.indeterminate {
                (disabled_color, Color32::TRANSPARENT, on_surface.gamma_multiply(0.38), 0.0)
            } else {
                (Color32::TRANSPARENT, disabled_color, disabled_color, self.border_width)
            }
        } else if self.is_error {
            // Error state styling
            if *self.checked || self.indeterminate {
                (error_color, Color32::TRANSPARENT, on_error, 0.0)
            } else if response.hovered() {
                (Color32::TRANSPARENT, error_color, on_surface, self.border_width)
            } else {
                (Color32::TRANSPARENT, error_color, on_surface, self.border_width)
            }
        } else if *self.checked || self.indeterminate {
            // Checked/indeterminate state
            (primary_color, Color32::TRANSPARENT, on_primary, 0.0)
        } else if response.hovered() {
            // Hover state for unchecked
            (Color32::TRANSPARENT, on_surface, on_surface, self.border_width)
        } else {
            // Default unchecked state
            (Color32::TRANSPARENT, on_surface_variant, on_surface, self.border_width)
        };

        // Draw checkbox background
        ui.painter().rect_filled(checkbox_rect, 2.0, bg_color);

        // Draw checkbox border (only for unchecked or when needed)
        if border_width > 0.0 {
            ui.painter().rect_stroke(
                checkbox_rect,
                2.0,
                Stroke::new(border_width, border_color),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Draw checkmark or indeterminate mark
        if *self.checked && !self.indeterminate {
            // Draw checkmark
            let center = checkbox_rect.center();
            let checkmark_size = checkbox_size * 0.6;

            let start = Pos2::new(center.x - checkmark_size * 0.3, center.y);
            let middle = Pos2::new(
                center.x - checkmark_size * 0.1,
                center.y + checkmark_size * 0.2,
            );
            let end = Pos2::new(
                center.x + checkmark_size * 0.3,
                center.y - checkmark_size * 0.2,
            );

            ui.painter()
                .line_segment([start, middle], Stroke::new(2.0, check_color));
            ui.painter()
                .line_segment([middle, end], Stroke::new(2.0, check_color));
        } else if self.indeterminate {
            // Draw indeterminate mark (horizontal line)
            let center = checkbox_rect.center();
            let line_width = checkbox_size * 0.5;
            let start = Pos2::new(center.x - line_width / 2.0, center.y);
            let end = Pos2::new(center.x + line_width / 2.0, center.y);

            ui.painter()
                .line_segment([start, end], Stroke::new(2.0, check_color));
        }

        // Draw label text
        if !self.text.is_empty() {
            let text_pos = Pos2::new(checkbox_rect.max.x + 8.0, rect.center().y);

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

        // Add state overlay effect (hover/focus/pressed)
        if self.enabled {
            let overlay_rect = Rect::from_center_size(checkbox_rect.center(), Vec2::splat(40.0));
            let overlay_color = if response.is_pointer_button_down_on() {
                // Pressed state: 10% opacity
                if self.is_error {
                    Color32::from_rgba_premultiplied(
                        error_color.r(),
                        error_color.g(),
                        error_color.b(),
                        25,
                    )
                } else if *self.checked || self.indeterminate {
                    Color32::from_rgba_premultiplied(
                        primary_color.r(),
                        primary_color.g(),
                        primary_color.b(),
                        25,
                    )
                } else {
                    Color32::from_rgba_premultiplied(
                        on_surface.r(),
                        on_surface.g(),
                        on_surface.b(),
                        25,
                    )
                }
            } else if response.hovered() {
                // Hover state: 8% opacity
                if self.is_error {
                    Color32::from_rgba_premultiplied(
                        error_color.r(),
                        error_color.g(),
                        error_color.b(),
                        20,
                    )
                } else if *self.checked || self.indeterminate {
                    Color32::from_rgba_premultiplied(
                        primary_color.r(),
                        primary_color.g(),
                        primary_color.b(),
                        20,
                    )
                } else {
                    Color32::from_rgba_premultiplied(
                        on_surface.r(),
                        on_surface.g(),
                        on_surface.b(),
                        20,
                    )
                }
            } else if response.has_focus() {
                // Focus state: 10% opacity
                if self.is_error {
                    Color32::from_rgba_premultiplied(
                        error_color.r(),
                        error_color.g(),
                        error_color.b(),
                        25,
                    )
                } else if *self.checked || self.indeterminate {
                    Color32::from_rgba_premultiplied(
                        primary_color.r(),
                        primary_color.g(),
                        primary_color.b(),
                        25,
                    )
                } else {
                    Color32::from_rgba_premultiplied(
                        on_surface.r(),
                        on_surface.g(),
                        on_surface.b(),
                        25,
                    )
                }
            } else {
                Color32::TRANSPARENT
            };

            if overlay_color != Color32::TRANSPARENT {
                ui.painter().circle_filled(
                    overlay_rect.center(),
                    overlay_rect.width() / 2.0,
                    overlay_color,
                );
            }
        }

        response
    }
}

pub fn checkbox(checked: &mut bool, text: impl Into<String>) -> MaterialCheckbox<'_> {
    MaterialCheckbox::new(checked, text)
}
