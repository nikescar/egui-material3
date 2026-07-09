#![doc(hidden)]

use crate::{input, InputMode, InputSize, MaterialButton, MaterialInput};
use eframe::egui::{self, Frame, Margin, Window};

#[doc(hidden)]
pub struct InputWindow {
    pub open: bool,
    disabled: bool,
    outline_value: String,
    filled_value: String,
    plain_value: String,
    small_value: String,
    normal_value: String,
    large_value: String,
    icon_value: String,
    error_value: String,
    helper_value: String,
    char_count_value: String,
    password_value: String,
}

impl Default for InputWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            outline_value: String::new(),
            filled_value: "Filled input".to_string(),
            plain_value: String::new(),
            small_value: String::new(),
            normal_value: "Normal size".to_string(),
            large_value: String::new(),
            icon_value: String::new(),
            error_value: String::new(),
            helper_value: String::new(),
            char_count_value: String::new(),
            password_value: String::new(),
        }
    }
}

impl InputWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Input Stories")
            .open(&mut open)
            .default_size([720.0, 720.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    Frame::new()
                        .inner_margin(Margin::symmetric(16, 12))
                        .show(ui, |ui| {
                            self.render_controls(ui);
                            ui.add_space(20.0);
                            self.render_modes(ui);
                            ui.add_space(20.0);
                            self.render_sizes(ui);
                            ui.add_space(20.0);
                            self.render_with_icons(ui);
                            ui.add_space(20.0);
                            self.render_states(ui);
                        });
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Input Controls");
            if ui.add(MaterialButton::filled("Leo Input").small()).clicked() {
                let _ = webbrowser::open(
                    "https://github.com/brave/leo/tree/main/src/components/input",
                );
            }
        });
        ui.checkbox(&mut self.disabled, "Disabled");
    }

    fn render_modes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input Modes");
        ui.label("Leo FormItem modes — outline (default), filled, and plain.");

        let disabled = self.disabled;
        let width = ui.available_width().min(420.0);

        ui.add(
            MaterialInput::new(&mut self.outline_value)
                .label("Outline")
                .placeholder("Enter text")
                .mode(InputMode::Outline)
                .width(width)
                .enabled(!disabled),
        );
        ui.add_space(12.0);

        ui.add(
            MaterialInput::new(&mut self.filled_value)
                .label("Filled")
                .placeholder("Enter text")
                .mode(InputMode::Filled)
                .width(width)
                .enabled(!disabled),
        );
        ui.add_space(12.0);

        ui.add(
            MaterialInput::new(&mut self.plain_value)
                .label("Plain")
                .placeholder("Borderless input")
                .mode(InputMode::Plain)
                .width(width)
                .enabled(!disabled),
        );
    }

    fn render_sizes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input Sizes");
        ui.label("Small, normal, and large scale.");

        let disabled = self.disabled;
        let width = ui.available_width().min(420.0);

        ui.add(
            input(&mut self.small_value)
                .label("Small")
                .placeholder("Small input")
                .size(InputSize::Small)
                .width(width)
                .enabled(!disabled),
        );
        ui.add_space(12.0);

        ui.add(
            input(&mut self.normal_value)
                .label("Normal")
                .placeholder("Normal input")
                .size(InputSize::Normal)
                .width(width)
                .enabled(!disabled),
        );
        ui.add_space(12.0);

        ui.add(
            input(&mut self.large_value)
                .label("Large")
                .placeholder("Large input")
                .size(InputSize::Large)
                .width(width)
                .enabled(!disabled),
        );
    }

    fn render_with_icons(&mut self, ui: &mut egui::Ui) {
        ui.heading("With Icons");
        ui.label("Leading and trailing icons with optional character count.");

        let disabled = self.disabled;
        let width = ui.available_width().min(420.0);
        let char_len = self.char_count_value.chars().count();

        ui.add(
            MaterialInput::new(&mut self.icon_value)
                .label("Search")
                .placeholder("Search…")
                .leading_icon("search")
                .trailing_icon("close")
                .width(width)
                .enabled(!disabled),
        );
        ui.add_space(12.0);

        ui.add(
            MaterialInput::new(&mut self.char_count_value)
                .label("Character count")
                .placeholder("Type something")
                .max_chars(50)
                .extra_text(format!("{char_len}/50"))
                .width(width)
                .enabled(!disabled),
        );
    }

    fn render_states(&mut self, ui: &mut egui::Ui) {
        ui.heading("Validation & Helper Text");
        let width = ui.available_width().min(420.0);
        let disabled = self.disabled;

        ui.add(
            MaterialInput::new(&mut self.error_value)
                .label("Email")
                .placeholder("name@example.com")
                .required(true)
                .error_text("Please enter a valid email address")
                .leading_icon("mail")
                .width(width)
                .enabled(!disabled),
        );
        ui.add_space(12.0);

        ui.add(
            MaterialInput::new(&mut self.helper_value)
                .label("Username")
                .placeholder("Choose a username")
                .helper_text("Must be 3–20 characters")
                .width(width)
                .enabled(!disabled),
        );
        ui.add_space(12.0);

        ui.add(
            MaterialInput::new(&mut self.password_value)
                .label("Password")
                .placeholder("Enter password")
                .password(true)
                .leading_icon("lock")
                .width(width)
                .enabled(!disabled),
        );
    }
}
