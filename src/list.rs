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
        let item_height = if self.items.iter().any(|item| item.secondary_text.is_some()) {
            72.0 // Two-line item height
        } else {
            56.0 // Single-line item height
        };

        total_height += item_height * self.items.len() as f32;
        if self.dividers && self.items.len() > 1 {
            total_height += (self.items.len() - 1) as f32; // 1px dividers
        }

        let desired_size = Vec2::new(ui.available_width(), total_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Material Design colors
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline_variant = get_global_color("outlineVariant");

        // Draw list background
        ui.painter().rect_filled(rect, 8.0, surface);

        let mut current_y = rect.min.y;
        let mut pending_actions = Vec::new();

        let items_len = self.items.len();
        for (index, item) in self.items.into_iter().enumerate() {
            let item_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, current_y),
                Vec2::new(rect.width(), item_height),
            );

            let item_response = ui.interact(item_rect, egui::Id::new(("list_item", index)), Sense::click());

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

            // Calculate text area
            let trailing_width = if item.trailing_icon.is_some() { 40.0 } else { 0.0 };
            let _text_width = item_rect.max.x - content_x - trailing_width - 16.0;

            // Draw text content
            let text_color = if item.enabled { on_surface } else {
                get_global_color("onSurfaceVariant").linear_multiply(0.38)
            };

            if let Some(ref secondary) = item.secondary_text {
                // Two-line layout
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
            } else {
                // Single-line layout
                let text_pos = Pos2::new(content_x, content_y);
                ui.painter().text(
                    text_pos,
                    egui::Align2::LEFT_CENTER,
                    &item.primary_text,
                    egui::FontId::default(),
                    text_color,
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