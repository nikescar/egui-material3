#![doc(hidden)]

use crate::{radio, radio_group, radio_list_tile, RadioListTile, ListTileControlAffinity, MaterialButton};
use eframe::egui::{self, Window, Color32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Animal {
    Birds,
    Cats,
    Dogs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MusicGenre {
    Rock,
    Jazz,
    Classical,
    Electronic,
    Country,
}

#[doc(hidden)]
pub struct RadioWindow {
    pub open: bool,
    disabled: bool,
    // Radio states
    basic_radio_selected: Option<usize>,
    labeled_radio_selected: Option<usize>,
    animals_selected: Option<Animal>,
    music_selected: Option<MusicGenre>,
    // New examples
    toggleable_selected: Option<usize>,
    themed_selected: Option<usize>,
    string_selected: Option<String>,
    list_tile_selected: Option<usize>,
    affinity_selected: Option<usize>,
    dense_selected: Option<usize>,
}

impl Default for RadioWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            basic_radio_selected: Some(0),
            labeled_radio_selected: Some(1),
            animals_selected: Some(Animal::Dogs),
            music_selected: None,
            toggleable_selected: Some(1),
            themed_selected: Some(0),
            string_selected: Some("option_a".to_string()),
            list_tile_selected: Some(0),
            affinity_selected: Some(0),
            dense_selected: Some(1),
        }
    }
}

