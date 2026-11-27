//! Typography and icon fonts: the bundled Roboto weights and the type
//! scale's text styles, and Material Symbols icons (glyph lookup, filled,
//! weight, RTL mirroring, SVG tint), with snapshots of a type specimen and
//! an icon grid.

mod support;

use egui::{Color32, FontFamily, FontId, TextStyle};
use egui_kittest::{Harness, SnapshotError, SnapshotOptions};
use egui_material3::fonts::{self, symbol_family_name, SymbolStyle};
use egui_material3::icon::{resolve_symbol, Icon};
use egui_material3::material_symbol::{get_material_symbol, ICON_HOME, ICON_NAMES};
use egui_material3::tokens::typography::{text_styles_with, weight_family_name};
use egui_material3::tokens::{ColorRole, LayoutDirection, TypeRole};
use egui_material3::{icon, MaterialTheme};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// A harness with the bundled fonts installed before the first frame, and
/// no allowance for missing glyphs: an icon whose glyph is not in the
/// fonts fails the test.
fn harness<'a>(
    theme: MaterialTheme,
    size: egui::Vec2,
    app: impl FnMut(&mut egui::Ui) + 'a,
) -> Harness<'a> {
    let mut harness = Harness::builder()
        .with_size(size)
        .with_context_setup(move |ctx| {
            fonts::install(ctx);
            MaterialTheme::set(ctx, theme);
        })
        .build_ui(app);
    harness.run();
    harness
}

