use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, TextStyle, Ui, Vec2, Widget, TextEdit,
};
use crate::get_global_color;

/// Material Design text field component.
///
/// Text fields allow users to enter text into a UI. They typically appear in forms and dialogs.
/// Available in filled and outlined variants following Material Design 3 specifications.
///
/// ```
/// # egui::__run_test_ui(|ui| {\
/// let mut text = String::new();
/// ui.add(MaterialTextField::filled(&mut text)
///     .label("Enter your name")
///     .hint_text("e.g. John Doe"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialTextField<'a> {
    text: &'a mut String,
    variant: TextFieldVariant,
    label: Option<String>,
    hint_text: Option<String>,
    supporting_text: Option<String>,
    prefix_text: Option<String>,
    suffix_text: Option<String>,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    disabled: bool,
    required: bool,
    error: bool,
    multiline: bool,
    width: Option<f32>,
    min_rows: usize,
    max_rows: Option<usize>,
}

/// Text field variants following Material Design 3.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextFieldVariant {
    /// Filled text field with background color.
    Filled,
    /// Outlined text field with border.
    Outlined,
}

impl<'a> MaterialTextField<'a> {
    /// Create a new filled text field.
    pub fn filled(text: &'a mut String) -> Self {
        Self::new(text, TextFieldVariant::Filled)
    }

    /// Create a new outlined text field.
    pub fn outlined(text: &'a mut String) -> Self {
        Self::new(text, TextFieldVariant::Outlined)
    }

    /// Create a new text field with the specified variant.
    pub fn new(text: &'a mut String, variant: TextFieldVariant) -> Self {
        Self {
            text,
            variant,
            label: None,
            hint_text: None,
            supporting_text: None,
            prefix_text: None,
            suffix_text: None,
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
            required: false,
            error: false,
            multiline: false,
            width: None,
            min_rows: 1,
            max_rows: None,
        }
    }

    /// Set the label text.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the hint/placeholder text.
    pub fn hint_text(mut self, hint: impl Into<String>) -> Self {
        self.hint_text = Some(hint.into());
        self
    }

    /// Set the supporting text below the field.
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set the prefix text.
    pub fn prefix_text(mut self, text: impl Into<String>) -> Self {
        self.prefix_text = Some(text.into());
        self
    }

    /// Set the suffix text.
    pub fn suffix_text(mut self, text: impl Into<String>) -> Self {
        self.suffix_text = Some(text.into());
        self
    }

    /// Set the leading icon.
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set the trailing icon.
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set whether the field is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Set whether the field is in error state.
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Set whether the field is multiline (textarea).
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// Set the width of the text field.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the minimum number of rows for multiline fields.
    pub fn min_rows(mut self, rows: usize) -> Self {
        self.min_rows = rows.max(1);
        self
    }

    /// Set the maximum number of rows for multiline fields.
    pub fn max_rows(mut self, rows: usize) -> Self {
        self.max_rows = Some(rows);
        self
    }
}

