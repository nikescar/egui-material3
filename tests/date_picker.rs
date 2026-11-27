// ============================================================================
// Module:       date_picker (integration test)
// Description:  Date pickers: keyboard grid navigation, range selection,
//               year list, input validation, the dialog, and snapshots.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Date pickers (WP-15): arrow keys move the day focus (across months), a
//! range selection returns `(start, end)`, the year list changes the year,
//! the input mode validates, the dialog writes only on confirm, and every
//! theme renders.

mod support;

use std::cell::RefCell;
use std::rc::Rc;

use egui::Key;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use egui_material3::date_picker::{
    Date, DatePicker, DatePickerDialog, DateRange, DialogAction, DisplayMode,
};
use egui_material3::MaterialTheme;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn date(y: i32, m: u8, d: u8) -> Result<Date, String> {
    Date::new(y, m, d).ok_or_else(|| format!("{y}-{m}-{d} is not a date"))
}

fn today() -> Result<Date, String> {
    date(2024, 3, 15)
}

const SIZE: egui::Vec2 = egui::vec2(380.0, 560.0);

/// A harness drawing a single-date picker on `value`.
fn single(value: Rc<RefCell<Option<Date>>>, mode: DisplayMode) -> Result<Harness<'static>, String> {
    let today = today()?;
    Ok(support::harness_for(
        MaterialTheme::baseline(false),
        SIZE,
        move |ui| {
            let mut v = value.borrow_mut();
            ui.add(
                DatePicker::single(&mut v)
                    .id_salt("single")
                    .today(today)
                    .display_mode(mode),
            );
        },
    ))
}

fn focused(harness: &Harness<'_>, label: &str) -> Result<bool, String> {
    harness
        .query_by_label(label)
        .map(|n| n.is_focused())
        .ok_or_else(|| format!("no node labelled {label:?}"))
}

#[test]
fn arrow_keys_move_the_day_focus() -> TestResult {
    let value = Rc::new(RefCell::new(Some(date(2024, 3, 5)?)));
    let mut harness = single(value.clone(), DisplayMode::Picker)?;
    harness
        .query_by_label("Tuesday, March 5, 2024")
        .ok_or("March 5 is drawn")?
        .focus();
    harness.run();
    assert!(
        focused(&harness, "Tuesday, March 5, 2024")?,
        "March 5 focused"
    );

    harness.key_press(Key::ArrowRight);
    harness.run();
    assert!(
        focused(&harness, "Wednesday, March 6, 2024")?,
        "right arrow moves to March 6"
    );

    harness.key_press(Key::ArrowDown);
    harness.run();
    assert!(
        focused(&harness, "Wednesday, March 13, 2024")?,
        "down arrow moves a week"
    );

    harness.key_press(Key::ArrowLeft);
    harness.run();
    assert!(
        focused(&harness, "Tuesday, March 12, 2024")?,
        "left arrow moves back a day"
    );

    harness.key_press(Key::ArrowUp);
    harness.run();
    assert!(
        focused(&harness, "Tuesday, March 5, 2024")?,
        "up arrow moves back a week"
    );
    assert_eq!(
        *value.borrow(),
        Some(date(2024, 3, 5)?),
        "moving focus does not select"
    );

    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(*value.borrow(), Some(date(2024, 3, 5)?), "Enter selects");
    Ok(())
}

#[test]
fn arrow_keys_cross_into_the_next_month() -> TestResult {
    let value = Rc::new(RefCell::new(None));
    let mut harness = single(value.clone(), DisplayMode::Picker)?;
    harness
        .query_by_label("Saturday, March 30, 2024")
        .ok_or("March 30 is drawn")?
        .focus();
    harness.run();
    harness.key_press(Key::ArrowDown);
    harness.run();
    assert!(
        focused(&harness, "Saturday, April 6, 2024")?,
        "down from March 30 lands on April 6"
    );
    assert!(
        harness
            .query_by_label("April 2024, Switch to selecting a year")
            .is_some(),
        "the grid shows April"
    );
    harness.key_press(Key::PageUp);
    harness.run();
    assert!(
        focused(&harness, "Wednesday, March 6, 2024")?,
        "Page Up goes back a month"
    );
    harness.key_press(Key::Space);
    harness.run();
    assert_eq!(*value.borrow(), Some(date(2024, 3, 6)?), "Space selects");
    Ok(())
}

#[test]
fn a_range_selection_returns_start_and_end() -> TestResult {
    let range = Rc::new(RefCell::new(DateRange::default()));
    let today = today()?;
    let shared = range.clone();
    let mut harness = support::harness_for(MaterialTheme::baseline(false), SIZE, move |ui| {
        let mut r = shared.borrow_mut();
        ui.add(DatePicker::range(&mut r).id_salt("range").today(today));
    });
    harness
        .query_by_label("Thursday, March 7, 2024")
        .ok_or("March 7")?
        .click();
    harness.run();
    assert_eq!(range.borrow().get(), None, "one tap is only a start");
    harness
        .query_by_label("Friday, March 22, 2024")
        .ok_or("March 22")?
        .click();
    harness.run();
    assert_eq!(
        range.borrow().get(),
        Some((date(2024, 3, 7)?, date(2024, 3, 22)?)),
        "two taps give (start, end)"
    );
    assert!(
        harness
            .query_by_label("Mar 7, 2024 – Mar 22, 2024")
            .is_some(),
        "the headline shows the range"
    );
    Ok(())
}

