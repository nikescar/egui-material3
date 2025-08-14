use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};
use crate::get_global_color;
use std::ops::RangeInclusive;

pub struct MaterialSlider<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    text: Option<String>,
    enabled: bool,
    width: Option<f32>,
    step: Option<f32>,
    show_value: bool,
}

impl<'a> MaterialSlider<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            text: None,
            enabled: true,
            width: None,
            step: None,
            show_value: true,
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

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }
}

impl<'a> Widget for MaterialSlider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let slider_width = self.width.unwrap_or(200.0);
        let height = 48.0;
        
        let desired_size = if self.text.is_some() || self.show_value {
            Vec2::new(slider_width + 100.0, height)
        } else {
            Vec2::new(slider_width, height)
        };

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Material Design colors
        let primary_color = get_global_color("primary");
        let on_primary = Color32::WHITE;
        let surface_variant = Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 231 });
        let on_surface = Color32::from_gray(if ui.visuals().dark_mode { 230 } else { 30 });
        let on_surface_variant = Color32::from_gray(if ui.visuals().dark_mode { 202 } else { 73 });

        // Calculate slider track area
        let track_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - 2.0),
            Vec2::new(slider_width, 4.0),
        );

        // Handle interaction
        if response.dragged() && self.enabled {
            let mouse_pos = response.interact_pointer_pos().unwrap_or(track_rect.center());
            let normalized = ((mouse_pos.x - track_rect.min.x) / track_rect.width()).clamp(0.0, 1.0);
            let mut new_value = *self.range.start() + normalized * (self.range.end() - self.range.start());
            
            // Apply step if specified
            if let Some(step) = self.step {
                new_value = (new_value / step).round() * step;
            }
            
            *self.value = new_value.clamp(*self.range.start(), *self.range.end());
            response.mark_changed();
        }

        if !self.enabled {
            response = response.on_disabled_hover_text("Slider is disabled");
        }

        // Calculate positions
        let normalized_value = (*self.value - self.range.start()) / (self.range.end() - self.range.start());
        let normalized_value = normalized_value.clamp(0.0, 1.0);
        let thumb_x = track_rect.min.x + normalized_value * track_rect.width();
        let thumb_center = Pos2::new(thumb_x, track_rect.center().y);

        // Determine colors based on state
        let (track_active_color, track_inactive_color, thumb_color) = if !self.enabled {
            let disabled_color = Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 189 });
            (disabled_color, disabled_color, disabled_color)
        } else if response.hovered() || response.dragged() {
            (
                Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 200),
                surface_variant,
                Color32::from_rgba_premultiplied(primary_color.r().saturating_add(20), primary_color.g().saturating_add(20), primary_color.b().saturating_add(20), 255),
            )
        } else {
            (primary_color, surface_variant, primary_color)
        };

        // Draw inactive track
        ui.painter().rect_filled(
            track_rect,
            2.0,
            track_inactive_color,
        );

        // Draw active track (from start to thumb)
        let active_track_rect = Rect::from_min_size(
            track_rect.min,
            Vec2::new(thumb_x - track_rect.min.x, track_rect.height()),
        );
        
        if active_track_rect.width() > 0.0 {
            ui.painter().rect_filled(
                active_track_rect,
                2.0,
                track_active_color,
            );
        }

        // Draw thumb
        let thumb_radius = if response.hovered() || response.dragged() { 12.0 } else { 10.0 };
        ui.painter().circle_filled(
            thumb_center,
            thumb_radius,
            thumb_color,
        );

        // Add ripple effect when interacting
        if response.hovered() && self.enabled {
            let ripple_color = Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 30);
            ui.painter().circle_filled(
                thumb_center,
                thumb_radius + 16.0,
                ripple_color,
            );
        }

        // Draw label text
        if let Some(ref text) = self.text {
            let text_pos = Pos2::new(track_rect.max.x + 16.0, rect.center().y - 16.0);
            let text_color = if self.enabled { on_surface } else {
                Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 189 })
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::default(),
                text_color,
            );
        }

        // Draw value
        if self.show_value {
            let value_text = if let Some(step) = self.step {
                if step >= 1.0 {
                    format!("{:.0}", *self.value)
                } else {
                    format!("{:.2}", *self.value)
                }
            } else {
                format!("{:.2}", *self.value)
            };
            
            let value_pos = Pos2::new(
                track_rect.max.x + 16.0, 
                rect.center().y + if self.text.is_some() { 8.0 } else { 0.0 }
            );
            
            let value_color = if self.enabled { on_surface_variant } else {
                Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 189 })
            };

            ui.painter().text(
                value_pos,
                egui::Align2::LEFT_CENTER,
                &value_text,
                egui::FontId::proportional(12.0),
                value_color,
            );
        }

        response
    }
}

pub fn slider<'a>(value: &'a mut f32, range: RangeInclusive<f32>) -> MaterialSlider<'a> {
    MaterialSlider::new(value, range)
}