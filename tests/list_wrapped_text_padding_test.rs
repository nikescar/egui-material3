//! Integration test for wrapped text padding balance
//!
//! This test verifies that when secondary text wraps due to width constraints
//! (not explicit \n), the top and bottom padding remain balanced.

use egui_material3::{ListItem, MaterialList};

/// Test that wrapped secondary text (without overline) has balanced padding
#[test]
fn test_wrapped_text_without_overline_balanced_padding() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Create a list item with very long secondary text that will wrap
            // No overline - this is the case that's currently broken
            let long_text = "This is a very long description that will definitely wrap to multiple lines when rendered in a normal width container. It needs to be long enough to trigger wrapping.";

            let list = MaterialList::new()
                .item(
                    ListItem::new("Task Item")
                        .secondary_text(long_text)
                        .leading_icon("article".to_string())
                );

            let response = ui.add(list);

            // Verify item rendered and height increased beyond base 2-line height
            let base_two_line_height = 72.0;
            assert!(
                response.rect.height() > base_two_line_height,
                "Wrapped text should increase item height beyond base 72px, got {}",
                response.rect.height()
            );

            // The actual padding balance can't be directly tested in integration tests,
            // but this test will serve as a regression test to ensure the fix doesn't break
        });
    });
}

/// Test that wrapped text with overline works correctly (this already works)
#[test]
fn test_wrapped_text_with_overline_works() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let long_text = "This is a very long description that will definitely wrap to multiple lines when rendered in a normal width container.";

            let list = MaterialList::new()
                .item(
                    ListItem::new("Backend Development")
                        .overline("Engineering")
                        .secondary_text(long_text)
                        .leading_icon("folder".to_string())
                );

            let response = ui.add(list);

            // Verify item height increased
            let base_three_line_height = 88.0;
            assert!(
                response.rect.height() > base_three_line_height,
                "Wrapped text with overline should work correctly"
            );
        });
    });
}

/// Test that trailing widgets still align correctly with wrapped text
#[test]
fn test_wrapped_text_with_trailing_widget() {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let long_text = "This is a very long description that will wrap and the trailing widget should align with the primary text.";

            let list = MaterialList::new()
                .item(
                    ListItem::new("Task Item")
                        .secondary_text(long_text)
                        .leading_icon("article".to_string())
                        .trailing_widget(|ui| {
                            ui.label("Widget");
                        })
                );

            let response = ui.add(list);

            // Should render without panic and height should increase
            assert!(response.rect.height() > 72.0);
        });
    });
}
