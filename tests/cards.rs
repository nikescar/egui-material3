// ============================================================================
// Module:       cards (integration test)
// Description:  Cards: token colours, elevation per state, selection,
//               keyboard, AccessKit and snapshots.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Elevated, filled and outlined cards (WP-14): container colours and
//! elevation from the Compose card tokens, hover elevation only for
//! interactive cards, the selected (checked) state, keyboard activation,
//! the focus ring, AccessKit roles and state, and snapshots.

mod support;

use std::cell::Cell;

use egui::accesskit::{Role, Toggled};
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape};
use egui::{Key, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use egui_material3::tokens::{ColorRole, Elevation, InteractionState};
use egui_material3::{Card2Variant, MaterialCard2, MaterialTheme};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Every rect shape painted in the last frame, nested `Shape::Vec`s
/// flattened.
fn rects(harness: &Harness<'_>) -> Vec<RectShape> {
    fn walk(shape: &PaintShape, out: &mut Vec<RectShape>) {
        match shape {
            PaintShape::Rect(r) => out.push(r.clone()),
            PaintShape::Vec(shapes) => shapes.iter().for_each(|s| walk(s, out)),
            PaintShape::Noop
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Band(_)
            | PaintShape::Text(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => {}
        }
    }
    let mut out = Vec::new();
    for ClippedShape { shape, .. } in &harness.output().shapes {
        walk(shape, &mut out);
    }
    out
}

/// The widest shadow blur painted: twice the elevation in dp.
fn shadow_blur(harness: &Harness<'_>) -> f32 {
    rects(harness)
        .iter()
        .map(|r| r.blur_width)
        .fold(0.0, f32::max)
}

fn key_blur(level: Elevation) -> f32 {
    level.dp() * 2.0
}

fn same_rect(a: Rect, b: Rect) -> bool {
    (a.min - b.min).length() < 0.5 && (a.max - b.max).length() < 0.5
}

fn light() -> MaterialTheme {
    MaterialTheme::baseline(false)
}

#[test]
fn elevation_follows_the_compose_tokens() {
    let hovered = InteractionState {
        hovered: true,
        ..Default::default()
    };
    let dragged = InteractionState {
        dragged: true,
        ..Default::default()
    };
    let pressed = InteractionState {
        pressed: true,
        hovered: true,
        ..Default::default()
    };
    let idle = InteractionState::default();
    let e = Card2Variant::Elevated;
    let f = Card2Variant::Filled;
    let o = Card2Variant::Outlined;
    // ElevatedCardTokens: Level1 resting, Level2 hover, Level1 press, Level4 drag.
    assert_eq!(
        e.elevation(true, true, idle),
        Elevation::Level1,
        "elevated rest"
    );
    assert_eq!(
        e.elevation(true, true, hovered),
        Elevation::Level2,
        "elevated hover"
    );
    assert_eq!(
        e.elevation(true, true, pressed),
        Elevation::Level1,
        "elevated press"
    );
    assert_eq!(
        e.elevation(true, true, dragged),
        Elevation::Level4,
        "elevated drag"
    );
    assert_eq!(
        e.elevation(false, true, hovered),
        Elevation::Level1,
        "elevated disabled"
    );
    // FilledCardTokens / OutlinedCardTokens: Level0, Level1 hover, Level3 drag.
    for v in [f, o] {
        assert_eq!(
            v.elevation(true, true, idle),
            Elevation::Level0,
            "{v:?} rest"
        );
        assert_eq!(
            v.elevation(true, true, hovered),
            Elevation::Level1,
            "{v:?} hover"
        );
        assert_eq!(
            v.elevation(true, true, dragged),
            Elevation::Level3,
            "{v:?} drag"
        );
    }
    // A card without a click handler has no interaction elevation.
    assert_eq!(
        e.elevation(true, false, hovered),
        Elevation::Level1,
        "static card"
    );
}

#[test]
fn containers_use_their_colour_roles() -> TestResult {
    for (name, theme) in support::themes() {
        for variant in Card2Variant::ALL {
            let rect = Cell::new(Rect::NOTHING);
            let harness = support::harness_for(theme.clone(), egui::vec2(400.0, 200.0), |ui| {
                let card = MaterialCard2::new_with_variant(variant).header("Title", None::<String>);
                rect.set(ui.add(card).rect);
            });
            let expected = theme.color(variant.container_role());
            let fill = rects(&harness)
                .into_iter()
                .find(|r| same_rect(r.rect, rect.get()) && r.blur_width == 0.0 && r.fill.a() > 0)
                .map(|r| r.fill)
                .ok_or("container painted")?;
            assert_eq!(fill, expected, "{name} {variant:?}: container colour");
            let outline = rects(&harness)
                .into_iter()
                .find(|r| same_rect(r.rect, rect.get()) && r.stroke.width > 0.0)
                .map(|r| r.stroke);
            match variant {
                Card2Variant::Outlined => assert_eq!(
                    outline.map(|s| (s.width, s.color)),
                    Some((1.0, theme.color(ColorRole::OutlineVariant))),
                    "{name}: outline 1 dp outlineVariant"
                ),
                Card2Variant::Elevated | Card2Variant::Filled => {
                    assert_eq!(outline, None, "{name} {variant:?}: no outline");
                }
            }
        }
    }
    Ok(())
}

#[test]
fn clickable_cards_rise_on_hover_and_static_ones_do_not() -> TestResult {
    for clickable in [true, false] {
        let rect = Cell::new(Rect::NOTHING);
        let mut harness = support::harness_for(light(), egui::vec2(400.0, 200.0), |ui| {
            let card = MaterialCard2::elevated()
                .header("Hover me", None::<String>)
                .clickable(clickable);
            rect.set(ui.add(card).rect);
        });
        assert_eq!(
            shadow_blur(&harness),
            key_blur(Elevation::Level1),
            "clickable={clickable}: Level1 at rest"
        );
        harness.hover_at(rect.get().center());
        harness.run();
        let expected = if clickable {
            Elevation::Level2
        } else {
            Elevation::Level1
        };
        assert_eq!(
            shadow_blur(&harness),
            key_blur(expected),
            "clickable={clickable}: hovered elevation"
        );
    }
    Ok(())
}

#[test]
fn hover_paints_an_on_surface_state_layer() -> TestResult {
    let theme = light();
    let on_surface = theme.color(ColorRole::OnSurface);
    let rect = Cell::new(Rect::NOTHING);
    let mut harness = support::harness_for(theme, egui::vec2(400.0, 200.0), |ui| {
        let card = MaterialCard2::filled()
            .header("Hover me", None::<String>)
            .clickable(true);
        rect.set(ui.add(card).rect);
    });
    let layer = on_surface.gamma_multiply(0.08);
    let has_layer = |h: &Harness<'_>| {
        rects(h)
            .iter()
            .any(|r| same_rect(r.rect, rect.get()) && r.fill == layer)
    };
    assert!(!has_layer(&harness), "no state layer at rest");
    harness.hover_at(rect.get().center());
    harness.run();
    assert!(has_layer(&harness), "onSurface at 8 % when hovered");
    Ok(())
}

#[test]
fn checkable_card_toggles_on_click_and_reports_its_state() -> TestResult {
    let checked = Cell::new(false);
    let mut harness = support::harness_for(light(), egui::vec2(400.0, 200.0), |ui| {
        let mut value = checked.get();
        ui.add(
            MaterialCard2::outlined()
                .header("Pick me", Some("Selectable"))
                .checked(&mut value),
        );
        checked.set(value);
    });
    let node = harness.get_by_label("Pick me");
    assert_eq!(
        node.accesskit_node().role(),
        Role::CheckBox,
        "checkbox role"
    );
    assert_eq!(
        node.accesskit_node().toggled(),
        Some(Toggled::False),
        "unchecked"
    );
    node.click();
    harness.run();
    assert!(checked.get(), "a click checks the card");
    assert_eq!(
        harness.get_by_label("Pick me").accesskit_node().toggled(),
        Some(Toggled::True),
        "AccessKit reports checked"
    );
    harness.get_by_label("Pick me").click();
    harness.run();
    assert!(!checked.get(), "a second click unchecks it");
    Ok(())
}

#[test]
fn selected_outlined_card_uses_secondary() -> TestResult {
    let theme = light();
    let secondary = theme.color(ColorRole::Secondary);
    let rect = Cell::new(Rect::NOTHING);
    let harness = support::harness_for(theme, egui::vec2(400.0, 200.0), |ui| {
        let card = MaterialCard2::outlined()
            .header("Selected", None::<String>)
            .selected(true);
        rect.set(ui.add(card).rect);
    });
    let all = rects(&harness);
    assert!(
        all.iter()
            .any(|r| same_rect(r.rect, rect.get()) && r.stroke.color == secondary),
        "outline turns secondary"
    );
    assert!(
        all.iter()
            .any(|r| same_rect(r.rect, rect.get()) && r.fill == secondary.gamma_multiply(0.16)),
        "secondary overlay at 16 %"
    );
    Ok(())
}

#[test]
fn keyboard_focuses_and_activates_cards() -> TestResult {
    let checked = Cell::new(false);
    let clicks = Cell::new(0);
    let mut harness = support::harness_for(light(), egui::vec2(400.0, 400.0), |ui| {
        if ui
            .add(
                MaterialCard2::elevated()
                    .header("Open", None::<String>)
                    .clickable(true),
            )
            .clicked()
        {
            clicks.set(clicks.get() + 1);
        }
        let mut value = checked.get();
        ui.add(
            MaterialCard2::filled()
                .header("Toggle", None::<String>)
                .checked(&mut value),
        );
        checked.set(value);
    });
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Open").is_focused(),
        "Tab reaches the card"
    );
    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(clicks.get(), 1, "Enter activates a clickable card");

    harness.key_press(Key::Tab);
    harness.run();
    assert!(harness.get_by_label("Toggle").is_focused(), "Tab moves on");
    harness.key_press(Key::Space);
    harness.run();
    assert!(checked.get(), "Space checks a checkable card");
    Ok(())
}

