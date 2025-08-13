use eframe::egui::{self, Ui, Window};
use crate::{MaterialIconButton, IconButtonVariant, MaterialButton, MaterialCheckbox, icon_button_standard, icon_button_filled, icon_button_filled_tonal, icon_button_outlined, icon_button_toggle};

pub struct IconButtonWindow {
    pub open: bool,
    icon_toggle_1: bool,
    icon_toggle_2: bool,
    icon_toggle_3: bool,
    icon_toggle_4: bool,
    enabled: bool,
    size: f32,
    selected_icon: String,
}

impl Default for IconButtonWindow {
    fn default() -> Self {
        Self {
            open: false,
            icon_toggle_1: false,
            icon_toggle_2: true,
            icon_toggle_3: false,
            icon_toggle_4: true,
            enabled: true,
            size: 40.0,
            selected_icon: "favorite".to_string(),
        }
    }
}

impl IconButtonWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Icon Button Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_icon_button_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Icon Button Controls");
            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://github.com/material-components/material-web/blob/main/docs/components/icon-button.md");
            }
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.enabled, "Enabled"));
        });

        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::Slider::new(&mut self.size, 24.0..=56.0).suffix("px"));
        });

        ui.horizontal(|ui| {
            ui.label("Icon:");
            ui.text_edit_singleline(&mut self.selected_icon);
            ui.label("(Material Icons name)");
        });
    }

    fn render_icon_button_examples(&mut self, ui: &mut Ui) {
        ui.heading("Icon Button Variants");
        
        ui.label("Standard Icon Buttons:");
        ui.horizontal(|ui| {
            if ui.add(icon_button_standard("star")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Standard star button clicked!");
            }
            
            if ui.add(icon_button_standard("favorite")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Standard favorite button clicked!");
            }
            
            if ui.add(icon_button_standard("bookmark")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Standard bookmark button clicked!");
            }
            
            if ui.add(icon_button_standard("home")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Standard home button clicked!");
            }
        });

        ui.add_space(15.0);
        
        ui.label("Filled Icon Buttons:");
        ui.horizontal(|ui| {
            if ui.add(icon_button_filled("star")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled star button clicked!");
            }
            
            if ui.add(icon_button_filled("favorite")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled favorite button clicked!");
            }
            
            if ui.add(icon_button_filled("bookmark")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled bookmark button clicked!");
            }
            
            if ui.add(icon_button_filled("home")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled home button clicked!");
            }
        });

        ui.add_space(15.0);
        
        ui.label("Filled Tonal Icon Buttons:");
        ui.horizontal(|ui| {
            if ui.add(icon_button_filled_tonal("star")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled tonal star button clicked!");
            }
            
            if ui.add(icon_button_filled_tonal("favorite")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled tonal favorite button clicked!");
            }
            
            if ui.add(icon_button_filled_tonal("bookmark")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled tonal bookmark button clicked!");
            }
            
            if ui.add(icon_button_filled_tonal("home")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Filled tonal home button clicked!");
            }
        });

        ui.add_space(15.0);
        
        ui.label("Outlined Icon Buttons:");
        ui.horizontal(|ui| {
            if ui.add(icon_button_outlined("star")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Outlined star button clicked!");
            }
            
            if ui.add(icon_button_outlined("favorite")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Outlined favorite button clicked!");
            }
            
            if ui.add(icon_button_outlined("bookmark")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Outlined bookmark button clicked!");
            }
            
            if ui.add(icon_button_outlined("home")
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Outlined home button clicked!");
            }
        });

        ui.add_space(20.0);

        ui.heading("Toggle Icon Buttons");
        
        ui.label("Toggle buttons maintain selected/unselected state:");
        ui.horizontal(|ui| {
            ui.add(icon_button_toggle("star", &mut self.icon_toggle_1)
                .size(self.size)
                .enabled(self.enabled)
            );
            ui.label(if self.icon_toggle_1 { "★ Selected" } else { "☆ Unselected" });
            
            ui.separator();
            
            ui.add(icon_button_toggle("favorite", &mut self.icon_toggle_2)
                .size(self.size)
                .enabled(self.enabled)
            );
            ui.label(if self.icon_toggle_2 { "♥ Favorited" } else { "♡ Not favorited" });
        });

        ui.horizontal(|ui| {
            ui.add(icon_button_toggle("bookmark", &mut self.icon_toggle_3)
                .size(self.size)
                .enabled(self.enabled)
            );
            ui.label(if self.icon_toggle_3 { "🔖 Bookmarked" } else { "📖 Not bookmarked" });
            
            ui.separator();
            
            ui.add(icon_button_toggle("notifications", &mut self.icon_toggle_4)
                .size(self.size)
                .enabled(self.enabled)
            );
            ui.label(if self.icon_toggle_4 { "🔔 Notifications on" } else { "🔕 Notifications off" });
        });

        ui.add_space(20.0);

        ui.heading("Custom Icon Demo");
        ui.label("Use your custom icon:");
        ui.horizontal(|ui| {
            if ui.add(icon_button_standard(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Custom icon '{}' clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Custom filled icon '{}' clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled_tonal(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Custom filled tonal icon '{}' clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_outlined(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
            ).clicked() {
                println!("Custom outlined icon '{}' clicked!", self.selected_icon);
            }
        });

        ui.add_space(20.0);

        ui.heading("Sizes Demo");
        ui.label("Different sizes (24px, 32px, 40px, 48px, 56px):");
        ui.horizontal(|ui| {
            for size in [24.0, 32.0, 40.0, 48.0, 56.0] {
                if ui.add(icon_button_filled("star")
                    .size(size)
                    .enabled(self.enabled)
                ).clicked() {
                    println!("Size {}px button clicked!", size);
                }
            }
        });

        ui.add_space(20.0);
        
        ui.heading("Usage Guidelines");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Icon Button Types:");
                ui.label("• Standard: Most common, minimal background");
                ui.label("• Filled: High emphasis, solid background");
                ui.label("• Filled Tonal: Medium emphasis, tinted background");
                ui.label("• Outlined: Medium emphasis, with border");
            });
            
            ui.vertical(|ui| {
                ui.label("Best Practices:");
                ui.label("• Use Material Icons for consistency");
                ui.label("• Standard size is 40px");
                ui.label("• Toggle buttons show selection state");
                ui.label("• Keep labels concise and clear");
            });
        });
    }
}