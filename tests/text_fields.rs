//! Text fields: geometry, colours and widths from the M3 tokens, the label
//! floating on focus, character-safe `max_length`, the clickable trailing
//! icon, AccessKit, and snapshots of every state in light and dark.

mod support;

use std::cell::{Cell, RefCell};

use egui::accesskit::Role;
use egui::epaint::Shape as PaintShape;
use egui::Rect;
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use egui_material3::input::TextFieldResponse;
use egui_material3::tokens::ColorRole;
use egui_material3::{MaterialInput, MaterialTheme};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Every text shape painted last frame whose text is `text`: (top y, font size).
fn painted_text(harness: &Harness<'_>, text: &str) -> Vec<(f32, f32)> {
    harness
        .output()
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            PaintShape::Text(shape) if shape.galley.text() == text => {
                let size = shape
                    .galley
                    .job
                    .sections
                    .first()
                    .map_or(0.0, |s| s.format.font_id.size);
                Some((shape.pos.y, size))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn typing_past_max_length_in_multibyte_text_does_not_panic() {
    let text = RefCell::new(String::new());
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 120.0),
        |ui| {
            ui.add(
                MaterialInput::outlined(&mut text.borrow_mut())
                    .label("Code")
                    .max_length(2),
            );
        },
    );
    harness.get_by_label("Code").focus();
    harness.run();
    harness.get_by_label("Code").type_text("€€€");
    harness.run();
    assert_eq!(
        text.borrow().as_str(),
        "€€",
        "typing stops at two characters"
    );
}

#[test]
fn label_floats_on_focus() -> TestResult {
    let text = RefCell::new(String::new());
    for variant in ["filled", "outlined"] {
        let mut harness = support::harness_for(
            MaterialTheme::baseline(false),
            egui::vec2(320.0, 120.0),
            |ui| {
                let mut text = text.borrow_mut();
                let field = if variant == "filled" {
                    MaterialInput::filled(&mut text)
                } else {
                    MaterialInput::outlined(&mut text)
                };
                ui.add(field.label("Name"));
            },
        );
        let resting = painted_text(&harness, "Name");
        harness.get_by_label("Name").focus();
        harness.run();
        let floated = painted_text(&harness, "Name");
        let (Some(&(rest_y, rest_size)), Some(&(float_y, float_size))) =
            (resting.first(), floated.first())
        else {
            return Err(format!("{variant}: label not painted: {resting:?} {floated:?}").into());
        };
        assert_eq!(rest_size, 16.0, "{variant}: resting label is BodyLarge");
        assert_eq!(float_size, 12.0, "{variant}: floated label is BodySmall");
        assert!(
            float_y < rest_y,
            "{variant}: label moves up ({rest_y} -> {float_y})"
        );
    }
    Ok(())
}

#[test]
fn field_geometry_follows_the_tokens() {
    let text = RefCell::new(String::from("hello"));
    let field = Cell::new(Rect::NOTHING);
    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 120.0),
        |ui| {
            let response = ui.add(
                MaterialInput::filled(&mut text.borrow_mut())
                    .label("Search")
                    .leading_icon("search")
                    .trailing_icon("mic"),
            );
            field.set(response.rect);
        },
    );
    let field = field.get();
    assert_eq!(field.height(), 56.0, "container is 56 dp tall");
    let text_rect = harness.get_by_label("Search").rect();
    assert_eq!(
        text_rect.left() - field.left(),
        52.0,
        "text starts 52 dp in, after the leading icon"
    );
    assert_eq!(
        field.right() - text_rect.right(),
        52.0,
        "text ends 52 dp before the trailing icon"
    );
}

