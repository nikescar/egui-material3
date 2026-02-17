#![doc(hidden)]

use crate::MaterialButton;
use eframe::egui::{self, Window};
use egui_material3::{text_field, text_field_outlined, MaterialTextField};

#[doc(hidden)]
pub struct TextFieldWindow {
    pub open: bool,
    // Text field values
    filled_text: String,
    outlined_text: String,
    error_text: String,
    disabled_text: String,
    password_text: String,
    multiline_text: String,
    counter_text: String,
    search_text: String,
    email_text: String,
    // Control states
    enabled: bool,
    show_error: bool,
    max_length: usize,
    show_counter: bool,
    show_password: bool,
}

impl Default for TextFieldWindow {
    fn default() -> Self {
        Self {
            open: false,
            filled_text: String::new(),
            outlined_text: String::new(),
            error_text: "Invalid input".to_string(),
            disabled_text: "Disabled field".to_string(),
            password_text: String::new(),
            multiline_text: String::new(),
            counter_text: String::new(),
            search_text: String::new(),
            email_text: String::new(),
            enabled: true,
            show_error: true,
            max_length: 50,
            show_counter: true,
            show_password: false,
        }
    }
}

impl TextFieldWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Text Field Stories")
            .open(&mut open)
            .default_size([800.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_filled_text_fields(ui);
                    ui.add_space(20.0);
                    self.render_outlined_text_fields(ui);
                    ui.add_space(20.0);
                    self.render_special_variants(ui);
                    ui.add_space(20.0);
                    self.render_m3_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.push_id("textfield_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Text Field Controls");

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
                    let _ = webbrowser::open("https://m3.material.io/components/text-fields/overview");
                }

                if ui.add(MaterialButton::filled("Flutter Spec").small()).clicked() {
                    let _ = webbrowser::open("https://api.flutter.dev/flutter/material/TextField-class.html");
                }
            });

            ui.checkbox(&mut self.enabled, "Enabled");
            ui.checkbox(&mut self.show_error, "Show Error State");
            ui.checkbox(&mut self.show_counter, "Show Character Counter");

            ui.horizontal(|ui| {
                ui.label("Max Length:");
                ui.add(egui::Slider::new(&mut self.max_length, 10..=200));
            });
        });
    }

    fn render_filled_text_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("Filled Text Fields");
        ui.label("Default Material Design 3 style with background fill");
        ui.add_space(8.0);

        // Basic filled field
        ui.label("Basic filled field:");
        ui.add(
            text_field(&mut self.filled_text)
                .label("Label")
                .hint("Placeholder text")
                .helper_text("Supporting text")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // With prefix icon
        ui.label("With prefix icon:");
        ui.add(
            text_field(&mut self.search_text)
                .label("Search")
                .hint("Search...")
                .prefix_icon("search")
                .helper_text("Enter search terms")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // With suffix icon
        ui.label("With suffix icon:");
        ui.add(
            text_field(&mut self.email_text)
                .label("Email")
                .hint("example@email.com")
                .suffix_icon("email")
                .helper_text("We'll never share your email")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // With error state
        if self.show_error {
            ui.label("With error state:");
            ui.add(
                text_field(&mut self.error_text)
                    .label("Username")
                    .error_text("Username is already taken")
                    .enabled(self.enabled)
            );
            ui.add_space(12.0);
        }

        // Disabled state
        ui.label("Disabled state:");
        ui.add(
            text_field(&mut self.disabled_text)
                .label("Disabled")
                .helper_text("This field is disabled")
                .enabled(false)
        );
    }

    fn render_outlined_text_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("Outlined Text Fields");
        ui.label("Text fields with border outline and no fill");
        ui.add_space(8.0);

        // Basic outlined field
        ui.label("Basic outlined field:");
        ui.add(
            text_field_outlined(&mut self.outlined_text)
                .label("Label")
                .hint("Placeholder text")
                .helper_text("Supporting text")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // With prefix icon
        ui.label("With prefix icon:");
        ui.add(
            text_field_outlined(&mut self.search_text)
                .label("Search")
                .hint("Search...")
                .prefix_icon("search")
                .helper_text("Enter search terms")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // With both icons
        ui.label("With prefix and suffix icons:");
        ui.add(
            text_field_outlined(&mut self.email_text)
                .label("Email")
                .hint("example@email.com")
                .prefix_icon("email")
                .suffix_icon("check")
                .helper_text("Email verified")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // With error
        if self.show_error {
            ui.label("With error state:");
            ui.add(
                text_field_outlined(&mut self.error_text)
                    .label("Username")
                    .error_text("Username is already taken")
                    .enabled(self.enabled)
            );
            ui.add_space(12.0);
        }

        // Disabled
        ui.label("Disabled state:");
        ui.add(
            text_field_outlined(&mut self.disabled_text)
                .label("Disabled")
                .helper_text("This field is disabled")
                .enabled(false)
        );
    }

    fn render_special_variants(&mut self, ui: &mut egui::Ui) {
        ui.heading("Special Variants");
        ui.add_space(8.0);

        // Password field
        ui.label("Password field (obscured text):");
        ui.horizontal(|ui| {
            ui.add(
                text_field(&mut self.password_text)
                    .label("Password")
                    .hint("Enter password")
                    .password(!self.show_password)
                    .prefix_icon("lock")
                    .helper_text("Minimum 8 characters")
                    .enabled(self.enabled)
            );
            ui.checkbox(&mut self.show_password, "Show");
        });

        ui.add_space(12.0);

        // Multiline text field
        ui.label("Multiline text field:");
        ui.add(
            text_field(&mut self.multiline_text)
                .label("Comments")
                .hint("Enter your comments here...")
                .multiline(4)
                .helper_text("Maximum 500 characters")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // With character counter
        ui.label("With character counter:");
        ui.add(
            text_field(&mut self.counter_text)
                .label("Bio")
                .hint("Tell us about yourself")
                .max_length(self.max_length)
                .show_counter(self.show_counter)
                .helper_text("Keep it concise")
                .enabled(self.enabled)
        );

        ui.add_space(12.0);

        // Side by side comparison
        ui.label("Side by side - Filled vs Outlined:");
        ui.horizontal(|ui| {
            ui.add(
                text_field(&mut self.filled_text)
                    .label("Filled")
                    .hint("Type here...")
                    .desired_width(250.0)
                    .enabled(self.enabled)
            );
            ui.add_space(16.0);
            ui.add(
                text_field_outlined(&mut self.outlined_text)
                    .label("Outlined")
                    .hint("Type here...")
                    .desired_width(250.0)
                    .enabled(self.enabled)
            );
        });
    }

    fn render_m3_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Material Design 3 Examples");
        ui.label("Examples from component_screen.dart");
        ui.add_space(8.0);

        // Form-like layout
        ui.group(|ui| {
            ui.label("User Registration Form:");
            ui.add_space(8.0);

            ui.add(
                text_field(&mut self.filled_text)
                    .label("Full Name")
                    .hint("John Doe")
                    .prefix_icon("person")
                    .helper_text("As it appears on your ID")
                    .enabled(self.enabled)
            );

            ui.add_space(8.0);

            ui.add(
                text_field(&mut self.email_text)
                    .label("Email Address")
                    .hint("john.doe@example.com")
                    .prefix_icon("email")
                    .helper_text("We'll send a verification email")
                    .enabled(self.enabled)
            );

            ui.add_space(8.0);

            ui.add(
                text_field(&mut self.password_text)
                    .label("Password")
                    .hint("Enter secure password")
                    .password(!self.show_password)
                    .prefix_icon("lock")
                    .max_length(100)
                    .helper_text("Must be at least 8 characters")
                    .enabled(self.enabled)
            );

            ui.add_space(8.0);

            ui.add(
                text_field(&mut self.multiline_text)
                    .label("About You")
                    .hint("Tell us about yourself...")
                    .multiline(3)
                    .max_length(200)
                    .show_counter(true)
                    .enabled(self.enabled)
            );

            ui.add_space(12.0);

            ui.horizontal(|ui| {
                if ui.add(MaterialButton::filled("Submit")).clicked() {
                    // Handle form submission
                }
                if ui.add(MaterialButton::text("Cancel")).clicked() {
                    // Handle cancel
                }
            });
        });

        ui.add_space(16.0);

        // Error state demonstration
        ui.group(|ui| {
            ui.label("Validation Examples:");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.add(
                    text_field(&mut "".to_string())
                        .label("Required Field")
                        .error_text("This field is required")
                        .desired_width(250.0)
                );

                ui.add_space(16.0);

                ui.add(
                    text_field_outlined(&mut "test@invalid".to_string())
                        .label("Email")
                        .error_text("Invalid email format")
                        .desired_width(250.0)
                );
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.add(
                    text_field(&mut "abc".to_string())
                        .label("Password")
                        .password(true)
                        .error_text("Password too short")
                        .desired_width(250.0)
                );

                ui.add_space(16.0);

                let mut long_text = "a".repeat(self.max_length + 10);
                ui.add(
                    text_field_outlined(&mut long_text)
                        .label("Username")
                        .max_length(self.max_length)
                        .show_counter(true)
                        .error_text("Username exceeds maximum length")
                        .desired_width(250.0)
                );
            });
        });

        ui.add_space(16.0);

        // Compact layout
        ui.group(|ui| {
            ui.label("Compact Layout:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.add(
                    text_field(&mut self.filled_text)
                        .label("First Name")
                        .desired_width(150.0)
                );
                ui.add(
                    text_field(&mut self.outlined_text)
                        .label("Last Name")
                        .desired_width(150.0)
                );
                ui.add(
                    text_field(&mut self.email_text)
                        .label("Age")
                        .desired_width(80.0)
                );
            });
        });
    }
}
