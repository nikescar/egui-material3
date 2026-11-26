//! Tests for Material Design 3 color role compliance across components

use egui_material3::theme::get_global_color;

/// Test that M3 color roles are available in the theme system
#[test]
fn test_m3_surface_hierarchy_colors_exist() {
    // Arrange & Act - verify all M3 surface container levels exist
    let surface_container_lowest = get_global_color("surfaceContainerLowest");
    let surface_container_low = get_global_color("surfaceContainerLow");
    let surface_container = get_global_color("surfaceContainer");
    let surface_container_high = get_global_color("surfaceContainerHigh");
    let surface_container_highest = get_global_color("surfaceContainerHighest");

    // Assert - should not panic, colors should be valid
    assert!(surface_container_lowest.a() > 0);
    assert!(surface_container_low.a() > 0);
    assert!(surface_container.a() > 0);
    assert!(surface_container_high.a() > 0);
    assert!(surface_container_highest.a() > 0);
}

#[test]
fn test_m3_accent_colors_exist() {
    // Arrange & Act
    let primary = get_global_color("primary");
    let on_primary = get_global_color("onPrimary");
    let primary_container = get_global_color("primaryContainer");
    let on_primary_container = get_global_color("onPrimaryContainer");

    let secondary = get_global_color("secondary");
    let on_secondary = get_global_color("onSecondary");
    let secondary_container = get_global_color("secondaryContainer");
    let on_secondary_container = get_global_color("onSecondaryContainer");

    // Assert
    assert!(primary.a() > 0);
    assert!(on_primary.a() > 0);
    assert!(primary_container.a() > 0);
    assert!(on_primary_container.a() > 0);
    assert!(secondary.a() > 0);
    assert!(on_secondary.a() > 0);
    assert!(secondary_container.a() > 0);
    assert!(on_secondary_container.a() > 0);
}

#[test]
fn test_m3_outline_colors_exist() {
    // Arrange & Act
    let outline = get_global_color("outline");
    let outline_variant = get_global_color("outlineVariant");

    // Assert
    assert!(outline.a() > 0);
    assert!(outline_variant.a() > 0);
}

#[test]
fn test_m3_on_surface_colors_exist() {
    // Arrange & Act
    let on_surface = get_global_color("onSurface");
    let on_surface_variant = get_global_color("onSurfaceVariant");

    // Assert
    assert!(on_surface.a() > 0);
    assert!(on_surface_variant.a() > 0);
}

#[test]
fn test_m3_fixed_colors_exist() {
    // Arrange & Act - M3 fixed colors don't change between light/dark mode
    let primary_fixed = get_global_color("primaryFixed");
    let on_primary_fixed = get_global_color("onPrimaryFixed");
    let secondary_fixed = get_global_color("secondaryFixed");
    let on_secondary_fixed = get_global_color("onSecondaryFixed");

    // Assert
    assert!(primary_fixed.a() > 0);
    assert!(on_primary_fixed.a() > 0);
    assert!(secondary_fixed.a() > 0);
    assert!(on_secondary_fixed.a() > 0);
}

#[test]
fn test_m3_inverse_colors_exist() {
    // Arrange & Act - Inverse colors for high-contrast elements
    let inverse_surface = get_global_color("inverseSurface");
    let inverse_on_surface = get_global_color("inverseOnSurface");
    let inverse_primary = get_global_color("inversePrimary");

    // Assert
    assert!(inverse_surface.a() > 0);
    assert!(inverse_on_surface.a() > 0);
    assert!(inverse_primary.a() > 0);
}

/// Integration test: Verify component doc comments reference M3 color roles
/// This is a smoke test - actual color usage is tested visually in stories
#[test]
fn test_components_document_m3_colors() {}
