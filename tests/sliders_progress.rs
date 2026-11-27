// ============================================================================
// Module:       sliders_progress (integration test)
// Description:  Sliders, progress indicators and the loading indicator.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! WP-7: slider track and handle geometry per size, the range slider's
//! `changed()`, stepping, keyboard and AccessKit; progress indicator sizes,
//! round caps and AccessKit; the loading indicator's container, morph
//! sequence and snapshots at fixed times.

mod support;

use std::cell::{Cell, RefCell};

use egui::accesskit::Role;
use egui::epaint::{CircleShape, ClippedShape, RectShape, Shape as PaintShape};
use egui::{Color32, Event, Key, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::loading_indicator::INDETERMINATE_SEQUENCE;
use egui_material3::tokens::ColorRole;
use egui_material3::{
    LoadingIndicator, MaterialProgress, MaterialRangeSlider, MaterialSlider, MaterialTheme,
    RangeValues, SliderSize,
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

    fn theme(&self) -> std::sync::Arc<MaterialTheme> {
        MaterialTheme::of(&self.ctx)
    }

    fn frame(
        &mut self,
        dt: f64,
        events: Vec<Event>,
        app: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<ClippedShape> {
        self.time += dt;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(800.0, 400.0))),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let mut output = self.ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        output.shapes
    }
}

