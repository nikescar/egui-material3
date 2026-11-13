#![doc(hidden)]

use crate::{MaterialButton, MaterialInput};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct InputWindow {
    pub open: bool,

    // Basic inputs
    basic_filled: String,
    basic_outlined: String,

    // With label
    labeled_text: String,
    labeled_email: String,
    labeled_password: String,

    // With placeholder
    placeholder_text: String,

    // With helper text
    helper_text_input: String,

    // With error
    error_input: String,
    email_input: String,

    // With icons
    icon_search: String,
    icon_email: String,
    icon_password: String,
    icon_phone: String,

    // With character counter
    counter_input: String,
    counter_limited: String,

    // Password
    password_text: String,
    confirm_password: String,

    // States
    disabled_filled: String,
    disabled_outlined: String,
    readonly_text: String,

    // Validation examples
    required_field: String,
    email_field: String,
    url_field: String,
    number_field: String,

    // Test field
    test_text: String,
}

impl Default for InputWindow {
    fn default() -> Self {
        Self {
            open: false,
            basic_filled: String::new(),
            basic_outlined: String::new(),
            labeled_text: String::new(),
            labeled_email: String::new(),
            labeled_password: String::new(),
            placeholder_text: String::new(),
            helper_text_input: String::new(),
            error_input: String::new(),
            email_input: String::new(),
            icon_search: String::new(),
            icon_email: String::new(),
            icon_password: String::new(),
            icon_phone: String::new(),
            counter_input: String::new(),
            counter_limited: String::new(),
            password_text: String::new(),
            confirm_password: String::new(),
            disabled_filled: "Disabled input".to_string(),
            disabled_outlined: "Disabled outlined".to_string(),
            readonly_text: "Read-only value".to_string(),
            required_field: String::new(),
            email_field: String::new(),
            url_field: String::new(),
            number_field: String::new(),
            test_text: String::new(),
        }
    }
}

