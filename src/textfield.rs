//! Material Design 3 Text Field component
//!
//! Text fields allow users to input text. They follow the Material Design 3
//! specification for text inputs with support for filled and outlined variants.
//!
//! # Examples
//!
//! ```rust
//! use egui_material3::*;
//!
//! let mut text = String::new();
//! 
//! // Filled text field (default Material Design 3 style)
//! ui.add(
//!     text_field(&mut text)
//!         .label("Email")
//!         .hint("example@email.com")
//!         .helper_text("We'll never share your email")
//! );
//!
//! // Outlined text field
//! ui.add(
//!     text_field_outlined(&mut text)
//!         .label("Username")
//!         .prefix_icon("person")
//! );
//!
//! // Password field
//! ui.add(
//!     text_field(&mut text)
//!         .label("Password")
//!         .password(true)
//!         .error_text("Password must be at least 8 characters")
//! );
//! ```

use eframe::egui::{self, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, TextEdit, Ui, Vec2, Widget};
use crate::theme::{get_global_theme, MaterialThemeContext};

/// Text field variant following Material Design 3 specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFieldVariant {
    /// Filled text field with background color (default)
    Filled,
    /// Outlined text field with border
    Outlined,
}

/// Material Design 3 Text Field widget
///
/// A text input field following Material Design 3 guidelines with support
/// for labels, icons, helper text, error states, and more.
pub struct MaterialTextField<'a> {
    text: &'a mut String,
    variant: TextFieldVariant,
    label: Option<String>,
    hint: Option<String>,
    helper_text: Option<String>,
    error_text: Option<String>,
    prefix_icon: Option<String>,
    suffix_icon: Option<String>,
    enabled: bool,
    password: bool,
    multiline: Option<usize>,
    max_length: Option<usize>,
    show_counter: bool,
    desired_width: Option<f32>,
}

impl<'a> MaterialTextField<'a> {
    /// Create a new filled text field (default Material Design 3 style)
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            variant: TextFieldVariant::Filled,
            label: None,
            hint: None,
            helper_text: None,
            error_text: None,
            prefix_icon: None,
            suffix_icon: None,
            enabled: true,
            password: false,
            multiline: None,
            max_length: None,
            show_counter: false,
            desired_width: None,
        }
    }

    /// Create a new outlined text field
    pub fn outlined(text: &'a mut String) -> Self {
        Self {
            variant: TextFieldVariant::Outlined,
            ..Self::new(text)
        }
    }

    /// Set the label text (floats up when focused or has content)
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the hint text (placeholder when empty)
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Set helper text shown below the field
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Set error text (replaces helper text and changes styling)
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    /// Set prefix icon (Material Symbol name)
    pub fn prefix_icon(mut self, icon: impl Into<String>) -> Self {
        self.prefix_icon = Some(icon.into());
        self
    }

    /// Set suffix icon (Material Symbol name)
    pub fn suffix_icon(mut self, icon: impl Into<String>) -> Self {
        self.suffix_icon = Some(icon.into());
        self
    }

    /// Enable or disable the text field
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Obscure text input (for passwords)
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Enable multiline input with specified number of rows
    pub fn multiline(mut self, lines: usize) -> Self {
        self.multiline = Some(lines);
        self
    }

    /// Set maximum character length
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Show character counter (requires max_length to be set)
    pub fn show_counter(mut self, show: bool) -> Self {
        self.show_counter = show;
        self
    }

    /// Set desired width of the text field
    pub fn desired_width(mut self, width: f32) -> Self {
        self.desired_width = Some(width);
        self
    }

    fn get_colors(&self, theme: &MaterialThemeContext) -> TextFieldColors {
        let has_error = self.error_text.is_some();
        let is_dark = theme.theme_mode == crate::theme::ThemeMode::Dark;

        TextFieldColors {
            primary: theme.get_primary_color(),
            on_surface: theme.get_color_by_name("onSurface"),
            on_surface_variant: theme.get_color_by_name("onSurfaceVariant"),
            outline: theme.get_color_by_name("outline"),
            outline_variant: theme.get_color_by_name("outlineVariant"),
            surface_container_highest: theme.get_color_by_name("surfaceContainerHighest"),
            error: theme.get_color_by_name("error"),
            on_error_container: theme.get_color_by_name("onErrorContainer"),
            disabled_opacity: 0.38,
            has_error,
            is_dark,
        }
    }
}

struct TextFieldColors {
    primary: Color32,
    on_surface: Color32,
    on_surface_variant: Color32,
    outline: Color32,
    outline_variant: Color32,
    surface_container_highest: Color32,
    error: Color32,
    on_error_container: Color32,
    disabled_opacity: f32,
    has_error: bool,
    is_dark: bool,
}

