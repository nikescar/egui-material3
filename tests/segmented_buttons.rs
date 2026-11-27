// ============================================================================
// Module:       segmented_buttons (integration test)
// Description:  Segmented buttons: geometry, selection, keyboard, AccessKit.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Segmented buttons (WP-11): the 40 dp height, single and multi
//! selection by pointer and keyboard (arrows rove; in a single-select group
//! they move the selection and only the selected segment is a Tab stop),
//! disabled segments, icon-only segments, the AccessKit tree, and snapshots.

mod support;

use std::cell::{Cell, RefCell};

use egui::accesskit::{Role, Toggled};
use egui::{Key, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::{MaterialTheme, Segment, SegmentedButton};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn size() -> Vec2 {
    Vec2::new(600.0, 200.0)
}

#[test]
fn segments_are_40_tall_and_equally_wide() -> TestResult {
    let harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        let mut selected = 0;
        ui.add(
            SegmentedButton::single(&mut selected)
                .label("View")
                .segment("Day")
                .segment("A much longer week")
                .segment("Month"),
        );
    });
    let day = harness.get_by_label("Day").rect();
    let week = harness.get_by_label("A much longer week").rect();
    assert_eq!(
        day.height(),
        40.0,
        "OutlinedSegmentedButtonTokens.ContainerHeight"
    );
    assert_eq!(day.width(), week.width(), "segments share the widest width");
    assert_eq!(
        week.min.x,
        day.max.x - 1.0,
        "neighbours overlap by the 1 dp outline"
    );
    let group = harness.get_by_label("View");
    assert_eq!(
        group.accesskit_node().role(),
        Role::RadioGroup,
        "group role"
    );
    Ok(())
}

#[test]
fn single_select_reports_radio_buttons_and_clicks_select() -> TestResult {
    for (name, theme) in support::themes() {
        let selected = Cell::new(0_usize);
        let changed = Cell::new(0);
        let mut harness = support::harness_for(theme, size(), |ui| {
            let mut s = selected.get();
            let response =
                ui.add(SegmentedButton::single(&mut s).segments(["Day", "Week", "Month"]));
            if response.changed() {
                changed.set(changed.get() + 1);
            }
            selected.set(s);
        });
        let toggled = |h: &egui_kittest::Harness<'_>, label: &str| {
            h.get_by_label(label).accesskit_node().toggled()
        };
        assert_eq!(
            harness.get_by_label("Week").accesskit_node().role(),
            Role::RadioButton,
            "{name}: radio role"
        );
        assert_eq!(
            toggled(&harness, "Day"),
            Some(Toggled::True),
            "{name}: Day selected"
        );
        harness.get_by_label("Month").click();
        harness.run();
        assert_eq!(selected.get(), 2, "{name}: clicking Month selects it");
        assert_eq!(changed.get(), 1, "{name}: one change reported");
        assert_eq!(
            toggled(&harness, "Month"),
            Some(Toggled::True),
            "{name}: Month selected"
        );
        assert_eq!(
            toggled(&harness, "Day"),
            Some(Toggled::False),
            "{name}: Day cleared"
        );
        harness.get_by_label("Month").click();
        harness.run();
        assert_eq!(
            changed.get(),
            1,
            "{name}: re-clicking the selection is no change"
        );
    }
    Ok(())
}

#[test]
fn arrows_move_a_single_selection_and_skip_disabled_segments() -> TestResult {
    let selected = Cell::new(0_usize);
    let mut harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        let mut s = selected.get();
        ui.add(
            SegmentedButton::single(&mut s)
                .segment("One")
                .segment(Segment::new("Two").enabled(false))
                .segment("Three"),
        );
        selected.set(s);
    });
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("One").is_focused(),
        "Tab reaches the selected segment"
    );
    harness.key_press(Key::ArrowRight);
    harness.run();
    assert_eq!(
        selected.get(),
        2,
        "Right skips the disabled segment and selects Three"
    );
    assert!(
        harness.get_by_label("Three").is_focused(),
        "focus follows the selection"
    );
    harness.key_press(Key::ArrowRight);
    harness.run();
    assert_eq!(selected.get(), 0, "Right wraps to the first segment");
    harness.key_press(Key::ArrowLeft);
    harness.run();
    assert_eq!(selected.get(), 2, "Left wraps back to the last");
    harness.key_press(Key::Home);
    harness.run();
    assert_eq!(selected.get(), 0, "Home selects the first");
    assert!(
        harness.get_by_label("Two").accesskit_node().is_disabled(),
        "the disabled segment is reported disabled"
    );
    Ok(())
}

