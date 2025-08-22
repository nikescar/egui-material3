use eframe::egui::{self, Ui, Window};
use crate::{MaterialCheckbox, icon_button_standard, icon_button_filled, icon_button_filled_tonal, icon_button_outlined, icon_button_toggle};

pub struct IconButtonWindow {
    pub open: bool,
    icon_toggle_1: bool,
    icon_toggle_2: bool,
    icon_toggle_3: bool,
    icon_toggle_4: bool,
    enabled: bool,
    size: f32,
    container: bool, // Container button style (rectangular vs circular)
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
            container: false, // circular by default
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
            ui.separator();
            ui.add(MaterialCheckbox::new(&mut self.container, "Container Button (Rectangular)"));
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
        
        ui.label(format!("Standard Icon Buttons {} (using '{}')", if self.container { "(Container)" } else { "(Circular)" }, self.selected_icon));
        ui.horizontal(|ui| {
            if ui.add(icon_button_standard(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Standard '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_standard(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Standard '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_standard(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Standard '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_standard(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Standard '{}' button clicked!", self.selected_icon);
            }
        });

        ui.add_space(15.0);
        
        ui.label(format!("Filled Icon Buttons {} (using '{}')", if self.container { "(Container)" } else { "(Circular)" }, self.selected_icon));
        ui.horizontal(|ui| {
            if ui.add(icon_button_filled(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled '{}' button clicked!", self.selected_icon);
            }
        });

        ui.add_space(15.0);
        
        ui.label(format!("Filled Tonal Icon Buttons {} (using '{}')", if self.container { "(Container)" } else { "(Circular)" }, self.selected_icon));
        ui.horizontal(|ui| {
            if ui.add(icon_button_filled_tonal(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled tonal '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled_tonal(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled tonal '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled_tonal(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled tonal '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled_tonal(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Filled tonal '{}' button clicked!", self.selected_icon);
            }
        });

        ui.add_space(15.0);
        
        ui.label(format!("Outlined Icon Buttons {} (using '{}')", if self.container { "(Container)" } else { "(Circular)" }, self.selected_icon));
        ui.horizontal(|ui| {
            if ui.add(icon_button_outlined(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Outlined '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_outlined(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Outlined '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_outlined(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Outlined '{}' button clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_outlined(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Outlined '{}' button clicked!", self.selected_icon);
            }
        });

        ui.add_space(20.0);

        ui.heading("Toggle Icon Buttons");
        
        ui.label(format!("Toggle buttons maintain selected/unselected state (using '{}'):", self.selected_icon));
        ui.horizontal(|ui| {
            ui.add(icon_button_toggle(&self.selected_icon, &mut self.icon_toggle_1)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            );
            ui.label(if self.icon_toggle_1 { "✓ Selected" } else { "○ Unselected" });
            
            ui.separator();
            
            ui.add(icon_button_toggle(&self.selected_icon, &mut self.icon_toggle_2)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            );
            ui.label(if self.icon_toggle_2 { "✓ Selected" } else { "○ Unselected" });
        });

        ui.horizontal(|ui| {
            ui.add(icon_button_toggle(&self.selected_icon, &mut self.icon_toggle_3)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            );
            ui.label(if self.icon_toggle_3 { "✓ Selected" } else { "○ Unselected" });
            
            ui.separator();
            
            ui.add(icon_button_toggle(&self.selected_icon, &mut self.icon_toggle_4)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            );
            ui.label(if self.icon_toggle_4 { "✓ Selected" } else { "○ Unselected" });
        });

        ui.add_space(20.0);

        ui.heading("Custom Icon Demo");
        ui.label("Use your custom icon:");
        ui.horizontal(|ui| {
            if ui.add(icon_button_standard(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Custom icon '{}' clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Custom filled icon '{}' clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_filled_tonal(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Custom filled tonal icon '{}' clicked!", self.selected_icon);
            }
            
            if ui.add(icon_button_outlined(&self.selected_icon)
                .size(self.size)
                .enabled(self.enabled)
                .container(self.container)
            ).clicked() {
                println!("Custom outlined icon '{}' clicked!", self.selected_icon);
            }
        });

        ui.add_space(20.0);

        ui.heading("Sizes Demo");
        ui.label(format!("Different sizes using '{}' (24px, 32px, 40px, 48px, 56px):", self.selected_icon));
        ui.horizontal(|ui| {
            for size in [24.0, 32.0, 40.0, 48.0, 56.0] {
                if ui.add(icon_button_filled(&self.selected_icon)
                    .size(size)
                    .enabled(self.enabled)
                    .container(self.container)
                ).clicked() {
                    println!("Size {}px '{}' button clicked!", size, self.selected_icon);
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
                ui.label("Container Styles:");
                ui.label("• Circular: Full rounded (traditional)");
                ui.label("• Container: Rectangular with rounded corners");
                ui.label("• Toggle to switch between styles");
                ui.label("• Container provides different visual weight");
            });
            
            ui.vertical(|ui| {
                ui.label("Best Practices:");
                ui.label("• Use Material Icons for consistency");
                ui.label("• Standard size is 40px");
                ui.label("• Toggle buttons show selection state");
                ui.label("• Choose container style for visual hierarchy");
            });
        });
    }
}