#![doc(hidden)]

use crate::{breadcrumbs, MaterialButton};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub struct BreadcrumbsWindow {
    pub open: bool,
    last_clicked: String,
    separator: String,
    font_size: f32,
    spacing: f32,
    show_separator: bool,
}

impl Default for BreadcrumbsWindow {
    fn default() -> Self {
        Self {
            open: false,
            last_clicked: "None".to_string(),
            separator: "/".to_string(),
            font_size: 14.0,
            spacing: 8.0,
            show_separator: true,
        }
    }
}

impl BreadcrumbsWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Breadcrumbs Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_breadcrumbs(ui);
                    ui.add_space(20.0);
                    self.render_custom_separator(ui);
                    ui.add_space(20.0);
                    self.render_custom_styling(ui);
                    ui.add_space(20.0);
                    self.render_long_breadcrumbs(ui);
                    ui.add_space(20.0);
                    self.render_interactive_example(ui);
                    ui.add_space(20.0);
                    self.render_button_variants(ui);
                    ui.add_space(20.0);
                    self.render_no_separator(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Breadcrumbs Controls");
        });

        ui.horizontal(|ui| {
            ui.label("Separator:");
            ui.text_edit_singleline(&mut self.separator);
        });

        ui.horizontal(|ui| {
            ui.label("Font Size:");
            ui.add(egui::Slider::new(&mut self.font_size, 10.0..=24.0));
        });

        ui.horizontal(|ui| {
            ui.label("Spacing:");
            ui.add(egui::Slider::new(&mut self.spacing, 4.0..=20.0));
        });

        ui.checkbox(&mut self.show_separator, "Show Separator");

        ui.horizontal(|ui| {
            ui.label("Last Clicked:");
            ui.label(&self.last_clicked);
        });
    }

    fn render_basic_breadcrumbs(&mut self, ui: &mut Ui) {
        ui.heading("Basic Breadcrumbs");
        ui.label("Simple breadcrumb navigation with small Material buttons:");
        ui.label("Clickable items use text buttons, active item uses filled tonal button");

        ui.add(
            breadcrumbs()
                .item_with_callback("Home", || println!("Home clicked"))
                .item_with_callback("Products", || println!("Products clicked"))
                .item_with_callback("Electronics", || println!("Electronics clicked"))
                .active_item("Laptops"),
        );
    }

    fn render_custom_separator(&mut self, ui: &mut Ui) {
        ui.heading("Custom Separators");
        ui.label("Breadcrumbs with different separator styles:");

        ui.label("Arrow separator (>):");
        ui.add(
            breadcrumbs()
                .separator(">")
                .item("Home")
                .item("Products")
                .active_item("Laptops"),
        );

        ui.add_space(8.0);

        ui.label("Chevron separator (›):");
        ui.add(
            breadcrumbs()
                .separator("›")
                .item("Home")
                .item("Products")
                .active_item("Laptops"),
        );

        ui.add_space(8.0);

        ui.label("Dot separator (•):");
        ui.add(
            breadcrumbs()
                .separator("•")
                .item("Home")
                .item("Products")
                .active_item("Laptops"),
        );
    }

    fn render_custom_styling(&mut self, ui: &mut Ui) {
        ui.heading("Custom Styling");
        ui.label("Breadcrumbs with customizable font size and spacing:");

        ui.add(
            breadcrumbs()
                .font_size(self.font_size)
                .spacing(self.spacing)
                .separator(&self.separator)
                .item("Home")
                .item("Category")
                .active_item("Current"),
        );
    }

    fn render_long_breadcrumbs(&mut self, ui: &mut Ui) {
        ui.heading("Long Breadcrumb Trail");
        ui.label("Breadcrumbs with many levels (scrollable):");

        ui.add(
            breadcrumbs()
                .item("Home")
                .item("Products")
                .item("Electronics")
                .item("Computers")
                .item("Laptops")
                .item("Gaming Laptops")
                .item("High Performance")
                .active_item("RTX 4090"),
        );
    }

    fn render_interactive_example(&mut self, ui: &mut Ui) {
        ui.heading("Interactive Breadcrumbs");
        ui.label("Click on any breadcrumb button to see the interaction:");
        ui.label("Small buttons provide better touch targets and visual feedback");

        ui.add(
            breadcrumbs()
                .item_with_callback("Dashboard", || {
                    println!("Navigating to: Dashboard");
                })
                .item_with_callback("Settings", || {
                    println!("Navigating to: Settings");
                })
                .item_with_callback("User Profile", || {
                    println!("Navigating to: User Profile");
                })
                .active_item("Edit Information"),
        );
    }

    fn render_button_variants(&mut self, ui: &mut Ui) {
        ui.heading("Small Button Variants");
        ui.label("Breadcrumbs using different Material button variants:");

        ui.label("Text buttons (default - low emphasis):");
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::text("Home").small()).clicked() {
                println!("Home clicked");
            }
            ui.label("/");
            if ui.add(MaterialButton::text("Products").small()).clicked() {
                println!("Products clicked");
            }
            ui.label("/");
            if ui.add(MaterialButton::text("Electronics").small()).clicked() {
                println!("Electronics clicked");
            }
            ui.label("/");
            ui.add(MaterialButton::filled_tonal("Laptops").small().enabled(false));
        });

        ui.add_space(8.0);

        ui.label("Outlined buttons (medium emphasis):");
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Home").small()).clicked() {
                println!("Home clicked");
            }
            ui.label("›");
            if ui.add(MaterialButton::outlined("Products").small()).clicked() {
                println!("Products clicked");
            }
            ui.label("›");
            if ui.add(MaterialButton::outlined("Electronics").small()).clicked() {
                println!("Electronics clicked");
            }
            ui.label("›");
            ui.add(MaterialButton::filled("Laptops").small());
        });

        ui.add_space(8.0);

        ui.label("Filled tonal buttons (medium emphasis):");
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled_tonal("Home").small()).clicked() {
                println!("Home clicked");
            }
            ui.label(">");
            if ui.add(MaterialButton::filled_tonal("Products").small()).clicked() {
                println!("Products clicked");
            }
            ui.label(">");
            if ui.add(MaterialButton::filled_tonal("Electronics").small()).clicked() {
                println!("Electronics clicked");
            }
            ui.label(">");
            ui.add(MaterialButton::filled("Laptops").small());
        });

        ui.add_space(8.0);

        ui.label("Elevated buttons (medium emphasis with shadow):");
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::elevated("Home").small()).clicked() {
                println!("Home clicked");
            }
            ui.label("•");
            if ui.add(MaterialButton::elevated("Products").small()).clicked() {
                println!("Products clicked");
            }
            ui.label("•");
            if ui.add(MaterialButton::elevated("Electronics").small()).clicked() {
                println!("Electronics clicked");
            }
            ui.label("•");
            ui.add(MaterialButton::filled("Laptops").small());
        });

        ui.add_space(8.0);

        ui.label("Mixed variants for visual hierarchy:");
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::text("Home").small()).clicked() {
                println!("Home clicked");
            }
            ui.label("/");
            if ui.add(MaterialButton::text("Products").small()).clicked() {
                println!("Products clicked");
            }
            ui.label("/");
            if ui.add(MaterialButton::filled_tonal("Electronics").small()).clicked() {
                println!("Electronics clicked");
            }
            ui.label("/");
            ui.add(MaterialButton::filled("Laptops").small());
        });
    }

    fn render_no_separator(&mut self, ui: &mut Ui) {
        ui.heading("Without Separator");
        ui.label("Breadcrumbs with separators hidden:");

        ui.add(
            breadcrumbs()
                .hide_separator()
                .item("Home")
                .item("Products")
                .item("Electronics")
                .active_item("Laptops"),
        );
    }
}
