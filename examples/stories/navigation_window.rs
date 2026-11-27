// ============================================================================
// Module:       stories::navigation_window (example)
// Description:  Story for the navigation bar, rail, drawer and tabs.
//
// Dependencies: eframe, egui_material3
// ============================================================================

//! The navigation components side by side: bar variants, the rail
//! collapsed, expanded and modal, tab layouts with badges and a disabled
//! tab, and a modal drawer at either edge, with a right-to-left switch.

#![doc(hidden)]

use crate::{
    DrawerAlignment, DrawerItem, DrawerVariant, MaterialButton, MaterialDrawer, MaterialTabs,
    MaterialTheme, NavigationBadge, NavigationBar, NavigationBarVariant, NavigationItem,
    NavigationRail, TabItem, TabLayout,
};
use eframe::egui::{self, Window};
use egui_material3::tokens::LayoutDirection;

#[doc(hidden)]
pub struct NavigationWindow {
    pub open: bool,
    bar_selected: usize,
    bar_variant: NavigationBarVariant,
    rail_selected: usize,
    rail_expanded: bool,
    rail_modal: bool,
    tab_selected: usize,
    tab_layout: TabLayout,
    tab_transparent: bool,
    secondary_selected: usize,
    drawer_open: bool,
    drawer_end: bool,
    right_to_left: bool,
}

impl Default for NavigationWindow {
    fn default() -> Self {
        Self {
            open: false,
            bar_selected: 0,
            bar_variant: NavigationBarVariant::Baseline,
            rail_selected: 0,
            rail_expanded: false,
            rail_modal: false,
            tab_selected: 0,
            tab_layout: TabLayout::Fixed,
            tab_transparent: false,
            secondary_selected: 0,
            drawer_open: false,
            drawer_end: false,
            right_to_left: false,
        }
    }
}

fn destinations() -> Vec<NavigationItem> {
    vec![
        NavigationItem::new("inbox", "Inbox").badge(NavigationBadge::Large("24".into())),
        NavigationItem::new("star", "Starred"),
        NavigationItem::new("send", "Sent").badge(NavigationBadge::Small),
        NavigationItem::new("delete", "Trash").enabled(false),
    ]
}

impl NavigationWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Navigation Stories")
            .open(&mut open)
            .default_size([900.0, 720.0])
            .show(ctx, |ui| {
                self.controls(ui);
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.bars(ui);
                    ui.add_space(16.0);
                    self.tabs(ui);
                    ui.add_space(16.0);
                    self.rail(ui);
                });
                self.drawer(ui);
            });
        self.open = open;
    }

    fn drawer(&mut self, ui: &mut egui::Ui) {
        MaterialDrawer::new(DrawerVariant::Modal, &mut self.drawer_open)
            .header("Mail", Some("all destinations"))
            .alignment(if self.drawer_end {
                DrawerAlignment::End
            } else {
                DrawerAlignment::Start
            })
            .section(
                Some("Mail"),
                vec![
                    DrawerItem::new("Inbox")
                        .icon("inbox")
                        .active(true)
                        .badge("24"),
                    DrawerItem::new("Outbox").icon("send"),
                    DrawerItem::new("Favorites").icon("favorite"),
                    DrawerItem::new("Trash").icon("delete").enabled(false),
                ],
            )
            .section(
                Some("Labels"),
                vec![
                    DrawerItem::new("Family").icon("bookmark"),
                    DrawerItem::new("Work").icon("bookmark"),
                ],
            )
            .show(ui);
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui
                .checkbox(&mut self.right_to_left, "Right to left")
                .changed()
            {
                if self.right_to_left {
                    let theme = MaterialTheme {
                        layout_direction: LayoutDirection::Rtl,
                        ..(*MaterialTheme::of(ui.ctx())).clone()
                    };
                    MaterialTheme::set(ui.ctx(), theme);
                } else {
                    MaterialTheme::clear(ui.ctx());
                }
            }
            ui.checkbox(&mut self.drawer_end, "Drawer at end");
            if ui.add(MaterialButton::filled("Open drawer")).clicked() {
                self.drawer_open = true;
            }
        });
    }

    fn bars(&mut self, ui: &mut egui::Ui) {
        ui.heading("Navigation bar");
        ui.horizontal(|ui| {
            for (variant, name) in [
                (NavigationBarVariant::Baseline, "Baseline (80 dp)"),
                (NavigationBarVariant::Flexible, "Flexible (64 dp)"),
                (
                    NavigationBarVariant::FlexibleHorizontal,
                    "Flexible, horizontal items",
                ),
            ] {
                ui.radio_value(&mut self.bar_variant, variant, name);
            }
        });
        let mut bar = NavigationBar::new(&mut self.bar_selected).variant(self.bar_variant);
        for item in destinations() {
            bar = bar.item(item);
        }
        ui.add(bar);
    }

    fn tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Tabs");
        ui.horizontal(|ui| {
            for (layout, name) in [
                (TabLayout::Fixed, "Fixed"),
                (TabLayout::Content, "Sized to labels"),
                (TabLayout::Scrollable, "Scrollable"),
            ] {
                ui.radio_value(&mut self.tab_layout, layout, name);
            }
            ui.checkbox(&mut self.tab_transparent, "Transparent");
        });
        let mut tabs = MaterialTabs::primary(&mut self.tab_selected)
            .id_salt("navigation_story_primary")
            .layout(self.tab_layout)
            .item(TabItem::new("Flights").icon("flight"))
            .item(
                TabItem::new("Trips")
                    .icon("luggage")
                    .badge(NavigationBadge::Small),
            )
            .item(TabItem::new("Explore").icon("explore"))
            .item(TabItem::new("Offline").icon("cloud_off").enabled(false))
            .item(TabItem::new("Saved").icon("bookmark"))
            .item(TabItem::new("Settings").icon("settings"));
        if self.tab_transparent {
            tabs = tabs.transparent();
        }
        ui.add(tabs);
        ui.add_space(8.0);
        let mut tabs = MaterialTabs::secondary(&mut self.secondary_selected)
            .id_salt("navigation_story_secondary")
            .layout(self.tab_layout)
            .tab("Overview")
            .tab("Specifications")
            .item(TabItem::new("Reviews").badge(NavigationBadge::Large("12".into())));
        if self.tab_transparent {
            tabs = tabs.transparent();
        }
        ui.add(tabs);
    }

    fn rail(&mut self, ui: &mut egui::Ui) {
        ui.heading("Navigation rail");
        ui.checkbox(&mut self.rail_modal, "Modal when expanded");
        ui.allocate_ui(egui::vec2(ui.available_width(), 420.0), |ui| {
            ui.horizontal(|ui| {
                let toggle = &mut self.rail_expanded;
                let mut menu_clicked = false;
                let mut rail = NavigationRail::new(&mut self.rail_selected)
                    .modal(self.rail_modal)
                    .header(|ui| {
                        let icon = crate::material_symbol::material_symbol_text("menu");
                        if ui.button(icon).clicked() {
                            menu_clicked = true;
                        }
                    });
                for item in destinations() {
                    rail = rail.item(item);
                }
                ui.add(rail.expanded(toggle));
                if menu_clicked {
                    self.rail_expanded = !self.rail_expanded;
                }
                ui.label("Page content");
            });
        });
    }
}
