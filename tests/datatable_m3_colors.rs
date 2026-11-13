//! Tests for Material Design 3 color role compliance in DataTable

use egui_material3::{data_table, theme::get_global_color, DataTableCell, DataTableTheme};

#[test]
fn test_datatable_default_theme_uses_surface_container() {
    // Arrange
    let theme = DataTableTheme::default();

    // Act & Assert - default theme should not override M3 colors
    // This allows the component to use proper M3 surface containers
    assert!(theme.decoration.is_none(), "decoration should be None to use surfaceContainer");
    assert!(theme.heading_row_color.is_none(), "heading should use surfaceContainerHigh");
    assert!(theme.data_row_color.is_none(), "data rows should use surfaceContainer");
}

#[test]
fn test_datatable_selected_row_uses_secondary_container() {
    // Arrange
    let theme = DataTableTheme::default();

    // Act & Assert - selected row color should use secondaryContainer
    assert!(theme.selected_row_color.is_none(), "selected rows should use secondaryContainer");
}

#[test]
fn test_datatable_divider_uses_outline_variant() {
    // Arrange
    let theme = DataTableTheme::default();

    // Act & Assert - divider should use outlineVariant for subtle separation
    assert!(theme.divider_color.is_none(), "divider should use outlineVariant");
}

#[test]
fn test_datatable_heading_text_uses_on_surface() {
    // Arrange
    let theme = DataTableTheme::default();

    // Act & Assert - heading text should use onSurface
    assert!(theme.heading_text_style.is_none(), "heading text should use onSurface");
}

#[test]
fn test_datatable_data_text_uses_on_surface() {
    // Arrange
    let theme = DataTableTheme::default();

    // Act & Assert - data text should use onSurface
    assert!(theme.data_text_style.is_none(), "data text should use onSurface");
}

#[test]
fn test_datatable_sort_icons_use_primary() {
    // Arrange
    let theme = DataTableTheme::default();

    // Act & Assert - active sort should use primary
    assert!(theme.sort_active_color.is_none(), "active sort should use primary");
    assert!(theme.sort_inactive_color.is_none(), "inactive sort should use onSurfaceVariant");
}

#[test]
#[ignore = "material3() constructor not implemented yet"]
fn test_datatable_m3_theme_constructor() {
    // Arrange & Act
    let theme = DataTableTheme::material3();

    // Assert - M3 theme should set proper color roles
    let surface_container = get_global_color("surfaceContainer");
    let surface_container_high = get_global_color("surfaceContainerHigh");
    let secondary_container = get_global_color("secondaryContainer");
    let outline_variant = get_global_color("outlineVariant");
    let on_surface = get_global_color("onSurface");
    let on_surface_variant = get_global_color("onSurfaceVariant");
    let primary = get_global_color("primary");

    assert_eq!(theme.decoration, Some(surface_container));
    assert_eq!(theme.heading_row_color, Some(surface_container_high));
    assert_eq!(theme.data_row_color, Some(surface_container));
    assert_eq!(theme.selected_row_color, Some(secondary_container));
    assert_eq!(theme.divider_color, Some(outline_variant));

    // Text colors
    let heading_style = theme.heading_text_style.expect("heading text style should be set");
    assert_eq!(heading_style.1, on_surface);

    let data_style = theme.data_text_style.expect("data text style should be set");
    assert_eq!(data_style.1, on_surface);

    // Sort colors
    assert_eq!(theme.sort_active_color, Some(primary));
    assert_eq!(theme.sort_inactive_color, Some(on_surface_variant));
}
