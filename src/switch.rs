use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use crate::get_global_color;

pub struct MaterialSwitch<'a> {
    selected: &'a mut bool,
    text: Option<String>,
    enabled: bool,
}

impl<'a> MaterialSwitch<'a> {
    pub fn new(selected: &'a mut bool) -> Self {
        Self {
            selected,
            text: None,
            enabled: true,
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
}

impl<'a> Widget for MaterialSwitch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let switch_width = 52.0;
        let switch_height = 32.0;
        
        let desired_size = if let Some(ref text) = self.text {
            let text_width = ui.fonts(|fonts| {
                fonts.glyph_width(&egui::FontId::default(), ' ') * text.len() as f32
            });
            Vec2::new(switch_width + 8.0 + text_width, switch_height)
        } else {
            Vec2::new(switch_width, switch_height)
        };

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() && self.enabled {
            *self.selected = !*self.selected;
            response.mark_changed();
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
        let on_primary = get_global_color("onPrimary");
        let surface_variant = get_global_color("surfaceVariant");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        // Calculate switch area
        let switch_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - switch_height / 2.0),
            Vec2::new(switch_width, switch_height),
        );

        let track_height = 14.0;
        let track_rect = Rect::from_center_size(
            switch_rect.center(),
            Vec2::new(switch_width, track_height),
        );

        let thumb_size = 24.0;
        let thumb_travel = switch_width - thumb_size - 4.0;
        let thumb_x = if *self.selected {
            switch_rect.min.x + 2.0 + thumb_travel
        } else {
            switch_rect.min.x + 2.0
        };
        
        let thumb_center = Pos2::new(thumb_x + thumb_size / 2.0, switch_rect.center().y);

        // Determine colors based on state
        let (track_color, thumb_color, thumb_outline) = if !self.enabled {
            let disabled_track = get_global_color("surfaceVariant").linear_multiply(0.38);
            let disabled_thumb = get_global_color("onSurface").linear_multiply(0.38);
            (disabled_track, disabled_thumb, Color32::TRANSPARENT)
        } else if *self.selected {
            if response.hovered() {
                (
                    Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 200),
                    Color32::from_rgba_premultiplied(on_primary.r().saturating_add(20), on_primary.g().saturating_add(20), on_primary.b().saturating_add(20), 255),
                    Color32::TRANSPARENT,
                )
            } else {
                (primary_color, on_primary, Color32::TRANSPARENT)
            }
        } else {
            if response.hovered() {
                (
                    Color32::from_rgba_premultiplied(surface_variant.r(), surface_variant.g(), surface_variant.b(), 200),
                    Color32::from_rgba_premultiplied(on_surface_variant.r(), on_surface_variant.g(), on_surface_variant.b(), 200),
                    outline,
                )
            } else {
                (surface_variant, on_surface_variant, outline)
            }
        };

        // Draw track
        ui.painter().rect_filled(
            track_rect,
            track_height / 2.0,
            track_color,
        );

        // Draw thumb
        ui.painter().circle_filled(
            thumb_center,
            thumb_size / 2.0,
            thumb_color,
        );

        // Draw thumb outline if needed
        if thumb_outline != Color32::TRANSPARENT {
            ui.painter().circle_stroke(
                thumb_center,
                thumb_size / 2.0,
                Stroke::new(2.0, thumb_outline),
            );
        }

        // Add ripple effect on hover
        if response.hovered() && self.enabled {
            let ripple_color = if *self.selected {
                Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 30)
            } else {
                Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 30)
            };
            
            ui.painter().circle_filled(
                thumb_center,
                thumb_size / 2.0 + 12.0,
                ripple_color,
            );
        }

        // Draw label text
        if let Some(ref text) = self.text {
            let text_pos = Pos2::new(
                switch_rect.max.x + 8.0,
                rect.center().y,
            );
            
            let text_color = if self.enabled { on_surface } else {
                get_global_color("onSurface").linear_multiply(0.38)
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::default(),
                text_color,
            );
        }

        response
    }
}

pub fn switch(selected: &mut bool) -> MaterialSwitch {
    MaterialSwitch::new(selected)
}