/// Rendering tests take this lock: several wgpu renderers created at once
/// in one process can crash the test binary on Windows.
fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The rendered image, or `None` when there is no renderer (headless CI
/// without a software adapter).
fn render(harness: &mut Harness<'_>) -> Result<Option<image::RgbaImage>, String> {
    match harness.render() {
        Ok(image) => Ok(Some(image)),
        Err(err) if err.contains("adapter") || err.contains("Adapter") => {
            eprintln!("skipping pixel check, no renderer: {err}");
            Ok(None)
        }
        Err(err) => Err(err),
    }
}

/// The dark (icon-coloured) pixels: the tests draw `onSurface` icons on a
/// light theme, so anything darker than mid-grey is ink.
fn ink(image: &image::RgbaImage) -> Vec<(u32, u32)> {
    image
        .enumerate_pixels()
        .filter(|(_, _, p)| u32::from(p.0[0]) + u32::from(p.0[1]) + u32::from(p.0[2]) < 300)
        .map(|(x, y, _)| (x, y))
        .collect()
}

/// Fills the whole window (the harness panel's margins too) with `color`.
fn fill_window(ui: &egui::Ui, color: Color32) {
    let window = egui::Rect::from_min_size(egui::pos2(-100.0, -100.0), egui::vec2(4000.0, 4000.0));
    ui.painter()
        .with_clip_rect(window)
        .rect_filled(window, 0.0, color);
}

fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let options = SnapshotOptions::new().threshold(2.0).max_failed_pixels(200);
    match harness.try_snapshot_options(name, &options) {
        Ok(()) => Ok(()),
        Err(SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

fn icon_app(name: &'static str, size: f32, filled: bool, weight: u16) -> impl FnMut(&mut egui::Ui) {
    move |ui| {
        let bg = MaterialTheme::of(ui.ctx()).color(ColorRole::Surface);
        fill_window(ui, bg);
        ui.add(icon(name).size(size).filled(filled).weight(weight));
    }
}

#[test]
fn every_weight_family_has_its_roboto_face() {
    let defs = fonts::material_font_definitions();
    for (weight, face, _) in fonts::ROBOTO {
        let family = FontFamily::Name(weight_family_name(weight).into());
        let first = defs.families.get(&family).and_then(|list| list.first());
        assert_eq!(
            first.map(String::as_str),
            Some(face),
            "plain-{weight} starts with {face}"
        );
    }
    let proportional = defs.families.get(&FontFamily::Proportional);
    assert_eq!(
        proportional.and_then(|l| l.first()).map(String::as_str),
        Some("Roboto-Regular"),
        "Roboto is the default proportional font"
    );
    let outlined = symbol_family_name(SymbolStyle::Outlined, false, 400);
    assert!(
        proportional.is_some_and(|l| l.last() == Some(&outlined)),
        "Material Symbols is the last proportional fallback"
    );
    for filled in [false, true] {
        let family = fonts::symbol_family(SymbolStyle::Outlined, filled, 400);
        assert!(
            defs.families.contains_key(&family),
            "Outlined fill {filled} registered"
        );
    }
}

#[test]
fn label_large_renders_medium_and_wider_than_regular() -> TestResult {
    let harness = harness(
        MaterialTheme::baseline(false),
        egui::vec2(10.0, 10.0),
        |_| {},
    );
    let width = |weight: u16| {
        let format = egui::TextFormat {
            font_id: FontId::new(14.0, FontFamily::Name(weight_family_name(weight).into())),
            ..Default::default()
        };
        let job = egui::text::LayoutJob::single_section("Label Large".to_owned(), format);
        harness.ctx.fonts_mut(|f| f.layout_job(job)).size().x
    };
    let (regular, medium, bold) = (width(400), width(500), width(700));
    assert!(
        regular < medium && medium < bold,
        "glyph advances grow with weight: 400 {regular}, 500 {medium}, 700 {bold}"
    );
    let label_large = TypeRole::LabelLarge.style();
    assert_eq!(label_large.weight, 500, "Label Large is Medium");
    let font = label_large.font_id(&harness.ctx);
    assert_eq!(
        font.family,
        FontFamily::Name("plain-500".into()),
        "Label Large uses the registered 500 family"
    );
    Ok(())
}

#[test]
fn label_large_line_height_is_20() {
    let harness = harness(
        MaterialTheme::baseline(false),
        egui::vec2(10.0, 10.0),
        |_| {},
    );
    let job = TypeRole::LabelLarge
        .style()
        .layout_job(&harness.ctx, "Label Large", Color32::BLACK);
    let galley = harness.ctx.fonts_mut(|f| f.layout_job(job));
    let row = galley.rows.first().map(|r| r.height());
    assert_eq!(row, Some(20.0), "one Label Large row is 20 points high");
    assert_eq!(galley.size().y, 20.0, "and so is the galley");
}

#[test]
fn install_maps_the_type_scale_into_text_styles() {
    let harness = harness(
        MaterialTheme::baseline(false),
        egui::vec2(10.0, 10.0),
        |_| {},
    );
    let styles = harness.ctx.global_style().text_styles.clone();
    let medium = FontFamily::Name("plain-500".into());
    let regular = FontFamily::Name("plain-400".into());
    let expect = [
        (TextStyle::Small, FontId::new(12.0, regular.clone())),
        (TextStyle::Body, FontId::new(14.0, regular.clone())),
        (TextStyle::Button, FontId::new(14.0, medium.clone())),
        (TextStyle::Heading, FontId::new(22.0, regular.clone())),
        (
            TextStyle::Monospace,
            FontId::new(14.0, FontFamily::Monospace),
        ),
        (
            TypeRole::DisplayLarge.text_style(),
            FontId::new(57.0, regular),
        ),
        (TypeRole::LabelSmall.text_style(), FontId::new(11.0, medium)),
    ];
    for (style, font) in expect {
        assert_eq!(styles.get(&style), Some(&font), "{style:?}");
    }
    assert_eq!(styles.len(), 5 + 15, "five built-in styles and 15 roles");
}

#[test]
fn text_styles_fall_back_to_proportional_without_weight_fonts() {
    let styles = text_styles_with(|_| false);
    assert_eq!(
        styles.get(&TextStyle::Button),
        Some(&FontId::new(14.0, FontFamily::Proportional)),
        "no plain-500 registered: Button uses Proportional"
    );
}

#[test]
fn load_fonts_and_apply_theme_install_roboto_and_text_styles() {
    use egui_material3::theme::{apply_theme, load_fonts, ThemeMode};
    let ctx = egui::Context::default();
    load_fonts(&ctx);
    apply_theme(&ctx, Some(|| ThemeMode::Light));
    let button = ctx
        .global_style()
        .text_styles
        .get(&TextStyle::Button)
        .cloned();
    assert_eq!(
        button,
        Some(FontId::new(14.0, FontFamily::Name("plain-500".into()))),
        "apply_theme maps Button to Label Large in the 500 family"
    );
}

#[test]
fn symbol_names_resolve_to_codepoints() {
    assert_eq!(get_material_symbol("home"), Some(ICON_HOME), "home");
    assert_eq!(get_material_symbol("not_an_icon"), None, "unknown name");
    assert_eq!(
        get_material_symbol("clear"),
        get_material_symbol("close"),
        "aliases share a codepoint"
    );
    assert!(
        ICON_NAMES.windows(2).all(|w| w[0].0 < w[1].0),
        "ICON_NAMES is sorted and unique, for binary search"
    );
    assert!(ICON_NAMES.len() > 4000, "the full Material Symbols set");
    assert_eq!(resolve_symbol("home"), Some((ICON_HOME, false)), "by name");
    assert_eq!(
        resolve_symbol("arrow_back_rtl"),
        get_material_symbol("arrow_back").map(|c| (c, true)),
        "_rtl draws the base icon mirrored"
    );
    assert_eq!(
        resolve_symbol(&ICON_HOME.to_string()),
        Some((ICON_HOME, false)),
        "a codepoint string is used as is"
    );
}

#[test]
fn symbol_family_falls_back_to_outlined_400() {
    let defs = fonts::material_font_definitions();
    let resolved = fonts::resolve_symbol_family(&defs, SymbolStyle::Outlined, true, 700);
    assert_eq!(
        resolved,
        fonts::symbol_family(SymbolStyle::Outlined, true, 400),
        "no 700 face bundled: the 400 face"
    );
    assert_eq!(
        symbol_family_name(SymbolStyle::Rounded, false, 500),
        "MaterialSymbols-Rounded-Fill0-W500",
        "family naming"
    );
}

#[test]
fn icon_home_draws_the_glyph_not_the_word() -> TestResult {
    let _gpu = one_renderer_at_a_time();
    let mut harness = harness(
        MaterialTheme::baseline(false),
        egui::vec2(48.0, 48.0),
        icon_app("home", 24.0, false, 400),
    );
    // A one-glyph galley, one em (24 points) wide; the word would be ~40.
    let format = egui::TextFormat {
        font_id: FontId::new(
            24.0,
            fonts::symbol_family(SymbolStyle::Outlined, false, 400),
        ),
        ..Default::default()
    };
    let job = egui::text::LayoutJob::single_section(ICON_HOME.to_string(), format);
    let galley = harness.ctx.fonts_mut(|f| f.layout_job(job));
    assert_eq!(galley.size().x, 24.0, "the icon glyph is one em wide");
    if let Some(image) = render(&mut harness)? {
        assert!(ink(&image).len() > 40, "the glyph is drawn");
    }
    Ok(())
}

#[test]
fn filled_changes_pixels() -> TestResult {
    let _gpu = one_renderer_at_a_time();
    let mut ink_count = Vec::new();
    for filled in [false, true] {
        let mut harness = harness(
            MaterialTheme::baseline(false),
            egui::vec2(64.0, 64.0),
            icon_app("favorite", 48.0, filled, 400),
        );
        match render(&mut harness)? {
            Some(image) => ink_count.push(ink(&image).len()),
            None => return Ok(()),
        }
    }
    assert!(
        ink_count[1] > ink_count[0] + 100,
        "the filled heart has more ink than the outline: {ink_count:?}"
    );
    Ok(())
}

#[test]
fn weight_changes_pixels_with_the_variable_font() -> TestResult {
    let _gpu = one_renderer_at_a_time();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf"
    );
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("skipping: {path} not present");
        return Ok(());
    };
    let bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
    let mut ink_count = Vec::new();
    for weight in [200, 700] {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(64.0, 64.0))
            .with_context_setup(move |ctx| {
                let mut defs = fonts::material_font_definitions();
                fonts::add_variable_symbols(
                    &mut defs,
                    SymbolStyle::Outlined,
                    egui::FontData::from_static(bytes),
                );
                ctx.set_fonts(defs);
            })
            .build_ui(icon_app("settings", 48.0, false, weight));
        harness.run();
        match render(&mut harness)? {
            Some(image) => ink_count.push(ink(&image).len()),
            None => return Ok(()),
        }
    }
    assert!(
        ink_count[1] > ink_count[0] + 100,
        "wght 700 has more ink than wght 200: {ink_count:?}"
    );
    Ok(())
}

