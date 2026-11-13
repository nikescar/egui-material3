// Integration tests for conditional drawer functionality
// Tests verify API compiles and can be used correctly

use egui::Id;
use egui_material3::data_table;

#[test]
fn test_drawer_if_api_compiles_with_true_condition() {
    // Arrange
    let condition = true;

    // Act - verify API compiles and creates table
    let _table = data_table()
        .id(Id::new("test_drawer_if_true"))
        .allow_drawer(true)
        .column("Product", 100.0, false)
        .column("Stock", 80.0, true)
        .row(|row| {
            row.cell("Keyboard").cell("45").drawer_if(condition, |ui| {
                ui.label("High stock - bulk order available");
            })
        });

    // Assert - compilation is the test
}

#[test]
fn test_drawer_if_api_compiles_with_false_condition() {
    // Arrange
    let condition = false;

    // Act - verify API compiles and creates table
    let _table = data_table()
        .id(Id::new("test_drawer_if_false"))
        .allow_drawer(true)
        .column("Product", 100.0, false)
        .column("Stock", 80.0, true)
        .row(|row| {
            row.cell("Mouse").cell("10").drawer_if(condition, |ui| {
                ui.label("Details");
            })
        });

    // Assert - compilation is the test
}

#[test]
fn test_mixed_drawer_conditions_api() {
    // Arrange & Act - build table with mixed conditions
    let table = data_table()
        .id(Id::new("test_mixed_drawers"))
        .allow_drawer(true)
        .column("Product", 100.0, false)
        .column("Stock", 80.0, true)
        .row(|row| {
            row.cell("Mouse").cell("10").drawer_if(10 > 20, |ui| {
                ui.label("Stock: 10 - bulk order available");
            })
        })
        .row(|row| {
            row.cell("Keyboard").cell("45").drawer_if(45 > 20, |ui| {
                ui.label("Stock: 45 - bulk order available");
            })
        })
        .row(|row| {
            row.cell("Monitor").cell("5").drawer_if(5 > 20, |ui| {
                ui.label("Stock: 5 - bulk order available");
            })
        })
        .row(|row| {
            row.cell("Laptop").cell("30").drawer_if(30 > 20, |ui| {
                ui.label("Stock: 30 - bulk order available");
            })
        });

    // Assert - successful table creation
    let _ = table;
}

#[test]
fn test_drawer_if_with_different_conditions() {
    // Arrange & Act - verify different condition expressions work
    let table = data_table()
        .id(Id::new("test_dynamic_condition"))
        .allow_drawer(true)
        .column("Product", 100.0, false)
        .column("Stock", 80.0, true)
        .row(|row| {
            let stock = 25;
            row.cell("Item A").cell("25").drawer_if(stock > 20, |ui| {
                ui.label("Available for bulk order");
            })
        })
        .row(|row| {
            let stock = 8;
            row.cell("Item B").cell("8").drawer_if(stock > 20, |ui| {
                ui.label("Available for bulk order");
            })
        });

    // Assert - successful API usage
    let _ = table;
}
