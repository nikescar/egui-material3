#![doc(hidden)]

use crate::{
    material_symbol::ICON_CHECK,
    switch, MaterialButton, MaterialSwitchSize,
};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct SwitchWindow {
    pub open: bool,
    disabled: bool,
    medium_checked: bool,
    medium_unchecked: bool,
    small_checked: bool,
    small_unchecked: bool,
    icon_checked: bool,
    icon_unchecked: bool,
    wifi_enabled: bool,
    bluetooth_enabled: bool,
    notifications_enabled: bool,
    dark_mode_enabled: bool,
}

impl Default for SwitchWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            medium_checked: true,
            medium_unchecked: false,
            small_checked: true,
            small_unchecked: false,
            icon_checked: true,
            icon_unchecked: false,
            wifi_enabled: true,
            bluetooth_enabled: false,
            notifications_enabled: true,
            dark_mode_enabled: false,
        }
    }
}

impl SwitchWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Switch Stories")
            .open(&mut open)
            .default_size([720.0, 640.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_sizes(ui);
                    ui.add_space(20.0);
                    self.render_with_icon(ui);
                    ui.add_space(20.0);
                    self.render_labeled_switches(ui);
                    ui.add_space(20.0);
                    self.render_settings_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Switch Controls");
            if ui.add(MaterialButton::filled("Leo Toggle").small()).clicked() {
                let _ = webbrowser::open(
                    "https://github.com/brave/leo/tree/main/src/components/toggle",
                );
            }
        });

        ui.checkbox(&mut self.disabled, "Disabled");
    }

    fn render_sizes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Toggle Sizes");
        ui.label("Nala toggle scale — medium (52×32) and small (40×24).");

        let disabled = self.disabled;

        egui::Grid::new("switch_sizes")
            .num_columns(3)
            .spacing([24.0, 12.0])
            .show(ui, |ui| {
                ui.label("");
                ui.label("Unchecked");
                ui.label("Checked");
                ui.end_row();

                ui.label("Medium");
                ui.add(nala_toggle(
                    &mut self.medium_unchecked,
                    MaterialSwitchSize::Medium,
                    disabled,
                    None,
                ));
                ui.add(nala_toggle(
                    &mut self.medium_checked,
                    MaterialSwitchSize::Medium,
                    disabled,
                    None,
                ));
                ui.end_row();

                ui.label("Small");
                ui.add(nala_toggle(
                    &mut self.small_unchecked,
                    MaterialSwitchSize::Small,
                    disabled,
                    None,
                ));
                ui.add(nala_toggle(
                    &mut self.small_checked,
                    MaterialSwitchSize::Small,
                    disabled,
                    None,
                ));
                ui.end_row();
            });
    }

    fn render_with_icon(&mut self, ui: &mut egui::Ui) {
        ui.heading("Toggle with On Icon");
        ui.label("Leo shows an optional icon on the thumb when checked.");

        let disabled = self.disabled;

        ui.horizontal(|ui| {
            ui.add(nala_toggle(
                &mut self.icon_unchecked,
                MaterialSwitchSize::Medium,
                disabled,
                Some(ICON_CHECK),
            ));
            ui.add(nala_toggle(
                &mut self.icon_checked,
                MaterialSwitchSize::Medium,
                disabled,
                Some(ICON_CHECK),
            ));
        });
    }

    fn render_labeled_switches(&mut self, ui: &mut egui::Ui) {
        ui.heading("Labeled Toggles");

        let disabled = self.disabled;

        ui.vertical(|ui| {
            ui.add(
                nala_toggle(
                    &mut self.wifi_enabled,
                    MaterialSwitchSize::Medium,
                    disabled,
                    None,
                )
                .text("Wi-Fi"),
            );
            ui.add_space(8.0);
            ui.add(
                nala_toggle(
                    &mut self.bluetooth_enabled,
                    MaterialSwitchSize::Medium,
                    disabled,
                    None,
                )
                .text("Bluetooth"),
            );
            ui.add_space(8.0);
            ui.add(
                nala_toggle(
                    &mut self.notifications_enabled,
                    MaterialSwitchSize::Medium,
                    disabled,
                    None,
                )
                .text("Notifications"),
            );
        });
    }

    fn render_settings_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings Example");

        let disabled = self.disabled;

        ui.vertical(|ui| {
            ui.label("Appearance");
            ui.separator();
            ui.add(
                nala_toggle(
                    &mut self.dark_mode_enabled,
                    MaterialSwitchSize::Medium,
                    disabled,
                    None,
                )
                .text("Dark Mode"),
            );
        });
    }
}

fn nala_toggle<'a>(
    value: &'a mut bool,
    size: MaterialSwitchSize,
    disabled: bool,
    icon: Option<char>,
) -> crate::MaterialSwitch<'a> {
    let mut control = switch(value).size(size);
    if disabled {
        control = control.enabled(false);
    }
    if let Some(icon) = icon {
        control = control.selected_icon(icon);
    }
    control
}