impl<'a> Widget for MaterialTextField<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = get_global_theme().lock().unwrap().clone();
        let colors = self.get_colors(&theme);

        // Calculate dimensions
        let available_width = self.desired_width.unwrap_or_else(|| ui.available_width());
        let field_height = match self.variant {
            TextFieldVariant::Filled => 56.0,
            TextFieldVariant::Outlined => 56.0,
        };
        
        let has_supporting_text = self.helper_text.is_some() || self.error_text.is_some() || (self.show_counter && self.max_length.is_some());
        let total_height = if has_supporting_text {
            field_height + 24.0 // Add space for helper text
        } else {
            field_height
        };

        let (rect, mut response) = ui.allocate_exact_size(
            Vec2::new(available_width, total_height),
            Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let field_rect = Rect::from_min_size(rect.min, Vec2::new(available_width, field_height));
            
            // Determine state
            let has_focus = response.has_focus();
            let has_content = !self.text.is_empty();
            let is_hovered = response.hovered();
            let label_floated = has_focus || has_content;

            // Draw background and border based on variant
            match self.variant {
                TextFieldVariant::Filled => {
                    let bg_color = if !self.enabled {
                        with_alpha(colors.on_surface, 0.04)
                    } else if colors.has_error {
                        with_alpha(colors.error, 0.08)
                    } else {
                        colors.surface_container_highest
                    };

                    // Draw filled background with rounded top corners
                    ui.painter().rect_filled(
                        field_rect,
                        4.0, // Radius for rounded corners
                        bg_color,
                    );

                    // Draw bottom border
                    let border_color = if !self.enabled {
                        with_alpha(colors.on_surface, colors.disabled_opacity)
                    } else if colors.has_error {
                        colors.error
                    } else if has_focus {
                        colors.primary
                    } else if is_hovered {
                        colors.on_surface
                    } else {
                        colors.on_surface_variant
                    };
                    
                    let border_width = if has_focus { 2.0 } else { 1.0 };
                    let border_y = field_rect.max.y;
                    ui.painter().line_segment(
                        [Pos2::new(field_rect.min.x, border_y), Pos2::new(field_rect.max.x, border_y)],
                        Stroke::new(border_width, border_color),
                    );
                }
                TextFieldVariant::Outlined => {
                    let border_color = if !self.enabled {
                        with_alpha(colors.on_surface, colors.disabled_opacity)
                    } else if colors.has_error {
                        colors.error
                    } else if has_focus {
                        colors.primary
                    } else if is_hovered {
                        colors.on_surface
                    } else {
                        colors.outline
                    };
                    
                    let border_width = if has_focus { 2.0 } else { 1.0 };
                    let stroke = Stroke::new(border_width, border_color);
                    ui.painter().rect_stroke(
                        field_rect,
                        4.0,
                        stroke,
                        egui::epaint::StrokeKind::Outside,
                    );
                }
            }

            // Calculate content area (accounting for icons and padding)
            let padding = 16.0;
            let icon_size = 24.0;
            let icon_spacing = 12.0;

            let mut content_min_x = field_rect.min.x + padding;
            let mut content_max_x = field_rect.max.x - padding;

            // Draw prefix icon
            if let Some(prefix_icon) = &self.prefix_icon {
                let icon_rect = Rect::from_min_size(
                    Pos2::new(content_min_x, field_rect.center().y - icon_size / 2.0),
                    Vec2::new(icon_size, icon_size),
                );
                draw_material_icon(ui, prefix_icon, icon_rect, colors.on_surface_variant);
                content_min_x += icon_size + icon_spacing;
            }

            // Draw suffix icon
            if let Some(suffix_icon) = &self.suffix_icon {
                let icon_rect = Rect::from_min_size(
                    Pos2::new(content_max_x - icon_size, field_rect.center().y - icon_size / 2.0),
                    Vec2::new(icon_size, icon_size),
                );
                draw_material_icon(ui, suffix_icon, icon_rect, colors.on_surface_variant);
                content_max_x -= icon_size + icon_spacing;
            }

            // Draw label
            if let Some(ref label_text) = self.label {
                let label_color = if !self.enabled {
                    with_alpha(colors.on_surface, colors.disabled_opacity)
                } else if colors.has_error {
                    colors.error
                } else if has_focus {
                    colors.primary
                } else {
                    colors.on_surface_variant
                };

                let (label_pos, label_font) = if label_floated {
                    // Small label at top
                    (
                        Pos2::new(content_min_x, field_rect.min.y + 8.0),
                        FontId::proportional(12.0),
                    )
                } else {
                    // Regular label in center
                    (
                        Pos2::new(content_min_x, field_rect.center().y - 8.0),
                        FontId::proportional(16.0),
                    )
                };

                ui.painter().text(
                    label_pos,
                    egui::Align2::LEFT_TOP,
                    label_text,
                    label_font,
                    label_color,
                );
            }

            // Draw text input
            let text_y_offset = if label_floated && self.label.is_some() { 24.0 } else { 16.0 };
            let text_rect = Rect::from_min_max(
                Pos2::new(content_min_x, field_rect.min.y + text_y_offset),
                Pos2::new(content_max_x, field_rect.max.y - 8.0),
            );

            let mut text_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(text_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
            );
            
            let mut text_edit = if let Some(lines) = self.multiline {
                TextEdit::multiline(self.text).desired_rows(lines)
            } else {
                TextEdit::singleline(self.text)
            };

            if self.password {
                text_edit = text_edit.password(true);
            }

            if let Some(hint) = &self.hint {
                if self.label.is_none() || !label_floated {
                    text_edit = text_edit.hint_text(hint);
                }
            }

            text_edit = text_edit
                .desired_width(text_rect.width())
                .frame(false);

            let text_response = text_ui.add(text_edit);
            
            if text_response.changed() {
                // Enforce max length
                if let Some(max_len) = self.max_length {
                    if self.text.len() > max_len {
                        self.text.truncate(max_len);
                    }
                }
            }

            response = response.union(text_response);

            // Draw supporting text (helper text, error text, or counter)
            if has_supporting_text {
                let support_y = field_rect.max.y + 4.0;
                let support_font = FontId::proportional(12.0);

                if let Some(error_text) = &self.error_text {
                    ui.painter().text(
                        Pos2::new(field_rect.min.x + padding, support_y),
                        egui::Align2::LEFT_TOP,
                        error_text,
                        support_font.clone(),
                        colors.error,
                    );
                } else if let Some(helper_text) = &self.helper_text {
                    let helper_color = if !self.enabled {
                        with_alpha(colors.on_surface_variant, colors.disabled_opacity)
                    } else {
                        colors.on_surface_variant
                    };
                    ui.painter().text(
                        Pos2::new(field_rect.min.x + padding, support_y),
                        egui::Align2::LEFT_TOP,
                        helper_text,
                        support_font.clone(),
                        helper_color,
                    );
                }

                // Draw counter on the right
                if self.show_counter {
                    if let Some(max_len) = self.max_length {
                        let counter_text = format!("{}/{}", self.text.len(), max_len);
                        let counter_color = if self.text.len() > max_len {
                            colors.error
                        } else if !self.enabled {
                            with_alpha(colors.on_surface_variant, colors.disabled_opacity)
                        } else {
                            colors.on_surface_variant
                        };
                        ui.painter().text(
                            Pos2::new(field_rect.max.x - padding, support_y),
                            egui::Align2::RIGHT_TOP,
                            counter_text,
                            support_font,
                            counter_color,
                        );
                    }
                }
            }
        }

        response
    }
}

