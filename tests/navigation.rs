// ============================================================================
// Module:       navigation (integration test)
// Description:  Navigation bar, navigation rail, drawer and tabs.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Navigation bar, rail, drawer and tabs (WP-10): token geometry and
//! colours, the modal scrim blocking clicks, arrow-key roving, AccessKit
//! `selected`, per-tab disabled state, tab layouts, and snapshots.

mod support;

use std::cell::Cell;

use egui::accesskit::Role;
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape, TextShape};
use egui::{Color32, Event, Pos2, Rect, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::tokens::{ColorRole, LayoutDirection, TypeRole};
use egui_material3::{
    DrawerAlignment, DrawerItem, DrawerVariant, MaterialDrawer, MaterialTabs, MaterialTheme,
    NavigationBadge, NavigationBar, NavigationBarVariant, NavigationItem, NavigationRail, TabItem,
    TabLayout,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ---------------------------------------------------------------------------
// A raw-context driver: exact control of the screen and pointer, and access
// to the painted shapes.

struct Driver {
    ctx: egui::Context,
    size: Vec2,
}

impl Driver {
    fn new(theme: MaterialTheme, size: Vec2) -> Self {
        let ctx = egui::Context::default();
        MaterialTheme::set(&ctx, theme);
        Self { ctx, size }
    }

    fn frame(&self, events: Vec<Event>, app: &mut dyn FnMut(&mut egui::Ui)) -> Vec<ClippedShape> {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
            events,
            ..Default::default()
        };
        let mut output = self.ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        output.shapes
    }

    /// A click at `pos` over three frames (move and press, release, settle).
    fn click(&self, pos: Pos2, app: &mut dyn FnMut(&mut egui::Ui)) {
        let button = |pressed| Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![Event::PointerMoved(pos), button(true)], app);
        self.frame(vec![button(false)], app);
        self.frame(Vec::new(), app);
    }
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

/// The rects painted in exactly `color`.
fn filled_with(shapes: &[ClippedShape], color: Color32) -> Vec<Rect> {
    rects(shapes)
        .into_iter()
        .filter(|r| r.fill == color)
        .map(|r| r.rect)
        .collect()
}

fn light() -> MaterialTheme {
    MaterialTheme::baseline(false)
}

fn rtl() -> MaterialTheme {
    MaterialTheme {
        layout_direction: LayoutDirection::Rtl,
        ..MaterialTheme::baseline(false)
    }
}

fn three_items() -> [NavigationItem; 3] {
    [
        NavigationItem::new("home", "Home"),
        NavigationItem::new("mail", "Mail").badge(NavigationBadge::Large("3".into())),
        NavigationItem::new("settings", "Settings"),
    ]
}

// ---------------------------------------------------------------------------
// Navigation bar

#[test]
fn navigation_bar_heights_and_indicator_match_tokens() {
    let theme = light();
    let indicator = theme.color(ColorRole::SecondaryContainer);
    let container = theme.color(ColorRole::SurfaceContainer);
    for (variant, height, indicator_size) in [
        // NavigationBarTokens.TallContainerHeight; vertical item 56 x 32.
        (NavigationBarVariant::Baseline, 80.0, Vec2::new(56.0, 32.0)),
        // NavigationBarTokens.ContainerHeight.
        (NavigationBarVariant::Flexible, 64.0, Vec2::new(56.0, 32.0)),
    ] {
        let driver = Driver::new(theme.clone(), Vec2::new(600.0, 300.0));
        let mut selected = 1;
        let mut bar_rect = Rect::NOTHING;
        let shapes = driver.frame(Vec::new(), &mut |ui| {
            let mut bar = NavigationBar::new(&mut selected).variant(variant);
            for item in three_items() {
                bar = bar.item(item);
            }
            bar_rect = ui.add(bar).rect;
        });
        assert_eq!(bar_rect.height(), height, "{variant:?}: bar height");
        assert!(
            filled_with(&shapes, container).contains(&bar_rect),
            "{variant:?}: surfaceContainer bar"
        );
        let pills = filled_with(&shapes, indicator);
        assert_eq!(pills.len(), 1, "{variant:?}: one indicator");
        let pill = pills.first().copied().unwrap_or(Rect::NOTHING);
        assert_eq!(pill.size(), indicator_size, "{variant:?}: indicator size");
        assert!(
            (pill.center().x - 300.0).abs() < 0.5,
            "{variant:?}: the middle item's indicator is centred in it"
        );
    }
}

#[test]
fn navigation_bar_horizontal_items_have_a_40_dp_indicator() {
    let theme = light();
    let driver = Driver::new(theme.clone(), Vec2::new(900.0, 200.0));
    let mut selected = 0;
    let shapes = driver.frame(Vec::new(), &mut |ui| {
        let mut bar =
            NavigationBar::new(&mut selected).variant(NavigationBarVariant::FlexibleHorizontal);
        for item in three_items() {
            bar = bar.item(item);
        }
        ui.add(bar);
    });
    let pills = filled_with(&shapes, theme.color(ColorRole::SecondaryContainer));
    let pill = pills.first().copied().unwrap_or(Rect::NOTHING);
    // NavigationBarHorizontalItemTokens.ActiveIndicatorHeight.
    assert_eq!(pill.height(), 40.0, "horizontal indicator height");
    assert!(
        pill.width() > 16.0 + 24.0 + 4.0 + 16.0,
        "the indicator hugs icon and label with 16 dp each side"
    );
}

#[test]
fn navigation_bar_selects_on_click_reports_selected_and_roves() {
    let selected = Cell::new(0);
    let mut harness = support::harness_for(light(), Vec2::new(600.0, 120.0), |ui| {
        let mut index = selected.get();
        let mut bar = NavigationBar::new(&mut index);
        for item in three_items() {
            bar = bar.item(item);
        }
        ui.add(bar);
        selected.set(index);
    });
    harness.get_by_label("Settings").click();
    harness.run();
    assert_eq!(selected.get(), 2, "clicking an item selects it");
    let node = harness.get_by_label("Settings");
    assert_eq!(node.accesskit_node().role(), Role::Tab, "items are tabs");
    assert_eq!(
        node.accesskit_node().is_selected(),
        Some(true),
        "the selected item reports selected"
    );
    assert_eq!(
        harness.get_by_label("Home").accesskit_node().is_selected(),
        Some(false),
        "the others report not selected"
    );

    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Home").is_focused(),
        "Tab enters the bar"
    );
    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    assert!(
        harness.get_by_label("Mail").is_focused(),
        "Right moves to the next item"
    );
    harness.key_press(egui::Key::End);
    harness.run();
    assert!(
        harness.get_by_label("Settings").is_focused(),
        "End moves to the last item"
    );
    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    assert!(harness.get_by_label("Home").is_focused(), "Right wraps");
}

