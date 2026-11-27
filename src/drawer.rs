//! Material Design 3 Navigation Drawer Components
//!
//! Values from the Jetpack Compose material3 token files
//! (`androidx.compose.material3.tokens`, `androidx-main`),
//! `NavigationDrawerTokens` unless noted:
//!
//! - **surface** (`StandardContainerColor`, `Level0`): permanent and
//!   dismissible drawers, square-cornered.
//! - **surfaceContainerLow** (`ModalContainerColor`, `Level1`): the modal
//!   drawer, with `CornerLargeEnd` (16 dp on the edge facing the page).
//! - **secondaryContainer** (`ActiveIndicatorColor`): the active item's
//!   56 dp `CornerFull` indicator, 12 dp in from each side (336 dp wide in a
//!   360 dp drawer).
//! - **onSecondaryContainer**: active icon and label; **onSurfaceVariant**:
//!   inactive icon, label and section headline (`TitleSmall`); labels are
//!   `LabelLarge`.
//! - **scrim @ 32%** (`ScrimTokens`): behind the modal drawer, above the
//!   page and its windows, so it blocks clicks.
//! - **State layers**: onSecondaryContainer (active) or onSurface
//!   (inactive) at the `StateLayerTokens` opacities.
//!
//! Start and end alignment follow the theme's layout direction, and the
//! dismissible and modal drawers slide in with the `DefaultSpatial` spring.

use crate::navigation::{
    describe_item, modal_scrim, paint_icon, rove_focus, scrim_color, text_galley, RoveAxis,
};
use crate::theme::get_global_color;
use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{
    animate_value_with_spring, ColorRole, InteractionState, LayoutDirection, Shape, SpringSpeed,
    TypeRole,
};
use crate::MaterialTheme;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke},
    pos2, Area, Id, Order, Rect, Response, Sense, Ui, Vec2, Widget,
};

/// `NavigationDrawerTokens.ContainerWidth`.
pub const DRAWER_WIDTH: f32 = 360.0;
/// `NavigationDrawerTokens.ActiveIndicatorHeight`: one item.
pub const ITEM_HEIGHT: f32 = 56.0;
/// `NavigationDrawerItemDefaults.ItemPadding`: items sit 12 dp in from each
/// side.
pub const ITEM_INSET: f32 = 12.0;
/// `NavigationDrawerTokens.IconSize`.
pub const ICON_SIZE: f32 = 24.0;
/// Where headlines, the header and dividers start: 28 dp, lined up with the
/// item labels' 12 + 16 dp.
const HEADLINE_INSET: f32 = 28.0;

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
#[derive(Clone, Debug, Default)]
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

