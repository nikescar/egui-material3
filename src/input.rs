//! Material Design 3 Text Input Components
//!
//! This module implements text input fields following Material Design 3 specifications.
//!
//! # M3 Color Role Usage
//!
//! ## Filled Variant
//! - **surfaceContainerHighest**: Input container background
//! - **onSurface**: Input text color
//! - **onSurfaceVariant**: Label color (unfocused), helper text, border
//! - **primary**: Label color (focused), active border color
//! - **error**: Border and label color in error state
//! - **State layers**: onSurface @ 8% (hover), 12% (pressed)
//!
//! ## Outlined Variant
//! - **Transparent**: Container background
//! - **outline**: Border color (unfocused)
//! - **primary**: Border color (focused)
//! - **error**: Border color in error state
//! - **onSurface**: Input text color
//! - **onSurfaceVariant**: Label color (unfocused), helper text
//! - **primary**: Label color (focused)
//! - **State layers**: onSurface @ 8% (hover), 12% (pressed)
//!
//! ## Disabled State
//! - **onSurface @ 38%**: All elements use this opacity
//!
//! ## Dimensions
//! - **Height**: 56dp (both variants)
//! - **Corner radius**: 4dp (top corners for filled, all corners for outlined)
//! - **Border width**: 1dp (unfocused), 2dp (focused/error)
//! - **Label font**: 16sp (placeholder), 12sp (floating)

use crate::get_global_color;
use egui::{self, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, TextStyle, Ui, Vec2, Widget};

/// Material Design text input field following Material Design 3 specifications
///
/// Provides text input with two visual variants (filled and outlined) and support for
/// labels, helper text, error states, icons, and character counting.
///
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut text = String::new();
///
/// // Basic filled input
/// ui.add(MaterialInput::filled(&mut text));
///
/// // Outlined input with label
/// ui.add(MaterialInput::outlined(&mut text)
///     .label("Email")
///     .helper_text("Enter your email address"));
///
/// // Input with error state
/// let mut email = String::new();
/// ui.add(MaterialInput::filled(&mut email)
///     .label("Email")
///     .error_text("Invalid email format")
///     .is_error(!email.contains('@')));
/// # });
/// ```
///
/// ## Material Design Spec
/// - Height: 56dp
/// - Corner radius: 4dp
/// - Label: Floating (12sp) or placeholder (16sp)
/// - States: Normal, hover, focus, error, disabled
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialInput<'a> {
    /// Mutable reference to the text content
    text: &'a mut String,
    /// Visual variant of the input (filled or outlined)
    variant: InputVariant,
    /// Optional floating label text
    label: Option<String>,
    /// Optional placeholder text (shown when empty and not focused)
    placeholder: Option<String>,
    /// Optional helper text shown below input
    helper_text: Option<String>,
    /// Optional error text shown below input (takes precedence over helper text)
    error_text: Option<String>,
    /// Optional leading icon (Material Symbol name)
    leading_icon: Option<String>,
    /// Optional trailing icon (Material Symbol name)
    trailing_icon: Option<String>,
    /// Whether the input is interactive
    enabled: bool,
    /// Whether the input is read-only (shows value but no editing)
    readonly: bool,
    /// Whether to show error state styling
    is_error: bool,
    /// Whether to mask text (password field)
    password: bool,
    /// Maximum character length (None = unlimited)
    max_length: Option<usize>,
    /// Whether to show character count below input
    show_character_count: bool,
    /// Optional hint text (shown in trailing icon area)
    hint: Option<String>,
}

/// Visual variants for Material Design text inputs
#[derive(Clone, Copy, PartialEq)]
pub enum InputVariant {
    /// Filled input with surface background and bottom border
    Filled,
    /// Outlined input with transparent background and full border
    Outlined,
}

impl<'a> MaterialInput<'a> {
    /// Create a new filled Material Design text input (default variant)
    ///
    /// ## Parameters
    /// - `text`: Mutable reference to String containing input value
    ///
    /// ## Returns
    /// A new MaterialInput instance with filled variant
    pub fn filled(text: &'a mut String) -> Self {
        Self::new(text, InputVariant::Filled)
    }

    /// Create a new outlined Material Design text input
    ///
    /// ## Parameters
    /// - `text`: Mutable reference to String containing input value
    ///
    /// ## Returns
    /// A new MaterialInput instance with outlined variant
    pub fn outlined(text: &'a mut String) -> Self {
        Self::new(text, InputVariant::Outlined)
    }