#[test]
fn navigation_bar_mirrors_in_right_to_left() {
    let driver = Driver::new(rtl(), Vec2::new(600.0, 200.0));
    let mut selected = 0;
    let shapes = driver.frame(Vec::new(), &mut |ui| {
        let mut bar = NavigationBar::new(&mut selected);
        for item in three_items() {
            bar = bar.item(item);
        }
        ui.add(bar);
    });
    let pills = filled_with(&shapes, rtl().color(ColorRole::SecondaryContainer));
    let pill = pills.first().copied().unwrap_or(Rect::NOTHING);
    assert!(
        (pill.center().x - 500.0).abs() < 0.5,
        "the first item is on the right"
    );
}

// ---------------------------------------------------------------------------
// Navigation rail

#[test]
fn navigation_rail_widths_and_indicators_match_tokens() {
    let theme = light();
    let indicator = theme.color(ColorRole::SecondaryContainer);
    for (expanded, width, indicator_height) in [(false, 96.0, 32.0), (true, 220.0, 56.0)] {
        let driver = Driver::new(theme.clone(), Vec2::new(800.0, 600.0));
        let mut selected = 0;
        let mut open = expanded;
        let mut rail_rect = Rect::NOTHING;
        let shapes = driver.frame(Vec::new(), &mut |ui| {
            let mut rail = NavigationRail::new(&mut selected).expanded(&mut open);
            for item in three_items() {
                rail = rail.item(item);
            }
            rail_rect = ui.add(rail).rect;
        });
        // NavigationRailCollapsedTokens.ContainerWidth / ExpandedTokens.ContainerWidthMinimum.
        assert_eq!(rail_rect.width(), width, "expanded={expanded}: rail width");
        assert!(
            filled_with(&shapes, theme.color(ColorRole::Surface)).contains(&rail_rect),
            "expanded={expanded}: surface rail"
        );
        let pills = filled_with(&shapes, indicator);
        let pill = pills.first().copied().unwrap_or(Rect::NOTHING);
        assert_eq!(
            pill.height(),
            indicator_height,
            "expanded={expanded}: indicator height"
        );
        if !expanded {
            assert_eq!(pill.width(), 56.0, "collapsed indicator width");
            // NavigationRailCollapsedTokens.TopSpace: items start 44 dp down.
            assert!(pill.min.y >= rail_rect.min.y + 44.0, "top space");
        }
    }
}

