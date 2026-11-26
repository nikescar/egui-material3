//! Common buttons, toggle buttons, button groups and split buttons (WP-3):
//! token geometry per size, state layers, shape morph, AccessKit state,
//! keyboard, and snapshots of every style and size.

mod support;

use std::cell::{Cell, RefCell};

use egui::accesskit::{Role, Toggled};
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape, TextShape};
use egui::{Color32, Event, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::{
    ButtonGroup, ButtonShape, ButtonSize, MaterialButton, MaterialButtonVariant, MaterialTheme,
    SplitButton,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ---------------------------------------------------------------------------
// A raw-context driver: exact control of time and pointer, and access to the
// painted shapes.

struct Driver {
    ctx: egui::Context,
    time: f64,
}

impl Driver {
    fn new() -> Self {
        let ctx = egui::Context::default();
        MaterialTheme::set(&ctx, MaterialTheme::baseline(false));
        Self { ctx, time: 0.0 }
    }

    /// Runs one frame `dt` seconds after the last, with `events`.
    fn frame(
        &mut self,
        dt: f64,
        events: Vec<Event>,
        app: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<ClippedShape> {
        self.time += dt;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(1200.0, 400.0))),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let mut output = self.ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        output.shapes
    }
}

fn rects(shapes: &[ClippedShape]) -> Vec<RectShape> {
    shapes
        .iter()
        .filter_map(|c| match &c.shape {
            PaintShape::Rect(r) => Some(r.clone()),
            _ => None,
        })
        .collect()
}

fn texts(shapes: &[ClippedShape]) -> Vec<TextShape> {
    shapes
        .iter()
        .filter_map(|c| match &c.shape {
            PaintShape::Text(t) => Some(t.clone()),
            _ => None,
        })
        .collect()
}

/// The container rect painted exactly at `rect`.
fn container_at(shapes: &[ClippedShape], rect: Rect) -> Option<RectShape> {
    rects(shapes)
        .into_iter()
        .find(|r| (r.rect.min - rect.min).length() < 0.5 && (r.rect.max - rect.max).length() < 0.5)
}

fn press(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

// ---------------------------------------------------------------------------
// Geometry

#[test]
fn each_size_matches_its_tokens() -> TestResult {
    for size in ButtonSize::ALL {
        let metrics = size.metrics();
        let mut driver = Driver::new();
        let rect = Cell::new(Rect::NOTHING);
        let mut app = |ui: &mut egui::Ui| {
            rect.set(ui.add(MaterialButton::outlined("Label").size(size)).rect);
        };
        driver.frame(0.0, vec![], &mut app);
        let shapes = driver.frame(0.1, vec![], &mut app);
        assert_eq!(rect.get().height(), metrics.height, "{size:?}: height");

        let container = container_at(&shapes, rect.get()).ok_or("container painted")?;
        assert_eq!(
            container.stroke.width, metrics.outline_width,
            "{size:?}: outline width"
        );
        let label = texts(&shapes)
            .into_iter()
            .find(|t| t.galley.text() == "Label")
            .ok_or("label painted")?;
        let left = label.pos.x - rect.get().min.x;
        let right = rect.get().max.x - (label.pos.x + label.galley.size().x);
        assert!(
            (left - metrics.leading_space).abs() < 0.5,
            "{size:?}: leading space {left}"
        );
        assert!(
            (right - metrics.trailing_space).abs() < 0.5,
            "{size:?}: trailing space {right}"
        );
        let font = label
            .galley
            .job
            .sections
            .first()
            .map(|s| s.format.font_id.size)
            .ok_or("label has a section")?;
        assert_eq!(font, metrics.label.style().size, "{size:?}: label size");
    }
    Ok(())
}

#[test]
fn each_size_uses_its_icon_size() -> TestResult {
    for (size, expected) in ButtonSize::ALL
        .into_iter()
        .zip([20.0, 20.0, 24.0, 32.0, 40.0])
    {
        let mut driver = Driver::new();
        let mut app = |ui: &mut egui::Ui| {
            ui.add(MaterialButton::filled("Add").leading_icon("add").size(size));
        };
        let shapes = driver.frame(0.0, vec![], &mut app);
        let sizes: Vec<f32> = texts(&shapes)
            .iter()
            .filter(|t| t.galley.text() != "Add")
            .filter_map(|t| t.galley.job.sections.first().map(|s| s.format.font_id.size))
            .collect();
        assert_eq!(sizes, vec![expected], "{size:?}: icon size");
    }
    Ok(())
}

#[test]
fn baseline_button_keeps_the_classic_metrics() -> TestResult {
    let mut driver = Driver::new();
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(ui.add(MaterialButton::filled("Plain")).rect);
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    assert_eq!(rect.get().height(), 40.0, "baseline height");
    let label = texts(&shapes)
        .into_iter()
        .find(|t| t.galley.text() == "Plain")
        .ok_or("label painted")?;
    assert!(
        (label.pos.x - rect.get().min.x - 24.0).abs() < 0.5,
        "baseline padding 24"
    );
    let container = container_at(&shapes, rect.get()).ok_or("container painted")?;
    assert_eq!(container.corner_radius.nw, 20, "round: half the height");
    Ok(())
}

#[test]
fn square_shape_and_pressed_morph() -> TestResult {
    let mut driver = Driver::new();
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(
            ui.add(
                MaterialButton::filled("Square")
                    .size(ButtonSize::Medium)
                    .shape(ButtonShape::Square),
            )
            .rect,
        );
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    let idle = container_at(&shapes, rect.get()).ok_or("container")?;
    assert_eq!(idle.corner_radius.nw, 16, "Medium square is CornerLarge");

    let pos = rect.get().center();
    driver.frame(0.1, vec![Event::PointerMoved(pos)], &mut app);
    driver.frame(0.1, vec![press(pos, true)], &mut app);
    let shapes = driver.frame(2.0, vec![], &mut app);
    let pressed = container_at(&shapes, rect.get()).ok_or("container")?;
    assert_eq!(
        pressed.corner_radius.nw, 12,
        "Medium pressed is CornerMedium"
    );
    Ok(())
}

#[test]
fn outlined_and_text_buttons_show_hover() -> TestResult {
    for variant in [MaterialButtonVariant::Outlined, MaterialButtonVariant::Text] {
        let mut driver = Driver::new();
        let rect = Cell::new(Rect::NOTHING);
        let mut app = |ui: &mut egui::Ui| {
            rect.set(
                ui.add(MaterialButton::opt_image_and_text_with_variant(
                    variant,
                    None,
                    Some("Hover".into()),
                ))
                .rect,
            );
        };
        let shapes = driver.frame(0.0, vec![], &mut app);
        let idle = container_at(&shapes, rect.get()).ok_or("container")?.fill;
        driver.frame(
            0.1,
            vec![Event::PointerMoved(rect.get().center())],
            &mut app,
        );
        let shapes = driver.frame(0.1, vec![], &mut app);
        let hovered = container_at(&shapes, rect.get()).ok_or("container")?.fill;
        assert_eq!(idle, Color32::TRANSPARENT, "{variant:?}: idle is clear");
        assert!(
            hovered.a() > 0 && hovered.a() < 40,
            "{variant:?}: hover paints a translucent layer: {hovered:?}"
        );
    }
    Ok(())
}

#[test]
fn hovered_filled_button_rises_to_level_one() -> TestResult {
    let mut driver = Driver::new();
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        ui.add_space(20.0);
        rect.set(ui.add(MaterialButton::filled("Lift")).rect);
    };
    let shadows =
        |shapes: &[ClippedShape]| rects(shapes).iter().filter(|r| r.blur_width > 0.0).count();
    let idle = driver.frame(0.0, vec![], &mut app);
    driver.frame(
        0.1,
        vec![Event::PointerMoved(rect.get().center())],
        &mut app,
    );
    let hovered = driver.frame(0.1, vec![], &mut app);
    assert_eq!(shadows(&idle), 0, "level 0 at rest");
    assert!(shadows(&hovered) > 0, "level 1 on hover");
    Ok(())
}

// ---------------------------------------------------------------------------
// Groups and split buttons

#[test]
fn connected_group_inner_corners_tighten_when_pressed() -> TestResult {
    let mut driver = Driver::new();
    let rects_out = RefCell::new(Vec::new());
    let mut app = |ui: &mut egui::Ui| {
        let group = ButtonGroup::connected()
            .size(ButtonSize::Small)
            .button(MaterialButton::outlined("Day"))
            .button(MaterialButton::outlined("Week"))
            .button(MaterialButton::outlined("Month"))
            .show(ui);
        *rects_out.borrow_mut() = group.items.iter().map(|r| r.rect).collect();
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    let [first, middle, last] = rects_out.borrow()[..] else {
        return Err("three items".into());
    };
    assert!(
        (middle.min.x - first.max.x - 2.0).abs() < 0.01,
        "connected items are 2 dp apart"
    );
    let first_shape = container_at(&shapes, first).ok_or("first")?.corner_radius;
    let middle_shape = container_at(&shapes, middle).ok_or("middle")?.corner_radius;
    let last_shape = container_at(&shapes, last).ok_or("last")?.corner_radius;
    assert_eq!(
        (first_shape.nw, first_shape.ne),
        (20, 8),
        "first: outer, inner"
    );
    assert_eq!(
        (
            middle_shape.nw,
            middle_shape.ne,
            middle_shape.sw,
            middle_shape.se
        ),
        (8, 8, 8, 8),
        "middle: inner corners 8 dp"
    );
    assert_eq!(
        (last_shape.nw, last_shape.ne),
        (8, 20),
        "last: inner, outer"
    );

    let pos = middle.center();
    driver.frame(0.1, vec![Event::PointerMoved(pos)], &mut app);
    driver.frame(0.1, vec![press(pos, true)], &mut app);
    let shapes = driver.frame(2.0, vec![], &mut app);
    let pressed = container_at(&shapes, middle)
        .ok_or("middle pressed")?
        .corner_radius;
    assert_eq!(
        (pressed.nw, pressed.ne, pressed.sw, pressed.se),
        (4, 4, 4, 4),
        "pressed: inner corners 4 dp"
    );
    Ok(())
}

#[test]
fn standard_group_grows_the_pressed_button() -> TestResult {
    let mut driver = Driver::new();
    let widths = RefCell::new(Vec::new());
    let rects_out = RefCell::new(Vec::new());
    let mut app = |ui: &mut egui::Ui| {
        let group = ButtonGroup::standard()
            .button(MaterialButton::filled("One"))
            .button(MaterialButton::filled("Two"))
            .button(MaterialButton::filled("Three"))
            .show(ui);
        *rects_out.borrow_mut() = group.items.iter().map(|r| r.rect).collect();
        *widths.borrow_mut() = group
            .items
            .iter()
            .map(|r| r.rect.width())
            .collect::<Vec<f32>>();
    };
    driver.frame(0.0, vec![], &mut app);
    let idle = widths.borrow().clone();
    let middle = rects_out.borrow().get(1).copied().ok_or("middle")?;
    let first = rects_out.borrow().first().copied().ok_or("first")?;
    assert!(
        (middle.min.x - first.max.x - 12.0).abs() < 0.01,
        "standard items are 12 dp apart"
    );
    driver.frame(0.1, vec![Event::PointerMoved(middle.center())], &mut app);
    driver.frame(0.1, vec![press(middle.center(), true)], &mut app);
    // The growth spring starts on the frame after the press is seen.
    driver.frame(0.1, vec![], &mut app);
    driver.frame(2.0, vec![], &mut app);
    let widths = widths.borrow().clone();
    let grown = widths.get(1).copied().unwrap_or(0.0) / idle.get(1).copied().unwrap_or(1.0);
    assert!(
        (grown - 1.15).abs() < 0.01,
        "pressed item grows 15%: {grown}"
    );
    let total = |w: &[f32]| w.iter().sum::<f32>();
    assert!(
        (total(&widths) - total(&idle)).abs() < 0.5,
        "row width unchanged: {widths:?} vs {idle:?}"
    );
    Ok(())
}

#[test]
fn open_split_button_trailing_is_fully_round() -> TestResult {
    let mut driver = Driver::new();
    let open = Cell::new(false);
    let trailing = Cell::new(Rect::NOTHING);
    let leading = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        let mut is_open = open.get();
        let split = SplitButton::filled("Send", &mut is_open).show(ui);
        open.set(is_open);
        trailing.set(split.trailing.rect);
        leading.set(split.leading.rect);
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    assert_eq!(trailing.get().height(), 40.0, "Small split button height");
    assert_eq!(trailing.get().width(), 48.0, "13 + 22 + 13");
    assert!(
        (trailing.get().min.x - leading.get().max.x - 2.0).abs() < 0.01,
        "halves 2 dp apart"
    );
    let closed = container_at(&shapes, trailing.get())
        .ok_or("trailing.get()")?
        .corner_radius;
    assert_eq!((closed.nw, closed.ne), (4, 20), "inner 4 dp, outer full");

    let pos = trailing.get().center();
    driver.frame(0.1, vec![Event::PointerMoved(pos)], &mut app);
    driver.frame(0.1, vec![press(pos, true)], &mut app);
    driver.frame(0.1, vec![press(pos, false)], &mut app);
    driver.frame(0.1, vec![Event::PointerGone], &mut app);
    let shapes = driver.frame(2.0, vec![], &mut app);
    let opened = container_at(&shapes, trailing.get())
        .ok_or("trailing.get() open")?
        .corner_radius;
    assert!(
        open.get(),
        "clicking the trailing.get() half opens the menu"
    );
    assert_eq!(
        (opened.nw, opened.ne, opened.sw, opened.se),
        (20, 20, 20, 20),
        "open: fully round"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// AccessKit and keyboard

#[test]
fn toggle_button_reports_and_flips_its_state() {
    let mut on = false;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(300.0, 100.0),
        |ui| {
            ui.add(MaterialButton::filled_tonal("Bold").toggle(&mut on));
        },
    );
    let toggled = |h: &egui_kittest::Harness<'_>| h.get_by_label("Bold").accesskit_node().toggled();
    assert_eq!(
        harness.get_by_label("Bold").accesskit_node().role(),
        Role::Button,
        "toggle button role"
    );
    assert_eq!(toggled(&harness), Some(Toggled::False), "starts off");
    harness.get_by_label("Bold").click();
    harness.run();
    assert_eq!(toggled(&harness), Some(Toggled::True), "click turns it on");
    drop(harness);
    assert!(on, "the bound flag flipped");
}

#[test]
fn disabled_toggle_does_not_flip() {
    let mut on = false;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(300.0, 100.0),
        |ui| {
            ui.add(
                MaterialButton::filled("Locked")
                    .toggle(&mut on)
                    .enabled(false),
            );
        },
    );
    assert!(
        harness
            .get_by_label("Locked")
            .accesskit_node()
            .is_disabled(),
        "reports disabled"
    );
    harness.get_by_label("Locked").click();
    harness.run();
    drop(harness);
    assert!(!on, "a disabled toggle ignores clicks");
}

#[test]
fn single_select_group_items_are_radio_buttons() {
    let mut view = Some(0);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(400.0, 100.0),
        |ui| {
            ButtonGroup::connected()
                .single_select(&mut view)
                .button(MaterialButton::outlined("Day"))
                .button(MaterialButton::outlined("Week"))
                .show(ui);
        },
    );
    let node = |h: &egui_kittest::Harness<'_>, label: &str| {
        let n = h.get_by_label(label);
        (n.accesskit_node().role(), n.accesskit_node().toggled())
    };
    assert_eq!(
        node(&harness, "Day"),
        (Role::RadioButton, Some(Toggled::True)),
        "Day selected"
    );
    harness.get_by_label("Week").click();
    harness.run();
    assert_eq!(
        node(&harness, "Week"),
        (Role::RadioButton, Some(Toggled::True)),
        "Week selected after click"
    );
    assert_eq!(
        node(&harness, "Day").1,
        Some(Toggled::False),
        "Day deselected"
    );
    drop(harness);
    assert_eq!(view, Some(1), "selection written back");
}

