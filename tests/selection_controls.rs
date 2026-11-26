//! Selection controls (checkbox, radio, switch) against Compose's
//! `CheckboxTokens`, `RadioButtonTokens`, `SwitchTokens` and `ListTokens`:
//! geometry from the AccessKit tree, colours from the painted shapes, the
//! keyboard model, and snapshots of every state in light and dark.

mod support;

use egui::accesskit::{Role, Toggled};
use egui::epaint::{ColorMode, Shape};
use egui::{Color32, Pos2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use egui_material3::checkbox::ToggleableState;
use egui_material3::{
    ColorRole, MaterialButton, MaterialCheckbox, MaterialRadio, MaterialRadioGroup, MaterialSwitch,
    MaterialTheme, RadioListTile,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Every shape painted in the last frame, with nested shape lists flattened.
fn shapes(harness: &Harness<'_>) -> Vec<Shape> {
    fn flatten(shape: &Shape, out: &mut Vec<Shape>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|s| flatten(s, out)),
            other => out.push(other.clone()),
        }
    }
    let mut out = Vec::new();
    for clipped in &harness.output().shapes {
        flatten(&clipped.shape, &mut out);
    }
    out
}

/// Filled circles of `radius`: their centres and fills.
fn circles(harness: &Harness<'_>, radius: f32) -> Vec<(Pos2, Color32)> {
    shapes(harness)
        .into_iter()
        .filter_map(|shape| match shape {
            Shape::Circle(c) if (c.radius - radius).abs() < 0.01 && c.fill.a() > 0 => {
                Some((c.center, c.fill))
            }
            _ => None,
        })
        .collect()
}

/// Snapshot `name`, skipping (not failing) where no GPU adapter exists.
fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(200);
    match harness.try_snapshot_options(name, &options) {
        Ok(()) => Ok(()),
        Err(egui_kittest::SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

#[test]
fn controls_have_48dp_touch_targets() {
    let mut checked = false;
    let mut choice = Some(0);
    let mut on = false;
    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(400.0, 300.0),
        |ui| {
            ui.add(MaterialCheckbox::new(&mut checked, "Box"));
            ui.add(MaterialRadio::new(&mut choice, 0, "Dot"));
            ui.add(MaterialSwitch::new(&mut on).text("Toggle"));
        },
    );
    for label in ["Box", "Dot", "Toggle"] {
        let height = harness.get_by_label(label).rect().height();
        assert_eq!(height, 48.0, "{label}: 48 dp touch target, got {height}");
    }
}

#[test]
fn switch_thumb_centre_is_16_off_and_36_on() {
    let mut on = false;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(200.0, 100.0),
        |ui| {
            ui.add(MaterialSwitch::new(&mut on).text("Wi-Fi"));
        },
    );
    let node = harness.get_by_label("Wi-Fi").rect();
    let off = circles(&harness, 8.0);
    assert_eq!(off.len(), 1, "one 16 dp thumb when off: {off:?}");
    let off_x = off[0].0.x - node.min.x;
    assert!(
        (off_x - 16.0).abs() < 0.01,
        "off thumb centre at 16, got {off_x}"
    );
    assert!(
        (off[0].0.y - node.center().y).abs() < 0.01,
        "thumb vertically centred"
    );

    harness.get_by_label("Wi-Fi").click();
    harness.run();
    // Move the pointer away so the thumb is not in its hovered colour.
    harness.remove_cursor();
    harness.run();
    let on_thumbs = circles(&harness, 12.0);
    assert_eq!(on_thumbs.len(), 1, "one 24 dp thumb when on: {on_thumbs:?}");
    let on_x = on_thumbs[0].0.x - node.min.x;
    assert!(
        (on_x - 36.0).abs() < 0.01,
        "on thumb centre at 36, got {on_x}"
    );
    drop(harness);
    assert!(on, "clicking turns the switch on");
}

#[test]
fn switch_thumb_has_icon_size_when_off_with_an_icon() {
    let mut on = false;
    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(200.0, 100.0),
        |ui| {
            ui.add(
                MaterialSwitch::new(&mut on)
                    .unselected_icon('x')
                    .text("Icon"),
            );
        },
    );
    assert_eq!(
        circles(&harness, 12.0).len(),
        1,
        "an unselected thumb with an icon is 24 dp"
    );
}