#[test]
fn only_the_selected_segment_is_a_tab_stop_in_single_select() -> TestResult {
    let mut harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        let mut s = 1_usize;
        ui.add(SegmentedButton::single(&mut s).segments(["A", "B", "C"]));
        ui.add(egui::Button::new("After"));
    });
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("B").is_focused(),
        "the first Tab lands on the selection"
    );
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("After").is_focused(),
        "the next Tab leaves the group"
    );
    Ok(())
}

#[test]
fn multi_select_toggles_with_pointer_and_space_and_arrows_only_move_focus() -> TestResult {
    let flags = RefCell::new([false, true, false]);
    let mut harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        let mut f = *flags.borrow();
        ui.add(
            SegmentedButton::multi(&mut f)
                .label("Sizes")
                .segment(Segment::icon_only("format_bold", "Bold"))
                .segment(Segment::icon_only("format_italic", "Italic"))
                .segment(Segment::new("Underline").icon("format_underlined")),
        );
        *flags.borrow_mut() = f;
    });
    let bold = harness.get_by_label("Bold");
    assert_eq!(
        bold.accesskit_node().role(),
        Role::CheckBox,
        "multi uses checkboxes"
    );
    assert_eq!(
        harness.get_by_label("Sizes").accesskit_node().role(),
        Role::Group,
        "multi group role"
    );
    assert_eq!(
        harness.get_by_label("Italic").accesskit_node().toggled(),
        Some(Toggled::True),
        "Italic starts on"
    );
    harness.get_by_label("Bold").click();
    harness.run();
    assert_eq!(
        *flags.borrow(),
        [true, true, false],
        "click toggles Bold on"
    );
    harness.key_press(Key::ArrowRight);
    harness.run();
    assert_eq!(*flags.borrow(), [true, true, false], "arrows do not toggle");
    assert!(
        harness.get_by_label("Italic").is_focused(),
        "Right moves focus"
    );
    harness.key_press(Key::Space);
    harness.run();
    assert_eq!(
        *flags.borrow(),
        [true, false, false],
        "Space toggles Italic off"
    );
    Ok(())
}

#[test]
fn a_disabled_group_does_not_change() -> TestResult {
    let selected = Cell::new(0_usize);
    let harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        let mut s = selected.get();
        ui.add(
            SegmentedButton::single(&mut s)
                .enabled(false)
                .segments(["One", "Two"]),
        );
        selected.set(s);
    });
    harness.get_by_label("Two").click();
    assert_eq!(selected.get(), 0, "clicks on a disabled group do nothing");
    assert!(
        harness.get_by_label("Two").accesskit_node().is_disabled(),
        "segments report disabled"
    );
    Ok(())
}

#[test]
fn full_width_splits_the_available_width() -> TestResult {
    let harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        let mut s = 0_usize;
        ui.add(
            SegmentedButton::single(&mut s)
                .full_width(true)
                .segments(["A", "B"]),
        );
    });
    let a = harness.get_by_label("A").rect();
    let b = harness.get_by_label("B").rect();
    assert!(
        (a.width() - b.width()).abs() < 0.01 && a.width() > 250.0,
        "two segments split ~600 dp: {a:?} {b:?}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshots.

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

fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn snapshot(harness: &mut egui_kittest::Harness<'_>, name: &str) -> TestResult {
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
        Err(err) => Err(err.to_string().into()),
    }
}

#[test]
fn segmented_button_snapshots() -> TestResult {
    for (name, theme) in support::themes() {
        let mut harness = support::harness_for(theme, Vec2::new(420.0, 170.0), |ui| {
            let mut single = 1_usize;
            ui.add(
                SegmentedButton::single(&mut single)
                    .segment(Segment::new("Day").icon("calendar_view_day"))
                    .segment("Week")
                    .segment("Month"),
            );
            let mut multi = [true, false, true];
            ui.add(SegmentedButton::multi(&mut multi).segments(["S", "M", "L"]));
            let mut disabled = 0_usize;
            ui.add(
                SegmentedButton::single(&mut disabled)
                    .enabled(false)
                    .segments(["On", "Off"]),
            );
        });
        harness.key_press(Key::Tab);
        harness.run();
        snapshot(&mut harness, &format!("segmented_button_{name}"))?;
    }
    Ok(())
}
