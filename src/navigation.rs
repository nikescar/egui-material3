//! Nala / Leo navigation component
//!
//! Implements Leo `navigation.svelte`, `navigationItem.svelte`, and related layout.

use crate::{
    get_global_color,
    material_symbol::material_symbol_text,
    theme::{get_global_theme, ThemeMode},
};
use egui::{
    self, epaint::CornerRadius, Align, Color32, FontId, Id, Pos2, Rect, Response, Sense, Shadow,
    Stroke, TextStyle, Ui, Vec2, Widget,
};
use std::cell::Cell;

const ITEM_HEIGHT: f32 = 48.0;
const ITEM_PADDING_LEFT: f32 = 24.0;
const ITEM_PADDING_RIGHT: f32 = 12.0;
const ITEM_ICON_GAP: f32 = 24.0;
const ICON_SIZE: f32 = 18.0;
const MENU_PADDING_Y: f32 = 24.0;
const HEADER_PADDING_X: f32 = 20.0;
const HEADER_PADDING_Y: f32 = 24.0;
const HEADER_GAP: f32 = 12.0;
const ACTIONS_PADDING_Y: f32 = 24.0;
const SUBNAV_INDENT: f32 = 33.0;
const INDICATOR_WIDTH: f32 = 4.0;
const INDICATOR_INSET: f32 = 12.0;
const INDICATOR_RADIUS: f32 = 2.0;

/// Vertical sidebar or horizontal nav bar (Leo `kind`)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NavigationKind {
    #[default]
    Vertical,
    Horizontal,
}

/// A navigable item (Leo `NavigationItem`)
#[derive(Clone, Debug, Default)]
pub struct NavigationItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub children: Vec<NavigationItem>,
    pub enabled: bool,
}

impl NavigationItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            children: Vec::new(),
            enabled: true,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn child(mut self, item: NavigationItem) -> Self {
        self.children.push(item);
        self
    }

    pub fn contains_id(&self, id: &str) -> bool {
        self.id == id || self.children.iter().any(|child| child.contains_id(id))
    }
}

/// Menu row or divider (Leo `NavigationMenu` + `Hr`)
#[derive(Clone, Debug)]
pub enum NavigationEntry {
    Item(NavigationItem),
    Divider,
}

/// Header block (Leo `NavigationHeader`)
#[derive(Clone, Debug, Default)]
pub struct NavigationHeader {
    pub title: String,
    /// Prefix shown in secondary color (e.g. "Brave" in "Brave Accounts")
    pub highlight: Option<String>,
    pub icon: Option<String>,
}

/// Nala navigation panel (Leo `Navigation`)
pub struct MaterialNavigation<'a> {
    selected: &'a mut String,
    kind: NavigationKind,
    width: f32,
    entries: Vec<NavigationEntry>,
    header: Option<NavigationHeader>,
    actions: Vec<NavigationItem>,
}

impl<'a> MaterialNavigation<'a> {
    pub fn new(selected: &'a mut String) -> Self {
        Self {
            selected,
            kind: NavigationKind::Vertical,
            width: 300.0,
            entries: Vec::new(),
            header: None,
            actions: Vec::new(),
        }
    }

    pub fn vertical(mut self) -> Self {
        self.kind = NavigationKind::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.kind = NavigationKind::Horizontal;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn header(mut self, header: NavigationHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn item(mut self, item: NavigationItem) -> Self {
        self.entries.push(NavigationEntry::Item(item));
        self
    }

    pub fn divider(mut self) -> Self {
        self.entries.push(NavigationEntry::Divider);
        self
    }

    pub fn action(mut self, item: NavigationItem) -> Self {
        self.actions.push(item);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        self.ui_impl(ui)
    }
}

impl<'a> Widget for MaterialNavigation<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.ui_impl(ui)
    }
}

impl<'a> MaterialNavigation<'a> {
    fn ui_impl(self, ui: &mut Ui) -> Response {
        let nav_id = ui.id().with("nala_navigation");
        let surface = get_global_color("surface");
        let shadow = Shadow {
            offset: [0, 1],
            blur: 4,
            spread: 0,
            color: Color32::from_black_alpha(18),
        };

        let outer_size = Vec2::new(self.width, ui.available_height().max(400.0));
        let (outer_rect, outer_response) = ui.allocate_exact_size(outer_size, Sense::hover());

        if !ui.is_rect_visible(outer_rect) {
            return outer_response;
        }

        let nav_rect = outer_rect.shrink(0.0);
        let shadow_rect = nav_rect.translate(Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32));
        ui.painter()
            .rect_filled(shadow_rect, CornerRadius::same(8), shadow.color);
        ui.painter().rect(
            nav_rect,
            CornerRadius::same(8),
            surface,
            Stroke::NONE,
            egui::epaint::StrokeKind::Outside,
        );

