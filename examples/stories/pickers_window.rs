// ============================================================================
// Module:       stories::pickers_window (example)
// Description:  Stories for the date and time pickers.
//
// Dependencies: eframe, egui_material3
// ============================================================================

//! Date pickers (embedded, docked, modal, modal input, range) and time
//! pickers (dial and input, 12/24 hour, vertical and horizontal, modal).

#![doc(hidden)]

use crate::date_picker::{
    Date, DatePicker, DatePickerDialog, DateRange, DialogAction, DisplayMode, DockedDatePicker,
    Weekday,
};
use crate::time_picker::{
    Time, TimeFormat, TimeInput, TimePicker, TimePickerDialog, TimePickerLayout, TimePickerMode,
};
use crate::MaterialButton;
use eframe::egui::{self, Window};

pub(crate) struct PickersWindow {
    pub(crate) open: bool,
    date: Option<Date>,
    range: DateRange,
    docked: Option<Date>,
    dialog_date: Option<Date>,
    dialog_range: DateRange,
    date_dialog: Option<DialogKind>,
    monday_first: bool,
    time: Time,
    time_24: bool,
    horizontal: bool,
    dialog_time: Time,
    time_dialog: Option<TimePickerMode>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DialogKind {
    Single,
    Input,
    Range,
}

impl Default for PickersWindow {
    fn default() -> Self {
        Self {
            open: false,
            date: Date::new(2024, 3, 5),
            range: DateRange::default(),
            docked: None,
            dialog_date: None,
            dialog_range: DateRange::default(),
            date_dialog: None,
            monday_first: false,
            time: Time::new(14, 25).unwrap_or_default(),
            time_24: false,
            horizontal: false,
            dialog_time: Time::new(9, 0).unwrap_or_default(),
            time_dialog: None,
        }
    }
}

impl PickersWindow {
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Date & Time Picker Stories")
            .open(&mut open)
            .default_size([820.0, 900.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.date_stories(ui);
                    ui.add_space(24.0);
                    self.time_stories(ui);
                });
            });
        self.open = open;
        self.dialogs(ctx);
    }

    fn first_day(&self) -> Weekday {
        if self.monday_first {
            Weekday::Monday
        } else {
            Weekday::Sunday
        }
    }

    fn date_stories(&mut self, ui: &mut egui::Ui) {
        ui.heading("Date pickers");
        ui.checkbox(&mut self.monday_first, "Weeks start on Monday");
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(MaterialButton::filled("Modal date picker"))
                .clicked()
            {
                self.date_dialog = Some(DialogKind::Single);
            }
            if ui.add(MaterialButton::filled("Modal input")).clicked() {
                self.date_dialog = Some(DialogKind::Input);
            }
            if ui.add(MaterialButton::filled("Modal range")).clicked() {
                self.date_dialog = Some(DialogKind::Range);
            }
        });
        ui.label(format!(
            "Dialog date: {}   dialog range: {}",
            self.dialog_date
                .map_or("none".to_owned(), |d| d.to_string()),
            self.dialog_range
                .get()
                .map_or("none".to_owned(), |(s, e)| format!("{s} to {e}")),
        ));
        ui.add_space(12.0);
        ui.label("Docked");
        let first_day = self.first_day();
        ui.add(
            DockedDatePicker::new(&mut self.docked)
                .id_salt("story_docked")
                .first_day_of_week(first_day),
        );
        ui.add_space(12.0);
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.label(format!(
                    "Embedded: {}",
                    self.date.map_or("none".to_owned(), |d| d.to_string())
                ));
                ui.add(
                    DatePicker::single(&mut self.date)
                        .id_salt("story_single")
                        .first_day_of_week(first_day),
                );
            });
            ui.add_space(16.0);
            ui.vertical(|ui| {
                ui.label(format!(
                    "Range: {}",
                    self.range
                        .get()
                        .map_or("incomplete".to_owned(), |(s, e)| format!("{s} to {e}"))
                ));
                ui.add(
                    DatePicker::range(&mut self.range)
                        .id_salt("story_range")
                        .first_day_of_week(first_day)
                        .selectable_dates(|d| {
                            !matches!(d.weekday(), Weekday::Saturday | Weekday::Sunday)
                        }),
                );
            });
        });
    }

    fn time_stories(&mut self, ui: &mut egui::Ui) {
        ui.heading("Time pickers");
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.time_24, "24-hour");
            ui.checkbox(&mut self.horizontal, "Horizontal layout");
            if ui.add(MaterialButton::filled("Modal dial")).clicked() {
                self.time_dialog = Some(TimePickerMode::Dial);
            }
            if ui.add(MaterialButton::filled("Modal input")).clicked() {
                self.time_dialog = Some(TimePickerMode::Input);
            }
        });
        ui.label(format!(
            "Time: {}   dialog time: {}",
            self.time, self.dialog_time
        ));
        let format = if self.time_24 {
            TimeFormat::H24
        } else {
            TimeFormat::H12
        };
        let layout = if self.horizontal {
            TimePickerLayout::Horizontal
        } else {
            TimePickerLayout::Vertical
        };
        ui.add(
            TimePicker::new(&mut self.time)
                .id_salt("story_time")
                .format(format)
                .layout(layout),
        );
        ui.add_space(12.0);
        ui.add(
            TimeInput::new(&mut self.time)
                .id_salt("story_time_input")
                .format(format),
        );
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        let first_day = self.first_day();
        if let Some(kind) = self.date_dialog {
            let action = match kind {
                DialogKind::Single | DialogKind::Input => {
                    let mode = if kind == DialogKind::Input {
                        DisplayMode::Input
                    } else {
                        DisplayMode::Picker
                    };
                    DatePickerDialog::new(
                        DatePicker::single(&mut self.dialog_date)
                            .id_salt("story_dialog")
                            .display_mode(mode)
                            .first_day_of_week(first_day),
                    )
                    .show(ctx)
                }
                DialogKind::Range => DatePickerDialog::new(
                    DatePicker::range(&mut self.dialog_range)
                        .id_salt("story_dialog_range")
                        .first_day_of_week(first_day),
                )
                .show(ctx),
            };
            match action {
                DialogAction::Open => {}
                DialogAction::Confirmed | DialogAction::Dismissed => self.date_dialog = None,
            }
        }
        if let Some(mode) = self.time_dialog {
            let format = if self.time_24 {
                TimeFormat::H24
            } else {
                TimeFormat::H12
            };
            let layout = if self.horizontal {
                TimePickerLayout::Horizontal
            } else {
                TimePickerLayout::Vertical
            };
            match TimePickerDialog::new(&mut self.dialog_time)
                .id_salt("story_time_dialog")
                .format(format)
                .layout(layout)
                .mode(mode)
                .show(ctx)
            {
                DialogAction::Open => {}
                DialogAction::Confirmed | DialogAction::Dismissed => self.time_dialog = None,
            }
        }
    }
}
