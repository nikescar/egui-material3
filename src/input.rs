//! Nala / Leo text input component
//!
//! Implements the Leo `input.svelte` + `formItem.svelte` visual model: label above
//! the field, 8px radius, outline/filled/plain modes, and a primary focus ring.

use crate::{
    get_global_color,
    material_symbol::material_symbol_text,
    theme::{get_global_theme, ThemeMode},
};
use egui::{
    epaint::{CornerRadius, Shadow, Stroke},
    Align, Align2, Color32, FontFamily, FontId, Margin, Pos2, Rect, Response, RichText, Sense,
    TextEdit, Ui, Vec2, Widget, WidgetInfo, WidgetType,
};

/// Nala input mode (Leo `FormItem` modes)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum InputMode {
    /// White/container background with subtle border
    #[default]
    Outline,
    /// Tinted container background
    Filled,
    /// Borderless, minimal chrome
    Plain,
}

/// Nala input size (Leo `FormItem` sizes)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum InputSize {
    Small,
    #[default]
    Normal,
    Large,
}

pub type MaterialTextFieldSize = InputSize;
pub type MaterialTextFieldMode = InputMode;

#[derive(Clone, Copy)]
struct NalaInputLayout {
    padding_x: f32,
    padding_y: f32,
    font_size: f32,
    line_height: f32,
    icon_size: f32,
    icon_gap: f32,
    label_font_size: f32,
    label_gap: f32,
    helper_font_size: f32,
    radius: f32,
}

impl InputSize {
    fn layout(self) -> NalaInputLayout {
        match self {
            InputSize::Small => NalaInputLayout {
                padding_x: 8.0,
                padding_y: 8.0,
                font_size: 12.0,
                line_height: 18.0,
                icon_size: 16.0,
                icon_gap: 8.0,
                label_font_size: 12.0,
                label_gap: 2.0,
                helper_font_size: 12.0,
                radius: 8.0,
            },
            InputSize::Normal => NalaInputLayout {
                padding_x: 8.0,
                padding_y: 11.0,
                font_size: 14.0,
                line_height: 22.0,
                icon_size: 20.0,
                icon_gap: 8.0,
                label_font_size: 14.0,
                label_gap: 4.0,
                helper_font_size: 12.0,
                radius: 8.0,
            },
            InputSize::Large => NalaInputLayout {
                padding_x: 16.0,
                padding_y: 14.0,
                font_size: 16.0,
                line_height: 24.0,
                icon_size: 22.0,
                icon_gap: 8.0,
                label_font_size: 16.0,
                label_gap: 12.0,
                helper_font_size: 12.0,
                radius: 8.0,
            },
        }
    }
}

fn is_dark_theme() -> bool {
    get_global_theme()
        .lock()
        .map(|theme| matches!(theme.theme_mode, ThemeMode::Dark))
        .unwrap_or(false)
}

fn foreground_color(dark: bool) -> Color32 {
    if dark {
        Color32::WHITE
    } else {
        Color32::BLACK
    }
}

fn color_mix_srgb(base: Color32, mix: Color32, mix_amount: f32) -> Color32 {
    let base_amount = 1.0 - mix_amount;
    Color32::from_rgb(
        ((base.r() as f32 * base_amount) + (mix.r() as f32 * mix_amount)).round() as u8,
        ((base.g() as f32 * base_amount) + (mix.g() as f32 * mix_amount)).round() as u8,
        ((base.b() as f32 * base_amount) + (mix.b() as f32 * mix_amount)).round() as u8,
    )
}

fn with_opacity(color: Color32, opacity: f32) -> Color32 {
    color.linear_multiply(opacity)
}

fn nala_hover_shadow() -> Shadow {
    Shadow {
        offset: [0, 1],
        blur: 3,
        spread: 0,
        color: Color32::from_black_alpha(25),
    }
}

