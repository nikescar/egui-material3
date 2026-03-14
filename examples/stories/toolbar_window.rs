#![doc(hidden)]

use crate::{
    icon_button_filled, icon_button_standard, toolbar, MaterialButton,
};
use egui_material3::material_symbol::material_symbol_text;
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub struct ToolbarWindow {
    pub open: bool,
    show_outline: bool,
    is_top: bool,
    tabbar_mode: bool,
    min_height: f32,
    item_spacing: f32,
}

impl Default for ToolbarWindow {
    fn default() -> Self {
        Self {
            open: false,
            show_outline: true,
            is_top: false,
            tabbar_mode: false,
            min_height: 56.0,
            item_spacing: 8.0,
        }
    }
}

impl ToolbarWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Toolbar Stories")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_toolbar(ui);
                    ui.add_space(20.0);
                    self.render_toolbar_with_icons(ui);
                    ui.add_space(20.0);
                    self.render_toolbar_with_spacer(ui);
                    ui.add_space(20.0);
                    self.render_bottom_navigation(ui);
                    ui.add_space(20.0);
                    self.render_tabbar_mode(ui);
                    ui.add_space(20.0);
                    self.render_top_toolbar(ui);
                    ui.add_space(20.0);
                    self.render_custom_styling(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Toolbar Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://konstaui.com/react/toolbar");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Min Height:");
            ui.add(egui::Slider::new(&mut self.min_height, 40.0..=80.0));
        });

        ui.horizontal(|ui| {
            ui.label("Item Spacing:");
            ui.add(egui::Slider::new(&mut self.item_spacing, 4.0..=20.0));
        });

        ui.checkbox(&mut self.show_outline, "Show Outline");
        ui.checkbox(&mut self.is_top, "Position at Top");
        ui.checkbox(&mut self.tabbar_mode, "Tabbar Mode");
    }

    fn render_basic_toolbar(&mut self, ui: &mut Ui) {
        ui.heading("Basic Toolbar");
        ui.label("Simple toolbar with text buttons:");

        ui.add(
            toolbar()
                .item(MaterialButton::text("Home"))
                .item(MaterialButton::text("Explore"))
                .item(MaterialButton::text("Library"))
                .min_height(self.min_height)
                .item_spacing(self.item_spacing)
                .outline(self.show_outline),
        );
    }

    fn render_toolbar_with_icons(&mut self, ui: &mut Ui) {
        ui.heading("Toolbar with Icon Buttons");
        ui.label("Toolbar using Material icon buttons:");

        ui.add(
            toolbar()
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("home"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("search"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("favorite"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("settings"))))
                .min_height(self.min_height)
                .item_spacing(self.item_spacing)
                .outline(self.show_outline),
        );
    }

    fn render_toolbar_with_spacer(&mut self, ui: &mut Ui) {
        ui.heading("Toolbar with Spacer");
        ui.label("Use spacer to push items apart:");

        ui.add(
            toolbar()
                .item(MaterialButton::text("Left"))
                .spacer()
                .item(MaterialButton::text("Right"))
                .min_height(self.min_height)
                .item_spacing(self.item_spacing)
                .outline(self.show_outline),
        );

        ui.add_space(8.0);

        ui.label("Multiple spacers:");
        ui.add(
            toolbar()
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("menu"))))
                .spacer()
                .item(MaterialButton::text("Center"))
                .spacer()
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("more_vert"))))
                .min_height(self.min_height)
                .item_spacing(self.item_spacing)
                .outline(self.show_outline),
        );
    }

    fn render_bottom_navigation(&mut self, ui: &mut Ui) {
        ui.heading("Bottom Navigation");
        ui.label("Toolbar with mixed buttons and icons:");

        ui.add(
            toolbar()
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("home"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("explore"))))
                .item_fn(|ui| ui.add(icon_button_filled(&material_symbol_text("add"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("notifications"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("account_circle"))))
                .min_height(self.min_height)
                .item_spacing(self.item_spacing)
                .outline(self.show_outline),
        );
    }

    fn render_tabbar_mode(&mut self, ui: &mut Ui) {
        ui.heading("Tabbar Mode");
        ui.label("Equal-width items for tab navigation:");

        ui.add(
            toolbar()
                .tabbar(true)
                .item(MaterialButton::text("Home").small())
                .item(MaterialButton::text("Explore").small())
                .item(MaterialButton::text("Library").small())
                .item(MaterialButton::text("Profile").small())
                .min_height(self.min_height)
                .outline(self.show_outline),
        );

        ui.add_space(8.0);

        ui.label("Tabbar with icons:");
        ui.add(
            toolbar()
                .tabbar(true)
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("home")).size(32.0)))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("search")).size(32.0)))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("favorite")).size(32.0)))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("person")).size(32.0)))
                .min_height(self.min_height)
                .outline(self.show_outline),
        );
    }

    fn render_top_toolbar(&mut self, ui: &mut Ui) {
        ui.heading("Top Toolbar");
        ui.label("Toolbar positioned at the top with shadow:");

        ui.add(
            toolbar()
                .top(true)
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("menu"))))
                .item(MaterialButton::text("My App").small())
                .spacer()
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("search"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("more_vert"))))
                .min_height(self.min_height)
                .item_spacing(self.item_spacing)
                .outline(self.show_outline),
        );
    }

    fn render_custom_styling(&mut self, ui: &mut Ui) {
        ui.heading("Custom Styling");
        ui.label("Toolbar with custom configuration:");

        ui.add(
            toolbar()
                .item(MaterialButton::filled_tonal("Action 1").small())
                .item(MaterialButton::filled_tonal("Action 2").small())
                .item(MaterialButton::filled_tonal("Action 3").small())
                .spacer()
                .item_fn(|ui| ui.add(icon_button_filled(&material_symbol_text("settings"))))
                .min_height(self.min_height)
                .item_spacing(self.item_spacing)
                .outline(self.show_outline)
                .top(self.is_top),
        );

        ui.add_space(8.0);

        ui.label("No outline:");
        ui.add(
            toolbar()
                .outline(false)
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("home"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("explore"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("library_books"))))
                .item_fn(|ui| ui.add(icon_button_standard(&material_symbol_text("person"))))
                .min_height(64.0)
                .item_spacing(16.0),
        );
    }
}
