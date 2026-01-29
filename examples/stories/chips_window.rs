#![doc(hidden)]

use crate::{assist_chip, filter_chip, image_utils, input_chip, suggestion_chip, MaterialCheckbox};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct ChipsWindow {
    pub open: bool,
    label: String,
    elevated: bool,
    disabled: bool,
    // Chip states
    filter_selected: bool,
    filter_with_icon_selected: bool,
    removable_filter_selected: bool,
    soft_disabled_filter_selected: bool,
}

impl Default for ChipsWindow {
    fn default() -> Self {
        Self {
            open: false,
            label: String::new(),
            elevated: false,
            disabled: false,
            filter_selected: false,
            filter_with_icon_selected: false,
            removable_filter_selected: false,
            soft_disabled_filter_selected: false,
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

            if ui.button("Target").clicked() {
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

    fn render_assist_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Assist Chips");

        ui.horizontal_wrapped(|ui| {
            self.render_assist_chips_content(ui);
        });
    }

    fn render_assist_chips_content(&mut self, ui: &mut egui::Ui) {
        // Basic assist chip
        let label = if self.label.is_empty() {
            "Assist chip"
        } else {
            &self.label
        };
        let mut chip = assist_chip(label);
        if self.disabled {
            chip = chip.enabled(false);
        }
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Assist chip clicked!")));
        ui.add_space(8.0);

        // Assist chip with material icon
        let label = if self.label.is_empty() {
            "Assist chip with icon"
        } else {
            &self.label
        };
        let mut chip =
            assist_chip(label).leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE);
        if self.disabled {
            chip = chip.enabled(false);
        }
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Assist chip with icon clicked!")));
        ui.add_space(8.0);

        // Force new line for problematic chip
        ui.end_row();

        // Assist link chip with Google SVG logo (never disabled)
        let label = if self.label.is_empty() {
            "Assist link chip"
        } else {
            &self.label
        };
        let mut chip = assist_chip(label).leading_icon_svg(image_utils::GOOGLE_LOGO_SVG);
        // Link chips are never disabled
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Assist link chip clicked!")));
        ui.add_space(12.0);

        // Soft-disabled assist chip
        let label = if self.label.is_empty() {
            "Soft-disabled assist chip (focusable)"
        } else {
            &self.label
        };
        let mut chip = assist_chip(label).soft_disabled(true);
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Soft-disabled assist chip clicked!")));
    }

    fn render_filter_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Filter Chips");

        ui.horizontal_wrapped(|ui| {
            self.render_filter_chips_content(ui);
        });
    }

    fn render_filter_chips_content(&mut self, ui: &mut egui::Ui) {
        // Basic filter chip
        let label = if self.label.is_empty() {
            "Filter chip"
        } else {
            &self.label
        };
        let mut chip = filter_chip(label, &mut self.filter_selected);
        if self.disabled {
            chip = chip.enabled(false);
        }
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip);
        ui.add_space(8.0);

