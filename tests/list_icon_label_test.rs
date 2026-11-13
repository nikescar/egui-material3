//! Tests for list item icon label rendering

use egui_material3::{ListItem, MaterialList};

#[test]
fn test_icon_without_label_renders() {
    // Arrange
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    // Act & Assert
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .id("test_icon_only")
                .item(ListItem::new("Test Item").leading_icon("folder"));

            let response = ui.add(list);
            assert!(response.rect.height() > 0.0);
        });
    });
}

#[test]
fn test_label_without_icon_ignored() {
    // Arrange
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    // Act & Assert
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .id("test_label_only")
                .item(ListItem::new("Test Item").leading_icon_label("Label"));

            let response = ui.add(list);
            // Label without icon renders without crash
            assert!(response.rect.height() > 0.0);
        });
    });
}

#[test]
fn test_icon_with_label_renders_both() {
    // Arrange
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    // Act & Assert
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new().id("test_icon_and_label").item(
                ListItem::new("Test Item")
                    .leading_icon("folder")
                    .leading_icon_label("Department"),
            );

            let response = ui.add(list);
            assert!(response.rect.height() > 0.0);
        });
    });
}

#[test]
fn test_icon_label_builder_method_exists() {
    // Arrange & Act
    let _item = ListItem::new("Test")
        .leading_icon("folder")
        .leading_icon_label("Label");

    // Assert - compilation is the test
}

#[test]
fn test_multiple_items_with_labels() {
    // Arrange
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    // Act & Assert
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let list = MaterialList::new()
                .id("test_multiple_labels")
                .item(
                    ListItem::new("Item 1")
                        .leading_icon("folder")
                        .leading_icon_label("Dept 1"),
                )
                .item(
                    ListItem::new("Item 2")
                        .leading_icon("inbox")
                        .leading_icon_label("Dept 2"),
                )
                .item(ListItem::new("Item 3").leading_icon("star"));

            let response = ui.add(list);
            assert!(response.rect.height() > 0.0);
        });
    });
}