#[test]
fn navigation_rail_roves_with_up_and_down() {
    let mut harness = support::harness_for(light(), Vec2::new(300.0, 500.0), |ui| {
        let mut selected = 0;
        let mut rail = NavigationRail::new(&mut selected);
        for item in three_items() {
            rail = rail.item(item);
        }
        ui.add(rail);
    });
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Home").is_focused(),
        "Tab enters the rail"
    );
    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    assert!(harness.get_by_label("Mail").is_focused(), "Down moves on");
    harness.key_press(egui::Key::ArrowUp);
    harness.key_press(egui::Key::ArrowUp);
    harness.run();
    assert!(harness.get_by_label("Settings").is_focused(), "Up wraps");
    assert_eq!(
        harness.get_by_label("Home").accesskit_node().is_selected(),
        Some(true),
        "selected item"
    );
}

#[test]
fn modal_rail_scrim_blocks_the_page_and_collapses() {
    let driver = Driver::new(light(), Vec2::new(800.0, 600.0));
    let mut selected = 0;
    let mut expanded = true;
    let mut page_clicks = 0;
    let mut app = |ui: &mut egui::Ui| {
        ui.horizontal(|ui| {
            let mut rail = NavigationRail::new(&mut selected)
                .expanded(&mut expanded)
                .modal(true);
            for item in three_items() {
                rail = rail.item(item);
            }
            let rail_rect = ui.add(rail).rect;
            assert_eq!(
                rail_rect.width(),
                96.0,
                "a modal rail stays collapsed in layout"
            );
            if ui
                .add_sized(Vec2::new(600.0, 500.0), egui::Button::new("page"))
                .clicked()
            {
                page_clicks += 1;
            }
        });
    };
    // Two frames: a new area is invisible on its first frame.
    driver.frame(Vec::new(), &mut app);
    let shapes = driver.frame(Vec::new(), &mut app);
    let theme = light();
    let scrim = theme
        .color(ColorRole::Scrim)
        .gamma_multiply(egui_material3::navigation::SCRIM_OPACITY);
    assert!(
        !filled_with(&shapes, scrim).is_empty(),
        "a 32 % scrim covers the page"
    );
    assert!(
        filled_with(&shapes, theme.color(ColorRole::SurfaceContainer))
            .iter()
            .any(|r| r.width() == 220.0),
        "the modal rail is a 220 dp surfaceContainer sheet"
    );
    driver.click(Pos2::new(500.0, 300.0), &mut app);
    drop(app);
    assert_eq!(page_clicks, 0, "the page under the scrim gets no click");
    assert!(!expanded, "clicking the scrim collapses the rail");
}

// ---------------------------------------------------------------------------
// Drawer