#[test]
fn directional_icons_mirror_in_rtl() -> TestResult {
    let _gpu = one_renderer_at_a_time();
    let mut inks = Vec::new();
    for direction in [LayoutDirection::Ltr, LayoutDirection::Rtl] {
        let theme = MaterialTheme {
            layout_direction: direction,
            ..MaterialTheme::baseline(false)
        };
        let mut harness = harness(
            theme,
            egui::vec2(64.0, 64.0),
            icon_app("arrow_back_ios", 48.0, false, 400),
        );
        let Some(image) = render(&mut harness)? else {
            return Ok(());
        };
        inks.push(ink(&image));
    }
    let (ltr, rtl) = (&inks[0], &inks[1]);
    assert!(ltr.len() > 50 && rtl.len() > 50, "both icons drawn");
    // Mirrored about x = c, LTR's rightmost ink lands on RTL's leftmost.
    let twice_c =
        ltr.iter().map(|p| p.0).max().unwrap_or(0) + rtl.iter().map(|p| p.0).min().unwrap_or(0);
    let near = |set: &[(u32, u32)], (x, y): (u32, u32)| {
        set.iter().any(|&(sx, sy)| sy == y && sx.abs_diff(x) <= 1)
    };
    let share = |matches: usize| matches as f32 / rtl.len() as f32;
    let flipped: Vec<(u32, u32)> = ltr
        .iter()
        .map(|&(x, y)| (twice_c.saturating_sub(x), y))
        .collect();
    let as_mirror = share(rtl.iter().filter(|&&p| near(&flipped, p)).count());
    let as_is = share(rtl.iter().filter(|&&p| near(ltr, p)).count());
    assert!(
        as_mirror > 0.9 && as_is < as_mirror - 0.2,
        "RTL is the LTR icon mirrored: mirror match {as_mirror}, unmirrored match {as_is}"
    );
    Ok(())
}

