use eframe::egui::{self, Ui, Window};
use crate::MaterialElevation;

pub struct ElevationWindow {
    pub open: bool,
    level: u8,
}

impl Default for ElevationWindow {
    fn default() -> Self {
        Self {
            open: false,
            level: 1,
        }
    }
}

impl ElevationWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Elevation Stories")
            .open(&mut open)
            .default_size([700.0, 500.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_single_elevation(ui);
                    ui.add_space(30.0);
                    self.render_all_levels(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Elevation Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://github.com/material-components/material-web/blob/main/docs/components/elevation.md");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Level:");
            ui.add(egui::Slider::new(&mut self.level, 0..=5).integer());
        });
    }

    fn render_single_elevation(&mut self, ui: &mut Ui) {
        ui.heading("Elevation Example");
        
        ui.horizontal(|ui| {
            ui.add(MaterialElevation::new()
                .level(self.level)
                .min_size(egui::Vec2::new(64.0, 64.0)));
        });
    }

    fn render_all_levels(&mut self, ui: &mut Ui) {
        ui.heading("All Elevation Levels");
        ui.label("Compare different elevation levels side by side:");
        
        ui.add_space(10.0);
        
        // First row: levels 0-2
        ui.horizontal(|ui| {
            ui.add(MaterialElevation::new()
                .level(0)
                .min_size(egui::Vec2::new(64.0, 64.0)));
            
            ui.add_space(16.0);
            
            ui.add(MaterialElevation::new()
                .level(1)
                .min_size(egui::Vec2::new(64.0, 64.0)));
            
            ui.add_space(16.0);
            
            ui.add(MaterialElevation::new()
                .level(2)
                .min_size(egui::Vec2::new(64.0, 64.0)));
        });
        
        ui.add_space(16.0);
        
        // Second row: levels 3-5
        ui.horizontal(|ui| {
            ui.add(MaterialElevation::new()
                .level(3)
                .min_size(egui::Vec2::new(64.0, 64.0)));
            
            ui.add_space(16.0);
            
            ui.add(MaterialElevation::new()
                .level(4)
                .min_size(egui::Vec2::new(64.0, 64.0)));
            
            ui.add_space(16.0);
            
            ui.add(MaterialElevation::new()
                .level(5)
                .min_size(egui::Vec2::new(64.0, 64.0)));
        });
        
        ui.add_space(20.0);
        
        // Examples with custom content
        ui.heading("Elevation with Custom Content");
        
        ui.horizontal(|ui| {
            ui.add(MaterialElevation::new()
                .level(2)
                .min_size(egui::Vec2::new(140.0, 100.0))
                .content(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Custom");
                        ui.label("Content");
                    });
                }));
            
            ui.add_space(16.0);
            
            ui.add(MaterialElevation::new()
                .level(3)
                .min_size(egui::Vec2::new(140.0, 100.0))
                .text("Text Content"));
            
            ui.add_space(16.0);
            
            ui.add(MaterialElevation::new()
                .level(4)
                .min_size(egui::Vec2::new(140.0, 100.0))
                .content(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        if ui.add_sized([100.0, 32.0], egui::Button::new("Elevated Button")).clicked() {
                            println!("Elevated button clicked!");
                        }
                        ui.add_space(20.0);
                    });
                }));
        });
    }
}