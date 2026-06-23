//! Nala / Leo toggle (switch) component
//!
//! Implements the Leo `toggle.svelte` visual model: pill track, circular thumb,
//! primary when checked, outline-variant when unchecked.

use crate::{
    get_global_color,
    theme::{get_global_theme, ThemeMode},
};
use egui::{
    self, epaint::Galley, Align2, Color32, FontId, Pos2, Rect, Response, Sense, Ui, Vec2, Widget,
};
use std::sync::Arc;

/// Nala toggle size (Leo component API)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MaterialSwitchSize {
    Small,
    #[default]
    Medium,
}

pub type SwitchSize = MaterialSwitchSize;

struct NalaToggleLayout {
    outer_width: f32,
    outer_height: f32,
    padding: f32,
    icon_size: f32,
}

impl MaterialSwitchSize {
    fn layout(self) -> NalaToggleLayout {
        match self {
            MaterialSwitchSize::Small => NalaToggleLayout {
                outer_width: 40.0,
                outer_height: 24.0,
                padding: 4.0,
                icon_size: 12.0,
            },
            MaterialSwitchSize::Medium => NalaToggleLayout {
                outer_width: 52.0,
                outer_height: 32.0,
                padding: 4.0,
                icon_size: 20.0,
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

/// Leo `color-mix(in srgb, base 80%, foreground 20%)` for hover backgrounds.
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

/// Nala toggle switch (Leo `Toggle` component)
pub struct MaterialSwitch<'a> {
    selected: &'a mut bool,
    text: Option<String>,
    enabled: bool,
    size: MaterialSwitchSize,
    /// Icon shown on the thumb when checked (Leo `on-icon` slot)
    selected_icon: Option<char>,
    /// Legacy M3: icon when unchecked (ignored in default Nala styling)
    unselected_icon: Option<char>,
    /// Legacy M3 track outline (off by default — Nala toggles have no outline)
    show_track_outline: bool,
}

impl<'a> MaterialSwitch<'a> {
    pub fn new(selected: &'a mut bool) -> Self {
        Self {
            selected,
            text: None,
            enabled: true,
            size: MaterialSwitchSize::Medium,
            selected_icon: None,
            unselected_icon: None,
            show_track_outline: false,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn size(mut self, size: MaterialSwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn selected_icon(mut self, icon: char) -> Self {
        self.selected_icon = Some(icon);
        self
    }

    pub fn unselected_icon(mut self, icon: char) -> Self {
        self.unselected_icon = Some(icon);
        self
    }

    pub fn with_icons(mut self, selected: char, unselected: char) -> Self {
        self.selected_icon = Some(selected);
        self.unselected_icon = Some(unselected);
        self
    }

    pub fn show_track_outline(mut self, show: bool) -> Self {
        self.show_track_outline = show;
        self
    }
}

impl<'a> Widget for MaterialSwitch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let layout = self.size.layout();
        let label_gap = 4.0;
        let dark = is_dark_theme();

        let primary = get_global_color("primary");
        let on_primary = get_global_color("onPrimary");
        let outline_variant = get_global_color("outlineVariant");
        let on_surface = get_global_color("onSurface");
        let outline = get_global_color("outline");
        let surface_container_highest = get_global_color("surfaceContainerHighest");

        let text_galley: Option<Arc<Galley>> = self.text.as_ref().map(|text| {
            ui.painter().layout(
                text.clone(),
                FontId::default(),
                on_surface,
                f32::INFINITY,
            )
        });

        let text_width = text_galley
            .as_ref()
            .map(|g| g.size().x)
            .unwrap_or(0.0);
        let desired_size = if text_galley.is_some() {
            Vec2::new(
                layout.outer_width + label_gap + text_width,
                layout.outer_height.max(
                    text_galley
                        .as_ref()
                        .map(|g| g.mesh_bounds.height())
                        .unwrap_or(0.0),
                ),
            )
        } else {
            Vec2::new(layout.outer_width, layout.outer_height)
        };

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() && self.enabled {
            *self.selected = !*self.selected;
            response.mark_changed();
        }

        let is_hovered = response.hovered() && self.enabled;
        let is_focused = response.has_focus() && self.enabled;

        let anim_t = ui
            .ctx()
            .animate_bool(response.id.with("nala_toggle"), *self.selected);

        let switch_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - layout.outer_height / 2.0),
            Vec2::new(layout.outer_width, layout.outer_height),
        );

        let track_radius = layout.outer_height / 2.0;
        let thumb_diameter = layout.outer_height - 2.0 * layout.padding;
        let thumb_travel = layout.outer_width - layout.outer_height;
        let thumb_x = switch_rect.min.x
            + layout.padding
            + thumb_travel * anim_t;
        let thumb_center = Pos2::new(
            thumb_x + thumb_diameter / 2.0,
            switch_rect.center().y,
        );

        let checked_track = primary;
        let unchecked_track = outline_variant;

        let mut track_color = Color32::from_rgba_unmultiplied(
            ((checked_track.r() as f32 * anim_t) + (unchecked_track.r() as f32 * (1.0 - anim_t)))
                as u8,
            ((checked_track.g() as f32 * anim_t) + (unchecked_track.g() as f32 * (1.0 - anim_t)))
                as u8,
            ((checked_track.b() as f32 * anim_t) + (unchecked_track.b() as f32 * (1.0 - anim_t)))
                as u8,
            255,
        );

        if is_hovered {
            track_color = color_mix_srgb(track_color, foreground_color(dark), 0.2);
        }

        if !self.enabled {
            track_color = with_opacity(track_color, 0.5);
        }

        let thumb_color = if !self.enabled {
            Color32::WHITE.linear_multiply(0.5)
        } else if anim_t > 0.5 {
            on_primary
        } else {
            Color32::WHITE
        };

        ui.painter()
            .rect_filled(switch_rect, track_radius, track_color);

        // Legacy M3 track outline
        if self.show_track_outline && anim_t < 0.5 {
            ui.painter().rect_stroke(
                switch_rect,
                track_radius,
                egui::Stroke::new(2.0, outline),
                egui::epaint::StrokeKind::Outside,
            );
        }

        if is_focused {
            let focus_expand = 2.0;
            ui.painter().rect_stroke(
                switch_rect.expand(focus_expand),
                track_radius + focus_expand,
                egui::Stroke::new(2.0, primary.linear_multiply(0.5)),
                egui::epaint::StrokeKind::Outside,
            );
        }

        ui.painter()
            .circle_filled(thumb_center, thumb_diameter / 2.0, thumb_color);

        if *self.selected {
            if let Some(icon) = self.selected_icon {
                let icon_font = FontId::proportional(layout.icon_size);
                let icon_color = if self.enabled {
                    primary
                } else {
                    with_opacity(primary, 0.5)
                };
                ui.painter().text(
                    thumb_center,
                    Align2::CENTER_CENTER,
                    icon.to_string(),
                    icon_font,
                    icon_color,
                );
            }
        } else if let Some(icon) = self.unselected_icon {
            let icon_font = FontId::proportional(layout.icon_size);
            ui.painter().text(
                thumb_center,
                Align2::CENTER_CENTER,
                icon.to_string(),
                icon_font,
                surface_container_highest,
            );
        }

        if let Some(galley) = text_galley {
            let text_x = switch_rect.max.x + label_gap;
            let text_y = rect.center().y - galley.mesh_bounds.center().y;
            let text_color = if self.enabled {
                on_surface
            } else {
                on_surface.linear_multiply(0.5)
            };
            ui.painter().galley(Pos2::new(text_x, text_y), galley, text_color);
        }

        response
    }
}

pub fn switch(selected: &mut bool) -> MaterialSwitch<'_> {
    MaterialSwitch::new(selected)
}
