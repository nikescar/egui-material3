#![doc(hidden)]

use crate::{assist_chip, filter_chip, image_utils, input_chip, suggestion_chip, MaterialButton, MaterialCheckbox};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct ChipsWindow {
    pub open: bool,
    label: String,
    elevated: bool,
    disabled: bool,
    // Chip states
    filter_selected: bool,
    filter_disabled_selected: bool,
    filter_icon_selected: bool,
    filter_removable_selected: bool,
    filter_elevated_selected: bool,
}

impl Default for ChipsWindow {
    fn default() -> Self {
        Self {
            open: false,
            label: String::new(),
            elevated: false,
            disabled: false,
            filter_selected: true,
            filter_disabled_selected: true,
            filter_icon_selected: false,
            filter_removable_selected: true,
            filter_elevated_selected: false,
        }
    }
}

impl ChipsWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Chips Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_chips(ui);
                    ui.add_space(20.0);
                    self.render_assist_chips(ui);
                    ui.add_space(20.0);
                    self.render_filter_chips(ui);
                    ui.add_space(20.0);
                    self.render_input_chips(ui);
                    ui.add_space(20.0);
                    self.render_suggestion_chips(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Chips Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/chip/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);
        });

        ui.add(MaterialCheckbox::new(&mut self.elevated, "Elevated"));
        ui.add(MaterialCheckbox::new(&mut self.disabled, "Disabled"));
    }

    fn render_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Chips");

        let label = self.label.clone();
        let elevated = self.elevated;
        let disabled = self.disabled;

        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        // Row 1: Enabled chips (matches Flutter component_screen.dart)
        ui.horizontal_wrapped(|ui| {
            let mut chip = assist_chip(l("Assist"))
                .leading_icon(image_utils::material_icons::EVENT);
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip.on_click(|| println!("Assist chip clicked!")));
            ui.add_space(8.0);

            let mut chip = filter_chip(l("Filter"), &mut self.filter_selected);
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let mut chip = input_chip(l("Input")).removable(true);
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip.on_click(|| println!("Input chip clicked!")));
            ui.add_space(8.0);

            let mut chip = suggestion_chip(l("Suggestion"));
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip.on_click(|| println!("Suggestion chip clicked!")));
        });

        ui.add_space(12.0);

        // Row 2: Disabled chips
        ui.horizontal_wrapped(|ui| {
            let mut chip = assist_chip(l("Assist"))
                .leading_icon(image_utils::material_icons::EVENT)
                .enabled(false);
            if elevated {
                chip = chip.elevated(true);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let mut chip =
                filter_chip(l("Filter"), &mut self.filter_disabled_selected).enabled(false);
            if elevated {
                chip = chip.elevated(true);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let mut chip = input_chip(l("Input")).removable(true).enabled(false);
            if elevated {
                chip = chip.elevated(true);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let chip = suggestion_chip(l("Suggestion")).enabled(false);
            ui.add(chip);
        });
    }

    fn render_assist_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Assist Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Plain assist chip
            ui.add(assist_chip(l("Assist chip")));
            ui.add_space(8.0);

            // With material icon
            ui.add(
                assist_chip(l("With icon"))
                    .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
            );
            ui.add_space(8.0);

            // With SVG icon (Google logo)
            ui.add(
                assist_chip(l("Assist link"))
                    .leading_icon_svg(image_utils::GOOGLE_LOGO_SVG),
            );
            ui.add_space(8.0);

            // Elevated
            ui.add(assist_chip(l("Elevated")).elevated(true));
            ui.add_space(8.0);

            // Soft-disabled
            ui.add(assist_chip(l("Soft-disabled")).soft_disabled(true));
        });
    }

    fn render_filter_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Filter Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Basic filter chip
            ui.add(filter_chip(l("Filter chip"), &mut self.filter_icon_selected));
            ui.add_space(8.0);

            // With leading icon
            ui.add(
                filter_chip(l("With icon"), &mut self.filter_elevated_selected)
                    .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
            );
            ui.add_space(8.0);

            // Removable filter chip
            ui.add(
                filter_chip(l("Removable"), &mut self.filter_removable_selected).removable(true),
            );
            ui.add_space(8.0);

            // Elevated filter chip
            let mut elevated_selected = false;
            ui.add(filter_chip(l("Elevated"), &mut elevated_selected).elevated(true));
            ui.add_space(8.0);

            // Soft-disabled filter chip
            let mut soft_selected = true;
            ui.add(filter_chip(l("Soft-disabled"), &mut soft_selected).soft_disabled(true));
        });
    }

    fn render_input_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Plain input chip
            ui.add(input_chip(l("Input chip")));
            ui.add_space(8.0);

            // With material icon
            ui.add(
                input_chip(l("With icon"))
                    .leading_icon(image_utils::material_icons::ACCOUNT_CIRCLE),
            );
            ui.add_space(8.0);

            // With avatar (SVG) - uses avatar styling
            ui.add(
                input_chip(l("With avatar"))
                    .leading_icon_svg(image_utils::AVATAR_SVG)
                    .avatar(true),
            );
            ui.add_space(8.0);

            // With Google logo link
            ui.add(
                input_chip(l("Input link")).leading_icon_svg(image_utils::GOOGLE_LOGO_SVG),
            );
            ui.add_space(8.0);

            // Removable
            ui.add(input_chip(l("Removable")).removable(true));
            ui.add_space(8.0);

            // Soft-disabled
            ui.add(input_chip(l("Soft-disabled")).soft_disabled(true));
        });
    }

    fn render_suggestion_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Suggestion Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Plain suggestion chip
            ui.add(suggestion_chip(l("Suggestion chip")));
            ui.add_space(8.0);

            // With icon
            ui.add(
                suggestion_chip(l("With icon"))
                    .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
            );
            ui.add_space(8.0);

            // With SVG icon
            ui.add(
                suggestion_chip(l("Suggestion link"))
                    .leading_icon_svg(image_utils::GOOGLE_LOGO_SVG),
            );
            ui.add_space(8.0);

            // Elevated
            ui.add(suggestion_chip(l("Elevated")).elevated(true));
            ui.add_space(8.0);

            // Soft-disabled
            ui.add(suggestion_chip(l("Soft-disabled")).soft_disabled(true));
        });
    }
}
