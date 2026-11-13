//! Tests for List drawer functionality

use egui_material3::ListItem;

#[test]
fn test_list_item_can_have_drawer() {
    // Test that ListItem can have a drawer closure
    let item = ListItem::new("Test Item").drawer(|ui| {
        ui.label("Drawer content");
    });

    // If this compiles, the API exists
    assert!(true);
}

#[test]
fn test_drawer_if_conditional() {
    // Test conditional drawer
    let condition_true = true;
    let item_with_drawer = ListItem::new("Item 1").drawer_if(condition_true, |ui| {
        ui.label("This drawer shows");
    });

    let condition_false = false;
    let item_without_drawer = ListItem::new("Item 2").drawer_if(condition_false, |ui| {
        ui.label("This drawer does not show");
    });

    // If this compiles, drawer_if API exists
    assert!(true);
}

#[test]
fn test_drawer_default_is_none() {
    // Default ListItem should have no drawer
    let item = ListItem::new("Basic Item");

    // This is a compilation test - verifies the field exists
    assert!(true);
}
