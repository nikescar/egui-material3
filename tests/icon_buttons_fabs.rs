// ============================================================================
// Module:       icon_buttons_fabs (integration test)
// Description:  Icon buttons, FABs, extended FABs and the FAB menu.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Icon buttons, FABs and the FAB menu (WP-4): token geometry per size and
//! width, shapes and the pressed morph, toggle colour sets, elevation
//! shadows, the extended FAB's collapse, AccessKit roles and states,
//! keyboard activation, and snapshots in light and dark.

mod support;

use std::cell::Cell;
use std::rc::Rc;

use egui::accesskit::{Role, Toggled};
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape, TextShape};
use egui::load::SizedTexture;
use egui::{Color32, Event, Pos2, Rect, TextureId, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use egui_material3::fonts;
use egui_material3::tokens::{ColorRole, Elevation, InteractionState};
use egui_material3::{
    fab::fab_elevation, FabMenu, FabMenuColors, FabSize, FabVariant, Icon, IconButtonShape,
    IconButtonSize, IconButtonVariant, IconButtonWidth, MaterialFab, MaterialIconButton,
    MaterialTheme,
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
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(1200.0, 600.0))),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let mut output = self.ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        output.shapes
    }
}

fn theme() -> MaterialTheme {
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

/// The bounds of every mesh (images: texture icons).
fn mesh_bounds(shapes: &[ClippedShape]) -> Vec<Rect> {
    shapes
        .iter()
        .filter_map(|c| match &c.shape {
            PaintShape::Mesh(m) => Some(m.calc_bounds()),
            PaintShape::Noop
            | PaintShape::Vec(_)
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Band(_)
            | PaintShape::Rect(_)
            | PaintShape::Text(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => None,
        })
        .collect()
}

fn near(a: Rect, b: Rect) -> bool {
    (a.min - b.min).length() < 0.5 && (a.max - b.max).length() < 0.5
}

/// The unblurred rects painted exactly at `rect`, in paint order.
fn rects_at(shapes: &[ClippedShape], rect: Rect) -> Vec<RectShape> {
    rects(shapes)
        .into_iter()
        .filter(|r| r.blur_width == 0.0 && near(r.rect, rect))
        .collect()
}

/// The widest shadow blur painted.
fn max_blur(shapes: &[ClippedShape]) -> f32 {
    rects(shapes)
        .iter()
        .map(|r| r.blur_width)
        .fold(0.0, f32::max)
}

/// A square texture icon: painted as a mesh filling the icon rect.
fn texture_icon() -> Icon {
    Icon::Texture(SizedTexture::new(
        TextureId::Managed(0),
        egui::vec2(10.0, 10.0),
    ))
}

fn press(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

/// Draws `button` and returns its response rect, with the shapes.
fn draw_icon_button(
    make: &dyn Fn() -> MaterialIconButton<'static>,
) -> (Rect, Vec<ClippedShape>, Driver) {
    let mut driver = Driver::new();
    let rect = Cell::new(Rect::NOTHING);
    let shapes = driver.frame(0.0, vec![], &mut |ui| {
        rect.set(ui.add(make()).rect);
    });
    (rect.get(), shapes, driver)
}

// ---------------------------------------------------------------------------
// Icon button geometry

/// `IconButton<Size>Tokens`: (size, height, icon, narrow, default, wide).
const ICON_BUTTON_TOKENS: [(IconButtonSize, f32, f32, f32, f32, f32); 5] = [
    (IconButtonSize::ExtraSmall, 32.0, 20.0, 28.0, 32.0, 40.0),
    (IconButtonSize::Small, 40.0, 24.0, 32.0, 40.0, 52.0),
    (IconButtonSize::Medium, 56.0, 24.0, 48.0, 56.0, 72.0),
    (IconButtonSize::Large, 96.0, 32.0, 64.0, 96.0, 128.0),
    (IconButtonSize::ExtraLarge, 136.0, 40.0, 104.0, 136.0, 184.0),
];

#[test]
fn icon_button_sizes_and_widths_match_their_tokens() -> TestResult {
    for (size, height, _, narrow, default, wide) in ICON_BUTTON_TOKENS {
        for (width, expected) in [
            (IconButtonWidth::Narrow, narrow),
            (IconButtonWidth::Default, default),
            (IconButtonWidth::Wide, wide),
        ] {
            let (response, shapes, _) =
                draw_icon_button(&|| MaterialIconButton::filled("add").size(size).width(width));
            let container = Rect::from_center_size(response.center(), egui::vec2(expected, height));
            assert!(
                !rects_at(&shapes, container).is_empty(),
                "{size:?} {width:?}: container {expected}x{height}"
            );
            assert!(
                (response.width() - expected.max(48.0)).abs() < 0.5
                    && (response.height() - height.max(48.0)).abs() < 0.5,
                "{size:?} {width:?}: touch target at least 48, got {:?}",
                response.size()
            );
        }
    }
    Ok(())
}

#[test]
fn icon_button_icon_sizes_match_their_tokens() -> TestResult {
    for (size, _, icon, ..) in ICON_BUTTON_TOKENS {
        let (_, shapes, _) =
            draw_icon_button(&|| MaterialIconButton::standard(texture_icon()).size(size));
        let bounds = mesh_bounds(&shapes);
        let Some(icon_rect) = bounds.first() else {
            return Err(format!("{size:?}: no icon painted").into());
        };
        assert!(
            (icon_rect.width() - icon).abs() < 0.5 && (icon_rect.height() - icon).abs() < 0.5,
            "{size:?}: icon {icon}, got {:?}",
            icon_rect.size()
        );
    }
    Ok(())
}

#[test]
fn legacy_pixel_sizes_still_work() -> TestResult {
    assert_eq!(
        IconButtonSize::from(40.0),
        IconButtonSize::Small,
        "40 is the small token size"
    );
    assert_eq!(
        IconButtonSize::from(48.0),
        IconButtonSize::Custom(48.0),
        "48 is a custom size"
    );
    let (response, shapes, _) = draw_icon_button(&|| MaterialIconButton::filled("add").size(48.0));
    let container = Rect::from_center_size(response.center(), egui::vec2(48.0, 48.0));
    assert!(
        !rects_at(&shapes, container).is_empty(),
        "a 48 custom button paints a 48 container"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Shapes

fn radius_of(shapes: &[ClippedShape], container: Rect) -> Option<u8> {
    rects_at(shapes, container)
        .first()
        .map(|r| r.corner_radius.nw)
}

#[test]
fn square_toggle_and_pressed_shapes_match_their_tokens() -> TestResult {
    // Small: ContainerShapeSquare CornerMedium 12, PressedContainerShape
    // CornerSmall 8, SelectedContainerShapeRound CornerMedium 12,
    // SelectedContainerShapeSquare CornerFull.
    let (r, shapes, _) =
        draw_icon_button(&|| MaterialIconButton::filled("add").shape(IconButtonShape::Square));
    let container = Rect::from_center_size(r.center(), egui::vec2(40.0, 40.0));
    assert_eq!(radius_of(&shapes, container), Some(12), "square is 12");

    let (r, shapes, _) = draw_icon_button(&|| MaterialIconButton::filled("add"));
    let container = Rect::from_center_size(r.center(), egui::vec2(40.0, 40.0));
    assert_eq!(radius_of(&shapes, container), Some(20), "round is full");

    let mut driver = Driver::new();
    let on = Rc::new(Cell::new(true));
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        let mut value = on.get();
        rect.set(
            ui.add(MaterialIconButton::filled("add").toggleable(&mut value))
                .rect,
        );
        on.set(value);
    };
    let shapes = driver.frame(0.0, vec![], &mut app);
    let container = Rect::from_center_size(rect.get().center(), egui::vec2(40.0, 40.0));
    assert_eq!(
        radius_of(&shapes, container),
        Some(12),
        "a selected round toggle is 12"
    );

    // Hold the button down; the corners spring to the pressed shape (egui
    // ends a click-only press after its maximum click duration, so sample
    // within half a second).
    let center = container.center();
    driver.frame(0.016, vec![Event::PointerMoved(center)], &mut app);
    driver.frame(0.016, vec![press(center, true)], &mut app);
    let mut shapes = Vec::new();
    for _ in 0..30 {
        shapes = driver.frame(0.016, vec![], &mut app);
    }
    assert_eq!(radius_of(&shapes, container), Some(8), "pressed is 8");
    Ok(())
}

// ---------------------------------------------------------------------------
// Colours

/// The container of a small `variant` button, toggled when `selected` is set.
fn container_fill(
    variant: IconButtonVariant,
    selected: Option<bool>,
    enabled: bool,
) -> Option<RectShape> {
    let mut driver = Driver::new();
    let rect = Cell::new(Rect::NOTHING);
    let shapes = driver.frame(0.0, vec![], &mut |ui| {
        let mut flag = selected.unwrap_or_default();
        let button = MaterialIconButton::new("add", variant).enabled(enabled);
        let button = match selected {
            Some(_) => button.toggleable(&mut flag),
            None => button,
        };
        rect.set(ui.add(button).rect);
    });
    let container = Rect::from_center_size(rect.get().center(), egui::vec2(40.0, 40.0));
    rects_at(&shapes, container).into_iter().next()
}

#[test]
fn icon_button_colours_follow_the_tokens() -> TestResult {
    let t = theme();
    let outlined = container_fill(IconButtonVariant::Outlined, None, true)
        .ok_or("outlined paints its border")?;
    assert_eq!(
        outlined.stroke.color,
        t.color(ColorRole::OutlineVariant),
        "outlined border is outlineVariant"
    );
    assert_eq!(outlined.fill, Color32::TRANSPARENT, "outlined has no fill");

    for (variant, on, role) in [
        (IconButtonVariant::Outlined, true, ColorRole::InverseSurface),
        (
            IconButtonVariant::Filled,
            false,
            ColorRole::SurfaceContainer,
        ),
        (IconButtonVariant::Filled, true, ColorRole::Primary),
        (
            IconButtonVariant::FilledTonal,
            false,
            ColorRole::SecondaryContainer,
        ),
        (IconButtonVariant::FilledTonal, true, ColorRole::Secondary),
    ] {
        let fill = container_fill(variant, Some(on), true)
            .ok_or("toggle paints a container")?
            .fill;
        assert_eq!(fill, t.color(role), "{variant:?} on={on}: {role:?}");
    }

    let disabled = container_fill(IconButtonVariant::Filled, None, false)
        .ok_or("disabled filled paints a container")?;
    assert_eq!(
        disabled.fill,
        t.color(ColorRole::OnSurface).gamma_multiply(0.1),
        "disabled container is onSurface at 0.1"
    );
    Ok(())
}

#[test]
fn hovered_standard_button_shows_a_state_layer() -> TestResult {
    let t = theme();
    let mut driver = Driver::new();
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| rect.set(ui.add(MaterialIconButton::standard("add")).rect);
    let rest = driver.frame(0.0, vec![], &mut app);
    let container = Rect::from_center_size(rect.get().center(), egui::vec2(40.0, 40.0));
    assert!(
        rects_at(&rest, container).is_empty(),
        "nothing behind at rest"
    );
    driver.frame(
        0.016,
        vec![Event::PointerMoved(container.center())],
        &mut app,
    );
    let hovered = driver.frame(0.016, vec![], &mut app);
    let layer = rects_at(&hovered, container)
        .into_iter()
        .next()
        .ok_or("hover paints a layer")?;
    assert_eq!(
        layer.fill,
        t.color(ColorRole::OnSurfaceVariant).gamma_multiply(0.08),
        "hover layer is the icon colour at 0.08"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// FABs

/// `Fab<Size>Tokens`: (size, side, icon, radius).
const FAB_TOKENS: [(FabSize, f32, f32, u8); 4] = [
    (FabSize::Small, 40.0, 24.0, 12),
    (FabSize::Regular, 56.0, 24.0, 16),
    (FabSize::Medium, 80.0, 28.0, 20),
    (FabSize::Large, 96.0, 36.0, 28),
];

#[test]
fn fab_sizes_radii_and_icons_match_their_tokens() -> TestResult {
    for (size, side, icon, radius) in FAB_TOKENS {
        let mut driver = Driver::new();
        let rect = Cell::new(Rect::NOTHING);
        let shapes = driver.frame(0.0, vec![], &mut |ui| {
            rect.set(
                ui.add(
                    MaterialFab::primary_container()
                        .icon(texture_icon())
                        .size(size),
                )
                .rect,
            );
        });
        let rect = rect.get();
        assert!(
            (rect.width() - side).abs() < 0.5 && (rect.height() - side).abs() < 0.5,
            "{size:?}: {side} square, got {:?}",
            rect.size()
        );
        let container = rects_at(&shapes, rect)
            .into_iter()
            .next()
            .ok_or("container painted")?;
        assert_eq!(container.corner_radius.nw, radius, "{size:?}: radius");
        assert_eq!(
            container.fill,
            theme().color(ColorRole::PrimaryContainer),
            "{size:?}: primaryContainer"
        );
        let icon_rect = mesh_bounds(&shapes)
            .into_iter()
            .next()
            .ok_or("icon painted")?;
        assert!(
            (icon_rect.width() - icon).abs() < 0.5,
            "{size:?}: icon {icon}, got {}",
            icon_rect.width()
        );
    }
    Ok(())
}

#[test]
fn fab_colour_pairs_follow_the_tokens() -> TestResult {
    let t = theme();
    for variant in FabVariant::ALL {
        let (container_role, _) = variant.roles();
        let mut driver = Driver::new();
        let rect = Cell::new(Rect::NOTHING);
        let shapes = driver.frame(0.0, vec![], &mut |ui| {
            rect.set(ui.add(MaterialFab::new(variant)).rect);
        });
        let fill = rects_at(&shapes, rect.get())
            .into_iter()
            .next()
            .ok_or("container painted")?
            .fill;
        assert_eq!(fill, t.color(container_role), "{variant:?}");
    }
    Ok(())
}

#[test]
fn fab_casts_a_level3_shadow_that_rises_on_hover() -> TestResult {
    assert_eq!(
        fab_elevation(InteractionState::default(), false),
        Elevation::Level3,
        "rest is Level3"
    );
    let hovered = InteractionState {
        hovered: true,
        ..Default::default()
    };
    assert_eq!(
        fab_elevation(hovered, false),
        Elevation::Level4,
        "hover is Level4"
    );
    assert_eq!(
        fab_elevation(hovered, true),
        Elevation::Level2,
        "lowered hover is Level2"
    );

    let mut driver = Driver::new();
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        ui.add_space(20.0);
        rect.set(ui.add(MaterialFab::primary_container()).rect);
    };
    let rest = driver.frame(0.0, vec![], &mut app);
    let rest_blur = max_blur(&rest);
    assert!(rest_blur > 0.0, "a resting FAB draws a blurred shadow");
    driver.frame(
        0.016,
        vec![Event::PointerMoved(rect.get().center())],
        &mut app,
    );
    let hover = driver.frame(0.016, vec![], &mut app);
    assert!(
        max_blur(&hover) > rest_blur,
        "the hovered FAB's shadow is wider ({} > {rest_blur})",
        max_blur(&hover)
    );
    Ok(())
}

/// `ExtendedFab<Size>Tokens`: (size, height, leading + icon + gap + trailing).
const EXTENDED_TOKENS: [(FabSize, f32, f32); 3] = [
    (FabSize::Regular, 56.0, 16.0 + 24.0 + 8.0 + 16.0),
    (FabSize::Medium, 80.0, 26.0 + 28.0 + 12.0 + 26.0),
    (FabSize::Large, 96.0, 28.0 + 36.0 + 16.0 + 28.0),
];

#[test]
fn extended_fab_geometry_matches_its_tokens() -> TestResult {
    for (size, height, chrome) in EXTENDED_TOKENS {
        let mut driver = Driver::new();
        let rect = Cell::new(Rect::NOTHING);
        let shapes = driver.frame(0.0, vec![], &mut |ui| {
            rect.set(
                ui.add(
                    MaterialFab::primary_container()
                        .icon("edit")
                        .text("Compose")
                        .size(size),
                )
                .rect,
            );
        });
        let label = texts(&shapes)
            .into_iter()
            .find(|t| t.galley.text() == "Compose")
            .ok_or("label painted")?;
        let rect = rect.get();
        assert!(
            (rect.height() - height).abs() < 0.5,
            "{size:?}: height {height}"
        );
        assert!(
            (rect.width() - (chrome + label.galley.size().x)).abs() < 0.5,
            "{size:?}: width is padding, icon, gap and label"
        );
    }
    Ok(())
}

#[test]
fn extended_fab_collapses_to_a_square_and_expands_again() -> TestResult {
    let mut driver = Driver::new();
    let expanded = Rc::new(Cell::new(true));
    let rect = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        rect.set(
            ui.add(
                MaterialFab::primary_container()
                    .icon("edit")
                    .text("Compose")
                    .expanded(expanded.get()),
            )
            .rect,
        );
    };
    driver.frame(0.0, vec![], &mut app);
    let full = rect.get().width();
    assert!(full > 56.0, "starts expanded");
    expanded.set(false);
    driver.frame(0.016, vec![], &mut app);
    driver.frame(0.05, vec![], &mut app);
    let mid = rect.get().width();
    assert!(
        mid < full && mid > 56.0,
        "animates: {mid} between 56 and {full}"
    );
    for _ in 0..60 {
        driver.frame(0.05, vec![], &mut app);
    }
    assert!((rect.get().width() - 56.0).abs() < 0.5, "collapsed to 56");
    expanded.set(true);
    for _ in 0..60 {
        driver.frame(0.05, vec![], &mut app);
    }
    assert!((rect.get().width() - full).abs() < 0.5, "expanded again");
    Ok(())
}

// ---------------------------------------------------------------------------
// AccessKit and keyboard

#[test]
fn icon_button_reports_label_role_and_toggle_state() {
    let mut on = false;
    let mut harness = support::harness_for(theme(), egui::vec2(300.0, 100.0), |ui| {
        ui.add(
            MaterialIconButton::filled("star")
                .toggleable(&mut on)
                .label("Star"),
        );
        ui.add(MaterialIconButton::standard("delete"));
    });
    let star = || harness.get_by_label("Star").accesskit_node().toggled();
    assert_eq!(star(), Some(Toggled::False), "starts off");
    assert_eq!(
        harness.get_by_label("delete").accesskit_node().role(),
        Role::Button,
        "a symbol name labels an unlabeled button"
    );
    harness.get_by_label("Star").click();
    harness.run();
    assert_eq!(
        harness.get_by_label("Star").accesskit_node().toggled(),
        Some(Toggled::True),
        "click turns it on"
    );
    drop(harness);
    assert!(on, "the bound flag flipped");
}

#[test]
fn disabled_icon_button_and_fab_do_not_click() {
    let clicks = Cell::new(0);
    let mut harness = support::harness_for(theme(), egui::vec2(300.0, 100.0), |ui| {
        if ui
            .add(
                MaterialIconButton::filled("add")
                    .label("Add")
                    .enabled(false),
            )
            .clicked()
        {
            clicks.set(clicks.get() + 1);
        }
        if ui
            .add(MaterialFab::primary().label("Compose").enabled(false))
            .clicked()
        {
            clicks.set(clicks.get() + 1);
        }
    });
    for label in ["Add", "Compose"] {
        assert!(
            harness.get_by_label(label).accesskit_node().is_disabled(),
            "{label} reports disabled"
        );
        harness.get_by_label(label).click();
        harness.run();
    }
    drop(harness);
    assert_eq!(clicks.get(), 0, "disabled buttons report no click");
}

#[test]
fn tab_and_space_activate_an_icon_button() {
    let mut on = false;
    let mut harness = support::harness_for(theme(), egui::vec2(300.0, 100.0), |ui| {
        ui.add(
            MaterialIconButton::outlined("bookmark")
                .toggleable(&mut on)
                .label("Save"),
        );
    });
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(harness.get_by_label("Save").is_focused(), "Tab reaches it");
    harness.key_press(egui::Key::Space);
    harness.run();
    assert_eq!(
        harness.get_by_label("Save").accesskit_node().toggled(),
        Some(Toggled::True),
        "Space toggles it"
    );
    drop(harness);
    assert!(on, "the bound flag flipped");
}

#[test]
fn fab_reports_a_labelled_button() {
    let harness = support::harness_for(theme(), egui::vec2(400.0, 200.0), |ui| {
        ui.add(
            MaterialFab::primary_container()
                .icon("edit")
                .text("Compose"),
        );
        ui.add(MaterialFab::secondary().icon("add"));
    });
    assert_eq!(
        harness.get_by_label("Compose").accesskit_node().role(),
        Role::Button,
        "extended FAB is a button named by its text"
    );
    assert_eq!(
        harness.get_by_label("add").accesskit_node().role(),
        Role::Button,
        "icon FAB is named by its symbol"
    );
}

// ---------------------------------------------------------------------------
// FAB menu

fn menu_harness<'a>(open: &'a mut bool, chosen: &'a Cell<Option<usize>>) -> Harness<'a> {
    support::harness_for(theme(), egui::vec2(400.0, 400.0), move |ui| {
        ui.add_space(280.0);
        let response = FabMenu::new(open)
            .item("reply", "Reply")
            .item("forward", "Forward")
            .show(ui);
        if response.clicked.is_some() {
            chosen.set(response.clicked);
        }
    })
}

#[test]
fn fab_menu_opens_and_escape_closes_it() {
    let mut open = false;
    let chosen = Cell::new(None);
    let mut harness = menu_harness(&mut open, &chosen);
    let toggled = |h: &Harness<'_>| h.get_by_label("Menu").accesskit_node().toggled();
    assert_eq!(toggled(&harness), Some(Toggled::False), "starts closed");
    assert!(
        harness.query_by_label("Reply").is_none(),
        "no items when closed"
    );

    harness.get_by_label("Menu").click();
    harness.run();
    assert_eq!(toggled(&harness), Some(Toggled::True), "click opens it");
    assert_eq!(
        harness.get_by_label("Reply").accesskit_node().role(),
        Role::MenuItem,
        "items are menu items"
    );
    assert_eq!(
        harness.get_all_by_role(Role::MenuItem).count(),
        2,
        "both items shown"
    );

    harness.key_press(egui::Key::Escape);
    harness.run();
    assert_eq!(toggled(&harness), Some(Toggled::False), "Escape closes it");
    assert!(harness.query_by_label("Reply").is_none(), "items gone");
    assert!(
        harness.get_by_label("Menu").is_focused(),
        "focus returns to the button"
    );
    drop(harness);
    assert!(!open, "the bound flag is false");
}

#[test]
fn fab_menu_item_click_reports_and_closes() {
    let mut open = true;
    let chosen = Cell::new(None);
    let mut harness = menu_harness(&mut open, &chosen);
    harness.run();
    harness.get_by_label("Forward").click();
    harness.run();
    assert_eq!(chosen.get(), Some(1), "Forward is item 1");
    assert_eq!(
        harness.get_by_label("Menu").accesskit_node().toggled(),
        Some(Toggled::False),
        "choosing closes the menu"
    );
    drop(harness);
    assert!(!open, "the bound flag is false");
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

fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    // Software and hardware rasterisers differ in anti-aliasing; allow that,
    // not a missing or moved button.
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(600);
    match no_adapter_is_an_error(|| harness.try_snapshot_options(name, &options)) {
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

/// A harness with the bundled fonts, so icons render as glyphs.
fn font_harness<'a>(
    theme: MaterialTheme,
    size: Vec2,
    app: impl FnMut(&mut egui::Ui) + 'a,
) -> Harness<'a> {
    let mut harness = Harness::builder()
        .with_size(size)
        .with_context_setup(move |ctx| {
            fonts::install(ctx);
            MaterialTheme::set(ctx, theme);
        })
        .build_ui(app);
    harness.run();
    harness
}

fn light_and_dark() -> Vec<(&'static str, MaterialTheme)> {
    support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
        .collect()
}

#[test]
fn icon_buttons_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = font_harness(theme, egui::vec2(760.0, 420.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            for variant in IconButtonVariant::ALL {
                ui.horizontal(|ui| {
                    for size in [
                        IconButtonSize::ExtraSmall,
                        IconButtonSize::Small,
                        IconButtonSize::Medium,
                    ] {
                        ui.add(MaterialIconButton::new("favorite", variant).size(size));
                    }
                    ui.add(
                        MaterialIconButton::new("favorite", variant)
                            .shape(IconButtonShape::Square)
                            .width(IconButtonWidth::Wide),
                    );
                    let mut off = false;
                    let mut on = true;
                    ui.add(MaterialIconButton::new("favorite", variant).toggleable(&mut off));
                    ui.add(MaterialIconButton::new("favorite", variant).toggleable(&mut on));
                    ui.add(MaterialIconButton::new("favorite", variant).enabled(false));
                    let mut on_disabled = true;
                    ui.add(
                        MaterialIconButton::new("favorite", variant)
                            .toggleable(&mut on_disabled)
                            .enabled(false),
                    );
                });
            }
        });
        snapshot(&mut harness, &format!("icon_buttons_{name}"))?;
    }
    Ok(())
}

#[test]
fn fabs_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = font_harness(theme, egui::vec2(820.0, 420.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                for variant in FabVariant::ALL {
                    ui.add(MaterialFab::new(variant).icon("edit"));
                    ui.add_space(16.0);
                }
            });
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                for size in FabSize::ALL {
                    ui.add(MaterialFab::primary_container().icon("edit").size(size));
                    ui.add_space(16.0);
                }
            });
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.add(
                    MaterialFab::primary_container()
                        .icon("edit")
                        .text("Compose"),
                );
                ui.add_space(16.0);
                ui.add(
                    MaterialFab::tertiary_container()
                        .icon("edit")
                        .text("Compose")
                        .size(FabSize::Medium),
                );
            });
        });
        snapshot(&mut harness, &format!("fabs_{name}"))?;
    }
    Ok(())
}

