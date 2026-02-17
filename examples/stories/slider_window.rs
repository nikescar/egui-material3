#![doc(hidden)]

use crate::{slider, range_slider, MaterialButton, RangeValues, SliderInteraction, ThumbShape};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct SliderWindow {
    pub open: bool,
    disabled: bool,
    
    // Basic slider values
    continuous_value: f32,
    labeled_value: f32,
    stepped_value: f32,
    
    // Range slider values
    range_values: RangeValues,
    price_range: RangeValues,
    time_range: RangeValues,
    age_range: RangeValues,
    
    // Real-world examples
    volume_value: f32,
    brightness_value: f32,
    contrast_value: f32,
    saturation_value: f32,
    warmth_value: f32,
    
    // Audio/video player simulation
    playback_position: f32,
    buffer_position: f32,
    playback_speed: f32,
    
    // Interaction modes
    tap_and_slide_value: f32,
    tap_only_value: f32,
    slide_only_value: f32,
    slide_thumb_value: f32,
    
    // Value indicators
    indicator_continuous: f32,
    indicator_discrete: f32,
    
    // Thumb shapes
    round_value: f32,
    handle_value: f32,
    
    // Temperature slider
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
            range_values: RangeValues::new(20.0, 80.0),
            price_range: RangeValues::new(100.0, 500.0),
            time_range: RangeValues::new(9.0, 17.0),
            age_range: RangeValues::new(25.0, 45.0),
            volume_value: 65.0,
            brightness_value: 80.0,
            contrast_value: 50.0,
            saturation_value: 50.0,
            warmth_value: 0.0,
            playback_position: 45.0,
            buffer_position: 60.0,
            playback_speed: 1.0,
            tap_and_slide_value: 50.0,
            tap_only_value: 50.0,
            slide_only_value: 50.0,
            slide_thumb_value: 50.0,
            indicator_continuous: 50.0,
            indicator_discrete: 50.0,
            round_value: 50.0,
            handle_value: 50.0,
            temperature_value: 72.0,
        }
    }
}

