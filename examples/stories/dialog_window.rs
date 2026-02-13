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
    // Form data
    pet_choice: Option<usize>,
    first_name: String,
    last_name: String,
    company: String,
    job_title: String,
    email: String,
    phone: String,
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
            pet_choice: Some(0),
            first_name: String::new(),
            last_name: String::new(),
            company: String::new(),
            job_title: String::new(),
            email: String::new(),
            phone: String::new(),
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

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/dialog/stories/");
            }
        });

        ui.checkbox(&mut self.quick, "Quick (no animation)");
        ui.checkbox(&mut self.no_focus_trap, "No focus trap");

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
    }

    fn render_dialog_triggers(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dialog Types");

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

            if ui.add(MaterialButton::filled("Floating Sheet")).clicked() {
                self.floating_sheet_open = true;
            }

            if ui.add(MaterialButton::filled("Contents Width")).clicked() {
                self.settings_dialog_open = true;
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
            .content(|ui| {
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
            dialog("alert_dialog", "Alert dialog", &mut self.alert_dialog_open)
                .content(|ui| {
                    ui.label("This is a standard alert dialog. Alert dialogs interrupt users with urgent information, details, or actions.");
                })
                .primary_action("OK", || {
                    println!("Alert dialog OK clicked!");
                })
                .show(ctx);
        }

        // Confirm Dialog
        if self.confirm_dialog_open {
            dialog(
                "confirm_dialog",
                "Permanently delete?",
                &mut self.confirm_dialog_open,
            )
            .icon("delete_outline")
            .content(|ui| {
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

        // Choice Dialog
        if self.choice_dialog_open {
            dialog(
                "choice_dialog",
                "Choose your favorite pet",
                &mut self.choice_dialog_open,
            )
            .content(|ui| {
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

        // Form Dialog
        if self.form_dialog_open {
            dialog(
                "form_dialog",
                "Create new contact",
                &mut self.form_dialog_open,
            )
            .content(|ui| {
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

        // Floating Sheet
        if self.floating_sheet_open {
            dialog("floating_sheet", "Floating Sheet", &mut self.floating_sheet_open)
                .content(|ui| {
                    ui.label("This is a floating sheet with title. Floating sheets offer no action buttons at the bottom, but there's a close icon button at the top right. They accept any HTML content.");
                })
                .show(ctx);
        }

        // Scroll Area Test
        dialog(
                "settings_dialog",
                "Settings",
                &mut self.settings_dialog_open,
            )
            .content(|ui| {
                let screen_width = ui.ctx().screen_rect().width();
                let dialog_width = screen_width - 100.0;
                let dialog_width = 400.0;
                ui.set_width(dialog_width);
                let screen_height = ui.ctx().screen_rect().height();
                let dialog_height = screen_height - 200.0;
                ui.set_height(dialog_height);

                ui.add_space(8.0);
                
                egui::ScrollArea::both()
                    .id_salt("settings_dialog_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(1024.0);
                        for i in 0..50 {
                            ui.horizontal(|ui| {
                                ui.label(format!("Setting Option {}", i + 1));
                                ui.text_edit_singleline(&mut format!("Value {}", i + 1));
                            });
                        }
                    });
            })
            .action("Close", || {
                println!("Settings dialog Close clicked!");
            })
            .show(ctx);
    }
}