impl DrawerThemeData {
    /// Create Material 3 defaults for drawer theming.
    ///
    /// Colours, scrim and corners are left unset: they depend on the variant
    /// and come from the context's [`MaterialTheme`] when the drawer is drawn.
    pub fn material3_defaults() -> Self {
        Self {
            background_color: None,
            scrim_color: None,
            elevation: Some(1.0),
            shadow_color: Some(Color32::TRANSPARENT),
            surface_tint_color: Some(Color32::TRANSPARENT),
            shape: None,
            end_shape: None,
            width: Some(DRAWER_WIDTH),
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
        let rect = ui
            .allocate_space(Vec2::new(ui.available_width(), self.height + self.margin))
            .1;

        let header_rect = Rect::from_min_size(
            rect.min + Vec2::new(0.0, 0.0),
            Vec2::new(rect.width(), self.height),
        );

        let theme = MaterialTheme::of(ui.ctx());
        // Draw decoration background
        let bg_color = self
            .decoration_color
            .unwrap_or_else(|| theme.color(ColorRole::SurfaceContainerHigh));
        ui.painter()
            .rect_filled(header_rect, CornerRadius::ZERO, bg_color);

        // Draw border at bottom
        let border_y = header_rect.max.y;
        ui.painter().line_segment(
            [
                egui::pos2(header_rect.min.x, border_y),
                egui::pos2(header_rect.max.x, border_y),
            ],
            Stroke::new(1.0, theme.color(ColorRole::OutlineVariant)),
        );

        // Draw content with padding
        let content_rect = header_rect.shrink2(self.padding);

        if let Some(title) = &self.title {
            let title_pos = egui::pos2(content_rect.min.x, content_rect.min.y);
            ui.painter().text(
                title_pos,
                egui::Align2::LEFT_TOP,
                title,
                TypeRole::TitleLarge.style().font_id(ui.ctx()),
                theme.color(ColorRole::OnSurface),
            );
        }

        if let Some(subtitle) = &self.subtitle {
            let subtitle_pos = egui::pos2(content_rect.min.x, content_rect.min.y + 32.0);
            ui.painter().text(
                subtitle_pos,
                egui::Align2::LEFT_TOP,
                subtitle,
                TypeRole::BodyMedium.style().font_id(ui.ctx()),
                theme.color(ColorRole::OnSurfaceVariant),
            );
        }

        ui.interact(rect, ui.scope_id().with("drawer_header"), Sense::hover())
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
    corner_radius: Option<CornerRadius>,
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
        let id = Id::unique(format!("material_drawer_{:?}", variant));
        let theme = DrawerThemeData::material3_defaults();
        let width = theme.width.unwrap_or(DRAWER_WIDTH);
        let corner_radius = theme.shape;
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
        let width = theme.width.unwrap_or(DRAWER_WIDTH);
        let corner_radius = theme.shape;
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
            self.corner_radius = Some(shape);
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
        self.corner_radius = Some(corner_radius.into());
        self
    }

    /// Set elevation value.
    pub fn elevation(mut self, elevation: f32) -> Self {
        self.elevation = Some(elevation);
        self
    }

    /// Whether the drawer sits on the left edge: start in left-to-right
    /// layouts, end in right-to-left ones.
    fn on_left(&self, direction: LayoutDirection) -> bool {
        matches!(
            (self.alignment, direction),
            (DrawerAlignment::Start, LayoutDirection::Ltr)
                | (DrawerAlignment::End, LayoutDirection::Rtl)
        )
    }

    /// The container colour and corners for this variant: standard drawers
    /// are `surface` and square (`StandardContainerColor`, Compose's
    /// `RectangleShape` sheets), the modal drawer `surfaceContainerLow` with
    /// `CornerLargeEnd` (`ModalContainerColor`, `ContainerShape`), the
    /// rounded corners facing the page.
    fn container_style(&self, theme: &MaterialTheme, rect: Rect) -> (Color32, CornerRadius) {
        let background = self.theme.background_color.unwrap_or_else(|| {
            theme.color(match self.variant {
                DrawerVariant::Permanent | DrawerVariant::Dismissible => ColorRole::Surface,
                DrawerVariant::Modal => ColorRole::SurfaceContainerLow,
            })
        });
        let radius = self.corner_radius.unwrap_or_else(|| match self.variant {
            DrawerVariant::Permanent | DrawerVariant::Dismissible => CornerRadius::ZERO,
            DrawerVariant::Modal => {
                // Round the edge facing the page.
                let shape = match self.alignment {
                    DrawerAlignment::Start => Shape::LARGE_END,
                    DrawerAlignment::End => Shape::LARGE_START,
                };
                shape.corner_radius(rect, theme.layout_direction)
            }
        });
        (background, radius)
    }

    /// The open fraction, 0 (closed) to 1 (open), sliding with Compose's
    /// `DefaultSpatial` spring. A drawer shown open on its first frame
    /// starts open.
    fn slide(&self, ctx: &egui::Context, theme: &MaterialTheme) -> f32 {
        animate_value_with_spring(
            ctx,
            self.id.with("slide"),
            if *self.open { 1.0 } else { 0.0 },
            theme.motion.spatial(SpringSpeed::Default),
        )
        .clamp(0.0, 1.0)
    }

    /// Show the drawer using appropriate egui layout.
    ///
    /// Permanent and dismissible drawers are shown as an [`egui::Panel`] on
    /// the drawer's side inside `ui` (typically the root `Ui` passed to
    /// `eframe::App::ui`); modal drawers are shown as overlay areas above a
    /// scrim.
    pub fn show(self, ui: &mut Ui) -> Response {
        match self.variant {
            DrawerVariant::Permanent => self.show_permanent(ui),
            DrawerVariant::Dismissible => self.show_dismissible(ui),
            DrawerVariant::Modal => self.show_modal(ui.ctx()),
        }
    }

    fn panel(&self, theme: &MaterialTheme, salt: &str, width: f32) -> egui::Panel {
        let id = self.id.with(salt);
        let panel = if self.on_left(theme.layout_direction) {
            egui::Panel::left(id)
        } else {
            egui::Panel::right(id)
        };
        panel
            .exact_size(width)
            .resizable(false)
            .show_separator_line(false)
            .frame(egui::Frame::NONE)
    }

    fn show_permanent(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        self.panel(&theme, "permanent", self.width)
            .show(ui, |ui| self.render_drawer_content(ui, 0.0))
            .inner
    }

    fn show_dismissible(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let open_fraction = self.slide(ui.ctx(), &theme);
        if open_fraction <= 0.0 {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }
        // The panel narrows as the drawer slides out; its content slides
        // with it rather than squashing.
        let hidden = self.width * (1.0 - open_fraction);
        let shift = if self.on_left(theme.layout_direction) {
            -hidden
        } else {
            0.0
        };
        self.panel(&theme, "dismissible", self.width * open_fraction)
            .show(ui, |ui| {
                ui.set_clip_rect(ui.max_rect());
                self.render_drawer_content(ui, shift)
            })
            .inner
    }

    fn show_modal(self, ctx: &egui::Context) -> Response {
        let theme = MaterialTheme::of(ctx);
        let open_fraction = self.slide(ctx, &theme);
        let screen_rect = ctx.content_rect();
        if open_fraction <= 0.0 {
            // Closed: take no space and nothing else.
            return Area::new(self.id.with("modal_dummy"))
                .fixed_pos(screen_rect.min)
                .show(ctx, |ui| ui.allocate_response(Vec2::ZERO, Sense::hover()))
                .response;
        }

        // The scrim covers the whole page, above panels and windows, so it
        // dims the page and blocks interaction with it. It fades with the
        // slide (`ScrimTokens`: `scrim` at 32 %).
        let scrim_color = match self.theme.scrim_color {
            Some(color) => color.gamma_multiply(open_fraction),
            None => scrim_color(&theme, open_fraction),
        };
        let scrim = modal_scrim(ctx, self.id.with("modal_scrim"), scrim_color);
        if scrim.clicked() && self.barrier_dismissible {
            *self.open = false;
        }

        // The sheet itself, kept above its scrim, sliding in from its edge.
        let x = if self.on_left(theme.layout_direction) {
            screen_rect.min.x - self.width * (1.0 - open_fraction)
        } else {
            screen_rect.max.x - self.width * open_fraction
        };
        let drawer_id = self.id.with("modal_drawer");
        ctx.move_to_top(egui::LayerId::new(Order::Foreground, drawer_id));
        Area::new(drawer_id)
            .order(Order::Foreground)
            .fade_in(false)
            .fixed_pos(pos2(x, screen_rect.min.y))
            .show(ctx, |ui| {
                ui.set_width(self.width);
                ui.set_height(screen_rect.height());
                self.render_drawer_content(ui, 0.0)
            })
            .inner
    }

    /// Draws the sheet, header, sections and items, `shift` points along x
    /// from where the ui would put it.
    fn render_drawer_content(self, ui: &mut Ui, shift: f32) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let rtl = theme.layout_direction == LayoutDirection::Rtl;

        // Handle ESC key for dismissible and modal drawers
        if matches!(
            self.variant,
            DrawerVariant::Dismissible | DrawerVariant::Modal
        ) && ui.input(|i| i.key_pressed(egui::Key::Escape))
        {
            *self.open = false;
        }

        let available_rect = ui.available_rect_before_wrap();
        let drawer_rect = Rect::from_min_size(
            pos2(available_rect.min.x + shift, available_rect.min.y),
            Vec2::new(self.width, available_rect.height()),
        );
        // A span `from_start` points in from the drawer's start edge.
        let span = |from_start: f32, width: f32| -> (f32, f32) {
            if rtl {
                let max = drawer_rect.max.x - from_start;
                (max - width, max)
            } else {
                let min = drawer_rect.min.x + from_start;
                (min, min + width)
            }
        };

        let (background_color, corner_radius) = self.container_style(&theme, drawer_rect);
        let painter = ui.painter().clone();
        painter.rect_filled(drawer_rect, corner_radius, background_color);

        let mut current_y = drawer_rect.min.y;

        if let Some(title) = &self.header_title {
            let header_height = 64.0;
            let title = text_galley(
                &painter,
                TypeRole::TitleLarge,
                title,
                theme.color(ColorRole::OnSurface),
            );
            let (x, _) = span(HEADLINE_INSET, title.size().x);
            painter.galley(
                pos2(x, current_y + 16.0),
                title,
                theme.color(ColorRole::OnSurface),
            );
            if let Some(subtitle) = &self.header_subtitle {
                let subtitle = text_galley(
                    &painter,
                    TypeRole::BodyMedium,
                    subtitle,
                    theme.color(ColorRole::OnSurfaceVariant),
                );
                let (x, _) = span(HEADLINE_INSET, subtitle.size().x);
                painter.galley(
                    pos2(x, current_y + 42.0),
                    subtitle,
                    theme.color(ColorRole::OnSurfaceVariant),
                );
            }
            current_y += header_height;
        }

        let mut response = ui.allocate_response(drawer_rect.size(), Sense::hover());
        let interactive = *self.open || self.variant == DrawerVariant::Permanent;
        let mut ids = Vec::new();

        let mut groups: Vec<(Option<&String>, &[DrawerItem], &str)> = Vec::new();
        if self.sections.is_empty() {
            groups.push((None, &self.items, "item"));
        } else {
            for section in &self.sections {
                groups.push((section.label.as_ref(), &section.items, "section"));
            }
        }
        let group_count = groups.len();
        for (group_index, (label, items, salt)) in groups.into_iter().enumerate() {
            if let Some(label) = label {
                // `NavigationDrawerTokens.HeadlineFont` / `HeadlineColor`.
                current_y += 16.0;
                let color = theme.color(ColorRole::OnSurfaceVariant);
                let galley = text_galley(&painter, TypeRole::TitleSmall, label, color);
                let (x, _) = span(HEADLINE_INSET, galley.size().x);
                painter.galley(pos2(x, current_y), galley, color);
                current_y += 20.0;
            }
            for (index, item) in items.iter().enumerate() {
                let item_id = if salt == "section" {
                    self.id.with("section").with(group_index).with(index)
                } else {
                    self.id.with("item").with(index)
                };
                let (min_x, max_x) = span(ITEM_INSET, self.width - ITEM_INSET * 2.0);
                let item_rect = Rect::from_min_max(
                    pos2(min_x, current_y),
                    pos2(max_x, current_y + ITEM_HEIGHT),
                );
                let item_response =
                    self.render_navigation_item(ui, &theme, item, item_rect, item_id, interactive);
                if interactive {
                    ids.push((item_id, item.enabled));
                }
                response = response.union(item_response);
                current_y += ITEM_HEIGHT;
            }
            if group_index + 1 < group_count {
                current_y += 8.0;
                let (min_x, max_x) = span(HEADLINE_INSET, self.width - HEADLINE_INSET * 2.0);
                painter.line_segment(
                    [pos2(min_x, current_y), pos2(max_x, current_y)],
                    Stroke::new(1.0, theme.color(ColorRole::OutlineVariant)),
                );
                current_y += 8.0;
            }
        }
        rove_focus(ui, &ids, RoveAxis::Vertical, rtl);

        response
    }