fn drawer_items() -> Vec<DrawerItem> {
    vec![
        DrawerItem::new("Inbox")
            .icon("inbox")
            .active(true)
            .badge("24"),
        DrawerItem::new("Outbox").icon("send"),
        DrawerItem::new("Spam").icon("report").enabled(false),
    ]
}

#[test]
fn drawer_containers_and_indicator_match_tokens() {
    let theme = light();
    for (variant, container) in [
        (DrawerVariant::Modal, ColorRole::SurfaceContainerLow),
        (DrawerVariant::Permanent, ColorRole::Surface),
    ] {
        let driver = Driver::new(theme.clone(), Vec2::new(1000.0, 700.0));
        let mut open = true;
        let mut app = |ui: &mut egui::Ui| {
            let mut drawer = MaterialDrawer::new(variant, &mut open);
            for item in drawer_items() {
                drawer = drawer.add_item(item);
            }
            drawer.show(ui);
        };
        driver.frame(Vec::new(), &mut app);
        let shapes = driver.frame(Vec::new(), &mut app);
        let sheets: Vec<RectShape> = rects(&shapes)
            .into_iter()
            .filter(|r| r.fill == theme.color(container) && r.rect.width() == 360.0)
            .collect();
        assert_eq!(
            sheets.len(),
            1,
            "{variant:?}: one 360 dp {container:?} sheet"
        );
        let corners = sheets.first().map(|s| s.corner_radius).unwrap_or_default();
        match variant {
            DrawerVariant::Modal => assert_eq!(
                (corners.nw, corners.ne, corners.sw, corners.se),
                (0, 16, 0, 16),
                "modal: CornerLargeEnd"
            ),
            DrawerVariant::Permanent | DrawerVariant::Dismissible => {
                assert_eq!(corners.ne, 0, "standard: square")
            }
        }
        let pills = filled_with(&shapes, theme.color(ColorRole::SecondaryContainer));
        let pill = pills.first().copied().unwrap_or(Rect::NOTHING);
        // NavigationDrawerTokens.ActiveIndicatorWidth x ActiveIndicatorHeight.
        assert_eq!(
            pill.size(),
            Vec2::new(336.0, 56.0),
            "{variant:?}: indicator"
        );
        assert!(
            sheets
                .first()
                .is_some_and(|s| (pill.min.x - s.rect.min.x - 12.0).abs() < 0.5),
            "{variant:?}: 12 dp in from the sheet"
        );
    }
}

#[test]
fn modal_drawer_scrim_is_32_percent_and_blocks_clicks() {
    let theme = light();
    let driver = Driver::new(theme.clone(), Vec2::new(800.0, 600.0));
    let mut open = true;
    let mut page_clicks = 0;
    let mut app = |ui: &mut egui::Ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let size = ui.available_size();
            if ui.add_sized(size, egui::Button::new("page")).clicked() {
                page_clicks += 1;
            }
        });
        MaterialDrawer::new(DrawerVariant::Modal, &mut open)
            .width(200.0)
            .show(ui);
    };
    driver.frame(Vec::new(), &mut app);
    let shapes = driver.frame(Vec::new(), &mut app);
    let scrim = theme
        .color(ColorRole::Scrim)
        .gamma_multiply(egui_material3::navigation::SCRIM_OPACITY);
    assert!(
        filled_with(&shapes, scrim)
            .iter()
            .any(|r| r.size() == Vec2::new(800.0, 600.0)),
        "the scrim is scrim @ 32 % over the whole page"
    );
    driver.click(Pos2::new(500.0, 300.0), &mut app);
    drop(app);
    assert_eq!(page_clicks, 0, "the click does not reach the page");
    assert!(!open, "clicking the scrim dismisses the drawer");
}

