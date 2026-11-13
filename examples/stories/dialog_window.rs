#![doc(hidden)]

use crate::{dialog, MaterialButton};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct DialogWindow {
    pub open: bool,
    quick: bool,
    no_focus_trap: bool,
    icon: String,
    headline: String,
    supporting_text: String,
    // Dialog states
    standard_dialog_open: bool,
    alert_dialog_open: bool,
    confirm_dialog_open: bool,
    choice_dialog_open: bool,
    form_dialog_open: bool,
    floating_sheet_open: bool,
    settings_dialog_open: bool,
    icon_dialog_open: bool,
    scrollable_dialog_open: bool,
    long_content_dialog_open: bool,
    custom_padding_dialog_open: bool,
    multi_action_dialog_open: bool,
    wizard_dialog_open: bool,
    wizard_current_step: usize,
    wizard_name: String,
    wizard_email: String,
    wizard_plan: Option<usize>,
    wizard_agree_terms: bool,
    // Form data
    pet_choice: Option<usize>,
    first_name: String,
    last_name: String,
    company: String,
    job_title: String,
    email: String,
    phone: String,
    // Customization options
    dialog_width: f32,
    dialog_height: f32,
    title_padding: [f32; 4],
    content_padding: [f32; 4],
    actions_padding: [f32; 4],
    button_padding: [f32; 2],
    actions_spacing: f32,
    scrollable_content: bool,
}

impl Default for DialogWindow {
    fn default() -> Self {
        Self {
            open: false,
            quick: false,
            no_focus_trap: false,
            icon: "info".to_string(),
            headline: "Example Dialog".to_string(),
            supporting_text: "This is a Material Design dialog example.".to_string(),
            standard_dialog_open: false,
            alert_dialog_open: false,
            confirm_dialog_open: false,
            choice_dialog_open: false,
            form_dialog_open: false,
            floating_sheet_open: false,
            settings_dialog_open: false,
            icon_dialog_open: false,
            scrollable_dialog_open: false,
            long_content_dialog_open: false,
            custom_padding_dialog_open: false,
            multi_action_dialog_open: false,
            wizard_dialog_open: false,
            wizard_current_step: 0,
            wizard_name: String::new(),
            wizard_email: String::new(),
            wizard_plan: Some(0),
            wizard_agree_terms: false,
            pet_choice: Some(0),
            first_name: String::new(),
            last_name: String::new(),
            company: String::new(),
            job_title: String::new(),
            email: String::new(),
            phone: String::new(),
            // Default values matching Material Design 3
            dialog_width: 400.0,
            dialog_height: 600.0,
            title_padding: [24.0, 24.0, 0.0, 0.0],
            content_padding: [24.0, 24.0, 0.0, 24.0],
            actions_padding: [24.0, 24.0, 0.0, 0.0],
            button_padding: [12.0, 8.0],
            actions_spacing: 8.0,
            scrollable_content: false,
        }
    }
}