        // Filter chip with material icon
        let label = if self.label.is_empty() {
            "Filter chip with icon"
        } else {
            &self.label
        };
        let mut chip = filter_chip(label, &mut self.filter_with_icon_selected)
            .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE);
        if self.disabled {
            chip = chip.enabled(false);
        }
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip);
        ui.add_space(8.0);

        // Force new line for problematic chip
        ui.end_row();

        // Removable filter chip
        let label = if self.label.is_empty() {
            "Removable filter chip"
        } else {
            &self.label
        };
        let mut chip = filter_chip(label, &mut self.removable_filter_selected).removable(true);
        if self.disabled {
            chip = chip.enabled(false);
        }
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip);
        ui.add_space(12.0);

        // Soft-disabled filter chip
        let label = if self.label.is_empty() {
            "Soft-disabled filter chip (focusable)"
        } else {
            &self.label
        };
        let mut chip = filter_chip(label, &mut self.soft_disabled_filter_selected)
            .soft_disabled(true)
            .removable(true);
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip);
    }

    fn render_input_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input Chips");

        ui.horizontal_wrapped(|ui| {
            self.render_input_chips_content(ui);
        });
    }

    fn render_input_chips_content(&mut self, ui: &mut egui::Ui) {
        // Basic input chip
        let label = if self.label.is_empty() {
            "Input chip"
        } else {
            &self.label
        };
        let mut chip = input_chip(label);
        if self.disabled {
            chip = chip.enabled(false);
        }
        ui.add(chip.on_click(|| println!("Input chip clicked!")));
        ui.add_space(8.0);

        // Input chip with material icon
        let label = if self.label.is_empty() {
            "Input chip with icon"
        } else {
            &self.label
        };
        let mut chip =
            input_chip(label).leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE);
        if self.disabled {
            chip = chip.enabled(false);
        }
        ui.add(chip.on_click(|| println!("Input chip with icon clicked!")));
        ui.add_space(8.0);

        // Force new line for problematic chip
        ui.end_row();

        // Input chip with avatar (using SVG) - keep roundy for avatar
        let label = if self.label.is_empty() {
            "Input chip with avatar"
        } else {
            &self.label
        };
        let mut chip = input_chip(label)
            .leading_icon_svg(image_utils::AVATAR_SVG)
            .avatar(true);
        if self.disabled {
            chip = chip.enabled(false);
        }
        ui.add(chip.on_click(|| println!("Input chip with avatar clicked!")));
        ui.add_space(12.0);

        // Input link chip with Google logo (never disabled)
        let label = if self.label.is_empty() {
            "Input link chip"
        } else {
            &self.label
        };
        let chip = input_chip(label).leading_icon_svg(image_utils::GOOGLE_LOGO_SVG);
        // Link chips are never disabled
        ui.add(chip.on_click(|| println!("Input link chip clicked!")));
        ui.add_space(8.0);

        // Remove-only input chip
        let label = if self.label.is_empty() {
            "Remove-only input chip"
        } else {
            &self.label
        };
        let mut chip = input_chip(label).removable(true);
        if self.disabled {
            chip = chip.enabled(false);
        }
        ui.add(chip.on_click(|| println!("Remove-only input chip clicked!")));
        ui.add_space(8.0);

        // Soft-disabled input chip
        let label = if self.label.is_empty() {
            "Soft-disabled input chip (focusable)"
        } else {
            &self.label
        };
        let chip = input_chip(label).soft_disabled(true);
        ui.add(chip.on_click(|| println!("Soft-disabled input chip clicked!")));
    }

    fn render_suggestion_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Suggestion Chips");

        ui.horizontal_wrapped(|ui| {
            self.render_suggestion_chips_content(ui);
        });
    }

    fn render_suggestion_chips_content(&mut self, ui: &mut egui::Ui) {
        // Basic suggestion chip
        let label = if self.label.is_empty() {
            "Suggestion chip"
        } else {
            &self.label
        };
        let mut chip = suggestion_chip(label);
        if self.disabled {
            chip = chip.enabled(false);
        }
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Suggestion chip clicked!")));
        ui.add_space(8.0);

        // Suggestion chip with material icon
        let label = if self.label.is_empty() {
            "Suggestion chip with icon"
        } else {
            &self.label
        };
        let mut chip =
            suggestion_chip(label).leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE);
        if self.disabled {
            chip = chip.enabled(false);
        }
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Suggestion chip with icon clicked!")));
        ui.add_space(8.0);

        // Force new line for problematic chip
        ui.end_row();

        // Suggestion link chip with Google logo (never disabled)
        let label = if self.label.is_empty() {
            "Suggestion link chip"
        } else {
            &self.label
        };
        let mut chip = suggestion_chip(label).leading_icon_svg(image_utils::GOOGLE_LOGO_SVG);
        // Link chips are never disabled
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Suggestion link chip clicked!")));
        ui.add_space(12.0);

        // Soft-disabled suggestion chip
        let label = if self.label.is_empty() {
            "Soft-disabled suggestion chip (focusable)"
        } else {
            &self.label
        };
        let mut chip = suggestion_chip(label).soft_disabled(true);
        if self.elevated {
            chip = chip.elevated(true);
        }
        ui.add(chip.on_click(|| println!("Soft-disabled suggestion chip clicked!")));
    }
}
