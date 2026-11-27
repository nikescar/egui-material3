// ============================================================================
// Module:       dynamic_color (integration test)
// Description:  Seed-to-scheme generation checked against Theme Builder.
//
// Dependencies: egui, egui_material3, serde_json
// ============================================================================

//! Dynamic colour (WP-1): schemes generated from a seed must match the
//! Material Theme Builder exports in `resources/` role for role, in light
//! and dark, at standard, medium and high contrast (Theme Builder runs
//! Material Color Utilities, TonalSpot, 2021 spec, contrast 0 / 0.5 / 1).
//! Plus: every variant and spec meeting its contrast curves, and
//! the generated scheme reaching `MaterialTheme::of`.

use egui::Color32;
use egui_material3::color::contrast::ratio_of_tones;
use egui_material3::color::{
    parse_hex_color, ContrastCurve, DynamicColor, DynamicScheme, Hct, SpecVersion, Variant,
};
use egui_material3::theme::MaterialThemeFile;
use egui_material3::tokens::{ColorRole, ColorScheme};
use egui_material3::MaterialTheme;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const THEMES: [&str; 7] = [
    include_str!("../resources/material-theme1.json"),
    include_str!("../resources/material-theme2.json"),
    include_str!("../resources/material-theme3.json"),
    include_str!("../resources/material-theme4.json"),
    include_str!("../resources/material-theme5.json"),
    include_str!("../resources/material-theme6.json"),
    include_str!("../resources/material-theme7.json"),
];

/// Theme Builder scheme names and the contrast level each was made at.
const SCHEMES: [(&str, bool, f64); 6] = [
    ("light", false, 0.0),
    ("light-medium-contrast", false, 0.5),
    ("light-high-contrast", false, 1.0),
    ("dark", true, 0.0),
    ("dark-medium-contrast", true, 0.5),
    ("dark-high-contrast", true, 1.0),
];

fn within_one(a: Color32, b: Color32) -> bool {
    a.r().abs_diff(b.r()) <= 1 && a.g().abs_diff(b.g()) <= 1 && a.b().abs_diff(b.b()) <= 1
}

fn hex(c: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
}

fn seed_of(json: &serde_json::Value) -> Result<Color32, String> {
    json.get("seed")
        .and_then(serde_json::Value::as_str)
        .and_then(parse_hex_color)
        .ok_or_else(|| "export has no parsable seed".to_owned())
}