#[test]
fn the_year_list_changes_the_year() -> TestResult {
    let value = Rc::new(RefCell::new(None));
    let mut harness = single(value, DisplayMode::Picker)?;
    harness
        .query_by_label("March 2024, Switch to selecting a year")
        .ok_or("month/year button")?
        .click();
    harness.run();
    let year = harness
        .query_by_label("2026")
        .ok_or("2026 in the year list")?;
    year.click();
    harness.run();
    assert!(
        harness
            .query_by_label("March 2026, Switch to selecting a year")
            .is_some(),
        "picking 2026 shows March 2026"
    );
    Ok(())
}

#[test]
fn input_mode_validates_typed_dates() -> TestResult {
    for (typed, expected) in [
        ("2024-04-07", Some(date(2024, 4, 7)?)),
        ("2024-13-01", None),
        ("1800-01-01", None),
        ("07/04/2024", None),
    ] {
        let value = Rc::new(RefCell::new(None));
        let mut harness = single(value.clone(), DisplayMode::Input)?;
        let field = harness.query_by_label("Date").ok_or("the date field")?;
        field.focus();
        harness.run();
        harness
            .query_by_label("Date")
            .ok_or("the date field")?
            .type_text(typed);
        harness.run();
        assert_eq!(*value.borrow(), expected, "typing {typed:?}");
    }
    Ok(())
}

#[test]
fn the_dialog_writes_only_on_confirm() -> TestResult {
    let today = today()?;
    for (button, expected_action, expected_value) in [
        ("OK", DialogAction::Confirmed, Some(date(2024, 3, 20)?)),
        ("Cancel", DialogAction::Dismissed, None),
    ] {
        let value = Rc::new(RefCell::new(None));
        let action = Rc::new(RefCell::new(DialogAction::Open));
        let (v, a) = (value.clone(), action.clone());
        let mut harness = support::harness_for(MaterialTheme::baseline(false), SIZE, move |ui| {
            if *a.borrow() == DialogAction::Open {
                let mut d = v.borrow_mut();
                let picker = DatePicker::single(&mut d).id_salt("dialog").today(today);
                *a.borrow_mut() = DatePickerDialog::new(picker).show(ui.ctx());
            }
        });
        harness
            .query_by_label("Wednesday, March 20, 2024")
            .ok_or("March 20 in the dialog")?
            .click();
        harness.run();
        assert_eq!(*value.borrow(), None, "nothing written before confirming");
        harness.query_by_label(button).ok_or(button)?.click();
        harness.run();
        assert_eq!(
            *action.borrow(),
            expected_action,
            "{button} ends the dialog"
        );
        assert_eq!(*value.borrow(), expected_value, "{button} result");
    }
    Ok(())
}

#[test]
fn days_outside_the_selectable_set_are_disabled() -> TestResult {
    let value = Rc::new(RefCell::new(None));
    let today = today()?;
    let shared = value.clone();
    let mut harness = support::harness_for(MaterialTheme::baseline(false), SIZE, move |ui| {
        let mut v = shared.borrow_mut();
        ui.add(
            DatePicker::single(&mut v)
                .id_salt("weekdays")
                .today(today)
                .selectable_dates(|d| d.day() % 2 == 0),
        );
    });
    harness
        .query_by_label("Thursday, March 7, 2024")
        .ok_or("March 7")?
        .click();
    harness.run();
    assert_eq!(*value.borrow(), None, "an odd day cannot be picked");
    harness
        .query_by_label("Friday, March 8, 2024")
        .ok_or("March 8")?
        .click();
    harness.run();
    assert_eq!(*value.borrow(), Some(date(2024, 3, 8)?), "an even day can");
    Ok(())
}

/// Serialises snapshot rendering: one GPU renderer at a time.
fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Runs a render, turning a panic into an error: with no GPU adapter at
/// all, egui_kittest's renderer panics instead of returning one.
fn no_adapter_is_an_error<T>(
    render: impl FnOnce() -> Result<T, egui_kittest::SnapshotError>,
) -> Result<T, egui_kittest::SnapshotError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(render)).unwrap_or_else(|_| {
        Err(egui_kittest::SnapshotError::RenderError {
            err: "no adapter: the renderer could not be created".to_owned(),
        })
    })
}

fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(600);
    match no_adapter_is_an_error(|| harness.try_snapshot_options(name, &options)) {
        Ok(()) => Ok(()),
        Err(egui_kittest::SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(other) => Err(format!("{other:?}").into()),
    }
}

#[test]
fn snapshots_in_every_theme() -> TestResult {
    let today = today()?;
    for (name, theme) in support::themes() {
        let mut range = DateRange::new(date(2024, 3, 7)?, date(2024, 3, 12)?);
        let mut harness = support::harness_for(theme, SIZE, move |ui| {
            ui.add(DatePicker::range(&mut range).id_salt("snap").today(today));
        });
        harness.run();
        snapshot(&mut harness, &format!("date_picker_range_{name}"))?;
    }
    Ok(())
}
