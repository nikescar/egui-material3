//! Shared kittest harness for widget tests.
//!
//! `harness_for` builds a headless egui_kittest harness whose context holds
//! `theme` (via `MaterialTheme::set`), so tests never touch the global theme
//! and run in parallel. `themes` gives the light, dark and high-contrast
//! parametrisation every widget test should cover.

use egui_kittest::Harness;
use egui_material3::theme::{ContrastLevel, MaterialThemeContext};
use egui_material3::MaterialTheme;

/// A harness of `size` points drawing `app` with `theme`.
pub fn harness_for<'a>(
    theme: MaterialTheme,
    size: egui::Vec2,
    app: impl FnMut(&mut egui::Ui) + 'a,
) -> Harness<'a> {
    // Icon glyphs come from the Material Symbols font, which tests do not load.
    let mut harness = Harness::builder()
        .with_size(size)
        .allow_missing_glyphs()
        .build_ui(app);
    MaterialTheme::set(&harness.ctx, theme);
    harness.run();
    harness
}

/// Light, dark and dark high-contrast themes from the default theme file.
pub fn themes() -> Vec<(&'static str, MaterialTheme)> {
    let file = MaterialThemeContext::default().material_theme;
    let from = |dark, contrast| match &file {
        Some(file) => MaterialTheme::from_theme_file(file, dark, contrast),
        None => MaterialTheme::baseline(dark),
    };
    vec![
        ("light", from(false, ContrastLevel::Normal)),
        ("dark", from(true, ContrastLevel::Normal)),
        ("dark_high_contrast", from(true, ContrastLevel::High)),
    ]
}
