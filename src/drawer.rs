use crate::theme::get_global_color;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke},
    pos2, Area, Id, Order, Rect, Response, Sense, SidePanel, Ui, Vec2, Widget,
};

/// Material Design navigation drawer variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerVariant {
    Permanent,
    Dismissible,
    Modal,
}

/// The alignment of a drawer (start or end side).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerAlignment {
    /// Drawer at the start side (left in LTR, right in RTL)
    Start,
    /// Drawer at the end side (right in LTR, left in RTL)
    End,
}

/// Theme data for Material Design drawers.
#[derive(Clone, Debug)]
pub struct DrawerThemeData {
    pub background_color: Option<Color32>,
    pub scrim_color: Option<Color32>,
    pub elevation: Option<f32>,
    pub shadow_color: Option<Color32>,
    pub surface_tint_color: Option<Color32>,
    pub shape: Option<CornerRadius>,
    pub end_shape: Option<CornerRadius>,
    pub width: Option<f32>,
    pub clip_behavior: Option<bool>,
}

impl Default for DrawerThemeData {
    fn default() -> Self {
        Self {
            background_color: None,
            scrim_color: None,
            elevation: None,
            shadow_color: None,
            surface_tint_color: None,
            shape: None,
            end_shape: None,
            width: None,
            clip_behavior: None,
        }
    }
}

impl DrawerThemeData {
    /// Create Material 3 defaults for drawer theming.
    pub fn material3_defaults() -> Self {
        Self {
            background_color: Some(get_global_color("surfaceContainerLow")),
            scrim_color: Some(Color32::from_rgba_unmultiplied(0, 0, 0, 138)),
            elevation: Some(1.0),
            shadow_color: Some(Color32::TRANSPARENT),
            surface_tint_color: Some(Color32::TRANSPARENT),
            shape: Some(CornerRadius::same(16)),
            end_shape: Some(CornerRadius::same(16)),
            width: Some(360.0),
            clip_behavior: Some(true),
        }
    }

    /// Create Material 2 defaults for drawer theming.
    pub fn material2_defaults() -> Self {
        Self {
            background_color: Some(get_global_color("surface")),
            scrim_color: Some(Color32::from_rgba_unmultiplied(0, 0, 0, 138)),
            elevation: Some(16.0),
            shadow_color: None,
            surface_tint_color: None,
            shape: Some(CornerRadius::ZERO),
            end_shape: Some(CornerRadius::ZERO),
            width: Some(304.0),
            clip_behavior: Some(true),
        }
    }
}

/// Material Design drawer header component.
///
/// Provides a header area for drawers with customizable decoration and content.
pub struct DrawerHeader {
    decoration_color: Option<Color32>,
    margin: f32,
    padding: Vec2,
    height: f32,
    title: Option<String>,
    subtitle: Option<String>,
}

impl Default for DrawerHeader {
    fn default() -> Self {
        Self {
            decoration_color: None,
            margin: 8.0,
            padding: Vec2::new(16.0, 16.0),
            height: 160.0,
            title: None,
            subtitle: None,
        }
    }
}

impl DrawerHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decoration_color(mut self, color: Color32) -> Self {
        self.decoration_color = Some(color);
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let rect = ui.allocate_space(Vec2::new(ui.available_width(), self.height + self.margin)).1;
        
        let header_rect = Rect::from_min_size(
            rect.min + Vec2::new(0.0, 0.0),
            Vec2::new(rect.width(), self.height),
        );

        // Draw decoration background
        let bg_color = self.decoration_color.unwrap_or_else(|| get_global_color("surfaceContainerHigh"));
        ui.painter().rect_filled(header_rect, CornerRadius::ZERO, bg_color);

        // Draw border at bottom
        let border_y = header_rect.max.y;
        ui.painter().line_segment(
            [egui::pos2(header_rect.min.x, border_y), egui::pos2(header_rect.max.x, border_y)],
            Stroke::new(1.0, get_global_color("outlineVariant")),
        );

        // Draw content with padding
        let content_rect = header_rect.shrink2(self.padding);
        
