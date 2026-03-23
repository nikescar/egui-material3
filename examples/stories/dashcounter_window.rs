#![doc(hidden)]

use crate::MaterialButton;
use eframe::egui::{self, Window};
use egui_material3::dashcounter;

#[doc(hidden)]
pub struct DashCounterWindow {
    pub open: bool,
    scroll_offset_1: f32,
    scroll_offset_2: f32,
    scroll_offset_3: f32,
    scroll_offset_4: f32,
    scroll_offset_5: f32,
    scroll_offset_6: f32,
    clicked_message: String,
}

impl Default for DashCounterWindow {
    fn default() -> Self {
        Self {
            open: false,
            scroll_offset_1: 0.0,
            scroll_offset_2: 0.0,
            scroll_offset_3: 0.0,
            scroll_offset_4: 0.0,
            scroll_offset_5: 0.0,
            scroll_offset_6: 0.0,
            clicked_message: String::new(),
        }
    }
}

impl DashCounterWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Dashboard Counter Stories")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_intro(ui);
                    ui.add_space(20.0);
                    self.render_basic_counter(ui);
                    ui.add_space(20.0);
                    self.render_interactive_counter(ui);
                    ui.add_space(20.0);
                    self.render_many_cards(ui);
                    ui.add_space(20.0);
                    self.render_colored_counters(ui);
                });
            });
        self.open = open;
    }

    fn render_intro(&mut self, ui: &mut egui::Ui) {
        ui.push_id("dashcounter_intro", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Dashboard Counter Stories");

                if ui
                    .add(MaterialButton::filled("Material 3 Docs").small())
                    .clicked()
                {
                    let _ = webbrowser::open("https://m3.material.io/components/cards/overview");
                }
            });

            ui.add_space(8.0);
            ui.label("Dashboard Counter - displays a title and horizontally scrollable counter cards.");
            ui.label("Each card shows a category name and counters in xx/yy format.");
            ui.label("Click on counter numbers to trigger events.");
        });
    }

    fn render_basic_counter(&mut self, ui: &mut egui::Ui) {
        ui.push_id("basic_counter", |ui| {
            ui.heading("Basic Dashboard Counter");
            ui.label("Simple counter with a few cards");
            ui.add_space(10.0);

            ui.add(
                dashcounter("System Overview", &mut self.scroll_offset_1)
                    .id_salt("basic")
                    .card("Apps", 5, 10)
                    .card("Updates", 2, 8)
                    .card("Alerts", 0, 3)
                    .card("Storage", 45, 100)
            );

            ui.add_space(10.0);
            ui.label(format!(
                "Scroll offset: {:.0}px",
                self.scroll_offset_1
            ));
        });
    }

    fn render_interactive_counter(&mut self, ui: &mut egui::Ui) {
        ui.push_id("interactive_counter", |ui| {
            ui.heading("Interactive Dashboard Counter with Descriptions");
            ui.label("Click on counter numbers to trigger events. Cards show separate descriptions below each number.");
            ui.add_space(10.0);

            let clicked_msg = self.clicked_message.clone();

            ui.add(
                dashcounter("Application Stats", &mut self.scroll_offset_2)
                    .id_salt("interactive")
                    .card_with_description("Installed", 25, 50, "current", "total")
                    .card_with_description("System", 18, 30, "active", "all")
                    .card_with_description("Bloat", 12, 25, "found", "known")
                    .card("Updates", 5, 15)
                    .card_with_description("Disabled", 8, 20, "disabled", "installed")
                    .on_click({
                        let categories = vec!["Installed", "System", "Bloat", "Updates", "Disabled"];
                        move |index| {
                            if index < categories.len() {
                                println!("Clicked on card: {}", categories[index]);
                            }
                        }
                    })
            );

            ui.add_space(10.0);
            if !clicked_msg.is_empty() {
                ui.colored_label(egui::Color32::GREEN, &clicked_msg);
            } else {
                ui.label("Click on any counter to see the event");
            }
        });
    }

    fn render_many_cards(&mut self, ui: &mut egui::Ui) {
        ui.push_id("many_cards", |ui| {
            ui.heading("Many Cards with Horizontal Scroll");
            ui.label("Dashboard with 15 cards - scroll horizontally to view all");
            ui.add_space(10.0);

            let mut counter_widget = dashcounter("Complete Statistics", &mut self.scroll_offset_3)
                .id_salt("many")
                .card_width(160.0)
                .height(100.0);

            // Add many cards to demonstrate scrolling
            let categories = [
                ("Total Apps", 120, 200),
                ("System Apps", 45, 80),
                ("User Apps", 75, 120),
                ("Bloatware", 30, 50),
                ("Updates Avail", 8, 120),
                ("Running", 25, 120),
                ("Cached", 40, 120),
                ("Disabled", 15, 120),
                ("Hidden", 5, 120),
                ("Permissions", 250, 500),
                ("Storage Used", 65, 100),
                ("Battery Drain", 12, 120),
                ("Network", 18, 120),
                ("Notifications", 42, 200),
                ("Services", 35, 80),
            ];

            for (name, sub, total) in categories.iter() {
                counter_widget = counter_widget.card(*name, *sub, *total);
            }

            ui.add(counter_widget);

            ui.add_space(10.0);
            ui.label("Use mouse wheel or drag to scroll horizontally");
        });
    }

    fn render_colored_counters(&mut self, ui: &mut egui::Ui) {
        ui.push_id("colored_counters", |ui| {
            ui.heading("Custom Color Variations");
            ui.label("Dashboard counters with different color schemes to showcase customization options.");
            ui.add_space(10.0);

            // Per-card colors example
            ui.label("🎨 Per-Card Colors (Mixed)");
            ui.add(
                dashcounter("Multi-Color Dashboard", &mut 0.0f32)
                    .id_salt("mixed")
                    .card_colored(
                        "Success",
                        45, 50,
                        egui::Color32::from_rgb(76, 175, 80),
                        egui::Color32::from_rgb(27, 94, 32),
                        egui::Color32::from_rgb(129, 199, 132),
                    )
                    .card_colored(
                        "Warning",
                        75, 100,
                        egui::Color32::from_rgb(255, 152, 0),
                        egui::Color32::from_rgb(230, 81, 0),
                        egui::Color32::from_rgb(255, 183, 77),
                    )
                    .card_colored(
                        "Error",
                        8, 50,
                        egui::Color32::from_rgb(244, 67, 54),
                        egui::Color32::from_rgb(183, 28, 28),
                        egui::Color32::from_rgb(239, 154, 154),
                    )
                    .card_with_description_colored(
                        "Info",
                        12, 20,
                        "active", "total",
                        egui::Color32::from_rgb(33, 150, 243),
                        egui::Color32::from_rgb(13, 71, 161),
                        egui::Color32::from_rgb(144, 202, 249),
                    )
            );

            ui.add_space(15.0);

            // Success/Green theme
            ui.label("✅ Success Theme (Green)");
            ui.add(
                dashcounter("Success Metrics", &mut self.scroll_offset_4)
                    .id_salt("success")
                    .card_with_description("Passed", 45, 50, "tests", "total")
                    .card_with_description("Active", 98, 100, "users", "max")
                    .card("Uptime", 99, 100)
                    .category_color(egui::Color32::from_rgb(76, 175, 80))
                    .counter_color(egui::Color32::from_rgb(27, 94, 32))
                    .description_color(egui::Color32::from_rgb(129, 199, 132))
            );

            ui.add_space(15.0);

            // Warning/Orange theme
            ui.label("⚠️ Warning Theme (Orange)");
            ui.add(
                dashcounter("Warning Indicators", &mut self.scroll_offset_5)
                    .id_salt("warning")
                    .card_with_description("Pending", 12, 20, "items", "queue")
                    .card_with_description("Memory", 75, 100, "used", "total")
                    .card("Alerts", 3, 10)
                    .category_color(egui::Color32::from_rgb(255, 152, 0))
                    .counter_color(egui::Color32::from_rgb(230, 81, 0))
                    .description_color(egui::Color32::from_rgb(255, 183, 77))
            );

            ui.add_space(15.0);

            // Error/Red theme
            ui.label("❌ Error Theme (Red)");
            ui.add(
                dashcounter("Critical Issues", &mut self.scroll_offset_6)
                    .id_salt("error")
                    .card_with_description("Failed", 8, 50, "critical", "total")
                    .card_with_description("Offline", 5, 25, "servers", "cluster")
                    .card("Errors", 15, 100)
                    .category_color(egui::Color32::from_rgb(244, 67, 54))
                    .counter_color(egui::Color32::from_rgb(183, 28, 28))
                    .description_color(egui::Color32::from_rgb(239, 154, 154))
            );

            ui.add_space(10.0);
            ui.label("💡 Tip: Use different color schemes to indicate status (success, warning, error)");
        });
    }
}
