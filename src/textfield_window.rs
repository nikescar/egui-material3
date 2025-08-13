use eframe::egui::{self, Ui, Window};
use crate::MaterialTextField;

pub struct TextFieldWindow {
    pub open: bool,
    // Story controls
    label: String,
    placeholder: String,
    disabled: bool,
    prefix_text: String,
    suffix_text: String,
    supporting_text: String,
    // Example text fields
    basic_filled_text: String,
    basic_outlined_text: String,
    textarea_filled_text: String,
    textarea_outlined_text: String,
    icon_filled_text: String,
    icon_outlined_text: String,
    validation_required_text: String,
    validation_numeric_text: String,
    validation_length_text: String,
    validation_pattern_text: String,
    form_first_name: String,
    form_last_name: String,
}

impl Default for TextFieldWindow {
    fn default() -> Self {
        Self {
            open: false,
            // Story controls - matching Material Web stories
            label: "Label".to_string(),
            placeholder: "Placeholder".to_string(),
            disabled: false,
            prefix_text: String::new(),
            suffix_text: String::new(),
            supporting_text: String::new(),
            // Example text fields
            basic_filled_text: String::new(),
            basic_outlined_text: String::new(),
            textarea_filled_text: String::new(),
            textarea_outlined_text: String::new(),
            icon_filled_text: "Value".to_string(),
            icon_outlined_text: "Value".to_string(),
            validation_required_text: "Value".to_string(),
            validation_numeric_text: String::new(),
            validation_length_text: String::new(),
            validation_pattern_text: String::new(),
            form_first_name: String::new(),
            form_last_name: String::new(),
        }
    }
}

