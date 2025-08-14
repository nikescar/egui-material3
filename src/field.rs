use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Align, Rect, Response, Sense, TextStyle, Ui, Vec2, Widget, WidgetText,
};
use crate::get_global_color;

/// Material Design field component variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldVariant {
    Filled,
    Outlined,
}

/// Material Design field component.
///
/// A field is a container component that provides structure for form inputs,
/// including labels, supporting text, and visual styling following Material Design 3 specifications.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Basic filled field with label
/// ui.add(MaterialField::filled()
///     .label("Username")
///     .content(|ui| {
///         ui.text_edit_singleline(&mut String::new());
///     }));
///
/// // Outlined field with supporting text
/// ui.add(MaterialField::outlined()
///     .label("Email")
///     .supporting_text("Enter your email address")
///     .content(|ui| {
///         ui.text_edit_singleline(&mut String::new());
///     }));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialField<'a> {
    variant: FieldVariant,
    label: Option<WidgetText>,
    supporting_text: Option<WidgetText>,
    error_text: Option<WidgetText>,
    content: Option<Box<dyn FnOnce(&mut Ui) -> Response + 'a>>,
    disabled: bool,
    error: bool,
    focused: bool,
    populated: bool,
    required: bool,
    no_asterisk: bool,
    has_start: bool,
    has_end: bool,
    count: i32,
    max: i32,
    min_width: f32,
}

impl<'a> MaterialField<'a> {
    /// Create a new filled material field.
    pub fn filled() -> Self {
        Self::new_with_variant(FieldVariant::Filled)
    }

    /// Create a new outlined material field.
    pub fn outlined() -> Self {
        Self::new_with_variant(FieldVariant::Outlined)
    }

    fn new_with_variant(variant: FieldVariant) -> Self {
        Self {
            variant,
            label: None,
            supporting_text: None,
            error_text: None,
            content: None,
            disabled: false,
            error: false,
            focused: false,
            populated: false,
            required: false,
            no_asterisk: false,
            has_start: false,
            has_end: false,
            count: -1,
            max: -1,
            min_width: 256.0,
        }
    }

    /// Set the field label.
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the supporting text.
    pub fn supporting_text(mut self, text: impl Into<WidgetText>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set the error text.
    pub fn error_text(mut self, text: impl Into<WidgetText>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    /// Set the content of the field.
    pub fn content<F, R>(mut self, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) -> R + 'a,
        R: Into<Response>,
    {
        self.content = Some(Box::new(move |ui| content(ui).into()));
        self
    }

    /// Enable or disable the field.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.disabled = !enabled;
        self
    }

    /// Set the error state.
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Set the focused state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set the populated state.
    pub fn populated(mut self, populated: bool) -> Self {
        self.populated = populated;
        self
    }

    /// Set if the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Hide the asterisk for required fields.
    pub fn no_asterisk(mut self, no_asterisk: bool) -> Self {
        self.no_asterisk = no_asterisk;
        self
    }

    /// Set if the field has start content.
    pub fn has_start(mut self, has_start: bool) -> Self {
        self.has_start = has_start;
        self
    }

    /// Set if the field has end content.
    pub fn has_end(mut self, has_end: bool) -> Self {
        self.has_end = has_end;
        self
    }

    /// Set character count and maximum.
    pub fn count(mut self, count: i32, max: i32) -> Self {
        self.count = count;
        self.max = max;
        self
    }

    /// Set the minimum width of the field.
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width;
        self
    }

    fn get_counter_text(&self) -> Option<String> {
        if self.count < 0 || self.max <= 0 {
            return None;
        }
        Some(format!("{} / {}", self.count, self.max))
    }

    fn get_supporting_or_error_text(&self) -> Option<&WidgetText> {
        if self.error && self.error_text.is_some() {
            self.error_text.as_ref()
        } else {
            self.supporting_text.as_ref()
        }
    }
}

impl<'a> Default for MaterialField<'a> {
    fn default() -> Self {
        Self::filled()
    }
}