#[test]
fn focused_card_paints_a_secondary_focus_ring() -> TestResult {
    let theme = light();
    let secondary = theme.color(ColorRole::Secondary);
    let mut harness = support::harness_for(theme, egui::vec2(400.0, 200.0), |ui| {
        ui.add(
            MaterialCard2::outlined()
                .header("Focus", None::<String>)
                .clickable(true),
        );
    });
    harness.get_by_label("Focus").focus();
    harness.run();
    assert!(
        rects(&harness)
            .iter()
            .any(|r| r.stroke.width == 3.0 && r.stroke.color == secondary),
        "3 dp secondary ring"
    );
    Ok(())
}

#[test]
fn disabled_card_ignores_clicks_and_reports_disabled() -> TestResult {
    let checked = Cell::new(false);
    let mut harness = support::harness_for(light(), egui::vec2(400.0, 200.0), |ui| {
        let mut value = checked.get();
        ui.add(
            MaterialCard2::elevated()
                .header("Off", None::<String>)
                .checked(&mut value)
                .enabled(false),
        );
        checked.set(value);
    });
    let node = harness.get_by_label("Off");
    assert!(node.accesskit_node().is_disabled(), "reports disabled");
    node.click();
    harness.run();
    assert!(!checked.get(), "a disabled card does not toggle");
    Ok(())
}

