#![doc(hidden)]

use crate::{checkbox, MaterialCheckbox};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct CheckboxWindow {
    pub open: bool,
    checked: bool,
    disabled: bool,
    indeterminate: bool,
    // For labeled checkboxes
    cats_checked: bool,
    dogs_checked: bool,
    birds_indeterminate: bool,
}

impl Default for CheckboxWindow {
    fn default() -> Self {
        Self {
            open: false,
            checked: false,
            disabled: false,
            indeterminate: false,
            cats_checked: false,
            dogs_checked: true,
            birds_indeterminate: false,
        }
    }
}

impl CheckboxWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Checkbox Stories")
            .open(&mut open)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_checkbox(ui);
                    ui.add_space(20.0);
                    self.render_labeled_checkboxes(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Checkbox Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/checkbox/stories/");
            }
        });

        ui.add(MaterialCheckbox::new(&mut self.checked, "Checked"));
        ui.add(MaterialCheckbox::new(&mut self.disabled, "Disabled"));
        ui.add(MaterialCheckbox::new(
            &mut self.indeterminate,
            "Indeterminate",
        ));
    }

    fn render_basic_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.heading("Basic Checkbox");

        let mut checkbox = checkbox(&mut self.checked, "Example checkbox");
        if self.disabled {
            checkbox = checkbox.enabled(false);
        }
        if self.indeterminate {
            checkbox = checkbox.indeterminate(true);
        }

        ui.add(checkbox);
    }

    fn render_labeled_checkboxes(&mut self, ui: &mut egui::Ui) {
        ui.heading("With Labels");
        ui.label("Animals:");

        ui.vertical(|ui| {
            // Cats checkbox
            let mut cats_checkbox = checkbox(&mut self.cats_checked, "Cats");
            if self.disabled {
                cats_checkbox = cats_checkbox.enabled(false);
            }
            ui.add(cats_checkbox);

            // Dogs checkbox (checked by default)
            let mut dogs_checkbox = checkbox(&mut self.dogs_checked, "Dogs");
            if self.disabled {
                dogs_checkbox = dogs_checkbox.enabled(false);
            }
            ui.add(dogs_checkbox);

            // Birds checkbox (indeterminate)
            let mut birds_checkbox =
                checkbox(&mut self.birds_indeterminate, "Birds").indeterminate(true);
            if self.disabled {
                birds_checkbox = birds_checkbox.enabled(false);
            }
            ui.add(birds_checkbox);
        });
    }
}
