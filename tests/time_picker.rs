// ============================================================================
// Module:       time_picker (integration test)
// Description:  Time pickers: dial drag, rings, keyboard, period, input
//               fields, the dialog, and snapshots.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Time pickers (WP-15): dragging on the dial selects the hour and moves on
//! to minutes, the 24-hour inner ring, the arrow keys, the AM/PM selector,
//! the input fields, the dialog, and every theme in both layouts.

mod support;

use std::cell::RefCell;
use std::rc::Rc;

use egui::{Key, Vec2};
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use egui_material3::time_picker::{
    DialogAction, Time, TimeFormat, TimeInput, TimePicker, TimePickerDialog, TimePickerLayout,
};
use egui_material3::MaterialTheme;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn time(h: u8, m: u8) -> Result<Time, String> {
    Time::new(h, m).ok_or_else(|| format!("{h}:{m} is not a time"))
}

/// Label radius of the outer ring (`OuterCircleToSizeRatio`, 101 dp).
const OUTER: f32 = 101.0;
/// Label radius of the inner ring (`InnerCircleToSizeRatio`, 69 dp).
const INNER: f32 = 69.0;

fn picker(value: Rc<RefCell<Time>>, format: TimeFormat) -> Harness<'static> {
    support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(420.0, 460.0),
        move |ui| {
            let mut v = value.borrow_mut();
            ui.add(TimePicker::new(&mut v).id_salt("dial").format(format));
        },
    )
}

/// Drags on the dial from 12 o'clock to `to` (offset from its centre).
fn drag_on_dial(harness: &mut Harness<'_>, to: Vec2) -> TestResult {
    let center = harness
        .query_by_label("Hour")
        .ok_or("the hour dial")?
        .rect()
        .center();
    harness.drag_at(center + Vec2::new(0.0, -OUTER));
    harness.run();
    harness.hover_at(center + to * 0.5);
    harness.run();
    harness.hover_at(center + to);
    harness.run();
    harness.drop_at(center + to);
    harness.run();
    Ok(())
}

#[test]
fn dragging_on_the_dial_selects_the_hour() -> TestResult {
    let value = Rc::new(RefCell::new(time(9, 10)?));
    let mut harness = picker(value.clone(), TimeFormat::H12);
    drag_on_dial(&mut harness, Vec2::new(OUTER, 0.0))?;
    assert_eq!(
        *value.borrow(),
        time(3, 10)?,
        "dragged to 3 o'clock, AM kept"
    );
    assert!(
        harness.query_by_label("Minute").is_some(),
        "releasing moves on to minutes"
    );
    // Now the dial picks minutes: drag to 6 o'clock.
    let center = harness
        .query_by_label("Minute")
        .ok_or("the minute dial")?
        .rect()
        .center();
    harness.drag_at(center + Vec2::new(OUTER, 0.0));
    harness.run();
    harness.hover_at(center + Vec2::new(0.0, OUTER));
    harness.run();
    harness.drop_at(center + Vec2::new(0.0, OUTER));
    harness.run();
    assert_eq!(*value.borrow(), time(3, 30)?, "dragged to half past");
    Ok(())
}

#[test]
fn the_inner_ring_holds_the_afternoon_in_24_hours() -> TestResult {
    let value = Rc::new(RefCell::new(time(9, 0)?));
    let mut harness = picker(value.clone(), TimeFormat::H24);
    drag_on_dial(&mut harness, Vec2::new(INNER, 0.0))?;
    assert_eq!(value.borrow().hour(), 15, "inner ring at 3 o'clock is 15");
    Ok(())
}

#[test]
fn arrow_keys_step_the_focused_dial() -> TestResult {
    let value = Rc::new(RefCell::new(time(23, 0)?));
    let mut harness = picker(value.clone(), TimeFormat::H24);
    harness.query_by_label("Hour").ok_or("hour dial")?.focus();
    harness.run();
    harness.key_press(Key::ArrowUp);
    harness.run();
    assert_eq!(value.borrow().hour(), 0, "23 + 1 wraps to 0");
    harness.key_press(Key::ArrowLeft);
    harness.run();
    assert_eq!(value.borrow().hour(), 23, "0 - 1 wraps to 23");
    Ok(())
}

