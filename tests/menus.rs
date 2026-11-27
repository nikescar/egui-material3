// ============================================================================
// Module:       menus (integration test)
// Description:  Menus: width, item height, keyboard, submenus, AccessKit.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Menus (WP-11): content-sized width within 112-280 dp, item heights per
//! variant, keyboard navigation and typeahead, submenus opened with the
//! Right arrow, check and radio items in the AccessKit tree, Escape and
//! outside clicks closing, the context-menu helper, and snapshots.

mod support;

use std::cell::{Cell, RefCell};

use egui::accesskit::{Role, Toggled};
use egui::{Event, Key, Pos2, Rect, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::{
    context_menu, menu_item, FocusState, MaterialMenu, MaterialTheme, MenuVariant,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const ANCHOR: Rect = Rect {
    min: Pos2::new(40.0, 40.0),
    max: Pos2::new(120.0, 80.0),
};

fn harness_size() -> Vec2 {
    Vec2::new(700.0, 700.0)
}

#[test]
fn width_is_content_sized_within_112_and_280() -> TestResult {
    let open = Cell::new(true);
    let narrow = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = open.get();
        MaterialMenu::new(egui::Id::unique("narrow"), &mut o)
            .anchor_rect(ANCHOR)
            .item(menu_item("A"))
            .show(ui.ctx());
        open.set(o);
    });
    let width = narrow.get_by_role(Role::Menu).rect().width();
    assert_eq!(width, 112.0, "a one-letter menu is the 112 dp minimum");

    let wide = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = true;
        MaterialMenu::new(egui::Id::unique("wide"), &mut o)
            .anchor_rect(ANCHOR)
            .item(menu_item(
                "An extremely long menu item label that cannot possibly fit in 280 dp",
            ))
            .show(ui.ctx());
    });
    let width = wide.get_by_role(Role::Menu).rect().width();
    assert_eq!(width, 280.0, "a long label is capped at the 280 dp maximum");

    let medium = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = true;
        MaterialMenu::new(egui::Id::unique("medium"), &mut o)
            .anchor_rect(ANCHOR)
            .item(
                menu_item("Select all")
                    .leading_icon("select_all")
                    .trailing_text("Ctrl+A"),
            )
            .show(ui.ctx());
    });
    let width = medium.get_by_role(Role::Menu).rect().width();
    assert!(
        width > 112.0 && width < 280.0,
        "icon, label and shortcut size the menu between the limits: {width}"
    );
    Ok(())
}

#[test]
fn item_height_is_48_baseline_and_44_expressive() -> TestResult {
    for (variant, expected) in [
        (MenuVariant::Baseline, 48.0),
        (MenuVariant::Standard, 44.0),
        (MenuVariant::Vibrant, 44.0),
    ] {
        let harness = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
            let mut o = true;
            MaterialMenu::new(egui::Id::unique("heights"), &mut o)
                .anchor_rect(ANCHOR)
                .variant(variant)
                .item(menu_item("Cut"))
                .item(menu_item("Copy"))
                .show(ui.ctx());
        });
        let height = harness.get_by_label("Cut").rect().height();
        assert_eq!(height, expected, "{variant:?}: item height");
        let menu = harness.get_by_role(Role::Menu).rect();
        assert!(
            menu.min.y >= ANCHOR.max.y,
            "{variant:?}: the menu opens below its anchor: {menu:?}"
        );
    }
    Ok(())
}

#[test]
fn arrows_move_the_highlight_and_enter_activates() -> TestResult {
    let open = Cell::new(true);
    let picked = RefCell::new(Vec::new());
    let mut harness = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = open.get();
        MaterialMenu::new(egui::Id::unique("keys"), &mut o)
            .anchor_rect(ANCHOR)
            .item(menu_item("Cut").on_click(|| picked.borrow_mut().push("Cut")))
            .item(
                menu_item("Copy")
                    .enabled(false)
                    .on_click(|| picked.borrow_mut().push("Copy")),
            )
            .item(menu_item("Paste").on_click(|| picked.borrow_mut().push("Paste")))
            .show(ui.ctx());
        open.set(o);
    });
    harness.key_press(Key::ArrowDown);
    harness.run();
    assert!(
        harness.get_by_label("Cut").is_focused(),
        "the first Down highlights (and focuses) the first item"
    );
    harness.key_press(Key::ArrowDown);
    harness.run();
    assert!(
        harness.get_by_label("Paste").is_focused(),
        "Down skips the disabled item"
    );
    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(
        *picked.borrow(),
        vec!["Paste"],
        "Enter activates Paste only"
    );
    assert!(!open.get(), "activating an item closes the menu");
    Ok(())
}

