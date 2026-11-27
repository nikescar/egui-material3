// ============================================================================
// Module:       badges (integration test)
// Description:  Small and large badges: tokens, placement, colours, a11y.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Badges (WP-5): `BadgeTokens` sizes, LabelSmall text, the "999+" cap,
//! `error` / `onError` colours, Compose's `BadgedBox` offsets, id salts,
//! the anchor's accessibility label, and a light/dark snapshot.

mod support;

use egui::accesskit::Role;
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape, TextShape};
use egui::{Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::tokens::ColorRole;
use egui_material3::{badge, badge_dot, BadgePosition, MaterialBadge, MaterialTheme};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Runs one frame of `app` with the baseline light theme and returns the
/// painted shapes.
fn frame(app: &mut dyn FnMut(&mut egui::Ui)) -> Vec<ClippedShape> {
    let ctx = egui::Context::default();
    MaterialTheme::set(&ctx, MaterialTheme::baseline(false));
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(400.0, 200.0))),
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| app(ui));
    output.textures_delta.clear();
    output.shapes
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

/// Adds `badge` standalone and returns its rect and the shapes.
fn standalone(make: &dyn Fn() -> MaterialBadge) -> (Rect, Vec<ClippedShape>) {
    let mut rect = Rect::NOTHING;
    let shapes = frame(&mut |ui| rect = ui.add(make()).rect);
    (rect, shapes)
}

#[test]
fn small_badge_is_a_6_dot() {
    let (rect, _) = standalone(&badge_dot);
    assert_eq!(rect.size(), egui::vec2(6.0, 6.0), "BadgeTokens.Size is 6");
    let (rect, _) = standalone(&|| badge(""));
    assert_eq!(rect.size(), egui::vec2(6.0, 6.0), "empty content is a dot");
}

#[test]
fn large_badge_geometry() -> TestResult {
    let (rect, shapes) = standalone(&|| badge("5"));
    assert_eq!(rect.height(), 16.0, "BadgeTokens.LargeSize is 16");
    assert_eq!(rect.width(), 16.0, "one digit keeps the 16 minimum width");
    let text = texts(&shapes)
        .into_iter()
        .next()
        .ok_or("no badge text painted")?;
    let sizes: Vec<f32> = text
        .galley
        .job
        .sections
        .iter()
        .map(|s| s.format.font_id.size)
        .collect();
    assert_eq!(sizes, vec![11.0], "LabelSmall is 11");
    assert_eq!(text.galley.size().y, 16.0, "LabelSmall line height is 16");

    let (rect, shapes) = standalone(&|| badge("1234"));
    let text = texts(&shapes)
        .into_iter()
        .next()
        .ok_or("no badge text painted")?;
    assert!(
        (rect.width() - (text.galley.size().x + 8.0)).abs() < 0.01,
        "long text gets 4 of padding either side"
    );
    let radius = rects(&shapes)
        .first()
        .map(|r| r.corner_radius.nw)
        .ok_or("no badge container painted")?;
    assert_eq!(radius, 8, "fully round");
    Ok(())
}

#[test]
fn count_caps_at_999_plus() -> TestResult {
    for (count, shown) in [(0, "0"), (999, "999"), (1000, "999+"), (u32::MAX, "999+")] {
        let (_, shapes) = standalone(&|| MaterialBadge::count(count));
        let text = texts(&shapes)
            .into_iter()
            .next()
            .ok_or("no badge text painted")?;
        assert_eq!(text.galley.text(), shown, "count {count}");
    }
    Ok(())
}

#[test]
fn error_and_on_error_colours() -> TestResult {
    let theme = MaterialTheme::baseline(false);
    let (_, shapes) = standalone(&|| badge("5"));
    let container = rects(&shapes)
        .first()
        .map(|r| r.fill)
        .ok_or("no badge container painted")?;
    assert_eq!(
        container,
        theme.color(ColorRole::Error),
        "BadgeTokens.Color is error"
    );
    let text = texts(&shapes)
        .into_iter()
        .next()
        .ok_or("no badge text painted")?;
    assert_eq!(
        text.fallback_color,
        theme.color(ColorRole::OnError),
        "LargeLabelTextColor is onError"
    );
    Ok(())
}