#[test]
fn every_role_matches_theme_builder_in_every_scheme() -> TestResult {
    let mut mismatches = Vec::new();
    let mut checked = 0;
    for (index, text) in THEMES.iter().enumerate() {
        let json: serde_json::Value = serde_json::from_str(text)?;
        let seed = seed_of(&json)?;
        for (name, dark, contrast) in SCHEMES {
            let expected = json
                .get("schemes")
                .and_then(|s| s.get(name))
                .ok_or_else(|| format!("theme {} has no {name} scheme", index + 1))?;
            let generated = ColorScheme::from_seed(seed, Variant::TonalSpot, dark, contrast);
            for role in ColorRole::ALL {
                let Some(want) = expected
                    .get(role.name())
                    .and_then(serde_json::Value::as_str)
                    .and_then(parse_hex_color)
                else {
                    return Err(format!("theme {} {name} lacks {}", index + 1, role.name()).into());
                };
                let got = generated.get(role);
                checked += 1;
                if !within_one(got, want) {
                    mismatches.push(format!(
                        "theme {} ({}) {name} {}: got {}, Theme Builder {}",
                        index + 1,
                        hex(seed),
                        role.name(),
                        hex(got),
                        hex(want)
                    ));
                }
            }
        }
    }
    assert_eq!(
        checked,
        7 * 6 * ColorRole::ALL.len(),
        "every role of every scheme is checked"
    );
    assert!(
        mismatches.is_empty(),
        "{} of {checked} roles differ by more than 1 per channel:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
    Ok(())
}

const VARIANTS: [Variant; 9] = [
    Variant::Monochrome,
    Variant::Neutral,
    Variant::TonalSpot,
    Variant::Vibrant,
    Variant::Expressive,
    Variant::Fidelity,
    Variant::Content,
    Variant::Rainbow,
    Variant::FruitSalad,
];

/// Text roles and the background each must be legible on, with the
/// minimum ratio both specs guarantee at standard contrast.
const LEGIBLE: [(ColorRole, ColorRole, f64); 6] = [
    (ColorRole::OnSurface, ColorRole::Surface, 4.5),
    (ColorRole::OnPrimary, ColorRole::Primary, 4.5),
    (
        ColorRole::OnPrimaryContainer,
        ColorRole::PrimaryContainer,
        3.0,
    ),
    (
        ColorRole::OnSecondaryContainer,
        ColorRole::SecondaryContainer,
        3.0,
    ),
    (
        ColorRole::OnTertiaryContainer,
        ColorRole::TertiaryContainer,
        3.0,
    ),
    (ColorRole::OnError, ColorRole::Error, 4.5),
];

#[test]
fn every_variant_and_spec_meets_its_contrast_floor() -> TestResult {
    let seeds = [
        0xFF67_50A4_u32,
        0xFF76_9CDF,
        0xFF02_F178,
        0xFFBD_7A5E,
        0xFFE0_C000,
    ];
    let mut failures = Vec::new();
    for seed in seeds {
        for variant in VARIANTS {
            for spec in [SpecVersion::Spec2021, SpecVersion::Spec2025] {
                for dark in [false, true] {
                    for contrast in [-1.0, 0.0, 0.5, 1.0] {
                        let scheme =
                            DynamicScheme::new(Hct::from_argb(seed), variant, dark, contrast, spec);
                        if contrast < 0.0 {
                            // Reduced contrast lowers the floors; only check
                            // that every colour resolves.
                            for color in DynamicColor::ALL {
                                let defined = scheme.argb(color).is_some();
                                let dim = matches!(
                                    color,
                                    DynamicColor::PrimaryDim
                                        | DynamicColor::SecondaryDim
                                        | DynamicColor::TertiaryDim
                                        | DynamicColor::ErrorDim
                                );
                                let expected =
                                    !dim || scheme.spec_version() == SpecVersion::Spec2025;
                                if defined != expected {
                                    failures.push(format!("{seed:08X} {variant:?} {spec:?} {color:?} defined={defined}"));
                                }
                            }
                            continue;
                        }
                        for (fg, bg, floor) in LEGIBLE {
                            let fg_tone = Hct::from_color32(scheme.color(fg)).tone();
                            let bg_tone = Hct::from_color32(scheme.color(bg)).tone();
                            let ratio = ratio_of_tones(fg_tone, bg_tone);
                            // 8-bit rounding can cost a hair of contrast.
                            if ratio < floor - 0.1 {
                                failures.push(format!(
                                    "{seed:08X} {variant:?} {spec:?} dark={dark} c={contrast}: {} on {} is {ratio:.2}:1",
                                    fg.name(),
                                    bg.name()
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "contrast failures:\n{}",
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn higher_contrast_never_lowers_text_contrast() -> TestResult {
    for variant in VARIANTS {
        for spec in [SpecVersion::Spec2021, SpecVersion::Spec2025] {
            for dark in [false, true] {
                let ratio_at = |contrast: f64| {
                    let s = DynamicScheme::new(
                        Hct::from_argb(0xFF67_50A4),
                        variant,
                        dark,
                        contrast,
                        spec,
                    );
                    let fg = Hct::from_color32(s.color(ColorRole::OnSurface)).tone();
                    let bg = Hct::from_color32(s.color(ColorRole::Surface)).tone();
                    ratio_of_tones(fg, bg)
                };
                let (low, std, high) = (ratio_at(-1.0), ratio_at(0.0), ratio_at(1.0));
                assert!(
                    low <= std + 0.05 && std <= high + 0.05,
                    "{variant:?} {spec:?} dark={dark}: onSurface ratios {low:.2} / {std:.2} / {high:.2}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn spec_2025_differs_from_2021_and_has_dim_roles() -> TestResult {
    let seed = Hct::from_argb(0xFF67_50A4);
    let s2021 = DynamicScheme::new(seed, Variant::TonalSpot, false, 0.0, SpecVersion::Spec2021);
    let s2025 = DynamicScheme::new(seed, Variant::TonalSpot, false, 0.0, SpecVersion::Spec2025);
    assert_eq!(
        s2025.spec_version(),
        SpecVersion::Spec2025,
        "TonalSpot uses 2025"
    );
    assert_ne!(
        s2021.color(ColorRole::Surface),
        s2025.color(ColorRole::Surface),
        "2025 surfaces are tinted differently"
    );
    assert_eq!(
        s2021.argb(DynamicColor::PrimaryDim),
        None,
        "2021 has no primaryDim"
    );
    let dim = s2025
        .tone(DynamicColor::PrimaryDim)
        .ok_or("2025 has primaryDim")?;
    let primary = s2025
        .tone(DynamicColor::Primary)
        .ok_or("2025 has primary")?;
    assert!(
        dim <= primary - 4.9,
        "primaryDim ({dim}) is darker than primary ({primary})"
    );

    let fidelity = DynamicScheme::new(seed, Variant::Fidelity, false, 0.0, SpecVersion::Spec2025);
    assert_eq!(
        fidelity.spec_version(),
        SpecVersion::Spec2021,
        "Fidelity falls back to 2021"
    );
    Ok(())
}

#[test]
fn fidelity_keeps_the_seed_as_primary_container() -> TestResult {
    // A seed outside tones 50..60, which the 2021 spec moves backgrounds out of.
    let seed = Hct::from_argb(0xFFF4_A261);
    assert!(
        !(50.0..60.0).contains(&seed.tone()),
        "seed tone {}",
        seed.tone()
    );
    let scheme = DynamicScheme::new(seed, Variant::Fidelity, false, 0.0, SpecVersion::Spec2021);
    let container = Hct::from_color32(scheme.color(ColorRole::PrimaryContainer));
    assert!(
        (container.tone() - seed.tone()).abs() < 1.0,
        "container tone {} vs seed tone {}",
        container.tone(),
        seed.tone()
    );
    Ok(())
}

#[test]
fn monochrome_is_grey() -> TestResult {
    let scheme = DynamicScheme::new(
        Hct::from_argb(0xFF67_50A4),
        Variant::Monochrome,
        false,
        0.0,
        SpecVersion::Spec2021,
    );
    for role in ColorRole::ALL {
        if matches!(
            role,
            ColorRole::Error
                | ColorRole::OnError
                | ColorRole::ErrorContainer
                | ColorRole::OnErrorContainer
        ) {
            continue;
        }
        let c = scheme.color(role);
        assert!(
            c.r().abs_diff(c.g()) <= 1 && c.g().abs_diff(c.b()) <= 1,
            "{} is {} in a monochrome scheme",
            role.name(),
            hex(c)
        );
    }
    Ok(())
}

#[test]
fn generated_scheme_reaches_material_theme_of() -> TestResult {
    let seed = Color32::from_rgb(0x76, 0x9C, 0xDF);
    let ctx = egui::Context::default();
    MaterialTheme::set(
        &ctx,
        MaterialTheme::from_seed(seed, Variant::TonalSpot, false, 0.0),
    );
    let theme = MaterialTheme::of(&ctx);
    assert_eq!(
        theme.color(ColorRole::Primary),
        Color32::from_rgb(0x41, 0x5F, 0x91),
        "primary"
    );
    assert!(!theme.is_dark, "light");

    // A second context holds a different, dark, high-contrast seed theme.
    let other = egui::Context::default();
    let scheme = DynamicScheme::new(
        Hct::from_color32(seed),
        Variant::Vibrant,
        true,
        1.0,
        SpecVersion::Spec2025,
    );
    MaterialTheme::set(&other, MaterialTheme::from_dynamic_scheme(&scheme));
    assert!(MaterialTheme::of(&other).is_dark, "dark");
    assert_ne!(
        MaterialTheme::of(&other).color(ColorRole::Primary),
        theme.color(ColorRole::Primary),
        "the contexts do not share a theme"
    );
    Ok(())
}

#[test]
fn theme_file_seed_and_custom_colors() -> TestResult {
    let file: MaterialThemeFile =
        serde_json::from_str(include_str!("../resources/material-theme2.json"))?;
    assert_eq!(
        file.seed_color(),
        Some(Color32::from_rgb(0x63, 0xA0, 0x02)),
        "seed"
    );
    let custom = file.custom_colors();
    assert_eq!(custom.len(), 8, "eight extended colours");
    let first = custom.first().ok_or("one custom colour")?;
    assert_eq!(first.name, "Custom Color 1", "name");
    assert_eq!(
        first.value,
        Color32::from_rgb(0x91, 0xEA, 0x2E),
        "not harmonised"
    );
    let light = Hct::from_color32(first.light.color).tone();
    let dark = Hct::from_color32(first.dark.color).tone();
    assert!(
        (light - 40.0).abs() < 1.0 && (dark - 80.0).abs() < 1.0,
        "tones {light} / {dark}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Ports of MCU's `dynamic_color_test.ts` and `fixed_dynamic_color_test.ts`
// (2021 spec, the MCU default).

const MCU_SEEDS: [u32; 4] = [0xFFFF_0000, 0xFFFF_FF00, 0xFF00_FF00, 0xFF00_00FF];
const MCU_VARIANTS: [Variant; 7] = [
    Variant::Content,
    Variant::Expressive,
    Variant::Fidelity,
    Variant::Monochrome,
    Variant::Neutral,
    Variant::TonalSpot,
    Variant::Vibrant,
];
const MCU_LEVELS: [f64; 9] = [-1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];

fn mcu_schemes() -> Vec<DynamicScheme> {
    let mut schemes = Vec::new();
    for seed in MCU_SEEDS {
        for level in MCU_LEVELS {
            for dark in [false, true] {
                for variant in MCU_VARIANTS {
                    schemes.push(DynamicScheme::new(
                        Hct::from_argb(seed),
                        variant,
                        dark,
                        level,
                        SpecVersion::Spec2021,
                    ));
                }
            }
        }
    }
    schemes
}

fn tone_of(scheme: &DynamicScheme, role: ColorRole) -> f64 {
    Hct::from_color32(scheme.color(role)).tone()
}

fn describe(s: &DynamicScheme) -> String {
    format!(
        "{:?} dark={} c={} seed={:08X}",
        s.variant(),
        s.is_dark(),
        s.contrast_level(),
        s.source().to_argb()
    )
}

#[test]
fn mcu_generates_colors_respecting_contrast() -> TestResult {
    use ColorRole as R;
    let pairs = [
        (R::OnPrimary, R::Primary),
        (R::OnPrimaryContainer, R::PrimaryContainer),
        (R::OnSecondary, R::Secondary),
        (R::OnSecondaryContainer, R::SecondaryContainer),
        (R::OnTertiary, R::Tertiary),
        (R::OnTertiaryContainer, R::TertiaryContainer),
        (R::OnError, R::Error),
        (R::OnErrorContainer, R::ErrorContainer),
        (R::OnBackground, R::Background),
        (R::OnSurfaceVariant, R::SurfaceBright),
        (R::OnSurfaceVariant, R::SurfaceDim),
    ];
    let mut failures = Vec::new();
    for scheme in mcu_schemes() {
        let minimum = if scheme.contrast_level() >= 0.0 {
            4.5
        } else {
            3.0
        };
        for (fg, bg) in pairs {
            let ratio = ratio_of_tones(tone_of(&scheme, fg), tone_of(&scheme, bg));
            if ratio < minimum {
                failures.push(format!(
                    "{}: {} on {} is {ratio:.2}",
                    describe(&scheme),
                    fg.name(),
                    bg.name()
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "contrast failures:\n{}",
        failures.join("\n")
    );
    Ok(())
}

/// One of MCU's contrast constraints: foregrounds against backgrounds
/// (pairwise when `respectively`, else every combination).
struct Constraint {
    curve: ContrastCurve,
    fores: Vec<ColorRole>,
    backs: Vec<ColorRole>,
    respectively: bool,
}

fn constraint(curve: ContrastCurve, fores: &[ColorRole], backs: &[ColorRole]) -> Constraint {
    Constraint {
        curve,
        fores: fores.to_vec(),
        backs: backs.to_vec(),
        respectively: false,
    }
}

fn respectively(curve: ContrastCurve, fores: &[ColorRole], backs: &[ColorRole]) -> Constraint {
    Constraint {
        respectively: true,
        ..constraint(curve, fores, backs)
    }
}

#[test]
fn mcu_constraint_conformance() -> TestResult {
    use ColorRole as R;
    let limiting = [R::SurfaceDim, R::SurfaceBright];
    let text = ContrastCurve::new(4.5, 7.0, 11.0, 21.0);
    let variant_text = ContrastCurve::new(3.0, 4.5, 7.0, 11.0);
    let accent = ContrastCurve::new(3.0, 4.5, 7.0, 7.0);
    let constraints = [
        constraint(text, &[R::OnSurface], &limiting),
        constraint(variant_text, &[R::OnSurfaceVariant], &limiting),
        constraint(
            accent,
            &[R::Primary, R::Secondary, R::Tertiary, R::Error],
            &limiting,
        ),
        constraint(
            ContrastCurve::new(1.5, 3.0, 4.5, 7.0),
            &[R::Outline],
            &limiting,
        ),
        constraint(
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
            &[
                R::PrimaryContainer,
                R::PrimaryFixed,
                R::PrimaryFixedDim,
                R::SecondaryContainer,
                R::SecondaryFixed,
                R::SecondaryFixedDim,
                R::TertiaryContainer,
                R::TertiaryFixed,
                R::TertiaryFixedDim,
                R::ErrorContainer,
                R::OutlineVariant,
            ],
            &limiting,
        ),
        constraint(text, &[R::InverseOnSurface], &[R::InverseSurface]),
        constraint(accent, &[R::InversePrimary], &[R::InverseSurface]),
        respectively(
            text,
            &[R::OnPrimary, R::OnSecondary, R::OnTertiary, R::OnError],
            &[R::Primary, R::Secondary, R::Tertiary, R::Error],
        ),
        respectively(
            variant_text,
            &[
                R::OnPrimaryContainer,
                R::OnSecondaryContainer,
                R::OnTertiaryContainer,
                R::OnErrorContainer,
            ],
            &[
                R::PrimaryContainer,
                R::SecondaryContainer,
                R::TertiaryContainer,
                R::ErrorContainer,
            ],
        ),
        constraint(
            text,
            &[R::OnPrimaryFixed],
            &[R::PrimaryFixed, R::PrimaryFixedDim],
        ),
        constraint(
            text,
            &[R::OnSecondaryFixed],
            &[R::SecondaryFixed, R::SecondaryFixedDim],
        ),
        constraint(
            text,
            &[R::OnTertiaryFixed],
            &[R::TertiaryFixed, R::TertiaryFixedDim],
        ),
        constraint(
            variant_text,
            &[R::OnPrimaryFixedVariant],
            &[R::PrimaryFixed, R::PrimaryFixedDim],
        ),
        constraint(
            variant_text,
            &[R::OnSecondaryFixedVariant],
            &[R::SecondaryFixed, R::SecondaryFixedDim],
        ),
        constraint(
            variant_text,
            &[R::OnTertiaryFixedVariant],
            &[R::TertiaryFixed, R::TertiaryFixedDim],
        ),
    ];
    // (role, partner, polarity is "darker" rather than "farther"), delta 10.
    let deltas = [
        (R::Primary, R::PrimaryContainer, false),
        (R::Secondary, R::SecondaryContainer, false),
        (R::Tertiary, R::TertiaryContainer, false),
        (R::Error, R::ErrorContainer, false),
        (R::PrimaryFixedDim, R::PrimaryFixed, true),
        (R::SecondaryFixedDim, R::SecondaryFixed, true),
        (R::TertiaryFixedDim, R::TertiaryFixed, true),
    ];
    let backgrounds = [
        R::Background,
        R::Error,
        R::ErrorContainer,
        R::Primary,
        R::PrimaryContainer,
        R::PrimaryFixed,
        R::PrimaryFixedDim,
        R::Secondary,
        R::SecondaryContainer,
        R::SecondaryFixed,
        R::SecondaryFixedDim,
        R::Surface,
        R::SurfaceBright,
        R::SurfaceContainer,
        R::SurfaceContainerHigh,
        R::SurfaceContainerHighest,
        R::SurfaceContainerLow,
        R::SurfaceContainerLowest,
        R::SurfaceDim,
        R::SurfaceTint,
        R::SurfaceVariant,
        R::Tertiary,
        R::TertiaryContainer,
        R::TertiaryFixed,
        R::TertiaryFixedDim,
    ];
    let min_requirement = |curve: &ContrastCurve, level: f64| {
        if level >= 1.0 {
            curve.high
        } else if level >= 0.5 {
            curve.medium
        } else if level >= 0.0 {
            curve.normal
        } else {
            curve.low
        }
    };
    let mut failures = Vec::new();
    for scheme in mcu_schemes() {
        let level = scheme.contrast_level();
        for c in &constraints {
            let needed = min_requirement(&c.curve, level);
            let pairs: Vec<(R, R)> = if c.respectively {
                c.fores
                    .iter()
                    .copied()
                    .zip(c.backs.iter().copied())
                    .collect()
            } else {
                c.fores
                    .iter()
                    .flat_map(|f| c.backs.iter().map(move |b| (*f, *b)))
                    .collect()
            };
            for (fore, back) in pairs {
                let (ft, bt) = (tone_of(&scheme, fore), tone_of(&scheme, back));
                let ratio = ratio_of_tones(ft, bt);
                // MCU enforces requirements up to 4.5 strictly; above that
                // (a stretch goal), black or white on either side excuses
                // a shortfall.
                let extreme = [ft, bt].iter().any(|t| *t == 0.0 || *t == 100.0);
                if ratio < needed - 0.05 && (needed <= 4.5 || !extreme) {
                    failures.push(format!(
                        "{}: {} {ft:.2} on {} {bt:.2} = {ratio:.2}, needs {needed}",
                        describe(&scheme),
                        fore.name(),
                        back.name()
                    ));
                }
            }
        }
        for (fore, back, darker) in deltas {
            let (ft, bt) = (tone_of(&scheme, fore), tone_of(&scheme, back));
            let is_lighter = !darker && scheme.is_dark();
            let observed = if is_lighter { ft - bt } else { bt - ft };
            if observed < 10.0 - 0.5 {
                failures.push(format!(
                    "{}: delta {} {ft:.2} / {} {bt:.2} = {observed:.2}",
                    describe(&scheme),
                    fore.name(),
                    back.name()
                ));
            }
        }
        for bg in backgrounds {
            let t = tone_of(&scheme, bg);
            if (50.5..59.5).contains(&t) {
                failures.push(format!(
                    "{}: background {} at {t:.2}",
                    describe(&scheme),
                    bg.name()
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn mcu_fixed_argb_colors_in_tonal_spot() -> TestResult {
    use ColorRole as R;
    let scheme = DynamicScheme::new(
        Hct::from_argb(0xFFFF_0000),
        Variant::TonalSpot,
        true,
        0.0,
        SpecVersion::Spec2021,
    );
    let expected = [
        (R::PrimaryFixed, 0xFFFF_DAD4_u32),
        (R::PrimaryFixedDim, 0xFFFF_B4A8),
        (R::OnPrimaryFixed, 0xFF3A_0905),
        (R::OnPrimaryFixedVariant, 0xFF73_342A),
        (R::SecondaryFixed, 0xFFFF_DAD4),
        (R::SecondaryFixedDim, 0xFFE7_BDB6),
        (R::OnSecondaryFixed, 0xFF2C_1512),
        (R::OnSecondaryFixedVariant, 0xFF5D_3F3B),
        (R::TertiaryFixed, 0xFFFB_DFA6),
        (R::TertiaryFixedDim, 0xFFDE_C48C),
        (R::OnTertiaryFixed, 0xFF25_1A00),
        (R::OnTertiaryFixedVariant, 0xFF56_4419),
    ];
    for (role, argb) in expected {
        let got = scheme.argb(role.into()).ok_or("every role is defined")?;
        assert_eq!(got, argb, "{}: {got:08X}, MCU {argb:08X}", role.name());
    }
    Ok(())
}
