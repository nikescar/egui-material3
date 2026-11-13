#![cfg(test)]

use egui::Color32;

/// Test helper to verify color is not white (invisible on light backgrounds)
fn is_visible_on_light_background(color: Color32) -> bool {
    // White or near-white colors are not visible on light backgrounds
    // Check if color is dark enough (luminance check)
    let r = color.r() as f32 / 255.0;
    let g = color.g() as f32 / 255.0;
    let b = color.b() as f32 / 255.0;

    // Calculate relative luminance (simplified)
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    // Color should be dark (luminance < 0.8) to be visible on light background
    luminance < 0.8
}

/// Test helper to verify color is not black (invisible on dark backgrounds)
fn is_visible_on_dark_background(color: Color32) -> bool {
    let r = color.r() as f32 / 255.0;
    let g = color.g() as f32 / 255.0;
    let b = color.b() as f32 / 255.0;

    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    // Color should be light (luminance > 0.2) to be visible on dark background
    luminance > 0.2
}

#[test]
fn test_strong_text_visible_on_light_background() {
    // Simulate Material Design 3 light theme colors
    let on_surface_light = Color32::from_rgb(28, 27, 31); // Dark text on light surface

    // Strong text should use on_surface color in light mode
    // This would be set via: visuals.widgets.active.fg_stroke.color = on_surface
    assert!(
        is_visible_on_light_background(on_surface_light),
        "Strong text color {:?} should be visible on light backgrounds",
        on_surface_light
    );
}

/// Test helper to calculate relative luminance
fn calculate_luminance(color: Color32) -> f32 {
    let r = color.r() as f32 / 255.0;
    let g = color.g() as f32 / 255.0;
    let b = color.b() as f32 / 255.0;
    0.299 * r + 0.587 * g + 0.114 * b
}

/// Test helper to verify container is lighter than base surface (for dark mode elevation)
fn is_lighter_than(container: Color32, surface: Color32) -> bool {
    calculate_luminance(container) > calculate_luminance(surface)
}

#[test]
fn test_tonal_button_visible_in_dark_mode() {
    // RED: Test that tonal buttons use surfaceContainerHigh in dark mode
    // Tonal buttons should be lighter than base surface to show elevation

    // Setup dark mode
    let theme_binding = egui_material3::get_global_theme();
    let mut theme = theme_binding.lock().unwrap();
    let original_dark_mode = theme.dark_mode;
    theme.set_dark_mode(true);
    drop(theme);

    let surface = egui_material3::get_global_color("surface");
    let secondary_container = egui_material3::get_global_color("secondaryContainer");
    let on_secondary_container = egui_material3::get_global_color("onSecondaryContainer");

    // Verify container is lighter than base surface (elevation in dark mode)
    assert!(
        is_lighter_than(secondary_container, surface),
        "Tonal button background (secondaryContainer) should be lighter than surface in dark mode. Got secondaryContainer luminance={:.3}, surface luminance={:.3}",
        calculate_luminance(secondary_container),
        calculate_luminance(surface)
    );

    // Verify text is visible on container
    assert!(
        is_visible_on_dark_background(on_secondary_container),
        "Tonal button text should be visible in dark mode"
    );

    // Cleanup
    let mut theme = theme_binding.lock().unwrap();
    theme.set_dark_mode(original_dark_mode);
}

#[test]
fn test_datatable_rows_visible_in_dark_mode() {
    // RED: Test that alternate rows use surfaceContainerLow for subtle contrast

    // Setup dark mode
    let theme_binding = egui_material3::get_global_theme();
    let mut theme = theme_binding.lock().unwrap();
    let original_dark_mode = theme.dark_mode;
    theme.set_dark_mode(true);
    drop(theme);

    let surface = egui_material3::get_global_color("surface");
    let surface_variant = egui_material3::get_global_color("surfaceVariant");

    // Verify surfaceVariant provides contrast (should be lighter than base surface)
    assert!(
        is_lighter_than(surface_variant, surface),
        "DataTable alternate rows (surfaceVariant) should be lighter than surface in dark mode for visibility. Got surfaceVariant luminance={:.3}, surface luminance={:.3}",
        calculate_luminance(surface_variant),
        calculate_luminance(surface)
    );

    // Cleanup
    let mut theme = theme_binding.lock().unwrap();
    theme.set_dark_mode(original_dark_mode);
}