    /// Create a new Material Design text input with specified variant
    ///
    /// ## Parameters
    /// - `text`: Mutable reference to String containing input value
    /// - `variant`: Visual variant (Filled or Outlined)
    ///
    /// ## Returns
    /// A new MaterialInput instance
    fn new(text: &'a mut String, variant: InputVariant) -> Self {
        Self {
            text,
            variant,
            label: None,
            placeholder: None,
            helper_text: None,
            error_text: None,
            leading_icon: None,
            trailing_icon: None,
            enabled: true,
            readonly: false,
            is_error: false,
            password: false,
            max_length: None,
            show_character_count: false,
            hint: None,
        }
    }

    /// Set the floating label text
    ///
    /// The label floats above the input when focused or when text is present.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the placeholder text
    ///
    /// Shown when the input is empty and not focused.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the helper text shown below the input
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Set the error text shown below the input
    ///
    /// Takes precedence over helper text when is_error is true.
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    /// Set the leading icon (Material Symbol name)
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set the trailing icon (Material Symbol name)
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set whether the input is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set whether the input is read-only
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Set whether to show error state styling
    pub fn is_error(mut self, is_error: bool) -> Self {
        self.is_error = is_error;
        self
    }

    /// Set whether to mask the text (password field)
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Set maximum character length
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Set whether to show character count below input
    pub fn show_character_count(mut self, show: bool) -> Self {
        self.show_character_count = show;
        self
    }

    /// Set hint text shown in trailing icon area
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

/// M3 dimensions for text inputs
const INPUT_HEIGHT: f32 = 56.0;
const INPUT_CORNER_RADIUS: f32 = 4.0;
const LABEL_FONT_SIZE_NORMAL: f32 = 16.0;
const LABEL_FONT_SIZE_FLOATING: f32 = 12.0;
const BORDER_WIDTH_NORMAL: f32 = 1.0;
const BORDER_WIDTH_FOCUSED: f32 = 2.0;
const HORIZONTAL_PADDING: f32 = 16.0;
const ICON_SIZE: f32 = 24.0;
const ICON_PADDING: f32 = 12.0;

impl<'a> Widget for MaterialInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let is_focused = ui.memory(|mem| mem.has_focus(ui.next_auto_id()));
        let has_content = !self.text.is_empty();
        let should_float_label = is_focused || has_content;

        // Calculate dimensions
        let available_width = ui.available_width();
        let mut content_width = available_width;

        // Reserve space for icons
        if self.leading_icon.is_some() {
            content_width -= ICON_SIZE + ICON_PADDING * 2.0;
        }
        if self.trailing_icon.is_some() {
            content_width -= ICON_SIZE + ICON_PADDING * 2.0;
        }

        let desired_size = Vec2::new(available_width, INPUT_HEIGHT);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        // M3 Color Roles
        let primary = get_global_color("primary");
        let error = get_global_color("error");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let surface_container_highest = get_global_color("surfaceContainerHighest");
        let outline = get_global_color("outline");

        // Determine colors based on state
        let (border_color, label_color) = if !self.enabled {
            (on_surface.linear_multiply(0.38), on_surface.linear_multiply(0.38))
        } else if self.is_error {
            (error, error)
        } else if is_focused {
            (primary, primary)
        } else {
            (
                if self.variant == InputVariant::Outlined { outline } else { on_surface_variant },
                on_surface_variant
            )
        };

        let border_width = if is_focused || self.is_error {
            BORDER_WIDTH_FOCUSED
        } else {
            BORDER_WIDTH_NORMAL
        };