#[test]
fn typeahead_jumps_to_the_matching_item() -> TestResult {
    let mut harness = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = true;
        MaterialMenu::new(egui::Id::unique("typeahead"), &mut o)
            .anchor_rect(ANCHOR)
            .item(menu_item("Apple"))
            .item(menu_item("Banana"))
            .item(menu_item("Cherry"))
            .show(ui.ctx());
    });
    harness.event(Event::Text("c".to_owned()));
    harness.run();
    assert!(
        harness.get_by_label("Cherry").is_focused(),
        "typing c highlights Cherry"
    );
    Ok(())
}

#[test]
fn right_arrow_opens_a_submenu_and_left_closes_it() -> TestResult {
    let shared = Cell::new(false);
    let mut harness = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = true;
        MaterialMenu::new(egui::Id::unique("cascade"), &mut o)
            .anchor_rect(ANCHOR)
            .default_focus(FocusState::FirstItem)
            .item(menu_item("Share").submenu([
                menu_item("Email").on_click(|| shared.set(true)),
                menu_item("Message"),
            ]))
            .item(menu_item("Delete"))
            .show(ui.ctx());
    });
    assert!(
        harness.query_by_label("Email").is_none(),
        "the submenu starts closed"
    );
    harness.key_press(Key::ArrowRight);
    harness.run();
    let email = harness.get_by_label("Email");
    assert!(
        email.is_focused(),
        "Right opens the submenu on its first item"
    );
    let share = harness.get_by_label("Share").rect();
    assert!(
        email.rect().min.x >= share.max.x,
        "the submenu opens beside its item"
    );
    harness.key_press(Key::ArrowLeft);
    harness.run();
    assert!(
        harness.query_by_label("Email").is_none(),
        "Left closes the submenu"
    );
    assert!(
        harness.get_by_label("Share").is_focused(),
        "focus returns to the parent item"
    );
    harness.key_press(Key::ArrowRight);
    harness.run();
    harness.key_press(Key::Enter);
    harness.run();
    assert!(shared.get(), "Enter in the submenu activates its item");
    Ok(())
}

#[test]
fn check_and_radio_items_report_their_state() -> TestResult {
    for (name, theme) in support::themes() {
        let bold = Cell::new(true);
        let mut harness = support::harness_for(theme, harness_size(), |ui| {
            let mut o = true;
            let mut b = bold.get();
            MaterialMenu::new(egui::Id::unique("format"), &mut o)
                .anchor_rect(ANCHOR)
                .item(menu_item("Bold").toggles(&mut b).keep_open(true))
                .group("Align")
                .item(menu_item("Left").radio(true))
                .item(menu_item("Right").radio(false))
                .item(menu_item("Plain"))
                .show(ui.ctx());
            bold.set(b);
        });
        let node = |label: &str| harness.get_by_label(label).accesskit_node().role();
        assert_eq!(node("Bold"), Role::MenuItemCheckBox, "{name}: check item");
        assert_eq!(node("Left"), Role::MenuItemRadio, "{name}: radio item");
        assert_eq!(node("Plain"), Role::MenuItem, "{name}: plain item");
        assert_eq!(
            harness.get_by_label("Left").accesskit_node().toggled(),
            Some(Toggled::True),
            "{name}: selected radio"
        );
        assert_eq!(
            harness.get_by_label("Right").accesskit_node().toggled(),
            Some(Toggled::False),
            "{name}: unselected radio"
        );
        harness.get_by_label("Bold").click();
        harness.run();
        assert!(!bold.get(), "{name}: clicking a bound check item flips it");
        assert!(
            harness.query_by_label("Bold").is_some(),
            "{name}: keep_open leaves the menu open"
        );
    }
    Ok(())
}