/// Helper function to create a filled text field
pub fn text_field(text: &mut String) -> MaterialTextField {
    MaterialTextField::new(text)
}

/// Helper function to create an outlined text field
pub fn text_field_outlined(text: &mut String) -> MaterialTextField {
    MaterialTextField::outlined(text)
}

/// Helper function to draw a Material Symbol icon
fn draw_material_icon(ui: &Ui, icon_name: &str, rect: Rect, color: Color32) {
    let font_id = FontId::proportional(24.0);
    
    // Map common icon names to Material Symbols unicode
    let icon_char = match icon_name {
        "search" => "\u{e8b6}",
        "person" => "\u{e7fd}",
        "email" => "\u{e0be}",
        "lock" => "\u{e897}",
        "visibility" => "\u{e8f4}",
        "visibility_off" => "\u{e8f5}",
        "clear" => "\u{e14c}",
        "close" => "\u{e5cd}",
        "check" => "\u{e5ca}",
        "error" => "\u{e000}",
        "info" => "\u{e88e}",
        "calendar_today" => "\u{e935}",
        "event" => "\u{e878}",
        "schedule" => "\u{e8b5}",
        _ => icon_name, // Use as-is if not in map
    };

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon_char,
        font_id,
        color,
    );
}

/// Helper function to apply alpha to a color
fn with_alpha(color: Color32, alpha: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (alpha * 255.0) as u8,
    )
}