#[test]
fn monochrome_svg_takes_the_tint() -> TestResult {
    let _gpu = one_renderer_at_a_time();
    const SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"><rect x="2" y="2" width="20" height="20" fill="black"/></svg>"#;
    let mut harness = Harness::builder()
        .with_size(egui::vec2(32.0, 32.0))
        .build_ui(|ui| {
            fill_window(ui, Color32::WHITE);
            let rect = egui::Rect::from_min_size(egui::pos2(4.0, 4.0), egui::vec2(24.0, 24.0));
            egui_material3::paint_icon(
                ui.painter(),
                rect,
                &Icon::svg(SVG),
                &Default::default(),
                Color32::from_rgb(200, 0, 0),
            );
        });
    harness.run();
    if let Some(image) = render(&mut harness)? {
        let p = image.get_pixel(16, 16).0;
        assert!(
            p[0] > 150 && p[1] < 60 && p[2] < 60,
            "black SVG drawn in the red tint: {p:?}"
        );
    }
    Ok(())
}

#[test]
fn type_specimen_snapshot() -> TestResult {
    let _gpu = one_renderer_at_a_time();
    for (name, theme) in support::themes().into_iter().take(2) {
        // The shared harness, with the fonts installed after its first frame.
        let mut harness = support::harness_for(theme, egui::vec2(720.0, 640.0), |ui| {
            let theme = MaterialTheme::of(ui.ctx());
            fill_window(ui, theme.color(ColorRole::Surface));
            let color = theme.color(ColorRole::OnSurface);
            for role in TypeRole::ALL {
                let job = role.style().layout_job(ui.ctx(), role.name(), color);
                ui.label(job);
            }
        });
        fonts::install(&harness.ctx);
        harness.run();
        snapshot(&mut harness, &format!("type_specimen_{name}"))?;
    }
    Ok(())
}

#[test]
fn icon_grid_snapshot() -> TestResult {
    let _gpu = one_renderer_at_a_time();
    const NAMES: [&str; 8] = [
        "home",
        "favorite",
        "settings",
        "search",
        "star",
        "delete",
        "person",
        "arrow_back",
    ];
    for (name, theme) in support::themes().into_iter().take(2) {
        let mut harness = harness(theme, egui::vec2(320.0, 96.0), |ui| {
            let bg = MaterialTheme::of(ui.ctx()).color(ColorRole::Surface);
            fill_window(ui, bg);
            for filled in [false, true] {
                ui.horizontal(|ui| {
                    for icon_name in NAMES {
                        ui.add(icon(icon_name).filled(filled));
                    }
                });
            }
        });
        snapshot(&mut harness, &format!("icon_grid_{name}"))?;
    }
    Ok(())
}
