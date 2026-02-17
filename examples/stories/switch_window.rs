#![doc(hidden)]

use crate::{switch, MaterialButton};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct SwitchWindow {
    pub open: bool,
    disabled: bool,
    selected: bool,
    icons: bool,
    show_only_selected_icon: bool,
    // Switch states
    basic_switch: bool,
    wifi_enabled: bool,
    bluetooth_enabled: bool,
    notifications_enabled: bool,
    location_enabled: bool,
    dark_mode_enabled: bool,
    auto_sync_enabled: bool,
    airplane_mode: bool,
    mobile_data: bool,
    battery_saver: bool,
}

impl Default for SwitchWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            selected: false,
            icons: false,
            show_only_selected_icon: false,
            basic_switch: false,
            wifi_enabled: true,
            bluetooth_enabled: false,
            notifications_enabled: true,
            location_enabled: false,
            dark_mode_enabled: false,
            auto_sync_enabled: true,
            airplane_mode: false,
            mobile_data: true,
            battery_saver: false,
        }
    }
}

impl SwitchWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Switch Stories")
            .open(&mut open)
            .default_size([600.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_switch(ui);
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

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/switch/stories/");
            }
        });

        ui.checkbox(&mut self.disabled, "Disabled");
        ui.checkbox(&mut self.selected, "Selected (for basic switch)");
        ui.checkbox(&mut self.icons, "Show Icons");
        ui.checkbox(&mut self.show_only_selected_icon, "Show Only Selected Icon");
    }

    fn render_basic_switch(&mut self, ui: &mut egui::Ui) {
        ui.heading("Basic Switch");

        // Apply the selected state control first
        if self.selected {
            self.basic_switch = true;
        }

        let mut basic_switch_control = switch(&mut self.basic_switch);

        if self.disabled {
            basic_switch_control = basic_switch_control.enabled(false);
        }

        ui.add(basic_switch_control.text("Example switch"));

        ui.label(format!(
            "Switch state: {}",
            if self.basic_switch { "ON" } else { "OFF" }
        ));
    }

    fn render_labeled_switches(&mut self, ui: &mut egui::Ui) {
        ui.heading("Switches with Labels");

        ui.vertical(|ui| {
            // Wi-Fi switch
            let mut wifi_switch = switch(&mut self.wifi_enabled).text("Wi-Fi");
            if self.disabled {
                wifi_switch = wifi_switch.enabled(false);
            }
            ui.add(wifi_switch);

            ui.add_space(8.0);

            // Bluetooth switch
            let mut bluetooth_switch = switch(&mut self.bluetooth_enabled).text("Bluetooth");
            if self.disabled {
                bluetooth_switch = bluetooth_switch.enabled(false);
            }
            ui.add(bluetooth_switch);

            ui.add_space(8.0);

            // Notifications switch
            let mut notifications_switch =
                switch(&mut self.notifications_enabled).text("Notifications");
            if self.disabled {
                notifications_switch = notifications_switch.enabled(false);
            }
            ui.add(notifications_switch);

            ui.add_space(8.0);

            // Location switch
            let mut location_switch = switch(&mut self.location_enabled).text("Location Services");
            if self.disabled {
                location_switch = location_switch.enabled(false);
            }
            ui.add(location_switch);
        });
    }

    fn render_settings_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings Examples");

        ui.vertical(|ui| {
            ui.label("Appearance");
            ui.separator();

            let mut dark_mode_switch = switch(&mut self.dark_mode_enabled).text("Dark Mode");
            if self.disabled {
                dark_mode_switch = dark_mode_switch.enabled(false);
            }
            ui.add(dark_mode_switch);

            ui.add_space(16.0);

            ui.label("Data & Sync");
            ui.separator();

            let mut auto_sync_switch = switch(&mut self.auto_sync_enabled).text("Auto-sync data");
            if self.disabled {
                auto_sync_switch = auto_sync_switch.enabled(false);
            }
            ui.add(auto_sync_switch);

            ui.add_space(8.0);

            let mut mobile_data_switch = switch(&mut self.mobile_data).text("Mobile data");
            if self.disabled {
                mobile_data_switch = mobile_data_switch.enabled(false);
            }
            ui.add(mobile_data_switch);

            ui.add_space(16.0);

            ui.label("Power & Performance");
            ui.separator();

            let mut airplane_switch = switch(&mut self.airplane_mode).text("Airplane mode");
            if self.disabled {
                airplane_switch = airplane_switch.enabled(false);
            }
            ui.add(airplane_switch);

            ui.add_space(8.0);

            let mut battery_saver_switch = switch(&mut self.battery_saver).text("Battery saver");
            if self.disabled {
                battery_saver_switch = battery_saver_switch.enabled(false);
            }
            ui.add(battery_saver_switch);
        });

        ui.add_space(16.0);

        // Status summary
        ui.separator();
        ui.label("Current Status:");
        ui.horizontal(|ui| {
            ui.label(format!(
                "Wi-Fi: {}",
                if self.wifi_enabled { "ON" } else { "OFF" }
            ));
            ui.label(format!(
                "Bluetooth: {}",
                if self.bluetooth_enabled { "ON" } else { "OFF" }
            ));
            ui.label(format!(
                "Notifications: {}",
                if self.notifications_enabled {
                    "ON"
                } else {
                    "OFF"
                }
            ));
        });
        ui.horizontal(|ui| {
            ui.label(format!(
                "Location: {}",
                if self.location_enabled { "ON" } else { "OFF" }
            ));
            ui.label(format!(
                "Dark Mode: {}",
                if self.dark_mode_enabled { "ON" } else { "OFF" }
            ));
            ui.label(format!(
                "Auto-sync: {}",
                if self.auto_sync_enabled { "ON" } else { "OFF" }
            ));
        });
        ui.horizontal(|ui| {
            ui.label(format!(
                "Airplane: {}",
                if self.airplane_mode { "ON" } else { "OFF" }
            ));
            ui.label(format!(
                "Mobile Data: {}",
                if self.mobile_data { "ON" } else { "OFF" }
            ));
            ui.label(format!(
                "Battery Saver: {}",
                if self.battery_saver { "ON" } else { "OFF" }
            ));
        });

        // Conditional warnings
        if self.airplane_mode && (self.wifi_enabled || self.bluetooth_enabled || self.mobile_data) {
            ui.add_space(8.0);
            ui.colored_label(
                egui::Color32::YELLOW,
                "⚠ Airplane mode typically disables wireless connections",
            );
        }

        if !self.notifications_enabled {
            ui.add_space(8.0);
            ui.colored_label(egui::Color32::GRAY, "ℹ You won't receive notifications");
        }
    }
}