fn paint_icon(ui: &Ui, rect: Rect, icon_name: &str, layout: NalaInputLayout, color: Color32) {
    let icon_str = material_symbol_text(icon_name);
    ui.painter().text(
        Pos2::new(rect.center().x, rect.center().y),
        Align2::CENTER_CENTER,
        icon_str,
        FontId::proportional(layout.icon_size),
        color,
    );
}
pub struct MaterialInput<'a> {
    value: &'a mut String,
    label: Option<String>,
    placeholder: Option<String>,
    mode: InputMode,
    size: InputSize,
    enabled: bool,
    required: bool,
    error_text: Option<String>,
    helper_text: Option<String>,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    extra_text: Option<String>,
    width: Option<f32>,
    max_chars: Option<usize>,
}

/// Alias for README / legacy naming.
pub type MaterialTextField<'a> = MaterialInput<'a>;

impl<'a> MaterialInput<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            label: None,
            placeholder: None,
            mode: InputMode::default(),
            size: InputSize::default(),
            enabled: true,
            required: false,
            error_text: None,
            helper_text: None,
            leading_icon: None,
            trailing_icon: None,
            extra_text: None,
            width: None,
            max_chars: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn mode(mut self, mode: InputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub fn extra_text(mut self, text: impl Into<String>) -> Self {
        self.extra_text = Some(text.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn max_chars(mut self, max_chars: usize) -> Self {
        self.max_chars = Some(max_chars);
        self
    }
}

pub fn input(value: &mut String) -> MaterialInput<'_> {
    MaterialInput::new(value)
}

fn paint_container(
    ui: &Ui,
    rect: Rect,
    layout: NalaInputLayout,
    mode: InputMode,
    enabled: bool,
    has_error: bool,
    hovered: bool,
    focused: bool,
    dark: bool,
) {
    let primary = get_global_color("primary");
    let surface_lowest = get_global_color("surfaceContainerLowest");
    let surface_variant = get_global_color("surfaceVariant");
    let error = get_global_color("error");
    let foreground = foreground_color(dark);

    let (mut background, mut border_color, show_border) = match mode {
        InputMode::Outline => (surface_lowest, surface_variant, true),
        InputMode::Filled => (
            color_mix_srgb(surface_lowest, foreground, 0.10).gamma_multiply(1.0),
            Color32::TRANSPARENT,
            true,
        ),
        InputMode::Plain => (Color32::TRANSPARENT, Color32::TRANSPARENT, false),
    };

    if mode == InputMode::Filled && !enabled {
        background = surface_variant;
    }

    if has_error {
        border_color = error;
    } else if hovered && enabled && show_border {
        border_color = color_mix_srgb(surface_lowest, foreground, 0.20);
    }

    if !enabled {
        background = surface_variant;
    }

    let radius = CornerRadius::same(layout.radius as u8);
    let painter = ui.painter();

    if enabled && hovered && mode != InputMode::Plain {
        let shadow = nala_hover_shadow();
        let shadow_rect = rect.translate(Vec2::new(
            shadow.offset[0] as f32,
            shadow.offset[1] as f32,
        ));
        painter.rect_filled(shadow_rect, radius, shadow.color);
    }

    painter.rect_filled(rect, radius, background);

    // Leo hides the inner border on focus — only the outer focus ring remains.
    if show_border && !(focused && enabled) {
        painter.rect_stroke(
            rect,
            radius,
            Stroke::new(1.0, border_color),
            egui::epaint::StrokeKind::Inside,
        );
    }

    if focused && enabled {
        painter.rect_stroke(
            rect.expand(2.0),
            CornerRadius::same((layout.radius as u8).saturating_add(2)),
            Stroke::new(2.0, primary),
            egui::epaint::StrokeKind::Outside,
        );
    }
}

impl Widget for MaterialInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let layout = self.size.layout();
        let dark = is_dark_theme();
        let has_error = self.error_text.is_some();
        let width = self.width.unwrap_or_else(|| ui.available_width());

        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let error = get_global_color("error");

        let container_height = if self.mode == InputMode::Plain {
            layout.line_height
        } else {
            layout.padding_y * 2.0 + layout.line_height
        };

        let outer_response = ui
            .vertical(|ui| {
            ui.set_width(width);

            if let Some(label) = &self.label {
                ui.horizontal(|ui| {
                    let label_color = if has_error { error } else { on_surface };
                    ui.label(
                        RichText::new(label)
                            .size(layout.label_font_size)
                            .color(label_color),
                    );
                    if self.required {
                        ui.label(RichText::new("*").size(layout.label_font_size).color(error));
                    }
                });
                ui.add_space(layout.label_gap);
            }

            let (container_rect, container_response) = ui.allocate_exact_size(
                Vec2::new(width, container_height),
                if self.enabled {
                    Sense::click()
                } else {
                    Sense::hover()
                },
            );

            let text_id = container_response.id.with("text");
            let focused = ui.memory(|m| m.has_focus(text_id));
            let hovered = container_response.hovered() || ui.rect_contains_pointer(container_rect);

            if container_response.clicked() && self.enabled {
                ui.memory_mut(|m| m.request_focus(text_id));
            }

            paint_container(
                ui,
                container_rect,
                layout,
                self.mode,
                self.enabled,
                has_error,
                hovered,
                focused,
                dark,
            );

            let mut content_left = container_rect.min.x;
            let mut content_right = container_rect.max.x;

            if self.mode != InputMode::Plain {
                content_left += layout.padding_x;
                content_right -= layout.padding_x;
            }

            if let Some(icon_name) = &self.leading_icon {
                let icon_rect = Rect::from_min_max(
                    Pos2::new(content_left, container_rect.min.y),
                    Pos2::new(content_left + layout.icon_size, container_rect.max.y),
                );
                let icon_color = if self.enabled {
                    on_surface_variant
                } else {
                    with_opacity(on_surface, 0.5)
                };
                paint_icon(ui, icon_rect, icon_name, layout, icon_color);
                content_left += layout.icon_size + layout.icon_gap;
            }

            let extra_width = if self.extra_text.is_some() {
                48.0
            } else {
                0.0
            };

            if let Some(icon_name) = &self.trailing_icon {
                let icon_rect = Rect::from_min_max(
                    Pos2::new(
                        content_right - layout.icon_size,
                        container_rect.min.y,
                    ),
                    Pos2::new(content_right, container_rect.max.y),
                );
                let icon_color = if self.enabled {
                    on_surface_variant
                } else {
                    with_opacity(on_surface, 0.5)
                };
                paint_icon(ui, icon_rect, icon_name, layout, icon_color);
                content_right -= layout.icon_size + layout.icon_gap;
            }

            content_right -= extra_width;

            let text_rect = Rect::from_min_max(
                Pos2::new(content_left + 4.0, container_rect.min.y),
                Pos2::new(content_right, container_rect.max.y),
            );

            let text_width = text_rect.width().max(0.0);
            let text_height = text_rect.height().max(0.0);

            let text_color = if self.enabled {
                on_surface
            } else {
                with_opacity(on_surface, 0.5)
            };

            let mut text_edit = TextEdit::singleline(self.value)
                .id(text_id)
                .frame(false)
                .margin(Margin::ZERO)
                .desired_width(text_width)
                .min_size(Vec2::new(text_width, text_height))
                .vertical_align(Align::Center)
                .text_color(text_color)
                .font(FontId::new(
                    layout.font_size,
                    FontFamily::Proportional,
                ))
                .interactive(self.enabled);

            if let Some(placeholder) = &self.placeholder {
                text_edit = text_edit.hint_text(
                    RichText::new(placeholder)
                        .color(with_opacity(on_surface_variant, 0.6)),
                );
            }
            if let Some(max_chars) = self.max_chars {
                text_edit = text_edit.char_limit(max_chars);
            }

            let text_response = ui.put(text_rect, text_edit);

            if let Some(extra) = &self.extra_text {
                let font = FontId::new(layout.helper_font_size, FontFamily::Proportional);
                ui.painter().text(
                    Pos2::new(container_rect.max.x - layout.padding_x, container_rect.center().y),
                    Align2::RIGHT_CENTER,
                    extra,
                    font,
                    on_surface_variant,
                );
            }

            let response = container_response | text_response;

            if let Some(error_text) = &self.error_text {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(error_text)
                        .size(layout.helper_font_size)
                        .color(error),
                );
            } else if let Some(helper_text) = &self.helper_text {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(helper_text)
                        .size(layout.helper_font_size)
                        .color(on_surface_variant),
                );
            }

            response
        })
        .inner;

        outer_response.widget_info(|| {
            WidgetInfo::new(WidgetType::TextEdit)
        });

        outer_response
    }
}
