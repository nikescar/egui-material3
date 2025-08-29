use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius, Shadow},
    Rect, Response, Sense, Ui, Vec2, Widget, Id, Area, SidePanel, Order, pos2,
};

/// Material Design navigation drawer variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerVariant {
    Permanent,
    Dismissible, 
    Modal,
}

/// Material Design navigation drawer component.
///
/// Navigation drawers provide access to destinations and app functionality.
/// They can be permanently on-screen or controlled by navigation triggers.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let mut drawer_open = true;
/// 
/// let drawer = MaterialDrawer::new(DrawerVariant::Permanent, &mut drawer_open)
///     .header("Mail", Some("email@material.io"))
///     .item("Inbox", Some("inbox"), true)
///     .item("Sent", Some("send"), false)
///     .item("Drafts", Some("drafts"), false);
///
/// drawer.show(ui.ctx());
/// # });
/// ```
pub struct MaterialDrawer<'a> {
    variant: DrawerVariant,
    open: &'a mut bool,
    width: f32,
    header_title: Option<String>,
    header_subtitle: Option<String>,
    items: Vec<DrawerItem>,
    corner_radius: CornerRadius,
    elevation: Option<Shadow>,
    id: Id,
}

pub struct DrawerItem {
    pub text: String,
    pub icon: Option<String>,
    pub active: bool,
    pub on_click: Option<Box<dyn Fn() + Send + Sync>>,
}

