//! Exposed dropdown menu: keyboard selection, typeahead, click-outside
//! close, filtering, the combo-box AccessKit node, and an open-menu
//! snapshot in light and dark.

mod support;

use std::cell::Cell;

use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use egui_material3::tokens::ColorRole;
use egui_material3::{MaterialSelect, MaterialTheme, SelectVariant};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const FRUITS: [&str; 4] = ["Apple", "Banana", "Cherry", "Grape"];

fn fruits(selected: &mut Option<usize>) -> MaterialSelect<'_> {
    FRUITS
        .iter()
        .enumerate()
        .fold(MaterialSelect::new(selected).label("Fruit"), |s, (i, f)| {
            s.option(i, *f)
        })
        .width(240.0)
}

fn menu_shows(harness: &Harness<'_>, option: &str) -> bool {
    harness.query_by_label(option).is_some()
}

#[test]
fn arrow_and_enter_select_an_option() {
    let selected = Cell::new(None);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 400.0),
        |ui| {
            let mut value = selected.get();
            ui.add(fruits(&mut value));
            selected.set(value);
        },
    );
    harness.get_by_label("Fruit").focus();
    harness.run();
    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    assert!(menu_shows(&harness, "Cherry"), "ArrowDown opens the menu");
    assert_eq!(
        harness
            .get_by_label("Fruit")
            .accesskit_node()
            .data()
            .is_expanded(),
        Some(true),
        "the combo box reports expanded"
    );
    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    harness.key_press(egui::Key::Enter);
    harness.run();
    assert_eq!(
        selected.get(),
        Some(1),
        "Down, Down, Enter picks the second option"
    );
    assert!(!menu_shows(&harness, "Cherry"), "selecting closes the menu");
    assert_eq!(
        harness
            .get_by_label("Fruit")
            .accesskit_node()
            .value()
            .as_deref(),
        Some("Banana"),
        "the combo box reports its value"
    );
}

#[test]
fn escape_closes_and_typeahead_jumps() {
    let selected = Cell::new(None);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 400.0),
        |ui| {
            let mut value = selected.get();
            ui.add(fruits(&mut value));
            selected.set(value);
        },
    );
    harness.get_by_label("Fruit").focus();
    harness.run();
    harness.key_press(egui::Key::Space);
    harness.run();
    assert!(menu_shows(&harness, "Apple"), "Space opens the menu");
    harness.key_press(egui::Key::Escape);
    harness.run();
    assert!(!menu_shows(&harness, "Apple"), "Escape closes it");
    harness.get_by_label("Fruit").type_text("ch");
    harness.run();
    harness.key_press(egui::Key::Enter);
    harness.run();
    assert_eq!(
        selected.get(),
        Some(2),
        "typing 'ch' then Enter picks Cherry"
    );
}

#[test]
fn click_opens_and_click_outside_closes() {
    let selected = Cell::new(Some(3));
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 400.0),
        |ui| {
            let mut value = selected.get();
            ui.add(fruits(&mut value).variant(SelectVariant::Outlined));
            selected.set(value);
        },
    );
    harness.get_by_label("Fruit").click();
    harness.run();
    assert!(menu_shows(&harness, "Banana"), "click opens the menu");
    assert_eq!(
        harness.get_by_label("Apple").accesskit_node().role(),
        Role::ListBoxOption,
        "options are list-box options"
    );
    let outside = egui::pos2(300.0, 390.0);
    harness.hover_at(outside);
    harness.event(egui::Event::PointerButton {
        pos: outside,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    });
    harness.event(egui::Event::PointerButton {
        pos: outside,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    });
    harness.run();
    assert!(
        !menu_shows(&harness, "Banana"),
        "a click outside closes the menu"
    );
    assert_eq!(
        selected.get(),
        Some(3),
        "closing does not change the selection"
    );

    harness.get_by_label("Fruit").click();
    harness.run();
    harness.get_by_label("Banana").click();
    harness.run();
    assert_eq!(selected.get(), Some(1), "clicking an option selects it");
}

#[test]
fn filter_shows_matching_options_and_enter_completes() {
    let selected = Cell::new(None);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 400.0),
        |ui| {
            let mut value = selected.get();
            ui.add(fruits(&mut value).enable_filter(true));
            selected.set(value);
        },
    );
    let field = harness.get_by_label("Fruit");
    assert_eq!(
        field.accesskit_node().role(),
        Role::ComboBox,
        "editable combo box"
    );
    field.focus();
    harness.run();
    harness.get_by_label("Fruit").type_text("ap");
    harness.run();
    assert!(menu_shows(&harness, "Grape"), "Grape contains 'ap'");
    assert!(menu_shows(&harness, "Apple"), "Apple contains 'ap'");
    assert!(!menu_shows(&harness, "Banana"), "Banana is filtered out");
    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    harness.key_press(egui::Key::Enter);
    harness.run();
    assert_eq!(
        selected.get(),
        Some(3),
        "Down then Enter picks the second match"
    );
    assert_eq!(
        harness
            .get_by_label("Fruit")
            .accesskit_node()
            .value()
            .as_deref(),
        Some("Grape"),
        "the field shows the picked option"
    );
}

#[test]
fn select_geometry_and_menu_tokens() -> TestResult {
    let theme = MaterialTheme::baseline(false);
    let container = theme.color(ColorRole::SurfaceContainer);
    let field = Cell::new(egui::Rect::NOTHING);
    let mut selected = None;
    let mut harness = support::harness_for(theme, egui::vec2(320.0, 400.0), |ui| {
        field.set(ui.add(fruits(&mut selected)).rect);
    });
    assert_eq!(field.get().height(), 56.0, "field is 56 dp tall");
    harness.get_by_label("Fruit").click();
    harness.run();
    let item = harness.get_by_label("Apple").rect();
    assert_eq!(item.height(), 48.0, "menu items are 48 dp");
    let menu = harness.output().shapes.iter().find_map(|c| match &c.shape {
        egui::epaint::Shape::Rect(r) if r.fill == container => Some(r.clone()),
        _ => None,
    });
    let Some(menu) = menu else {
        return Err("menu container in surfaceContainer".into());
    };
    assert_eq!(menu.corner_radius.nw, 4, "menu corners are extra-small");
    assert_eq!(
        menu.rect.top(),
        field.get().bottom(),
        "menu opens below the field"
    );
    Ok(())
}

#[test]
fn open_select_snapshot() -> TestResult {
    for (name, theme) in support::themes() {
        let background = theme.color(ColorRole::Surface);
        let mut filled = Some(1);
        let mut outlined = None;
        let mut harness = support::harness_for(theme, egui::vec2(560.0, 340.0), |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.horizontal(|ui| {
                ui.add(fruits(&mut filled).id_salt("filled"));
                ui.add(
                    MaterialSelect::new(&mut outlined)
                        .label("Outlined")
                        .variant(SelectVariant::Outlined)
                        .option(0, "One")
                        .helper_text("Helper text")
                        .width(240.0),
                );
            });
        });
        harness.get_by_label("Fruit").focus();
        harness.run();
        harness.key_press(egui::Key::ArrowDown);
        harness.run();
        harness.key_press(egui::Key::ArrowDown);
        harness.run();
        let options = egui_kittest::SnapshotOptions::new()
            .threshold(2.0)
            .max_failed_pixels(400);
        match harness.try_snapshot_options(format!("select_open_{name}"), &options) {
            Ok(()) => {}
            Err(egui_kittest::SnapshotError::RenderError { err }) => {
                eprintln!("skipping snapshot, no renderer: {err}");
            }
            Err(err) => return Err(err.to_string().into()),
        }
    }
    Ok(())
}
