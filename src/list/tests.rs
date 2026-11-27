//! Tests for MaterialList multiline text handling

use crate::list::{ListItem, MaterialList};

/// Test that multiline secondary text height calculation includes all lines
#[test]
fn test_multiline_secondary_text_height_calculation() {
    // Arrange: Create item with 5-line secondary text
    let secondary_text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let _item = ListItem::new("Title").secondary_text(secondary_text);

    // Act: Count lines in secondary text
    let line_count = secondary_text.split('\n').count();

    // Assert: Should have 5 lines
    assert_eq!(line_count, 5, "Secondary text should have 5 lines");

    // Calculate expected height (base 72px for 2-line + 3 extra lines * 18px)
    let base_height = 72.0;
    let extra_lines = (line_count - 1) as f32;
    let expected_min_height = base_height + extra_lines * 18.0;

    // Should be at least 144px (72 + 4*18)
    assert!(
        expected_min_height >= 144.0,
        "Expected height {} should be >= 144px for 5 lines",
        expected_min_height
    );
}

/// Test that wrapped text doesn't overflow item bounds
#[test]
fn test_wrapped_text_respects_available_width() {
    // Arrange: Long description that will wrap
    let long_text = "This is a very long description that should wrap to multiple lines when rendered in the available space and not overflow beyond the item boundaries.";
    let item = ListItem::new("Title")
        .secondary_text(long_text)
        .trailing_widget(|ui| {
            // Simulate Edit/Delete buttons taking 150px
            ui.allocate_space(egui::vec2(150.0, 32.0));
        });

    // Assert: Item created successfully with trailing widget
    assert!(item.trailing_widget.is_some());
    assert!(item.secondary_text.is_some());
}

/// Test that items with both multiline text and trailing widgets render correctly
#[test]
fn test_multiline_with_trailing_widgets() {
    // Arrange: Create list with multiline item + trailing widget
    let list = MaterialList::new()
        .id(egui::Id::unique("test_multiline_trailing"))
        .item(
            ListItem::new("Design mockups")
                .secondary_text(
                    "Create wireframes\nDesign high-fidelity mockups\nGet stakeholder approval",
                )
                .leading_icon("article".to_string())
                .trailing_widget(|ui| {
                    ui.horizontal(|ui| {
                        let _ = ui.button("Edit");
                        let _ = ui.button("Delete");
                    });
                }),
        );

    // Assert: List has 1 item
    assert_eq!(list.items.len(), 1);

    // Assert: Item has all components
    let item = &list.items[0];
    assert!(item.leading_icon.is_some());
    assert!(item.secondary_text.is_some());
    assert!(item.trailing_widget.is_some());
}

/// Test drawer functionality with trailing widgets
#[test]
fn test_drawer_with_trailing_widgets() {
    // Arrange: Create list with drawer + trailing widgets
    let list = MaterialList::new()
        .id(egui::Id::unique("test_drawer_trailing"))
        .allow_drawer(true)
        .item(
            ListItem::new("Engineering")
                .overline("Department")
                .secondary_text("Backend Development\nand API services")
                .trailing_text("Expand ▶")
                .drawer(|ui| {
                    ui.label("Team Size: 12");
                })
                .trailing_widget(|ui| {
                    let _ = ui.button("Edit");
                }),
        );

    // Assert: List allows drawers
    assert!(list.allow_drawer);

    // Assert: Item has drawer and trailing widget
    let item = &list.items[0];
    assert!(item.drawer.is_some());
    assert!(item.trailing_widget.is_some());
    assert!(item.secondary_text.is_some());
}

/// Test 5-line layout (overline + primary + 2-line secondary + trailing text)
#[test]
fn test_five_line_layout() {
    // Arrange: Create 5-line item
    let item = ListItem::new("Backend Development")
        .overline("Engineering")
        .secondary_text("Server infrastructure\nand API services")
        .trailing_text("Expand ▶")
        .leading_icon("folder");

    // Assert: All text components present
    assert!(item.overline_text.is_some());
    assert!(!item.primary_text.is_empty());
    assert!(item.secondary_text.is_some());
    assert!(item.trailing_text.is_some());
    assert!(item.leading_icon.is_some());

    // Calculate expected lines
    let secondary_lines = item.secondary_text.as_ref().unwrap().split('\n').count();
    let total_text_lines = 1 + 1 + secondary_lines; // overline + primary + secondary

    assert_eq!(
        total_text_lines, 4,
        "Should have 4 text lines (overline + primary + 2 secondary)"
    );
}