#[test]
fn arrow_keys_move_focus_within_a_group() {
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(500.0, 100.0),
        |ui| {
            ButtonGroup::standard()
                .button(MaterialButton::filled("Alpha"))
                .button(MaterialButton::filled("Beta"))
                .button(MaterialButton::filled("Gamma"))
                .show(ui);
        },
    );
    let focused = |h: &egui_kittest::Harness<'_>, label: &str| h.get_by_label(label).is_focused();
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(focused(&harness, "Alpha"), "Tab enters the group");
    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    assert!(focused(&harness, "Beta"), "Right moves to the next button");
    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    assert!(
        focused(&harness, "Alpha"),
        "Right wraps from the last button"
    );
    harness.key_press(egui::Key::ArrowLeft);
    harness.run();
    assert!(
        focused(&harness, "Gamma"),
        "Left wraps from the first button"
    );
}

#[test]
fn disabled_button_does_not_click() {
    let mut clicks = 0;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(300.0, 100.0),
        |ui| {
            if ui
                .add(MaterialButton::filled("Off").enabled(false))
                .clicked()
            {
                clicks += 1;
            }
        },
    );
    harness.get_by_label("Off").click();
    harness.run();
    drop(harness);
    assert_eq!(clicks, 0, "a disabled button reports no click");
}

