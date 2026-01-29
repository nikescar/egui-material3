#![doc(hidden)]

use crate::slider;
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct SliderWindow {
    pub open: bool,
    disabled: bool,
    // Slider values
    continuous_value: f32,
    labeled_value: f32,
    stepped_value: f32,
    range_start: f32,
    range_end: f32,
    range_labeled_start: f32,
    range_labeled_end: f32,
    range_stepped_start: f32,
    range_stepped_end: f32,
    custom_value_start: f32,
    custom_value_end: f32,
    volume_value: f32,
    brightness_value: f32,
    temperature_value: f32,
}

impl Default for SliderWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            continuous_value: 50.0,
            labeled_value: 30.0,
            stepped_value: 40.0,
            range_start: 20.0,
            range_end: 80.0,
            range_labeled_start: 25.0,
            range_labeled_end: 75.0,
            range_stepped_start: 30.0,
            range_stepped_end: 70.0,
            custom_value_start: 2.0,
            custom_value_end: 5.0,
            volume_value: 65.0,
            brightness_value: 80.0,
            temperature_value: 72.0,
        }
    }
}

impl SliderWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Slider Stories")
            .open(&mut open)
            .default_size([700.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_single_point_sliders(ui);
                    ui.add_space(20.0);
                    self.render_range_sliders(ui);
                    ui.add_space(20.0);
                    self.render_custom_styling(ui);
                    ui.add_space(20.0);
                    self.render_practical_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Slider Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/slider/stories/");
            }
        });

        ui.checkbox(&mut self.disabled, "Disabled");
    }

    fn render_single_point_sliders(&mut self, ui: &mut egui::Ui) {
        ui.heading("Single Point Sliders");

        ui.vertical(|ui| {
            // Continuous slider
            ui.label("Continuous:");
            let mut continuous_slider = slider(&mut self.continuous_value, 0.0..=100.0);
            if self.disabled {
                continuous_slider = continuous_slider.enabled(false);
            }
            ui.add(continuous_slider);

            ui.add_space(10.0);

            // Labeled slider
            ui.label("Labeled:");
            let mut labeled_slider = slider(&mut self.labeled_value, 0.0..=100.0).text("Value");
            if self.disabled {
                labeled_slider = labeled_slider.enabled(false);
            }
            ui.add(labeled_slider);

            ui.add_space(10.0);

            // Slider with tick marks
            ui.label("Tick marks:");
            let mut stepped_slider = slider(&mut self.stepped_value, 0.0..=100.0)
                .step(10.0)
                .text("Stepped");
            if self.disabled {
                stepped_slider = stepped_slider.enabled(false);
            }
            ui.add(stepped_slider);
        });
    }

    fn render_range_sliders(&mut self, ui: &mut egui::Ui) {
        ui.heading("Range Sliders");

        ui.vertical(|ui| {
            // Basic range
            ui.label("Range:");
            ui.horizontal(|ui| {
                let mut start_slider = slider(&mut self.range_start, 0.0..=100.0).text("Start");
                let mut end_slider = slider(&mut self.range_end, 0.0..=100.0).text("End");

                if self.disabled {
                    start_slider = start_slider.enabled(false);
                    end_slider = end_slider.enabled(false);
                }

                ui.add(start_slider);
                ui.add(end_slider);
            });

            // Ensure proper order
            if self.range_start > self.range_end {
                std::mem::swap(&mut self.range_start, &mut self.range_end);
            }

            ui.add_space(10.0);

            // Labeled range
            ui.label("Labeled Range:");
            ui.horizontal(|ui| {
                let mut labeled_start =
                    slider(&mut self.range_labeled_start, 0.0..=100.0).text("Min");
                let mut labeled_end = slider(&mut self.range_labeled_end, 0.0..=100.0).text("Max");

                if self.disabled {
                    labeled_start = labeled_start.enabled(false);
                    labeled_end = labeled_end.enabled(false);
                }

                ui.add(labeled_start);
                ui.add(labeled_end);
            });

            // Ensure proper order
            if self.range_labeled_start > self.range_labeled_end {
                std::mem::swap(&mut self.range_labeled_start, &mut self.range_labeled_end);
            }

            ui.add_space(10.0);

            // Stepped range with tick marks
            ui.label("Tick marks Range:");
            ui.horizontal(|ui| {
                let mut stepped_start = slider(&mut self.range_stepped_start, 0.0..=100.0)
                    .step(10.0)
                    .text("Start");
                let mut stepped_end = slider(&mut self.range_stepped_end, 0.0..=100.0)
                    .step(10.0)
                    .text("End");

                if self.disabled {
                    stepped_start = stepped_start.enabled(false);
                    stepped_end = stepped_end.enabled(false);
                }

                ui.add(stepped_start);
                ui.add(stepped_end);
            });

            // Ensure proper order
            if self.range_stepped_start > self.range_stepped_end {
                std::mem::swap(&mut self.range_stepped_start, &mut self.range_stepped_end);
            }
        });
    }

    fn render_custom_styling(&mut self, ui: &mut egui::Ui) {
        ui.heading("Custom Styling");

        ui.label("Custom styles (mood selector):");

        // Custom emoji slider simulation
        ui.horizontal(|ui| {
            let start_emoji = self.get_mood_emoji(self.custom_value_start);
            let end_emoji = self.get_mood_emoji(self.custom_value_end);

            let mut custom_start = slider(&mut self.custom_value_start, 0.0..=6.0)
                .step(1.0)
                .text(&format!("{}", start_emoji));
            let mut custom_end = slider(&mut self.custom_value_end, 0.0..=6.0)
                .step(1.0)
                .text(&format!("{}", end_emoji));

            if self.disabled {
                custom_start = custom_start.enabled(false);
                custom_end = custom_end.enabled(false);
            }

            ui.add(custom_start);
            ui.add(custom_end);
        });

        // Ensure proper order
        if self.custom_value_start > self.custom_value_end {
            std::mem::swap(&mut self.custom_value_start, &mut self.custom_value_end);
        }

        ui.label(format!(
            "Mood range: {} to {}",
            self.get_mood_emoji(self.custom_value_start),
            self.get_mood_emoji(self.custom_value_end)
        ));
    }

    fn render_practical_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Practical Examples");

        ui.vertical(|ui| {
            // Volume control
            ui.horizontal(|ui| {
                ui.label("🔊 Volume:");
                let volume_text = format!("{}%", self.volume_value as i32);
                let mut volume_slider =
                    slider(&mut self.volume_value, 0.0..=100.0).text(&volume_text);
                if self.disabled {
                    volume_slider = volume_slider.enabled(false);
                }
                ui.add(volume_slider);
            });

            ui.add_space(10.0);

            // Brightness control
            ui.horizontal(|ui| {
                ui.label("💡 Brightness:");
                let brightness_text = format!("{}%", self.brightness_value as i32);
                let mut brightness_slider =
                    slider(&mut self.brightness_value, 0.0..=100.0).text(&brightness_text);
                if self.disabled {
                    brightness_slider = brightness_slider.enabled(false);
                }
                ui.add(brightness_slider);
            });

            ui.add_space(10.0);

            // Temperature control
            ui.horizontal(|ui| {
                ui.label("🌡️ Temperature:");
                let temp_text = format!("{}°F", self.temperature_value as i32);
                let mut temp_slider = slider(&mut self.temperature_value, 50.0..=90.0)
                    .step(1.0)
                    .text(&temp_text);
                if self.disabled {
                    temp_slider = temp_slider.enabled(false);
                }
                ui.add(temp_slider);
            });
        });

        ui.add_space(10.0);

        // Display all current values
        ui.separator();
        ui.label("Current Values:");
        ui.horizontal(|ui| {
            ui.label(format!("Continuous: {:.1}", self.continuous_value));
            ui.label(format!("Labeled: {:.1}", self.labeled_value));
            ui.label(format!("Stepped: {:.1}", self.stepped_value));
        });
        ui.horizontal(|ui| {
            ui.label(format!(
                "Range: {:.1} - {:.1}",
                self.range_start, self.range_end
            ));
            ui.label(format!("Volume: {:.0}%", self.volume_value));
            ui.label(format!("Brightness: {:.0}%", self.brightness_value));
        });
    }

    fn get_mood_emoji(&self, value: f32) -> &'static str {
        let moods = ["🤬", "😡", "😔", "😐", "😌", "😁", "🤪"];
        let index = (value.round() as usize).min(moods.len() - 1);
        moods[index]
    }
}
