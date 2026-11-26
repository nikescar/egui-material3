//! Tests for DataTable cell tooltip functionality

use egui_material3::DataTableTheme;

#[test]
fn test_tooltip_disabled_by_default() {
    let theme = DataTableTheme::default();

    // Tooltips should be disabled by default
    assert!(!theme.enable_cell_tooltips);
    assert!(!theme.smart_cell_tooltips);
}

#[test]
fn test_tooltip_always_enabled() {
    let theme = DataTableTheme {
        enable_cell_tooltips: true,
        ..Default::default()
    };

    assert!(theme.enable_cell_tooltips);
    assert!(!theme.smart_cell_tooltips);
}

#[test]
fn test_smart_tooltips() {
    let theme = DataTableTheme {
        smart_cell_tooltips: true,
        ..Default::default()
    };

    assert!(!theme.enable_cell_tooltips);
    assert!(theme.smart_cell_tooltips);
}

#[test]
fn test_both_tooltip_modes_can_coexist() {
    // If both are enabled, enable_cell_tooltips takes precedence (always show)
    let theme = DataTableTheme {
        enable_cell_tooltips: true,
        smart_cell_tooltips: true,
        ..Default::default()
    };

    assert!(theme.enable_cell_tooltips);
    assert!(theme.smart_cell_tooltips);
}

#[test]
fn test_tooltip_imports_available() {
    // Verify required tooltip functions are accessible
    use egui_material3::TooltipPosition;

    // Position variants should be available
    let _pos = TooltipPosition::Top;
    let _pos = TooltipPosition::Bottom;
    let _pos = TooltipPosition::Auto;
}
