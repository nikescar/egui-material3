#![doc(hidden)]

use crate::{checkbox, MaterialButton, MaterialCheckboxSize};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct CheckboxWindow {
    pub open: bool,
    disabled: bool,
    normal_unchecked: bool,
    normal_checked: bool,
    small_unchecked: bool,
    small_checked: bool,
    indeterminate: bool,
    option1_checked: bool,
    option2_checked: bool,
    option3_checked: bool,
}

impl Default for CheckboxWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            normal_unchecked: false,
            normal_checked: true,
            small_unchecked: false,
            small_checked: true,
            indeterminate: false,
            option1_checked: true,
            option2_checked: false,
            option3_checked: false,
        }
    }
}

impl CheckboxWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Checkbox Stories")
            .open(&mut open)
            .default_size([640.0, 520.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_sizes(ui);
                    ui.add_space(20.0);
                    self.render_indeterminate(ui);
                    ui.add_space(20.0);
                    self.render_labeled_checkboxes(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Checkbox Controls");
            if ui.add(MaterialButton::filled("Leo Checkbox").small()).clicked() {
                let _ = webbrowser::open(
                    "https://github.com/brave/leo/tree/main/src/components/checkbox",
                );
            }
        });
        ui.checkbox(&mut self.disabled, "Disabled");
    }

    fn render_sizes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Checkbox Sizes");
        ui.label("Nala checkbox scale — normal (20px) and small (16px).");

        let disabled = self.disabled;

        egui::Grid::new("checkbox_sizes")
            .num_columns(3)
            .spacing([24.0, 12.0])
            .show(ui, |ui| {
                ui.label("");
                ui.label("Unchecked");
                ui.label("Checked");
                ui.end_row();

                ui.label("Normal");
                ui.add(nala_checkbox(
                    &mut self.normal_unchecked,
                    MaterialCheckboxSize::Normal,
                    disabled,
                    "",
                ));
                ui.add(nala_checkbox(
                    &mut self.normal_checked,
                    MaterialCheckboxSize::Normal,
                    disabled,
                    "",
                ));
                ui.end_row();

                ui.label("Small");
                ui.add(nala_checkbox(
                    &mut self.small_unchecked,
                    MaterialCheckboxSize::Small,
                    disabled,
                    "",
                ));
                ui.add(nala_checkbox(
                    &mut self.small_checked,
                    MaterialCheckboxSize::Small,
                    disabled,
                    "",
                ));
                ui.end_row();
            });
    }

    fn render_indeterminate(&mut self, ui: &mut egui::Ui) {
        ui.heading("Indeterminate");
        ui.label("Extension for select-all patterns (Leo icon, not in the base component).");

        let disabled = self.disabled;
        ui.add(
            nala_checkbox(
                &mut self.indeterminate,
                MaterialCheckboxSize::Normal,
                disabled,
                "Select all",
            )
            .indeterminate(true),
        );
    }

    fn render_labeled_checkboxes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Labeled Checkboxes");

        let disabled = self.disabled;

        ui.vertical(|ui| {
            ui.add(nala_checkbox(
                &mut self.option1_checked,
                MaterialCheckboxSize::Normal,
                disabled,
                "Option 1",
            ));
            ui.add_space(8.0);
            ui.add(nala_checkbox(
                &mut self.option2_checked,
                MaterialCheckboxSize::Normal,
                disabled,
                "Option 2",
            ));
            ui.add_space(8.0);
            ui.add(nala_checkbox(
                &mut self.option3_checked,
                MaterialCheckboxSize::Normal,
                disabled,
                "Option 3",
            ));
        });
    }
}

fn nala_checkbox<'a>(
    value: &'a mut bool,
    size: MaterialCheckboxSize,
    disabled: bool,
    label: &str,
) -> crate::MaterialCheckbox<'a> {
    let mut control = checkbox(value, label).size(size);
    if disabled {
        control = control.enabled(false);
    }
    control
}
