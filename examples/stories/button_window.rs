#![doc(hidden)]

use crate::MaterialButton;
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