        let selected_rect = Cell::new(None::<Rect>);

        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(nav_rect), |ui| {
            ui.set_width(self.width);

            if let Some(header) = &self.header {
                render_header(ui, header);
            }

            ui.vertical(|ui| {
                let menu_height = ui.available_height()
                    - if self.actions.is_empty() {
                        0.0
                    } else {
                        self.actions.len() as f32 * ITEM_HEIGHT + ACTIONS_PADDING_Y * 2.0 + 1.0
                    };

                ui.allocate_ui_with_layout(
                    Vec2::new(self.width, menu_height.max(100.0)),
                    egui::Layout::top_down(Align::Min),
                    |ui| {
                        ui.add_space(MENU_PADDING_Y);
                        render_entries(
                            ui,
                            &self.entries,
                            self.selected,
                            nav_id,
                            0,
                            false,
                            &selected_rect,
                        );
                        ui.add_space(MENU_PADDING_Y);
                    },
                );

                if !self.actions.is_empty() {
                    render_divider(ui, self.width);
                    ui.add_space(12.0);
                    for action in &self.actions {
                        render_nav_item(
                            ui,
                            action,
                            self.selected,
                            nav_id.with("actions"),
                            0,
                            false,
                            true,
                            &selected_rect,
                        );
                    }
                    ui.add_space(ACTIONS_PADDING_Y);
                }
            });
        });

        if let Some(rect) = selected_rect.get() {
            draw_active_indicator(ui, nav_id, nav_rect, rect);
        }

        outer_response
    }
}

fn is_dark_theme() -> bool {
    get_global_theme()
        .lock()
        .map(|theme| matches!(theme.theme_mode, ThemeMode::Dark))
        .unwrap_or(false)
}

fn nav_text_style() -> TextStyle {
    TextStyle::Name("Button".into())
}

fn render_header(ui: &mut Ui, header: &NavigationHeader) {
    let on_surface = get_global_color("onSurface");
    let on_surface_variant = get_global_color("onSurfaceVariant");
    let primary = get_global_color("primary");

    ui.horizontal(|ui| {
        ui.add_space(HEADER_PADDING_X);
        ui.vertical(|ui| {
            ui.add_space(HEADER_PADDING_Y);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = HEADER_GAP;
                if let Some(icon) = &header.icon {
                    let icon_str = material_symbol_text(icon);
                    ui.label(
                        egui::RichText::new(icon_str)
                            .size(24.0)
                            .color(primary),
                    );
                }
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    if let Some(highlight) = &header.highlight {
                        ui.label(
                            egui::RichText::new(highlight)
                                .size(16.0)
                                .color(on_surface_variant),
                        );
                        ui.label(
                            egui::RichText::new(format!(" {}", header.title))
                                .size(16.0)
                                .strong()
                                .color(on_surface),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(&header.title)
                                .size(16.0)
                                .strong()
                                .color(on_surface),
                        );
                    }
                });
            });
            ui.add_space(HEADER_PADDING_Y);
        });
    });
}

fn render_entries(
    ui: &mut Ui,
    entries: &[NavigationEntry],
    selected: &mut String,
    nav_id: Id,
    depth: u32,
    inside_selected_branch: bool,
    selected_rect: &Cell<Option<Rect>>,
) {
    if depth > 0 {
        let divider_x = ITEM_PADDING_LEFT + (depth as f32 - 1.0) * SUBNAV_INDENT + 8.0;
        let available = ui.available_rect_before_wrap();
        ui.painter().line_segment(
            [
                Pos2::new(available.min.x + divider_x, available.min.y),
                Pos2::new(available.min.x + divider_x, available.max.y),
            ],
            Stroke::new(1.0, get_global_color("outlineVariant")),
        );
        ui.add_space(8.0);
    }

    for entry in entries {
        match entry {
            NavigationEntry::Divider => render_divider(ui, ui.available_width()),
            NavigationEntry::Item(item) => {
                let is_selected = *selected == item.id;
                let show_subnav = !item.children.is_empty() && item.contains_id(selected);

                let response = render_nav_item(
                    ui,
                    item,
                    selected,
                    nav_id.with(&item.id),
                    depth,
                    inside_selected_branch,
                    false,
                    selected_rect,
                );

                if is_selected {
                    selected_rect.set(Some(response.rect));
                }

                if show_subnav {
                    ui.indent(format!("subnav_{}", item.id), |ui| {
                        ui.set_width(ui.available_width() - SUBNAV_INDENT);
                        render_entries(
                            ui,
                            &item
                                .children
                                .iter()
                                .map(|child| NavigationEntry::Item(child.clone()))
                                .collect::<Vec<_>>(),
                            selected,
                            nav_id.with(("sub", &item.id)),
                            depth + 1,
                            is_selected || inside_selected_branch,
                            selected_rect,
                        );
                    });
                }
            }
        }
    }
}

