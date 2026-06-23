//! Nala / Leo checkbox component
//!
//! Implements the Leo `checkbox.svelte` visual model using Leo SVG icons.

use crate::{get_global_color, image_utils::create_texture_from_svg, theme::{get_global_theme, ThemeMode}};
use egui::{
    self, epaint::Galley, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, TextureHandle,
    Ui, Vec2, Widget,
};
use std::sync::Arc;

const CHECKBOX_CHECKED_SVG: &str = include_str!("../resources/checkbox-checked.svg");
const CHECKBOX_UNCHECKED_SVG: &str = include_str!("../resources/checkbox-unchecked.svg");
const CHECKBOX_INDETERMINATE_SVG: &str = include_str!("../resources/checkbox-indeterminate.svg");

/// Nala checkbox size (Leo component API)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MaterialCheckboxSize {
    Small,
    #[default]
    Normal,
}

pub type CheckboxSize = MaterialCheckboxSize;

impl MaterialCheckboxSize {
    fn icon_size(self) -> f32 {
        match self {
            MaterialCheckboxSize::Small => 16.0,
            MaterialCheckboxSize::Normal => 20.0,
        }
    }
}

fn is_dark_theme() -> bool {
    get_global_theme()
        .lock()
        .map(|theme| matches!(theme.theme_mode, ThemeMode::Dark))
        .unwrap_or(false)
}

fn color_hex(color: Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b())
}

fn tinted_svg(svg: &str, color: Color32) -> String {
    let hex = color_hex(color);
    svg.replace("#62757e", &hex).replace("#fff", &hex)
}

fn checkbox_icon_texture(
    ui: &Ui,
    svg: &str,
    icon_kind: &str,
    color: Color32,
) -> Option<TextureHandle> {
    let cache_name = format!("nala_checkbox_{icon_kind}_{}", color_hex(color));
    create_texture_from_svg(ui.ctx(), &tinted_svg(svg, color), &cache_name).ok()
}

fn center_galley_y(galley: &Galley, rect: Rect) -> f32 {
    rect.center().y - galley.mesh_bounds.center().y
}

/// Nala checkbox (Leo `Checkbox` component)
pub struct MaterialCheckbox<'a> {
    checked: &'a mut bool,
    text: String,
    indeterminate: bool,
    enabled: bool,
    is_error: bool,
    size: MaterialCheckboxSize,
    /// Legacy override for checked icon color
    check_color: Option<Color32>,
    /// Legacy override for checked icon color
    fill_color: Option<Color32>,
}

impl<'a> MaterialCheckbox<'a> {
    pub fn new(checked: &'a mut bool, text: impl Into<String>) -> Self {
        Self {
            checked,
            text: text.into(),
            indeterminate: false,
            enabled: true,
            is_error: false,
            size: MaterialCheckboxSize::Normal,
            check_color: None,
            fill_color: None,
        }
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn is_error(mut self, is_error: bool) -> Self {
        self.is_error = is_error;
        self
    }

    pub fn size(mut self, size: MaterialCheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn check_color(mut self, color: Color32) -> Self {
        self.check_color = Some(color);
        self
    }

    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    #[deprecated(note = "Nala checkboxes use fixed SVG icons; border width is ignored")]
    pub fn border_width(self, _width: f32) -> Self {
        self
    }
}

impl<'a> Widget for MaterialCheckbox<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let icon_size = self.size.icon_size();
        let label_gap = 12.0;
        let dark = is_dark_theme();

        let primary = self.fill_color.unwrap_or_else(|| get_global_color("primary"));
        let error = get_global_color("error");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let checked_color = self.check_color.unwrap_or(if self.is_error {
            error
        } else {
            primary
        });
        let checked_hover = if dark {
            Color32::from_rgb(91, 103, 232)
        } else {
            Color32::from_rgb(67, 79, 207)
        };
        let unchecked_color = on_surface_variant;
        let unchecked_hover = outline;
        let disabled_color = on_surface.linear_multiply(0.38);

        let text_galley: Option<Arc<Galley>> = if self.text.is_empty() {
            None
        } else {
            Some(ui.painter().layout(
                self.text.clone(),
                FontId::default(),
                on_surface,
                f32::INFINITY,
            ))
        };

        let text_width = text_galley.as_ref().map(|g| g.size().x).unwrap_or(0.0);
        let text_height = text_galley
            .as_ref()
            .map(|g| g.mesh_bounds.height())
            .unwrap_or(0.0);
        let desired_size = Vec2::new(
            icon_size + if text_galley.is_some() { label_gap + text_width } else { 0.0 },
            icon_size.max(text_height),
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

        let is_hovered = response.hovered() && self.enabled;
        let is_focused = response.has_focus() && self.enabled;

        let icon_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - icon_size / 2.0),
            Vec2::splat(icon_size),
        );

        let icon_color = if !self.enabled {
            disabled_color
        } else if self.indeterminate || *self.checked {
            if is_hovered {
                checked_hover
            } else {
                checked_color
            }
        } else if is_hovered {
            unchecked_hover
        } else {
            unchecked_color
        };

        let checked_opacity = if self.indeterminate {
            0.0
        } else {
            ui.ctx()
                .animate_bool(response.id.with("nala_checkbox_checked"), *self.checked)
        };
        let unchecked_opacity = if self.indeterminate {
            0.0
        } else {
            1.0 - checked_opacity
        };

        if is_focused {
            ui.painter().rect_stroke(
                icon_rect.expand(1.5),
                2.0,
                Stroke::new(2.0, primary.linear_multiply(0.85)),
                egui::epaint::StrokeKind::Outside,
            );
        }

        if self.indeterminate {
            if let Some(texture) =
                checkbox_icon_texture(ui, CHECKBOX_INDETERMINATE_SVG, "indeterminate", icon_color)
            {
                ui.painter().image(
                    texture.id(),
                    icon_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
            }
        } else {
            if unchecked_opacity > 0.01 {
                if let Some(texture) =
                    checkbox_icon_texture(ui, CHECKBOX_UNCHECKED_SVG, "unchecked", icon_color)
                {
                    ui.painter().image(
                        texture.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE.linear_multiply(unchecked_opacity),
                    );
                }
            }
            if checked_opacity > 0.01 {
                if let Some(texture) =
                    checkbox_icon_texture(ui, CHECKBOX_CHECKED_SVG, "checked", icon_color)
                {
                    ui.painter().image(
                        texture.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE.linear_multiply(checked_opacity),
                    );
                }
            }
        }

        if let Some(galley) = text_galley {
            let text_x = icon_rect.max.x + label_gap;
            let text_y = center_galley_y(&galley, rect);
            let text_color = if self.enabled {
                on_surface
            } else {
                disabled_color
            };
            ui.painter()
                .galley(Pos2::new(text_x, text_y), galley, text_color);
        }

        response
    }
}

pub fn checkbox(checked: &mut bool, text: impl Into<String>) -> MaterialCheckbox<'_> {
    MaterialCheckbox::new(checked, text)
}