        if let Some(title) = &self.title {
            let title_pos = egui::pos2(content_rect.min.x, content_rect.min.y);
            ui.painter().text(
                title_pos,
                egui::Align2::LEFT_TOP,
                title,
                egui::FontId::proportional(22.0),
                get_global_color("onSurface"),
            );
        }

        if let Some(subtitle) = &self.subtitle {
            let subtitle_pos = egui::pos2(content_rect.min.x, content_rect.min.y + 32.0);
            ui.painter().text(
                subtitle_pos,
                egui::Align2::LEFT_TOP,
                subtitle,
                egui::FontId::proportional(14.0),
                get_global_color("onSurfaceVariant"),
            );
        }

        ui.interact(rect, ui.id().with("drawer_header"), Sense::hover())
    }
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
    alignment: DrawerAlignment,
    header_title: Option<String>,
    header_subtitle: Option<String>,
    items: Vec<DrawerItem>,
    sections: Vec<DrawerSection>,
    corner_radius: CornerRadius,
    elevation: Option<f32>,
    theme: DrawerThemeData,
    enable_drag_gesture: bool,
    edge_drag_width: Option<f32>,
    barrier_dismissible: bool,
    semantic_label: Option<String>,
    id: Id,
}

/// A section in the navigation drawer with a label and items.
pub struct DrawerSection {
    pub label: Option<String>,
    pub items: Vec<DrawerItem>,
}

/// A navigation item in a drawer.
pub struct DrawerItem {
    pub text: String,
    pub icon: Option<String>,
    pub active: bool,
    pub enabled: bool,
    pub badge: Option<String>,
    pub on_click: Option<Box<dyn Fn() + Send + Sync>>,
}

