use eframe::egui::{self, Ui, Window};
use crate::MaterialField;

pub struct FieldWindow {
    pub open: bool,
    label: String,
    supporting_text: String,
    error_text: String,
    disabled: bool,
    error: bool,
    focused: bool,
    populated: bool,
    required: bool,
    no_asterisk: bool,
    leading_icon: bool,
    trailing_icon: bool,
    resizable: bool,
    count: i32,
    max: i32,
    // Content for demonstration
    input_text: String,
    textarea_text: String,
}

impl Default for FieldWindow {
    fn default() -> Self {
        Self {
            open: false,
            label: "Label".to_string(),
            supporting_text: "Supporting text".to_string(),
            error_text: "Error text".to_string(),
            disabled: false,
            error: false,
            focused: false,
            populated: false,
            required: false,
            no_asterisk: false,
            leading_icon: false,
            trailing_icon: false,
            resizable: false,
            count: 0,
            max: 100,
            input_text: String::new(),
            textarea_text: String::new(),
        }
    }
}

impl FieldWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Field Stories")
            .open(&mut open)
            .default_size([800.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_filled_field(ui);
                    ui.add_space(30.0);
                    self.render_outlined_field(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Field Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/field/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);
        });

        ui.horizontal(|ui| {
            ui.label("Supporting text:");
            ui.text_edit_singleline(&mut self.supporting_text);
        });

        ui.horizontal(|ui| {
            ui.label("Error text:");
            ui.text_edit_singleline(&mut self.error_text);
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.disabled, "disabled");
            ui.checkbox(&mut self.error, "error");
            ui.checkbox(&mut self.focused, "focused");
            ui.checkbox(&mut self.populated, "populated");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.required, "required");
            ui.checkbox(&mut self.no_asterisk, "noAsterisk");
            ui.checkbox(&mut self.leading_icon, "Leading icon");
            ui.checkbox(&mut self.trailing_icon, "Trailing icon");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.resizable, "resizable");
            ui.label("count:");
            ui.add(egui::DragValue::new(&mut self.count).range(0..=self.max));
            ui.label("max:");
            ui.add(egui::DragValue::new(&mut self.max).range(0..=1000));
        });
    }

    fn render_filled_field(&mut self, ui: &mut Ui) {
        ui.heading("Filled Field");

        let mut field = MaterialField::filled()
            .min_width(256.0)
            .enabled(!self.disabled)
            .error(self.error)
            .focused(self.focused)
            .populated(self.populated || !self.input_text.is_empty())
            .required(self.required)
            .no_asterisk(self.no_asterisk)
            .has_start(self.leading_icon)
            .has_end(self.trailing_icon);

        if !self.label.is_empty() {
            field = field.label(&self.label);
        }

        if !self.supporting_text.is_empty() && !self.error {
            field = field.supporting_text(&self.supporting_text);
        }

        if !self.error_text.is_empty() && self.error {
            field = field.error_text(&self.error_text);
        }

        if self.count >= 0 && self.max > 0 {
            field = field.count(self.count, self.max);
        }

        if self.resizable {
            ui.add(field.content(|ui| {
                ui.text_edit_multiline(&mut self.textarea_text)
            }));
        } else {
            ui.add(field.content(|ui| {
                ui.text_edit_singleline(&mut self.input_text)
            }));
        }

        // Update count based on actual text length
        if self.resizable {
            self.count = self.textarea_text.len() as i32;
        } else {
            self.count = self.input_text.len() as i32;
        }
    }

    fn render_outlined_field(&mut self, ui: &mut Ui) {
        ui.heading("Outlined Field");

        let mut field = MaterialField::outlined()
            .min_width(256.0)
            .enabled(!self.disabled)
            .error(self.error)
            .focused(self.focused)
            .populated(self.populated || !self.input_text.is_empty())
            .required(self.required)
            .no_asterisk(self.no_asterisk)
            .has_start(self.leading_icon)
            .has_end(self.trailing_icon);

        if !self.label.is_empty() {
            field = field.label(&self.label);
        }

        if !self.supporting_text.is_empty() && !self.error {
            field = field.supporting_text(&self.supporting_text);
        }

        if !self.error_text.is_empty() && self.error {
            field = field.error_text(&self.error_text);
        }

        if self.count >= 0 && self.max > 0 {
            field = field.count(self.count, self.max);
        }

        if self.resizable {
            ui.add(field.content(|ui| {
                ui.text_edit_multiline(&mut self.textarea_text)
            }));
        } else {
            ui.add(field.content(|ui| {
                ui.text_edit_singleline(&mut self.input_text)
            }));
        }

        ui.add_space(20.0);

        // Additional examples
        ui.heading("Field Examples");

        ui.label("Basic field with label:");
        ui.add(MaterialField::filled()
            .label("Basic Field")
            .content(|ui| {
                let mut text = String::new();
                ui.text_edit_singleline(&mut text)
            }));

        ui.add_space(10.0);

        ui.label("Field with supporting text:");
        ui.add(MaterialField::outlined()
            .label("Email")
            .supporting_text("Enter your email address")
            .content(|ui| {
                let mut email = String::new();
                ui.text_edit_singleline(&mut email)
            }));

        ui.add_space(10.0);

        ui.label("Error field:");
        ui.add(MaterialField::filled()
            .label("Password")
            .error(true)
            .error_text("Password must be at least 8 characters")
            .content(|ui| {
                let mut password = String::new();
                ui.text_edit_singleline(&mut password)
            }));

        ui.add_space(10.0);

        ui.label("Required field:");
        ui.add(MaterialField::outlined()
            .label("Username")
            .required(true)
            .supporting_text("This field is required")
            .content(|ui| {
                let mut username = String::new();
                ui.text_edit_singleline(&mut username)
            }));
    }
}