#[test]
fn drawer_end_alignment_and_rtl_put_it_on_the_right() {
    for (theme, alignment, on_right) in [
        (light(), DrawerAlignment::Start, false),
        (light(), DrawerAlignment::End, true),
        (rtl(), DrawerAlignment::Start, true),
        (rtl(), DrawerAlignment::End, false),
    ] {
        let container = theme.color(ColorRole::SurfaceContainerLow);
        let driver = Driver::new(theme, Vec2::new(800.0, 600.0));
        let mut open = true;
        let mut app = |ui: &mut egui::Ui| {
            MaterialDrawer::new(DrawerVariant::Modal, &mut open)
                .alignment(alignment)
                .show(ui);
        };
        driver.frame(Vec::new(), &mut app);
        let shapes = driver.frame(Vec::new(), &mut app);
        let sheet = rects(&shapes)
            .into_iter()
            .find(|r| r.fill == container && r.rect.width() == 360.0);
        let expected_x = if on_right { 440.0 } else { 0.0 };
        assert!(
            sheet.as_ref().is_some_and(|s| s.rect.min.x == expected_x),
            "{alignment:?}: sheet at x = {expected_x}"
        );
        // The rounded corners face the page.
        let corners = sheet.map(|s| s.corner_radius).unwrap_or_default();
        let rounded_right = corners.ne > 0 && corners.nw == 0;
        assert_eq!(
            rounded_right, !on_right,
            "{alignment:?}: corners face the page"
        );
    }
}

#[test]
fn drawer_items_report_selected_and_rove() {
    let mut harness = support::harness_for(light(), Vec2::new(500.0, 400.0), |ui| {
        let mut open = true;
        let mut drawer = MaterialDrawer::new(DrawerVariant::Permanent, &mut open);
        for item in drawer_items() {
            drawer = drawer.add_item(item);
        }
        drawer.show(ui);
    });
    let inbox = harness.get_by_label("Inbox");
    assert_eq!(
        inbox.accesskit_node().role(),
        Role::Tab,
        "drawer items are tabs"
    );
    assert_eq!(
        inbox.accesskit_node().is_selected(),
        Some(true),
        "active item"
    );
    assert!(
        harness.get_by_label("Spam").accesskit_node().is_disabled(),
        "disabled item"
    );
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(harness.get_by_label("Inbox").is_focused(), "Tab enters");
    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    assert!(harness.get_by_label("Outbox").is_focused(), "Down moves");
    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    assert!(
        harness.get_by_label("Inbox").is_focused(),
        "Down skips the disabled item and wraps"
    );
}

// ---------------------------------------------------------------------------
// Tabs

#[test]
fn right_arrow_moves_tab_focus_skipping_disabled_tabs() {
    let selected = Cell::new(0);
    let mut harness = support::harness_for(light(), Vec2::new(600.0, 100.0), |ui| {
        let mut index = selected.get();
        ui.add(
            MaterialTabs::primary(&mut index)
                .tab("One")
                .item(TabItem::new("Two").enabled(false))
                .tab("Three"),
        );
        selected.set(index);
    });
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("One").is_focused(),
        "Tab enters the tabs"
    );
    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    assert!(
        harness.get_by_label("Three").is_focused(),
        "Right skips the disabled tab"
    );
    harness.key_press(egui::Key::ArrowRight);
    harness.run();
    assert!(harness.get_by_label("One").is_focused(), "Right wraps");
    harness.key_press(egui::Key::ArrowLeft);
    harness.run();
    assert!(harness.get_by_label("Three").is_focused(), "Left goes back");
    harness.key_press(egui::Key::Enter);
    harness.run();
    assert_eq!(selected.get(), 2, "Enter selects the focused tab");
    assert_eq!(
        harness.get_by_label("Three").accesskit_node().is_selected(),
        Some(true),
        "the active tab reports selected"
    );
    let two = harness.get_by_label("Two");
    assert!(
        two.accesskit_node().is_disabled(),
        "the disabled tab says so"
    );
    two.click();
    harness.run();
    assert_eq!(selected.get(), 2, "a disabled tab ignores clicks");
}