impl InputWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Input Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Material Design 3 Text Input Component");
                    ui.add_space(10.0);

                    self.render_controls(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_basic_inputs(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_labeled_inputs(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_helper_text_examples(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_error_examples(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_icon_examples(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_character_counter(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_password_examples(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_state_examples(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    self.render_validation_examples(ui);
                    ui.add_space(20.0);

                    ui.add(
                        egui::TextEdit::singleline(&mut self.test_text)
                            .hint_text("Search here...") // Placeholder text
                            .text_color(egui::Color32::from_rgb(0, 255, 0)) // Change active text color
                            .background_color(egui::Color32::BLACK) // Box background
                            .margin(egui::vec2(10.0, 10.0)) // Padding around text
                            .font(egui::FontId::monospace(16.0)) // Custom text font/size
                            .frame(true), // False strips the background border entirely
                    );
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Input Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://m3.material.io/components/text-fields/specs");
            }

            ui.add_space(10.0);

            if ui
                .add(MaterialButton::outlined("Angular Examples").small())
                .clicked()
            {
                let _ = webbrowser::open("https://material.angular.dev/components/input/examples");
            }
        });

        ui.add_space(10.0);
        ui.label("Material Design 3 text input fields with filled and outlined variants");
    }

    fn render_basic_inputs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Basic Input Fields");
        ui.label("Simple text inputs without labels or extra features");
        ui.add_space(10.0);

        ui.label("Filled variant (default):");
        ui.add(MaterialInput::filled(&mut self.basic_filled));
        ui.add_space(10.0);

        ui.label("Outlined variant:");
        ui.add(MaterialInput::outlined(&mut self.basic_outlined));
    }

    fn render_labeled_inputs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input Fields with Labels");
        ui.label("Labels float above the input when focused or filled");
        ui.add_space(10.0);

        ui.add(MaterialInput::filled(&mut self.labeled_text).label("Full Name"));
        ui.add_space(10.0);

        ui.add(MaterialInput::outlined(&mut self.labeled_email).label("Email Address"));
        ui.add_space(10.0);

        ui.add(
            MaterialInput::filled(&mut self.labeled_password)
                .label("Password")
                .password(true),
        );
    }

    fn render_helper_text_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input with Helper Text");
        ui.label("Helper text provides additional context below the input");
        ui.add_space(10.0);

        ui.add(
            MaterialInput::filled(&mut self.helper_text_input)
                .label("Username")
                .helper_text("Choose a unique username"),
        );
        ui.add_space(10.0);

        ui.add(
            MaterialInput::outlined(&mut self.placeholder_text)
                .placeholder("Enter your message here")
                .helper_text("Optional message for the recipient"),
        );
    }

    fn render_error_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input with Error State");
        ui.label("Error state shows validation feedback");
        ui.add_space(10.0);

        let is_error_empty = self.error_input.is_empty();
        ui.add(
            MaterialInput::filled(&mut self.error_input)
                .label("Required Field")
                .error_text("This field is required")
                .is_error(is_error_empty),
        );
        ui.add_space(10.0);

        let is_invalid_email = !self.email_input.is_empty() && !self.email_input.contains('@');
        ui.add(
            MaterialInput::outlined(&mut self.email_input)
                .label("Email")
                .error_text("Please enter a valid email address")
                .is_error(is_invalid_email)
                .helper_text("example@domain.com"),
        );
    }

    fn render_icon_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input with Icons");
        ui.label("Leading and trailing icons for visual context");
        ui.add_space(10.0);

        ui.add(
            MaterialInput::filled(&mut self.icon_search)
                .label("Search")
                .leading_icon("search"),
        );
        ui.add_space(10.0);

        ui.add(
            MaterialInput::outlined(&mut self.icon_email)
                .label("Email")
                .leading_icon("mail")
                .trailing_icon("done"),
        );
        ui.add_space(10.0);

        ui.add(
            MaterialInput::filled(&mut self.icon_password)
                .label("Password")
                .leading_icon("lock")
                .password(true),
        );
        ui.add_space(10.0);

        ui.add(
            MaterialInput::outlined(&mut self.icon_phone)
                .label("Phone")
                .leading_icon("phone")
                .trailing_icon("clear"),
        );
    }

    fn render_character_counter(&mut self, ui: &mut egui::Ui) {
        ui.heading("Character Counter");
        ui.label("Track character count with optional limit");
        ui.add_space(10.0);

        ui.add(
            MaterialInput::filled(&mut self.counter_input)
                .label("Message")
                .show_character_count(true)
                .helper_text("Type your message"),
        );
        ui.add_space(10.0);

        ui.add(
            MaterialInput::outlined(&mut self.counter_limited)
                .label("Bio")
                .max_length(100)
                .show_character_count(true)
                .helper_text("Maximum 100 characters"),
        );
    }

    fn render_password_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Password Input");
        ui.label("Masked text input for sensitive data");
        ui.add_space(10.0);

        ui.add(
            MaterialInput::filled(&mut self.password_text)
                .label("Password")
                .password(true)
                .leading_icon("lock"),
        );
        ui.add_space(10.0);

        let passwords_match = !self.password_text.is_empty()
            && !self.confirm_password.is_empty()
            && self.password_text == self.confirm_password;
        let show_error = !self.confirm_password.is_empty() && !passwords_match;

        ui.add(
            MaterialInput::outlined(&mut self.confirm_password)
                .label("Confirm Password")
                .password(true)
                .leading_icon("lock")
                .trailing_icon(if passwords_match { "done" } else { "close" })
                .error_text("Passwords do not match")
                .is_error(show_error),
        );
    }

    fn render_state_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input States");
        ui.label("Disabled and read-only states");
        ui.add_space(10.0);

        ui.label("Disabled filled:");
        ui.add(
            MaterialInput::filled(&mut self.disabled_filled)
                .label("Disabled Field")
                .enabled(false),
        );
        ui.add_space(10.0);

        ui.label("Disabled outlined:");
        ui.add(
            MaterialInput::outlined(&mut self.disabled_outlined)
                .label("Disabled Outlined")
                .enabled(false),
        );
        ui.add_space(10.0);

        ui.label("Read-only:");
        ui.add(
            MaterialInput::filled(&mut self.readonly_text)
                .label("Read Only")
                .readonly(true)
                .helper_text("This field cannot be edited"),
        );
    }

    fn render_validation_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Validation Examples");
        ui.label("Common validation patterns");
        ui.add_space(10.0);

        let is_required_empty = self.required_field.is_empty();
        ui.add(
            MaterialInput::filled(&mut self.required_field)
                .label("Required Field *")
                .error_text("This field is required")
                .is_error(is_required_empty)
                .helper_text("Please fill in this field"),
        );
        ui.add_space(10.0);

        let email_valid = self.email_field.is_empty()
            || (self.email_field.contains('@') && self.email_field.contains('.'));
        ui.add(
            MaterialInput::outlined(&mut self.email_field)
                .label("Email Validation")
                .leading_icon("mail")
                .error_text("Please enter a valid email (user@example.com)")
                .is_error(!email_valid),
        );
        ui.add_space(10.0);

        let url_valid = self.url_field.is_empty()
            || (self.url_field.starts_with("http://") || self.url_field.starts_with("https://"));
        ui.add(
            MaterialInput::filled(&mut self.url_field)
                .label("Website URL")
                .leading_icon("link")
                .error_text("URL must start with http:// or https://")
                .is_error(!url_valid)
                .helper_text("https://example.com"),
        );
        ui.add_space(10.0);

        let number_valid = self.number_field.is_empty() || self.number_field.parse::<i32>().is_ok();
        ui.add(
            MaterialInput::outlined(&mut self.number_field)
                .label("Number Only")
                .leading_icon("tag")
                .error_text("Please enter a valid number")
                .is_error(!number_valid),
        );
    }
}
