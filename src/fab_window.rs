use eframe::egui::{self, Window};
use crate::{fab_surface, fab_primary, fab_secondary, fab_tertiary, fab_branded, FabSize, google_branded_icon};

pub struct FabWindow {
    pub open: bool,
    icon: String,
    label: String,
    lowered: bool,
    size: FabSize,
}

impl Default for FabWindow {
    fn default() -> Self {
        Self {
            open: false,
            icon: "add".to_string(),
            label: String::new(),
            lowered: false,
            size: FabSize::Regular,
        }
    }
}

impl FabWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("FAB Stories")
            .open(&mut open)
            .default_size([700.0, 500.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_fab_variants(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("FAB Controls");
            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/fab/stories/");
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Icon:");
            ui.text_edit_singleline(&mut self.icon);
        });
        
        ui.horizontal(|ui| {
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);
        });
        
        ui.checkbox(&mut self.lowered, "Lowered");
        
        ui.horizontal(|ui| {
            ui.label("Size:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.size))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.size, FabSize::Small, "Small");
                    ui.selectable_value(&mut self.size, FabSize::Regular, "Regular");
                    ui.selectable_value(&mut self.size, FabSize::Large, "Large");
                });
        });
    }

    fn render_fab_variants(&mut self, ui: &mut egui::Ui) {
        ui.heading("Floating Action Buttons");

        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label("Surface");
                let icon = if self.icon.is_empty() { "add" } else { &self.icon };
                let mut fab = fab_surface()
                    .icon(icon)
                    .size(self.size);
                
                if !self.label.is_empty() {
                    fab = fab.text(&self.label);
                }
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Surface FAB clicked!")));
            });
            
            ui.vertical(|ui| {
                ui.label("Primary");
                let icon = if self.icon.is_empty() { "add" } else { &self.icon };
                let mut fab = fab_primary()
                    .icon(icon)
                    .size(self.size);
                
                if !self.label.is_empty() {
                    fab = fab.text(&self.label);
                }
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Primary FAB clicked!")));
            });
            
            ui.vertical(|ui| {
                ui.label("Secondary");
                let icon = if self.icon.is_empty() { "add" } else { &self.icon };
                let mut fab = fab_secondary()
                    .icon(icon)
                    .size(self.size);
                
                if !self.label.is_empty() {
                    fab = fab.text(&self.label);
                }
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Secondary FAB clicked!")));
            });
            
            ui.vertical(|ui| {
                ui.label("Tertiary");
                let icon = if self.icon.is_empty() { "add" } else { &self.icon };
                let mut fab = fab_tertiary()
                    .icon(icon)
                    .size(self.size);
                
                if !self.label.is_empty() {
                    fab = fab.text(&self.label);
                }
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Tertiary FAB clicked!")));
            });
            
            ui.vertical(|ui| {
                ui.label("Branded");
                let mut fab = fab_branded()
                    .svg_icon(google_branded_icon())
                    .size(self.size);
                
                if !self.label.is_empty() {
                    fab = fab.text(&self.label);
                }
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Branded FAB clicked!")));
            });
        });
        
        ui.add_space(20.0);
        
        // Extended FABs with different configurations
        ui.heading("Extended FABs");
        
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label("Small Extended");
                let mut fab = fab_primary()
                    .icon("create")
                    .text("Create")
                    .size(FabSize::Small);
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Small Extended FAB clicked!")));
            });
            
            ui.vertical(|ui| {
                ui.label("Medium Extended");
                let mut fab = fab_primary()
                    .icon("edit")
                    .text("Edit Document")
                    .size(FabSize::Regular);
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Medium Extended FAB clicked!")));
            });
            
            ui.vertical(|ui| {
                ui.label("Large Extended");
                let mut fab = fab_primary()
                    .icon("save")
                    .text("Save Changes")
                    .size(FabSize::Large);
                
                if self.lowered {
                    fab = fab.lowered(true);
                }
                
                ui.add(fab.on_click(|| println!("Large Extended FAB clicked!")));
            });
        });
    }
}