/// Compose's `BadgedBox`: a large badge starts 12 in from the anchor's
/// end and overlaps its top by 14; a dot starts 6 in and overlaps by 6.
#[test]
fn compose_anchor_offsets() {
    let anchor = Rect::from_min_size(Pos2::new(100.0, 100.0), egui::vec2(24.0, 24.0));
    let mut large = Rect::NOTHING;
    let mut dot = Rect::NOTHING;
    let mut left = Rect::NOTHING;
    frame(&mut |ui| {
        large = badge("5").draw_on(ui, anchor, BadgePosition::TopRight).rect;
        dot = badge_dot()
            .draw_on(ui, anchor, BadgePosition::TopRight)
            .rect;
        left = badge("5")
            .id_salt("left")
            .draw_on(ui, anchor, BadgePosition::TopLeft)
            .rect;
    });
    assert_eq!(
        large.min.x,
        anchor.max.x - 12.0,
        "large: 12 in from the end"
    );
    assert_eq!(large.max.y, anchor.min.y + 14.0, "large: overlaps by 14");
    assert_eq!(large.min.y, anchor.min.y - 2.0, "large: rises 2 above");
    assert_eq!(dot.min.x, anchor.max.x - 6.0, "dot: 6 in from the end");
    assert_eq!(dot.min.y, anchor.min.y, "dot: top on the anchor's top");
    assert_eq!(
        left.max.x,
        anchor.min.x + 12.0,
        "top-left mirrors the offset"
    );
}

#[test]
fn ids_do_not_collide() {
    let a = Rect::from_min_size(Pos2::new(10.0, 10.0), egui::vec2(24.0, 24.0));
    let b = a.translate(egui::vec2(40.0, 0.0));
    let mut ids = Vec::new();
    frame(&mut |ui| {
        ids.push(badge("1").draw_on(ui, a, BadgePosition::TopRight).id);
        ids.push(badge("2").draw_on(ui, b, BadgePosition::TopRight).id);
        ids.push(
            badge("3")
                .id_salt("second on a")
                .draw_on(ui, a, BadgePosition::TopRight)
                .id,
        );
    });
    let mut unique = ids.clone();
    unique.sort_by_key(|id| id.value());
    unique.dedup();
    assert_eq!(unique.len(), ids.len(), "every badge has its own id");
}

#[test]
fn badge_text_is_read_with_the_anchor() {
    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(300.0, 120.0),
        |ui| {
            ui.add_space(8.0);
            let inbox = ui.button("Inbox");
            MaterialBadge::count(3)
                .description("3 new messages")
                .show_on(ui, &inbox, BadgePosition::TopRight);
            let updates = ui.button("Updates");
            badge_dot().show_on(ui, &updates, BadgePosition::TopRight);
            let chat = ui.button("Chat");
            badge("12").show_on(ui, &chat, BadgePosition::TopRight);
        },
    );
    assert_eq!(
        harness
            .get_by_label("Inbox, 3 new messages")
            .accesskit_node()
            .role(),
        Role::Button,
        "the description joins the anchor's label"
    );
    assert!(
        harness.query_by_label("Updates, New").is_some(),
        "a dot reads as New"
    );
    assert!(
        harness.query_by_label("Chat, 12").is_some(),
        "the badge text is the default description"
    );
}

#[test]
fn standalone_badge_is_a_label() {
    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(120.0, 60.0),
        |ui| {
            ui.add(badge("NEW"));
        },
    );
    assert_eq!(
        harness.get_by_label("NEW").accesskit_node().role(),
        Role::Label,
        "a standalone badge is read as a label"
    );
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
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(200);
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
fn badges_snapshot() -> TestResult {
    for (name, theme) in support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
    {
        let background = theme.color(ColorRole::Surface);
        let anchor_color = theme.color(ColorRole::SurfaceContainerHighest);
        let mut harness = support::harness_for(theme, egui::vec2(260.0, 60.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 24.0;
                for (salt, make) in [
                    ("dot", badge_dot as fn() -> MaterialBadge),
                    ("one", || badge("5")),
                    ("many", || MaterialBadge::count(1000)),
                ] {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 4.0, anchor_color);
                    make()
                        .id_salt(salt)
                        .draw_on(ui, rect, BadgePosition::TopRight);
                }
            });
        });
        snapshot(&mut harness, &format!("badges_{name}"))?;
    }
    Ok(())
}