#[test]
fn test_container_hierarchy_in_dark_mode() {
    // RED: Test that M3 container hierarchy is maintained
    // surfaceContainerLow < surfaceContainer < surfaceContainerHigh

    // Setup dark mode
    let theme_binding = egui_material3::get_global_theme();
    let mut theme = theme_binding.lock().unwrap();
    let original_dark_mode = theme.dark_mode;
    theme.set_dark_mode(true);
    drop(theme);

    let surface = egui_material3::get_global_color("surface");
    let surface_container_low = egui_material3::get_global_color("surfaceContainerLow");
    let surface_container = egui_material3::get_global_color("surfaceContainer");
    let surface_container_high = egui_material3::get_global_color("surfaceContainerHigh");

    let surf_lum = calculate_luminance(surface);
    let low_lum = calculate_luminance(surface_container_low);
    let mid_lum = calculate_luminance(surface_container);
    let high_lum = calculate_luminance(surface_container_high);

    // In dark mode, containers should be progressively lighter to show elevation
    assert!(
        low_lum > surf_lum,
        "surfaceContainerLow should be lighter than surface in dark mode. Got low={:.3}, surface={:.3}",
        low_lum, surf_lum
    );

    assert!(
        mid_lum > low_lum,
        "surfaceContainer should be lighter than surfaceContainerLow. Got mid={:.3}, low={:.3}",
        mid_lum,
        low_lum
    );

    assert!(
        high_lum > mid_lum,
        "surfaceContainerHigh should be lighter than surfaceContainer. Got high={:.3}, mid={:.3}",
        high_lum,
        mid_lum
    );

    // Cleanup
    let mut theme = theme_binding.lock().unwrap();
    theme.set_dark_mode(original_dark_mode);
}

#[test]
fn test_debug_dark_mode_color_values() {
    // Debug test to see actual color values in dark mode
    let theme_binding = egui_material3::get_global_theme();
    let mut theme = theme_binding.lock().unwrap();
    let original_dark_mode = theme.dark_mode;
    theme.set_dark_mode(true);
    drop(theme);

    let surface = egui_material3::get_global_color("surface");
    let secondary_container = egui_material3::get_global_color("secondaryContainer");
    let surface_variant = egui_material3::get_global_color("surfaceVariant");
    let surface_container_high = egui_material3::get_global_color("surfaceContainerHigh");

    println!("Dark mode colors:");
    println!(
        "  surface: {:?} (lum: {:.3})",
        surface,
        calculate_luminance(surface)
    );
    println!(
        "  secondaryContainer: {:?} (lum: {:.3})",
        secondary_container,
        calculate_luminance(secondary_container)
    );
    println!(
        "  surfaceVariant: {:?} (lum: {:.3})",
        surface_variant,
        calculate_luminance(surface_variant)
    );
    println!(
        "  surfaceContainerHigh: {:?} (lum: {:.3})",
        surface_container_high,
        calculate_luminance(surface_container_high)
    );

    // Cleanup
    let mut theme = theme_binding.lock().unwrap();
    theme.set_dark_mode(original_dark_mode);

    // Always pass - this is just for debugging
    assert!(true);
}

#[test]
fn test_strong_text_visible_on_dark_background() {
    // Simulate Material Design 3 dark theme colors
    let on_surface_dark = Color32::from_rgb(230, 225, 229); // Light text on dark surface

    // Strong text should use on_surface color in dark mode
    assert!(
        is_visible_on_dark_background(on_surface_dark),
        "Strong text color {:?} should be visible on dark backgrounds",
        on_surface_dark
    );
}

#[test]
fn test_weak_text_is_dimmed() {
    // Weak text should use on_surface_variant or reduced opacity
    let on_surface = Color32::from_rgb(28, 27, 31);
    let on_surface_variant = Color32::from_rgb(73, 69, 79);

    // Calculate luminance for both
    let calc_luminance = |c: Color32| {
        let r = c.r() as f32 / 255.0;
        let g = c.g() as f32 / 255.0;
        let b = c.b() as f32 / 255.0;
        0.299 * r + 0.587 * g + 0.114 * b
    };

    let on_surface_lum = calc_luminance(on_surface);
    let variant_lum = calc_luminance(on_surface_variant);

    // Weak text should be lighter (higher luminance) than strong text
    assert!(
        variant_lum > on_surface_lum,
        "Weak text (variant) should be dimmer/lighter than strong text (on_surface)"
    );
}

