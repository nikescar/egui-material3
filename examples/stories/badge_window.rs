#![doc(hidden)]

use crate::{badge, badge_dot, BadgeColor, BadgePosition, BadgeSize, MaterialButton, MaterialIconButton, MaterialFab, FabSize, noto_emoji};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct BadgeWindow {
    pub open: bool,
    // Customization options
    selected_color: BadgeColor,
    selected_size: BadgeSize,
    badge_content: String,
    show_as_dot: bool,
    // Notification counts for demo
    inbox_count: i32,
    calendar_count: i32,
    upload_count: i32,
}

impl Default for BadgeWindow {
    fn default() -> Self {
        Self {
            open: false,
            selected_color: BadgeColor::Error,
            selected_size: BadgeSize::Regular,
            badge_content: "5".to_string(),
            show_as_dot: false,
            inbox_count: 5,
            calendar_count: 7,
            upload_count: 1,
        }
    }
}

impl BadgeWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Badge Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Badge Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://konstaui.com/react/badge");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Badge Text:");
            ui.text_edit_singleline(&mut self.badge_content);
        });

        ui.checkbox(&mut self.show_as_dot, "Show as dot (no text)");

        ui.horizontal(|ui| {
            ui.label("Color:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.selected_color))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_color, BadgeColor::Primary, "Primary");
                    ui.selectable_value(&mut self.selected_color, BadgeColor::Error, "Error (Red)");
                    ui.selectable_value(&mut self.selected_color, BadgeColor::Success, "Success (Green)");
                    ui.selectable_value(&mut self.selected_color, BadgeColor::Warning, "Warning (Yellow)");
                    ui.selectable_value(&mut self.selected_color, BadgeColor::Neutral, "Neutral (Gray)");
                });
        });

        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.radio_value(&mut self.selected_size, BadgeSize::Small, "Small");
            ui.radio_value(&mut self.selected_size, BadgeSize::Regular, "Regular");
            ui.radio_value(&mut self.selected_size, BadgeSize::Large, "Large");
        });
    }

    fn render_examples(&mut self, ui: &mut egui::Ui) {
        // Standalone Badges
        ui.heading("Standalone Badges");
        ui.horizontal_wrapped(|ui| {
            ui.add(badge("5").color(BadgeColor::Error).size(BadgeSize::Small));
            ui.add(badge("NEW").color(BadgeColor::Success).size(BadgeSize::Regular));
            ui.add(badge("CEO").color(BadgeColor::Primary).size(BadgeSize::Regular));
            ui.add(badge("99+").color(BadgeColor::Warning).size(BadgeSize::Regular));
            ui.add(badge("0").color(BadgeColor::Neutral).size(BadgeSize::Regular));
            ui.add(badge_dot().color(BadgeColor::Error).size(BadgeSize::Small));
            ui.add(badge_dot().color(BadgeColor::Success).size(BadgeSize::Regular));
        });

        ui.add_space(20.0);

        // Custom Badge Demo
        ui.heading("Custom Badge");
        ui.horizontal(|ui| {
            let badge_widget = if self.show_as_dot {
                badge_dot()
                    .color(self.selected_color)
                    .size(self.selected_size)
            } else {
                badge(&self.badge_content)
                    .color(self.selected_color)
                    .size(self.selected_size)
            };
            ui.add(badge_widget);
        });

        ui.add_space(20.0);

        // Badges on Icon Buttons
        ui.heading("Badges on Icon Buttons");
        ui.horizontal_wrapped(|ui| {
            // Icon button with badge overlay
            let icon_response = ui.add(MaterialIconButton::standard(noto_emoji::ENVELOPE).size(48.0));
            badge("5")
                .color(BadgeColor::Success)
                .size(BadgeSize::Small)
                .draw_on(ui, icon_response.rect, BadgePosition::TopRight);

            let icon_response = ui.add(MaterialIconButton::filled(noto_emoji::CALENDAR).size(48.0));
            badge("7")
                .color(BadgeColor::Error)
                .size(BadgeSize::Small)
                .draw_on(ui, icon_response.rect, BadgePosition::TopRight);

            let icon_response = ui.add(MaterialIconButton::outlined(noto_emoji::UPWARDS_BLACK_ARROW).size(48.0));
            badge("1")
                .color(BadgeColor::Error)
                .size(BadgeSize::Small)
                .draw_on(ui, icon_response.rect, BadgePosition::TopRight);

            let icon_response = ui.add(MaterialIconButton::filled_tonal(noto_emoji::BELL).size(48.0));
            badge_dot()
                .color(BadgeColor::Error)
                .size(BadgeSize::Small)
                .draw_on(ui, icon_response.rect, BadgePosition::TopRight);
        });

        ui.add_space(20.0);

        // Badges on FABs
        ui.heading("Badges on FABs");
        ui.horizontal_wrapped(|ui| {
            let fab_response = ui.add(
                MaterialFab::primary()
                    .icon("add")
                    .size(FabSize::Regular),
            );
            badge("3")
                .color(BadgeColor::Error)
                .size(BadgeSize::Small)
                .draw_on(ui, fab_response.rect, BadgePosition::TopRight);

            let fab_response = ui.add(
                MaterialFab::secondary()
                    .icon("edit")
                    .size(FabSize::Small),
            );
            badge_dot()
                .color(BadgeColor::Success)
                .size(BadgeSize::Small)
                .draw_on(ui, fab_response.rect, BadgePosition::TopRight);
        });

        ui.add_space(20.0);

        // Badges in List-like Layout
        ui.heading("Badges in Lists");
        ui.group(|ui| {
            ui.set_width(ui.available_width());

            // List item 1
            ui.horizontal(|ui| {
                ui.label("📧 Foo Bar");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(badge("0").color(BadgeColor::Neutral).size(BadgeSize::Regular));
                });
            });
            ui.separator();

            // List item 2
            ui.horizontal(|ui| {
                ui.label("👤 Ivan Petrov");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(badge("CEO").color(BadgeColor::Primary).size(BadgeSize::Regular));
                });
            });
            ui.separator();

            // List item 3
            ui.horizontal(|ui| {
                ui.label("📧 John Doe");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(badge("5").color(BadgeColor::Success).size(BadgeSize::Regular));
                });
            });
            ui.separator();

            // List item 4
            ui.horizontal(|ui| {
                ui.label("📧 Jane Doe");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(badge("NEW").color(BadgeColor::Warning).size(BadgeSize::Regular));
                });
            });
        });

        ui.add_space(20.0);

        // Tab Bar with Badges (simulated)
        ui.heading("Navigation Bar with Badges");
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_width(ui.available_width() - 20.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
                        // Tab 1 - Inbox
                        ui.vertical(|ui| {
                            ui.set_width(100.0);
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                let icon_response = ui.add(MaterialIconButton::standard(noto_emoji::E_MAIL_SYMBOL).size(32.0));
                                badge(&format!("{}", self.inbox_count))
                                    .color(BadgeColor::Success)
                                    .size(BadgeSize::Small)
                                    .draw_on(ui, icon_response.rect, BadgePosition::TopRight);
                                ui.small("Inbox");
                            });
                        });

                        // Tab 2 - Calendar
                        ui.vertical(|ui| {
                            ui.set_width(100.0);
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                let icon_response = ui.add(MaterialIconButton::standard(noto_emoji::CALENDAR).size(32.0));
                                badge(&format!("{}", self.calendar_count))
                                    .color(BadgeColor::Error)
                                    .size(BadgeSize::Small)
                                    .draw_on(ui, icon_response.rect, BadgePosition::TopRight);
                                ui.small("Calendar");
                            });
                        });

                        // Tab 3 - Upload
                        ui.vertical(|ui| {
                            ui.set_width(100.0);
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                let icon_response = ui.add(MaterialIconButton::standard(noto_emoji::UPWARDS_BLACK_ARROW).size(32.0));
                                badge(&format!("{}", self.upload_count))
                                    .color(BadgeColor::Error)
                                    .size(BadgeSize::Small)
                                    .draw_on(ui, icon_response.rect, BadgePosition::TopRight);
                                ui.small("Upload");
                            });
                        });
                    });
                });
            });
        });

        ui.add_space(20.0);

        // All Color Variants
        ui.heading("All Color Variants");
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label("Primary");
                ui.add(badge("5").color(BadgeColor::Primary).size(BadgeSize::Regular));
            });
            ui.vertical(|ui| {
                ui.label("Error");
                ui.add(badge("5").color(BadgeColor::Error).size(BadgeSize::Regular));
            });
            ui.vertical(|ui| {
                ui.label("Success");
                ui.add(badge("5").color(BadgeColor::Success).size(BadgeSize::Regular));
            });
            ui.vertical(|ui| {
                ui.label("Warning");
                ui.add(badge("5").color(BadgeColor::Warning).size(BadgeSize::Regular));
            });
            ui.vertical(|ui| {
                ui.label("Neutral");
                ui.add(badge("5").color(BadgeColor::Neutral).size(BadgeSize::Regular));
            });
        });

        ui.add_space(20.0);

        // All Size Variants
        ui.heading("All Size Variants");
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label("Small");
                ui.add(badge("5").color(BadgeColor::Error).size(BadgeSize::Small));
            });
            ui.vertical(|ui| {
                ui.label("Regular");
                ui.add(badge("5").color(BadgeColor::Error).size(BadgeSize::Regular));
            });
            ui.vertical(|ui| {
                ui.label("Large");
                ui.add(badge("5").color(BadgeColor::Error).size(BadgeSize::Large));
            });
        });

        ui.add_space(20.0);

        // Dot Badges
        ui.heading("Dot Badges (Indicators)");
        ui.horizontal_wrapped(|ui| {
            ui.add(badge_dot().color(BadgeColor::Error).size(BadgeSize::Small));
            ui.add(badge_dot().color(BadgeColor::Success).size(BadgeSize::Small));
            ui.add(badge_dot().color(BadgeColor::Warning).size(BadgeSize::Small));
            ui.add(badge_dot().color(BadgeColor::Primary).size(BadgeSize::Regular));
            ui.add(badge_dot().color(BadgeColor::Neutral).size(BadgeSize::Regular));
        });
    }
}
