use egui_material3::DataTableTheme;

#[test]
fn test_header_font_size_default() {
    let theme = DataTableTheme::default();
    // Default should be None, allowing rendering.rs to use 16.0
    assert!(theme.header_font_size.is_none());
}

#[test]
fn test_header_font_size_custom() {
    let mut theme = DataTableTheme::default();
    theme.header_font_size = Some(20.0);

    // Test ensures the API exists and compiles
    assert_eq!(theme.header_font_size, Some(20.0));
}

#[test]
fn test_header_font_size_builder_chain() {
    let mut theme = DataTableTheme::default();
    theme.header_font_size = Some(18.0);
    theme.heading_row_height = Some(64.0);

    assert_eq!(theme.header_font_size, Some(18.0));
    assert_eq!(theme.heading_row_height, Some(64.0));
}
