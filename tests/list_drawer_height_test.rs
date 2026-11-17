//! Integration tests for List drawer height expansion behavior

use egui_material3::{ListItem, MaterialList, MaterialListState};

/// Test that a list with closed drawer has correct height
#[test]
fn test_drawer_closed_height() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let item_height = 56.0; // Single-line item default height

            let list =
                MaterialList::new()
                    .allow_drawer(true)
                    .item(ListItem::new("Test Item").drawer(|ui| {
                        ui.label("Drawer content");
                    }));

            let response = ui.add(list);

            // When drawer is closed, list height should be just the item height
            assert!(
                response.rect.height() <= item_height + 20.0, // Small margin for borders
                "Closed drawer list height {} should be ~{}",
                response.rect.height(),
                item_height
            );
        });
    });
}

/// Test that a list with open drawer expands to include drawer content
#[test]
fn test_drawer_open_expands_height() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let item_height = 56.0;

            let list = MaterialList::new()
                .allow_drawer(true)
                .id(egui::Id::new("test_list_1"))
                .item(ListItem::new("Test Item").drawer(|ui| {
                    // Create drawer content with known height
                    ui.label("Line 1");
                    ui.label("Line 2");
                    ui.label("Line 3");
                }));

            let response = ui.add(list);

            // First render: drawer is closed
            let closed_height = response.rect.height();
            assert!(
                closed_height <= item_height + 20.0,
                "Initial height should not include drawer"
            );
        });
    });

    // Second frame: open the drawer
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let item_height = 56.0;

            // Manually open the drawer by inserting state
            let list_id = egui::Id::new("test_list_1");
            let mut state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();
            state.drawer_open_items.insert(0);
            ui.data_mut(|d| d.insert_persisted(list_id, state));

            let list = MaterialList::new().allow_drawer(true).id(list_id).item(
                ListItem::new("Test Item").drawer(|ui| {
                    ui.label("Line 1");
                    ui.label("Line 2");
                    ui.label("Line 3");
                }),
            );

            let response = ui.add(list);

            // With drawer open, height should be larger
            assert!(
                response.rect.height() > item_height + 40.0,
                "Open drawer height {} should be > item_height + drawer content",
                response.rect.height()
            );
        });
    });
}

/// Test that multiple open drawers all contribute to total height
#[test]
fn test_multiple_drawers_open() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list_id = egui::Id::new("test_list_multi");

            // Open both drawers
            let mut state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();
            state.drawer_open_items.insert(0);
            state.drawer_open_items.insert(1);
            ui.data_mut(|d| d.insert_persisted(list_id, state));

            let list = MaterialList::new()
                .allow_drawer(true)
                .id(list_id)
                .item(ListItem::new("Item 1").drawer(|ui| {
                    ui.label("Drawer 1 content");
                    ui.label("More content");
                }))
                .item(ListItem::new("Item 2").drawer(|ui| {
                    ui.label("Drawer 2 content");
                    ui.label("Even more content");
                }));

            let response = ui.add(list);

            // Height should include both items + both drawer contents
            let min_expected = 56.0 * 2.0 + 40.0 * 2.0; // 2 items + 2 drawers
            assert!(
                response.rect.height() > min_expected,
                "Multiple drawer height {} should be > {}",
                response.rect.height(),
                min_expected
            );
        });
    });
}

/// Test that drawer content is fully visible (not clipped)
#[test]
fn test_drawer_content_not_clipped() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list_id = egui::Id::new("test_list_clip");

            // Open drawer
            let mut state = ui
                .data_mut(|d| d.get_persisted::<MaterialListState>(list_id))
                .unwrap_or_default();
            state.drawer_open_items.insert(0);
            ui.data_mut(|d| d.insert_persisted(list_id, state));

            let list = MaterialList::new().allow_drawer(true).id(list_id).item(
                ListItem::new("Test Item").drawer(|ui| {
                    // Large drawer content
                    for i in 0..10 {
                        ui.label(format!("Line {}", i));
                    }
                }),
            );

            let response = ui.add(list);

            // The list should be tall enough to contain all drawer content
            // Each egui label is approximately 18-19px tall with default font
            // 10 lines * ~18px per line + item height (56) + padding (16) ≈ 250px
            let min_height = 56.0 + 10.0 * 18.0 + 16.0; // item + lines + padding
            assert!(
                response.rect.height() >= min_height,
                "List height {} should accommodate all drawer content (>= {})",
                response.rect.height(),
                min_height
            );
        });
    });
}
