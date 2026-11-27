// ============================================================================
// Module:       chips (integration test)
// Description:  Assist, filter, input and suggestion chips, and chip sets.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Chips (WP-5): token geometry and paddings, colours per variant and
//! state, elevation, the Expressive shapes, AccessKit roles and toggled
//! state, Space to toggle, the separate remove target, chip sets, and
//! snapshots of every variant in light and dark.

mod support;

use std::cell::Cell;

use egui::accesskit::{Role, Toggled};
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape, TextShape};
use egui::{Color32, Event, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::tokens::ColorRole;
use egui_material3::{
    assist_chip, chip_set, filter_chip, input_chip, suggestion_chip, MaterialChip, MaterialTheme,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ---------------------------------------------------------------------------
// A raw-context driver: exact control of the pointer, and the painted shapes.

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

    fn frame(
        &mut self,
        events: Vec<Event>,
        app: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<ClippedShape> {
        self.time += 1.0 / 60.0;
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

fn baseline() -> MaterialTheme {
    MaterialTheme::baseline(false)
}

fn rects(shapes: &[ClippedShape]) -> Vec<RectShape> {
    shapes
        .iter()
        .filter_map(|c| match &c.shape {
            PaintShape::Rect(r) => Some(r.clone()),
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
        .collect()
}

fn texts(shapes: &[ClippedShape]) -> Vec<TextShape> {
    shapes
        .iter()
        .filter_map(|c| match &c.shape {
            PaintShape::Text(t) => Some(t.clone()),
            PaintShape::Noop
            | PaintShape::Vec(_)
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Band(_)
            | PaintShape::Rect(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => None,
        })
        .collect()
}

/// The unblurred rects painted exactly at `rect` (container fill, state
/// layer, outline), in paint order.
fn at(shapes: &[ClippedShape], rect: Rect) -> Vec<RectShape> {
    rects(shapes)
        .into_iter()
        .filter(|r| r.rect == rect && r.blur_width == 0.0)
        .collect()
}

/// The widest shadow blur painted, 0 when there is no shadow.
fn max_blur(shapes: &[ClippedShape]) -> f32 {
    rects(shapes)
        .iter()
        .map(|r| r.blur_width)
        .fold(0.0, f32::max)
}

/// Draws one chip with the driver and returns its rect and the shapes.
fn draw(
    driver: &mut Driver,
    events: Vec<Event>,
    make: &mut dyn FnMut() -> MaterialChip<'static>,
) -> (Rect, Vec<ClippedShape>) {
    let mut rect = Rect::NOTHING;
    let shapes = driver.frame(events, &mut |ui| rect = ui.add(make()).rect);
    (rect, shapes)
}

fn label_of(shapes: &[ClippedShape], text: &str) -> Result<TextShape, String> {
    texts(shapes)
        .into_iter()
        .find(|t| t.galley.text() == text)
        .ok_or_else(|| format!("no text shape for {text:?}"))
}

// ---------------------------------------------------------------------------
// Geometry

#[test]
fn every_variant_is_32_high() -> TestResult {
    let mut on = false;
    let mut harness = support::harness_for(baseline(), egui::vec2(600.0, 80.0), |ui| {
        ui.horizontal(|ui| {
            ui.add(assist_chip("Assist"));
            ui.add(filter_chip("Filter", &mut on));
            ui.add(input_chip("Input"));
            ui.add(suggestion_chip("Suggestion"));
        });
    });
    harness.run();
    for label in ["Assist", "Filter", "Input", "Suggestion"] {
        let height = harness.get_by_label(label).rect().height();
        assert_eq!(height, 32.0, "{label}: ContainerHeight is 32");
    }
    Ok(())
}

/// `Chip.kt`: content padding 8 plus the 8 spacing around the empty icon
/// slots puts the label 16 from both edges; input chips use 4 + 8 = 12.
#[test]
fn label_padding_without_icons() -> TestResult {
    let mut driver = Driver::new();
    for (make, inset) in [
        (
            Box::new(|| assist_chip("Assist")) as Box<dyn Fn() -> MaterialChip<'static>>,
            16.0,
        ),
        (Box::new(|| suggestion_chip("Assist")), 16.0),
        (Box::new(|| input_chip("Assist")), 12.0),
    ] {
        let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || make());
        let label = label_of(&shapes, "Assist")?;
        assert_eq!(label.pos.x - rect.min.x, inset, "start inset {inset}");
        let end = label.pos.x + label.galley.size().x;
        assert!(
            (rect.max.x - end - inset).abs() < 0.01,
            "end inset {inset}, got {}",
            rect.max.x - end
        );
    }
    Ok(())
}

/// Icon 18 at 8 from the edge, 8 to the label; input-chip avatar 24 at 4.
#[test]
fn icon_and_avatar_padding() -> TestResult {
    let mut driver = Driver::new();
    let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || {
        assist_chip("Event").leading_icon("event")
    });
    let label = label_of(&shapes, "Event")?;
    assert_eq!(
        label.pos.x - rect.min.x,
        8.0 + 18.0 + 8.0,
        "icon 8 in, 18 wide, 8 to the label"
    );

    let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || {
        input_chip("Ada")
            .leading_icon_svg(egui_material3::image_utils::AVATAR_SVG)
            .avatar(true)
            .removable(true)
    });
    let label = label_of(&shapes, "Ada")?;
    assert_eq!(
        label.pos.x - rect.min.x,
        4.0 + 24.0 + 8.0,
        "avatar 4 in, 24 wide, 8 to the label"
    );
    let end = label.pos.x + label.galley.size().x;
    assert!(
        (rect.max.x - end - (8.0 + 18.0 + 8.0)).abs() < 0.01,
        "trailing icon 8 after the label, 18 wide, 8 from the end"
    );
    Ok(())
}

#[test]
fn label_is_label_large() -> TestResult {
    let mut driver = Driver::new();
    let (_, shapes) = draw(&mut driver, Vec::new(), &mut || assist_chip("Assist"));
    let label = label_of(&shapes, "Assist")?;
    let sizes: Vec<f32> = label
        .galley
        .job
        .sections
        .iter()
        .map(|s| s.format.font_id.size)
        .collect();
    assert_eq!(sizes, vec![14.0], "LabelLarge is 14");
    assert_eq!(label.galley.size().y, 20.0, "LabelLarge line height is 20");
    Ok(())
}

// ---------------------------------------------------------------------------
// Colours

#[test]
fn flat_chip_colours() -> TestResult {
    let theme = baseline();
    let mut driver = Driver::new();
    for (make, label_role) in [
        (
            Box::new(|| assist_chip("Chip")) as Box<dyn Fn() -> MaterialChip<'static>>,
            ColorRole::OnSurface,
        ),
        (
            Box::new(|| suggestion_chip("Chip")),
            ColorRole::OnSurfaceVariant,
        ),
        (Box::new(|| input_chip("Chip")), ColorRole::OnSurfaceVariant),
    ] {
        let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || make());
        let painted = at(&shapes, rect);
        assert!(
            painted.iter().all(|r| r.fill == Color32::TRANSPARENT),
            "flat chips have no container"
        );
        assert!(
            painted
                .iter()
                .any(|r| r.stroke.color == theme.color(ColorRole::OutlineVariant)
                    && r.stroke.width == 1.0),
            "1-wide outlineVariant outline"
        );
        let label = label_of(&shapes, "Chip")?;
        assert_eq!(
            label.fallback_color,
            theme.color(label_role),
            "label {label_role:?}"
        );
    }
    Ok(())
}

#[test]
fn selected_filter_and_input_chips() -> TestResult {
    let theme = baseline();
    let mut driver = Driver::new();
    for variant in ["filter", "input"] {
        let mut rect = Rect::NOTHING;
        let mut on = true;
        let shapes = driver.frame(Vec::new(), &mut |ui| {
            let chip = match variant {
                "filter" => filter_chip("Chip", &mut on),
                _ => input_chip("Chip").selected(&mut on),
            };
            rect = ui.add(chip).rect;
        });
        let painted = at(&shapes, rect);
        assert!(
            painted
                .iter()
                .any(|r| r.fill == theme.color(ColorRole::SecondaryContainer)),
            "{variant}: selected container is secondaryContainer"
        );
        assert!(
            painted.iter().all(|r| r.stroke.width == 0.0),
            "{variant}: selected chips have no outline"
        );
        let label = label_of(&shapes, "Chip")?;
        assert_eq!(
            label.fallback_color,
            theme.color(ColorRole::OnSecondaryContainer),
            "{variant}: label onSecondaryContainer"
        );
    }
    Ok(())
}

#[test]
fn disabled_colours() -> TestResult {
    let theme = baseline();
    let on_surface = theme.color(ColorRole::OnSurface);
    let mut driver = Driver::new();

    let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || {
        assist_chip("Chip").enabled(false)
    });
    let painted = at(&shapes, rect);
    assert!(
        painted.iter().all(|r| r.fill == Color32::TRANSPARENT),
        "a disabled flat chip stays transparent"
    );
    assert!(
        painted
            .iter()
            .any(|r| r.stroke.color == on_surface.gamma_multiply(0.12)),
        "a disabled flat chip has an onSurface 12 % outline"
    );
    assert_eq!(
        label_of(&shapes, "Chip")?.fallback_color,
        on_surface.gamma_multiply(0.38),
        "disabled label is onSurface at 38 %"
    );

    let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || {
        assist_chip("Chip").elevated(true).enabled(false)
    });
    assert!(
        at(&shapes, rect)
            .iter()
            .any(|r| r.fill == on_surface.gamma_multiply(0.12)),
        "a disabled elevated chip fills with onSurface at 12 %"
    );
    assert_eq!(
        max_blur(&shapes),
        0.0,
        "a disabled elevated chip is Level 0"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Elevation and state layers

#[test]
fn elevated_chip_rises_on_hover() -> TestResult {
    let theme = baseline();
    let mut driver = Driver::new();
    let mut make = || assist_chip("Chip").elevated(true);
    let (rect, shapes) = draw(&mut driver, Vec::new(), &mut make);
    assert!(
        at(&shapes, rect)
            .iter()
            .any(|r| r.fill == theme.color(ColorRole::SurfaceContainerLow)),
        "elevated container is surfaceContainerLow"
    );
    assert_eq!(max_blur(&shapes), 2.0, "Level 1 at rest");

    let (_, shapes) = draw(
        &mut driver,
        vec![Event::PointerMoved(rect.center())],
        &mut make,
    );
    assert_eq!(max_blur(&shapes), 6.0, "Level 2 on hover");
    Ok(())
}

#[test]
fn hover_paints_a_state_layer() -> TestResult {
    let theme = baseline();
    let mut driver = Driver::new();
    let mut make = || assist_chip("Chip");
    let (rect, idle) = draw(&mut driver, Vec::new(), &mut make);
    let (_, hovered) = draw(
        &mut driver,
        vec![Event::PointerMoved(rect.center())],
        &mut make,
    );
    let layer = theme.color(ColorRole::OnSurface).gamma_multiply(0.08);
    assert!(
        !at(&idle, rect).iter().any(|r| r.fill == layer),
        "no layer when idle"
    );
    assert!(
        at(&hovered, rect).iter().any(|r| r.fill == layer),
        "hover paints the label colour at 8 %"
    );
    Ok(())
}

#[test]
fn expressive_shapes() -> TestResult {
    let mut driver = Driver::new();
    let radius = |shapes: &[ClippedShape], rect: Rect| {
        at(shapes, rect)
            .first()
            .map(|r| r.corner_radius.nw)
            .unwrap_or_default()
    };

    let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || assist_chip("Chip"));
    assert_eq!(radius(&shapes, rect), 8, "baseline CornerSmall");

    let (rect, shapes) = draw(&mut driver, Vec::new(), &mut || {
        suggestion_chip("Chip").expressive(true)
    });
    assert_eq!(
        radius(&shapes, rect),
        12,
        "Expressive unselected CornerMedium"
    );

    // The same widget id, now selected: the corner morphs from 12 to full
    // with the fast spatial spring rather than jumping.
    let mut on = true;
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(ui.add(filter_chip("Chip", &mut on).expressive(true)).rect);
    };
    let shapes = driver.frame(Vec::new(), &mut app);
    assert_eq!(radius(&shapes, rect.get()), 12, "the morph starts from 12");
    let mut settled = 0;
    for _ in 0..60 {
        let shapes = driver.frame(Vec::new(), &mut app);
        settled = radius(&shapes, rect.get());
    }
    assert_eq!(settled, 16, "Expressive selected settles at CornerFull");

    // A fresh chip starts at its target.
    let mut fresh = Driver::new();
    let shapes = fresh.frame(Vec::new(), &mut app);
    assert_eq!(
        radius(&shapes, rect.get()),
        16,
        "Expressive selected CornerFull"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Accessibility and keyboard

#[test]
fn roles_and_toggled_state() {
    let mut filter = false;
    let mut input = true;
    let harness = support::harness_for(baseline(), egui::vec2(600.0, 80.0), |ui| {
        ui.horizontal(|ui| {
            ui.add(assist_chip("Assist"));
            ui.add(filter_chip("Filter", &mut filter));
            ui.add(input_chip("Tag").selected(&mut input));
            ui.add(input_chip("Plain"));
            ui.add(suggestion_chip("Suggestion"));
        });
    });
    let node = |label: &str| {
        let n = harness.get_by_label(label);
        (n.accesskit_node().role(), n.accesskit_node().toggled())
    };
    assert_eq!(node("Assist"), (Role::Button, None), "assist is a button");
    assert_eq!(
        node("Filter"),
        (Role::CheckBox, Some(Toggled::False)),
        "filter chip is an unchecked checkbox"
    );
    assert_eq!(
        node("Tag"),
        (Role::CheckBox, Some(Toggled::True)),
        "selectable input chip is a checked checkbox"
    );
    assert_eq!(node("Plain"), (Role::Button, None), "plain input chip");
    assert_eq!(
        node("Suggestion"),
        (Role::Button, None),
        "suggestion is a button"
    );
}

#[test]
fn space_toggles_a_filter_chip() {
    let mut on = false;
    let mut harness = support::harness_for(baseline(), egui::vec2(300.0, 80.0), |ui| {
        ui.add(filter_chip("Photos", &mut on));
    });
    let toggled =
        |h: &egui_kittest::Harness<'_>| h.get_by_label("Photos").accesskit_node().toggled();
    harness.get_by_label("Photos").focus();
    harness.run();
    harness.key_press(egui::Key::Space);
    harness.run();
    assert_eq!(toggled(&harness), Some(Toggled::True), "Space selects");
    harness.key_press(egui::Key::Enter);
    harness.run();
    assert_eq!(toggled(&harness), Some(Toggled::False), "Enter deselects");
    harness.key_press(egui::Key::Space);
    harness.run();
    drop(harness);
    assert!(on, "the bound value follows");
}

#[test]
fn disabled_chip_does_not_toggle() {
    let mut on = false;
    let mut harness = support::harness_for(baseline(), egui::vec2(300.0, 80.0), |ui| {
        ui.add(filter_chip("Locked", &mut on).enabled(false));
    });
    assert!(
        harness
            .get_by_label("Locked")
            .accesskit_node()
            .is_disabled(),
        "reported disabled"
    );
    harness.get_by_label("Locked").click();
    harness.run();
    drop(harness);
    assert!(!on, "a disabled chip ignores clicks");
}

#[test]
fn remove_target_is_separate() {
    let removed = Cell::new(0);
    let clicked = Cell::new(0);
    let mut on = false;
    let mut harness = support::harness_for(baseline(), egui::vec2(300.0, 80.0), |ui| {
        let response = MaterialChip::input("Ada")
            .selected(&mut on)
            .removable(true)
            .on_click(|| clicked.set(clicked.get() + 1))
            .show(ui);
        if response.removed() {
            removed.set(removed.get() + 1);
        }
    });
    let remove = harness.get_by_label("Remove Ada");
    assert_eq!(
        remove.accesskit_node().role(),
        Role::Button,
        "the remove target is its own button"
    );
    remove.click();
    harness.run();
    assert_eq!(removed.get(), 1, "clicking the X removes");
    assert_eq!(clicked.get(), 0, "clicking the X does not click the chip");

    harness.get_by_label("Ada").focus();
    harness.run();
    harness.key_press(egui::Key::Delete);
    harness.run();
    assert_eq!(removed.get(), 2, "Delete on the focused chip removes");
    drop(harness);
    assert!(!on, "removing does not toggle the selection");
}

// ---------------------------------------------------------------------------
// Chip sets

#[test]
fn chip_set_wraps_with_8_spacing() -> TestResult {
    let mut driver = Driver::new();
    let mut rects = Vec::new();
    driver.frame(Vec::new(), &mut |ui| {
        rects.clear();
        ui.set_max_width(200.0);
        chip_set(ui, |ui| {
            for label in ["One", "Two", "Three", "Four", "Five", "Six"] {
                rects.push(ui.add(assist_chip(label)).rect);
            }
        });
    });
    let [first, second, ..] = rects.as_slice() else {
        return Err("chips were not drawn".into());
    };
    assert!(
        (second.min.x - first.max.x - 8.0).abs() < 0.01,
        "8 between chips, got {}",
        second.min.x - first.max.x
    );
    let next_row = rects
        .iter()
        .find(|r| r.min.y > first.min.y)
        .ok_or("the set did not wrap")?;
    assert!(
        (next_row.min.y - first.max.y - 8.0).abs() < 0.01,
        "8 between rows, got {}",
        next_row.min.y - first.max.y
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshots

/// Rendering tests take this lock: several wgpu renderers created at once
/// in one process can crash the test binary on Windows.
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
    // not a missing or moved chip.
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(600);
    match no_adapter_is_an_error(|| harness.try_snapshot_options(name, &options)) {
        Ok(()) => Ok(()),
        // No GPU adapter: the geometry is still covered by the tests above.
        Err(egui_kittest::SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

#[test]
fn variants_and_states_snapshot() -> TestResult {
    for (name, theme) in support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
    {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(560.0, 200.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            let (mut off, mut on) = (false, true);
            chip_set(ui, |ui| {
                ui.add(assist_chip("Assist"));
                ui.add(assist_chip("Elevated").elevated(true));
                ui.add(filter_chip("Filter", &mut off));
                ui.add(filter_chip("Selected", &mut on));
                ui.add(input_chip("Input").removable(true));
                ui.add(input_chip("Input on").selected(&mut on).removable(true));
                ui.add(suggestion_chip("Suggestion"));
                ui.add(assist_chip("Disabled").enabled(false));
                ui.add(filter_chip("Disabled on", &mut on).enabled(false));
                ui.add(assist_chip("Disabled raised").elevated(true).enabled(false));
                ui.add(suggestion_chip("Expressive").expressive(true));
                ui.add(filter_chip("Expressive on", &mut on).expressive(true));
            });
        });
        snapshot(&mut harness, &format!("chips_variants_{name}"))?;
    }
    Ok(())
}
