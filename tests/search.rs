// ============================================================================
// Module:       search (integration test)
// Description:  Search bar and search view: tokens, expand/collapse, keyboard.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! The search bar and view (WP-16): bar geometry and colour from
//! `SearchBarTokens`, focus expanding into the view, Escape collapsing it,
//! keyboard navigation of the suggestions, docked and full-screen layouts,
//! and snapshots in light and dark.

mod support;

use std::cell::{Cell, RefCell};

use egui::accesskit::Role;
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape};
use egui::{Color32, Key, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use egui_material3::search::{self, MaterialSearch, SearchViewMode, Suggestion};
use egui_material3::tokens::ColorRole;
use egui_material3::MaterialTheme;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const ID: &str = "test_search";
const FRUIT: [&str; 4] = ["Apple", "Apricot", "Banana", "Cherry"];
const SIZE: egui::Vec2 = egui::vec2(480.0, 480.0);

/// What the search reported, across frames.
#[derive(Default)]
struct Log {
    query: RefCell<String>,
    selected: Cell<Option<usize>>,
    submitted: RefCell<Option<String>>,
}

impl Log {
    fn draw(&self, ui: &mut egui::Ui, mode: SearchViewMode) {
        let mut query = self.query.borrow_mut();
        let out = MaterialSearch::new(ID, &mut query)
            .placeholder("Search fruit")
            .mode(mode)
            .suggestions(FRUIT)
            .show(ui);
        if out.selected.is_some() {
            self.selected.set(out.selected);
        }
        if out.submitted.is_some() {
            *self.submitted.borrow_mut() = out.submitted;
        }
    }
}

fn harness<'a>(log: &'a Log, mode: SearchViewMode, theme: MaterialTheme) -> Harness<'a> {
    support::harness_for(theme, SIZE, move |ui| log.draw(ui, mode))
}

fn light() -> MaterialTheme {
    MaterialTheme::baseline(false)
}

/// Focuses the search field and lets the view open.
fn focus_field(harness: &mut Harness<'_>) -> TestResult {
    harness
        .query_by_role(Role::SearchInput)
        .ok_or("the search field is a SearchInput node")?
        .focus();
    harness.run();
    Ok(())
}

fn options(harness: &Harness<'_>) -> Vec<(String, bool)> {
    harness
        .query_all_by_role(Role::ListBoxOption)
        .map(|n| {
            let node = n.accesskit_node();
            (
                node.label().unwrap_or_default(),
                node.is_selected().unwrap_or(false),
            )
        })
        .collect()
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

#[test]
fn bar_is_56_high_full_rounded_surface_container_high() -> TestResult {
    let ctx = egui::Context::default();
    let theme = light();
    let container = theme.color(ColorRole::SurfaceContainerHigh);
    MaterialTheme::set(&ctx, theme);
    let mut query = String::new();
    let mut shapes = Vec::new();
    for _ in 0..2 {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SIZE)),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            MaterialSearch::new(ID, &mut query).width(400.0).show(ui);
        });
        output.textures_delta.clear();
        shapes = output.shapes;
    }
    let bar = rects(&shapes)
        .into_iter()
        .find(|r| r.fill == container)
        .ok_or("the bar paints a surfaceContainerHigh container")?;
    assert_eq!(
        bar.rect.height(),
        search::BAR_HEIGHT,
        "SearchBarTokens.ContainerHeight"
    );
    assert_eq!(bar.rect.width(), 400.0, "the requested width");
    assert_eq!(bar.corner_radius.nw, 28, "CornerFull: half the height");
    assert_eq!(bar.corner_radius.se, 28, "CornerFull on every corner");
    Ok(())
}

#[test]
fn bar_reports_search_roles() -> TestResult {
    let log = Log::default();
    let harness = harness(&log, SearchViewMode::Docked, light());
    let field = harness
        .query_by_role(Role::SearchInput)
        .ok_or("a SearchInput node")?;
    assert_eq!(
        field.accesskit_node().label().as_deref(),
        Some("Search fruit"),
        "the placeholder names the field"
    );
    assert!(
        harness.query_by_role(Role::Search).is_some(),
        "the bar is a search landmark"
    );
    assert!(
        harness.query_by_role(Role::ListBox).is_none(),
        "no suggestion list while collapsed"
    );
    Ok(())
}