#[test]
fn disabled_items_report_disabled_and_do_not_click() -> TestResult {
    let clicked = Cell::new(false);
    let harness = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = true;
        MaterialMenu::new(egui::Id::unique("disabled"), &mut o)
            .anchor_rect(ANCHOR)
            .item(
                menu_item("Undo")
                    .enabled(false)
                    .on_click(|| clicked.set(true)),
            )
            .show(ui.ctx());
    });
    let undo = harness.get_by_label("Undo");
    assert!(
        undo.accesskit_node().is_disabled(),
        "disabled item is reported"
    );
    undo.click();
    assert!(!clicked.get(), "a disabled item does not run its action");
    Ok(())
}

#[test]
fn escape_and_outside_clicks_close() -> TestResult {
    let open = Cell::new(true);
    let mut harness = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        let mut o = open.get();
        MaterialMenu::new(egui::Id::unique("closing"), &mut o)
            .anchor_rect(ANCHOR)
            .item(menu_item("One"))
            .show(ui.ctx());
        open.set(o);
    });
    harness.key_press(Key::Escape);
    harness.run();
    assert!(!open.get(), "Escape closes the menu");

    open.set(true);
    harness.run();
    let far = Pos2::new(650.0, 650.0);
    harness.event(Event::PointerMoved(far));
    harness.event(Event::PointerButton {
        pos: far,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    });
    harness.event(Event::PointerButton {
        pos: far,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    });
    harness.run();
    assert!(!open.get(), "a click outside closes the menu");
    Ok(())
}

#[test]
fn context_menu_opens_at_the_pointer() -> TestResult {
    let mut harness = support::harness_for(MaterialTheme::baseline(false), harness_size(), |ui| {
        ui.add_space(200.0);
        let response = ui.add(egui::Button::new("Target"));
        context_menu(
            &response,
            MaterialMenu::context(egui::Id::unique("target_menu")).item(menu_item("Inspect")),
        );
    });
    assert!(
        harness.query_by_label("Inspect").is_none(),
        "closed until a secondary click"
    );
    let target = harness.get_by_label("Target");
    let at = target.rect().center();
    target.click_secondary();
    harness.run();
    let menu = harness.get_by_role(Role::Menu).rect();
    assert!(
        (menu.min.x - at.x).abs() < 1.0 && (menu.min.y - at.y).abs() < 1.0,
        "the menu's corner is at the pointer: {menu:?} vs {at:?}"
    );
    harness.key_press(Key::Escape);
    harness.run();
    assert!(
        harness.query_by_label("Inspect").is_none(),
        "Escape closes the context menu"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshots.

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

fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn snapshot(harness: &mut egui_kittest::Harness<'_>, name: &str) -> TestResult {
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

#[test]
fn menu_snapshots() -> TestResult {
    for (name, theme) in support::themes() {
        for variant in [
            MenuVariant::Baseline,
            MenuVariant::Standard,
            MenuVariant::Vibrant,
        ] {
            for segmented in [false, true] {
                if segmented && variant == MenuVariant::Baseline {
                    continue;
                }
                let mut harness =
                    support::harness_for(theme.clone(), Vec2::new(340.0, 380.0), |ui| {
                        let mut o = true;
                        MaterialMenu::new(egui::Id::unique("snapshot"), &mut o)
                            .anchor_rect(Rect::from_min_size(Pos2::new(16.0, 0.0), Vec2::ZERO))
                            .variant(variant)
                            .segmented(segmented)
                            .item(
                                menu_item("Cut")
                                    .leading_icon("content_cut")
                                    .trailing_text("Ctrl+X"),
                            )
                            .item(menu_item("Copy").leading_icon("content_copy"))
                            .item(menu_item("Paste").enabled(false))
                            .divider()
                            .item(menu_item("Bold").checked(true))
                            .item(
                                menu_item("Italic")
                                    .checked(false)
                                    .supporting_text("Slanted text"),
                            )
                            .item(menu_item("Share").submenu([menu_item("Email")]))
                            .show(ui.ctx());
                    });
                harness.key_press(Key::ArrowDown);
                harness.run();
                let seg = if segmented { "_segmented" } else { "" };
                snapshot(
                    &mut harness,
                    &format!("menu_{variant:?}{seg}_{name}").to_lowercase(),
                )?;
            }
        }
    }
    Ok(())
}