// ---------------------------------------------------------------------------
// Snapshots

fn snapshot(harness: &mut egui_kittest::Harness<'_>, name: &str) -> TestResult {
    // Software and hardware rasterisers differ in anti-aliasing; allow that,
    // not a missing or moved button.
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(600);
    match harness.try_snapshot_options(name, &options) {
        Ok(()) => Ok(()),
        // No GPU adapter (headless CI without a software renderer): the
        // geometry is still covered by the tests above.
        Err(egui_kittest::SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

/// The light and dark themes of the default theme file.
fn light_and_dark() -> Vec<(&'static str, MaterialTheme)> {
    support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
        .collect()
}

#[test]
fn styles_by_size_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(egui_material3::tokens::ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(1240.0, 470.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            for size in ButtonSize::ALL {
                ui.horizontal(|ui| {
                    for variant in MaterialButtonVariant::ALL {
                        ui.add(
                            MaterialButton::opt_image_and_text_with_variant(
                                variant,
                                None,
                                Some("Button".into()),
                            )
                            .size(size),
                        );
                    }
                });
            }
        });
        snapshot(&mut harness, &format!("buttons_styles_by_size_{name}"))?;
    }
    Ok(())
}

#[test]
fn states_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(egui_material3::tokens::ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(560.0, 260.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            for variant in MaterialButtonVariant::ALL {
                ui.horizontal(|ui| {
                    let b = |label: &str| {
                        MaterialButton::opt_image_and_text_with_variant(
                            variant,
                            None,
                            Some(label.into()),
                        )
                        .size(ButtonSize::ExtraSmall)
                    };
                    ui.add(b("Enabled"));
                    ui.add(b("Disabled").enabled(false));
                    ui.add(b("Off").selected(false));
                    ui.add(b("On").selected(true));
                    ui.add(b("Square on").selected(true).shape(ButtonShape::Square));
                    ui.add(b("On disabled").selected(true).enabled(false));
                });
            }
        });
        snapshot(&mut harness, &format!("buttons_states_{name}"))?;
    }
    Ok(())
}

#[test]
fn groups_and_split_buttons_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(egui_material3::tokens::ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(520.0, 260.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            let mut view = Some(1);
            ButtonGroup::connected()
                .id_salt("view")
                .single_select(&mut view)
                .button(MaterialButton::outlined("Day"))
                .button(MaterialButton::outlined("Week"))
                .button(MaterialButton::outlined("Month"))
                .show(ui);
            let mut flags = [true, false, true];
            ButtonGroup::standard()
                .id_salt("format")
                .multi_select(&mut flags)
                .button(MaterialButton::filled_tonal("Bold"))
                .button(MaterialButton::filled_tonal("Italic"))
                .button(MaterialButton::filled_tonal("Underline"))
                .show(ui);
            let mut closed = false;
            let mut open = true;
            ui.horizontal(|ui| {
                SplitButton::filled("Send", &mut closed).show(ui);
                SplitButton::outlined("Save", &mut open).show(ui);
                SplitButton::filled_tonal("Share", &mut closed)
                    .size(ButtonSize::Medium)
                    .show(ui);
            });
        });
        snapshot(&mut harness, &format!("buttons_groups_split_{name}"))?;
    }
    Ok(())
}
