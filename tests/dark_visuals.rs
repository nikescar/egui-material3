use egui_material3::theme::{apply_theme, set_theme_mode, ThemeMode};

fn lum(c: egui::Color32) -> u32 {
    c.r() as u32 + c.g() as u32 + c.b() as u32
}

#[test]
fn dark_mode_visuals_use_dark_scheme() {
    explicit_modes();
    auto_mode_follows_detector();
}

fn explicit_modes() {
    let ctx = egui::Context::default();
    set_theme_mode(ThemeMode::Light);
    apply_theme(&ctx, None::<fn() -> ThemeMode>);
    let light = ctx.global_style().visuals.panel_fill;
    set_theme_mode(ThemeMode::Dark);
    apply_theme(&ctx, None::<fn() -> ThemeMode>);
    let dark = ctx.global_style().visuals.panel_fill;
    println!("light={light:?} dark={dark:?}");
    assert!(lum(dark) < lum(light), "panel_fill not darker in Dark mode");
}

// Global theme state is shared, so run as one sequential test.
fn auto_mode_follows_detector() {
    let ctx = egui::Context::default();
    set_theme_mode(ThemeMode::Auto);
    apply_theme(&ctx, Some(|| ThemeMode::Dark));
    let dark = ctx.global_style().visuals.panel_fill;
    apply_theme(&ctx, Some(|| ThemeMode::Light));
    let light = ctx.global_style().visuals.panel_fill;
    assert!(
        lum(dark) < lum(light),
        "Auto+Dark detector not dark: {dark:?} vs {light:?}"
    );
}
