//! Integration tests for list item spacing and padding

use egui_material3::{ListItem, MaterialList};

/// Test that multi-line items use correct height during rendering
#[test]
fn test_multiline_item_height_applied() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Single-line item
            let single = MaterialList::new()
                .id(egui::Id::new("single"))
                .item(
                    ListItem::new("Primary")
                        .secondary_text("One line")
                );

            // Multi-line item (should be taller)
            let multi = MaterialList::new()
                .id(egui::Id::new("multi"))
                .item(
                    ListItem::new("Primary")
                        .secondary_text("Line 1\nLine 2\nLine 3")
                );

            let single_response = ui.add(single);
            let multi_response = ui.add(multi);

            // Multi-line should be taller
            assert!(
                multi_response.rect.height() > single_response.rect.height(),
                "Multi-line item should be taller than single-line"
            );
        });
    });
}

/// Test that 5-line items have proper spacing
#[test]
fn test_five_line_item_spacing() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .item(
                    ListItem::new("Backend Development")
                        .overline("Engineering")
                        .secondary_text("Server infrastructure\nand API services")
                        .trailing_text("Expand ▶")
                );

            let response = ui.add(list);

            // 5-line item should have height:
            // base_height (88px for 3-line) + 1 extra line * 18px = 106px
            let expected_min_height = 88.0 + 18.0;
            assert!(
                response.rect.height() >= expected_min_height,
                "5-line item height {} should be >= {}",
                response.rect.height(),
                expected_min_height
            );
        });
    });
}

/// Test that multiple items don't accumulate spacing at bottom
#[test]
fn test_no_accumulated_bottom_spacing() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .item(
                    ListItem::new("Item 1")
                        .secondary_text("Line A\nLine B")
                )
                .item(
                    ListItem::new("Item 2")
                        .secondary_text("Line C\nLine D")
                )
                .item(
                    ListItem::new("Item 3")
                        .secondary_text("Line E\nLine F")
                );

            let response = ui.add(list);

            // Each item: base 72px + 1 extra line * 18px = 90px
            // 3 items * 90px = 270px total
            let expected_height = 3.0 * (72.0 + 18.0);

            // Allow small tolerance for rounding
            let diff = (response.rect.height() - expected_height).abs();
            assert!(
                diff < 5.0,
                "Total height {} should be close to {} (diff: {})",
                response.rect.height(),
                expected_height,
                diff
            );
        });
    });
}

/// Test that dividers don't cause spacing issues
#[test]
fn test_dividers_with_multiline() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .dividers(true)
                .item(
                    ListItem::new("Item 1")
                        .secondary_text("Line A\nLine B")
                )
                .item(
                    ListItem::new("Item 2")
                        .secondary_text("Line C\nLine D")
                );

            let response = ui.add(list);

            // 2 items * 90px + 1 divider * 1px = 181px
            let expected_height = 2.0 * (72.0 + 18.0) + 1.0;

            let diff = (response.rect.height() - expected_height).abs();
            assert!(
                diff < 5.0,
                "Height with dividers {} should be close to {}",
                response.rect.height(),
                expected_height
            );
        });
    });
}
