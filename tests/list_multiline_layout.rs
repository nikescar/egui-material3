use egui_material3::{ListItem, MaterialList};

#[test]
fn test_trailing_widget_structure() {
    // Structural test: verify ListItem accepts trailing_widget with multiline text
    // The actual positioning fix is validated visually in examples/stories

    let _list = MaterialList::new().id(egui::Id::unique("test_list")).item(
        ListItem::new("Short Title")
            .secondary_text("Multiline\nDescription\nText")
            .trailing_widget(|ui| {
                ui.label("Button");
            }),
    );
}

#[test]
fn test_trailing_text_top_right_structure() {
    // Structural test: verify ListItem accepts both trailing_text and trailing_widget
    // Position verification is done visually in examples

    let _list = MaterialList::new()
        .id(egui::Id::unique("test_trailing_text"))
        .item(
            ListItem::new("Department")
                .secondary_text("Description text")
                .trailing_text("Expand ▶")
                .trailing_widget(|ui| {
                    ui.label("Edit");
                }),
        );
}

#[test]
fn test_trailing_text_and_widget_no_overlap_structure() {
    // Structural test: verify both trailing_text and trailing_widget can coexist
    // Visual overlap check is done in examples/stories

    let _list = MaterialList::new()
        .id(egui::Id::unique("test_no_overlap"))
        .item(
            ListItem::new("Title")
                .secondary_text("Description")
                .trailing_text("Status")
                .trailing_widget(|ui| {
                    ui.horizontal(|ui| {
                        let _ = ui.button("Edit");
                        let _ = ui.button("Delete");
                    });
                }),
        );
}

#[test]
fn test_multiline_secondary_text_structure() {
    // Structural test: verify multiline secondary text compiles
    // Padding balance is validated visually in examples

    let _list = MaterialList::new()
        .id(egui::Id::unique("test_padding"))
        .item(ListItem::new("Title").secondary_text("Line 1\nLine 2\nLine 3"));
}

#[test]
fn test_wrapped_text_with_trailing_widget() {
    // Test that very long text that wraps doesn't break with trailing widgets
    let long_text = "This is a very long description that will definitely wrap to multiple lines when rendered in a normal width list item, and it should handle the wrapping gracefully without breaking the layout.";

    let _list = MaterialList::new()
        .id(egui::Id::unique("test_wrapped"))
        .item(
            ListItem::new("Title")
                .secondary_text(long_text)
                .trailing_widget(|ui| {
                    ui.horizontal(|ui| {
                        let _ = ui.button("Edit");
                        let _ = ui.button("Delete");
                    });
                }),
        );
}