impl DialogWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Dialog Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_dialog_triggers(ui);
                });
            });
        self.open = open;

        // Show dialogs
        self.show_dialogs(ctx);
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Dialog Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/dialog/stories/");
            }
        });

        ui.checkbox(&mut self.quick, "Quick (no animation)");
        ui.checkbox(&mut self.no_focus_trap, "No focus trap");
        ui.checkbox(&mut self.scrollable_content, "Scrollable content");

        ui.horizontal(|ui| {
            ui.label("Icon:");
            ui.text_edit_singleline(&mut self.icon);
        });

        ui.horizontal(|ui| {
            ui.label("Headline:");
            ui.text_edit_singleline(&mut self.headline);
        });

        ui.horizontal(|ui| {
            ui.label("Supporting text:");
            ui.text_edit_singleline(&mut self.supporting_text);
        });

        ui.separator();

        ui.label("Advanced Customization:");

        ui.horizontal(|ui| {
            ui.label("Dialog Width:");
            ui.add(egui::DragValue::new(&mut self.dialog_width).range(280.0..=800.0));
        });

        ui.horizontal(|ui| {
            ui.label("Dialog Height:");
            ui.add(egui::DragValue::new(&mut self.dialog_height).range(200.0..=800.0));
        });

        ui.horizontal(|ui| {
            ui.label("Actions Spacing:");
            ui.add(egui::DragValue::new(&mut self.actions_spacing).range(0.0..=24.0));
        });

        egui::CollapsingHeader::new("Padding Settings").show(ui, |ui| {
            ui.label("Title Padding [L, R, T, B]:");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.title_padding[0])
                        .range(0.0..=48.0)
                        .prefix("L:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.title_padding[1])
                        .range(0.0..=48.0)
                        .prefix("R:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.title_padding[2])
                        .range(0.0..=48.0)
                        .prefix("T:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.title_padding[3])
                        .range(0.0..=48.0)
                        .prefix("B:"),
                );
            });

            ui.label("Content Padding [L, R, T, B]:");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.content_padding[0])
                        .range(0.0..=48.0)
                        .prefix("L:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.content_padding[1])
                        .range(0.0..=48.0)
                        .prefix("R:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.content_padding[2])
                        .range(0.0..=48.0)
                        .prefix("T:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.content_padding[3])
                        .range(0.0..=48.0)
                        .prefix("B:"),
                );
            });

            ui.label("Actions Padding [L, R, T, B]:");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.actions_padding[0])
                        .range(0.0..=48.0)
                        .prefix("L:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.actions_padding[1])
                        .range(0.0..=48.0)
                        .prefix("R:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.actions_padding[2])
                        .range(0.0..=48.0)
                        .prefix("T:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.actions_padding[3])
                        .range(0.0..=48.0)
                        .prefix("B:"),
                );
            });

            ui.label("Button Padding [H, V]:");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.button_padding[0])
                        .range(0.0..=32.0)
                        .prefix("H:"),
                );
                ui.add(
                    egui::DragValue::new(&mut self.button_padding[1])
                        .range(0.0..=32.0)
                        .prefix("V:"),
                );
            });
        });
    }

    fn render_dialog_triggers(&mut self, ui: &mut egui::Ui) {
        ui.heading("Basic Dialog Types");

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled("Standard Dialog")).clicked() {
                self.standard_dialog_open = true;
            }

            if ui.add(MaterialButton::filled("Alert Dialog")).clicked() {
                self.alert_dialog_open = true;
            }

            if ui.add(MaterialButton::filled("Confirm Dialog")).clicked() {
                self.confirm_dialog_open = true;
            }

            if ui.add(MaterialButton::filled("Choice Dialog")).clicked() {
                self.choice_dialog_open = true;
            }

            if ui.add(MaterialButton::filled("Form Dialog")).clicked() {
                self.form_dialog_open = true;
            }
        });

        ui.add_space(10.0);
        ui.heading("Advanced Examples");

        ui.horizontal_wrapped(|ui| {
            if ui
                .add(MaterialButton::filled_tonal("Icon Dialog"))
                .clicked()
            {
                self.icon_dialog_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Scrollable Content"))
                .clicked()
            {
                self.scrollable_dialog_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Long Content"))
                .clicked()
            {
                self.long_content_dialog_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Custom Padding"))
                .clicked()
            {
                self.custom_padding_dialog_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Multi-Action"))
                .clicked()
            {
                self.multi_action_dialog_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Floating Sheet"))
                .clicked()
            {
                self.floating_sheet_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Scrollable Settings"))
                .clicked()
            {
                self.settings_dialog_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Wizard Dialog"))
                .clicked()
            {
                self.wizard_dialog_open = true;
                self.wizard_current_step = 0; // Reset to first step
            }
        });
    }

    fn show_dialogs(&mut self, ctx: &egui::Context) {
        // Standard Dialog
        if self.standard_dialog_open {
            dialog(
                "standard_dialog",
                &self.headline,
                &mut self.standard_dialog_open,
            )
            .icon(&self.icon)
            .max_width(400.0)
            .content(|ui, _open| {
                ui.label(&self.supporting_text);
            })
            .primary_action("OK", || {
                println!("Standard dialog OK clicked!");
            })
            .action("Close", || {
                println!("Standard dialog Close clicked!");
            })
            .show(ctx);
        }

        // Alert Dialog
        if self.alert_dialog_open {
            dialog("alert_dialog", "Alert", &mut self.alert_dialog_open)
                .max_width(320.0)
                .content(|ui, _open| {
                    ui.label("This is a standard alert dialog. Alert dialogs interrupt users with urgent information, details, or actions.");
                })
                .primary_action("OK", || {
                    println!("Alert dialog OK clicked!");
                })
                .show(ctx);
        }

        // Confirm Dialog - demonstrates icon usage
        if self.confirm_dialog_open {
            dialog(
                "confirm_dialog",
                "Permanently delete?",
                &mut self.confirm_dialog_open,
            )
            .icon("delete_outline")
            .max_width(350.0)
            .content(|ui, _open| {
                ui.label(
                    "Deleting the selected photos will also remove them from all synced devices.",
                );
            })
            .action("Delete", || {
                println!("Confirm dialog Delete clicked!");
            })
            .primary_action("Cancel", || {
                println!("Confirm dialog Cancel clicked!");
            })
            .show(ctx);
        }

        // Choice Dialog - demonstrates radio buttons
        if self.choice_dialog_open {
            dialog(
                "choice_dialog",
                "Choose your favorite pet",
                &mut self.choice_dialog_open,
            )
            .max_width(350.0)
            .content(|ui, _open| {
                ui.label("Choose your favorite pet:");
                ui.radio_value(&mut self.pet_choice, Some(0), "Cats");
                ui.radio_value(&mut self.pet_choice, Some(1), "Dogs");
                ui.radio_value(&mut self.pet_choice, Some(2), "Birds");
            })
            .action("Cancel", || {
                println!("Choice dialog Cancel clicked!");
            })
            .primary_action("OK", || {
                println!("Choice dialog OK clicked!");
            })
            .show(ctx);
        }

        // Form Dialog - demonstrates form inputs
        if self.form_dialog_open {
            dialog(
                "form_dialog",
                "Create new contact",
                &mut self.form_dialog_open,
            )
            .max_width(560.0)
            .content(|ui, _open| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("First Name:");
                        ui.text_edit_singleline(&mut self.first_name);
                        ui.label("Last Name:");
                        ui.text_edit_singleline(&mut self.last_name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Company:");
                        ui.text_edit_singleline(&mut self.company);
                        ui.label("Job Title:");
                        ui.text_edit_singleline(&mut self.job_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Email:");
                        ui.text_edit_singleline(&mut self.email);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Phone:");
                        ui.text_edit_singleline(&mut self.phone);
                    });
                });
            })
            .action("Reset", || {
                println!("Form dialog Reset clicked!");
            })
            .action("Cancel", || {
                println!("Form dialog Cancel clicked!");
            })
            .primary_action("Save", || {
                println!("Form dialog Save clicked!");
            })
            .show(ctx);
        }

        // Icon Dialog - demonstrates centered icon above title
        if self.icon_dialog_open {
            dialog(
                "icon_dialog",
                "Save your changes?",
                &mut self.icon_dialog_open,
            )
            .icon("💾")
            .max_width(380.0)
            .content(|ui, _open| {
                ui.label("You have unsaved changes. Would you like to save them before closing?");
            })
            .action("Don't Save", || {
                println!("Icon dialog Don't Save clicked!");
            })
            .action("Cancel", || {
                println!("Icon dialog Cancel clicked!");
            })
            .primary_action("Save", || {
                println!("Icon dialog Save clicked!");
            })
            .show(ctx);
        }

        // Scrollable Dialog - demonstrates scrollable content
        if self.scrollable_dialog_open {
            dialog(
                "scrollable_dialog",
                "Terms and Conditions",
                &mut self.scrollable_dialog_open,
            )
            .max_width(450.0)
            .max_height(400.0)
            .scrollable(true)
            .content(|ui, _open| {
                ui.label("Please read the following terms and conditions carefully:");
                ui.add_space(10.0);
                for i in 1..=20 {
                    ui.label(format!("{}. This is a term or condition that you need to read and understand before proceeding.", i));
                    ui.add_space(5.0);
                }
            })
            .action("Decline", || {
                println!("Scrollable dialog Decline clicked!");
            })
            .primary_action("Accept", || {
                println!("Scrollable dialog Accept clicked!");
            })
            .show(ctx);
        }

        // Long Content Dialog
        if self.long_content_dialog_open {
            dialog(
                "long_content_dialog",
                "Important Information",
                &mut self.long_content_dialog_open,
            )
            .max_width(450.0)
            .max_height(400.0)
            .scrollable(true)
            .content(|ui, _open| {
                ui.label("This dialog demonstrates how content wraps and scrolls when it's too long.");
                ui.add_space(10.0);
                ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
                ui.add_space(10.0);
                ui.label("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
            })
            .primary_action("Got it", || {
                println!("Long content dialog Got it clicked!");
            })
            .show(ctx);
        }

        // Custom Padding Dialog - uses user-defined padding
        if self.custom_padding_dialog_open {
            dialog(
                "custom_padding_dialog",
                "Custom Padding",
                &mut self.custom_padding_dialog_open,
            )
            .max_width(400.0)
            .title_padding(self.title_padding)
            .content_padding(self.content_padding)
            .actions_padding(self.actions_padding)
            .button_padding(self.button_padding)
            .actions_spacing(self.actions_spacing)
            .content(|ui, _open| {
                ui.label("This dialog uses the custom padding settings from the control panel.");
                ui.add_space(5.0);
                ui.label("Adjust the values in the 'Padding Settings' section to see the changes.");
            })
            .primary_action("OK", || {
                println!("Custom padding dialog OK clicked!");
            })
            .show(ctx);
        }

        // Multi-Action Dialog - demonstrates many action buttons
        if self.multi_action_dialog_open {
            dialog(
                "multi_action_dialog",
                "Multiple Actions",
                &mut self.multi_action_dialog_open,
            )
            .max_width(420.0)
            .content(|ui, _open| {
                ui.label("This dialog demonstrates multiple action buttons with different styles.");
            })
            .text_action("Option 1", || {
                println!("Multi-action dialog Option 1 clicked!");
            })
            .text_action("Option 2", || {
                println!("Multi-action dialog Option 2 clicked!");
            })
            .filled_tonal_action("Maybe", || {
                println!("Multi-action dialog Maybe clicked!");
            })
            .action("Cancel", || {
                println!("Multi-action dialog Cancel clicked!");
            })
            .primary_action("Confirm", || {
                println!("Multi-action dialog Confirm clicked!");
            })
            .show(ctx);
        }

        // Floating Sheet
        if self.floating_sheet_open {
            dialog("floating_sheet", "Floating Sheet", &mut self.floating_sheet_open)
                .max_width(400.0)
                .content(|ui, _open| {
                    ui.label("This is a floating sheet with title. Floating sheets offer no action buttons at the bottom, but there's a close icon button at the top right. They accept any HTML content.");
                })
                .show(ctx);
        }

        // Scrollable Settings Dialog
        if self.settings_dialog_open {
            dialog(
                "settings_dialog",
                "Settings",
                &mut self.settings_dialog_open,
            )
            .max_width(500.0)
            .max_height(self.dialog_height)
            .scrollable(true)
            .content(|ui, _open| {
                ui.set_width(ui.available_width());
                for i in 0..50 {
                    ui.horizontal(|ui| {
                        ui.label(format!("Setting Option {}", i + 1));
                        let mut value = format!("Value {}", i + 1);
                        ui.add(egui::TextEdit::singleline(&mut value).desired_width(150.0));
                    });
                }
            })
            .action("Close", || {
                println!("Settings dialog Close clicked!");
            })
            .show(ctx);
        }

        // Wizard Dialog - using enhanced dialog() builder with programmatic close
        if self.wizard_dialog_open {
            let mut open = self.wizard_dialog_open;
            dialog("wizard_dialog", "Setup Wizard", &mut open)
                .max_width(500.0)
                .content(|ui, open| {
                    // Step indicator
                    ui.add_space(10.0);
                    self.render_wizard_step_indicator(ui);
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Step content
                    match self.wizard_current_step {
                        0 => self.render_wizard_step_account(ui),
                        1 => self.render_wizard_step_plan(ui),
                        2 => self.render_wizard_step_confirm(ui),
                        3 => self.render_wizard_step_complete(ui),
                        _ => {}
                    }

                    // Navigation buttons
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        // Close button - now works with enhanced API!
                        if ui.add(MaterialButton::outlined("Close")).clicked() {
                            *open = false;
                        }

                        ui.add_space(10.0);

                        // Previous button (disabled on first step)
                        let can_go_back = self.wizard_current_step > 0;
                        if ui
                            .add(MaterialButton::outlined("Previous").enabled(can_go_back))
                            .clicked()
                        {
                            if self.wizard_current_step > 0 {
                                self.wizard_current_step -= 1;
                            }
                        }

                        ui.add_space(10.0);

                        // Next/Finish button
                        if self.wizard_current_step < 3 {
                            // Validate current step before allowing Next
                            let can_proceed = match self.wizard_current_step {
                                0 => !self.wizard_name.is_empty() && !self.wizard_email.is_empty(),
                                1 => self.wizard_plan.is_some(),
                                2 => self.wizard_agree_terms,
                                _ => true,
                            };

                            if ui
                                .add(MaterialButton::filled("Next").enabled(can_proceed))
                                .clicked()
                            {
                                self.wizard_current_step += 1;
                            }
                        } else {
                            // Final step - show Finish button
                            if ui.add(MaterialButton::filled("Finish")).clicked() {
                                *open = false;
                                // Reset wizard state
                                self.wizard_current_step = 0;
                                self.wizard_name.clear();
                                self.wizard_email.clear();
                                self.wizard_plan = Some(0);
                                self.wizard_agree_terms = false;
                            }
                        }
                    });
                })
                .show(ctx);
            self.wizard_dialog_open = open;
        }
    }

    fn render_wizard_step_indicator(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let steps = [
                ("1. Account", 0),
                ("2. Plan", 1),
                ("3. Review", 2),
                ("4. Complete", 3),
            ];

            for (i, (label, step_num)) in steps.iter().enumerate() {
                let is_current = self.wizard_current_step == *step_num;
                let is_complete = self.wizard_current_step > *step_num;

                if is_current {
                    // Current step: filled button
                    ui.add(MaterialButton::filled(*label).small());
                } else if is_complete {
                    // Completed step: outlined clickable button
                    if ui.add(MaterialButton::outlined(*label).small()).clicked() {
                        // Allow jumping back to previous steps
                    }
                } else {
                    // Future step: disabled outlined button
                    ui.add(MaterialButton::outlined(*label).small().enabled(false));
                }

                if i < steps.len() - 1 {
                    ui.add_space(5.0);
                }
            }
        });
    }

    fn render_wizard_step_account(&mut self, ui: &mut egui::Ui) {
        ui.heading("Create Your Account");
        ui.add_space(10.0);
        ui.label("Enter your basic information to get started.");
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.wizard_name);
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Email:");
            ui.text_edit_singleline(&mut self.wizard_email);
        });
    }

    fn render_wizard_step_plan(&mut self, ui: &mut egui::Ui) {
        ui.heading("Choose Your Plan");
        ui.add_space(10.0);
        ui.label("Select a subscription plan that fits your needs.");
        ui.add_space(15.0);

        ui.radio_value(&mut self.wizard_plan, Some(0), "Free - $0/month");
        ui.add_space(5.0);
        ui.radio_value(&mut self.wizard_plan, Some(1), "Basic - $9/month");
        ui.add_space(5.0);
        ui.radio_value(&mut self.wizard_plan, Some(2), "Pro - $29/month");
    }

    fn render_wizard_step_confirm(&mut self, ui: &mut egui::Ui) {
        ui.heading("Review & Confirm");
        ui.add_space(10.0);
        ui.label("Please review your information before continuing.");
        ui.add_space(15.0);

        ui.label(format!(
            "Name: {}",
            if self.wizard_name.is_empty() {
                "Not provided"
            } else {
                &self.wizard_name
            }
        ));
        ui.label(format!(
            "Email: {}",
            if self.wizard_email.is_empty() {
                "Not provided"
            } else {
                &self.wizard_email
            }
        ));

        let plan_name = match self.wizard_plan {
            Some(0) => "Free",
            Some(1) => "Basic",
            Some(2) => "Pro",
            _ => "None",
        };
        ui.label(format!("Plan: {}", plan_name));

        ui.add_space(15.0);
        ui.checkbox(
            &mut self.wizard_agree_terms,
            "I agree to the terms and conditions",
        );
    }

    fn render_wizard_step_complete(&mut self, ui: &mut egui::Ui) {
        ui.heading("✓ Setup Complete!");
        ui.add_space(10.0);
        ui.label("Your account has been created successfully.");
        ui.add_space(15.0);
        ui.label("You can now start using the application.");
    }
}
