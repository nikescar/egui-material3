use eframe::egui::{self, Ui, Window};
use crate::MaterialDivider;

pub struct DividerWindow {
    pub open: bool,
    inset: bool,
    inset_start: bool,
    inset_end: bool,
}

impl Default for DividerWindow {
    fn default() -> Self {
        Self {
            open: false,
            inset: false,
            inset_start: false,
            inset_end: false,
        }
    }
}

impl DividerWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Divider Stories")
            .open(&mut open)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_divider_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Divider Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://github.com/material-components/material-web/blob/main/docs/components/divider.md");
            }
        });

        ui.checkbox(&mut self.inset, "inset");
        ui.checkbox(&mut self.inset_start, "inset (start)");
        ui.checkbox(&mut self.inset_end, "inset (end)");
    }

    fn render_divider_examples(&mut self, ui: &mut Ui) {
        ui.heading("Divider Examples");
        
        // Create a list-like container to demonstrate dividers
        egui::Frame::NONE
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(121, 116, 126)))
            .inner_margin(0.0)
            .show(ui, |ui| {
                ui.set_width(256.0);
                
                // List item one
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label("List item one");
                });
                ui.add_space(16.0);
                
                // First divider with controls
                ui.add(MaterialDivider::new()
                    .inset(self.inset)
                    .inset_start(self.inset_start)
                    .inset_end(self.inset_end));
                
                ui.add_space(16.0);
                
                // List item two
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label("List item two");
                });
                ui.add_space(16.0);
                
                // Second divider (separator role)
                ui.add(MaterialDivider::new());
                
                ui.add_space(16.0);
                
                // List item three
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label("List item three");
                });
                ui.add_space(16.0);
                
                // Third divider with controls
                ui.add(MaterialDivider::new()
                    .inset(self.inset)
                    .inset_start(self.inset_start)
                    .inset_end(self.inset_end));
                
                ui.add_space(16.0);
                
                // List item four
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label("List item four");
                });
                ui.add_space(16.0);
            });
        
        ui.add_space(20.0);
        
        // Additional examples showing different use cases
        ui.heading("Additional Examples");
        
        ui.label("Standard divider:");
        ui.add(MaterialDivider::new());
        
        ui.add_space(10.0);
        
        ui.label("Inset divider:");
        ui.add(MaterialDivider::new().inset(true));
        
        ui.add_space(10.0);
        
        ui.label("Inset start divider:");
        ui.add(MaterialDivider::new().inset_start(true));
        
        ui.add_space(10.0);
        
        ui.label("Inset end divider:");
        ui.add(MaterialDivider::new().inset_end(true));
    }
}