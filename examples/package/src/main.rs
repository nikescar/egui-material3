// Minimal egui-material3 example
use eframe::egui;
use egui_material3::{
    MaterialButton, MaterialCheckbox, MaterialSlider, MaterialSwitch,
    theme::{setup_local_fonts, setup_local_theme, load_fonts, load_themes, update_window_background}
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Material Design Example",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and themes
            setup_local_fonts(Some("resources/NanumGothic-Regular.ttf"));
            setup_local_theme(Some("resources/material-theme.json")); // Use default theme
            
            // Load fonts and themes
            load_fonts(&cc.egui_ctx);
            load_themes();
            
            // Apply theme background
            update_window_background(&cc.egui_ctx);
            
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    checked: bool,
    slider_value: f32,
    switch_enabled: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Material Design 3 Components 테스트");
            ui.add_space(20.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Material Button
                if ui.add(MaterialButton::filled("Material Button")).clicked() {
                    println!("Button clicked!");
                }
                
                ui.add_space(10.0);
                
                // Material Checkbox
                ui.add(MaterialCheckbox::new(&mut self.checked, "Material Checkbox"));
                
                ui.add_space(10.0);
                
                // Material Slider
                ui.label("Material Slider:");
                ui.add(MaterialSlider::new(&mut self.slider_value, 0.0..=100.0));
                ui.label(format!("Value: {:.1}", self.slider_value));
                
                ui.add_space(10.0);
                
                // Material Switch
                ui.horizontal(|ui| {
                    ui.add(MaterialSwitch::new(&mut self.switch_enabled));
                    ui.label("Material Switch");
                });
                
                ui.add_space(20.0);
                
                // Status display
                ui.group(|ui| {
                    ui.label("Component States:");
                    ui.label(format!("• Checkbox: {}", if self.checked { "Checked" } else { "Unchecked" }));
                    ui.label(format!("• Slider: {:.1}", self.slider_value));
                    ui.label(format!("• Switch: {}", if self.switch_enabled { "On" } else { "Off" }));
                });
            });
            
        });
    }
}