impl Widget for MaterialTextField<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialTextField {
            text,
            variant,
            label,
            hint_text,
            supporting_text,
            prefix_text,
            suffix_text,
            leading_icon,
            trailing_icon,
            disabled,
            required,
            error,
            multiline,
            width,
            min_rows,
            max_rows,
        } = self;

        // Material Design colors
        let md_surface = get_global_color("surface"); // md-sys-color-surface
        let md_surface_variant = get_global_color("surfaceVariant"); // md-sys-color-surface-variant
        let md_on_surface = get_global_color("onSurface"); // md-sys-color-on-surface
        let md_on_surface_variant = get_global_color("onSurfaceVariant"); // md-sys-color-on-surface-variant
        let md_primary = get_global_color("primary"); // md-sys-color-primary
        let md_outline = get_global_color("outline"); // md-sys-color-outline
        let md_outline_variant = get_global_color("outlineVariant"); // md-sys-color-outline-variant
        let md_error = get_global_color("error"); // md-sys-color-error

        let desired_width = width.unwrap_or(200.0);
        let field_height = if multiline {
            let row_height = 24.0;
            let base_height = 56.0;
            base_height + (min_rows.saturating_sub(1) as f32 * row_height)
        } else {
            56.0
        };

        // Total height includes label, field, and supporting text
        let label_height = if label.is_some() { 16.0 } else { 0.0 };
        let supporting_height = if supporting_text.is_some() { 16.0 } else { 0.0 };
        let total_height = label_height + field_height + supporting_height + 8.0; // padding

        let desired_size = Vec2::new(desired_width, total_height);
        let mut response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            let mut current_y = rect.min.y;

            // Draw label
            if let Some(label_text) = &label {
                let label_rect = Rect::from_min_size(
                    egui::Pos2::new(rect.min.x, current_y),
                    Vec2::new(desired_width, 16.0),
                );
                
                let label_color = if error {
                    md_error
                } else if disabled {
                    Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                } else {
                    md_on_surface_variant
                };

                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(label_rect), |ui| {
                    ui.style_mut().override_text_style = Some(TextStyle::Small);
                    ui.colored_label(label_color, format!("{}{}", label_text, if required { " *" } else { "" }));
                });
                current_y += 20.0;
            }

            // Field rect
            let field_rect = Rect::from_min_size(
                egui::Pos2::new(rect.min.x, current_y),
                Vec2::new(desired_width, field_height),
            );

            // Determine field colors
            let (bg_color, border_color, text_color) = match variant {
                TextFieldVariant::Filled => {
                    let bg = if disabled {
                        Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 31)
                    } else {
                        md_surface_variant
                    };
                    let border = if error {
                        md_error
                    } else if disabled {
                        Color32::TRANSPARENT
                    } else {
                        md_outline
                    };
                    let text = if disabled {
                        Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                    } else {
                        md_on_surface
                    };
                    (bg, border, text)
                }
                TextFieldVariant::Outlined => {
                    let bg = Color32::TRANSPARENT;
                    let border = if error {
                        md_error
                    } else if disabled {
                        Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 31)
                    } else {
                        md_outline
                    };
                    let text = if disabled {
                        Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                    } else {
                        md_on_surface
                    };
                    (bg, border, text)
                }
            };

            // Draw field background and border
            let corner_radius = match variant {
                TextFieldVariant::Filled => CornerRadius {
                    nw: 4,
                    ne: 4,
                    sw: 0,
                    se: 0,
                },
                TextFieldVariant::Outlined => CornerRadius::from(4),
            };

            ui.painter().rect_filled(field_rect, corner_radius, bg_color);
            if border_color != Color32::TRANSPARENT {
                let stroke_width = if error { 2.0 } else { 1.0 };
                ui.painter().rect_stroke(field_rect, corner_radius, Stroke::new(stroke_width, border_color), egui::epaint::StrokeKind::Outside);
            }

            // Calculate content area (excluding padding and icons)
            let mut content_rect = field_rect.shrink2(Vec2::new(16.0, 12.0));
            
            // Adjust for leading icon
            if leading_icon.is_some() {
                content_rect.min.x += 24.0;
            }
            
            // Adjust for trailing icon
            if trailing_icon.is_some() {
                content_rect.max.x -= 24.0;
            }

            // Adjust for prefix text
            if let Some(prefix) = &prefix_text {
                let prefix_width = ui.fonts(|f| f.layout_no_wrap(prefix.clone(), egui::FontId::default(), egui::Color32::WHITE).rect.width()) + 4.0;
                content_rect.min.x += prefix_width;
            }

            // Adjust for suffix text
            if let Some(suffix) = &suffix_text {
                let suffix_width = ui.fonts(|f| f.layout_no_wrap(suffix.clone(), egui::FontId::default(), egui::Color32::WHITE).rect.width()) + 4.0;
                content_rect.max.x -= suffix_width;
            }

            // Draw leading icon
            if let Some(icon) = &leading_icon {
                let icon_rect = Rect::from_center_size(
                    egui::Pos2::new(field_rect.min.x + 24.0, field_rect.center().y),
                    Vec2::splat(24.0),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(icon_rect), |ui| {
                    ui.style_mut().override_text_style = Some(TextStyle::Body);
                    ui.centered_and_justified(|ui| {
                        ui.colored_label(text_color, icon);
                    });
                });
            }

            // Draw prefix text
            if let Some(prefix) = &prefix_text {
                let prefix_pos = if leading_icon.is_some() {
                    field_rect.min.x + 48.0
                } else {
                    field_rect.min.x + 16.0
                };
                let prefix_rect = Rect::from_min_size(
                    egui::Pos2::new(prefix_pos, field_rect.center().y - 8.0),
                    Vec2::new(100.0, 16.0),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(prefix_rect), |ui| {
                    ui.style_mut().override_text_style = Some(TextStyle::Body);
                    ui.colored_label(text_color, prefix);
                });
            }

            // Draw text input
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                ui.style_mut().override_text_style = Some(TextStyle::Body);
                ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;

                let mut text_edit = if multiline {
                    TextEdit::multiline(text)
                } else {
                    TextEdit::singleline(text)
                };

                if let Some(hint) = &hint_text {
                    text_edit = text_edit.hint_text(hint);
                }

                if disabled {
                    text_edit = text_edit.interactive(false);
                }

                let text_response = ui.add(text_edit);
                response = response.union(text_response);
            });

            // Draw suffix text
            if let Some(suffix) = &suffix_text {
                let suffix_pos = if trailing_icon.is_some() {
                    field_rect.max.x - 48.0 - ui.fonts(|f| f.layout_no_wrap(suffix.clone(), egui::FontId::default(), egui::Color32::WHITE).rect.width())
                } else {
                    field_rect.max.x - 16.0 - ui.fonts(|f| f.layout_no_wrap(suffix.clone(), egui::FontId::default(), egui::Color32::WHITE).rect.width())
                };
                let suffix_rect = Rect::from_min_size(
                    egui::Pos2::new(suffix_pos, field_rect.center().y - 8.0),
                    Vec2::new(100.0, 16.0),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(suffix_rect), |ui| {
                    ui.style_mut().override_text_style = Some(TextStyle::Body);
                    ui.colored_label(text_color, suffix);
                });
            }

            // Draw trailing icon
            if let Some(icon) = &trailing_icon {
                let icon_rect = Rect::from_center_size(
                    egui::Pos2::new(field_rect.max.x - 24.0, field_rect.center().y),
                    Vec2::splat(24.0),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(icon_rect), |ui| {
                    ui.style_mut().override_text_style = Some(TextStyle::Body);
                    ui.centered_and_justified(|ui| {
                        ui.colored_label(text_color, icon);
                    });
                });
            }

            current_y += field_height + 4.0;

            // Draw supporting text
            if let Some(support_text) = &supporting_text {
                let support_rect = Rect::from_min_size(
                    egui::Pos2::new(rect.min.x, current_y),
                    Vec2::new(desired_width, 16.0),
                );
                
                let support_color = if error {
                    md_error
                } else if disabled {
                    Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                } else {
                    md_on_surface_variant
                };

                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(support_rect), |ui| {
                    ui.style_mut().override_text_style = Some(TextStyle::Small);
                    ui.colored_label(support_color, support_text);
                });
            }
        }

        response
    }
}

/// Convenience function to create a filled text field.
pub fn material_text_field_filled(text: &mut String) -> MaterialTextField {
    MaterialTextField::filled(text)
}

/// Convenience function to create an outlined text field.
pub fn material_text_field_outlined(text: &mut String) -> MaterialTextField {
    MaterialTextField::outlined(text)
}