fn render_nav_item(
    ui: &mut Ui,
    item: &NavigationItem,
    selected: &mut String,
    _id: Id,
    depth: u32,
    inside_selected_branch: bool,
    is_action: bool,
    selected_rect: &Cell<Option<Rect>>,
) -> Response {
    let primary = get_global_color("primary");
    let on_surface_variant = get_global_color("onSurfaceVariant");
    let on_surface = get_global_color("onSurface");

    let is_selected = *selected == item.id;
    let use_interactive = is_selected && !inside_selected_branch;

    let text_color = if !item.enabled {
        on_surface.linear_multiply(0.38)
    } else if use_interactive {
        primary
    } else {
        on_surface_variant
    };

    let icon_color = if !item.enabled {
        on_surface_variant.linear_multiply(0.38)
    } else if use_interactive {
        primary
    } else {
        on_surface_variant
    };

    let indent = if is_action {
        0.0
    } else {
        depth as f32 * SUBNAV_INDENT
    };
    let row_width = ui.available_width();
    let (rect, mut response) = ui.allocate_exact_size(
        Vec2::new(row_width, ITEM_HEIGHT),
        if item.enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );

    if response.clicked() && item.enabled {
        *selected = item.id.clone();
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        if response.hovered() && item.enabled {
            let highlight = if is_dark_theme() {
                Color32::from_rgb(70, 70, 73)
            } else {
                Color32::from_rgb(228, 228, 229)
            };
            ui.painter().rect_filled(rect, 0.0, highlight);
        }

        if response.has_focus() && item.enabled {
            ui.painter().rect_stroke(
                rect,
                2.0,
                Stroke::new(2.0, primary.linear_multiply(0.5)),
                egui::epaint::StrokeKind::Outside,
            );
        }

        let mut cursor_x = rect.min.x + ITEM_PADDING_LEFT + indent;
        let center_y = rect.center().y;

        if let Some(icon) = &item.icon {
            let icon_str = material_symbol_text(icon);
            ui.painter().text(
                Pos2::new(cursor_x + ICON_SIZE / 2.0, center_y),
                egui::Align2::CENTER_CENTER,
                icon_str,
                FontId::proportional(ICON_SIZE),
                icon_color,
            );
            cursor_x += ICON_SIZE + ITEM_ICON_GAP;
        }

        let font_id = ui.style().text_styles.get(&nav_text_style()).cloned().unwrap_or_else(|| FontId::new(14.0, egui::FontFamily::Proportional));
        ui.painter().text(
            Pos2::new(cursor_x, center_y),
            egui::Align2::LEFT_CENTER,
            &item.label,
            font_id,
            text_color,
        );
    }

    if is_selected && !is_action {
        selected_rect.set(Some(rect));
    }

    response
}

fn render_divider(ui: &mut Ui, width: f32) {
    let y = ui.cursor().min.y + 6.0;
    let rect = Rect::from_min_size(Pos2::new(ui.cursor().min.x, y), Vec2::new(width, 12.0));
    ui.allocate_rect(rect, Sense::hover());
    ui.painter().line_segment(
        [
            Pos2::new(rect.min.x + ITEM_PADDING_LEFT, rect.center().y),
            Pos2::new(rect.max.x - ITEM_PADDING_RIGHT, rect.center().y),
        ],
        Stroke::new(1.0, get_global_color("outlineVariant")),
    );
}

fn draw_active_indicator(ui: &Ui, nav_id: Id, nav_rect: Rect, item_rect: Rect) {
    let primary = get_global_color("primary");
    let target_top = item_rect.min.y + INDICATOR_INSET;
    let target_bottom = item_rect.max.y - INDICATOR_INSET;

    const INDICATOR_ANIMATION: f32 = 0.12;

    let top = ui.ctx().animate_value_with_time(
        nav_id.with("indicator_top"),
        target_top,
        INDICATOR_ANIMATION,
    );
    let bottom = ui.ctx().animate_value_with_time(
        nav_id.with("indicator_bottom"),
        target_bottom,
        INDICATOR_ANIMATION,
    );

    let indicator = Rect::from_min_max(
        Pos2::new(nav_rect.min.x, top),
        Pos2::new(nav_rect.min.x + INDICATOR_WIDTH, bottom),
    );

    ui.painter().rect_filled(
        indicator,
        CornerRadius {
            nw: 0,
            ne: INDICATOR_RADIUS as u8,
            sw: 0,
            se: INDICATOR_RADIUS as u8,
        },
        primary,
    );
}

pub fn navigation<'a>(selected: &'a mut String) -> MaterialNavigation<'a> {
    MaterialNavigation::new(selected)
}