#[test]
fn height_follows_content() -> TestResult {
    let short = Cell::new(Rect::NOTHING);
    let tall = Cell::new(Rect::NOTHING);
    let _harness = support::harness_for(light(), egui::vec2(400.0, 600.0), |ui| {
        short.set(
            ui.add(MaterialCard2::filled().header("Title", None::<String>))
                .rect,
        );
        tall.set(
            ui.add(
                MaterialCard2::filled()
                    .header("Title", None::<String>)
                    .content(|ui| {
                        ui.label("One");
                        ui.label("Two");
                        ui.label("Three");
                    }),
            )
            .rect,
        );
    });
    // Title Large has a 28 dp line height; 16 dp padding above and below.
    assert!(
        (short.get().height() - (28.0 + 32.0)).abs() < 1.5,
        "header-only card is padding + one title line: {}",
        short.get().height()
    );
    assert!(
        tall.get().height() > short.get().height() + 30.0,
        "three lines of content make the card taller"
    );
    Ok(())
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
        Err(err) => Err(err.to_string().into()),
    }
}

fn light_and_dark() -> Vec<(&'static str, MaterialTheme)> {
    support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
        .collect()
}

#[test]
fn hover_snapshot_shows_level2() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let rect = Cell::new(Rect::NOTHING);
        let mut harness = support::harness_for(theme, egui::vec2(320.0, 120.0), |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.add_space(12.0);
            let card = MaterialCard2::elevated()
                .header("Hovered card", Some("Level 2"))
                .width(280.0)
                .margin(12.0)
                .clickable(true);
            rect.set(ui.add(card).rect);
        });
        harness.hover_at(rect.get().center());
        harness.run();
        assert_eq!(
            shadow_blur(&harness),
            key_blur(Elevation::Level2),
            "{name}: hovered card is at Level2"
        );
        snapshot(&mut harness, &format!("cards_hover_{name}"))?;
    }
    Ok(())
}

#[test]
fn variants_and_states_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(900.0, 380.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.horizontal_top(|ui| {
                for variant in Card2Variant::ALL {
                    ui.vertical(|ui| {
                        ui.add(
                            MaterialCard2::new_with_variant(variant)
                                .header(format!("{variant:?}"), Some("Resting"))
                                .width(280.0)
                                .margin(8.0),
                        );
                        ui.add(
                            MaterialCard2::new_with_variant(variant)
                                .header("Selected", Some("Checked"))
                                .width(280.0)
                                .margin(8.0)
                                .selected(true),
                        );
                        ui.add(
                            MaterialCard2::new_with_variant(variant)
                                .header("Disabled", Some("Not interactive"))
                                .width(280.0)
                                .margin(8.0)
                                .clickable(true)
                                .enabled(false),
                        );
                    });
                }
            });
        });
        harness.run();
        snapshot(&mut harness, &format!("cards_states_{name}"))?;
    }
    Ok(())
}

#[test]
fn elevated_container_is_not_tinted() {
    // Guard against a regression to the old surface-tint blend, which
    // produced an opaque mix of the container and primary.
    let theme = light();
    let primary = theme.color(ColorRole::Primary);
    let harness = support::harness_for(theme.clone(), egui::vec2(400.0, 200.0), |ui| {
        ui.add(MaterialCard2::elevated().header("No tint", None::<String>));
    });
    let container = theme.color(ColorRole::SurfaceContainerLow);
    assert!(
        rects(&harness).iter().any(|r| r.fill == container),
        "elevated container is plain surfaceContainerLow"
    );
    assert!(
        !rects(&harness).iter().any(|r| r.fill == primary),
        "no primary fill"
    );
}
