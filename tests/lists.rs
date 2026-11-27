// ============================================================================
// Module:       lists (integration test)
// Description:  Lists: heights, type roles, colours, leading elements, keyboard.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Lists (WP-11): one/two/three-line heights 56/72/88 with BodyLarge
//! labels, `surface` container and `secondaryContainer` selection, leading
//! avatars, images, checkboxes and radios, segmented items 2 dp apart, the
//! AccessKit tree, keyboard roving with Up/Down, and snapshots.

mod support;

use std::cell::Cell;

use egui::accesskit::{Role, Toggled};
use egui::epaint::{ClippedShape, Shape as PaintShape};
use egui::{Color32, Key, Pos2, Rect, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::{ColorRole, ListItem, MaterialList, MaterialTheme};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn size() -> Vec2 {
    Vec2::new(500.0, 600.0)
}

/// Runs one frame of `app` on a raw context and returns its shapes.
fn shapes_of(app: impl FnMut(&mut egui::Ui)) -> Vec<ClippedShape> {
    let ctx = egui::Context::default();
    MaterialTheme::set(&ctx, MaterialTheme::baseline(false));
    let mut app = app;
    let input = || egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size())),
        ..Default::default()
    };
    // Twice: the first frame sizes fonts.
    let mut first = ctx.run_ui(input(), |ui| app(ui));
    first.textures_delta.clear();
    let mut output = ctx.run_ui(input(), |ui| app(ui));
    output.textures_delta.clear();
    output.shapes
}

fn flatten(shape: &PaintShape, out: &mut Vec<PaintShape>) {
    if let PaintShape::Vec(shapes) = shape {
        for s in shapes {
            flatten(s, out);
        }
    } else {
        out.push(shape.clone());
    }
}

fn all_shapes(clipped: &[ClippedShape]) -> Vec<PaintShape> {
    let mut out = Vec::new();
    for c in clipped {
        flatten(&c.shape, &mut out);
    }
    out
}

#[test]
fn heights_are_56_72_and_88() -> TestResult {
    let harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        ui.add(
            MaterialList::new()
                .dividers(false)
                .fill_width(true)
                .item(ListItem::new("One line").leading_icon("inbox"))
                .item(ListItem::new("Two lines").secondary_text("Supporting text"))
                .item(
                    ListItem::new("Three lines")
                        .overline("Overline")
                        .secondary_text("Supporting text"),
                ),
        );
    });
    let height = |label: &str| harness.get_by_label(label).rect().height();
    assert_eq!(
        height("One line"),
        56.0,
        "ListTokens.ItemOneLineContainerHeight"
    );
    assert_eq!(
        height("Two lines"),
        72.0,
        "ListTokens.ItemTwoLineContainerHeight"
    );
    assert_eq!(
        height("Three lines"),
        88.0,
        "ListTokens.ItemThreeLineContainerHeight"
    );
    Ok(())
}

#[test]
fn label_is_body_large_and_supporting_text_body_medium() -> TestResult {
    let shapes = shapes_of(|ui| {
        ui.add(
            MaterialList::new().item(
                ListItem::new("Headline")
                    .secondary_text("Supporting")
                    .overline("Over"),
            ),
        );
    });
    let mut sizes = std::collections::HashMap::new();
    for shape in all_shapes(&shapes) {
        if let PaintShape::Text(text) = shape {
            let galley = &text.galley;
            if let Some(section) = galley.job.sections.first() {
                sizes.insert(galley.job.text.clone(), section.format.font_id.size);
            }
        }
    }
    assert_eq!(
        sizes.get("Headline"),
        Some(&16.0),
        "BodyLarge 16: {sizes:?}"
    );
    assert_eq!(
        sizes.get("Supporting"),
        Some(&14.0),
        "BodyMedium 14: {sizes:?}"
    );
    assert_eq!(sizes.get("Over"), Some(&11.0), "LabelSmall 11: {sizes:?}");
    Ok(())
}