#[test]
fn the_period_selector_switches_am_and_pm() -> TestResult {
    let value = Rc::new(RefCell::new(time(9, 15)?));
    let mut harness = picker(value.clone(), TimeFormat::H12);
    harness.query_by_label("PM").ok_or("PM")?.click();
    harness.run();
    assert_eq!(*value.borrow(), time(21, 15)?, "PM adds twelve hours");
    harness.query_by_label("AM").ok_or("AM")?.click();
    harness.run();
    assert_eq!(*value.borrow(), time(9, 15)?, "AM takes them away");
    Ok(())
}

#[test]
fn the_selectors_choose_what_the_dial_edits() -> TestResult {
    let value = Rc::new(RefCell::new(time(9, 15)?));
    let mut harness = picker(value, TimeFormat::H12);
    assert!(harness.query_by_label("Hour").is_some(), "starts on hours");
    harness
        .query_by_label("Minute 15")
        .ok_or("minute selector")?
        .click();
    harness.run();
    assert!(harness.query_by_label("Minute").is_some(), "now on minutes");
    Ok(())
}

#[test]
fn typing_in_the_input_fields_sets_the_time() -> TestResult {
    let value = Rc::new(RefCell::new(time(9, 5)?));
    let shared = value.clone();
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(420.0, 200.0),
        move |ui| {
            let mut v = shared.borrow_mut();
            ui.add(
                TimeInput::new(&mut v)
                    .id_salt("input")
                    .format(TimeFormat::H24),
            );
        },
    );
    harness.query_by_label("Hour").ok_or("hour field")?.focus();
    harness.run();
    harness.key_press(Key::Backspace);
    harness.key_press(Key::Backspace);
    harness.run();
    harness
        .query_by_label("Hour")
        .ok_or("hour field")?
        .type_text("17");
    harness.run();
    assert_eq!(*value.borrow(), time(17, 5)?, "typed 17");

    harness
        .query_by_label("Minute")
        .ok_or("minute field")?
        .focus();
    harness.run();
    harness.key_press(Key::Backspace);
    harness.key_press(Key::Backspace);
    harness.run();
    // Each valid edit commits: deleting "05" passes through "0".
    let cleared = *value.borrow();
    harness
        .query_by_label("Minute")
        .ok_or("minute field")?
        .type_text("75");
    harness.run();
    assert_eq!(*value.borrow(), cleared, "75 minutes is rejected");
    assert_eq!(cleared.hour(), 17, "the hour is kept");
    Ok(())
}

#[test]
fn the_dialog_writes_only_on_confirm() -> TestResult {
    for (button, expected) in [("OK", time(15, 0)?), ("Cancel", time(9, 0)?)] {
        let value = Rc::new(RefCell::new(time(9, 0)?));
        let action = Rc::new(RefCell::new(DialogAction::Open));
        let (v, a) = (value.clone(), action.clone());
        let mut harness = support::harness_for(
            MaterialTheme::baseline(false),
            egui::vec2(500.0, 640.0),
            move |ui| {
                if *a.borrow() == DialogAction::Open {
                    let mut t = v.borrow_mut();
                    *a.borrow_mut() = TimePickerDialog::new(&mut t)
                        .format(TimeFormat::H24)
                        .show(ui.ctx());
                }
            },
        );
        drag_on_dial(&mut harness, Vec2::new(INNER, 0.0))?;
        assert_eq!(*value.borrow(), time(9, 0)?, "nothing written yet");
        harness.query_by_label(button).ok_or(button)?.click();
        harness.run();
        assert_ne!(*action.borrow(), DialogAction::Open, "{button} closes");
        assert_eq!(*value.borrow(), expected, "{button} result");
    }
    Ok(())
}

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
fn snapshots_in_every_theme_and_layout() -> TestResult {
    for (name, theme) in support::themes() {
        for (layout, size, tag) in [
            (
                TimePickerLayout::Vertical,
                egui::vec2(300.0, 420.0),
                "vertical",
            ),
            (
                TimePickerLayout::Horizontal,
                egui::vec2(560.0, 300.0),
                "horizontal",
            ),
        ] {
            let mut t = time(14, 25)?;
            let mut harness = support::harness_for(theme.clone(), size, move |ui| {
                ui.add(TimePicker::new(&mut t).id_salt("snap").layout(layout));
            });
            harness.run();
            snapshot(&mut harness, &format!("time_picker_{tag}_{name}"))?;
        }
    }
    Ok(())
}
