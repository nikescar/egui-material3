use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget, TextureHandle};
use crate::{get_global_color, image_utils};

#[derive(Clone, Copy, PartialEq)]
pub enum ChipVariant {
    Assist,
    Filter,
    Input,
    Suggestion,
}

#[derive(Clone)]
pub enum IconType {
    MaterialIcon(String),  // Material icon name/unicode
    SvgData(String),      // SVG content
    PngBytes(Vec<u8>),    // PNG image data
    Texture(TextureHandle), // Pre-loaded texture
}

pub struct MaterialChip<'a> {
    text: String,
    variant: ChipVariant,
    selected: Option<&'a mut bool>,
    enabled: bool,
    elevated: bool,
    removable: bool,
    leading_icon: Option<IconType>,
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialChip<'a> {
    pub fn new(text: impl Into<String>, variant: ChipVariant) -> Self {
        Self {
            text: text.into(),
            variant,
            selected: None,
            enabled: true,
            elevated: false,
            removable: false,
            leading_icon: None,
            action: None,
        }
    }

    pub fn assist(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Assist)
    }

    pub fn filter(text: impl Into<String>, selected: &'a mut bool) -> Self {
        let mut chip = Self::new(text, ChipVariant::Filter);
        chip.selected = Some(selected);
        chip
    }

    pub fn input(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Input)
    }

    pub fn suggestion(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Suggestion)
    }

    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(IconType::MaterialIcon(icon.into()));
        self
    }

    pub fn leading_icon_svg(mut self, svg_data: impl Into<String>) -> Self {
        self.leading_icon = Some(IconType::SvgData(svg_data.into()));
        self
    }

    pub fn leading_icon_png(mut self, png_bytes: Vec<u8>) -> Self {
        self.leading_icon = Some(IconType::PngBytes(png_bytes));
        self
    }

    pub fn leading_icon_texture(mut self, texture: TextureHandle) -> Self {
        self.leading_icon = Some(IconType::Texture(texture));
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self 
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

impl<'a> Widget for MaterialChip<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let text_width = ui.fonts(|fonts| {
            fonts.layout_no_wrap(
                self.text.clone(),
                egui::FontId::default(),
                egui::Color32::WHITE
            ).rect.width()
        });
        
        let icon_width = if self.leading_icon.is_some() || (self.variant == ChipVariant::Filter && self.selected.as_ref().map_or(false, |s| **s)) {
            24.0
        } else {
            0.0
        };
        
        let remove_width = if self.removable { 24.0 } else { 0.0 };
        let padding = 16.0;
        
        let desired_size = Vec2::new(
            (text_width + icon_width + remove_width + padding).min(ui.available_width()),
            32.0,
        );

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface = get_global_color("surface");
        let surface_variant = get_global_color("surfaceVariant");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let (bg_color, border_color, text_color) = match self.variant {
            ChipVariant::Assist => {
                if !self.enabled {
                    (
                        get_global_color("surfaceContainer"),
                        get_global_color("outline"),
                        get_global_color("outline"),
                    )
                } else if self.elevated {
                    (surface, Color32::TRANSPARENT, on_surface)
                } else if response.hovered() {
                    (surface_variant, outline, on_surface_variant)
                } else {
                    (Color32::TRANSPARENT, outline, on_surface_variant)
                }
            }
            ChipVariant::Filter => {
                let is_selected = self.selected.as_ref().map_or(false, |s| **s);
                if !self.enabled {
                    (
                        get_global_color("surfaceContainer"),
                        get_global_color("outline"),
                        get_global_color("outline"),
                    )
                } else if is_selected {
                    (
                        Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 24),
                        primary_color,
                        on_surface,
                    )
                } else if self.elevated {
                    (surface, Color32::TRANSPARENT, on_surface_variant)
                } else if response.hovered() {
                    (surface_variant, outline, on_surface_variant)
                } else {
                    (Color32::TRANSPARENT, outline, on_surface_variant)
                }
            }
            ChipVariant::Input => {
                if !self.enabled {
                    (
                        get_global_color("surfaceContainer"),
                        get_global_color("outline"),
                        get_global_color("outline"),
                    )
                } else if response.hovered() {
                    (surface_variant, outline, on_surface_variant)
                } else {
                    (Color32::TRANSPARENT, outline, on_surface_variant)
                }
            }
            ChipVariant::Suggestion => {
                if !self.enabled {
                    (
                        get_global_color("surfaceContainer"),
                        get_global_color("outline"),
                        get_global_color("outline"),
                    )
                } else if self.elevated {
                    (surface, Color32::TRANSPARENT, on_surface_variant)
                } else if response.hovered() {
                    (surface_variant, outline, on_surface_variant)
                } else {
                    (Color32::TRANSPARENT, outline, on_surface_variant)
                }
            }
        };

        // Draw chip background
        ui.painter().rect_filled(
            rect,
            16.0,
            bg_color,
        );

        // Draw chip border (if not transparent)
        if border_color != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                16.0,
                Stroke::new(1.0, border_color),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Draw elevation shadow for elevated chips
        if self.elevated {
            let shadow_rect = rect.expand(2.0);
            ui.painter().rect_filled(
                shadow_rect,
                16.0,
                Color32::from_black_alpha(20),
            );
        }

        // Layout content
        let mut content_x = rect.min.x + 8.0;
        
        // Draw leading icon or checkmark
        if let Some(icon) = &self.leading_icon {
            let icon_rect = Rect::from_min_size(
                Pos2::new(content_x, rect.center().y - 10.0),
                Vec2::splat(20.0),
            );
            
            match icon {
                IconType::MaterialIcon(icon_str) => {
                    // Draw material icon as text
                    ui.painter().text(
                        icon_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        icon_str,
                        egui::FontId::proportional(16.0),
                        text_color,
                    );
                }
                IconType::SvgData(svg_data) => {
                    // Convert SVG to texture and draw
                    if let Ok(texture) = image_utils::create_texture_from_svg(ui.ctx(), svg_data, &format!("chip_svg_{}", svg_data.len())) {
                        ui.painter().image(
                            texture.id(),
                            icon_rect,
                            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                            Color32::WHITE,
                        );
                    }
                }
                IconType::PngBytes(png_bytes) => {
                    // Convert PNG bytes to texture and draw
                    if let Ok(texture) = image_utils::create_texture_from_png_bytes(ui.ctx(), png_bytes, &format!("chip_png_{}", png_bytes.len())) {
                        ui.painter().image(
                            texture.id(),
                            icon_rect,
                            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                            Color32::WHITE,
                        );
                    }
                }
                IconType::Texture(texture) => {
                    // Draw pre-loaded texture
                    ui.painter().image(
                        texture.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
            }
            content_x += 24.0;
        } else if self.variant == ChipVariant::Filter && self.selected.as_ref().map_or(false, |s| **s) {
            // Draw checkmark for selected filter chips
            let icon_rect = Rect::from_min_size(
                Pos2::new(content_x, rect.center().y - 10.0),
                Vec2::splat(20.0),
            );
            
            let center = icon_rect.center();
            let checkmark_size = 12.0;
            
            let start = Pos2::new(
                center.x - checkmark_size * 0.3,
                center.y,
            );
            let middle = Pos2::new(
                center.x - checkmark_size * 0.1,
                center.y + checkmark_size * 0.2,
            );
            let end = Pos2::new(
                center.x + checkmark_size * 0.3,
                center.y - checkmark_size * 0.2,
            );

            ui.painter().line_segment([start, middle], Stroke::new(2.0, text_color));
            ui.painter().line_segment([middle, end], Stroke::new(2.0, text_color));
            content_x += 24.0;
        }

        // Draw text
        let text_pos = Pos2::new(content_x, rect.center().y);
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            &self.text,
            egui::FontId::default(),
            text_color,
        );

        // Draw remove button for removable chips
        if self.removable {
            let remove_rect = Rect::from_min_size(
                Pos2::new(rect.max.x - 24.0, rect.center().y - 10.0),
                Vec2::splat(20.0),
            );
            
            // Draw X
            let center = remove_rect.center();
            let cross_size = 8.0;
            ui.painter().line_segment([
                Pos2::new(center.x - cross_size / 2.0, center.y - cross_size / 2.0),
                Pos2::new(center.x + cross_size / 2.0, center.y + cross_size / 2.0),
            ], Stroke::new(1.5, text_color));
            ui.painter().line_segment([
                Pos2::new(center.x + cross_size / 2.0, center.y - cross_size / 2.0),
                Pos2::new(center.x - cross_size / 2.0, center.y + cross_size / 2.0),
            ], Stroke::new(1.5, text_color));
        }

        // Handle interactions
        if response.clicked() && self.enabled {
            match self.variant {
                ChipVariant::Filter => {
                    if let Some(selected) = self.selected {
                        *selected = !*selected;
                        response.mark_changed();
                    }
                }
                _ => {
                    if let Some(action) = self.action {
                        action();
                    }
                }
            }
        }

        // Add ripple effect on hover
        if response.hovered() && self.enabled {
            let ripple_color = Color32::from_rgba_premultiplied(text_color.r(), text_color.g(), text_color.b(), 20);
            ui.painter().rect_filled(
                rect,
                16.0,
                ripple_color,
            );
        }

        response
    }
}

pub fn assist_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::assist(text)
}

pub fn filter_chip(text: impl Into<String>, selected: &mut bool) -> MaterialChip {
    MaterialChip::filter(text, selected)
}

pub fn input_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::input(text)
}

pub fn suggestion_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::suggestion(text)
}