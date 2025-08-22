use crate::theme::get_global_color;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use crate::icons::icon_text;

pub struct MaterialList<'a> {
    items: Vec<ListItem<'a>>,
    dividers: bool,
}

pub struct ListItem<'a> {
    primary_text: String,
    secondary_text: Option<String>,
    overline_text: Option<String>,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    trailing_text: Option<String>,
    enabled: bool,
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialList<'a> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            dividers: true,
        }
    }

    pub fn item(mut self, item: ListItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    pub fn dividers(mut self, dividers: bool) -> Self {
        self.dividers = dividers;
        self
    }
}

impl<'a> ListItem<'a> {
    pub fn new(primary_text: impl Into<String>) -> Self {
        Self {
            primary_text: primary_text.into(),
            secondary_text: None,
            overline_text: None,
            leading_icon: None,
            trailing_icon: None,
            trailing_text: None,
            enabled: true,
            action: None,
        }
    }

    pub fn secondary_text(mut self, text: impl Into<String>) -> Self {
        self.secondary_text = Some(text.into());
        self
    }

    pub fn overline(mut self, text: impl Into<String>) -> Self {
        self.overline_text = Some(text.into());
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

    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
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

impl<'a> Widget for MaterialList<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut total_height = 0.0;
        let item_height = if self.items.iter().any(|item| item.secondary_text.is_some() || item.overline_text.is_some()) {
            if self.items.iter().any(|item| item.overline_text.is_some() && item.secondary_text.is_some()) {
                88.0 // Three-line item height (overline + primary + secondary)
            } else {
                72.0 // Two-line item height
            }
        } else {
            56.0 // Single-line item height
        };

        // Calculate dynamic width based on content
        let mut max_content_width = 200.0; // minimum width
        for item in &self.items {
            let mut item_width = 32.0; // base padding
            
            // Add leading icon width
            if item.leading_icon.is_some() {
                item_width += 40.0;
            }
            
            // Add text width (approximate)
            let primary_text_width = item.primary_text.len() as f32 * 8.0; // rough estimate
            let secondary_text_width = item.secondary_text.as_ref()
                .map_or(0.0, |s| s.len() as f32 * 6.0); // smaller font
            let overline_text_width = item.overline_text.as_ref()
                .map_or(0.0, |s| s.len() as f32 * 5.5); // smallest font
            let max_text_width = primary_text_width.max(secondary_text_width).max(overline_text_width);
            item_width += max_text_width;
            
            // Add trailing text width
            if let Some(ref trailing_text) = item.trailing_text {
                item_width += trailing_text.len() as f32 * 6.0;
            }
            
            // Add trailing icon width
            if item.trailing_icon.is_some() {
                item_width += 40.0;
            }
            
            // Add some padding
            item_width += 32.0;
            
            if item_width > max_content_width {
                max_content_width = item_width;
            }
        }
        
        // Cap the width to available width but allow it to be smaller
        let list_width = max_content_width.min(ui.available_width());

        total_height += item_height * self.items.len() as f32;
        if self.dividers && self.items.len() > 1 {
            total_height += (self.items.len() - 1) as f32; // 1px dividers
        }

        let desired_size = Vec2::new(list_width, total_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Material Design colors
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline_variant = get_global_color("outlineVariant");

        // Draw list background with rounded rectangle border
        ui.painter().rect_filled(rect, 8.0, surface);
        ui.painter().rect_stroke(rect, 8.0, Stroke::new(1.0, outline_variant), egui::epaint::StrokeKind::Outside);

        let mut current_y = rect.min.y;
        let mut pending_actions = Vec::new();

        let items_len = self.items.len();
        for (index, item) in self.items.into_iter().enumerate() {
            let item_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, current_y),
                Vec2::new(rect.width(), item_height),
            );

            // Use unique ID combining index and text content to prevent clashes
            let unique_id = egui::Id::new(("list_item", index, item.primary_text.clone()));
            let item_response = ui.interact(item_rect, unique_id, Sense::click());

            // Draw item background on hover
            if item_response.hovered() && item.enabled {
                let hover_color = Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20);
                ui.painter().rect_filled(item_rect, 0.0, hover_color);
            }

            // Handle click
            if item_response.clicked() && item.enabled {
                if let Some(action) = item.action {
                    pending_actions.push(action);
                }
            }

            // Layout item content
            let mut content_x = item_rect.min.x + 16.0;
            let content_y = item_rect.center().y;

