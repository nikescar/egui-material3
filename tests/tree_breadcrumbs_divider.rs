// ============================================================================
// Module:       tree_breadcrumbs_divider (integration test)
// Description:  Tree view responses and roles, breadcrumbs, dividers.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! The smaller WP-11 widgets: the tree view reports `TreeItem` rows with
//! their selected state, returns responses that click and change, and
//! works from the keyboard; the current breadcrumb is not disabled; and
//! dividers are 1 dp `outlineVariant` lines, full or inset.

mod support;

use std::cell::{Cell, RefCell};

use egui::accesskit::Role;
use egui::epaint::Shape as PaintShape;
use egui::{Key, Pos2, Rect, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::{
    breadcrumbs, divider, ColorRole, MaterialDivider, MaterialTheme, MaterialTreeView,
    TreeViewItem, TreeViewState,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn tree() -> Vec<TreeViewItem> {
    vec![
        TreeViewItem::new("docs", "Documents")
            .icon("folder")
            .child(TreeViewItem::new("cv", "Resume"))
            .child(TreeViewItem::new("tax", "Taxes")),
        TreeViewItem::new("pics", "Pictures"),
    ]
}

#[test]
fn tree_rows_are_tree_items_with_selection() -> TestResult {
    let items = tree();
    let state = RefCell::new(TreeViewState::new());
    let changes = Cell::new(0);
    let clicks = Cell::new(0);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        Vec2::new(400.0, 300.0),
        |ui| {
            let mut s = state.borrow_mut();
            let response = ui.add(MaterialTreeView::new(&items, &mut s));
            if response.changed() {
                changes.set(changes.get() + 1);
            }
            if response.clicked() {
                clicks.set(clicks.get() + 1);
            }
        },
    );
    let docs = harness.get_by_label("Documents");
    assert_eq!(docs.accesskit_node().role(), Role::TreeItem, "row role");
    assert_eq!(docs.rect().height(), 40.0, "default row height");
    assert!(
        harness.query_by_role(Role::Tree).is_some(),
        "rows sit in a Tree node"
    );
    harness.get_by_label("Pictures").click();
    harness.run();
    assert!(
        state.borrow().is_selected("pics"),
        "clicking a row selects it"
    );
    assert!(changes.get() >= 1, "the response reports the change");
    assert!(clicks.get() >= 1, "the response reports the click");
    assert_eq!(
        harness
            .get_by_label("Pictures")
            .accesskit_node()
            .is_selected(),
        Some(true),
        "AccessKit sees the selection"
    );
    Ok(())
}

#[test]
fn tree_keyboard_expands_collapses_and_moves() -> TestResult {
    let items = tree();
    let state = RefCell::new(TreeViewState::new());
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        Vec2::new(400.0, 300.0),
        |ui| {
            let mut s = state.borrow_mut();
            ui.add(MaterialTreeView::new(&items, &mut s));
        },
    );
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Documents").is_focused(),
        "Tab reaches the first row"
    );
    harness.key_press(Key::ArrowRight);
    harness.run();
    assert!(state.borrow().is_expanded("docs"), "Right expands");
    assert!(
        harness.query_by_label("Resume").is_some(),
        "children appear"
    );
    harness.key_press(Key::ArrowDown);
    harness.run();
    assert!(
        harness.get_by_label("Resume").is_focused(),
        "Down moves to the child"
    );
    harness.key_press(Key::ArrowLeft);
    harness.run();
    assert!(
        harness.get_by_label("Documents").is_focused(),
        "Left moves to the parent"
    );
    harness.key_press(Key::ArrowLeft);
    harness.run();
    assert!(
        !state.borrow().is_expanded("docs"),
        "Left on an open row collapses it"
    );
    harness.key_press(Key::Space);
    harness.run();
    assert!(state.borrow().is_selected("docs"), "Space selects");
    Ok(())
}

#[test]
fn the_current_breadcrumb_is_not_disabled() -> TestResult {
    for (name, theme) in support::themes() {
        let harness = support::harness_for(theme, Vec2::new(500.0, 100.0), |ui| {
            ui.add(
                breadcrumbs()
                    .item("Home")
                    .item("Products")
                    .active_item("Laptops"),
            );
        });
        let current = harness.get_by_label("Laptops");
        assert!(
            !current.accesskit_node().is_disabled(),
            "{name}: the current page is not reported disabled"
        );
        assert_eq!(
            current.accesskit_node().role(),
            Role::Label,
            "{name}: it is a label, not a button"
        );
        assert_eq!(
            harness.get_by_label("Home").accesskit_node().role(),
            Role::Button,
            "{name}: earlier crumbs are buttons"
        );
    }
    Ok(())
}

fn divider_rect(make: impl Fn() -> MaterialDivider) -> Option<(Rect, egui::Color32)> {
    let ctx = egui::Context::default();
    MaterialTheme::set(&ctx, MaterialTheme::baseline(false));
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 100.0))),
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        ui.add(make());
    });
    output.textures_delta.clear();
    output.shapes.into_iter().find_map(|c| match c.shape {
        PaintShape::Rect(r) => Some((r.rect, r.fill)),
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
}

#[test]
fn dividers_are_one_dp_outline_variant_lines() -> TestResult {
    let outline_variant = MaterialTheme::baseline(false).color(ColorRole::OutlineVariant);
    let (full, color) = divider_rect(divider).ok_or("full divider painted nothing")?;
    assert_eq!(full.height(), 1.0, "DividerTokens.Thickness");
    assert_eq!(color, outline_variant, "DividerTokens.Color");
    let (inset, _) = divider_rect(MaterialDivider::inset).ok_or("inset divider painted nothing")?;
    assert_eq!(
        inset.min.x - full.min.x,
        16.0,
        "inset by 16 dp at the start"
    );
    assert_eq!(inset.max.x, full.max.x, "full to the end");
    let (middle, _) =
        divider_rect(MaterialDivider::middle_inset).ok_or("middle divider painted nothing")?;
    assert_eq!(
        full.max.x - middle.max.x,
        16.0,
        "middle inset at the end too"
    );
    let (vertical, _) = divider_rect(|| MaterialDivider::vertical().length(40.0))
        .ok_or("vertical painted nothing")?;
    assert_eq!(
        (vertical.width(), vertical.height()),
        (1.0, 40.0),
        "a vertical divider is 1 dp wide"
    );
    Ok(())
}
