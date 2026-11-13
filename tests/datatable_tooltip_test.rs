//! Tests for DataTable cell tooltip functionality

use egui_material3::DataTableTheme;

#[test]
fn test_tooltip_disabled_by_default() {
    let theme = DataTableTheme::default();

    // Tooltips should be disabled by default
    assert_eq!(theme.enable_cell_tooltips, false);
    assert_eq!(theme.smart_cell_tooltips, false);
}

#[test]
fn test_tooltip_always_enabled() {
    let mut theme = DataTableTheme::default();
    theme.enable_cell_tooltips = true;

    assert_eq!(theme.enable_cell_tooltips, true);
    assert_eq!(theme.smart_cell_tooltips, false);
}

#[test]
fn test_smart_tooltips() {
    let mut theme = DataTableTheme::default();
    theme.smart_cell_tooltips = true;

    assert_eq!(theme.enable_cell_tooltips, false);
    assert_eq!(theme.smart_cell_tooltips, true);
}

#[test]
fn test_both_tooltip_modes_can_coexist() {
    // If both are enabled, enable_cell_tooltips takes precedence (always show)
    let mut theme = DataTableTheme::default();
    theme.enable_cell_tooltips = true;
    theme.smart_cell_tooltips = true;

    assert_eq!(theme.enable_cell_tooltips, true);
    assert_eq!(theme.smart_cell_tooltips, true);
}

#[test]
fn test_tooltip_imports_available() {
    // Verify required tooltip functions are accessible
    use egui_material3::TooltipPosition;

    // Position variants should be available
    let _pos = TooltipPosition::Top;
    let _pos = TooltipPosition::Bottom;
    let _pos = TooltipPosition::Auto;

    assert!(true, "Tooltip imports available");
}