#[test]
fn indicator_and_outline_widths_change_on_focus() {
    let theme = MaterialTheme::baseline(false);
    let primary = theme.color(ColorRole::Primary);
    let text = RefCell::new(String::new());

    let mut filled = support::harness_for(theme.clone(), egui::vec2(320.0, 120.0), |ui| {
        ui.add(MaterialInput::filled(&mut text.borrow_mut()).label("First"));
    });
    filled.get_by_label("First").focus();
    filled.run();
    let indicator = filled.output().shapes.iter().find_map(|c| match &c.shape {
        PaintShape::Vec(shapes) => shapes.iter().find_map(|s| match s {
            PaintShape::Rect(r) if r.fill == primary => Some(r.rect.height()),
            _ => None,
        }),
        _ => None,
    });
    assert_eq!(
        indicator,
        Some(2.0),
        "focused filled indicator is 2 dp of primary"
    );

    let mut outlined = support::harness_for(theme, egui::vec2(320.0, 120.0), |ui| {
        ui.add(MaterialInput::outlined(&mut text.borrow_mut()).label("Second"));
    });
    let stroke = |h: &Harness<'_>| {
        h.output().shapes.iter().find_map(|c| match &c.shape {
            PaintShape::Path(p) => Some((p.stroke.width, p.closed)),
            _ => None,
        })
    };
    assert_eq!(
        stroke(&outlined),
        Some((1.0, true)),
        "idle outline is 1 dp, closed"
    );
    outlined.get_by_label("Second").focus();
    outlined.run();
    assert_eq!(
        stroke(&outlined),
        Some((3.0, false)),
        "focused outline is 3 dp and notched for the label"
    );
}

/// Pixel test: the focused outline is three device pixels of `primary`.
#[test]
fn outlined_focus_stroke_is_three_pixels() -> TestResult {
    let theme = MaterialTheme::baseline(false);
    let primary = theme.color(ColorRole::Primary);
    let background = theme.color(ColorRole::Surface);
    let text = RefCell::new(String::new());
    let field = Cell::new(Rect::NOTHING);
    let mut harness = support::harness_for(theme, egui::vec2(320.0, 100.0), |ui| {
        ui.painter().rect_filled(ui.max_rect(), 0.0, background);
        let response = ui.add(
            MaterialInput::outlined(&mut text.borrow_mut())
                .label("Email")
                .width(240.0),
        );
        field.set(response.rect);
    });
    harness.get_by_label("Email").focus();
    harness.run();
    let image = match harness.render() {
        Ok(image) => image,
        Err(err) => {
            eprintln!("skipping pixel test, no renderer: {err}");
            return Ok(());
        }
    };
    let field = field.get();
    // A column on the right half, clear of the label notch and the corners.
    let x = (field.right() - 40.0).round() as u32;
    let close = |p: [u8; 4]| {
        let d = |a: u8, b: u8| (i32::from(a) - i32::from(b)).abs();
        d(p[0], primary.r()) < 40 && d(p[1], primary.g()) < 40 && d(p[2], primary.b()) < 40
    };
    let top = field.top().round() as u32;
    let count = (top.saturating_sub(2)..top + 8)
        .filter(|&y| close(image.get_pixel(x, y).0))
        .count();
    assert_eq!(
        count, 3,
        "focused outline is 3 px of primary at the top edge"
    );
    Ok(())
}

#[test]
fn trailing_icon_button_reports_clicks() {
    let text = RefCell::new(String::from("query"));
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 120.0),
        |ui| {
            let mut text = text.borrow_mut();
            let TextFieldResponse {
                trailing_icon_clicked,
                ..
            } = MaterialInput::outlined(&mut text)
                .label("Search")
                .trailing_icon_button("close", "Clear")
                .show(ui);
            if trailing_icon_clicked {
                text.clear();
            }
        },
    );
    let clear = harness.get_by_label("Clear");
    assert_eq!(
        clear.accesskit_node().role(),
        Role::Button,
        "trailing icon is a button"
    );
    clear.click();
    harness.run();
    assert!(
        text.borrow().is_empty(),
        "clicking the trailing icon cleared the text"
    );
}

#[test]
fn fields_report_role_label_and_state() {
    for (name, theme) in support::themes() {
        let mut one = String::from("Ada");
        let mut two = String::new();
        let mut notes = String::from("line one\nline two");
        let harness = support::harness_for(theme, egui::vec2(400.0, 400.0), |ui| {
            ui.add(MaterialInput::filled(&mut one).label("First name"));
            ui.add(
                MaterialInput::outlined(&mut two)
                    .label("Last name")
                    .enabled(false),
            );
            ui.add(
                MaterialInput::outlined(&mut notes)
                    .label("Notes")
                    .multiline(true)
                    .min_lines(3),
            );
        });
        let first = harness.get_by_label("First name");
        assert_eq!(
            first.accesskit_node().role(),
            Role::TextInput,
            "{name}: text input"
        );
        assert_eq!(
            first.accesskit_node().value().as_deref(),
            Some("Ada"),
            "{name}: value"
        );
        assert!(
            harness
                .get_by_label("Last name")
                .accesskit_node()
                .is_disabled(),
            "{name}: disabled field reports disabled"
        );
        assert_eq!(
            harness.get_by_label("Notes").accesskit_node().role(),
            Role::MultilineTextInput,
            "{name}: multiline"
        );
    }
}