            // Draw leading icon
            if let Some(icon_name) = &item.leading_icon {
                let icon_pos = Pos2::new(content_x + 12.0, content_y);
                
                let icon_color = if item.enabled { on_surface_variant } else {
                    get_global_color("onSurfaceVariant").linear_multiply(0.38)
                };

                let icon_string = icon_text(icon_name);
                ui.painter().text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    &icon_string,
                    egui::FontId::proportional(20.0),
                    icon_color,
                );
                content_x += 40.0;
            }

            // Calculate text area with trailing text support
            let trailing_icon_width = if item.trailing_icon.is_some() { 40.0 } else { 0.0 };
            let trailing_text_width = if item.trailing_text.is_some() { 80.0 } else { 0.0 }; // Estimate
            let total_trailing_width = trailing_icon_width + trailing_text_width;
            let _text_width = item_rect.max.x - content_x - total_trailing_width - 16.0;

            // Draw text content
            let text_color = if item.enabled { on_surface } else {
                get_global_color("onSurfaceVariant").linear_multiply(0.38)
            };

            // Layout text based on what's present
            match (&item.overline_text, &item.secondary_text) {
                (Some(overline), Some(secondary)) => {
                    // Three-line layout: overline + primary + secondary
                    let overline_pos = Pos2::new(content_x, content_y - 20.0);
                    let primary_pos = Pos2::new(content_x, content_y);
                    let secondary_pos = Pos2::new(content_x, content_y + 20.0);

                    ui.painter().text(
                        overline_pos,
                        egui::Align2::LEFT_CENTER,
                        overline,
                        egui::FontId::proportional(11.0),
                        on_surface_variant,
                    );

                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::default(),
                        text_color,
                    );

                    ui.painter().text(
                        secondary_pos,
                        egui::Align2::LEFT_CENTER,
                        secondary,
                        egui::FontId::proportional(12.0),
                        on_surface_variant,
                    );
                },
                (Some(overline), None) => {
                    // Two-line layout: overline + primary
                    let overline_pos = Pos2::new(content_x, content_y - 10.0);
                    let primary_pos = Pos2::new(content_x, content_y + 10.0);

                    ui.painter().text(
                        overline_pos,
                        egui::Align2::LEFT_CENTER,
                        overline,
                        egui::FontId::proportional(11.0),
                        on_surface_variant,
                    );

                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::default(),
                        text_color,
                    );
                },
                (None, Some(secondary)) => {
                    // Two-line layout: primary + secondary
                    let primary_pos = Pos2::new(content_x, content_y - 10.0);
                    let secondary_pos = Pos2::new(content_x, content_y + 10.0);

                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::default(),
                        text_color,
                    );

                    ui.painter().text(
                        secondary_pos,
                        egui::Align2::LEFT_CENTER,
                        secondary,
                        egui::FontId::proportional(12.0),
                        on_surface_variant,
                    );
                },
                (None, None) => {
                    // Single-line layout: primary only
                    let text_pos = Pos2::new(content_x, content_y);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::default(),
                        text_color,
                    );
                }
            }

            // Draw trailing supporting text
            if let Some(ref trailing_text) = item.trailing_text {
                let trailing_text_pos = Pos2::new(
                    item_rect.max.x - trailing_icon_width - trailing_text_width + 10.0,
                    content_y
                );
                
                ui.painter().text(
                    trailing_text_pos,
                    egui::Align2::LEFT_CENTER,
                    trailing_text,
                    egui::FontId::proportional(12.0),
                    on_surface_variant,
                );
            }

            // Draw trailing icon
            if let Some(icon_name) = &item.trailing_icon {
                let icon_pos = Pos2::new(item_rect.max.x - 28.0, content_y);
                
                let icon_color = if item.enabled { on_surface_variant } else {
                    get_global_color("onSurfaceVariant").linear_multiply(0.38)
                };

                let icon_string = icon_text(icon_name);
                ui.painter().text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    &icon_string,
                    egui::FontId::proportional(20.0),
                    icon_color,
                );
            }

            current_y += item_height;

            // Draw divider
            if self.dividers && index < items_len - 1 {
                let divider_y = current_y;
                let divider_start = Pos2::new(rect.min.x + 16.0, divider_y);
                let divider_end = Pos2::new(rect.max.x - 16.0, divider_y);
                
                ui.painter().line_segment(
                    [divider_start, divider_end],
                    Stroke::new(1.0, outline_variant),
                );
                current_y += 1.0;
            }
        }

        // Execute pending actions
        for action in pending_actions {
            action();
        }

        response
    }
}

pub fn list_item(primary_text: impl Into<String>) -> ListItem<'static> {
    ListItem::new(primary_text)
}

pub fn list() -> MaterialList<'static> {
    MaterialList::new()
}