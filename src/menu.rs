use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Context, Id};

pub struct MaterialMenu<'a> {
    id: Id,
    open: &'a mut bool,
    anchor_rect: Option<Rect>,
    items: Vec<MenuItem<'a>>,
    elevation: u8,
}

pub struct MenuItem<'a> {
    text: String,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    enabled: bool,
    divider_after: bool,
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialMenu<'a> {
    pub fn new(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self {
            id: id.into(),
            open,
            anchor_rect: None,
            items: Vec::new(),
            elevation: 3,
        }
    }

    pub fn anchor_rect(mut self, rect: Rect) -> Self {
        self.anchor_rect = Some(rect);
        self
    }

    pub fn item(mut self, item: MenuItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    pub fn elevation(mut self, elevation: u8) -> Self {
        self.elevation = elevation;
        self
    }

    pub fn show(self, ctx: &Context) {
        if !*self.open {
            return;
        }

        let item_height = 48.0;
        let total_height = self.items.len() as f32 * item_height + 
                          self.items.iter().filter(|item| item.divider_after).count() as f32;
        let menu_width = 280.0;

        let menu_size = Vec2::new(menu_width, total_height);

        // Determine position
        let position = if let Some(anchor) = self.anchor_rect {
            // Position menu below the anchor
            Pos2::new(anchor.min.x, anchor.max.y + 4.0)
        } else {
            // Center on screen
            let screen_rect = ctx.screen_rect();
            screen_rect.center() - menu_size / 2.0
        };

        let open_ref = self.open;
        let id = self.id;
        let items = self.items;
        let elevation = self.elevation;

        // Create a popup window for the menu with a stable layer and unique ID
        let stable_id = egui::Id::new(format!("menu_{}", id.value()));
        let area_response = egui::Area::new(stable_id)
            .fixed_pos(position)
            .order(egui::Order::Foreground)
            .interactable(true)
            .show(ctx, |ui| {
                render_menu_content(ui, menu_size, items, elevation, open_ref)
            });

        // Only close menu on explicit click or escape, not on mouse outside
        // This prevents the menu from closing when mouse cursor is outside
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *open_ref = false;
        }
    }

}

fn render_menu_content<'a>(ui: &mut Ui, size: Vec2, items: Vec<MenuItem<'a>>, elevation: u8, open_ref: &'a mut bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

    // Material Design colors
    let surface_container = Color32::from_gray(if ui.visuals().dark_mode { 28 } else { 244 });
    let on_surface = Color32::from_gray(if ui.visuals().dark_mode { 230 } else { 30 });
    let on_surface_variant = Color32::from_gray(if ui.visuals().dark_mode { 202 } else { 73 });
    let outline_variant = Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 231 });

    // Draw shadow for elevation
    let shadow_offset = elevation as f32 * 2.0;
    let shadow_rect = rect.expand(shadow_offset);
    ui.painter().rect_filled(
        shadow_rect,
        8.0,
        Color32::from_black_alpha((elevation * 10).min(80)),
    );

    // Draw menu background
    ui.painter().rect_filled(rect, 8.0, surface_container);

    // Draw menu border
    ui.painter().rect_stroke(
        rect,
        8.0,
        Stroke::new(1.0, outline_variant),
        egui::epaint::StrokeKind::Outside,
    );

    let mut current_y = rect.min.y + 8.0;
    let mut pending_actions = Vec::new();
    let mut should_close = false;

    for (index, item) in items.into_iter().enumerate() {
        let item_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 8.0, current_y),
            Vec2::new(rect.width() - 16.0, 48.0),
        );

        let item_response = ui.interact(
            item_rect,
            egui::Id::new(format!("menu_item_{}_{}", rect.min.x as i32, index)),
            Sense::click(),
        );

        // Draw item background on hover
        if item_response.hovered() && item.enabled {
            let hover_color = Color32::from_rgba_premultiplied(
                on_surface.r(), on_surface.g(), on_surface.b(), 20
            );
            ui.painter().rect_filled(item_rect, 4.0, hover_color);
        }

        // Handle click
        if item_response.clicked() && item.enabled {
            if let Some(action) = item.action {
                pending_actions.push(action);
                // Only close menu after item click
                should_close = true;
            }
        }

        // Layout item content
        let mut content_x = item_rect.min.x + 12.0;
        let content_y = item_rect.center().y;

        // Draw leading icon
        if let Some(_icon) = &item.leading_icon {
            let icon_rect = Rect::from_min_size(
                Pos2::new(content_x, content_y - 12.0),
                Vec2::splat(24.0),
            );
            
            let icon_color = if item.enabled { on_surface_variant } else {
                Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 189 })
            };

            ui.painter().circle_filled(icon_rect.center(), 8.0, icon_color);
            content_x += 36.0;
        }

        // Draw text
        let text_color = if item.enabled { on_surface } else {
            Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 189 })
        };

        let text_pos = Pos2::new(content_x, content_y);
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            &item.text,
            egui::FontId::default(),
            text_color,
        );

        // Draw trailing icon
        if let Some(_icon) = &item.trailing_icon {
            let icon_rect = Rect::from_min_size(
                Pos2::new(item_rect.max.x - 36.0, content_y - 12.0),
                Vec2::splat(24.0),
            );
            
            let icon_color = if item.enabled { on_surface_variant } else {
                Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 189 })
            };

            ui.painter().circle_filled(icon_rect.center(), 8.0, icon_color);
        }

        current_y += 48.0;

        // Draw divider
        if item.divider_after {
            let divider_y = current_y;
            let divider_start = Pos2::new(rect.min.x + 12.0, divider_y);
            let divider_end = Pos2::new(rect.max.x - 12.0, divider_y);
            
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

    if should_close {
        *open_ref = false;
    }

    response
}

impl<'a> MenuItem<'a> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            leading_icon: None,
            trailing_icon: None,
            enabled: true,
            divider_after: false,
            action: None,
        }
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn divider_after(mut self, divider: bool) -> Self {
        self.divider_after = divider;
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

pub fn menu(id: impl Into<egui::Id>, open: &mut bool) -> MaterialMenu {
    MaterialMenu::new(id, open)
}

pub fn menu_item(text: impl Into<String>) -> MenuItem<'static> {
    MenuItem::new(text)
}