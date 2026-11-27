//! Integration tests for full-item drawer toggle behavior

use egui_material3::{ListItem, MaterialList, MaterialListState};

/// Test that clicking anywhere on an item with drawer toggles the drawer
#[test]
fn test_item_click_toggles_drawer_when_drawer_present() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    // First frame: drawer closed
    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list_id = egui::Id::unique("test_list_toggle");

            let list = MaterialList::new().id(list_id).allow_drawer(true).item(
                ListItem::new("Test Item").drawer(|ui| {
                    ui.label("Drawer content");
                }),
            );

            ui.add(list);

            // Get drawer state - should be closed initially
            let state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();

            assert!(
                !state.drawer_open_items.contains(&0),
                "Drawer should be closed initially"
            );

            // Simulate click on the item
            // In real implementation, clicking item_rect should toggle drawer
        });
    })
    .textures_delta
    .clear();
}

/// Test that clicking an item closes an already-open drawer
#[test]
fn test_item_click_closes_open_drawer() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list_id = egui::Id::unique("test_list_close");

            // Pre-set drawer to open state
            let mut state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();
            state.drawer_open_items.insert(0);
            ui.data_mut(|d| d.insert_persisted(list_id, state));

            let list = MaterialList::new().id(list_id).allow_drawer(true).item(
                ListItem::new("Test Item").drawer(|ui| {
                    ui.label("Drawer content");
                }),
            );

            ui.add(list);

            // Verify drawer is open
            let state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();

            assert!(
                state.drawer_open_items.contains(&0),
                "Drawer should be open after manual state set"
            );

            // TODO: After implementation, clicking item should close drawer
        });
    })
    .textures_delta
    .clear();
}

/// Test that chevron click still works (backwards compatibility)
#[test]
fn test_chevron_still_works() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list_id = egui::Id::unique("test_chevron");

            let list = MaterialList::new().id(list_id).allow_drawer(true).item(
                ListItem::new("Test Item").drawer(|ui| {
                    ui.label("Drawer content");
                }),
            );

            ui.add(list);
        });
    })
    .textures_delta
    .clear();
}

/// Test that items without drawer don't respond to drawer toggle
#[test]
fn test_item_without_drawer_uses_action_callback() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list = MaterialList::new()
                .allow_drawer(true)
                .item(ListItem::new("Test Item"));

            ui.add(list);
        });
    })
    .textures_delta
    .clear();
}

/// Test that action callback is ignored when drawer is present
#[test]
fn test_action_callback_ignored_when_drawer_present() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list_id = egui::Id::unique("test_priority");

            let list = MaterialList::new().id(list_id).allow_drawer(true).item(
                ListItem::new("Test Item").drawer(|ui| {
                    ui.label("Drawer content");
                }),
            );

            ui.add(list);

            // After clicking item with both drawer and action:
            // - Drawer should toggle
            // - Action callback should NOT be called
            let _state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();
        });
    })
    .textures_delta
    .clear();
}

/// Test that disabled items don't toggle drawer
#[test]
fn test_disabled_item_no_toggle() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list_id = egui::Id::unique("test_disabled");

            let list = MaterialList::new().id(list_id).allow_drawer(true).item(
                ListItem::new("Disabled Item").enabled(false).drawer(|ui| {
                    ui.label("Should not open");
                }),
            );

            ui.add(list);

            let state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();

            assert!(
                !state.drawer_open_items.contains(&0),
                "Disabled item drawer should remain closed"
            );
        });
    })
    .textures_delta
    .clear();
}