impl SliderWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Slider Stories")
            .open(&mut open)
            .default_size([900.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_sliders(ui);
                    ui.add_space(20.0);
                    self.render_range_sliders(ui);
                    ui.add_space(20.0);
                    self.render_media_player(ui);
                    ui.add_space(20.0);
                    self.render_image_editing(ui);
                    ui.add_space(20.0);
                    self.render_filter_examples(ui);
                    ui.add_space(20.0);
                    self.render_interaction_modes(ui);
                    ui.add_space(20.0);
                    self.render_value_indicators(ui);
                    ui.add_space(20.0);
                    self.render_thumb_shapes(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Slider Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/slider/stories/");
            }
        });

        ui.checkbox(&mut self.disabled, "Disabled");
    }

    fn render_basic_sliders(&mut self, ui: &mut egui::Ui) {
        ui.heading("Basic Sliders");

        ui.vertical(|ui| {
            ui.label("Continuous:");
            let mut continuous_slider = slider(&mut self.continuous_value, 0.0..=100.0)
                .width(300.0);
            if self.disabled {
                continuous_slider = continuous_slider.enabled(false);
            }
            ui.add(continuous_slider);

            ui.add_space(10.0);

            ui.label("Labeled:");
            let mut labeled_slider = slider(&mut self.labeled_value, 0.0..=100.0)
                .text("Value")
                .width(300.0);
            if self.disabled {
                labeled_slider = labeled_slider.enabled(false);
            }
            ui.add(labeled_slider);

            ui.add_space(10.0);

            ui.label("Tick marks (discrete):");
            let mut stepped_slider = slider(&mut self.stepped_value, 0.0..=100.0)
                .step(10.0)
                .text("Stepped")
                .width(300.0);
            if self.disabled {
                stepped_slider = stepped_slider.enabled(false);
            }
            ui.add(stepped_slider);
        });
    }

    fn render_range_sliders(&mut self, ui: &mut egui::Ui) {
        ui.heading("Range Sliders - Select Value Ranges");

        ui.vertical(|ui| {
            ui.label("Basic Range:");
            let mut range_slider = range_slider(&mut self.range_values, 0.0..=100.0)
                .text("Range")
                .width(300.0);
            if self.disabled {
                range_slider = range_slider.enabled(false);
            }
            ui.add(range_slider);

            ui.add_space(10.0);

            ui.label(format!(
                "Selected range: {:.0} - {:.0}",
                self.range_values.start, self.range_values.end
            ));
        });
    }

    fn render_media_player(&mut self, ui: &mut egui::Ui) {
        ui.heading("📺 Audio/Video Player with Buffering");

        ui.push_id("media_player", |ui| {
            ui.horizontal(|ui| {
                ui.label("⏯");
                ui.vertical(|ui| {
                    ui.label("Playback Position:");
                    
                    // Simulate buffer loading
                    if self.buffer_position < 100.0 && self.playback_position > self.buffer_position - 10.0 {
                        self.buffer_position = (self.buffer_position + 0.5).min(100.0);
                    }
                    
                    let mut player_slider = slider(&mut self.playback_position, 0.0..=100.0)
                        .secondary_track_value(self.buffer_position)
                        .show_value(false)
                        .width(400.0);
                    
                    if self.disabled {
                        player_slider = player_slider.enabled(false);
                    }
                    
                    ui.add(player_slider);
                    
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:{:02}", 
                            (self.playback_position as i32) / 60,
                            (self.playback_position as i32) % 60
                        ));
                        ui.label("/");
                        ui.label("100:00");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("Buffer: {:.0}%", self.buffer_position));
                        });
                    });
                });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("⏩ Playback Speed:");
                let mut speed_slider = slider(&mut self.playback_speed, 0.25..=2.0)
                    .step(0.25)
                    .width(200.0);
                if self.disabled {
                    speed_slider = speed_slider.enabled(false);
                }
                ui.add(speed_slider);
                ui.label(format!("{:.2}x", self.playback_speed));
            });
        });
    }

    fn render_image_editing(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎨 Image Editing Controls");

        ui.push_id("image_editing", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("💡 Brightness");
                    let mut brightness = slider(&mut self.brightness_value, 0.0..=100.0)
                        .show_value_indicator(true)
                        .width(250.0);
                    if self.disabled {
                        brightness = brightness.enabled(false);
                    }
                    ui.add(brightness);
                    ui.label(format!("{}%", self.brightness_value as i32));

                    ui.add_space(5.0);

                    ui.label("◐ Contrast");
                    let mut contrast = slider(&mut self.contrast_value, 0.0..=100.0)
                        .show_value_indicator(true)
                        .width(250.0);
                    if self.disabled {
                        contrast = contrast.enabled(false);
                    }
                    ui.add(contrast);
                    ui.label(format!("{}%", self.contrast_value as i32));
                });

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label("🎨 Saturation");
                    let mut saturation = slider(&mut self.saturation_value, 0.0..=100.0)
                        .show_value_indicator(true)
                        .width(250.0);
                    if self.disabled {
                        saturation = saturation.enabled(false);
                    }
                    ui.add(saturation);
                    ui.label(format!("{}%", self.saturation_value as i32));

                    ui.add_space(5.0);

                    ui.label("🌡 Warmth");
                    let mut warmth = slider(&mut self.warmth_value, -50.0..=50.0)
                        .show_value_indicator(true)
                        .width(250.0);
                    if self.disabled {
                        warmth = warmth.enabled(false);
                    }
                    ui.add(warmth);
                    let warmth_text = if self.warmth_value > 0.0 {
                        format!("+{:.0} (warmer)", self.warmth_value)
                    } else if self.warmth_value < 0.0 {
                        format!("{:.0} (cooler)", self.warmth_value)
                    } else {
                        "0 (neutral)".to_string()
                    };
                    ui.label(warmth_text);
                });
            });

            ui.add_space(10.0);

            if ui.button("Reset All").clicked() {
                self.brightness_value = 50.0;
                self.contrast_value = 50.0;
                self.saturation_value = 50.0;
                self.warmth_value = 0.0;
            }
        });
    }

    fn render_filter_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔍 Filter & Selection Examples");

        ui.push_id("filters", |ui| {
            ui.label("💰 Price Range");
            let mut price_slider = range_slider(&mut self.price_range, 0.0..=1000.0)
                .step(10.0)
                .width(350.0);
            if self.disabled {
                price_slider = price_slider.enabled(false);
            }
            ui.add(price_slider);
            ui.label(format!(
                "${:.0} - ${:.0}",
                self.price_range.start, self.price_range.end
            ));

            ui.add_space(10.0);

            ui.label("🕐 Time Range (Hours)");
            let mut time_slider = range_slider(&mut self.time_range, 0.0..=24.0)
                .step(1.0)
                .width(350.0);
            if self.disabled {
                time_slider = time_slider.enabled(false);
            }
            ui.add(time_slider);
            ui.label(format!(
                "{}:00 - {}:00",
                self.time_range.start as i32, self.time_range.end as i32
            ));

            ui.add_space(10.0);

            ui.label("👤 Age Range");
            let mut age_slider = range_slider(&mut self.age_range, 18.0..=80.0)
                .step(1.0)
                .min_separation(5.0)
                .width(350.0);
            if self.disabled {
                age_slider = age_slider.enabled(false);
            }
            ui.add(age_slider);
            ui.label(format!(
                "{:.0} - {:.0} years old",
                self.age_range.start, self.age_range.end
            ));
        });
    }

    fn render_interaction_modes(&mut self, ui: &mut egui::Ui) {
        ui.heading("🖱 Interaction Modes");

        ui.push_id("interaction_modes", |ui| {
            ui.label("Tap & Slide (default) - Click anywhere or drag:");
            let mut tap_and_slide = slider(&mut self.tap_and_slide_value, 0.0..=100.0)
                .interaction_mode(SliderInteraction::TapAndSlide)
                .width(300.0);
            if self.disabled {
                tap_and_slide = tap_and_slide.enabled(false);
            }
            ui.add(tap_and_slide);

            ui.add_space(5.0);

            ui.label("Tap Only - Only clicking sets value:");
            let mut tap_only = slider(&mut self.tap_only_value, 0.0..=100.0)
                .interaction_mode(SliderInteraction::TapOnly)
                .width(300.0);
            if self.disabled {
                tap_only = tap_only.enabled(false);
            }
            ui.add(tap_only);

            ui.add_space(5.0);

            ui.label("Slide Only - Only dragging from current position:");
            let mut slide_only = slider(&mut self.slide_only_value, 0.0..=100.0)
                .interaction_mode(SliderInteraction::SlideOnly)
                .width(300.0);
            if self.disabled {
                slide_only = slide_only.enabled(false);
            }
            ui.add(slide_only);

            ui.add_space(5.0);

            ui.label("Slide Thumb - Only drag the thumb itself:");
            let mut slide_thumb = slider(&mut self.slide_thumb_value, 0.0..=100.0)
                .interaction_mode(SliderInteraction::SlideThumb)
                .width(300.0);
            if self.disabled {
                slide_thumb = slide_thumb.enabled(false);
            }
            ui.add(slide_thumb);
        });
    }

    fn render_value_indicators(&mut self, ui: &mut egui::Ui) {
      ui.heading("📊 Value Indicators");

        ui.push_id("value_indicators", |ui| {
            ui.label("Continuous with Value Indicator (drag to see):");
            let mut continuous_indicator = slider(&mut self.indicator_continuous, 0.0..=100.0)
                .show_value_indicator(true)
                .width(300.0);
            if self.disabled {
                continuous_indicator = continuous_indicator.enabled(false);
            }
            ui.add(continuous_indicator);

            ui.add_space(10.0);

            ui.label("Discrete with Value Indicator (drag to see):");
            let mut discrete_indicator = slider(&mut self.indicator_discrete, 0.0..=100.0)
                .step(10.0)
                .show_value_indicator(true)
                .width(300.0);
            if self.disabled {
                discrete_indicator = discrete_indicator.enabled(false);
            }
            ui.add(discrete_indicator);
        });
    }

    fn render_thumb_shapes(&mut self, ui: &mut egui::Ui) {
        ui.heading("👆 Thumb Shapes");

        ui.push_id("thumb_shapes", |ui| {
            ui.label("Round Thumb (Classic):");
            let mut round_slider = slider(&mut self.round_value, 0.0..=100.0)
                .thumb_shape(ThumbShape::Round)
                .width(300.0);
            if self.disabled {
                round_slider = round_slider.enabled(false);
            }
            ui.add(round_slider);

            ui.add_space(10.0);

            ui.label("Handle Thumb (Material 3 2024):");
            let mut handle_slider = slider(&mut self.handle_value, 0.0..=100.0)
                .thumb_shape(ThumbShape::Handle)
                .width(300.0);
            if self.disabled {
                handle_slider = handle_slider.enabled(false);
            }
            ui.add(handle_slider);

            ui.add_space(10.0);

            ui.label("🌡️ Temperature Control (Handle Thumb):");
            let temp_emoji = if self.temperature_value < 60.0 {
                "🥶"
            } else if self.temperature_value > 78.0 {
                "🥵"
            } else {
                "😊"
            };
            
            ui.horizontal(|ui| {
                ui.label(temp_emoji);
                let mut temp_slider = slider(&mut self.temperature_value, 50.0..=90.0)
                    .step(1.0)
                    .thumb_shape(ThumbShape::Handle)
                    .show_value_indicator(true)
                    .width(280.0);
                if self.disabled {
                    temp_slider = temp_slider.enabled(false);
                }
                ui.add(temp_slider);
                ui.label(format!("{}°F", self.temperature_value as i32));
            });
        });
    }
}
