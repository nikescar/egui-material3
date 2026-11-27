#![doc(hidden)]

use crate::{action_sheet, BottomSheet, MaterialButton, SideSheet};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct ActionSheetWindow {
    pub open: bool,
    // Action sheet states
    one_group_open: bool,
    two_groups_open: bool,
    with_labels_open: bool,
    many_actions_open: bool,
    custom_open: bool,
    // Bottom and side sheets
    modal_sheet_open: bool,
    standard_sheet_open: bool,
    modal_side_open: bool,
    standard_side_open: bool,
    last_close: String,
    // Customization options
    backdrop: bool,
    backdrop_dismissible: bool,
    max_width: f32,
}

impl Default for ActionSheetWindow {
    fn default() -> Self {
        Self {
            open: false,
            one_group_open: false,
            two_groups_open: false,
            with_labels_open: false,
            many_actions_open: false,
            custom_open: false,
            modal_sheet_open: false,
            standard_sheet_open: false,
            modal_side_open: false,
            standard_side_open: false,
            last_close: String::new(),
            backdrop: true,
            backdrop_dismissible: true,
            max_width: 448.0,
        }
    }
}

impl ActionSheetWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Action Sheet Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_triggers(ui);
                });
            });
        self.open = open;

        // Show action sheets
        self.show_action_sheets(ctx);
        self.show_sheets(ctx);
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Action Sheet Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://konstaui.com/react/action-sheet");
            }
        });

        ui.checkbox(&mut self.backdrop, "Show backdrop");
        ui.checkbox(&mut self.backdrop_dismissible, "Backdrop dismissible");

        ui.horizontal(|ui| {
            ui.label("Max Width:");
            ui.add(egui::DragValue::new(&mut self.max_width).range(280.0..=800.0));
        });
    }

    fn render_triggers(&mut self, ui: &mut egui::Ui) {
        ui.heading("Open Action Sheet");

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled("One Group")).clicked() {
                self.one_group_open = true;
            }

            if ui.add(MaterialButton::filled("Two Groups")).clicked() {
                self.two_groups_open = true;
            }

            if ui.add(MaterialButton::filled("With Labels")).clicked() {
                self.with_labels_open = true;
            }

            if ui.add(MaterialButton::filled("Many Actions")).clicked() {
                self.many_actions_open = true;
            }

            if ui
                .add(MaterialButton::filled_tonal("Custom Options"))
                .clicked()
            {
                self.custom_open = true;
            }
        });

        ui.add_space(20.0);
        ui.heading("Bottom and Side Sheets");
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(MaterialButton::filled("Modal Bottom Sheet"))
                .clicked()
            {
                self.modal_sheet_open = true;
            }
            if ui
                .add(MaterialButton::filled_tonal("Standard Bottom Sheet"))
                .clicked()
            {
                self.standard_sheet_open = !self.standard_sheet_open;
            }
            if ui.add(MaterialButton::filled("Modal Side Sheet")).clicked() {
                self.modal_side_open = true;
            }
            if ui
                .add(MaterialButton::filled_tonal("Standard Side Sheet"))
                .clicked()
            {
                self.standard_side_open = !self.standard_side_open;
            }
        });
        if !self.last_close.is_empty() {
            ui.label(format!("Last sheet closed by: {}", self.last_close));
        }
    }

    // Sheets are shown every frame (not only while open) so they can
    // animate out.
    fn show_sheets(&mut self, ctx: &egui::Context) {
        let modal = BottomSheet::modal(
            egui::Id::unique("story_modal_sheet"),
            &mut self.modal_sheet_open,
        )
        .show(ctx, |ui, open| {
            ui.add_space(8.0);
            ui.heading("Modal bottom sheet");
            ui.label("Drag it down past 56 dp, click the scrim or the handle, or press Escape.");
            ui.add_space(16.0);
            if ui.add(MaterialButton::text("Close from content")).clicked() {
                *open = false;
            }
            ui.add_space(24.0);
        });
        let standard = BottomSheet::standard(
            egui::Id::unique("story_standard_sheet"),
            &mut self.standard_sheet_open,
        )
        .show(ctx, |ui, _| {
            ui.add_space(8.0);
            ui.heading("Standard bottom sheet");
            ui.label("Drag or click the handle to collapse to the 56 dp peek height.");
            ui.add_space(120.0);
        });
        let modal_side = SideSheet::modal(
            egui::Id::unique("story_modal_side"),
            &mut self.modal_side_open,
        )
        .title("Filters")
        .show(ctx, |ui, open| {
            ui.label("A modal side sheet on the end edge.");
            if ui.add(MaterialButton::filled("Apply")).clicked() {
                *open = false;
            }
        });
        let standard_side = SideSheet::standard(
            egui::Id::unique("story_standard_side"),
            &mut self.standard_side_open,
        )
        .title("Details")
        .show(ctx, |ui, _| {
            ui.label("A standard side sheet does not block the app.");
        });
        let reasons = [
            modal.close_reason,
            standard.close_reason,
            modal_side.close_reason,
            standard_side.close_reason,
        ];
        if let Some(reason) = reasons.into_iter().flatten().next() {
            self.last_close = format!("{reason:?}");
        }
    }

    fn show_action_sheets(&mut self, ctx: &egui::Context) {
        // One Group Action Sheet
        if self.one_group_open {
            action_sheet(egui::Id::unique("one_group"), &mut self.one_group_open)
                .label("Do something")
                .bold_button("Button 1", || {
                    println!("Button 1 clicked!");
                })
                .button("Button 2", || {
                    println!("Button 2 clicked!");
                })
                .button("Cancel", || {
                    println!("Cancel clicked!");
                })
                .show(ctx);
        }

        // Two Groups Action Sheet
        if self.two_groups_open {
            action_sheet(egui::Id::unique("two_groups"), &mut self.two_groups_open)
                .label("Do something")
                .bold_button("Button 1", || {
                    println!("Button 1 clicked!");
                })
                .button("Button 2", || {
                    println!("Button 2 clicked!");
                })
                .new_group() // Start new group
                .button("Cancel", || {
                    println!("Cancel clicked!");
                })
                .show(ctx);
        }

        // With Labels Action Sheet
        if self.with_labels_open {
            action_sheet(egui::Id::unique("with_labels"), &mut self.with_labels_open)
                .label("Choose an action")
                .button("Share", || {
                    println!("Share clicked!");
                })
                .button("Copy Link", || {
                    println!("Copy Link clicked!");
                })
                .new_group()
                .label("Danger Zone")
                .button("Delete", || {
                    println!("Delete clicked!");
                })
                .new_group()
                .button("Cancel", || {
                    println!("Cancel clicked!");
                })
                .show(ctx);
        }

        // Many Actions Action Sheet
        if self.many_actions_open {
            let mut sheet = action_sheet(
                egui::Id::unique("many_actions"),
                &mut self.many_actions_open,
            )
            .label("Choose an option");

            // Add many buttons
            for i in 1..=10 {
                let button_text = format!("Option {}", i);
                sheet = sheet.button(button_text, move || {
                    println!("Option {} clicked!", i);
                });
            }

            sheet = sheet.new_group().button("Cancel", || {
                println!("Cancel clicked!");
            });

            sheet.show(ctx);
        }

        // Custom Options Action Sheet
        if self.custom_open {
            action_sheet(egui::Id::unique("custom"), &mut self.custom_open)
                .backdrop(self.backdrop)
                .backdrop_dismissible(self.backdrop_dismissible)
                .max_width(self.max_width)
                .label("Custom Settings")
                .button("Action 1", || {
                    println!("Action 1 clicked!");
                })
                .button("Action 2", || {
                    println!("Action 2 clicked!");
                })
                .button("Action 3", || {
                    println!("Action 3 clicked!");
                })
                .new_group()
                .button("Cancel", || {
                    println!("Cancel clicked!");
                })
                .show(ctx);
        }
    }
}
