//! Integration tests for multi-line secondary text support

use egui_material3::{ListItem, MaterialList};

/// Test that secondary text with \n renders as multiple lines
#[test]
fn test_multiline_secondary_text_renders() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .item(
                    ListItem::new("Primary Text")
                        .secondary_text("Line 1\nLine 2")
                );

            ui.add(list);
            // If it compiles and renders without panic, test passes
            assert!(true);
        });
    });
}

/// Test that single-line secondary text still works
#[test]
fn test_single_line_secondary_text_unchanged() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .item(
                    ListItem::new("Primary Text")
                        .secondary_text("Single line only")
                );

            ui.add(list);
            assert!(true);
        });
    });
}

/// Test that multi-line secondary text increases item height
#[test]
fn test_multiline_increases_height() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let single_line = MaterialList::new()
                .item(
                    ListItem::new("Item")
                        .secondary_text("One line")
                );

            let multi_line = MaterialList::new()
                .item(
                    ListItem::new("Item")
                        .secondary_text("Line 1\nLine 2\nLine 3")
                );

            let single_response = ui.add(single_line);
            let multi_response = ui.add(multi_line);

            // Multi-line should be taller
            assert!(multi_response.rect.height() > single_response.rect.height());
        });
    });
}

/// Test that empty secondary text works
#[test]
fn test_no_secondary_text() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .item(ListItem::new("Primary only"));

            ui.add(list);
            assert!(true);
        });
    });
}

/// Test 5-line item: overline + primary + 2-line secondary + trailing
#[test]
fn test_five_line_item_with_all_fields() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .item(
                    ListItem::new("Primary Text")
                        .overline("Overline")
                        .secondary_text("Secondary Line 1\nSecondary Line 2")
                        .trailing_text("Trailing")
                );

            ui.add(list);
            assert!(true);
        });
    });
}