impl DrawerItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            active: false,
            enabled: true,
            badge: None,
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

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
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
    /// Create a new navigation drawer with Material 3 defaults.
    pub fn new(variant: DrawerVariant, open: &'a mut bool) -> Self {
        let id = Id::new(format!("material_drawer_{:?}", variant));
        let theme = DrawerThemeData::material3_defaults();
        let width = theme.width.unwrap_or(360.0);
        let corner_radius = theme.shape.unwrap_or(CornerRadius::same(16));
        let elevation = theme.elevation;
        
        Self {
            variant,
            open,
            width,
            alignment: DrawerAlignment::Start,
            header_title: None,
            header_subtitle: None,
            items: Vec::new(),
            sections: Vec::new(),
            corner_radius,
            elevation,
            theme,
            enable_drag_gesture: true,
            edge_drag_width: None,
            barrier_dismissible: true,
            semantic_label: None,
            id,
        }
    }

    /// Create a new navigation drawer with custom ID.
    pub fn new_with_id(variant: DrawerVariant, open: &'a mut bool, id: Id) -> Self {
        let theme = DrawerThemeData::material3_defaults();
        let width = theme.width.unwrap_or(360.0);
        let corner_radius = theme.shape.unwrap_or(CornerRadius::same(16));
        let elevation = theme.elevation;
        
        Self {
            variant,
            open,
            width,
            alignment: DrawerAlignment::Start,
            header_title: None,
            header_subtitle: None,
            items: Vec::new(),
            sections: Vec::new(),
            corner_radius,
            elevation,
            theme,
            enable_drag_gesture: true,
            edge_drag_width: None,
            barrier_dismissible: true,
            semantic_label: None,
            id,
        }
    }

    /// Set drawer alignment (start or end).
    pub fn alignment(mut self, alignment: DrawerAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set drawer width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set drawer theme.
    pub fn theme(mut self, theme: DrawerThemeData) -> Self {
        if let Some(width) = theme.width {
            self.width = width;
        }
        if let Some(shape) = theme.shape {
            self.corner_radius = shape;
        }
        if let Some(elevation) = theme.elevation {
            self.elevation = Some(elevation);
        }
        self.theme = theme;
        self
    }

    /// Enable or disable drag gestures.
    pub fn enable_drag_gesture(mut self, enable: bool) -> Self {
        self.enable_drag_gesture = enable;
        self
    }

    /// Set the edge drag width for opening the drawer.
    pub fn edge_drag_width(mut self, width: f32) -> Self {
        self.edge_drag_width = Some(width);
        self
    }

    /// Set whether tapping the barrier dismisses the drawer.
    pub fn barrier_dismissible(mut self, dismissible: bool) -> Self {
        self.barrier_dismissible = dismissible;
        self
    }

    /// Set semantic label for accessibility.
    pub fn semantic_label(mut self, label: impl Into<String>) -> Self {
        self.semantic_label = Some(label.into());
        self
    }

    /// Add header with title and optional subtitle.
    pub fn header(mut self, title: impl Into<String>, subtitle: Option<impl Into<String>>) -> Self {
        self.header_title = Some(title.into());
        self.header_subtitle = subtitle.map(|s| s.into());
        self
    }

    /// Add a navigation item.
    pub fn item(
        mut self,
        text: impl Into<String>,
        icon: Option<impl Into<String>>,
        active: bool,
    ) -> Self {
        self.items.push(DrawerItem {
            text: text.into(),
            icon: icon.map(|i| i.into()),
            active,
            enabled: true,
            badge: None,
            on_click: None,
        });
        self
    }

    /// Add a navigation item with callback.
    pub fn item_with_callback<F>(
        mut self,
        text: impl Into<String>,
        icon: Option<impl Into<String>>,
        active: bool,
        callback: F,
    ) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.items.push(DrawerItem {
            text: text.into(),
            icon: icon.map(|i| i.into()),
            active,
            enabled: true,
            badge: None,
            on_click: Some(Box::new(callback)),
        });
        self
    }

    /// Add a drawer item object.
    pub fn add_item(mut self, item: DrawerItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add a section with label and items.
    pub fn section(mut self, label: Option<impl Into<String>>, items: Vec<DrawerItem>) -> Self {
        self.sections.push(DrawerSection {
            label: label.map(|l| l.into()),
            items,
        });
        self
    }

    /// Set corner radius.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set elevation value.
    pub fn elevation(mut self, elevation: f32) -> Self {
        self.elevation = Some(elevation);
        self
    }

    fn get_drawer_style(&self) -> (Color32, Option<Stroke>, f32) {
        let background_color = self.theme.background_color
            .unwrap_or_else(|| get_global_color("surfaceContainerLow"));
        
        let elevation = self.elevation.unwrap_or(1.0);
        
        match self.variant {
            DrawerVariant::Permanent => {
                // Permanent drawer: surface with subtle border
                let border_color = get_global_color("outlineVariant");
                (background_color, Some(Stroke::new(1.0, border_color)), elevation)
            }
            DrawerVariant::Modal => {
                // Modal drawer: elevated surface, no border
                (background_color, None, elevation)
            }
            DrawerVariant::Dismissible => {
                // Dismissible drawer: surface with subtle border
                let border_color = get_global_color("outlineVariant");
                (background_color, Some(Stroke::new(1.0, border_color)), elevation)
            }
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
            .show(ctx, |ui| self.render_drawer_content(ui))
            .response
    }

    fn show_dismissible(self, ctx: &egui::Context) -> Response {
        if *self.open {
            SidePanel::left(self.id.with("dismissible"))
                .default_width(self.width)
                .resizable(false)
                .show(ctx, |ui| self.render_drawer_content(ui))
                .response
        } else {
            // Return empty response when closed
            Area::new(self.id.with("dismissible_dummy"))
                .fixed_pos(pos2(-1000.0, -1000.0)) // Place offscreen
                .show(ctx, |ui| ui.allocate_response(Vec2::ZERO, Sense::hover()))
                .response
        }
    }

    fn show_modal(self, ctx: &egui::Context) -> Response {
        if *self.open {
            // Draw scrim background
            let screen_rect = ctx.screen_rect();
            let scrim_color = self.theme.scrim_color
                .unwrap_or(Color32::from_rgba_unmultiplied(0, 0, 0, 138));
            
            Area::new(self.id.with("modal_scrim"))
                .order(Order::Background)
                .show(ctx, |ui| {
                    let scrim_response = ui.allocate_response(screen_rect.size(), Sense::click());
                    ui.painter().rect_filled(
                        screen_rect,
                        CornerRadius::ZERO,
                        scrim_color,
                    );

                    // Close drawer if scrim is clicked and barrier is dismissible
                    if scrim_response.clicked() && self.barrier_dismissible {
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
                .show(ctx, |ui| ui.allocate_response(Vec2::ZERO, Sense::hover()))
                .response
        }
    }

    fn render_drawer_content(self, ui: &mut Ui) -> Response {
        let (background_color, border_stroke, _elevation) = self.get_drawer_style();

        // Handle ESC key for dismissible and modal drawers
        if matches!(
            self.variant,
            DrawerVariant::Dismissible | DrawerVariant::Modal
        ) {
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                *self.open = false;
            }
        }

        let available_rect = ui.available_rect_before_wrap();
        let drawer_rect = Rect::from_min_size(
            available_rect.min,
            Vec2::new(self.width, available_rect.height()),
        );

        // Draw drawer background with corner radius
        ui.painter()
            .rect_filled(drawer_rect, self.corner_radius, background_color);

        // Draw border if present
        if let Some(stroke) = border_stroke {
            ui.painter().rect_stroke(
                drawer_rect,
                self.corner_radius,
                stroke,
                egui::epaint::StrokeKind::Outside,
            );
        }

        let mut current_y = drawer_rect.min.y;
        let item_height = 56.0; // Material 3 standard item height
        let section_padding_top = 16.0;
        let section_padding_bottom = 10.0;
        let horizontal_padding = 12.0; // Container padding

        // Draw header if present
        if let Some(title) = &self.header_title {
            let header_height = 64.0;
            let header_rect = Rect::from_min_size(
                egui::pos2(drawer_rect.min.x, current_y),
                Vec2::new(self.width, header_height),
            );

            // Header text with proper padding
            let title_pos = egui::pos2(header_rect.min.x + 28.0, header_rect.min.y + 16.0);
            ui.painter().text(
                title_pos,
                egui::Align2::LEFT_TOP,
                title,
                egui::FontId::proportional(22.0),
                get_global_color("onSurfaceVariant"),
            );

            if let Some(subtitle) = &self.header_subtitle {
                let subtitle_pos = egui::pos2(header_rect.min.x + 28.0, header_rect.min.y + 42.0);
                ui.painter().text(
                    subtitle_pos,
                    egui::Align2::LEFT_TOP,
                    subtitle,
                    egui::FontId::proportional(14.0),
                    get_global_color("onSurfaceVariant"),
                );
            }

            current_y += header_height;
        }

        let mut response = ui.allocate_response(drawer_rect.size(), Sense::hover());

        // Render sections if any
        if !self.sections.is_empty() {
            for (section_idx, section) in self.sections.iter().enumerate() {
                // Draw section label if present
                if let Some(label) = &section.label {
                    current_y += section_padding_top;
                    let label_pos = egui::pos2(drawer_rect.min.x + 28.0, current_y);
                    ui.painter().text(
                        label_pos,
                        egui::Align2::LEFT_TOP,
                        label,
                        egui::FontId::proportional(14.0),
                        get_global_color("onSurfaceVariant"),
                    );
                    current_y += section_padding_bottom + 10.0;
                }

                // Draw section items
                for (index, item) in section.items.iter().enumerate() {
                    let item_response = self.render_navigation_item(
                        ui,
                        item,
                        drawer_rect,
                        current_y,
                        item_height,
                        horizontal_padding,
                        self.id.with("section").with(section_idx).with(index),
                    );
                    response = response.union(item_response);
                    current_y += item_height;
                }

                // Add divider between sections (except after last section)
                if section_idx < self.sections.len() - 1 {
                    current_y += 8.0;
                    let divider_y = current_y;
                    ui.painter().line_segment(
                        [
                            egui::pos2(drawer_rect.min.x + 28.0, divider_y),
                            egui::pos2(drawer_rect.max.x - 28.0, divider_y),
                        ],
                        Stroke::new(1.0, get_global_color("outlineVariant")),
                    );
                    current_y += 8.0;
                }
            }
        } else {
            // Render simple items list if no sections
            for (index, item) in self.items.iter().enumerate() {
                let item_response = self.render_navigation_item(
                    ui,
                    item,
                    drawer_rect,
                    current_y,
                    item_height,
                    horizontal_padding,
                    self.id.with("item").with(index),
                );
                response = response.union(item_response);
                current_y += item_height;
            }
        }

        response
    }

    fn render_navigation_item(
        &self,
        ui: &mut Ui,
        item: &DrawerItem,
        drawer_rect: Rect,
        y_pos: f32,
        item_height: f32,
        horizontal_padding: f32,
        item_id: Id,
    ) -> Response {
        // Item container with padding
        let item_outer_rect = Rect::from_min_size(
            egui::pos2(drawer_rect.min.x + horizontal_padding, y_pos),
            Vec2::new(self.width - horizontal_padding * 2.0, item_height),
        );

        let item_response = ui.interact(item_outer_rect, item_id, Sense::click());

        // Active indicator (rounded rectangle on the left)
        if item.active {
            let indicator_width = item_outer_rect.width();
            let indicator_height = 32.0;
            let indicator_y = y_pos + (item_height - indicator_height) / 2.0;
            
            let indicator_rect = Rect::from_min_size(
                egui::pos2(item_outer_rect.min.x, indicator_y),
                Vec2::new(indicator_width, indicator_height),
            );

            let active_color = get_global_color("secondaryContainer");
            ui.painter().rect_filled(
                indicator_rect,
                CornerRadius::same(16),
                active_color,
            );
        } else if item_response.hovered() && item.enabled {
            let indicator_width = item_outer_rect.width();
            let indicator_height = 32.0;
            let indicator_y = y_pos + (item_height - indicator_height) / 2.0;
            
            let indicator_rect = Rect::from_min_size(
                egui::pos2(item_outer_rect.min.x, indicator_y),
                Vec2::new(indicator_width, indicator_height),
            );

            let hover_color = get_global_color("onSurface").linear_multiply(0.08);
            ui.painter().rect_filled(
                indicator_rect,
                CornerRadius::same(16),
                hover_color,
            );
        }

        let mut current_x = item_outer_rect.min.x + 16.0;

        // Draw icon if present
        if let Some(_icon) = &item.icon {
            let icon_center = egui::pos2(current_x + 12.0, y_pos + item_height / 2.0);
            let icon_color = if !item.enabled {
                get_global_color("onSurface").linear_multiply(0.38)
            } else if item.active {
                get_global_color("onSecondaryContainer")
            } else {
                get_global_color("onSurfaceVariant")
            };

            ui.painter().circle_filled(icon_center, 12.0, icon_color);
            current_x += 40.0;
        }

        // Draw item text
        let text_color = if !item.enabled {
            get_global_color("onSurface").linear_multiply(0.38)
        } else if item.active {
            get_global_color("onSecondaryContainer")
        } else {
            get_global_color("onSurfaceVariant")
        };

        let text_pos = egui::pos2(
            current_x,
            y_pos + (item_height - 20.0) / 2.0,
        );
        
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            &item.text,
            egui::FontId::proportional(14.0),
            text_color,
        );

        // Draw badge if present
        if let Some(badge) = &item.badge {
            let badge_x = item_outer_rect.max.x - 40.0;
            let badge_center = egui::pos2(badge_x, y_pos + item_height / 2.0);
            
            // Badge background
            ui.painter().circle_filled(
                badge_center,
                10.0,
                get_global_color("error"),
            );
            
            // Badge text
            ui.painter().text(
                badge_center,
                egui::Align2::CENTER_CENTER,
                badge,
                egui::FontId::proportional(10.0),
                get_global_color("onError"),
            );
        }

        // Handle item click
        if item_response.clicked() && item.enabled {
            if let Some(callback) = &item.on_click {
                callback();
            }
        }

        item_response
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
pub fn permanent_drawer(open: &mut bool) -> MaterialDrawer<'_> {
    MaterialDrawer::new(DrawerVariant::Permanent, open)
}

/// Convenience function to create a dismissible drawer.
pub fn dismissible_drawer(open: &mut bool) -> MaterialDrawer<'_> {
    MaterialDrawer::new(DrawerVariant::Dismissible, open)
}

/// Convenience function to create a modal drawer.
pub fn modal_drawer(open: &mut bool) -> MaterialDrawer<'_> {
    MaterialDrawer::new(DrawerVariant::Modal, open)
}

// Legacy support - these will be deprecated
pub fn standard_drawer(open: &mut bool) -> MaterialDrawer<'_> {
    permanent_drawer(open)
}