#[test]
fn tab_reaches_the_field_once() {
    let mut a = String::new();
    let mut b = String::new();
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(320.0, 200.0),
        |ui| {
            ui.add(
                MaterialInput::filled(&mut a)
                    .label("A")
                    .leading_icon("person"),
            );
            ui.add(MaterialInput::filled(&mut b).label("B"));
        },
    );
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("A").is_focused(),
        "first Tab focuses the first field"
    );
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("B").is_focused(),
        "second Tab moves straight to the next field"
    );
}

/// A filled or outlined field.
fn field(text: &mut String, filled: bool) -> MaterialInput<'_> {
    if filled {
        MaterialInput::filled(text)
    } else {
        MaterialInput::outlined(text)
    }
}

fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(400);
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
fn text_field_states_snapshot() -> TestResult {
    for (name, theme) in support::themes() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(640.0, 520.0), |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.columns(2, |columns| {
                for (column, filled) in columns.iter_mut().zip([true, false]) {
                    column.add(field(&mut String::new(), filled).label("Empty"));
                    column.add(field(&mut "With text".to_owned(), filled).label("With text"));
                    column.add(
                        field(&mut "Disabled".to_owned(), filled)
                            .label("Disabled")
                            .enabled(false),
                    );
                    column.add(
                        field(&mut "Wrong".to_owned(), filled)
                            .label("Error")
                            .is_error(true)
                            .error_text("Error message"),
                    );
                    column.add(
                        field(&mut "42".to_owned(), filled)
                            .label("Amount")
                            .prefix("$")
                            .suffix("USD")
                            .helper_text("Helper text")
                            .show_character_count(true)
                            .max_length(8),
                    );
                    column.add(
                        field(&mut String::new(), filled)
                            .label("Icons")
                            .leading_icon("search")
                            .trailing_icon("close"),
                    );
                }
            });
        });
        harness.run();
        snapshot(&mut harness, &format!("text_fields_{name}"))?;
    }
    Ok(())
}

#[test]
fn text_field_focused_and_hovered_snapshot() -> TestResult {
    for (name, theme) in support::themes() {
        let background = theme.color(ColorRole::Surface);
        let mut filled = String::from("Focused");
        let mut outlined = String::new();
        let mut hovered = String::new();
        let field = Cell::new(Rect::NOTHING);
        let mut harness = support::harness_for(theme, egui::vec2(320.0, 260.0), |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.add(MaterialInput::filled(&mut filled).label("Filled"));
            ui.add(
                MaterialInput::outlined(&mut outlined)
                    .label("Outlined")
                    .placeholder("Placeholder"),
            );
            field.set(
                ui.add(MaterialInput::outlined(&mut hovered).label("Hovered"))
                    .rect,
            );
        });
        harness.hover_at(field.get().center());
        harness.get_by_label("Outlined").focus();
        harness.run();
        snapshot(&mut harness, &format!("text_field_focused_{name}"))?;
    }
    Ok(())
}

#[test]
fn caret_uses_primary() {
    let theme = MaterialTheme::baseline(false);
    let primary = theme.color(ColorRole::Primary);
    let mut text = String::from("abc");
    let mut harness = support::harness_for(theme, egui::vec2(320.0, 120.0), |ui| {
        ui.add(MaterialInput::outlined(&mut text).label("Caret"));
    });
    harness.get_by_label("Caret").focus();
    harness.run();
    let caret = harness.output().shapes.iter().any(|c| match &c.shape {
        PaintShape::Rect(r) => r.fill == primary && r.rect.width() <= 2.0,
        PaintShape::LineSegment { stroke, .. } => stroke.color == primary,
        _ => false,
    });
    assert!(caret, "the caret is painted in primary");
}
