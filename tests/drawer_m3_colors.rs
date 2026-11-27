//! Tests for Material Design 3 color role compliance in Drawer

use egui_material3::DrawerThemeData;

#[test]
fn test_drawer_material3_leaves_background_to_the_variant() {
    // Arrange & Act
    let theme = DrawerThemeData::material3_defaults();

    // Assert - M3 drawer background depends on the variant (surface for
    // standard drawers, surfaceContainerLow for modal ones) and is read from
    // the context's theme when drawn; tests/navigation.rs checks both.
    assert_eq!(theme.background_color, None);
}

#[test]
fn test_drawer_scrim_opacity() {
    // Arrange & Act
    let theme = DrawerThemeData::material3_defaults();

    // Assert - Unset: the scrim is the theme's scrim role @ 32% (ScrimTokens),
    // checked by tests/navigation.rs.
    assert_eq!(theme.scrim_color, None);
}

#[test]
fn test_drawer_elevation() {
    // Arrange & Act
    let theme = DrawerThemeData::material3_defaults();

    // Assert - M3 uses minimal elevation (1dp for modal/dismissible)
    assert_eq!(theme.elevation, Some(1.0));
}

#[test]
fn test_drawer_shape() {
    // Arrange & Act
    let theme = DrawerThemeData::material3_defaults();

    // Assert - Unset: square for standard drawers, CornerLargeEnd (16dp on
    // the edge facing the page) for modal ones, resolved when drawn.
    assert_eq!(theme.shape, None);
    assert_eq!(theme.end_shape, None);
}

#[test]
fn test_drawer_width() {
    // Arrange & Act
    let theme = DrawerThemeData::material3_defaults();

    // Assert - Standard width is 360dp
    assert_eq!(theme.width, Some(360.0));
}

#[test]
fn test_drawer_material2_differences() {
    // Arrange & Act
    let m3_theme = DrawerThemeData::material3_defaults();
    let m2_theme = DrawerThemeData::material2_defaults();

    // Assert - M2 uses surface, M3 uses surfaceContainerLow
    assert_ne!(m3_theme.background_color, m2_theme.background_color);

    // M2 has higher elevation
    assert!(m2_theme.elevation.unwrap() > m3_theme.elevation.unwrap());

    // M2 has no corner radius
    assert_ne!(m3_theme.shape, m2_theme.shape);
}