        // Draw container
        match self.variant {
            InputVariant::Filled => {
                // Filled background
                let bg_color = if self.enabled {
                    surface_container_highest
                } else {
                    surface_container_highest.linear_multiply(0.38)
                };

                ui.painter().rect_filled(
                    rect,
                    egui::Rounding {
                        nw: INPUT_CORNER_RADIUS as u8,
                        ne: INPUT_CORNER_RADIUS as u8,
                        sw: 0,
                        se: 0,
                    },
                    bg_color,
                );

                // Bottom border
                let border_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x, rect.max.y - border_width),
                    Vec2::new(rect.width(), border_width),
                );
                ui.painter().rect_filled(border_rect, 0.0, border_color);
            }
            InputVariant::Outlined => {
                // Outlined border
                ui.painter().rect_stroke(
                    rect,
                    INPUT_CORNER_RADIUS,
                    Stroke::new(border_width, border_color),
                    egui::epaint::StrokeKind::Outside,
                );
            }
        }

        // Draw floating label or placeholder
        if let Some(ref label_text) = self.label {
            let label_font_size = if should_float_label {
                LABEL_FONT_SIZE_FLOATING
            } else {
                LABEL_FONT_SIZE_NORMAL
            };

            let label_y = if should_float_label {
                rect.min.y + 8.0
            } else {
                rect.center().y
            };

            let label_x = rect.min.x + HORIZONTAL_PADDING +
                if self.leading_icon.is_some() { ICON_SIZE + ICON_PADDING * 2.0 } else { 0.0 };

            ui.painter().text(
                Pos2::new(label_x, label_y),
                egui::Align2::LEFT_TOP,
                label_text,
                FontId::proportional(label_font_size),
                label_color,
            );
        } else if let Some(ref placeholder_text) = self.placeholder {
            if !has_content && !is_focused {
                let placeholder_x = rect.min.x + HORIZONTAL_PADDING +
                    if self.leading_icon.is_some() { ICON_SIZE + ICON_PADDING * 2.0 } else { 0.0 };

                ui.painter().text(
                    Pos2::new(placeholder_x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    placeholder_text,
                    FontId::proportional(LABEL_FONT_SIZE_NORMAL),
                    on_surface_variant,
                );
            }
        }

        // Calculate text input position
        let text_y_offset = if should_float_label && self.label.is_some() {
            24.0
        } else {
            0.0
        };

        let text_x = rect.min.x + HORIZONTAL_PADDING +
            if self.leading_icon.is_some() { ICON_SIZE + ICON_PADDING * 2.0 } else { 0.0 };

        let text_rect = Rect::from_min_size(
            Pos2::new(text_x, rect.min.y + text_y_offset),
            Vec2::new(content_width - HORIZONTAL_PADDING * 2.0, INPUT_HEIGHT - text_y_offset),
        );

        // Draw text input
        let text_response = ui.allocate_ui_at_rect(text_rect, |ui| {
            ui.style_mut().spacing.interact_size.y = 0.0;

            let mut text_edit = egui::TextEdit::singleline(self.text)
                .font(TextStyle::Body)
                .desired_width(text_rect.width())
                .frame(false);

            if self.password {
                text_edit = text_edit.password(true);
            }

            if !self.enabled || self.readonly {
                text_edit = text_edit.interactive(false);
            }

            ui.add(text_edit)
        }).inner;

        response = response.union(text_response);

        // Enforce max length
        if let Some(max) = self.max_length {
            if self.text.len() > max {
                self.text.truncate(max);
            }
        }

        // Draw leading icon
        if let Some(ref icon) = self.leading_icon {
            let icon_x = rect.min.x + ICON_PADDING;
            let icon_y = rect.center().y;

            ui.painter().text(
                Pos2::new(icon_x, icon_y),
                egui::Align2::LEFT_CENTER,
                crate::material_symbol::material_symbol_text(icon),
                FontId::proportional(ICON_SIZE),
                if self.enabled { on_surface_variant } else { on_surface.linear_multiply(0.38) },
            );
        }

        // Draw trailing icon
        if let Some(ref icon) = self.trailing_icon {
            let icon_x = rect.max.x - ICON_SIZE - ICON_PADDING;
            let icon_y = rect.center().y;

            ui.painter().text(
                Pos2::new(icon_x, icon_y),
                egui::Align2::LEFT_CENTER,
                crate::material_symbol::material_symbol_text(icon),
                FontId::proportional(ICON_SIZE),
                if self.enabled { on_surface_variant } else { on_surface.linear_multiply(0.38) },
            );
        }

        // Draw helper/error text below input
        ui.allocate_ui_at_rect(
            Rect::from_min_size(
                Pos2::new(rect.min.x + HORIZONTAL_PADDING, rect.max.y + 4.0),
                Vec2::new(rect.width() - HORIZONTAL_PADDING * 2.0, 16.0),
            ),
            |ui| {
                ui.horizontal(|ui| {
                    if self.is_error {
                        if let Some(ref error_text) = self.error_text {
                            ui.label(egui::RichText::new(error_text).size(12.0).color(error));
                        }
                    } else if let Some(ref helper) = self.helper_text {
                        ui.label(egui::RichText::new(helper).size(12.0).color(on_surface_variant));
                    }

                    // Character counter
                    if self.show_character_count {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let counter_text = if let Some(max) = self.max_length {
                                format!("{}/{}", self.text.len(), max)
                            } else {
                                format!("{}", self.text.len())
                            };
                            ui.label(egui::RichText::new(counter_text).size(12.0).color(on_surface_variant));
                        });
                    }
                });
            },
        );

        response
    }
}
