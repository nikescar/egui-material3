#![doc(hidden)]

use crate::MaterialButton;
#[cfg(feature = "svg_emoji")]
use crate::svg_emoji::SOLAR_ICONS;
use eframe::egui::{self, epaint::Stroke, Ui, Vec2, Window};

#[doc(hidden)]
pub struct ButtonWindow {
    pub open: bool,
    label: String,
    disabled: bool,
    soft_disabled: bool,
}

impl Default for ButtonWindow {
    fn default() -> Self {
        Self {
            open: false,
            label: String::new(),
            disabled: false,
            soft_disabled: false,
        }
    }
}

impl ButtonWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Button Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_button_variants(ui);
                    ui.add_space(20.0);
                    self.render_buttons_with_leading_icons(ui);
                    ui.add_space(20.0);
                    self.render_buttons_with_trailing_icons(ui);
                    ui.add_space(20.0);
                    self.render_buttons_with_both_icons(ui);
                    ui.add_space(20.0);
                    self.render_small_buttons(ui);
                    ui.add_space(20.0);
                    self.render_small_buttons_with_svg_icons(ui);
                    ui.add_space(20.0);
                    self.render_custom_stroke_buttons(ui);
                    ui.add_space(20.0);
                    self.render_frameless_buttons(ui);
                    ui.add_space(20.0);
                    self.render_custom_size_buttons(ui);
                    ui.add_space(20.0);
                    self.render_custom_corner_radius_buttons(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Button Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/button/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);
        });

        ui.checkbox(&mut self.disabled, "Disabled");
        ui.checkbox(&mut self.soft_disabled, "Soft Disabled");
    }

    fn is_disabled(&self) -> bool {
        self.disabled || self.soft_disabled
    }

    fn label_or<'a>(&'a self, default: &'a str) -> &'a str {
        if self.label.is_empty() {
            default
        } else {
            &self.label
        }
    }

    fn render_button_variants(&mut self, ui: &mut Ui) {
        ui.heading("Button Variants");

        let disabled = self.is_disabled();

        ui.horizontal(|ui| {
            for (label, button) in self.all_variants() {
                let button = if disabled { button.enabled(false) } else { button };
                if ui.add(button).clicked() && !disabled {
                    println!("{label} button clicked!");
                }
            }
        });
    }

    fn render_buttons_with_leading_icons(&mut self, ui: &mut Ui) {
        ui.heading("Buttons with Leading Icons");

        let disabled = self.is_disabled();

        ui.horizontal(|ui| {
            for (label, button) in self.all_variants() {
                let button = button.leading_icon("upload");
                let button = if disabled { button.enabled(false) } else { button };
                if ui.add(button).clicked() && !disabled {
                    println!("{label} button with leading icon clicked!");
                }
            }
        });
    }

    fn render_buttons_with_trailing_icons(&mut self, ui: &mut Ui) {
        ui.heading("Buttons with Trailing Icons");

        let disabled = self.is_disabled();

        ui.horizontal(|ui| {
            for (label, button) in self.all_variants() {
                let button = button.trailing_icon("arrow_forward");
                let button = if disabled { button.enabled(false) } else { button };
                if ui.add(button).clicked() && !disabled {
                    println!("{label} button with trailing icon clicked!");
                }
            }
        });
    }

    fn render_buttons_with_both_icons(&mut self, ui: &mut Ui) {
        ui.heading("Buttons with Both Icons");

        let disabled = self.is_disabled();

        ui.horizontal(|ui| {
            for (label, button) in self.all_variants() {
                let button = button
                    .leading_icon("open_in_new")
                    .trailing_icon("arrow_forward");
                let button = if disabled { button.enabled(false) } else { button };
                if ui.add(button).clicked() && !disabled {
                    println!("{label} button with both icons clicked!");
                }
            }
        });
    }

    fn render_small_buttons(&mut self, ui: &mut Ui) {
        ui.heading("Small Buttons");

        let disabled = self.is_disabled();

        ui.horizontal(|ui| {
            for (label, button) in self.all_variants() {
                let button = button.small();
                let button = if disabled { button.enabled(false) } else { button };
                if ui.add(button).clicked() && !disabled {
                    println!("{label} small button clicked!");
                }
            }
        });
    }

    fn render_small_buttons_with_svg_icons(&mut self, ui: &mut Ui) {
        ui.heading("Small Buttons with SVG Icons");

        let disabled = self.is_disabled();
                
        #[cfg(feature = "svg_emoji")]
        {
            // Leading SVG icons
            ui.label("With leading SVG icon:");
            ui.horizontal_wrapped(|ui| {
                if let Some(&star_svg) = SOLAR_ICONS.get("star") {
                    let button = MaterialButton::filled(self.label_or("Star"))
                        .small()
                        .leading_svg(star_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small filled button with SVG clicked!");
                    }
                }

                if let Some(&heart_svg) = SOLAR_ICONS.get("heart") {
                    let button = MaterialButton::outlined(self.label_or("Like"))
                        .small()
                        .leading_svg(heart_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small outlined button with SVG clicked!");
                    }
                }

                if let Some(&bookmark_svg) = SOLAR_ICONS.get("bookmark") {
                    let button = MaterialButton::filled_tonal(self.label_or("Save"))
                        .small()
                        .leading_svg(bookmark_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small tonal button with SVG clicked!");
                    }
                }

                if let Some(&settings_svg) = SOLAR_ICONS.get("settings") {
                    let button = MaterialButton::elevated(self.label_or("Settings"))
                        .small()
                        .leading_svg(settings_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small elevated button with SVG clicked!");
                    }
                }
            });

            ui.add_space(8.0);

            // Trailing SVG icons
            ui.label("With trailing SVG icon:");
            ui.horizontal_wrapped(|ui| {
                if let Some(&arrow_right_svg) = SOLAR_ICONS.get("arrow-right") {
                    let button = MaterialButton::filled(self.label_or("Next"))
                        .small()
                        .trailing_svg(arrow_right_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small button with trailing SVG clicked!");
                    }
                }

                if let Some(&share_svg) = SOLAR_ICONS.get("share") {
                    let button = MaterialButton::text(self.label_or("Share"))
                        .small()
                        .trailing_svg(share_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small text button with trailing SVG clicked!");
                    }
                }
            });

            ui.add_space(8.0);

            // Both leading and trailing SVG icons
            ui.label("With both leading and trailing SVG icons:");
            ui.horizontal_wrapped(|ui| {
                if let (Some(&star_svg), Some(&arrow_right_svg)) =
                    (SOLAR_ICONS.get("star"), SOLAR_ICONS.get("arrow-right")) {
                    let button = MaterialButton::filled(self.label_or("Featured"))
                        .small()
                        .leading_svg(star_svg)
                        .trailing_svg(arrow_right_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small button with both SVG icons clicked!");
                    }
                }

                if let (Some(&download_svg), Some(&check_svg)) =
                    (SOLAR_ICONS.get("download"), SOLAR_ICONS.get("check")) {
                    let button = MaterialButton::outlined(self.label_or("Download"))
                        .small()
                        .leading_svg(download_svg)
                        .trailing_svg(check_svg);
                    let button = if disabled { button.enabled(false) } else { button };
                    if ui.add(button).clicked() && !disabled {
                        println!("Small outlined button with both SVG icons clicked!");
                    }
                }
            });
        }
    }

    fn render_custom_stroke_buttons(&mut self, ui: &mut Ui) {
        ui.heading("Custom Stroke Buttons");

        ui.horizontal(|ui| {
            let _ = ui.add(
                MaterialButton::filled(self.label_or("Stroke 2px"))
                    .stroke(Stroke::new(2.0, egui::Color32::RED)),
            );
            let _ = ui.add(
                MaterialButton::outlined(self.label_or("Stroke 2px"))
                    .stroke(Stroke::new(2.0, egui::Color32::BLUE)),
            );
            let _ = ui.add(
                MaterialButton::text(self.label_or("Stroke 1px"))
                    .stroke(Stroke::new(1.0, egui::Color32::GREEN)),
            );
            let _ = ui.add(
                MaterialButton::elevated(self.label_or("Stroke 3px"))
                    .stroke(Stroke::new(3.0, egui::Color32::GOLD)),
            );
            let _ = ui.add(
                MaterialButton::filled_tonal(self.label_or("Stroke 2px"))
                    .stroke(Stroke::new(2.0, egui::Color32::from_rgb(200, 100, 255))),
            );
        });
    }

    fn render_frameless_buttons(&mut self, ui: &mut Ui) {
        ui.heading("Frameless Buttons (frame=false)");

        ui.horizontal(|ui| {
            for (label, button) in self.all_variants() {
                let button = button.frame(false);
                let _ = ui.add(button);
                ui.label(format!("({label})"));
            }
        });
    }

    fn render_custom_size_buttons(&mut self, ui: &mut Ui) {
        ui.heading("Custom Min Size Buttons");

        ui.horizontal(|ui| {
            let _ = ui.add(
                MaterialButton::filled(self.label_or("Wide"))
                    .min_size(Vec2::new(200.0, 40.0)),
            );
            let _ = ui.add(
                MaterialButton::outlined(self.label_or("Tall"))
                    .min_size(Vec2::new(80.0, 60.0)),
            );
            let _ = ui.add(
                MaterialButton::filled_tonal(self.label_or("Large"))
                    .min_size(Vec2::new(160.0, 56.0)),
            );
        });
    }

    fn render_custom_corner_radius_buttons(&mut self, ui: &mut Ui) {
        ui.heading("Custom Corner Radius Buttons");

        ui.horizontal(|ui| {
            let _ = ui.add(
                MaterialButton::filled(self.label_or("Square")).corner_radius(0),
            );
            let _ = ui.add(
                MaterialButton::outlined(self.label_or("Slight")).corner_radius(4),
            );
            let _ = ui.add(
                MaterialButton::elevated(self.label_or("Medium")).corner_radius(10),
            );
            let _ = ui.add(
                MaterialButton::filled_tonal(self.label_or("Default (20)")).corner_radius(20),
            );
            let _ = ui.add(
                MaterialButton::filled(self.label_or("Pill")).corner_radius(50),
            );
        });
    }

    fn all_variants(&self) -> Vec<(&str, MaterialButton<'_>)> {
        vec![
            ("Filled", MaterialButton::filled(self.label_or("Filled"))),
            ("Outlined", MaterialButton::outlined(self.label_or("Outlined"))),
            ("Elevated", MaterialButton::elevated(self.label_or("Elevated"))),
            ("Tonal", MaterialButton::filled_tonal(self.label_or("Tonal"))),
            ("Text", MaterialButton::text(self.label_or("Text"))),
        ]
    }
}
