use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius, Shadow},
    Rect, Response, Sense, Ui, Vec2, Widget,
};

/// Material Design navigation drawer variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerVariant {
    Standard,
    Modal,
    Dismissible,
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
/// let drawer = MaterialDrawer::new(DrawerVariant::Standard, &mut drawer_open)
///     .header("Mail", Some("email@material.io"))
///     .item("Inbox", Some("inbox"), true)
///     .item("Sent", Some("send"), false)
///     .item("Drafts", Some("drafts"), false);
///
/// ui.add(drawer);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialDrawer<'a> {
    variant: DrawerVariant,
    open: &'a mut bool,
    width: f32,
    header_title: Option<String>,
    header_subtitle: Option<String>,
    items: Vec<DrawerItem>,
    corner_radius: CornerRadius,
    elevation: Option<Shadow>,
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
        Self {
            variant,
            open,
            width: 256.0, // Standard Material Design drawer width
            header_title: None,
            header_subtitle: None,
            items: Vec::new(),
            corner_radius: CornerRadius::ZERO,
            elevation: None,
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
            DrawerVariant::Standard => {
                // Standard drawer: surface with border
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
}

impl Widget for MaterialDrawer<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (background_color, border_stroke, has_elevation) = self.get_drawer_style();
        
        let MaterialDrawer {
            variant,
            open,
            width,
            header_title,
            header_subtitle,
            items,
            corner_radius,
            elevation: _,
        } = self;

        // Only show drawer if it's open (for modal and dismissible) or if it's standard
        let should_show = match variant {
            DrawerVariant::Standard => true,
            DrawerVariant::Modal | DrawerVariant::Dismissible => *open,
        };

        if !should_show {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        // Calculate drawer dimensions
        let header_height = if header_title.is_some() { 64.0 } else { 0.0 };
        let item_height = 48.0;
        let items_height = items.len() as f32 * item_height;
        let total_height = header_height + items_height;

        let desired_size = Vec2::new(width, total_height);
        let mut response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw scrim for modal drawer
            if variant == DrawerVariant::Modal {
                let screen_rect = ui.ctx().screen_rect();
                let scrim_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);
                ui.painter().rect_filled(screen_rect, CornerRadius::ZERO, scrim_color);
                
                // Handle scrim click to close modal
                let scrim_response = ui.interact(screen_rect, ui.next_auto_id(), Sense::click());
                if scrim_response.clicked() {
                    *open = false;
                }
            }

            // Draw elevation shadow if needed
            if has_elevation {
                let shadow_rect = rect.translate(Vec2::new(2.0, 2.0));
                ui.painter().rect_filled(
                    shadow_rect,
                    corner_radius,
                    Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                );
            }

            // Draw drawer background
            ui.painter().rect_filled(rect, corner_radius, background_color);
            
            // Draw border if present
            if let Some(stroke) = border_stroke {
                ui.painter().rect_stroke(rect, corner_radius, stroke, egui::epaint::StrokeKind::Outside);
            }

            let mut current_y = rect.min.y;

            // Draw header if present
            if let Some(title) = &header_title {
                let header_rect = Rect::from_min_size(rect.min, Vec2::new(width, header_height));
                let header_bg = get_global_color("surfaceVariant");
                ui.painter().rect_filled(header_rect, CornerRadius::ZERO, header_bg);

                // Title
                let title_pos = egui::pos2(rect.min.x + 16.0, current_y + 16.0);
                ui.painter().text(
                    title_pos,
                    egui::Align2::LEFT_TOP,
                    title,
                    egui::FontId::proportional(18.0),
                    get_global_color("onSurface")
                );

                // Subtitle if present
                if let Some(subtitle) = &header_subtitle {
                    let subtitle_pos = egui::pos2(rect.min.x + 16.0, current_y + 40.0);
                    ui.painter().text(
                        subtitle_pos,
                        egui::Align2::LEFT_TOP,
                        subtitle,
                        egui::FontId::proportional(14.0),
                        get_global_color("onSurfaceVariant")
                    );
                }

                current_y += header_height;
            }

            // Draw navigation items
            for item in &items {
                let item_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(width, item_height)
                );

                // Item background
                let item_bg = if item.active {
                    get_global_color("primaryContainer") // Selected state
                } else {
                    Color32::TRANSPARENT
                };

                ui.painter().rect_filled(item_rect, CornerRadius::ZERO, item_bg);

                // Handle item interaction
                let item_response = ui.interact(item_rect, ui.next_auto_id(), Sense::click());
                if item_response.hovered() && !item.active {
                    let hover_color = get_global_color("primary").linear_multiply(0.05);
                    ui.painter().rect_filled(item_rect, CornerRadius::ZERO, hover_color);
                }

                if item_response.clicked() {
                    if let Some(callback) = &item.on_click {
                        callback();
                    }
                    // For modal drawers, close on item click
                    if variant == DrawerVariant::Modal {
                        *open = false;
                    }
                }

                let mut current_x = rect.min.x + 16.0;

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

                current_y += item_height;
            }
        }

        // Add close button interaction for modal and dismissible variants
        if variant != DrawerVariant::Standard {
            // ESC key handling
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                *open = false;
            }
        }

        response
    }
}

/// Convenience function to create a standard drawer.
pub fn standard_drawer(open: &mut bool) -> MaterialDrawer {
    MaterialDrawer::new(DrawerVariant::Standard, open)
}

/// Convenience function to create a modal drawer.
pub fn modal_drawer(open: &mut bool) -> MaterialDrawer {
    MaterialDrawer::new(DrawerVariant::Modal, open)
}

/// Convenience function to create a dismissible drawer.
pub fn dismissible_drawer(open: &mut bool) -> MaterialDrawer {
    MaterialDrawer::new(DrawerVariant::Dismissible, open)
}