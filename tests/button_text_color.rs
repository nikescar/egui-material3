// Tests for Filled button text color in light and dark modes
// Verifies Material Design 3 spec compliance

use egui_material3::theme::{get_global_color, get_global_theme, MaterialThemeContext, ThemeMode};

#[test]
fn test_filled_button_text_color_light_mode() {
    // Arrange - Set light mode
    let mut theme = MaterialThemeContext::default();
    theme.dark_mode = false;
    egui_material3::theme::update_global_theme(theme);

    // Act - Get the color that Filled buttons should use
    let on_primary = get_global_color("onPrimary");
    let on_primary_fixed = get_global_color("onPrimaryFixed");

    // Assert - In light mode, onPrimary and onPrimaryFixed should be different
    // This test verifies the theme system has the colors we need
    assert_ne!(
        on_primary, on_primary_fixed,
        "onPrimary and onPrimaryFixed should differ in light mode"
    );
}

#[test]
fn test_filled_button_text_color_dark_mode() {
    // Arrange - Set dark mode
    let mut theme = MaterialThemeContext::default();
    theme.dark_mode = true;
    egui_material3::theme::update_global_theme(theme);

    // Act - Get the colors
    let on_primary = get_global_color("onPrimary");
    let on_primary_fixed = get_global_color("onPrimaryFixed");

    // Assert - In dark mode, colors should be different
    // Filled buttons should use onPrimaryFixed (not onPrimary)
    assert_ne!(
        on_primary, on_primary_fixed,
        "onPrimary and onPrimaryFixed should differ in dark mode"
    );
}

#[test]
fn test_theme_dark_mode_detection() {
    // Arrange
    let mut theme = MaterialThemeContext::default();

    // Act & Assert - Light mode
    theme.dark_mode = false;
    egui_material3::theme::update_global_theme(theme.clone());

    if let Ok(global) = get_global_theme().lock() {
        assert_eq!(global.dark_mode, false, "Should be in light mode");
    }

    // Act & Assert - Dark mode
    theme.dark_mode = true;
    egui_material3::theme::update_global_theme(theme);

    if let Ok(global) = get_global_theme().lock() {
        assert_eq!(global.dark_mode, true, "Should be in dark mode");
    }
}