impl DrawerItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            active: false,
            on_click: None,
        }
    }
    
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    
    pub fn on_click<F>(mut self, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl<'a> MaterialDrawer<'a> {
    /// Create a new navigation drawer.
    pub fn new(variant: DrawerVariant, open: &'a mut bool) -> Self {
        let id = Id::new(format!("material_drawer_{:?}", variant));
        Self {
            variant,
            open,
            width: 256.0, // Standard Material Design drawer width
            header_title: None,
            header_subtitle: None,
            items: Vec::new(),
            corner_radius: CornerRadius::ZERO,
            elevation: None,
            id,
        }
    }

    /// Create a new navigation drawer with custom ID.
    pub fn new_with_id(variant: DrawerVariant, open: &'a mut bool, id: Id) -> Self {
        Self {
            variant,
            open,
            width: 256.0,
            header_title: None,
            header_subtitle: None,
            items: Vec::new(),
            corner_radius: CornerRadius::ZERO,
            elevation: None,
            id,
        }
    }

    /// Set drawer width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Add header with title and optional subtitle.
    pub fn header(mut self, title: impl Into<String>, subtitle: Option<impl Into<String>>) -> Self {
        self.header_title = Some(title.into());
        self.header_subtitle = subtitle.map(|s| s.into());
        self
    }

    /// Add a navigation item.
    pub fn item(mut self, text: impl Into<String>, icon: Option<impl Into<String>>, active: bool) -> Self {
        self.items.push(DrawerItem {
            text: text.into(),
            icon: icon.map(|i| i.into()),
            active,
            on_click: None,
        });
        self
    }

    /// Add a navigation item with callback.
    pub fn item_with_callback<F>(mut self, text: impl Into<String>, icon: Option<impl Into<String>>, active: bool, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.items.push(DrawerItem {
            text: text.into(),
            icon: icon.map(|i| i.into()),
            active,
            on_click: Some(Box::new(callback)),
        });
        self
    }

    /// Set corner radius.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set elevation shadow.
    pub fn elevation(mut self, elevation: impl Into<Shadow>) -> Self {
        self.elevation = Some(elevation.into());
        self
    }

    fn get_drawer_style(&self) -> (Color32, Option<Stroke>, bool) {
        let md_surface = get_global_color("surface");
        let md_outline = get_global_color("outline");
        
        match self.variant {
            DrawerVariant::Permanent => {
                // Permanent drawer: surface with border
                (md_surface, Some(Stroke::new(1.0, md_outline)), false)
            },
            DrawerVariant::Modal => {
                // Modal drawer: surface with elevation
                (md_surface, None, true)
            },
            DrawerVariant::Dismissible => {
                // Dismissible drawer: surface with border
                (md_surface, Some(Stroke::new(1.0, md_outline)), false)
            },
        }
    }

    /// Show the drawer using appropriate egui layout.
    pub fn show(self, ctx: &egui::Context) -> Response {
        match self.variant {
            DrawerVariant::Permanent => self.show_permanent(ctx),
            DrawerVariant::Dismissible => self.show_dismissible(ctx),
            DrawerVariant::Modal => self.show_modal(ctx),
        }
    }

    fn show_permanent(self, ctx: &egui::Context) -> Response {
        SidePanel::left(self.id.with("permanent"))
            .default_width(self.width)
            .resizable(false)
            .show(ctx, |ui| {
                self.render_drawer_content(ui)
            })
            .response
    }

    fn show_dismissible(self, ctx: &egui::Context) -> Response {
        if *self.open {
            SidePanel::left(self.id.with("dismissible"))
                .default_width(self.width)
                .resizable(false)
                .show(ctx, |ui| {
                    self.render_drawer_content(ui)
                })
                .response
        } else {
            // Return empty response when closed
            Area::new(self.id.with("dismissible_dummy"))
                .fixed_pos(pos2(-1000.0, -1000.0)) // Place offscreen
                .show(ctx, |ui| {
                    ui.allocate_response(Vec2::ZERO, Sense::hover())
                })
                .response
        }
    }

    fn show_modal(self, ctx: &egui::Context) -> Response {
        if *self.open {
            // Draw scrim background
            let screen_rect = ctx.screen_rect();
            Area::new(self.id.with("modal_scrim"))
                .order(Order::Background)
                .show(ctx, |ui| {
                    let scrim_response = ui.allocate_response(screen_rect.size(), Sense::click());
                    ui.painter().rect_filled(
                        screen_rect,
                        CornerRadius::ZERO,
                        Color32::from_rgba_unmultiplied(0, 0, 0, 128), // Semi-transparent scrim
                    );
                    
                    // Close drawer if scrim is clicked
                    if scrim_response.clicked() {
                        *self.open = false;
                    }
                });

            // Draw the actual modal drawer
            Area::new(self.id.with("modal_drawer"))
                .order(Order::Foreground)
                .fixed_pos(pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.set_width(self.width);
                    ui.set_height(screen_rect.height());
                    self.render_drawer_content(ui)
                })
                .response
        } else {
            // Return empty response when closed
            Area::new(self.id.with("modal_dummy"))
                .fixed_pos(pos2(-1000.0, -1000.0)) // Place offscreen
                .show(ctx, |ui| {
                    ui.allocate_response(Vec2::ZERO, Sense::hover())
                })
                .response
        }
    }

    fn render_drawer_content(self, ui: &mut Ui) -> Response {
        let (background_color, border_stroke, has_elevation) = self.get_drawer_style();
        
        // Handle ESC key for dismissible and modal drawers
        if matches!(self.variant, DrawerVariant::Dismissible | DrawerVariant::Modal) {
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                *self.open = false;
            }
        }

        // Calculate drawer dimensions
        let header_height = if self.header_title.is_some() { 64.0 } else { 0.0 };
        let item_height = 48.0;
        let items_height = self.items.len() as f32 * item_height;
        let _total_height = header_height + items_height;

        let available_rect = ui.available_rect_before_wrap();
        let drawer_rect = Rect::from_min_size(available_rect.min, Vec2::new(self.width, available_rect.height()));

        // Draw drawer background
        ui.painter().rect_filled(drawer_rect, self.corner_radius, background_color);
        
        // Draw border if present
        if let Some(stroke) = border_stroke {
            ui.painter().rect_stroke(drawer_rect, self.corner_radius, stroke, egui::epaint::StrokeKind::Outside);
        }

        // Draw elevation shadow if needed (simplified approach for egui)
        if has_elevation {
            // Draw a simple drop shadow by drawing darker rectangles behind
            let shadow_offset = Vec2::new(0.0, 4.0);
            let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, 20);
            
            // Draw multiple shadow layers for better effect
            for i in 1..=3 {
                let shadow_rect = drawer_rect.translate(shadow_offset * i as f32 * 0.5);
                ui.painter().rect_filled(shadow_rect, self.corner_radius, shadow_color);
            }
        }

        let mut current_y = drawer_rect.min.y;

        // Draw header if present
        if let Some(title) = &self.header_title {
            let header_rect = Rect::from_min_size(
                egui::pos2(drawer_rect.min.x, current_y),
                Vec2::new(self.width, header_height)
            );

            // Header background (slightly different color)
            let header_color = background_color.linear_multiply(0.95);
            ui.painter().rect_filled(header_rect, CornerRadius::ZERO, header_color);

            // Header text
            let title_pos = egui::pos2(
                header_rect.min.x + 16.0,
                header_rect.min.y + 16.0
            );
            ui.painter().text(
                title_pos,
                egui::Align2::LEFT_TOP,
                title,
                egui::TextStyle::Heading.resolve(ui.style()),
                get_global_color("onSurface")
            );

            if let Some(subtitle) = &self.header_subtitle {
                let subtitle_pos = egui::pos2(
                    header_rect.min.x + 16.0,
                    header_rect.min.y + 36.0
                );
                ui.painter().text(
                    subtitle_pos,
                    egui::Align2::LEFT_TOP,
                    subtitle,
                    egui::TextStyle::Body.resolve(ui.style()),
                    get_global_color("onSurfaceVariant")
                );
            }

            current_y += header_height;
        }

        let mut response = ui.allocate_response(drawer_rect.size(), Sense::hover());

        // Draw navigation items with unique IDs
        for (index, item) in self.items.iter().enumerate() {
            let item_rect = Rect::from_min_size(
                egui::pos2(drawer_rect.min.x, current_y),
                Vec2::new(self.width, item_height)
            );

            // Create unique ID for each item
            let item_id = self.id.with("item").with(index);
            let item_response = ui.interact(item_rect, item_id, Sense::click());

            // Draw item background for active state or hover
            if item.active {
                let active_color = get_global_color("primary").linear_multiply(0.12);
                ui.painter().rect_filled(item_rect, CornerRadius::ZERO, active_color);
            } else if item_response.hovered() {
                let hover_color = get_global_color("onSurface").linear_multiply(0.08);
                ui.painter().rect_filled(item_rect, CornerRadius::ZERO, hover_color);
            }

            let mut current_x = item_rect.min.x + 16.0;

            // Draw icon if present
            if let Some(_icon) = &item.icon {
                // Draw a simple placeholder for the icon (circle)
                let icon_center = egui::pos2(current_x + 12.0, current_y + item_height / 2.0);
                let icon_color = if item.active {
                    get_global_color("primary")
                } else {
                    get_global_color("onSurfaceVariant")
                };
                
                ui.painter().circle_filled(icon_center, 10.0, icon_color);
                current_x += 40.0; // Icon width + spacing
            }

            // Draw item text
            let text_color = if item.active {
                get_global_color("primary")
            } else {
                get_global_color("onSurface")
            };

            let text_pos = egui::pos2(current_x, current_y + (item_height - ui.text_style_height(&egui::TextStyle::Body)) / 2.0);
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_TOP,
                &item.text,
                egui::TextStyle::Body.resolve(ui.style()),
                text_color
            );

            // Handle item click
            if item_response.clicked() {
                if let Some(callback) = &item.on_click {
                    callback();
                }
            }

            response = response.union(item_response);
            current_y += item_height;
        }

        response
    }
}

impl Widget for MaterialDrawer<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // This implementation is kept for backward compatibility
        // but the preferred way is to use the show() method
        self.render_drawer_content(ui)
    }
}

/// Convenience function to create a permanent drawer.
pub fn permanent_drawer(open: &mut bool) -> MaterialDrawer {
    MaterialDrawer::new(DrawerVariant::Permanent, open)
}

/// Convenience function to create a dismissible drawer.
pub fn dismissible_drawer(open: &mut bool) -> MaterialDrawer {
    MaterialDrawer::new(DrawerVariant::Dismissible, open)
}

/// Convenience function to create a modal drawer.
pub fn modal_drawer(open: &mut bool) -> MaterialDrawer {
    MaterialDrawer::new(DrawerVariant::Modal, open)
}

// Legacy support - these will be deprecated
pub fn standard_drawer(open: &mut bool) -> MaterialDrawer {
    permanent_drawer(open)
}