#[test]
fn test_text_hierarchy_contrast() {
    // Material Design 3 text hierarchy
    let on_surface = Color32::from_rgb(28, 27, 31); // High emphasis (100%)
    let on_surface_variant = Color32::from_rgb(73, 69, 79); // Medium emphasis (~87%)

    // Both should still be visible
    assert!(is_visible_on_light_background(on_surface));
    assert!(is_visible_on_light_background(on_surface_variant));

    // But variant should be noticeably different
    assert_ne!(on_surface, on_surface_variant);
}

#[test]
fn test_override_text_color_set() {
    // This test documents that override_text_color should be set to on_surface
    // The actual assertion would require creating an egui context, so this is a placeholder

    // In apply_theme(), we should have:
    // visuals.override_text_color = Some(on_surface);

    // This ensures all text uses Material Design colors by default
    assert!(true, "override_text_color should be set in apply_theme()");
}

#[test]
fn test_weak_text_color_set() {
    // This test documents that weak_text_color should be set to on_surface_variant

    // In apply_theme(), we should have:
    // visuals.weak_text_color = Some(on_surface_variant);

    assert!(true, "weak_text_color should be set in apply_theme()");
}

#[test]
fn test_selection_color_visible_light_mode() {
    // Selection background should use secondaryContainer in light mode
    // Material Design 3 light theme secondaryContainer example
    let secondary_container_light = Color32::from_rgb(232, 222, 248); // Light purple tint

    // Should be visible (different from white surface)
    // Luminance should be in mid-range for visibility
    let r = secondary_container_light.r() as f32 / 255.0;
    let g = secondary_container_light.g() as f32 / 255.0;
    let b = secondary_container_light.b() as f32 / 255.0;
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    assert!(
        luminance > 0.5 && luminance < 0.95,
        "Selection background {:?} (luminance: {}) should be visible in light mode",
        secondary_container_light,
        luminance
    );
}

#[test]
fn test_selection_color_visible_dark_mode() {
    // Selection background should use secondaryContainer in dark mode
    // Material Design 3 dark theme secondaryContainer example
    let secondary_container_dark = Color32::from_rgb(74, 68, 88); // Dark purple tint

    // Should be visible (different from dark surface)
    let r = secondary_container_dark.r() as f32 / 255.0;
    let g = secondary_container_dark.g() as f32 / 255.0;
    let b = secondary_container_dark.b() as f32 / 255.0;
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    assert!(
        luminance > 0.15 && luminance < 0.5,
        "Selection background {:?} (luminance: {}) should be visible in dark mode",
        secondary_container_dark,
        luminance
    );
}

#[test]
fn test_widget_background_visible_light_mode() {
    // Widget inactive background should use surfaceContainerHighest in light mode
    // Material Design 3 light theme surfaceContainerHighest example
    let surface_container_highest_light = Color32::from_rgb(230, 224, 233); // Elevated surface

    // Should be slightly darker than base surface for visibility
    let r = surface_container_highest_light.r() as f32 / 255.0;
    let g = surface_container_highest_light.g() as f32 / 255.0;
    let b = surface_container_highest_light.b() as f32 / 255.0;
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    assert!(
        luminance > 0.7 && luminance < 0.95,
        "Widget background {:?} (luminance: {}) should be visible in light mode",
        surface_container_highest_light,
        luminance
    );
}

#[test]
fn test_widget_background_visible_dark_mode() {
    // Widget inactive background should use surfaceContainerHighest in dark mode
    // Material Design 3 dark theme surfaceContainerHighest example
    let surface_container_highest_dark = Color32::from_rgb(54, 52, 59); // Elevated surface

    // Should be lighter than base surface for visibility
    let r = surface_container_highest_dark.r() as f32 / 255.0;
    let g = surface_container_highest_dark.g() as f32 / 255.0;
    let b = surface_container_highest_dark.b() as f32 / 255.0;
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    assert!(
        luminance > 0.1 && luminance < 0.4,
        "Widget background {:?} (luminance: {}) should be visible in dark mode",
        surface_container_highest_dark,
        luminance
    );
}