#[test]
fn fab_menu_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = font_harness(theme, egui::vec2(640.0, 320.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.add_space(240.0);
            ui.horizontal(|ui| {
                for colors in FabMenuColors::ALL {
                    ui.add_space(150.0);
                    let mut open = true;
                    let salt = format!("{colors:?}");
                    let _response = FabMenu::new(&mut open)
                        .id_salt(salt)
                        .colors(colors)
                        .item("reply", "Reply")
                        .item("reply_all", "Reply all")
                        .item("forward", "Forward")
                        .show(ui);
                }
            });
        });
        snapshot(&mut harness, &format!("fab_menu_open_{name}"))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Symbol glyphs use the symbol family

/// The glyph uv (as text) and family of the first text shape an icon button paints,
/// with the bundled fonts, optionally after an app font that maps the
/// same private-use codepoints to other glyphs is put first in
/// `Proportional`.
fn icon_glyph(app_font_first: bool) -> Result<(egui::FontFamily, String), String> {
    let mut defs = fonts::material_font_definitions();
    if app_font_first {
        // The filled Material Symbols face stands in for an app's own icon
        // font: same codepoints, different glyphs.
        let faces = fonts::SymbolStyle::Outlined
            .bundled()
            .ok_or("the outlined faces are bundled")?;
        let [_, filled] = faces;
        defs.font_data.insert(
            "app-icons".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(filled)),
        );
        defs.families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "app-icons".to_owned());
    }
    let mut driver = Driver::new();
    driver.ctx.set_fonts(defs);
    let mut app = |ui: &mut egui::Ui| {
        ui.add(MaterialIconButton::standard("home").label("Home"));
    };
    driver.frame(0.0, vec![], &mut app);
    let shapes = driver.frame(0.016, vec![], &mut app);
    let text = texts(&shapes)
        .into_iter()
        .next()
        .ok_or("the icon is a glyph")?;
    let family = text
        .galley
        .job
        .sections
        .first()
        .map(|s| s.format.font_id.family.clone())
        .ok_or("one section")?;
    let uv = text
        .galley
        .rows
        .first()
        .and_then(|r| r.glyphs.first())
        .map(|g| format!("{:?}", g.uv_rect))
        .ok_or("one glyph")?;
    Ok((family, uv))
}

#[test]
fn an_app_icon_font_in_proportional_does_not_change_the_glyph() -> TestResult {
    let (family, plain) = icon_glyph(false)?;
    assert_eq!(
        family,
        fonts::symbol_family(fonts::SymbolStyle::Outlined, false, fonts::SYMBOL_WEIGHT),
        "the glyph is laid out in the outlined symbol family, not Proportional"
    );
    let (family_with_app_font, clashing) = icon_glyph(true)?;
    assert_eq!(
        family_with_app_font, family,
        "same family with the app font"
    );
    assert_eq!(
        clashing, plain,
        "an app font first in Proportional with the same codepoint leaves the glyph alone"
    );
    Ok(())
}