#[test]
fn focus_expands_into_the_view_and_escape_collapses_it() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    focus_field(&mut harness)?;
    assert!(
        search::is_expanded(&harness.ctx, ID),
        "focus expands the bar"
    );
    assert!(
        harness.query_by_role(Role::ListBox).is_some(),
        "the expanded view shows the suggestion list"
    );
    assert_eq!(
        options(&harness).len(),
        FRUIT.len(),
        "one row per suggestion"
    );
    assert!(
        harness
            .query_by_role(Role::SearchInput)
            .is_some_and(|n| n.is_focused()),
        "the field keeps focus in the view"
    );

    harness.key_press(Key::Escape);
    harness.run();
    assert!(!search::is_expanded(&harness.ctx, ID), "Escape collapses");
    assert!(
        harness.query_by_role(Role::ListBox).is_none(),
        "the view is gone after Escape"
    );
    Ok(())
}

#[test]
fn tab_into_the_field_expands_it() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        search::is_expanded(&harness.ctx, ID),
        "Tab focuses and expands"
    );
    Ok(())
}

#[test]
fn arrows_move_the_highlight_and_enter_picks_it() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    focus_field(&mut harness)?;
    assert!(
        options(&harness).iter().all(|(_, selected)| !selected),
        "nothing highlighted at first"
    );

    harness.key_press(Key::ArrowDown);
    harness.run();
    harness.key_press(Key::ArrowDown);
    harness.run();
    let highlighted: Vec<String> = options(&harness)
        .into_iter()
        .filter(|(_, s)| *s)
        .map(|(label, _)| label)
        .collect();
    assert_eq!(
        highlighted,
        ["Apricot"],
        "two Downs highlight the second row"
    );

    harness.key_press(Key::ArrowUp);
    harness.run();
    let highlighted: Vec<String> = options(&harness)
        .into_iter()
        .filter(|(_, s)| *s)
        .map(|(label, _)| label)
        .collect();
    assert_eq!(highlighted, ["Apple"], "Up moves back to the first row");

    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(log.selected.get(), Some(0), "Enter picks the highlight");
    assert_eq!(
        log.query.borrow().as_str(),
        "Apple",
        "the query takes its text"
    );
    assert!(!search::is_expanded(&harness.ctx, ID), "picking collapses");
    Ok(())
}

#[test]
fn down_stops_at_the_last_suggestion() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    focus_field(&mut harness)?;
    for _ in 0..(FRUIT.len() + 3) {
        harness.key_press(Key::ArrowDown);
        harness.run();
    }
    let last = options(&harness).into_iter().filter(|(_, s)| *s).count();
    assert_eq!(last, 1, "exactly one row highlighted");
    assert!(
        options(&harness)
            .last()
            .is_some_and(|(label, s)| *s && label == "Cherry"),
        "the highlight stays on the last row"
    );
    Ok(())
}

#[test]
fn enter_without_highlight_submits_the_query() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    focus_field(&mut harness)?;
    harness
        .query_by_role(Role::SearchInput)
        .ok_or("a SearchInput node")?
        .type_text("ban");
    harness.run();
    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(
        log.submitted.borrow().as_deref(),
        Some("ban"),
        "Enter submits what was typed"
    );
    assert_eq!(log.selected.get(), None, "no suggestion picked");
    assert!(
        !search::is_expanded(&harness.ctx, ID),
        "submitting collapses"
    );
    Ok(())
}

#[test]
fn clicking_a_suggestion_picks_it() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    focus_field(&mut harness)?;
    harness
        .query_by_role_and_label(Role::ListBoxOption, "Banana")
        .ok_or("a Banana row")?
        .click();
    harness.run();
    assert_eq!(log.selected.get(), Some(2), "the clicked row is picked");
    assert_eq!(
        log.query.borrow().as_str(),
        "Banana",
        "the query takes its text"
    );
    assert!(!search::is_expanded(&harness.ctx, ID), "picking collapses");
    Ok(())
}

#[test]
fn back_button_and_outside_click_collapse() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    focus_field(&mut harness)?;
    harness
        .query_by_role_and_label(Role::Button, "Back")
        .ok_or("a Back button in the view")?
        .click();
    harness.run();
    assert!(!search::is_expanded(&harness.ctx, ID), "Back collapses");

    focus_field(&mut harness)?;
    assert!(search::is_expanded(&harness.ctx, ID), "expanded again");
    harness.hover_at(Pos2::new(470.0, 470.0));
    harness.run();
    harness.event(egui::Event::PointerButton {
        pos: Pos2::new(470.0, 470.0),
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    });
    harness.event(egui::Event::PointerButton {
        pos: Pos2::new(470.0, 470.0),
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    });
    harness.run();
    assert!(
        !search::is_expanded(&harness.ctx, ID),
        "a click outside the docked view collapses it"
    );
    Ok(())
}