#[test]
fn tab_labels_are_title_small_and_heights_match_tokens() {
    let theme = light();
    let title_small = TypeRole::TitleSmall.style();
    let driver = Driver::new(theme.clone(), Vec2::new(600.0, 400.0));
    let mut selected = 0;
    let mut heights = Vec::new();
    let shapes = driver.frame(Vec::new(), &mut |ui| {
        heights.push(
            ui.add(
                MaterialTabs::primary(&mut selected)
                    .tab("Alpha")
                    .tab("Beta"),
            )
            .rect
            .height(),
        );
        heights.push(
            ui.add(
                MaterialTabs::secondary(&mut selected)
                    .tab_with_icon("Alpha", "home")
                    .tab_with_icon("Beta", "star"),
            )
            .rect
            .height(),
        );
    });
    // PrimaryNavigationTabTokens.ContainerHeight / LargeContainerHeight.
    assert_eq!(heights, vec![48.0, 64.0], "tab heights");
    let alpha = texts(&shapes)
        .into_iter()
        .find(|t| t.galley.text() == "Alpha");
    let format = alpha
        .as_ref()
        .and_then(|t| t.galley.job.sections.first())
        .map(|s| s.format.clone());
    assert_eq!(
        format.as_ref().map(|f| f.font_id.size),
        Some(title_small.size),
        "labels use TitleSmall's size"
    );
    assert_eq!(
        format.and_then(|f| f.line_height),
        Some(title_small.line_height),
        "and its line height"
    );
    assert!(
        filled_with(&shapes, theme.color(ColorRole::Surface))
            .iter()
            .any(|r| r.height() == 48.0),
        "the bar is surface by default"
    );
}

#[test]
fn transparent_tabs_paint_no_container() {
    let theme = light();
    let driver = Driver::new(theme.clone(), Vec2::new(600.0, 200.0));
    let mut selected = 0;
    let mut rect = Rect::NOTHING;
    let shapes = driver.frame(Vec::new(), &mut |ui| {
        rect = ui
            .add(
                MaterialTabs::primary(&mut selected)
                    .tab("Alpha")
                    .tab("Beta")
                    .transparent(),
            )
            .rect;
    });
    assert!(
        !rects(&shapes).iter().any(|r| r.rect == rect),
        "nothing fills the whole bar"
    );
    assert!(
        filled_with(&shapes, theme.color(ColorRole::Primary))
            .iter()
            .any(|r| r.height() == 3.0),
        "the primary indicator is still drawn"
    );
}

#[test]
fn tab_layouts_size_tabs() {
    let tab_rects = |layout: TabLayout| {
        let harness = support::harness_for(light(), Vec2::new(800.0, 100.0), move |ui| {
            let mut selected = 0;
            ui.add(
                MaterialTabs::primary(&mut selected)
                    .tab("A")
                    .tab("A much longer label")
                    .layout(layout),
            );
        });
        (
            harness.get_by_label("A").rect(),
            harness.get_by_label("A much longer label").rect(),
        )
    };
    let (short, long) = tab_rects(TabLayout::Fixed);
    // The harness's panel has a margin each side.
    let start = short.min.x;
    assert_eq!(short.width(), long.width(), "fixed: equal widths");
    assert_eq!(long.max.x, 800.0 - start, "fixed: fills the width");

    let (short, long) = tab_rects(TabLayout::Content);
    assert!(long.width() > short.width(), "content: sized to labels");
    assert!(short.width() < 90.0, "content: no minimum");
    assert_eq!(short.min.x, start, "content: from the start edge");
    assert_eq!(long.min.x, short.max.x, "content: tabs abut");

    let (short, _) = tab_rects(TabLayout::Scrollable);
    // TabRowDefaults.ScrollableTabRowMinTabWidth / EdgeStartPadding.
    assert_eq!(short.width(), 90.0, "scrollable: 90 dp minimum");
    assert_eq!(short.min.x, start + 52.0, "scrollable: 52 dp start inset");
}

