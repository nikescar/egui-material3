#![doc(hidden)]

use crate::{checkbox, MaterialCheckbox};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct CheckboxWindow {
    pub open: bool,
    checked: bool,
    disabled: bool,
    indeterminate: bool,
    error: bool,
    // For labeled checkboxes
    option1_checked: bool,
    option2_checked: bool,
    option3_checked: bool,
    // For state demonstration
    state_checked: bool,
    state_unchecked: bool,
    state_indeterminate: bool,
    // For enabled/disabled comparison
    enabled_unchecked: bool,
    enabled_checked: bool,
    enabled_indeterminate: bool,
    disabled_unchecked: bool,
    disabled_checked: bool,
    disabled_indeterminate: bool,
    // For error state examples
    error_normal: bool,
    error_error: bool,
    error_error_checked: bool,
}

impl Default for CheckboxWindow {
    fn default() -> Self {
        Self {
            open: false,
            checked: false,
            disabled: false,
            indeterminate: false,
            error: false,
            option1_checked: true,
            option2_checked: false,
            option3_checked: false,
            state_checked: true,
            state_unchecked: false,
            state_indeterminate: false,
            // Enabled/disabled comparison states
            enabled_unchecked: false,
            enabled_checked: true,
            enabled_indeterminate: false,
            disabled_unchecked: false,
            disabled_checked: true,
            disabled_indeterminate: false,
            // Error state example states
            error_normal: false,
            error_error: false,
            error_error_checked: true,
        }
    }
}

impl CheckboxWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Checkbox Stories")
            .open(&mut open)
            .default_size([700.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Material Design Checkbox Component");
                    ui.add_space(10.0);
                    
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);
                    
                    self.render_checkbox_states(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);
                    
                    self.render_enabled_disabled(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);
                    
                    self.render_error_state(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);
                    
                    self.render_interactive_example(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Controls");
            ui.label("(Apply to examples below)");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("📖 Material Design Spec").clicked() {
                    let _ = webbrowser::open("https://m3.material.io/components/checkbox/overview");
                }
            });
        });
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.checked, "Checked"));
            ui.add(MaterialCheckbox::new(&mut self.disabled, "Disabled"));
            ui.add(MaterialCheckbox::new(&mut self.indeterminate, "Indeterminate"));
            ui.add(MaterialCheckbox::new(&mut self.error, "Error"));
        });
    }

    fn render_checkbox_states(&mut self, ui: &mut egui::Ui) {
        ui.heading("Checkbox States");
        ui.label("Checkboxes support three states: checked, unchecked, and indeterminate.");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Checked");
                ui.add(checkbox(&mut self.state_checked, ""));
            });
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.label("Unchecked");
                ui.add(checkbox(&mut self.state_unchecked, ""));
            });
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.label("Indeterminate");
                ui.add(checkbox(&mut self.state_indeterminate, "").indeterminate(true));
            });
        });
    }

    fn render_enabled_disabled(&mut self, ui: &mut egui::Ui) {
        ui.heading("Enabled vs Disabled");
        ui.label("Disabled checkboxes cannot be interacted with and appear dimmed.");
        ui.add_space(10.0);

        egui::Grid::new("enabled_disabled_grid")
            .num_columns(2)
            .spacing([40.0, 10.0])
            .show(ui, |ui| {
                ui.label("State");
                ui.label("Enabled");
                ui.label("Disabled");
                ui.end_row();

                ui.label("Unchecked:");
                ui.add(checkbox(&mut self.enabled_unchecked, ""));
                ui.add(checkbox(&mut self.disabled_unchecked, "").enabled(false));
                ui.end_row();

                ui.label("Checked:");
                ui.add(checkbox(&mut self.enabled_checked, ""));
                ui.add(checkbox(&mut self.disabled_checked, "").enabled(false));
                ui.end_row();

                ui.label("Indeterminate:");
                ui.add(checkbox(&mut self.enabled_indeterminate, "").indeterminate(true));
                ui.add(checkbox(&mut self.disabled_indeterminate, "").indeterminate(true).enabled(false));
                ui.end_row();
            });
    }

    fn render_error_state(&mut self, ui: &mut egui::Ui) {
        ui.heading("Error State");
        ui.label("Error state indicates validation failure or invalid selection.");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Normal");
                ui.add(checkbox(&mut self.error_normal, "Regular checkbox"));
            });
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.label("Error");
                ui.add(checkbox(&mut self.error_error, "Error checkbox").is_error(true));
            });
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.label("Error (Checked)");
                ui.add(checkbox(&mut self.error_error_checked, "Error checked").is_error(true));
            });
        });
    }

    fn render_interactive_example(&mut self, ui: &mut egui::Ui) {
        ui.heading("Interactive Example");
        ui.label("Toggle checkboxes with labels (affected by controls above).");
        ui.add_space(10.0);

        ui.vertical(|ui| {
            let mut option1 = checkbox(&mut self.option1_checked, "Option 1");
            let mut option2 = checkbox(&mut self.option2_checked, "Option 2");
            let mut option3 = checkbox(&mut self.option3_checked, "Option 3");

            // Apply controls
            if self.disabled {
                option1 = option1.enabled(false);
                option2 = option2.enabled(false);
                option3 = option3.enabled(false);
            }
            if self.indeterminate {
                option1 = option1.indeterminate(true);
            }
            if self.error {
                option1 = option1.is_error(true);
                option2 = option2.is_error(true);
                option3 = option3.is_error(true);
            }

            ui.add(option1);
            ui.add(option2);
            ui.add(option3);
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Selected:");
            let mut selected = Vec::new();
            if self.option1_checked {
                selected.push("Option 1");
            }
            if self.option2_checked {
                selected.push("Option 2");
            }
            if self.option3_checked {
                selected.push("Option 3");
            }
            if selected.is_empty() {
                ui.label("None");
            } else {
                ui.label(selected.join(", "));
            }
        });
    }
}