#[test]
fn container_is_surface_and_selection_is_secondary_container() -> TestResult {
    let theme = MaterialTheme::baseline(false);
    let shapes = shapes_of(|ui| {
        ui.add(
            MaterialList::new()
                .item(ListItem::new("Plain"))
                .item(ListItem::new("Picked").selected(true)),
        );
    });
    let fills: Vec<Color32> = all_shapes(&shapes)
        .into_iter()
        .filter_map(|s| match s {
            PaintShape::Rect(r) => Some(r.fill),
            PaintShape::Noop
            | PaintShape::Vec(_)
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Band(_)
            | PaintShape::Text(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => None,
        })
        .collect();
    assert!(
        fills.contains(&theme.color(ColorRole::Surface)),
        "the list is painted in surface"
    );
    assert!(
        fills.contains(&theme.color(ColorRole::SecondaryContainer)),
        "the selected item is painted in secondaryContainer"
    );
    assert!(
        !fills.contains(&theme.color(ColorRole::PrimaryContainer)),
        "primaryContainer is no longer used for selection"
    );
    Ok(())
}

#[test]
fn leading_elements_size_the_item_and_report_roles() -> TestResult {
    for (name, theme) in support::themes() {
        let harness = support::harness_for(theme, size(), |ui| {
            ui.add(
                MaterialList::new()
                    .dividers(false)
                    .fill_width(true)
                    .item(ListItem::new("Avatar").leading_avatar("AB"))
                    .item(ListItem::new("Check").leading_checkbox(true))
                    .item(ListItem::new("Radio").leading_radio(false))
                    .item(
                        ListItem::new("Image")
                            .leading_image(egui::include_image!("../resources/320x240.png")),
                    ),
            );
        });
        let node = |label: &'static str| harness.get_by_label(label);
        assert_eq!(
            node("Avatar").rect().height(),
            56.0,
            "{name}: avatar fits 56"
        );
        assert_eq!(
            node("Image").rect().height(),
            72.0,
            "{name}: a 56 dp image plus 8 dp above and below"
        );
        assert_eq!(
            node("Check").accesskit_node().role(),
            Role::CheckBox,
            "{name}"
        );
        assert_eq!(
            node("Check").accesskit_node().toggled(),
            Some(Toggled::True),
            "{name}: checked"
        );
        assert_eq!(
            node("Radio").accesskit_node().role(),
            Role::RadioButton,
            "{name}"
        );
        assert_eq!(
            node("Avatar").accesskit_node().role(),
            Role::ListItem,
            "{name}"
        );
    }
    Ok(())
}

#[test]
fn segmented_items_are_two_dp_apart() -> TestResult {
    let harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        ui.add(
            MaterialList::new()
                .segmented(true)
                .fill_width(true)
                .item(ListItem::new("First"))
                .item(ListItem::new("Second")),
        );
    });
    let first = harness.get_by_label("First").rect();
    let second = harness.get_by_label("Second").rect();
    assert_eq!(second.min.y - first.max.y, 2.0, "ListTokens.SegmentedGap");
    Ok(())
}

#[test]
fn up_and_down_rove_and_space_activates() -> TestResult {
    let clicked = Cell::new(0);
    let mut harness = support::harness_for(MaterialTheme::baseline(false), size(), |ui| {
        ui.add(
            MaterialList::new()
                .item(ListItem::new("Alpha"))
                .item(ListItem::new("Beta").enabled(false))
                .item(ListItem::new("Gamma").on_click(|| clicked.set(clicked.get() + 1))),
        );
    });
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Alpha").is_focused(),
        "Tab reaches the first item"
    );
    harness.key_press(Key::ArrowDown);
    harness.run();
    assert!(
        harness.get_by_label("Gamma").is_focused(),
        "Down skips the disabled item"
    );
    harness.key_press(Key::Space);
    harness.run();
    assert_eq!(clicked.get(), 1, "Space activates the focused item");
    harness.key_press(Key::ArrowUp);
    harness.run();
    assert!(harness.get_by_label("Alpha").is_focused(), "Up moves back");
    assert!(
        harness.get_by_label("Beta").accesskit_node().is_disabled(),
        "disabled items are reported"
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
fn list_snapshots() -> TestResult {
    for (name, theme) in support::themes() {
        for segmented in [false, true] {
            let mut harness = support::harness_for(theme.clone(), Vec2::new(380.0, 480.0), |ui| {
                ui.add(
                    MaterialList::new()
                        .segmented(segmented)
                        .fill_width(true)
                        .item(
                            ListItem::new("Inbox")
                                .leading_icon("inbox")
                                .trailing_text("24"),
                        )
                        .item(
                            ListItem::new("Ada Lovelace")
                                .leading_avatar("AL")
                                .secondary_text("Notes on the analytical engine"),
                        )
                        .item(
                            ListItem::new("Selected")
                                .selected(true)
                                .leading_icon("star"),
                        )
                        .item(ListItem::new("Checked").leading_checkbox(true))
                        .item(ListItem::new("Radio").leading_radio(true))
                        .item(ListItem::new("Disabled").enabled(false))
                        .item(
                            ListItem::new("Three lines")
                                .overline("Overline")
                                .secondary_text("Supporting text\nthat runs to two lines"),
                        ),
                );
            });
            harness.key_press(Key::Tab);
            harness.run();
            let seg = if segmented { "_segmented" } else { "" };
            snapshot(&mut harness, &format!("list{seg}_{name}"))?;
        }
    }
    Ok(())
}
