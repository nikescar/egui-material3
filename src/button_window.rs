use eframe::egui::{self, Ui, Window};
use crate::{MaterialButton, icons::icon_text};

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
                    self.render_button_links(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {

        ui.horizontal(|ui| {
            ui.heading("Button Controls");

            if ui.button("Target").clicked() {
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

    fn render_button_variants(&mut self, ui: &mut Ui) {
        ui.heading("Button Variants");
        
        // First row - basic buttons
        ui.horizontal(|ui| {
            let label = if self.label.is_empty() { "Filled" } else { &self.label };
            let mut button = MaterialButton::filled(label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Filled button clicked!");
            }

            let label = if self.label.is_empty() { "Outlined" } else { &self.label };
            let mut button = MaterialButton::outlined(label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Outlined button clicked!");
            }

            let label = if self.label.is_empty() { "Elevated" } else { &self.label };
            let mut button = MaterialButton::elevated(label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Elevated button clicked!");
            }

            let label = if self.label.is_empty() { "Tonal" } else { &self.label };
            let mut button = MaterialButton::filled_tonal(label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Tonal button clicked!");
            }

            let label = if self.label.is_empty() { "Text" } else { &self.label };
            let mut button = MaterialButton::text(label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Text button clicked!");
            }
        });

        ui.add_space(10.0);

        // Second row - buttons with icons
        ui.horizontal(|ui| {
            let base_label = if self.label.is_empty() { "Filled" } else { &self.label };
            let label = format!("{} {}", icon_text("upload"), base_label);
            let mut button = MaterialButton::filled(&label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Filled button with icon clicked!");
            }

            let base_label = if self.label.is_empty() { "Outlined" } else { &self.label };
            let label = format!("{} {}", icon_text("upload"), base_label);
            let mut button = MaterialButton::outlined(&label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Outlined button with icon clicked!");
            }

            let base_label = if self.label.is_empty() { "Elevated" } else { &self.label };
            let label = format!("{} {}", icon_text("upload"), base_label);
            let mut button = MaterialButton::elevated(&label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Elevated button with icon clicked!");
            }

            let base_label = if self.label.is_empty() { "Tonal" } else { &self.label };
            let label = format!("{} {}", icon_text("upload"), base_label);
            let mut button = MaterialButton::filled_tonal(&label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Tonal button with icon clicked!");
            }

            let base_label = if self.label.is_empty() { "Text" } else { &self.label };
            let label = format!("{} {}", icon_text("upload"), base_label);
            let mut button = MaterialButton::text(&label);
            if self.disabled || self.soft_disabled {
                button = button.enabled(false);
            }
            if ui.add(button).clicked() && !self.disabled && !self.soft_disabled {
                println!("Text button with icon clicked!");
            }
        });
    }

    fn render_button_links(&mut self, ui: &mut Ui) {
        ui.heading("Button Links");
        
        // First row - basic link buttons
        ui.horizontal(|ui| {
            let base_label = if self.label.is_empty() { "Filled" } else { &self.label };
            let label = format!("{} {}", base_label, icon_text("open_in_new"));
            let button = MaterialButton::filled(&label);
            if ui.add(button).clicked() {
                println!("Filled link button clicked!");
            }

            let base_label = if self.label.is_empty() { "Outlined" } else { &self.label };
            let label = format!("{} {}", base_label, icon_text("open_in_new"));
            let button = MaterialButton::outlined(&label);
            if ui.add(button).clicked() {
                println!("Outlined link button clicked!");
            }

            let base_label = if self.label.is_empty() { "Elevated" } else { &self.label };
            let label = format!("{} {}", base_label, icon_text("open_in_new"));
            let button = MaterialButton::elevated(&label);
            if ui.add(button).clicked() {
                println!("Elevated link button clicked!");
            }

            let base_label = if self.label.is_empty() { "Tonal" } else { &self.label };
            let label = format!("{} {}", base_label, icon_text("open_in_new"));
            let button = MaterialButton::filled_tonal(&label);
            if ui.add(button).clicked() {
                println!("Tonal link button clicked!");
            }

            let base_label = if self.label.is_empty() { "Text" } else { &self.label };
            let label = format!("{} {}", base_label, icon_text("open_in_new"));
            let button = MaterialButton::text(&label);
            if ui.add(button).clicked() {
                println!("Text link button clicked!");
            }
        });

        ui.add_space(10.0);

        // Second row - link buttons with both leading and trailing icons
        ui.horizontal(|ui| {
            let label = if self.label.is_empty() { "Filled" } else { &self.label };
            let button = MaterialButton::filled(label)
                .leading_icon("open_in_new")
                .trailing_icon("arrow_forward");
            if ui.add(button).clicked() {
                println!("Filled link button with both icons clicked!");
            }

            let label = if self.label.is_empty() { "Outlined" } else { &self.label };
            let button = MaterialButton::outlined(label)
                .leading_icon("open_in_new")
                .trailing_icon("arrow_forward");
            if ui.add(button).clicked() {
                println!("Outlined link button with both icons clicked!");
            }

            let label = if self.label.is_empty() { "Elevated" } else { &self.label };
            let button = MaterialButton::elevated(label)
                .leading_icon("open_in_new")
                .trailing_icon("arrow_forward");
            if ui.add(button).clicked() {
                println!("Elevated link button with both icons clicked!");
            }

            let label = if self.label.is_empty() { "Tonal" } else { &self.label };
            let button = MaterialButton::filled_tonal(label)
                .leading_icon("open_in_new")
                .trailing_icon("arrow_forward");
            if ui.add(button).clicked() {
                println!("Tonal link button with both icons clicked!");
            }

            let label = if self.label.is_empty() { "Text" } else { &self.label };
            let button = MaterialButton::text(label)
                .leading_icon("open_in_new")
                .trailing_icon("arrow_forward");
            if ui.add(button).clicked() {
                println!("Text link button with both icons clicked!");
            }
        });
    }
}