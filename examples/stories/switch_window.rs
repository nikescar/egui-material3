#![doc(hidden)]

use crate::{
    material_symbol::{
        ICON_CHECK, ICON_CLEAR, ICON_CLOSE, ICON_DONE, ICON_STAR, ICON_STAR_OUTLINE,
    },
    switch, MaterialButton,
};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct SwitchWindow {
    pub open: bool,
    disabled: bool,
    selected: bool,
    icons: bool,
    show_only_selected_icon: bool,
    show_track_outline: bool,
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
    // Icon example switches
    icon_check_close: bool,
    icon_check_only: bool,
    icon_star_empty: bool,
    icon_done_clear: bool,
    // State demonstration
    state_demo_normal: bool,
    state_demo_hover: bool,
    state_demo_focused: bool,
    state_demo_pressed: bool,
    state_demo_disabled: bool,
}

impl Default for SwitchWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            selected: false,
            icons: false,
            show_only_selected_icon: false,
            show_track_outline: true,
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
            icon_check_close: true,
            icon_check_only: false,
            icon_star_empty: true,
            icon_done_clear: false,
            state_demo_normal: false,
            state_demo_hover: false,
            state_demo_focused: false,
            state_demo_pressed: false,
            state_demo_disabled: true,
        }
    }
}

impl SwitchWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Switch Stories")
            .open(&mut open)
            .default_size([800.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_switch(ui);
                    ui.add_space(20.0);
                    self.render_icon_examples(ui);
                    ui.add_space(20.0);
                    self.render_state_examples(ui);
                    ui.add_space(20.0);
                    self.render_track_outline_examples(ui);
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
        ui.checkbox(&mut self.show_track_outline, "Show Track Outline (Material 3)");
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

        // Apply icon controls
        if self.icons {
            if self.show_only_selected_icon {
                basic_switch_control = basic_switch_control.selected_icon(ICON_CHECK);
            } else {
                basic_switch_control =
                    basic_switch_control.with_icons(ICON_CHECK, ICON_CLOSE);
            }
        }

        basic_switch_control = basic_switch_control.show_track_outline(self.show_track_outline);

        ui.add(basic_switch_control.text("Example switch"));

        ui.label(format!(
            "Switch state: {}",
            if self.basic_switch { "ON" } else { "OFF" }
        ));
    }

    fn render_icon_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Icon Examples");
        ui.label("Switches can display Material icons on the thumb");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Check/Close:");
            ui.add(switch(&mut self.icon_check_close).with_icons(ICON_CHECK, ICON_CLOSE));
            ui.label(if self.icon_check_close {
                "Accepted ✓"
            } else {
                "Rejected ✗"
            });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Check only (selected):");
            ui.add(switch(&mut self.icon_check_only).selected_icon(ICON_CHECK));
            ui.label(if self.icon_check_only {
                "Confirmed"
            } else {
                "Unconfirmed"
            });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Star/Empty Star:");
            ui.add(switch(&mut self.icon_star_empty).with_icons(ICON_STAR, ICON_STAR_OUTLINE));
            ui.label(if self.icon_star_empty {
                "Favorited"
            } else {
                "Not favorited"
            });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Done/Clear:");
            ui.add(switch(&mut self.icon_done_clear).with_icons(ICON_DONE, ICON_CLEAR));
            ui.label(if self.icon_done_clear {
                "Complete"
            } else {
                "Incomplete"
            });
        });

        ui.add_space(8.0);

        ui.label("💡 Tip: Icons provide better visual feedback for the switch state");
    }

    fn render_state_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("State Examples");
        ui.label("Different visual states provide feedback for user interactions");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Normal:");
            ui.add(switch(&mut self.state_demo_normal).text("Default state"));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Hover:");
            ui.add(switch(&mut self.state_demo_hover).text("Hover over me"));
            ui.label("(Shows state layer overlay on hover)");
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Pressed:");
            ui.add(switch(&mut self.state_demo_pressed).text("Click and hold"));
            ui.label("(Thumb expands to 28dp when pressed)");
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Disabled:");
            ui.add(
                switch(&mut self.state_demo_disabled)
                    .enabled(false)
                    .text("Cannot interact"),
            );
            ui.label("(Reduced opacity, no interaction)");
        });

        ui.add_space(8.0);

        ui.label("🎨 Note: Colors adapt based on selection and interaction state");
    }

    fn render_track_outline_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Track Outline Examples");
        ui.label("Material 3 includes a 2dp outline on the track when unselected");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let mut outline_example_1 = false;
            ui.label("With outline (M3):");
            ui.add(switch(&mut outline_example_1).show_track_outline(true));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let mut outline_example_2 = false;
            ui.label("Without outline (M2):");
            ui.add(switch(&mut outline_example_2).show_track_outline(false));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let mut outline_example_3 = true;
            ui.label("Selected (transparent outline):");
            ui.add(switch(&mut outline_example_3).show_track_outline(true));
            ui.label("← Outline disappears when on");
        });

        ui.add_space(8.0);

        ui.label("📐 Material 3 Spec: The outline provides better visual separation when off");
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
