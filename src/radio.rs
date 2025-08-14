use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use crate::get_global_color;

pub struct MaterialRadio<'a> {
    selected: &'a mut Option<usize>,
    value: usize,
    text: String,
    enabled: bool,
}

pub struct MaterialRadioGroup<'a> {
    selected: &'a mut Option<usize>,
    options: Vec<RadioOption>,
    enabled: bool,
}

pub struct RadioOption {
    text: String,
    value: usize,
}

impl<'a> MaterialRadio<'a> {
    pub fn new(selected: &'a mut Option<usize>, value: usize, text: impl Into<String>) -> Self {
        Self {
            selected,
            value,
            text: text.into(),
            enabled: true,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for MaterialRadio<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(
            ui.available_width().min(300.0),
            24.0,
        );

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        let is_selected = self.selected.map_or(false, |s| s == self.value);

        if response.clicked() && self.enabled {
            *self.selected = Some(self.value);
            response.mark_changed();
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let radio_size = 20.0;
        let radio_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - radio_size / 2.0),
            Vec2::splat(radio_size),
        );

        let (border_color, fill_color, inner_color) = if !self.enabled {
            let disabled_color = get_global_color("onSurfaceVariant").linear_multiply(0.38);
            (disabled_color, Color32::TRANSPARENT, disabled_color)
        } else if is_selected {
            (primary_color, Color32::TRANSPARENT, primary_color)
        } else if response.hovered() {
            (outline, Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20), on_surface_variant)
        } else {
            (outline, Color32::TRANSPARENT, on_surface_variant)
        };

        // Draw hover background
        if fill_color != Color32::TRANSPARENT {
            ui.painter().circle_filled(radio_rect.center(), radio_size / 2.0 + 8.0, fill_color);
        }

        // Draw radio border
        ui.painter().circle_stroke(
            radio_rect.center(),
            radio_size / 2.0,
            Stroke::new(2.0, border_color),
        );

        // Draw selected inner circle
        if is_selected {
            ui.painter().circle_filled(
                radio_rect.center(),
                radio_size / 4.0,
                inner_color,
            );
        }

        // Draw label text
        if !self.text.is_empty() {
            let text_pos = Pos2::new(
                radio_rect.max.x + 8.0,
                rect.center().y,
            );
            
            let text_color = if self.enabled { on_surface } else {
                get_global_color("onSurfaceVariant").linear_multiply(0.38)
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                &self.text,
                egui::FontId::default(),
                text_color,
            );
        }

        // Add ripple effect on hover
        if response.hovered() && self.enabled {
            let ripple_color = if is_selected {
                Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 20)
            } else {
                Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20)
            };
            
            ui.painter().circle_filled(
                radio_rect.center(),
                radio_size / 2.0 + 12.0,
                ripple_color,
            );
        }

        response
    }
}

impl<'a> MaterialRadioGroup<'a> {
    pub fn new(selected: &'a mut Option<usize>) -> Self {
        Self {
            selected,
            options: Vec::new(),
            enabled: true,
        }
    }

    pub fn option(mut self, value: usize, text: impl Into<String>) -> Self {
        self.options.push(RadioOption {
            text: text.into(),
            value,
        });
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for MaterialRadioGroup<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut group_response = None;
        
        ui.vertical(|ui| {
            for option in self.options {
                let radio = MaterialRadio::new(self.selected, option.value, option.text)
                    .enabled(self.enabled);
                
                let response = ui.add(radio);
                
                if group_response.is_none() {
                    group_response = Some(response);
                } else if let Some(ref mut group_resp) = group_response {
                    *group_resp = group_resp.union(response);
                }
            }
        });

        group_response.unwrap_or_else(|| {
            let (rect, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::hover());
            response
        })
    }
}

pub fn radio<'a>(selected: &'a mut Option<usize>, value: usize, text: impl Into<String>) -> MaterialRadio<'a> {
    MaterialRadio::new(selected, value, text)
}

pub fn radio_group<'a>(selected: &'a mut Option<usize>) -> MaterialRadioGroup<'a> {
    MaterialRadioGroup::new(selected)
}