impl RadioWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Radio Stories")
            .open(&mut open)
            .default_size([700.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_radios(ui);
                    ui.add_space(20.0);
                    self.render_labeled_radios(ui);
                    ui.add_space(20.0);
                    self.render_radio_groups(ui);
                    ui.add_space(20.0);
                    self.render_toggleable_radios(ui);
                    ui.add_space(20.0);
                    self.render_themed_radios(ui);
                    ui.add_space(20.0);
                    self.render_string_radios(ui);
                    ui.add_space(20.0);
                    self.render_radio_list_tiles(ui);
                    ui.add_space(20.0);
                    self.render_control_affinity(ui);
                    ui.add_space(20.0);
                    self.render_dense_list_tiles(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Radio Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
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
        ui.heading("Radio Groups with Enums");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Animals:");
                let mut animals_group = radio_group(&mut self.animals_selected)
                    .option(Animal::Birds, "Birds")
                    .option(Animal::Cats, "Cats")
                    .option(Animal::Dogs, "Dogs");

                if self.disabled {
                    animals_group = animals_group.enabled(false);
                }

                ui.add(animals_group);
            });

            ui.add_space(40.0);

            ui.vertical(|ui| {
                ui.label("Music Genres:");
                let mut music_group = radio_group(&mut self.music_selected)
                    .option(MusicGenre::Rock, "Rock")
                    .option(MusicGenre::Jazz, "Jazz")
                    .option(MusicGenre::Classical, "Classical")
                    .option(MusicGenre::Electronic, "Electronic")
                    .option(MusicGenre::Country, "Country");

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
            ui.label(format!("Animals: {:?}", self.animals_selected));
            ui.label(format!("Music: {:?}", self.music_selected));
        });
    }
    fn render_toggleable_radios(&mut self, ui: &mut egui::Ui) {
        ui.heading("Toggleable Radios");
        ui.label("Click a selected radio to deselect it:");

        ui.vertical(|ui| {
            let mut radio1 = radio(&mut self.toggleable_selected, 0, "First option")
                .toggleable(true);
            if self.disabled {
                radio1 = radio1.enabled(false);
            }
            ui.add(radio1);

            let mut radio2 = radio(&mut self.toggleable_selected, 1, "Second option")
                .toggleable(true);
            if self.disabled {
                radio2 = radio2.enabled(false);
            }
            ui.add(radio2);

            let mut radio3 = radio(&mut self.toggleable_selected, 2, "Third option")
                .toggleable(true);
            if self.disabled {
                radio3 = radio3.enabled(false);
            }
            ui.add(radio3);
        });

        ui.label(format!("Selected: {:?}", self.toggleable_selected));
    }

    fn render_themed_radios(&mut self, ui: &mut egui::Ui) {
        ui.heading("Themed Radios");
        ui.label("Custom colors and styling:");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Custom Fill Colors:");
                
                let mut radio1 = radio(&mut self.themed_selected, 0, "Red")
                    .fill_color(Color32::from_rgb(220, 50, 50));
                if self.disabled {
                    radio1 = radio1.enabled(false);
                }
                ui.add(radio1);

                let mut radio2 = radio(&mut self.themed_selected, 1, "Green")
                    .fill_color(Color32::from_rgb(50, 200, 50));
                if self.disabled {
                    radio2 = radio2.enabled(false);
                }
                ui.add(radio2);

                let mut radio3 = radio(&mut self.themed_selected, 2, "Blue")
                    .fill_color(Color32::from_rgb(50, 120, 220));
                if self.disabled {
                    radio3 = radio3.enabled(false);
                }
                ui.add(radio3);
            });

            ui.add_space(40.0);

            ui.vertical(|ui| {
                ui.label("Custom Radius:");
                
                let mut radio1 = radio(&mut self.themed_selected, 3, "Small inner")
                    .inner_radius(3.0);
                if self.disabled {
                    radio1 = radio1.enabled(false);
                }
                ui.add(radio1);

                let mut radio2 = radio(&mut self.themed_selected, 4, "Large inner")
                    .inner_radius(7.0);
                if self.disabled {
                    radio2 = radio2.enabled(false);
                }
                ui.add(radio2);

                let mut radio3 = radio(&mut self.themed_selected, 5, "Custom splash")
                    .splash_radius(20.0);
                if self.disabled {
                    radio3 = radio3.enabled(false);
                }
                ui.add(radio3);
            });
        });
    }

    fn render_string_radios(&mut self, ui: &mut egui::Ui) {
        ui.heading("String-based Radios");
        ui.label("Using String values instead of integers:");

        ui.vertical(|ui| {
            let mut radio1 = radio(&mut self.string_selected, "option_a".to_string(), "Option A");
            if self.disabled {
                radio1 = radio1.enabled(false);
            }
            ui.add(radio1);

            let mut radio2 = radio(&mut self.string_selected, "option_b".to_string(), "Option B");
            if self.disabled {
                radio2 = radio2.enabled(false);
            }
            ui.add(radio2);

            let mut radio3 = radio(&mut self.string_selected, "option_c".to_string(), "Option C");
            if self.disabled {
                radio3 = radio3.enabled(false);
            }
            ui.add(radio3);
        });

        ui.label(format!("Selected: {:?}", self.string_selected));
    }

    fn render_radio_list_tiles(&mut self, ui: &mut egui::Ui) {
        ui.heading("Radio List Tiles");
        ui.label("Full-width interactive tiles with title and subtitle:");

        ui.vertical(|ui| {
            let mut tile1 = radio_list_tile(&mut self.list_tile_selected, 0)
                .title("Wi-Fi")
                .subtitle("Connect to wireless networks");
            if self.disabled {
                tile1 = tile1.enabled(false);
            }
            ui.add(tile1);

            let mut tile2 = radio_list_tile(&mut self.list_tile_selected, 1)
                .title("Bluetooth")
                .subtitle("Connect to nearby devices");
            if self.disabled {
                tile2 = tile2.enabled(false);
            }
            ui.add(tile2);

            let mut tile3 = radio_list_tile(&mut self.list_tile_selected, 2)
                .title("Mobile Data")
                .subtitle("Use cellular network connection");
            if self.disabled {
                tile3 = tile3.enabled(false);
            }
            ui.add(tile3);

            let mut tile4 = radio_list_tile(&mut self.list_tile_selected, 3)
                .title("Airplane Mode")
                .subtitle("Disable all wireless connections");
            if self.disabled {
                tile4 = tile4.enabled(false);
            }
            ui.add(tile4);
        });
    }

    fn render_control_affinity(&mut self, ui: &mut egui::Ui) {
        ui.heading("Control Affinity");
        ui.label("Radio button position (leading vs trailing):");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Leading (default):");
                
                let mut tile1 = radio_list_tile(&mut self.affinity_selected, 0)
                    .title("Option 1")
                    .subtitle("Radio on the left")
                    .control_affinity(ListTileControlAffinity::Leading);
                if self.disabled {
                    tile1 = tile1.enabled(false);
                }
                ui.add(tile1);

                let mut tile2 = radio_list_tile(&mut self.affinity_selected, 1)
                    .title("Option 2")
                    .subtitle("Radio on the left")
                    .control_affinity(ListTileControlAffinity::Leading);
                if self.disabled {
                    tile2 = tile2.enabled(false);
                }
                ui.add(tile2);
            });

            ui.add_space(20.0);

            ui.vertical(|ui| {
                ui.label("Trailing:");
                
                let mut tile3 = radio_list_tile(&mut self.affinity_selected, 2)
                    .title("Option 3")
                    .subtitle("Radio on the right")
                    .control_affinity(ListTileControlAffinity::Trailing);
                if self.disabled {
                    tile3 = tile3.enabled(false);
                }
                ui.add(tile3);

                let mut tile4 = radio_list_tile(&mut self.affinity_selected, 3)
                    .title("Option 4")
                    .subtitle("Radio on the right")
                    .control_affinity(ListTileControlAffinity::Trailing);
                if self.disabled {
                    tile4 = tile4.enabled(false);
                }
                ui.add(tile4);
            });
        });
    }

    fn render_dense_list_tiles(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dense List Tiles");
        ui.label("Compact radio list tiles:");

        ui.vertical(|ui| {
            let mut tile1 = radio_list_tile(&mut self.dense_selected, 0)
                .title("Compact Option 1")
                .dense(true);
            if self.disabled {
                tile1 = tile1.enabled(false);
            }
            ui.add(tile1);

            let mut tile2 = radio_list_tile(&mut self.dense_selected, 1)
                .title("Compact Option 2")
                .dense(true);
            if self.disabled {
                tile2 = tile2.enabled(false);
            }
            ui.add(tile2);

            let mut tile3 = radio_list_tile(&mut self.dense_selected, 2)
                .title("Compact with subtitle")
                .subtitle("This is still quite compact")
                .dense(true);
            if self.disabled {
                tile3 = tile3.enabled(false);
            }
            ui.add(tile3);
        });
    }}