#[test]
fn switch_colours_follow_tokens() {
    let theme = MaterialTheme::baseline(false);
    let surface = theme.color(ColorRole::Surface);
    let on_primary = theme.color(ColorRole::OnPrimary);
    let outline = theme.color(ColorRole::Outline);
    let mut on = true;
    let mut off = false;
    let mut locked = true;
    let harness = support::harness_for(theme, egui::vec2(300.0, 200.0), |ui| {
        ui.add(MaterialSwitch::new(&mut on).text("On"));
        ui.add(MaterialSwitch::new(&mut off).text("Off"));
        ui.add(
            MaterialSwitch::new(&mut locked)
                .text("Locked")
                .enabled(false),
        );
    });
    let thumb_in = |label: &str, radius: f32| {
        let rect = harness.get_by_label(label).rect();
        circles(&harness, radius)
            .into_iter()
            .find(|(c, _)| c.y > rect.min.y && c.y < rect.max.y)
            .map(|(_, fill)| fill)
    };
    assert_eq!(
        thumb_in("On", 12.0),
        Some(on_primary),
        "selected thumb onPrimary"
    );
    assert_eq!(
        thumb_in("Off", 8.0),
        Some(outline),
        "unselected thumb outline"
    );
    assert_eq!(
        thumb_in("Locked", 12.0),
        Some(surface),
        "disabled selected thumb is surface (DisabledSelectedHandleColor)"
    );
}

#[test]
fn disabled_checked_checkbox_draws_its_mark_in_surface() {
    let theme = MaterialTheme::baseline(false);
    let surface = theme.color(ColorRole::Surface);
    let container = theme
        .color(ColorRole::OnSurface)
        .gamma_multiply(egui_material3::tokens::state::DISABLED_CONTENT_OPACITY);
    let mut checked = true;
    let harness = support::harness_for(theme, egui::vec2(200.0, 80.0), |ui| {
        ui.add(MaterialCheckbox::new(&mut checked, "Locked").enabled(false));
    });
    let marks: Vec<Color32> = shapes(&harness)
        .into_iter()
        .filter_map(|shape| match shape {
            Shape::Path(path) if !path.closed => match path.stroke.color {
                ColorMode::Solid(color) => Some(color),
                ColorMode::UV(_) => None,
            },
            _ => None,
        })
        .collect();
    assert!(!marks.is_empty(), "a checked box draws a mark");
    assert!(
        marks.iter().all(|c| *c == surface && *c != container),
        "the disabled mark is surface, not the container colour: {marks:?}"
    );
}

#[test]
fn tri_state_checkbox_reports_mixed_and_checks_on_click() {
    let mut state = ToggleableState::Indeterminate;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(200.0, 80.0),
        |ui| {
            ui.add(MaterialCheckbox::tri_state(&mut state, "All files"));
        },
    );
    let node = harness.get_by_label("All files");
    assert_eq!(
        node.accesskit_node().role(),
        Role::CheckBox,
        "checkbox role"
    );
    assert_eq!(
        node.accesskit_node().toggled(),
        Some(Toggled::Mixed),
        "indeterminate is reported as mixed"
    );
    harness.get_by_label("All files").click();
    harness.run();
    assert_eq!(
        harness.get_by_label("All files").accesskit_node().toggled(),
        Some(Toggled::True),
        "a click checks an indeterminate box"
    );
    harness.get_by_label("All files").click();
    harness.run();
    drop(harness);
    assert_eq!(state, ToggleableState::Off, "a second click clears it");
}

#[test]
fn space_toggles_a_focused_checkbox_and_switch() {
    let mut checked = false;
    let mut on = false;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(300.0, 150.0),
        |ui| {
            ui.add(MaterialCheckbox::new(&mut checked, "Box"));
            ui.add(MaterialSwitch::new(&mut on).text("Toggle"));
        },
    );
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Box").is_focused(),
        "Tab reaches the checkbox"
    );
    harness.key_press(egui::Key::Space);
    harness.run();
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Toggle").is_focused(),
        "Tab reaches the switch"
    );
    harness.key_press(egui::Key::Space);
    harness.run();
    drop(harness);
    assert!(checked, "Space checks the checkbox");
    assert!(on, "Space turns the switch on");
}