#[test]
fn docked_view_sits_on_the_bar_and_full_screen_covers_the_window() -> TestResult {
    let log = Log::default();
    let mut docked = harness(&log, SearchViewMode::Docked, light());
    let bar = docked.query_by_role(Role::Search).ok_or("the bar")?.rect();
    focus_field(&mut docked)?;
    let list = docked
        .query_by_role(Role::ListBox)
        .ok_or("the docked list")?
        .rect();
    assert_eq!(
        list.width(),
        bar.width(),
        "the docked view is the bar's width"
    );
    assert!(
        (list.top() - (bar.top() + search::BAR_HEIGHT + 1.0)).abs() < 0.5,
        "results start under the 56 dp header and its divider: {list:?} vs {bar:?}"
    );
    assert!(
        list.height() >= 240.0,
        "DockedExpandedTableMinHeight: {}",
        list.height()
    );
    drop(docked);

    let log = Log::default();
    let mut full = harness(&log, SearchViewMode::FullScreen, light());
    focus_field(&mut full)?;
    let list = full
        .query_by_role(Role::ListBox)
        .ok_or("the full-screen list")?
        .rect();
    assert_eq!(list.width(), SIZE.x, "full screen spans the window");
    assert!(
        (list.top() - (search::FULL_SCREEN_HEADER_HEIGHT + 1.0)).abs() < 0.5,
        "results start under the 72 dp header: {list:?}"
    );
    assert!(
        (list.bottom() - SIZE.y).abs() < 0.5,
        "results reach the bottom of the window: {list:?}"
    );
    Ok(())
}

#[test]
fn suggestion_rows_follow_list_item_heights() -> TestResult {
    let query = RefCell::new(String::new());
    let mut harness = support::harness_for(light(), SIZE, |ui| {
        let mut query = query.borrow_mut();
        MaterialSearch::new(ID, &mut query)
            .suggestions([
                Suggestion::new("One line").icon("history"),
                Suggestion::new("Two lines").supporting_text("Supporting text"),
            ])
            .show(ui);
    });
    focus_field(&mut harness)?;
    let one = harness
        .query_by_role_and_label(Role::ListBoxOption, "One line")
        .ok_or("one-line row")?
        .rect();
    let two = harness
        .query_by_role_and_label(Role::ListBoxOption, "Two lines")
        .ok_or("two-line row")?
        .rect();
    assert_eq!(one.height(), 56.0, "ListTokens.ItemOneLineContainerHeight");
    assert_eq!(two.height(), 72.0, "ListTokens.ItemTwoLineContainerHeight");
    Ok(())
}

#[test]
fn hover_paints_a_state_layer_on_the_bar() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::Docked, light());
    let container: Color32 = light().color(ColorRole::SurfaceContainerHigh);
    let bar = harness.query_by_role(Role::Search).ok_or("the bar")?.rect();
    let count_layers = |harness: &mut Harness<'_>| {
        let output = harness.output().clone();
        rects(&output.shapes)
            .into_iter()
            .filter(|r| r.rect == bar && r.fill != container && r.fill.a() > 0 && r.fill.a() < 255)
            .map(|r| r.fill)
            .collect::<Vec<_>>()
    };
    let idle = count_layers(&mut harness);
    harness.hover_at(bar.left_center() + egui::vec2(8.0, 0.0));
    harness.run();
    let hovered = count_layers(&mut harness);
    assert!(idle.is_empty(), "no state layer while idle: {idle:?}");
    assert_eq!(
        hovered.len(),
        1,
        "hover paints one translucent layer over the bar"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshots

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

/// Rendering tests take this lock: several wgpu renderers created at once
/// in one process can crash the test binary on Windows.
fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    // Software and hardware rasterisers differ in anti-aliasing; allow that,
    // not a missing or moved bar.
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

fn light_and_dark() -> Vec<(&'static str, MaterialTheme)> {
    support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
        .collect()
}

#[test]
fn snapshots_collapsed_and_docked() -> TestResult {
    for (name, theme) in light_and_dark() {
        let log = Log::default();
        let mut harness = harness(&log, SearchViewMode::Docked, theme);
        snapshot(&mut harness, &format!("search_bar_{name}"))?;
        focus_field(&mut harness)?;
        harness.key_press(Key::ArrowDown);
        harness.run();
        snapshot(&mut harness, &format!("search_docked_{name}"))?;
    }
    Ok(())
}

#[test]
fn snapshot_full_screen() -> TestResult {
    let log = Log::default();
    let mut harness = harness(&log, SearchViewMode::FullScreen, light());
    focus_field(&mut harness)?;
    snapshot(&mut harness, "search_full_screen_light")
}