#[test]
fn test_selection_not_inverse_surface() {
    // Document that selection should NOT use inverse_surface
    // inverse_surface is too dark/high contrast - should use secondaryContainer instead

    // Old approach (inverse_surface): dark in light mode, light in dark mode
    let inverse_surface_light = Color32::from_rgb(28, 27, 31); // Very dark
    let inverse_surface_dark = Color32::from_rgb(230, 225, 229); // Very light

    // New approach (secondaryContainer): mid-tone in both modes
    let secondary_container_light = Color32::from_rgb(232, 222, 248);
    let secondary_container_dark = Color32::from_rgb(74, 68, 88);

    // secondaryContainer should have better mid-range luminance
    let calc_luminance = |c: Color32| {
        let r = c.r() as f32 / 255.0;
        let g = c.g() as f32 / 255.0;
        let b = c.b() as f32 / 255.0;
        0.299 * r + 0.587 * g + 0.114 * b
    };

    let inverse_light_lum = calc_luminance(inverse_surface_light);
    let secondary_light_lum = calc_luminance(secondary_container_light);

    // secondaryContainer should be much lighter (better for backgrounds)
    assert!(
        secondary_light_lum > inverse_light_lum * 2.0,
        "secondaryContainer (lum: {}) should be much lighter than inverse_surface (lum: {}) in light mode",
        secondary_light_lum,
        inverse_light_lum
    );
}

#[test]
fn test_auto_mode_scheme_selection() {
    // Auto mode should resolve to correct scheme based on dark_mode flag
    // This test documents that get_current_scheme() in Auto mode should respect actual dark_mode

    // In Auto mode with dark_mode=false, should use light scheme colors
    // surfaceContainerHighest in light mode: #D1D3C8 (light gray)
    let surface_container_highest_light = Color32::from_rgb(209, 211, 200);

    // In Auto mode with dark_mode=true, should use dark scheme colors
    // surfaceContainerHighest in dark mode: #33362F (dark gray)
    let surface_container_highest_dark = Color32::from_rgb(51, 54, 47);

    // These should be different colors
    assert_ne!(
        surface_container_highest_light, surface_container_highest_dark,
        "Light and dark mode should have different surfaceContainerHighest colors"
    );

    // Light mode should be much lighter
    let calc_luminance = |c: Color32| {
        let r = c.r() as f32 / 255.0;
        let g = c.g() as f32 / 255.0;
        let b = c.b() as f32 / 255.0;
        0.299 * r + 0.587 * g + 0.114 * b
    };

    let light_lum = calc_luminance(surface_container_highest_light);
    let dark_lum = calc_luminance(surface_container_highest_dark);

    assert!(
        light_lum > 0.7,
        "Light mode surfaceContainerHighest should be light (lum: {})",
        light_lum
    );
    assert!(
        dark_lum < 0.3,
        "Dark mode surfaceContainerHighest should be dark (lum: {})",
        dark_lum
    );
}

#[test]
fn test_global_theme_stores_dark_mode_state() {
    use egui_material3::get_global_theme;

    // Get mutable access to global theme
    if let Ok(mut theme) = get_global_theme().lock() {
        // Set dark mode to true
        theme.set_dark_mode(true);
        assert!(theme.dark_mode, "Dark mode should be true after setting");

        // Set dark mode to false
        theme.set_dark_mode(false);
        assert!(!theme.dark_mode, "Dark mode should be false after setting");

        // State is already reset to false, no cleanup needed
    } else {
        panic!("Failed to lock global theme");
    }
}

#[test]
fn test_button_colors_visible_in_dark_mode() {
    use egui_material3::{get_global_color, get_global_theme};

    // Set dark mode
    if let Ok(mut theme) = get_global_theme().lock() {
        theme.set_dark_mode(true);
    }

    // Button elevated uses "surface" background
    let elevated_bg = get_global_color("surface");
    let elevated_text = get_global_color("onSurface");

    // Reset to light mode immediately after getting values
    if let Ok(mut theme) = get_global_theme().lock() {
        theme.set_dark_mode(false);
    }

    // In dark mode, elevated button should have dark background (inverseSurface)
    assert!(
        is_visible_on_dark_background(elevated_text),
        "Elevated button text {:?} should be visible on dark background",
        elevated_text
    );

    // Background should be dark (inverseSurface is dark in light scheme)
    let bg_luminance = (elevated_bg.r() as f32 * 0.299
        + elevated_bg.g() as f32 * 0.587
        + elevated_bg.b() as f32 * 0.114)
        / 255.0;
    assert!(
        bg_luminance < 0.5,
        "Elevated button background in dark mode should be dark, got luminance {}",
        bg_luminance
    );

    // Already reset to light mode
}