#[test]
fn radio_group_is_one_tab_stop_and_arrows_move_the_selection() {
    let mut choice = Some(0);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(300.0, 300.0),
        |ui| {
            ui.add(
                MaterialRadioGroup::new(&mut choice)
                    .id_salt("sizes")
                    .option(0, "Small")
                    .option(1, "Medium")
                    .option(2, "Large"),
            );
            ui.add(MaterialButton::filled("Next"));
        },
    );
    let toggled = |h: &Harness<'_>, label: &str| h.get_by_label(label).accesskit_node().toggled();

    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Small").is_focused(),
        "Tab reaches the selected option"
    );

    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    assert_eq!(
        toggled(&harness, "Medium"),
        Some(Toggled::True),
        "ArrowDown selects the next option"
    );
    assert_eq!(
        toggled(&harness, "Small"),
        Some(Toggled::False),
        "and deselects the old one"
    );
    assert!(
        harness.get_by_label("Medium").is_focused(),
        "focus follows the selection"
    );

    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    assert_eq!(
        toggled(&harness, "Large"),
        Some(Toggled::True),
        "ArrowRight also moves forward"
    );

    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    assert_eq!(
        toggled(&harness, "Small"),
        Some(Toggled::True),
        "ArrowDown wraps to the first"
    );

    harness.key_press(egui::Key::ArrowUp);
    harness.run();
    assert_eq!(
        toggled(&harness, "Large"),
        Some(Toggled::True),
        "ArrowUp wraps to the last"
    );

    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Next").is_focused(),
        "Tab leaves the group instead of visiting each option"
    );
    drop(harness);
    assert_eq!(choice, Some(2), "the selection is written back");
}

#[test]
fn radio_list_tile_uses_list_heights() {
    let mut choice = Some(0);
    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(400.0, 300.0),
        |ui| {
            ui.add(RadioListTile::new(&mut choice, 0).title("One line"));
            ui.add(
                RadioListTile::new(&mut choice, 1)
                    .title("Two lines")
                    .subtitle("Supporting text"),
            );
        },
    );
    assert_eq!(
        harness.get_by_label("One line").rect().height(),
        56.0,
        "one-line item"
    );
    assert_eq!(
        harness.get_by_label("Two lines").rect().height(),
        72.0,
        "two-line item"
    );
    assert_eq!(
        harness.get_by_label("One line").accesskit_node().role(),
        Role::RadioButton,
        "tile is a radio button"
    );
}

/// Every resting state of the three controls, enabled and disabled.
fn all_states(ui: &mut egui::Ui) {
    let background = MaterialTheme::of(ui.ctx()).color(ColorRole::Surface);
    ui.painter().rect_filled(ui.max_rect(), 0.0, background);
    for enabled in [true, false] {
        ui.horizontal(|ui| {
            let mut off = false;
            let mut on = true;
            let mut mixed = ToggleableState::Indeterminate;
            let mut error = true;
            ui.add(MaterialCheckbox::new(&mut off, "A").enabled(enabled));
            ui.add(MaterialCheckbox::new(&mut on, "B").enabled(enabled));
            ui.add(MaterialCheckbox::tri_state(&mut mixed, "C").enabled(enabled));
            ui.add(
                MaterialCheckbox::new(&mut error, "D")
                    .is_error(true)
                    .enabled(enabled),
            );
            let mut choice = Some(1);
            ui.add(MaterialRadio::new(&mut choice, 0, "E").enabled(enabled));
            ui.add(MaterialRadio::new(&mut choice, 1, "F").enabled(enabled));
            let mut switch_off = false;
            let mut switch_on = true;
            ui.add(
                MaterialSwitch::new(&mut switch_off)
                    .text("G")
                    .enabled(enabled),
            );
            ui.add(
                MaterialSwitch::new(&mut switch_on)
                    .text("H")
                    .enabled(enabled),
            );
        });
    }
}

#[test]
fn selection_controls_snapshot() -> TestResult {
    for (name, theme) in support::themes() {
        let mut harness = support::harness_for(theme, egui::vec2(640.0, 110.0), all_states);
        snapshot(&mut harness, &format!("selection_controls_{name}"))?;
    }
    Ok(())
}

#[test]
fn selection_controls_interaction_snapshots() -> TestResult {
    for (name, theme) in support::themes().into_iter().take(2) {
        let mut checked = true;
        let mut on = true;
        let mut choice = Some(0);
        let mut harness = support::harness_for(theme, egui::vec2(360.0, 60.0), |ui| {
            let background = MaterialTheme::of(ui.ctx()).color(ColorRole::Surface);
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(&mut checked, "Box"));
                ui.add(MaterialRadio::new(&mut choice, 0, "Dot"));
                ui.add(MaterialSwitch::new(&mut on).text("Toggle"));
            });
        });
        harness.get_by_label("Box").hover();
        harness.run();
        snapshot(&mut harness, &format!("selection_controls_hovered_{name}"))?;
        harness.remove_cursor();
        harness.key_press(egui::Key::Tab);
        harness.key_press(egui::Key::Tab);
        harness.run();
        snapshot(&mut harness, &format!("selection_controls_focused_{name}"))?;
    }
    Ok(())
}