fn flatten(shapes: &[ClippedShape]) -> Vec<PaintShape> {
    fn walk(shape: &PaintShape, out: &mut Vec<PaintShape>) {
        match shape {
            PaintShape::Vec(inner) => inner.iter().for_each(|s| walk(s, out)),
            PaintShape::Noop
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Rect(_)
            | PaintShape::Text(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Band(_)
            | PaintShape::Callback(_) => out.push(shape.clone()),
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

fn rects(shapes: &[ClippedShape]) -> Vec<RectShape> {
    flatten(shapes)
        .into_iter()
        .filter_map(|s| match s {
            PaintShape::Rect(r) => Some(r),
            PaintShape::Noop
            | PaintShape::Vec(_)
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Text(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Band(_)
            | PaintShape::Callback(_) => None,
        })
        .collect()
}

fn circles(shapes: &[ClippedShape]) -> Vec<CircleShape> {
    flatten(shapes)
        .into_iter()
        .filter_map(|s| match s {
            PaintShape::Circle(c) => Some(c),
            PaintShape::Noop
            | PaintShape::Vec(_)
            | PaintShape::Rect(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Text(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Band(_)
            | PaintShape::Callback(_) => None,
        })
        .collect()
}

fn meshes(shapes: &[ClippedShape]) -> usize {
    flatten(shapes)
        .iter()
        .filter(|s| matches!(s, PaintShape::Mesh(_)))
        .count()
}

fn button(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

/// The handle: the `Primary` rect `width` wide.
fn handle_with_width(shapes: &[ClippedShape], color: Color32, width: f32) -> Option<RectShape> {
    rects(shapes)
        .into_iter()
        .find(|r| r.fill == color && (r.rect.width() - width).abs() < 0.01)
}

// ---------------------------------------------------------------------------
// Slider geometry

#[test]
fn each_slider_size_paints_its_track_and_handle() -> TestResult {
    for size in SliderSize::ALL {
        let metrics = size.metrics();
        let mut driver = Driver::new();
        let theme = driver.theme();
        let mut value = 50.0;
        let rect = Cell::new(Rect::NOTHING);
        let mut app = |ui: &mut egui::Ui| {
            let r = ui.add(
                MaterialSlider::new(&mut value, 0.0..=100.0)
                    .size(size)
                    .width(300.0)
                    .show_value(false),
            );
            rect.set(r.rect);
        };
        let shapes = driver.frame(0.0, vec![], &mut app);
        let inactive = theme.color(ColorRole::SecondaryContainer);
        let primary = theme.color(ColorRole::Primary);
        let track = rects(&shapes)
            .into_iter()
            .find(|r| r.fill == inactive)
            .ok_or("inactive track painted")?;
        assert_eq!(
            track.rect.height(),
            metrics.track_height,
            "{size:?}: track height"
        );
        let active = rects(&shapes)
            .into_iter()
            .find(|r| r.fill == primary && r.rect.height() == metrics.track_height)
            .ok_or("active track painted")?;
        // The handle sits 6 dp past the active track's end.
        let handle = handle_with_width(&shapes, primary, 4.0).ok_or("4 dp handle")?;
        assert_eq!(
            handle.rect.height(),
            metrics.handle_height,
            "{size:?}: handle height"
        );
        assert!(
            (handle.rect.min.x - active.rect.max.x - 6.0).abs() < 0.01,
            "{size:?}: 6 dp gap, active ends {} handle starts {}",
            active.rect.max.x,
            handle.rect.min.x
        );
        assert_eq!(
            rect.get().height(),
            metrics.handle_height,
            "{size:?}: widget height is the handle's"
        );
    }
    Ok(())
}

#[test]
fn handle_narrows_to_two_dp_when_pressed() -> TestResult {
    let mut driver = Driver::new();
    let primary = driver.theme().color(ColorRole::Primary);
    let mut value = 50.0;
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(
            ui.add(
                MaterialSlider::new(&mut value, 0.0..=100.0)
                    .width(204.0)
                    .show_value(false),
            )
            .rect,
        );
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    let handle = handle_with_width(&shapes, primary, 4.0).ok_or("idle handle is 4 dp")?;
    let at = handle.rect.center();
    driver.frame(
        0.1,
        vec![Event::PointerMoved(at), button(at, true)],
        &mut app,
    );
    let shapes = driver.frame(0.1, vec![], &mut app);
    assert!(
        handle_with_width(&shapes, primary, 2.0).is_some(),
        "pressed handle is 2 dp"
    );
    driver.frame(0.1, vec![button(at, false)], &mut app);
    let shapes = driver.frame(0.5, vec![Event::PointerGone], &mut app);
    // Clicking focuses the slider, and a focused handle is 2 dp too.
    let focused = driver.ctx.memory(|m| m.focused().is_some());
    let expected = if focused { 2.0 } else { 4.0 };
    assert!(
        handle_with_width(&shapes, primary, expected).is_some(),
        "released handle is {expected} dp (focused: {focused})"
    );
    Ok(())
}

#[test]
fn steps_count_from_the_range_start() -> TestResult {
    let mut driver = Driver::new();
    let mut value = 3.0_f32;
    let rect = Cell::new(Rect::NOTHING);
    let observed = Cell::new(0.0_f32);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(
            ui.add(
                MaterialSlider::new(&mut value, 3.0..=23.0)
                    .step(5.0)
                    .width(204.0)
                    .show_value(false),
            )
            .rect,
        );
        observed.set(value);
    };
    driver.frame(0.0, vec![], &mut app);
    // 40% along the track is 11, which snaps to 13 (3 + 2 * 5), not 10.
    let at = Pos2::new(rect.get().min.x + 2.0 + 0.4 * 200.0, rect.get().center().y);
    driver.frame(
        0.1,
        vec![Event::PointerMoved(at), button(at, true)],
        &mut app,
    );
    driver.frame(0.1, vec![button(at, false)], &mut app);
    assert_eq!(observed.get(), 13.0, "snapped to a step from the start");
    Ok(())
}

#[test]
fn vertical_slider_grows_upwards() -> TestResult {
    let mut driver = Driver::new();
    let mut value = 0.0_f32;
    let rect = Cell::new(Rect::NOTHING);
    let observed = Cell::new(0.0_f32);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(
            ui.add(
                MaterialSlider::new(&mut value, 0.0..=100.0)
                    .vertical(true)
                    .width(204.0),
            )
            .rect,
        );
        observed.set(value);
    };
    driver.frame(0.0, vec![], &mut app);
    assert_eq!(rect.get().height(), 204.0, "vertical length");
    assert_eq!(rect.get().width(), 44.0, "vertical thickness is the handle");
    let at = Pos2::new(rect.get().center().x, rect.get().max.y - 2.0 - 150.0);
    driver.frame(
        0.1,
        vec![Event::PointerMoved(at), button(at, true)],
        &mut app,
    );
    driver.frame(0.1, vec![button(at, false)], &mut app);
    assert!(
        (observed.get() - 75.0).abs() < 0.5,
        "75% up is 75, got {}",
        observed.get()
    );
    Ok(())
}

#[test]
fn value_indicator_uses_inverse_colours() -> TestResult {
    let mut driver = Driver::new();
    let theme = driver.theme();
    let mut value = 50.0;
    let mut app = |ui: &mut egui::Ui| {
        ui.add_space(80.0);
        ui.add(
            MaterialSlider::new(&mut value, 0.0..=100.0)
                .width(204.0)
                .show_value(false)
                .show_value_indicator(true),
        );
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    let primary = theme.color(ColorRole::Primary);
    let at = handle_with_width(&shapes, primary, 4.0)
        .ok_or("handle")?
        .rect
        .center();
    let to = at + egui::vec2(20.0, 0.0);
    driver.frame(
        0.1,
        vec![Event::PointerMoved(at), button(at, true)],
        &mut app,
    );
    driver.frame(0.1, vec![Event::PointerMoved(to)], &mut app);
    let shapes = driver.frame(0.1, vec![Event::PointerMoved(to)], &mut app);
    let bubble = rects(&shapes)
        .into_iter()
        .find(|r| r.fill == theme.color(ColorRole::InverseSurface))
        .ok_or("value indicator painted in InverseSurface while dragging")?;
    assert_eq!(bubble.rect.height(), 44.0, "value indicator height");
    let handle = handle_with_width(&shapes, primary, 2.0).ok_or("pressed handle")?;
    assert!(
        (handle.rect.min.y - bubble.rect.max.y - 12.0).abs() < 0.01,
        "12 dp above the handle"
    );
    driver.frame(0.1, vec![button(to, false)], &mut app);
    Ok(())
}

// ---------------------------------------------------------------------------
// Range slider

/// Regression test for nikescar/egui-material3#13: dragging a range slider
/// thumb sets `changed()`, and the thumb picked when the drag started keeps
/// moving even when dragged past the other one.
#[test]
fn range_drag_marks_changed_and_keeps_thumb() -> TestResult {
    let mut driver = Driver::new();
    let values = RefCell::new(RangeValues::new(40.0, 60.0));
    let out = Cell::new((Rect::NOTHING, false));
    let mut app = |ui: &mut egui::Ui| {
        let mut v = values.borrow_mut();
        let r = ui.add(
            MaterialRangeSlider::new(&mut v, 0.0..=100.0)
                .width(204.0)
                .show_values(false),
        );
        out.set((r.rect, r.changed()));
    };
    driver.frame(0.0, vec![], &mut app);
    let rect = out.get().0;
    // The track starts half a handle in and is 200 long.
    let at = |value: f32| Pos2::new(rect.min.x + 2.0 + value * 2.0, rect.center().y);
    driver.frame(
        0.1,
        vec![Event::PointerMoved(at(40.0)), button(at(40.0), true)],
        &mut app,
    );
    driver.frame(0.1, vec![Event::PointerMoved(at(50.0))], &mut app);
    driver.frame(0.1, vec![Event::PointerMoved(at(70.0))], &mut app);
    assert!(out.get().1, "dragging a thumb marks the response changed");
    let v = *values.borrow();
    assert!(v.start > 40.0, "the start thumb moves, got {v:?}");
    assert!(
        (v.end - 60.0).abs() < f32::EPSILON,
        "the end thumb stays put, got {v:?}"
    );
    driver.frame(0.1, vec![button(at(70.0), false)], &mut app);
    Ok(())
}

#[test]
fn range_drag_on_the_track_moves_the_nearer_thumb() -> TestResult {
    let mut driver = Driver::new();
    let values = RefCell::new(RangeValues::new(20.0, 80.0));
    let out = Cell::new((Rect::NOTHING, false));
    let mut app = |ui: &mut egui::Ui| {
        let mut v = values.borrow_mut();
        let r = ui.add(
            MaterialRangeSlider::new(&mut v, 0.0..=100.0)
                .width(204.0)
                .show_values(false),
        );
        out.set((r.rect, r.changed()));
    };
    driver.frame(0.0, vec![], &mut app);
    let rect = out.get().0;
    let at = |value: f32| Pos2::new(rect.min.x + 2.0 + value * 2.0, rect.center().y);
    driver.frame(
        0.1,
        vec![Event::PointerMoved(at(70.0)), button(at(70.0), true)],
        &mut app,
    );
    driver.frame(0.1, vec![Event::PointerMoved(at(65.0))], &mut app);
    assert!(out.get().1, "a track drag marks changed");
    let v = *values.borrow();
    assert!(
        (v.end - 65.0).abs() < 0.5 && (v.start - 20.0).abs() < f32::EPSILON,
        "the end thumb follows, got {v:?}"
    );
    driver.frame(0.1, vec![button(at(65.0), false)], &mut app);
    Ok(())
}

#[test]
fn range_thumbs_are_separate_accessible_sliders() {
    let mut values = RangeValues::new(20.0, 80.0);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(400.0, 100.0),
        |ui| {
            ui.add(
                MaterialRangeSlider::new(&mut values, 0.0..=100.0)
                    .text("Price")
                    .step(10.0),
            );
        },
    );
    let start = harness.get_by_label("Price start");
    assert_eq!(
        start.accesskit_node().role(),
        Role::Slider,
        "start thumb role"
    );
    assert_eq!(
        start.accesskit_node().numeric_value(),
        Some(20.0),
        "start thumb value"
    );
    harness.get_by_label("Price end").focus();
    harness.run();
    harness.key_press(Key::ArrowLeft);
    harness.run();
    assert_eq!(
        harness
            .get_by_label("Price end")
            .accesskit_node()
            .numeric_value(),
        Some(70.0),
        "Left moves the end thumb down a step"
    );
    drop(harness);
    assert_eq!(values.end, 70.0, "written back");
}

// ---------------------------------------------------------------------------
// Slider keyboard and AccessKit

#[test]
fn keyboard_moves_a_focused_slider() {
    let mut value = 50.0_f32;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(400.0, 100.0),
        |ui| {
            ui.add(
                MaterialSlider::new(&mut value, 0.0..=100.0)
                    .step(5.0)
                    .text("Volume"),
            );
        },
    );
    let node = harness.get_by_label("Volume");
    assert_eq!(node.accesskit_node().role(), Role::Slider, "slider role");
    assert_eq!(node.accesskit_node().numeric_value(), Some(50.0), "value");
    node.focus();
    harness.run();
    harness.key_press(Key::ArrowRight);
    harness.run();
    let value_now =
        |h: &egui_kittest::Harness<'_>| h.get_by_label("Volume").accesskit_node().numeric_value();
    assert_eq!(value_now(&harness), Some(55.0), "Right adds a step");
    harness.key_press(Key::PageDown);
    harness.run();
    assert_eq!(value_now(&harness), Some(5.0), "Page Down takes ten steps");
    harness.key_press(Key::End);
    harness.run();
    assert_eq!(value_now(&harness), Some(100.0), "End goes to the maximum");
    harness.key_press(Key::Home);
    harness.run();
    assert_eq!(value_now(&harness), Some(0.0), "Home goes to the minimum");
    drop(harness);
    assert_eq!(value, 0.0, "written back");
}

#[test]
fn disabled_slider_reports_disabled_and_ignores_input() {
    let mut value = 30.0_f32;
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(400.0, 100.0),
        |ui| {
            ui.add(
                MaterialSlider::new(&mut value, 0.0..=100.0)
                    .text("Locked")
                    .enabled(false),
            );
        },
    );
    let node = harness.get_by_label("Locked");
    assert!(node.accesskit_node().is_disabled(), "reports disabled");
    node.click();
    harness.run();
    harness.key_press(Key::End);
    harness.run();
    drop(harness);
    assert_eq!(value, 30.0, "a disabled slider keeps its value");
}

// ---------------------------------------------------------------------------
// Progress

#[test]
fn progress_default_sizes_match_the_tokens() {
    let sizes = RefCell::new(Vec::new());
    let mut driver = Driver::new();
    let mut app = |ui: &mut egui::Ui| {
        let mut s = sizes.borrow_mut();
        s.clear();
        s.push(ui.add(MaterialProgress::linear().value(0.5)).rect.size());
        s.push(
            ui.add(MaterialProgress::linear().wavy(true).value(0.5))
                .rect
                .size(),
        );
        s.push(ui.add(MaterialProgress::circular().value(0.5)).rect.size());
        s.push(
            ui.add(MaterialProgress::circular().wavy(true).value(0.5))
                .rect
                .size(),
        );
    };
    driver.frame(0.0, vec![], &mut app);
    assert_eq!(
        *sizes.borrow(),
        vec![
            egui::vec2(240.0, 4.0),
            egui::vec2(240.0, 10.0),
            egui::vec2(40.0, 40.0),
            egui::vec2(48.0, 48.0),
        ],
        "linear 240 x 4, wavy 240 x 10, circular 40, wavy circular 48"
    );
}

#[test]
fn circular_progress_has_round_caps_and_a_gap() -> TestResult {
    let mut driver = Driver::new();
    let theme = driver.theme();
    let mut app = |ui: &mut egui::Ui| {
        ui.add(MaterialProgress::circular().value(0.25));
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    let primary = theme.color(ColorRole::Primary);
    let track = theme.color(ColorRole::SecondaryContainer);
    let caps = circles(&shapes);
    let active_caps = caps
        .iter()
        .filter(|c| c.fill == primary && (c.radius - 2.0).abs() < 0.01)
        .count();
    let track_caps = caps
        .iter()
        .filter(|c| c.fill == track && (c.radius - 2.0).abs() < 0.01)
        .count();
    assert_eq!(active_caps, 2, "the indicator arc has two round caps");
    assert_eq!(track_caps, 2, "the track arc has two round caps");
    Ok(())
}

#[test]
fn indeterminate_circular_has_no_track() {
    let mut driver = Driver::new();
    let track = driver.theme().color(ColorRole::SecondaryContainer);
    let mut app = |ui: &mut egui::Ui| {
        ui.add(MaterialProgress::circular().indeterminate(true));
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    assert!(
        circles(&shapes).iter().all(|c| c.fill != track),
        "the indeterminate circular track is transparent"
    );
}

#[test]
fn linear_progress_stop_indicator_and_value() {
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(300.0, 60.0),
        |ui| {
            ui.add(MaterialProgress::linear().value(0.5).label("Upload"));
        },
    );
    let node = harness.get_by_label("Upload");
    assert_eq!(
        node.accesskit_node().role(),
        Role::ProgressIndicator,
        "progress role"
    );
    assert_eq!(node.accesskit_node().numeric_value(), Some(0.5), "value");
    harness.run();
}

#[test]
fn linear_determinate_paints_a_stop_dot_at_the_end() -> TestResult {
    let mut driver = Driver::new();
    let primary = driver.theme().color(ColorRole::Primary);
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(ui.add(MaterialProgress::linear().value(0.3)).rect);
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    let stop = circles(&shapes)
        .into_iter()
        .find(|c| c.fill == primary)
        .ok_or("stop dot")?;
    assert_eq!(stop.radius, 2.0, "4 dp stop");
    assert!(
        (stop.center.x - (rect.get().max.x - 2.0)).abs() < 0.01,
        "at the track end"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Loading indicator

#[test]
fn loading_indicator_container_and_accessibility() -> TestResult {
    let mut driver = Driver::new();
    let theme = driver.theme();
    let rects_out = RefCell::new(Vec::new());
    let mut app = |ui: &mut egui::Ui| {
        let mut r = rects_out.borrow_mut();
        r.clear();
        r.push(ui.add(LoadingIndicator::new()).rect);
        r.push(ui.add(LoadingIndicator::contained().progress(0.5)).rect);
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    for rect in rects_out.borrow().iter() {
        assert_eq!(rect.size(), egui::vec2(48.0, 48.0), "48 dp container");
    }
    let container = rects(&shapes)
        .into_iter()
        .find(|r| r.fill == theme.color(ColorRole::PrimaryContainer))
        .ok_or("contained indicator paints PrimaryContainer")?;
    assert_eq!(container.rect.size(), egui::vec2(48.0, 48.0), "container");
    assert_eq!(meshes(&shapes), 2, "each indicator fills one shape");

    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(100.0, 100.0),
        |ui| {
            ui.add(
                LoadingIndicator::contained()
                    .progress(0.25)
                    .label("Syncing"),
            );
        },
    );
    let node = harness.get_by_label("Syncing");
    assert_eq!(
        node.accesskit_node().role(),
        Role::ProgressIndicator,
        "role"
    );
    assert_eq!(node.accesskit_node().numeric_value(), Some(0.25), "value");
    Ok(())
}

#[test]
fn loading_indicator_morphs_in_compose_order() {
    // Compose's `IndeterminateIndicatorPolygons`.
    assert_eq!(
        INDETERMINATE_SEQUENCE.map(|s| s.name()),
        [
            "SoftBurst",
            "Cookie9Sided",
            "Pentagon",
            "Pill",
            "Sunny",
            "Cookie4Sided",
            "Oval"
        ],
        "shape order"
    );
    let order: Vec<usize> = (0..14)
        .map(|k| LoadingIndicator::frame_at(f64::from(k) * 0.65 + 0.3).morph_index)
        .collect();
    assert_eq!(
        order,
        vec![0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6],
        "a morph every 650 ms, wrapping around"
    );
}

// ---------------------------------------------------------------------------
// Snapshots

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

fn snapshot(harness: &mut egui_kittest::Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    // Software and hardware rasterisers differ in anti-aliasing; allow that,
    // not a missing or moved widget.
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

/// The light and dark themes of the default theme file.
fn light_and_dark() -> Vec<(&'static str, MaterialTheme)> {
    support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
        .collect()
}

/// A harness for animated widgets: they always request a repaint, so it is
/// stepped a fixed number of times (each `dt` seconds) instead of run until
/// idle, which makes the frame time deterministic.
fn animated_harness<'a>(
    theme: MaterialTheme,
    size: egui::Vec2,
    dt: f32,
    app: impl FnMut(&mut egui::Ui) + 'a,
) -> egui_kittest::Harness<'a> {
    let mut harness = egui_kittest::Harness::builder()
        .with_size(size)
        .with_step_dt(dt)
        .allow_missing_glyphs()
        .build_ui(app);
    MaterialTheme::set(&harness.ctx, theme);
    harness.step();
    harness
}

#[test]
fn sliders_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(560.0, 620.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            for size in SliderSize::ALL {
                let mut v = 35.0;
                ui.add(
                    MaterialSlider::new(&mut v, 0.0..=100.0)
                        .size(size)
                        .width(320.0)
                        .text(format!("{size:?}"))
                        .inset_icon("\u{e050}"),
                );
            }
            let mut stepped = 40.0;
            ui.add(
                MaterialSlider::new(&mut stepped, 0.0..=100.0)
                    .step(10.0)
                    .width(320.0)
                    .text("Stepped"),
            );
            let mut centered = 30.0;
            ui.add(
                MaterialSlider::new(&mut centered, -50.0..=50.0)
                    .centered(true)
                    .width(320.0)
                    .text("Centred"),
            );
            let mut range = RangeValues::new(25.0, 70.0);
            ui.add(
                MaterialRangeSlider::new(&mut range, 0.0..=100.0)
                    .width(320.0)
                    .text("Range"),
            );
            let mut disabled = 60.0;
            ui.add(
                MaterialSlider::new(&mut disabled, 0.0..=100.0)
                    .width(320.0)
                    .enabled(false)
                    .text("Disabled"),
            );
        });
        snapshot(&mut harness, &format!("sliders_{name}"))?;
    }
    Ok(())
}

#[test]
fn progress_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = animated_harness(theme, egui::vec2(300.0, 260.0), 0.25, move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.add(MaterialProgress::linear().value(0.4));
            ui.add(MaterialProgress::linear().wavy(true).value(0.6));
            ui.add(MaterialProgress::linear().indeterminate(true));
            ui.add(MaterialProgress::linear().large(true).value(0.7));
            ui.horizontal(|ui| {
                ui.add(MaterialProgress::circular().value(0.3));
                ui.add(MaterialProgress::circular().indeterminate(true));
                ui.add(MaterialProgress::circular().wavy(true).value(0.6));
            });
        });
        // Let the wavy amplitude finish easing in (500 ms).
        harness.run_steps(3);
        snapshot(&mut harness, &format!("progress_{name}"))?;
    }
    Ok(())
}

#[test]
fn loading_indicator_frames_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        // Steps of 650 ms: each frame is the start of the next morph.
        let mut harness = animated_harness(theme, egui::vec2(240.0, 70.0), 0.65, move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.horizontal(|ui| {
                ui.add(LoadingIndicator::new());
                ui.add(LoadingIndicator::contained());
                ui.add(LoadingIndicator::new().progress(0.3));
                ui.add(LoadingIndicator::contained().progress(0.9));
            });
        });
        harness.run_steps(2);
        snapshot(&mut harness, &format!("loading_indicator_{name}"))?;
    }
    Ok(())
}