impl Widget for MaterialField<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialField {
            variant,
            label,
            supporting_text,
            error_text,
            content,
            disabled,
            error,
            focused,
            populated,
            required,
            no_asterisk,
            has_start,
            has_end,
            count,
            max,
            min_width,
        } = self;
        
        let supporting_text_value = if error {
            error_text.as_ref()
        } else {
            supporting_text.as_ref()
        };
        
        let counter_text_value = if count >= 0 && max > 0 {
            Some(format!("{}/{}", count, max))
        } else {
            None
        };

        // Material Design colors
        let md_surface = get_global_color("surface");
        let md_on_surface = get_global_color("onSurface");
        let md_on_surface_variant = get_global_color("onSurfaceVariant");
        let md_outline = get_global_color("outline");
        let md_outline_variant = get_global_color("outlineVariant");
        let md_primary = get_global_color("primary");
        let md_error = get_global_color("error");

        // Calculate colors based on state
        let (background_color, border_color, label_color) = match variant {
            FieldVariant::Filled => {
                let bg = if disabled {
                    Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 31)
                } else {
                    md_outline_variant
                };
                
                let border = if error && !disabled {
                    md_error
                } else if focused && !disabled {
                    md_primary
                } else if disabled {
                    Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                } else {
                    md_on_surface_variant
                };

                let label = if error && !disabled {
                    md_error
                } else if focused && !disabled {
                    md_primary
                } else if disabled {
                    Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                } else {
                    md_on_surface_variant
                };

                (bg, border, label)
            },
            FieldVariant::Outlined => {
                let bg = Color32::TRANSPARENT;
                
                let border = if error && !disabled {
                    md_error
                } else if focused && !disabled {
                    md_primary
                } else if disabled {
                    Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 31)
                } else {
                    md_outline
                };

                let label = if error && !disabled {
                    md_error
                } else if focused && !disabled {
                    md_primary
                } else if disabled {
                    Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                } else {
                    md_on_surface_variant
                };

                (bg, border, label)
            },
        };

        let border_width = if focused && !disabled { 2.0 } else { 1.0 };
        let corner_radius = CornerRadius::from(4);

        // Calculate field height
        let field_height = 56.0; // Standard Material Design field height
        let label_height = 16.0;
        let content_height = field_height - 16.0; // Padding top and bottom

        let desired_width = min_width.max(ui.available_width());
        let desired_size = Vec2::new(desired_width, field_height);

        let mut response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw field background and border
            match variant {
                FieldVariant::Filled => {
                    // Draw filled background
                    ui.painter().rect_filled(rect, corner_radius, background_color);
                    // Draw bottom border
                    let border_rect = Rect::from_min_size(
                        rect.min + Vec2::new(0.0, rect.height() - border_width),
                        Vec2::new(rect.width(), border_width),
                    );
                    ui.painter().rect_filled(border_rect, 0.0, border_color);
                },
                FieldVariant::Outlined => {
                    // Draw outlined border
                    ui.painter().rect_stroke(
                        rect,
                        corner_radius,
                        Stroke::new(border_width, border_color),
                        egui::epaint::StrokeKind::Outside,
                    );
                },
            }

            // Draw label
            if let Some(label_text) = &label {
                let label_string = format!(
                    "{}{}",
                    label_text.text(),
                    if required && !no_asterisk { " *" } else { "" }
                );

                let label_y = if focused || populated {
                    // Floating label position
                    rect.min.y + 8.0
                } else {
                    // Resting label position
                    rect.min.y + (field_height - label_height) / 2.0
                };

                let label_size = if focused || populated { 12.0 } else { 16.0 };
                let label_rect = Rect::from_min_size(
                    rect.min + Vec2::new(16.0, label_y - label_height / 2.0),
                    Vec2::new(rect.width() - 32.0, label_height),
                );

                // For outlined variant, draw background behind floating label
                if variant == FieldVariant::Outlined && (focused || populated) {
                    let text_width = label_string.len() as f32 * label_size * 0.6; // Approximate
                    let label_bg_rect = Rect::from_min_size(
                        label_rect.min + Vec2::new(-4.0, 0.0),
                        Vec2::new(text_width + 8.0, label_height),
                    );
                    ui.painter().rect_filled(label_bg_rect, 0.0, md_surface);
                }

                ui.scope_builder(egui::UiBuilder::new().max_rect(label_rect), |ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.style_mut().override_text_style = Some(if focused || populated {
                            TextStyle::Small
                        } else {
                            TextStyle::Body
                        });
                        ui.colored_label(label_color, label_string);
                    });
                });
            }

            // Draw content
            if let Some(content_fn) = content {
                let content_rect = Rect::from_min_size(
                    rect.min + Vec2::new(16.0, if focused || populated { 24.0 } else { 16.0 }),
                    Vec2::new(rect.width() - 32.0, content_height),
                );

                let content_response = ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        content_fn(ui)
                    }).inner
                });

                response = response.union(content_response.response);
            }
        }

        // Draw supporting text        
        if supporting_text_value.is_some() || counter_text_value.is_some() {
            let supporting_height = 16.0;
            let (supporting_rect, _) = ui.allocate_exact_size(
                Vec2::new(desired_width, supporting_height),
                Sense::hover(),
            );

            if ui.is_rect_visible(supporting_rect) {
                ui.scope_builder(egui::UiBuilder::new().max_rect(supporting_rect), |ui| {
                    ui.horizontal(|ui| {
                        if let Some(text) = supporting_text_value {
                            let text_color = if error && !disabled {
                                md_error
                            } else if disabled {
                                Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97)
                            } else {
                                md_on_surface_variant
                            };
                            ui.style_mut().override_text_style = Some(TextStyle::Small);
                            ui.colored_label(text_color, text.text());
                        }

                        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                            if let Some(counter) = counter_text_value {
                                ui.style_mut().override_text_style = Some(TextStyle::Small);
                                ui.colored_label(md_on_surface_variant, counter);
                            }
                        });
                    });
                });
            }
        }

        response
    }
}