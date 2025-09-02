#![doc(hidden)]

use eframe::egui::{self, Window};
use crate::{linear_progress, circular_progress, MaterialButton, icon_button_standard};

#[doc(hidden)]
pub struct ProgressWindow {
    pub open: bool,
    value: f32,
    max: f32,
    buffer: f32,
    indeterminate: bool,
    four_color: bool,
    track_color: String,
    track_height: f32,
    indicator_height: f32,
    custom_theme: bool,
    // Component demo states
    loading_button_active: bool,
    player_progress: f32,
    player_playing: bool,
}

impl Default for ProgressWindow {
    fn default() -> Self {
        Self {
            open: false,
            value: 0.5,
            max: 1.0,
            buffer: 0.7,
            indeterminate: false,
            four_color: false,
            track_color: "gainsboro".to_string(),
            track_height: 4.0,
            indicator_height: 4.0,
            custom_theme: false,
            loading_button_active: false,
            player_progress: 0.3,
            player_playing: false,
        }
    }
}

impl ProgressWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Progress Stories")
            .open(&mut open)
            .default_size([700.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_linear_progress(ui);
                    ui.add_space(20.0);
                    self.render_circular_progress(ui);
                    ui.add_space(20.0);
                    self.render_component_demos(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Progress Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/progress/stories/");
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Value:");
            ui.add(egui::Slider::new(&mut self.value, 0.0..=1.0).step_by(0.1));
        });
        
        ui.horizontal(|ui| {
            ui.label("Max:");
            ui.add(egui::Slider::new(&mut self.max, 0.1..=2.0).step_by(0.1));
        });
        
        ui.horizontal(|ui| {
            ui.label("Buffer (Linear):");
            ui.add(egui::Slider::new(&mut self.buffer, 0.0..=1.0).step_by(0.1));
        });
        
        ui.checkbox(&mut self.indeterminate, "Indeterminate");
        ui.checkbox(&mut self.four_color, "Four Color");
        ui.checkbox(&mut self.custom_theme, "Custom Theme (Linear)");
        
        ui.horizontal(|ui| {
            ui.label("Track Color (Linear):");
            ui.text_edit_singleline(&mut self.track_color);
        });
        
        ui.horizontal(|ui| {
            ui.label("Track Height (Linear):");
            ui.add(egui::Slider::new(&mut self.track_height, 1.0..=20.0).step_by(1.0));
        });
        
        ui.horizontal(|ui| {
            ui.label("Indicator Height (Linear):");
            ui.add(egui::Slider::new(&mut self.indicator_height, 1.0..=20.0).step_by(1.0));
        });
    }

    fn render_linear_progress(&mut self, ui: &mut egui::Ui) {
        ui.heading("Linear Progress");
        
        let mut progress = linear_progress()
            .value(if self.indeterminate { 0.0 } else { self.value })
            .width(400.0);
        
        if self.indeterminate {
            progress = progress.indeterminate(true);
        }
        
        if self.four_color {
            progress = progress.four_color_enabled(true);
        }
        
        ui.add(progress);
        
        ui.add_space(10.0);
        
        // Custom themed linear progress
        if self.custom_theme {
            ui.label("Custom Themed Linear Progress:");
            let mut custom_progress = linear_progress()
                .value(if self.indeterminate { 0.0 } else { self.value })
                .width(400.0);
            
            if self.indeterminate {
                custom_progress = custom_progress.indeterminate(true);
            }
            
            ui.add(custom_progress);
        }
    }

    fn render_circular_progress(&mut self, ui: &mut egui::Ui) {
        ui.heading("Circular Progress");
        
        let mut progress = circular_progress()
            .value(if self.indeterminate { 0.0 } else { self.value });
        
        if self.indeterminate {
            progress = progress.indeterminate(true);
        }
        
        if self.four_color {
            progress = progress.four_color_enabled(true);
        }
        
        ui.add(progress);
    }

    fn render_component_demos(&mut self, ui: &mut egui::Ui) {
        ui.heading("Indicators in Components");
        
        ui.horizontal(|ui| {
            // Loading button demo
            if self.loading_button_active {
                ui.add(MaterialButton::filled("Loading...")
                    .leading_icon("sync")
                    .enabled(false));
                
                if ui.add(MaterialButton::text("Stop")).clicked() {
                    self.loading_button_active = false;
                }
            } else {
                if ui.add(MaterialButton::filled("Load")).clicked() {
                    self.loading_button_active = true;
                }
            }
            
            ui.add_space(20.0);
            
            // Player progress demo
            ui.vertical(|ui| {
                ui.label("Player Progress:");
                ui.add(circular_progress()
                    .value(self.player_progress));
                
                // Play/pause button simulation
                let play_button = if self.player_playing {
                    icon_button_standard("pause")
                } else {
                    icon_button_standard("play_arrow")
                };
                
                if ui.add(play_button).clicked() {
                    self.player_playing = !self.player_playing;
                    if self.player_playing {
                        // Simulate progress when playing
                        self.player_progress = (self.player_progress + 0.1).min(1.0);
                    }
                }
            });
        });
        
        ui.add_space(10.0);
        
        // Progress control buttons
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::text("Reset Progress")).clicked() {
                self.value = 0.0;
                self.player_progress = 0.0;
            }
            
            if ui.add(MaterialButton::text("Advance Progress")).clicked() {
                self.value = (self.value + 0.1).min(1.0);
                self.player_progress = (self.player_progress + 0.1).min(1.0);
            }
        });
    }
}