    /// One destination: the 56 dp `CornerFull` indicator when active, the
    /// state layer, the 24 dp icon, the `LabelLarge` label and the badge
    /// text (`NavigationDrawerTokens`; padding from `NavigationDrawerItem`:
    /// 16 dp start, 12 dp after the icon and before the badge, 24 dp end).
    fn render_navigation_item(
        &self,
        ui: &mut Ui,
        theme: &MaterialTheme,
        item: &DrawerItem,
        item_rect: Rect,
        item_id: Id,
        interactive: bool,
    ) -> Response {
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let sense = if interactive {
            crate::a11y::sense(item.enabled, Sense::click())
        } else {
            Sense::hover()
        };
        let item_response = ui.interact(item_rect, item_id, sense);
        describe_item(&item_response, item.enabled, &item.text, item.active);

        let painter = ui.painter();
        let pill = CornerRadius::same((ITEM_HEIGHT / 2.0) as u8);
        if item.active {
            painter.rect_filled(item_rect, pill, theme.color(ColorRole::SecondaryContainer));
        }
        let content_color = match (item.enabled, item.active) {
            (false, _) => theme
                .color(ColorRole::OnSurface)
                .gamma_multiply(DISABLED_CONTENT_OPACITY),
            (true, true) => theme.color(ColorRole::OnSecondaryContainer),
            (true, false) => theme.color(ColorRole::OnSurfaceVariant),
        };
        // Hovered, focused and pressed labels are onSurface when inactive
        // (`InactiveHoverLabelTextColor` etc.); the state layer follows.
        let layer_color = if item.active {
            theme.color(ColorRole::OnSecondaryContainer)
        } else {
            theme.color(ColorRole::OnSurface)
        };
        crate::paint::paint_state_layer(
            painter,
            item_rect,
            pill,
            layer_color,
            InteractionState::from_response(&item_response),
        );

        // x of a point `from_start` in from the item's start edge.
        let at = |from_start: f32| {
            if rtl {
                item_rect.max.x - from_start
            } else {
                item_rect.min.x + from_start
            }
        };
        let center_y = item_rect.center().y;
        let mut label_start = 16.0;
        if let Some(icon) = &item.icon {
            paint_icon(
                painter,
                pos2(at(16.0 + ICON_SIZE / 2.0), center_y),
                icon,
                ICON_SIZE,
                content_color,
                item.active,
            );
            label_start += ICON_SIZE + 12.0;
        }
        let label = text_galley(painter, TypeRole::LabelLarge, &item.text, content_color);
        let label_x = if rtl {
            at(label_start) - label.size().x
        } else {
            at(label_start)
        };
        painter.galley(
            pos2(label_x, center_y - label.size().y / 2.0),
            label,
            content_color,
        );
        if let Some(badge) = &item.badge {
            // `LargeBadgeLabelFont`: the badge is a trailing label.
            let galley = text_galley(painter, TypeRole::LabelLarge, badge, content_color);
            let x = if rtl {
                at(24.0)
            } else {
                at(24.0) - galley.size().x
            };
            painter.galley(
                pos2(x, center_y - galley.size().y / 2.0),
                galley,
                content_color,
            );
        }
        if item_response.has_focus() {
            crate::paint::paint_focus_ring(
                painter,
                item_rect,
                pill,
                theme.color(ColorRole::Secondary),
            );
        }

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
        self.render_drawer_content(ui, 0.0)
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

#[cfg(test)]
mod modal_tests {
    use super::*;

    /// Regression test for nikescar/egui-material3#14: the modal drawer's
    /// scrim sits above the page, so a click outside the drawer dismisses
    /// it and does not reach the page underneath.
    #[test]
    fn test_modal_scrim_blocks_page_and_dismisses() {
        let ctx = egui::Context::default();
        let mut open = true;
        let mut page_clicked = false;
        let frame = |events: Vec<egui::Event>, open: &mut bool, page_clicked: &mut bool| {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            ctx.run_ui(input, |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    let size = ui.available_size();
                    if ui.add_sized(size, egui::Button::new("page")).clicked() {
                        *page_clicked = true;
                    }
                });
                egui::Window::new("window")
                    .fixed_pos(pos2(400.0, 200.0))
                    .show(ui.ctx(), |ui| {
                        let button = egui::Button::new("window button");
                        if ui.add_sized(Vec2::new(200.0, 200.0), button).clicked() {
                            *page_clicked = true;
                        }
                    });
                MaterialDrawer::new(DrawerVariant::Modal, open)
                    .width(200.0)
                    .show(ui);
            })
            .textures_delta
            .clear();
        };
        let pos = pos2(500.0, 300.0);
        let button = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        // Two frames: a new area is invisible on its first frame.
        frame(Vec::new(), &mut open, &mut page_clicked);
        frame(Vec::new(), &mut open, &mut page_clicked);
        frame(
            vec![egui::Event::PointerMoved(pos), button(true)],
            &mut open,
            &mut page_clicked,
        );
        frame(vec![button(false)], &mut open, &mut page_clicked);
        frame(Vec::new(), &mut open, &mut page_clicked);
        assert!(
            !page_clicked,
            "the page and windows under a modal drawer must not get the click"
        );
        assert!(!open, "clicking the scrim should dismiss the drawer");
    }
}
