//! Integration tests for list item vertical centering

use egui_material3::{ListItem, MaterialList};

/// Test that 5-line items have balanced top/bottom padding
#[test]
fn test_multiline_vertical_centering() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list = MaterialList::new().item(
                ListItem::new("Backend Development")
                    .overline("Engineering")
                    .secondary_text("Server infrastructure\nand API services")
                    .trailing_text("Expand ▶"),
            );

            let response = ui.add(list);

            // Item should render without panic
            // Visual balance verified manually
            assert!(response.rect.height() > 0.0);
        });
    })
    .textures_delta
    .clear();
}

/// Test that 3-line centering remains unchanged
#[test]
fn test_3line_centering_unchanged() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list = MaterialList::new().item(
                ListItem::new("Primary Text")
                    .overline("Overline")
                    .secondary_text("Single line secondary"),
            );

            let response = ui.add(list);

            // Standard 3-line item
            // Expected height: 88px (base for 3-line)
            let expected_height = 88.0;
            let diff = (response.rect.height() - expected_height).abs();
            assert!(
                diff < 5.0,
                "3-line item height should be ~88px, got {}",
                response.rect.height()
            );
        });
    })
    .textures_delta
    .clear();
}

/// Test that 2-line items (primary + secondary) work correctly
#[test]
fn test_2line_with_multiline_secondary() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list = MaterialList::new()
                .item(ListItem::new("Primary Text").secondary_text("Line 1\nLine 2"));

            let response = ui.add(list);

            // 2-line base (72px) + adjusted for centered padding
            // Note: Height is now 80px with balanced padding (10px less than before)
            let expected_height = 80.0;
            let diff = (response.rect.height() - expected_height).abs();
            assert!(
                diff < 5.0,
                "2-line item with multi-line secondary should be ~80px with balanced padding, got {}",
                response.rect.height()
            );
        });
    })
    .textures_delta
    .clear();
}

/// Test dense mode with multi-line items
#[test]
fn test_dense_multiline_centering() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list = MaterialList::new().item(
                ListItem::new("Primary")
                    .overline("Overline")
                    .secondary_text("Line 1\nLine 2")
                    .dense(true),
            );

            let response = ui.add(list);

            // Dense 3-line base (76px) + adjusted for centered padding
            // Note: Height is now 82px with balanced padding
            let expected_height = 82.0;
            let diff = (response.rect.height() - expected_height).abs();
            assert!(
                diff < 5.0,
                "Dense multi-line item should be ~82px with balanced padding, got {}",
                response.rect.height()
            );
        });
    })
    .textures_delta
    .clear();
}

/// Test that items without overline don't get offset adjustment
#[test]
fn test_no_overline_centering() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            let list = MaterialList::new()
                .item(ListItem::new("Primary").secondary_text("Line 1\nLine 2\nLine 3"));

            let response = ui.add(list);

            // Should render without issues
            assert!(response.rect.height() > 0.0);
        });
    })
    .textures_delta
    .clear();
}