#[test]
fn scrollable_tabs_overflow_into_a_scroll_area() {
    let mut harness = support::harness_for(light(), Vec2::new(300.0, 100.0), |ui| {
        let mut selected = 0;
        let mut tabs = MaterialTabs::primary(&mut selected).scrollable();
        for i in 0..8 {
            tabs = tabs.tab(format!("Destination {i}"));
        }
        ui.add(tabs);
    });
    let last = harness.get_by_label("Destination 7").rect();
    assert!(last.min.x > 300.0, "the last tab starts off screen");
    harness.get_by_label("Destination 0").focus();
    harness.run();
    harness.key_press(egui::Key::End);
    harness.run();
    let last = harness.get_by_label("Destination 7");
    assert!(last.is_focused(), "End focuses the last tab");
    assert!(
        last.rect().max.x <= 300.5,
        "focusing it scrolls it into view"
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
    // Software and hardware rasterisers differ in anti-aliasing; allow that,
    // not a missing or moved item.
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

/// The light and dark themes of the default theme file.
fn light_and_dark() -> Vec<(&'static str, MaterialTheme)> {
    support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
        .collect()
}

#[test]
fn navigation_bar_and_tabs_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, Vec2::new(480.0, 520.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            let mut selected = 1;
            let mut bar = NavigationBar::new(&mut selected);
            for item in three_items() {
                bar = bar.item(item);
            }
            ui.add(bar);
            ui.add_space(8.0);
            let mut selected = 0;
            let mut bar =
                NavigationBar::new(&mut selected).variant(NavigationBarVariant::FlexibleHorizontal);
            for item in three_items() {
                bar = bar.item(item);
            }
            ui.add(bar);
            ui.add_space(8.0);
            let mut selected = 0;
            ui.add(
                MaterialTabs::primary(&mut selected)
                    .tab_with_icon("Flights", "flight")
                    .item(
                        TabItem::new("Trips")
                            .icon("luggage")
                            .badge(NavigationBadge::Small),
                    )
                    .item(TabItem::new("Explore").icon("explore").enabled(false)),
            );
            ui.add_space(8.0);
            let mut selected = 1;
            ui.add(
                MaterialTabs::secondary(&mut selected)
                    .tab("Overview")
                    .tab("Specifications")
                    .item(TabItem::new("Reviews").badge(NavigationBadge::Large("12".into()))),
            );
            ui.add_space(8.0);
            let mut selected = 2;
            let mut tabs = MaterialTabs::primary(&mut selected).scrollable();
            for label in ["Music", "Movies", "Books", "Podcasts", "Audiobooks"] {
                tabs = tabs.tab(label);
            }
            ui.add(tabs);
            ui.add_space(8.0);
            let mut selected = 0;
            ui.add(
                MaterialTabs::primary(&mut selected)
                    .tab("Fit")
                    .tab("To labels")
                    .layout(TabLayout::Content)
                    .transparent(),
            );
        });
        snapshot(&mut harness, &format!("navigation_bar_and_tabs_{name}"))?;
    }
    Ok(())
}

#[test]
fn navigation_rail_and_drawer_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::SurfaceContainerHighest);
        let mut harness = support::harness_for(theme, Vec2::new(760.0, 420.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.horizontal(|ui| {
                let mut selected = 1;
                let mut rail = NavigationRail::new(&mut selected).header(|ui| {
                    ui.label("menu");
                });
                for item in three_items() {
                    rail = rail.item(item);
                }
                ui.add(rail);
                let mut selected = 0;
                let mut expanded = true;
                let mut rail = NavigationRail::new(&mut selected).expanded(&mut expanded);
                for item in three_items() {
                    rail = rail.item(item);
                }
                ui.add(rail);
                ui.vertical(|ui| {
                    ui.set_width(380.0);
                    let mut open = true;
                    ui.add(
                        MaterialDrawer::new(DrawerVariant::Permanent, &mut open)
                            .width(380.0)
                            .section(Some("Mail"), drawer_items()),
                    );
                });
            });
        });
        snapshot(&mut harness, &format!("navigation_rail_and_drawer_{name}"))?;
    }
    Ok(())
}