impl TextFieldWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Text Field Stories")
            .open(&mut open)
            .default_size([800.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Text Field Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/textfield/stories/");
            }
        });
        
        ui.label("Interactive controls for configuring text field properties:");
        
        ui.add_space(10.0);
        
        // Story knobs - matching the Material Web stories interface
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Label:");
                ui.add(egui::TextEdit::singleline(&mut self.label).desired_width(120.0));
            });
            
            ui.vertical(|ui| {
                ui.label("Placeholder:");
                ui.add(egui::TextEdit::singleline(&mut self.placeholder).desired_width(120.0));
            });
            
            ui.vertical(|ui| {
                ui.label("Prefix text:");
                ui.add(egui::TextEdit::singleline(&mut self.prefix_text).desired_width(80.0));
            });
            
            ui.vertical(|ui| {
                ui.label("Suffix text:");
                ui.add(egui::TextEdit::singleline(&mut self.suffix_text).desired_width(80.0));
            });
        });
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Supporting text:");
                ui.add(egui::TextEdit::singleline(&mut self.supporting_text).desired_width(200.0));
            });
            
            ui.vertical(|ui| {
                ui.label("Options:");
                ui.checkbox(&mut self.disabled, "Disabled");
            });
        });
    }

    fn render_examples(&mut self, ui: &mut Ui) {
        // Text fields story
        ui.heading("Text Fields");
        ui.label("Basic filled and outlined text fields with configurable properties:");
        
        ui.horizontal(|ui| {
            let mut filled_field = MaterialTextField::filled(&mut self.basic_filled_text)
                .width(200.0)
                .disabled(self.disabled);
                
            if !self.label.is_empty() {
                filled_field = filled_field.label(&self.label);
            }
            if !self.placeholder.is_empty() {
                filled_field = filled_field.hint_text(&self.placeholder);
            }
            if !self.prefix_text.is_empty() {
                filled_field = filled_field.prefix_text(&self.prefix_text);
            }
            if !self.suffix_text.is_empty() {
                filled_field = filled_field.suffix_text(&self.suffix_text);
            }
            if !self.supporting_text.is_empty() {
                filled_field = filled_field.supporting_text(&self.supporting_text);
            }
            
            ui.add(filled_field);

            let mut outlined_field = MaterialTextField::outlined(&mut self.basic_outlined_text)
                .width(200.0)
                .disabled(self.disabled);
                
            if !self.label.is_empty() {
                outlined_field = outlined_field.label(&self.label);
            }
            if !self.placeholder.is_empty() {
                outlined_field = outlined_field.hint_text(&self.placeholder);
            }
            if !self.prefix_text.is_empty() {
                outlined_field = outlined_field.prefix_text(&self.prefix_text);
            }
            if !self.suffix_text.is_empty() {
                outlined_field = outlined_field.suffix_text(&self.suffix_text);
            }
            if !self.supporting_text.is_empty() {
                outlined_field = outlined_field.supporting_text(&self.supporting_text);
            }
            
            ui.add(outlined_field);
        });

        ui.add_space(20.0);

        // Text areas story
        ui.heading("Text Areas");
        ui.label("Multiline text fields for longer text input:");
        
        ui.horizontal(|ui| {
            ui.add(MaterialTextField::filled(&mut self.textarea_filled_text)
                .multiline(true)
                .min_rows(3)
                .width(200.0)
                .label("Notes")
                .hint_text("Enter your notes here...")
                .disabled(self.disabled));

            ui.add(MaterialTextField::outlined(&mut self.textarea_outlined_text)
                .multiline(true)
                .min_rows(3)
                .width(200.0)
                .label("Description")
                .hint_text("Enter description...")
                .disabled(self.disabled));
        });

        ui.add_space(20.0);

        // Icons story
        ui.heading("Icons");
        ui.label("Text fields with leading and trailing icons:");
        
        ui.horizontal(|ui| {
            ui.add(MaterialTextField::filled(&mut self.icon_filled_text)
                .width(200.0)
                .label("Search")
                .hint_text("Search...")
                .leading_icon("🔍")
                .trailing_icon("❌")
                .supporting_text("Click X to clear")
                .disabled(self.disabled));

            ui.add(MaterialTextField::outlined(&mut self.icon_outlined_text)
                .width(200.0)
                .label("Email")
                .hint_text("user@example.com")
                .leading_icon("✉️")
                .trailing_icon("❌")
                .supporting_text("Enter your email address")
                .disabled(self.disabled));
        });

        ui.add_space(20.0);

        // Validation story
        ui.heading("Validation");
        ui.label("Text fields with validation states and constraints:");
        
        ui.horizontal_wrapped(|ui| {
            ui.add(MaterialTextField::outlined(&mut self.validation_required_text)
                .width(180.0)
                .label("Required")
                .required(true)
                .supporting_text("* this field is required")
                .disabled(self.disabled));

            ui.add(MaterialTextField::outlined(&mut self.validation_numeric_text)
                .width(180.0)
                .label("Numeric")
                .hint_text("1-10")
                .supporting_text("Enter a number between 1 and 10")
                .disabled(self.disabled));

            ui.add(MaterialTextField::outlined(&mut self.validation_length_text)
                .width(180.0)
                .label("Length")
                .supporting_text("3 to 10 characters")
                .disabled(self.disabled));

            ui.add(MaterialTextField::outlined(&mut self.validation_pattern_text)
                .width(180.0)
                .label("Pattern")
                .hint_text("username")
                .suffix_text("@gmail.com")
                .supporting_text("Characters only")
                .disabled(self.disabled));
        });

        ui.add_space(20.0);

        // Forms story
        ui.heading("Forms");
        ui.label("Text fields used in form contexts:");
        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(MaterialTextField::filled(&mut self.form_first_name)
                    .width(200.0)
                    .label("First name")
                    .required(true)
                    .disabled(self.disabled));

                ui.add(MaterialTextField::filled(&mut self.form_last_name)
                    .width(200.0)
                    .label("Last name")
                    .required(true)
                    .disabled(self.disabled));
            });
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.form_first_name.clear();
                    self.form_last_name.clear();
                }
                
                if ui.button("Submit").clicked() {
                    // In a real app, you would validate and submit the form
                    println!("Form submitted: {} {}", self.form_first_name, self.form_last_name);
                }
            });
        });

        ui.add_space(20.0);

        // Additional examples
        ui.heading("Additional Examples");
        
        ui.label("Prefix and suffix text examples:");
        ui.horizontal(|ui| {
            let mut price_text = String::new();
            ui.add(MaterialTextField::outlined(&mut price_text)
                .width(150.0)
                .label("Price")
                .prefix_text("$")
                .suffix_text("USD"));

            let mut website_text = String::new();
            ui.add(MaterialTextField::outlined(&mut website_text)
                .width(200.0)
                .label("Website")
                .prefix_text("https://")
                .suffix_text(".com"));
        });

        ui.add_space(10.0);

        ui.label("Error states:");
        ui.horizontal(|ui| {
            let mut error_text = String::new();
            ui.add(MaterialTextField::filled(&mut error_text)
                .width(200.0)
                .label("Invalid Email")
                .error(true)
                .supporting_text("Please enter a valid email address"));

            let mut error_outlined_text = String::new();
            ui.add(MaterialTextField::outlined(&mut error_outlined_text)
                .width(200.0)
                .label("Password")
                .error(true)
                .supporting_text("Password must be at least 8 characters"));
        });

        ui.add_space(20.0);

        // Show current values
        ui.separator();
        ui.heading("Current Values");
        ui.label(format!("Basic filled: '{}'", self.basic_filled_text));
        ui.label(format!("Basic outlined: '{}'", self.basic_outlined_text));
        ui.label(format!("Textarea filled: '{}'", self.textarea_filled_text.replace('\n', "\\n")));
        ui.label(format!("Textarea outlined: '{}'", self.textarea_outlined_text.replace('\n', "\\n")));
        ui.label(format!("Icon filled: '{}'", self.icon_filled_text));
        ui.label(format!("Icon outlined: '{}'", self.icon_outlined_text));
        ui.label(format!("Form: '{}' '{}'", self.form_first_name, self.form_last_name));
        
        ui.add_space(10.0);
        ui.label("💡 Tips:");
        ui.label("• Filled text fields are recommended for forms and dialogs");
        ui.label("• Outlined text fields work well on surfaces with strong backgrounds");
        ui.label("• Use supporting text to provide context or validation feedback");
        ui.label("• Icons can improve usability and provide visual context");
        ui.label("• Prefix/suffix text is useful for units, currency, or domains");
    }
}