#![doc(hidden)]

use eframe::egui::{self, Window};
use crate::{radio, radio_group};

#[doc(hidden)]
pub struct RadioWindow {
    pub open: bool,
    disabled: bool,
    // Radio states
    basic_radio_selected: Option<usize>,
    labeled_radio_selected: Option<usize>,
    animals_selected: Option<usize>,
    music_selected: Option<usize>,
}

impl Default for RadioWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            basic_radio_selected: Some(0),
            labeled_radio_selected: Some(1),
            animals_selected: Some(1), // Dogs selected by default
            music_selected: None,
        }
    }
}

impl RadioWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Radio Stories")
            .open(&mut open)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_radios(ui);
                    ui.add_space(20.0);
                    self.render_labeled_radios(ui);
                    ui.add_space(20.0);
                    self.render_radio_groups(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Radio Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/radio/stories/");
            }
        });
        
        ui.checkbox(&mut self.disabled, "Disabled");
    }

    fn render_basic_radios(&mut self, ui: &mut egui::Ui) {
        ui.heading("Basic Radios");
        
        ui.label("An example group of radio buttons:");
        
        ui.vertical(|ui| {
            // First radio
            let mut first_radio = radio(&mut self.basic_radio_selected, 0, "First radio");
            if self.disabled {
                first_radio = first_radio.enabled(false);
            }
            ui.add(first_radio);
            
            // Second radio
            let mut second_radio = radio(&mut self.basic_radio_selected, 1, "Second radio");
            if self.disabled {
                second_radio = second_radio.enabled(false);
            }
            ui.add(second_radio);
            
            // Third radio
            let mut third_radio = radio(&mut self.basic_radio_selected, 2, "Third radio");
            if self.disabled {
                third_radio = third_radio.enabled(false);
            }
            ui.add(third_radio);
        });
    }

    fn render_labeled_radios(&mut self, ui: &mut egui::Ui) {
        ui.heading("Radios with Labels");
        
        ui.label("Birds, Cats, Dogs:");
        
        ui.vertical(|ui| {
            // Birds
            let mut birds_radio = radio(&mut self.labeled_radio_selected, 0, "Birds");
            if self.disabled {
                birds_radio = birds_radio.enabled(false);
            }
            ui.add(birds_radio);
            
            // Cats
            let mut cats_radio = radio(&mut self.labeled_radio_selected, 1, "Cats");
            if self.disabled {
                cats_radio = cats_radio.enabled(false);
            }
            ui.add(cats_radio);
            
            // Dogs
            let mut dogs_radio = radio(&mut self.labeled_radio_selected, 2, "Dogs");
            if self.disabled {
                dogs_radio = dogs_radio.enabled(false);
            }
            ui.add(dogs_radio);
        });
    }

    fn render_radio_groups(&mut self, ui: &mut egui::Ui) {
        ui.heading("Radio Groups");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Animals:");
                let mut animals_group = radio_group(&mut self.animals_selected)
                    .option(0, "Birds")
                    .option(1, "Cats")
                    .option(2, "Dogs");
                
                if self.disabled {
                    animals_group = animals_group.enabled(false);
                }
                
                ui.add(animals_group);
            });
            
            ui.add_space(40.0);
            
            ui.vertical(|ui| {
                ui.label("Music Genres:");
                let mut music_group = radio_group(&mut self.music_selected)
                    .option(0, "Rock")
                    .option(1, "Jazz")
                    .option(2, "Classical")
                    .option(3, "Electronic")
                    .option(4, "Country");
                
                if self.disabled {
                    music_group = music_group.enabled(false);
                }
                
                ui.add(music_group);
            });
        });
        
        ui.add_space(10.0);
        
        // Display current selections
        ui.separator();
        ui.label("Current Selections:");
        ui.horizontal(|ui| {
            ui.label(format!("Basic: {:?}", self.basic_radio_selected));
            ui.label(format!("Labeled: {:?}", self.labeled_radio_selected));
            ui.label(format!("Animals: {:?}", self.animals_selected));
            ui.label(format!("Music: {:?}", self.music_selected));
        });
    }
}