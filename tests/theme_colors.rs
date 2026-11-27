//! Theme colour behaviour, against per-context themes.
//!
//! These replace `examples/stories/theme_test.rs`, which flipped the global
//! theme's dark mode and so raced when run in parallel. Each test here
//! builds its own `MaterialTheme`; none touch the global theme.

use egui::Color32;
use egui_material3::theme::{ContrastLevel, MaterialThemeContext, MaterialThemeFile, ThemeMode};
use egui_material3::tokens::ColorRole;
use egui_material3::MaterialTheme;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn luminance(c: Color32) -> f32 {
    (0.299 * f32::from(c.r()) + 0.587 * f32::from(c.g()) + 0.114 * f32::from(c.b())) / 255.0
}

/// The default theme (the crate's built-in theme file) at light or dark.
fn default_theme(dark: bool) -> MaterialTheme {
    MaterialTheme::from_context(&MaterialThemeContext {
        theme_mode: ThemeMode::Auto,
        dark_mode: dark,
        ..Default::default()
    })
}

#[test]
fn dark_containers_are_lighter_than_surface() {
    let t = default_theme(true);
    let surface = luminance(t.color(ColorRole::Surface));
    for role in [ColorRole::SecondaryContainer, ColorRole::SurfaceVariant] {
        assert!(
            luminance(t.color(role)) > surface,
            "{role:?} lighter than surface in dark mode"
        );
    }
    let low = luminance(t.color(ColorRole::SurfaceContainerLow));
    let mid = luminance(t.color(ColorRole::SurfaceContainer));
    let high = luminance(t.color(ColorRole::SurfaceContainerHigh));
    assert!(
        low < mid && mid < high,
        "container hierarchy {low} < {mid} < {high}"
    );
}

#[test]
fn dark_content_is_visible_and_hierarchical() {
    let t = default_theme(true);
    for role in [
        ColorRole::OnSurface,
        ColorRole::OnSecondaryContainer,
        ColorRole::OnSurfaceVariant,
    ] {
        assert!(luminance(t.color(role)) > 0.2, "{role:?} visible on dark");
    }
    assert!(
        luminance(t.color(ColorRole::OnSurfaceVariant)) < luminance(t.color(ColorRole::OnSurface)),
        "onSurfaceVariant is dimmer than onSurface"
    );
    assert!(
        luminance(t.color(ColorRole::Surface)) < 0.5,
        "dark surface is dark"
    );
}

#[test]
fn light_content_is_visible() {
    let t = default_theme(false);
    assert!(
        luminance(t.color(ColorRole::OnSurface)) < 0.8,
        "onSurface visible on light"
    );
    assert!(
        luminance(t.color(ColorRole::SurfaceContainerHighest)) > 0.7,
        "light surfaceContainerHighest is light"
    );
}

#[test]
fn auto_mode_follows_the_dark_flag() {
    let light = default_theme(false);
    let dark = default_theme(true);
    assert!(
        !light.is_dark && dark.is_dark,
        "is_dark follows dark_mode in Auto"
    );
    assert_ne!(
        light.color(ColorRole::SurfaceContainerHighest),
        dark.color(ColorRole::SurfaceContainerHighest),
        "light and dark schemes differ"
    );
}

/// Two contexts hold different themes at once, and a context without one
/// falls back to the global theme.
#[test]
fn contexts_hold_independent_themes() {
    let a = egui::Context::default();
    let b = egui::Context::default();
    MaterialTheme::set(&a, MaterialTheme::baseline(false));
    MaterialTheme::set(&b, MaterialTheme::baseline(true));
    assert_ne!(
        MaterialTheme::of(&a).color(ColorRole::Primary),
        MaterialTheme::of(&b).color(ColorRole::Primary),
        "each context keeps its own theme"
    );
    MaterialTheme::clear(&b);
    let fallback = MaterialTheme::of(&b);
    assert_ne!(
        fallback.color(ColorRole::Primary),
        Color32::GRAY,
        "fallback resolves real colours"
    );
}

/// Every Theme Builder export loads with every role byte-identical to the
/// hex in the file, for all six schemes.
#[test]
fn json_themes_load_byte_identical() -> TestResult {
    let mut checked = 0;
    for entry in std::fs::read_dir("resources")? {
        let path = entry?.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if !(name.starts_with("material-theme") && name.ends_with(".json")) {
            continue;
        }
        let file: MaterialThemeFile = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
        for (dark, contrast, key) in [
            (false, ContrastLevel::Normal, "light"),
            (false, ContrastLevel::Medium, "light-medium-contrast"),
            (false, ContrastLevel::High, "light-high-contrast"),
            (true, ContrastLevel::Normal, "dark"),
            (true, ContrastLevel::Medium, "dark-medium-contrast"),
            (true, ContrastLevel::High, "dark-high-contrast"),
        ] {
            let Some(scheme) = file.schemes.get(key) else {
                continue;
            };
            let theme = MaterialTheme::from_theme_file(&file, dark, contrast);
            for role in ColorRole::ALL {
                let hex = MaterialThemeContext::color32_to_hex(theme.color(role));
                assert!(
                    hex.eq_ignore_ascii_case(scheme.hex(role)),
                    "{name} {key} {role:?}: {hex} vs {}",
                    scheme.hex(role)
                );
            }
            checked += 1;
        }
    }
    assert!(checked > 0, "found theme files under resources/");
    Ok(())
}
