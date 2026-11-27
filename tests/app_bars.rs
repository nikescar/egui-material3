// ============================================================================
// Module:       app_bars (integration test)
// Description:  Top app bars, bottom app bar, docked and floating toolbars.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! App bars and toolbars (WP-9): heights per variant, collapse under
//! scroll, the scrolled container colour, title truncation, the action
//! overflow menu, 40 dp circular state layers, AccessKit roles and labels,
//! keyboard activation, RTL, the floating toolbar's full corner radius, and
//! snapshots in light and dark.

mod support;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use egui::accesskit::{Role, Toggled};
use egui::epaint::{CircleShape, ClippedShape, RectShape, Shape as PaintShape, TextShape};
use egui::{Event, Key, Pos2, Rect, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::{Harness, SnapshotError, SnapshotOptions};
use egui_material3::tokens::{ColorRole, LayoutDirection};
use egui_material3::{
    fonts, scroll_area_id, MaterialBottomAppBar, MaterialFloatingToolbar, MaterialTheme,
    MaterialToolbar, MaterialTopAppBar, TopAppBarScrollBehavior, TopAppBarVariant,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ---------------------------------------------------------------------------
// Helpers

/// Runs `frames` frames of `app` on a fresh context with the light baseline
/// theme (or `theme`), sending `events` in the last one, and returns the
/// last frame's shapes.
fn shapes_of(
    theme: MaterialTheme,
    size: Vec2,
    frames: usize,
    events: Vec<Event>,
    app: &mut dyn FnMut(&mut egui::Ui),
) -> Vec<ClippedShape> {
    let ctx = egui::Context::default();
    MaterialTheme::set(&ctx, theme);
    let mut shapes = Vec::new();
    let mut events = Some(events);
    for frame in 0..frames {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            time: Some(frame as f64 * 0.1),
            events: if frame + 1 == frames {
                events.take().unwrap_or_default()
            } else {
                Vec::new()
            },
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        shapes = output.shapes;
    }
    shapes
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

fn circles(shapes: &[ClippedShape]) -> Vec<CircleShape> {
    shapes
        .iter()
        .filter_map(|c| match &c.shape {
            PaintShape::Circle(circle) => Some(*circle),
            PaintShape::Noop
            | PaintShape::Vec(_)
            | PaintShape::Rect(_)
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
            | PaintShape::Rect(_)
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Band(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => None,
        })
        .collect()
}

/// The rect painted exactly at `rect`.
fn rect_at(shapes: &[ClippedShape], rect: Rect) -> Option<RectShape> {
    rects(shapes)
        .into_iter()
        .find(|r| (r.rect.min - rect.min).length() < 0.5 && (r.rect.max - rect.max).length() < 0.5)
}

fn light() -> MaterialTheme {
    MaterialTheme::baseline(false)
}

fn counter() -> (Arc<AtomicUsize>, impl Fn() + Clone + Send + Sync + 'static) {
    let count = Arc::new(AtomicUsize::new(0));
    let inner = Arc::clone(&count);
    (count, move || {
        inner.fetch_add(1, Ordering::Relaxed);
    })
}

// ---------------------------------------------------------------------------
// Heights

#[test]
fn heights_per_variant() -> TestResult {
    let expected = [
        (TopAppBarVariant::Regular, 64.0, 64.0),
        (TopAppBarVariant::CenterAligned, 64.0, 64.0),
        (TopAppBarVariant::Medium, 112.0, 112.0),
        (TopAppBarVariant::Large, 152.0, 152.0),
        (TopAppBarVariant::MediumFlexible, 112.0, 136.0),
        (TopAppBarVariant::LargeFlexible, 120.0, 152.0),
    ];
    for (variant, plain, with_subtitle) in expected {
        for subtitle in [false, true] {
            let harness = support::harness_for(light(), egui::vec2(600.0, 300.0), |ui| {
                let bar = MaterialTopAppBar::new(variant, "Title");
                let bar = if subtitle { bar.subtitle("Sub") } else { bar };
                ui.add(bar);
            });
            let height = harness.get_by_label("Title").rect().height();
            let want = if subtitle { with_subtitle } else { plain };
            assert_eq!(height, want, "{variant:?}, subtitle {subtitle}: height");
            assert_eq!(
                variant.expanded_height(subtitle),
                want,
                "{variant:?}: expanded_height"
            );
        }
    }
    Ok(())
}

#[test]
fn bottom_app_bar_is_80_with_fab_in_its_slot() -> TestResult {
    let harness = support::harness_for(light(), egui::vec2(400.0, 200.0), |ui| {
        ui.add(
            MaterialBottomAppBar::new()
                .action("check_box", "Select", || {})
                .action("edit", "Edit", || {})
                .fab("add", "New item", || {}),
        );
    });
    let select = harness.get_by_label("Select").rect();
    let fab = harness.get_by_label("New item").rect();
    let bar_top = select.center().y - 40.0;
    assert_eq!(fab.size(), egui::vec2(56.0, 56.0), "FAB is 56 dp");
    assert!(
        (fab.min.y - bar_top - 12.0).abs() < 0.5,
        "FAB sits 4 + 8 dp below the bar's top: {fab:?}, bar top {bar_top}"
    );
    assert_eq!(
        select.size(),
        egui::vec2(48.0, 48.0),
        "action target is 48 dp"
    );
    assert_eq!(
        harness.get_by_label("New item").accesskit_node().role(),
        Role::Button,
        "FAB is a button"
    );

    let mut height = 0.0;
    shapes_of(light(), egui::vec2(400.0, 200.0), 2, vec![], &mut |ui| {
        height = ui
            .add(MaterialBottomAppBar::new().fab("add", "New", || {}))
            .rect
            .height();
    });
    assert_eq!(height, 80.0, "bottom app bar height");
    Ok(())
}

#[test]
fn docked_toolbar_is_64_in_surface_container() -> TestResult {
    let theme = light();
    let want = theme.color(ColorRole::SurfaceContainer);
    let mut rect = Rect::NOTHING;
    let shapes = shapes_of(theme, egui::vec2(400.0, 200.0), 2, vec![], &mut |ui| {
        rect = ui
            .add(MaterialToolbar::new().item(egui::Button::new("Home")))
            .rect;
    });
    assert_eq!(rect.height(), 64.0, "docked toolbar height");
    let container = rect_at(&shapes, rect).ok_or("container painted")?;
    assert_eq!(container.fill, want, "docked toolbar colour");
    Ok(())
}

// ---------------------------------------------------------------------------
// Scrolling

fn inbox(
    behavior: TopAppBarScrollBehavior,
    variant: TopAppBarVariant,
) -> impl FnMut(&mut egui::Ui) {
    move |ui| {
        let content = scroll_area_id(ui, "content");
        ui.add(
            MaterialTopAppBar::new(variant, "Inbox")
                .id_salt("bar")
                .scroll_behavior(behavior)
                .scroll_area(content),
        );
        egui::ScrollArea::vertical()
            .id_salt("content")
            .show(ui, |ui| {
                for i in 0..200 {
                    ui.label(format!("Message {i}"));
                }
            });
    }
}

fn scroll(harness: &mut Harness<'_>, down: bool, times: usize) {
    for _ in 0..times {
        let node = harness.get_by_label("Message 0");
        if down {
            node.scroll_down();
        } else {
            node.scroll_up();
        }
        // The container colour animates for a few frames after each step.
        harness.run_steps(30);
    }
}

#[test]
fn exit_until_collapsed_collapses_152_to_64_and_expands_at_the_top() -> TestResult {
    let mut harness = support::harness_for(
        light(),
        egui::vec2(500.0, 600.0),
        inbox(
            TopAppBarScrollBehavior::ExitUntilCollapsed,
            TopAppBarVariant::Large,
        ),
    );
    let bar = |h: &Harness<'_>| h.get_by_label("Inbox").rect().height();
    assert_eq!(bar(&harness), 152.0, "expanded at rest");

    scroll(&mut harness, true, 3);
    assert_eq!(bar(&harness), 64.0, "collapsed after scrolling down");

    scroll(&mut harness, true, 2);
    scroll(&mut harness, false, 1);
    assert_eq!(bar(&harness), 64.0, "stays collapsed away from the top");

    scroll(&mut harness, false, 10);
    assert_eq!(bar(&harness), 152.0, "expanded again at the top");
    Ok(())
}

#[test]
fn enter_always_hides_a_small_bar_and_brings_it_back_on_scroll_up() -> TestResult {
    let mut harness = support::harness_for(
        light(),
        egui::vec2(500.0, 600.0),
        inbox(
            TopAppBarScrollBehavior::EnterAlways,
            TopAppBarVariant::Regular,
        ),
    );
    let bar = |h: &Harness<'_>| h.get_by_label("Inbox").rect().height();
    assert_eq!(bar(&harness), 64.0, "shown at rest");
    scroll(&mut harness, true, 3);
    assert_eq!(bar(&harness), 0.0, "hidden after scrolling down");
    scroll(&mut harness, false, 1);
    assert_eq!(bar(&harness), 64.0, "back after scrolling up a little");
    Ok(())
}

#[test]
fn pinned_keeps_its_height() -> TestResult {
    let mut harness = support::harness_for(
        light(),
        egui::vec2(500.0, 600.0),
        inbox(TopAppBarScrollBehavior::Pinned, TopAppBarVariant::Large),
    );
    scroll(&mut harness, true, 3);
    assert_eq!(
        harness.get_by_label("Inbox").rect().height(),
        152.0,
        "pinned large bar"
    );
    Ok(())
}

#[test]
fn container_turns_surface_container_when_scrolled_or_collapsed() -> TestResult {
    let theme = light();
    let surface = theme.color(ColorRole::Surface);
    let scrolled = theme.color(ColorRole::SurfaceContainer);
    let cases: [(&str, MaterialTopAppBar<'static>, egui::Color32); 4] = [
        ("at rest", MaterialTopAppBar::regular("Bar"), surface),
        (
            "scrolled",
            MaterialTopAppBar::regular("Bar").scrolled(true),
            scrolled,
        ),
        (
            "content under the bar",
            MaterialTopAppBar::regular("Bar").content_offset(40.0),
            scrolled,
        ),
        (
            "collapsed",
            MaterialTopAppBar::large("Bar")
                .scroll_behavior(TopAppBarScrollBehavior::ExitUntilCollapsed)
                .content_offset(200.0),
            scrolled,
        ),
    ];
    for (name, bar, want) in cases {
        let mut bar = Some(bar);
        let mut rect = Rect::NOTHING;
        let shapes = shapes_of(
            theme.clone(),
            egui::vec2(400.0, 300.0),
            1,
            vec![],
            &mut |ui| {
                if let Some(bar) = bar.take() {
                    rect = ui.add(bar).rect;
                }
            },
        );
        let container = rect_at(&shapes, rect).ok_or(format!("{name}: container painted"))?;
        assert_eq!(container.fill, want, "{name}: container colour");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Title, actions and overflow

#[test]
fn long_title_is_cut_before_the_actions() -> TestResult {
    let title = "A very long title that cannot possibly fit in this narrow bar";
    let mut rect = Rect::NOTHING;
    let shapes = shapes_of(light(), egui::vec2(300.0, 200.0), 2, vec![], &mut |ui| {
        rect = ui
            .add(
                MaterialTopAppBar::regular(title)
                    .navigation("menu", "Menu", || {})
                    .action("search", "Search", || {})
                    .action("share", "Share", || {}),
            )
            .rect;
    });
    let text = texts(&shapes)
        .into_iter()
        .find(|t| t.galley.job.text == title)
        .ok_or("title painted")?;
    assert!(text.galley.elided, "title is elided");
    let actions_start = rect.max.x - 4.0 - 2.0 * 48.0;
    let title_end = text.pos.x + text.galley.size().x;
    assert!(
        title_end <= actions_start + 0.5,
        "title ends at {title_end}, before the actions at {actions_start}"
    );
    assert!(
        text.pos.x >= rect.min.x + 52.0 - 0.5,
        "title starts after the navigation icon"
    );
    Ok(())
}

#[test]
fn actions_are_labelled_buttons_and_extra_ones_overflow_into_a_menu() -> TestResult {
    let (archived, archive) = counter();
    let archive = Arc::new(archive);
    let mut harness = support::harness_for(light(), egui::vec2(600.0, 400.0), {
        let archive = Arc::clone(&archive);
        move |ui| {
            let archive = Arc::clone(&archive);
            ui.add(
                MaterialTopAppBar::regular("Mail")
                    .max_actions(3)
                    .action("search", "Search", || {})
                    .action("share", "Share", || {})
                    .action("delete", "Delete", || {})
                    .action("archive", "Archive", move || archive()),
            );
        }
    });
    for label in ["Search", "Share", "More options"] {
        let node = harness.get_by_label(label);
        assert_eq!(node.accesskit_node().role(), Role::Button, "{label}: role");
        assert_eq!(
            node.rect().size(),
            egui::vec2(48.0, 48.0),
            "{label}: target"
        );
    }
    assert!(
        harness.query_by_label("Delete").is_none() && harness.query_by_label("Archive").is_none(),
        "the extra actions are not on the bar"
    );

    harness.get_by_label("More options").click();
    harness.run();
    assert_eq!(
        harness.get_by_label("Archive").accesskit_node().role(),
        Role::MenuItem,
        "an overflowed action is a menu item"
    );
    assert!(
        harness.query_by_label("Delete").is_some(),
        "Delete in the menu"
    );

    harness.get_by_label("Archive").click();
    harness.run();
    assert_eq!(
        archived.load(Ordering::Relaxed),
        1,
        "the menu item ran its action"
    );
    assert!(
        harness.query_by_label("Archive").is_none(),
        "the menu closed"
    );
    Ok(())
}

#[test]
fn hovered_action_paints_a_40dp_circle() -> TestResult {
    let width = 400.0;
    // The only action's 48 dp target ends 4 dp from the bar's end.
    let center = Pos2::new(width - 4.0 - 24.0, 32.0);
    let origin = std::cell::Cell::new(Pos2::ZERO);
    let mut app = |ui: &mut egui::Ui| {
        origin.set(
            ui.add(MaterialTopAppBar::regular("Bar").action("search", "Search", || {}))
                .rect
                .min,
        );
    };
    // Find the bar's origin first; the panel has a margin.
    shapes_of(light(), egui::vec2(width, 200.0), 1, vec![], &mut app);
    let origin = origin.get();
    let bar_width = width - 2.0 * origin.x;
    let center = Pos2::new(origin.x + bar_width - 4.0 - 24.0, origin.y + center.y);
    let idle = shapes_of(light(), egui::vec2(width, 200.0), 2, vec![], &mut app);
    assert!(
        circles(&idle).is_empty(),
        "no state layer while the pointer is away"
    );
    let hovered = shapes_of(
        light(),
        egui::vec2(width, 200.0),
        3,
        vec![Event::PointerMoved(center)],
        &mut app,
    );
    let layer = circles(&hovered)
        .into_iter()
        .find(|c| (c.center - center).length() < 0.5)
        .ok_or("a circle under the pointer")?;
    assert_eq!(layer.radius, 20.0, "40 dp state layer");
    assert!(layer.fill.a() > 0, "the layer is visible");
    Ok(())
}

#[test]
fn navigation_is_reached_by_tab_and_activated_by_enter() -> TestResult {
    let (count, back) = counter();
    let mut harness = support::harness_for(light(), egui::vec2(400.0, 200.0), move |ui| {
        let back = back.clone();
        ui.add(MaterialTopAppBar::regular("Details").navigation("arrow_back", "Back", back));
    });
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Back").is_focused(),
        "Tab focuses Back"
    );
    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(count.load(Ordering::Relaxed), 1, "Enter activates it");
    harness.get_by_label("Back").click();
    harness.run();
    assert_eq!(count.load(Ordering::Relaxed), 2, "a click activates it");
    Ok(())
}

#[test]
fn rtl_puts_navigation_at_the_right() -> TestResult {
    let theme = MaterialTheme {
        layout_direction: LayoutDirection::Rtl,
        ..light()
    };
    let harness = support::harness_for(theme, egui::vec2(400.0, 200.0), |ui| {
        ui.add(
            MaterialTopAppBar::regular("Bar")
                .navigation("menu", "Menu", || {})
                .action("search", "Search", || {}),
        );
    });
    let bar = harness.get_by_label("Bar").rect();
    let menu = harness.get_by_label("Menu").rect();
    let search = harness.get_by_label("Search").rect();
    assert!(
        (bar.max.x - 4.0 - menu.max.x).abs() < 0.5,
        "navigation 4 dp from the right edge"
    );
    assert!(
        (search.min.x - bar.min.x - 4.0).abs() < 0.5,
        "action 4 dp from the left edge"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Floating toolbar

#[test]
fn floating_toolbar_corner_radius_is_half_its_height() -> TestResult {
    for vertical in [false, true] {
        let mut rect = Rect::NOTHING;
        let shapes = shapes_of(light(), egui::vec2(400.0, 400.0), 2, vec![], &mut |ui| {
            let toolbar = if vertical {
                MaterialFloatingToolbar::vertical()
            } else {
                MaterialFloatingToolbar::horizontal()
            };
            rect = ui
                .add(
                    toolbar
                        .action("format_bold", "Bold", || {})
                        .action("format_italic", "Italic", || {})
                        .action("format_underlined", "Underline", || {}),
                )
                .rect;
        });
        // 8 padding, three 48 dp items with 4 dp gaps, 8 padding.
        let (along, across) = if vertical {
            (rect.height(), rect.width())
        } else {
            (rect.width(), rect.height())
        };
        assert_eq!(across, 64.0, "vertical {vertical}: container size");
        assert_eq!(along, 168.0, "vertical {vertical}: length");
        let container = rect_at(&shapes, rect).ok_or("container painted")?;
        let half = (across / 2.0) as u8;
        let r = container.corner_radius;
        assert!(
            r.nw == half && r.ne == half && r.sw == half && r.se == half,
            "vertical {vertical}: corner radius {r:?} is half of {across}"
        );
    }
    Ok(())
}

#[test]
fn floating_toolbar_toggle_flips_and_reports_its_state() -> TestResult {
    let mut bold = false;
    {
        let mut harness = support::harness_for(light(), egui::vec2(400.0, 200.0), |ui| {
            ui.add(
                MaterialFloatingToolbar::horizontal()
                    .toggle("format_bold", "Bold", &mut bold)
                    .fab("add", "Insert", || {}),
            );
        });
        let toggled = |h: &Harness<'_>| h.get_by_label("Bold").accesskit_node().toggled();
        assert_eq!(toggled(&harness), Some(Toggled::False), "off at first");
        harness.get_by_label("Bold").click();
        harness.run();
        assert_eq!(toggled(&harness), Some(Toggled::True), "on after a click");
        let fab = harness.get_by_label("Insert").rect();
        let bold_rect = harness.get_by_label("Bold").rect();
        assert_eq!(fab.size(), egui::vec2(56.0, 56.0), "FAB is 56 dp");
        assert!(
            (fab.min.x - (bold_rect.max.x + 8.0 + 8.0)).abs() < 0.5,
            "FAB 8 dp after the toolbar's 8 dp padding"
        );
    }
    assert!(bold, "the bound bool is on");
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
    render: impl FnOnce() -> Result<T, SnapshotError>,
) -> Result<T, SnapshotError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(render)).unwrap_or_else(|_| {
        Err(SnapshotError::RenderError {
            err: "no adapter: the renderer could not be created".to_owned(),
        })
    })
}

fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    // Software and hardware rasterisers differ in anti-aliasing.
    let options = SnapshotOptions::new().threshold(2.0).max_failed_pixels(600);
    match no_adapter_is_an_error(|| harness.try_snapshot_options(name, &options)) {
        Ok(()) => Ok(()),
        Err(SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

/// A harness with the bundled fonts, so icons and titles render.
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

fn background(ui: &egui::Ui) {
    let window = Rect::from_min_size(egui::pos2(-100.0, -100.0), egui::vec2(4000.0, 4000.0));
    let color = MaterialTheme::of(ui.ctx()).color(ColorRole::SurfaceContainerHighest);
    ui.painter()
        .with_clip_rect(window)
        .rect_filled(window, 0.0, color);
}

#[test]
fn top_app_bars_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let mut harness = font_harness(theme, egui::vec2(420.0, 940.0), |ui| {
            background(ui);
            let bar = |variant| {
                MaterialTopAppBar::new(variant, "Title")
                    .navigation("arrow_back", "Back", || {})
                    .action("attach_file", "Attach", || {})
                    .action("more_vert", "More", || {})
            };
            ui.add(bar(TopAppBarVariant::Regular));
            ui.add(bar(TopAppBarVariant::CenterAligned).subtitle("Subtitle"));
            ui.add(bar(TopAppBarVariant::Regular).scrolled(true));
            ui.add(bar(TopAppBarVariant::Medium));
            ui.add(bar(TopAppBarVariant::Large));
            ui.add(bar(TopAppBarVariant::MediumFlexible).subtitle("Subtitle"));
            ui.add(bar(TopAppBarVariant::LargeFlexible).subtitle("Subtitle"));
            ui.add(
                MaterialTopAppBar::regular("A long title that is cut with an ellipsis")
                    .navigation("menu", "Menu", || {})
                    .action("search", "Search", || {}),
            );
        });
        snapshot(&mut harness, &format!("top_app_bars_{name}"))?;
    }
    Ok(())
}

#[test]
fn toolbars_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let mut harness = font_harness(theme, egui::vec2(420.0, 520.0), |ui| {
            background(ui);
            ui.add(
                MaterialBottomAppBar::new()
                    .action("check_box", "Select", || {})
                    .action("edit", "Edit", || {})
                    .action("mic", "Voice", || {})
                    .action("image", "Image", || {})
                    .fab("add", "New", || {}),
            );
            ui.add_space(8.0);
            ui.add(
                MaterialToolbar::new()
                    .item(egui::Label::new("Docked"))
                    .spacer()
                    .item_fn(|ui| ui.label("End")),
            );
            ui.add_space(8.0);
            let mut on = true;
            let mut off = false;
            ui.add(
                MaterialFloatingToolbar::horizontal()
                    .toggle("format_bold", "Bold", &mut on)
                    .action("format_italic", "Italic", || {})
                    .action("format_underlined", "Underline", || {})
                    .fab("add", "Insert", || {}),
            );
            ui.add_space(8.0);
            ui.add(
                MaterialFloatingToolbar::horizontal()
                    .style(egui_material3::FloatingToolbarStyle::Vibrant)
                    .toggle("format_bold", "Bold", &mut off)
                    .action("format_italic", "Italic", || {})
                    .fab("add", "Insert", || {}),
            );
            ui.add_space(8.0);
            ui.add(
                MaterialFloatingToolbar::vertical()
                    .action("undo", "Undo", || {})
                    .action("redo", "Redo", || {})
                    .fab("edit", "Edit", || {}),
            );
        });
        snapshot(&mut harness, &format!("toolbars_{name}"))?;
    }
    Ok(())
}
