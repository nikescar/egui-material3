#![doc(hidden)]

use egui_material3::material_symbol::{ICON_PAUSE, ICON_PLAY_ARROW};
use crate::{circular_progress, icon_button_standard, linear_progress, MaterialButton};
use eframe::egui::{self, Color32, Vec2, Window};

#[doc(hidden)]
pub struct ProgressWindow {
    pub open: bool,
    value: f32,
    max: f32,
    buffer: f32,
    indeterminate: bool,
    four_color: bool,
    track_height: f32,
    stroke_width: f32,
    track_gap: f32,
    stop_indicator_radius: f32,
    border_radius: f32,
    custom_theme: bool,
    // Component demo states
    loading_button_active: bool,
    player_progress: f32,
    player_playing: bool,
    // Loading demo state (inspired by about.dart CircularProgressIndicator usage)
    content_loading: bool,
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
            track_height: 4.0,
            stroke_width: 4.0,
            track_gap: 4.0,
            stop_indicator_radius: 2.0,
            border_radius: 2.0,
            custom_theme: false,
            loading_button_active: false,
            player_progress: 0.3,
            player_playing: false,
            content_loading: true,
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
                    self.render_loading_demo(ui);
                    ui.add_space(20.0);
                    self.render_component_demos(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Progress Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/progress/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Value:");
            ui.add(egui::Slider::new(&mut self.value, 0.0..=1.0).step_by(0.01));
        });

        ui.horizontal(|ui| {
            ui.label("Max:");
            ui.add(egui::Slider::new(&mut self.max, 0.1..=2.0).step_by(0.1));
        });

        ui.horizontal(|ui| {
            ui.label("Buffer (Linear):");
            ui.add(egui::Slider::new(&mut self.buffer, 0.0..=1.0).step_by(0.01));
        });

        ui.checkbox(&mut self.indeterminate, "Indeterminate");
        ui.checkbox(&mut self.four_color, "Four Color");
        ui.checkbox(&mut self.custom_theme, "Custom Theme");

        ui.separator();
        ui.label("Style Controls:");

        ui.horizontal(|ui| {
            ui.label("Track Height (Linear):");
            ui.add(egui::Slider::new(&mut self.track_height, 1.0..=20.0).step_by(1.0));
        });

        ui.horizontal(|ui| {
            ui.label("Stroke Width (Circular):");
            ui.add(egui::Slider::new(&mut self.stroke_width, 1.0..=10.0).step_by(0.5));
        });

        ui.horizontal(|ui| {
            ui.label("Track Gap:");
            ui.add(egui::Slider::new(&mut self.track_gap, 0.0..=10.0).step_by(0.5));
        });

        ui.horizontal(|ui| {
            ui.label("Stop Indicator Radius:");
            ui.add(egui::Slider::new(&mut self.stop_indicator_radius, 0.0..=6.0).step_by(0.5));
        });

        ui.horizontal(|ui| {
            ui.label("Border Radius (Linear):");
            ui.add(egui::Slider::new(&mut self.border_radius, 0.0..=10.0).step_by(0.5));
        });
    }

    fn render_linear_progress(&mut self, ui: &mut egui::Ui) {
        ui.heading("Linear Progress");

        // Determinate with all features
        ui.label("Determinate:");
        let progress = linear_progress()
            .value(self.value)
            .max(self.max)
            .buffer(self.buffer)
            .width(400.0)
            .height(self.track_height)
            .track_gap(self.track_gap)
            .stop_indicator_radius(self.stop_indicator_radius)
            .border_radius(self.border_radius);
        ui.add(progress);

        ui.add_space(10.0);

        // Indeterminate
        ui.label("Indeterminate:");
        let mut indet_progress = linear_progress()
            .indeterminate(true)
            .width(400.0)
            .height(self.track_height)
            .track_gap(self.track_gap)
            .border_radius(self.border_radius);
        if self.four_color {
            indet_progress = indet_progress.four_color_enabled(true);
        }
        ui.add(indet_progress);

        ui.add_space(10.0);

        // Interactive (controlled by sliders)
        ui.label("Interactive (controlled by settings):");
        let mut interactive = linear_progress()
            .value(if self.indeterminate { 0.0 } else { self.value })
            .max(self.max)
            .width(400.0)
            .height(self.track_height)
            .track_gap(self.track_gap)
            .stop_indicator_radius(self.stop_indicator_radius)
            .border_radius(self.border_radius);

        if !self.indeterminate {
            interactive = interactive.buffer(self.buffer);
        }

        if self.indeterminate {
            interactive = interactive.indeterminate(true);
        }

        if self.four_color {
            interactive = interactive.four_color_enabled(true);
        }

        ui.add(interactive);

        ui.add_space(10.0);

        // Custom themed linear progress
        if self.custom_theme {
            ui.label("Custom Themed:");
            let custom = linear_progress()
                .value(if self.indeterminate { 0.0 } else { self.value })
                .indeterminate(self.indeterminate)
                .width(400.0)
                .height(self.track_height)
                .active_color(Color32::from_rgb(220, 50, 50))
                .track_color(Color32::from_rgb(255, 200, 200))
                .buffer_color(Color32::from_rgb(255, 150, 150))
                .stop_indicator_color(Color32::from_rgb(180, 30, 30))
                .track_gap(self.track_gap)
                .stop_indicator_radius(self.stop_indicator_radius)
                .border_radius(self.border_radius)
                .buffer(self.buffer);
            ui.add(custom);
        }
    }

    fn render_circular_progress(&mut self, ui: &mut egui::Ui) {
        ui.heading("Circular Progress");

        ui.horizontal(|ui| {
            // Determinate
            ui.vertical(|ui| {
                ui.label("Determinate:");
                ui.add(
                    circular_progress()
                        .value(self.value)
                        .max(self.max)
                        .stroke_width(self.stroke_width)
                        .track_gap(self.track_gap),
                );
            });

            ui.add_space(20.0);

            // Indeterminate
            ui.vertical(|ui| {
                ui.label("Indeterminate:");
                let mut indet = circular_progress()
                    .indeterminate(true)
                    .stroke_width(self.stroke_width)
                    .track_gap(self.track_gap);
                if self.four_color {
                    indet = indet.four_color_enabled(true);
                }
                ui.add(indet);
            });

            ui.add_space(20.0);

            // Interactive
            ui.vertical(|ui| {
                ui.label("Interactive:");
                let mut interactive = circular_progress()
                    .value(if self.indeterminate { 0.0 } else { self.value })
                    .max(self.max)
                    .stroke_width(self.stroke_width)
                    .track_gap(self.track_gap);
                if self.indeterminate {
                    interactive = interactive.indeterminate(true);
                }
                if self.four_color {
                    interactive = interactive.four_color_enabled(true);
                }
                ui.add(interactive);
            });

            if self.custom_theme {
                ui.add_space(20.0);
                ui.vertical(|ui| {
                    ui.label("Custom Themed:");
                    ui.add(
                        circular_progress()
                            .value(if self.indeterminate { 0.0 } else { self.value })
                            .indeterminate(self.indeterminate)
                            .stroke_width(self.stroke_width)
                            .track_gap(self.track_gap)
                            .active_color(Color32::from_rgb(50, 180, 50))
                            .track_color(Color32::from_rgb(200, 240, 200)),
                    );
                });
            }
        });
    }

    fn render_loading_demo(&mut self, ui: &mut egui::Ui) {
        ui.heading("Loading State (about.dart pattern)");
        ui.label("Circular spinner as a loading indicator, as used in Flutter's AboutDialog/LicensePage.");

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled(if self.content_loading { "Stop Loading" } else { "Start Loading" })).clicked() {
                self.content_loading = !self.content_loading;
            }
        });

        ui.add_space(8.0);

        if self.content_loading {
            // Show loading state with centered spinner (mimics about.dart lines 812, 1077)
            ui.horizontal(|ui| {
                ui.add(circular_progress().indeterminate(true).size(Vec2::splat(36.0)).stroke_width(3.0));
                ui.add_space(12.0);
                ui.label("Loading content...");
            });
        } else {
            ui.label("Content loaded successfully.");
            ui.label("This demonstrates using CircularProgressIndicator as an inline loading spinner.");
        }
    }

    fn render_component_demos(&mut self, ui: &mut egui::Ui) {
        ui.heading("Indicators in Components");

        ui.horizontal(|ui| {
            // Loading button demo
            if self.loading_button_active {
                ui.add(
                    MaterialButton::filled("Loading...")
                        .leading_icon("sync".to_string())
                        .enabled(false),
                );

                if ui.add(MaterialButton::text("Stop")).clicked() {
                    self.loading_button_active = false;
                }
            } else if ui.add(MaterialButton::filled("Load")).clicked() {
                self.loading_button_active = true;
            }

            ui.add_space(20.0);

            // Player progress demo
            ui.vertical(|ui| {
                ui.label("Player Progress:");
                ui.add(
                    circular_progress()
                        .value(self.player_progress)
                        .stroke_width(self.stroke_width)
                        .track_gap(self.track_gap),
                );

                // Play/pause button simulation
                let play_button = if self.player_playing {
                    icon_button_standard(ICON_PAUSE.to_string())
                } else {
                    icon_button_standard(ICON_PLAY_ARROW.to_string())
                };

                if ui.add(play_button).clicked() {
                    self.player_playing = !self.player_playing;
                    if self.player_playing {
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
