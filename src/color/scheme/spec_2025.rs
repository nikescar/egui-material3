// ============================================================================
// Module:       color::scheme::spec_2025
// Description:  The 2025 (M3 Expressive) colour spec: tone rules per role.
//
// Dependencies: super; super::super::{dynamic_color, hct, tonal_palette, utils}
// ============================================================================

//! The 2025 colour spec, role by role, for the phone platform.
//!
//! Ported from Material Color Utilities (`dynamiccolor/color_spec_2025.ts`;
//! Apache-2.0, Copyright 2025 Google LLC). Roles 2025 does not override
//! (the palette key colours, `shadow`, `scrim`) use the 2021 rules, as
//! MCU's class inheritance does.

use super::super::dynamic_color::{
    ContrastCurve, DeltaConstraint, DynamicColor as C, ToneDeltaPair, TonePolarity,
};
use super::super::hct::Hct;
use super::super::tonal_palette::TonalPalette;
use super::super::utils::clamp_double;
use super::{spec_2021, ColorSpec, DynamicScheme, PaletteRole as P, Variant};

/// MCU 2025's `findBestToneForChroma`: from `tone`, step towards the
/// other end until the requested chroma is reached; the tone of highest
/// chroma seen.
fn find_best_tone_for_chroma(hue: f64, chroma: f64, start: f64, by_decreasing_tone: bool) -> f64 {
    let mut tone = start;
    let mut answer = tone;
    let mut best = Hct::from(hue, chroma, answer);
    while best.chroma() < chroma {
        if !(0.0..=100.0).contains(&tone) {
            break;
        }
        tone += if by_decreasing_tone { -1.0 } else { 1.0 };
        let candidate = Hct::from(hue, chroma, tone);
        if best.chroma() < candidate.chroma() {
            best = candidate;
            answer = tone;
        }
    }
    answer
}

/// MCU's `tMaxC`: the lightest tone at which the palette reaches its
/// (scaled) chroma, clamped to `lower..=upper`.
fn t_max_c(palette: &TonalPalette, lower: f64, upper: f64, chroma_multiplier: f64) -> f64 {
    let answer = find_best_tone_for_chroma(
        palette.hue(),
        palette.chroma() * chroma_multiplier,
        100.0,
        true,
    );
    clamp_double(lower, upper, answer)
}

/// MCU's `tMinC`: the darkest tone at which the palette reaches its
/// chroma, clamped to `lower..=upper`.
fn t_min_c(palette: &TonalPalette, lower: f64, upper: f64) -> f64 {
    let answer = find_best_tone_for_chroma(palette.hue(), palette.chroma(), 0.0, false);
    clamp_double(lower, upper, answer)
}

/// MCU 2025's `getCurve`: the standard curve for a default contrast.
fn curve(default_contrast: f64) -> ContrastCurve {
    if default_contrast == 1.5 {
        ContrastCurve::new(1.5, 1.5, 3.0, 5.5)
    } else if default_contrast == 3.0 {
        ContrastCurve::new(3.0, 3.0, 4.5, 7.0)
    } else if default_contrast == 4.5 {
        ContrastCurve::new(4.5, 4.5, 7.0, 11.0)
    } else if default_contrast == 6.0 {
        ContrastCurve::new(6.0, 6.0, 7.0, 11.0)
    } else if default_contrast == 7.0 {
        ContrastCurve::new(7.0, 7.0, 11.0, 21.0)
    } else if default_contrast == 9.0 {
        ContrastCurve::new(9.0, 9.0, 11.0, 21.0)
    } else if default_contrast == 11.0 {
        ContrastCurve::new(11.0, 11.0, 21.0, 21.0)
    } else if default_contrast == 21.0 {
        ContrastCurve::new(21.0, 21.0, 21.0, 21.0)
    } else {
        ContrastCurve::new(default_contrast, default_contrast, 7.0, 21.0)
    }
}

fn dl(s: &DynamicScheme, dark: f64, light: f64) -> f64 {
    if s.is_dark {
        dark
    } else {
        light
    }
}

/// Light-scheme surface tone: `yellow` for a yellow neutral hue, `vibrant`
/// for Vibrant, else `other`.
fn light_surface(s: &DynamicScheme, yellow: f64, vibrant: f64, other: f64) -> f64 {
    if Hct::is_yellow(s.palettes.neutral.hue()) {
        yellow
    } else if s.variant == Variant::Vibrant {
        vibrant
    } else {
        other
    }
}

/// A surface chroma multiplier per variant (1 for the others).
fn surface_chroma(
    s: &DynamicScheme,
    neutral: f64,
    tonal_spot: f64,
    expressive: (f64, f64),
    vibrant: f64,
) -> f64 {
    let yellow = Hct::is_yellow(s.palettes.neutral.hue());
    match s.variant {
        Variant::Neutral => neutral,
        Variant::TonalSpot => tonal_spot,
        Variant::Expressive => {
            if yellow {
                expressive.0
            } else {
                expressive.1
            }
        }
        Variant::Vibrant => vibrant,
        Variant::Monochrome
        | Variant::Fidelity
        | Variant::Content
        | Variant::Rainbow
        | Variant::FruitSalad => 1.0,
    }
}

/// The chroma multiplier of `onSurface`, `onSurfaceVariant`, `outline` and
/// `outlineVariant`.
fn on_surface_chroma(s: &DynamicScheme) -> f64 {
    let expressive_yellow = if s.is_dark { 3.0 } else { 2.3 };
    surface_chroma(s, 2.2, 1.7, (expressive_yellow, 1.6), 1.0)
}

/// A container's curve: none at standard or reduced contrast, so the
/// container keeps its palette tone.
fn container_curve(s: &DynamicScheme) -> Option<ContrastCurve> {
    (s.contrast_level > 0.0).then(|| curve(1.5))
}

fn on_opt(spec: ColorSpec, bg: C, curve: Option<ContrastCurve>) -> ColorSpec {
    match curve {
        Some(curve) => spec.on(bg, curve),
        None => spec,
    }
}

fn accent_pair(container: C, accent: C) -> ToneDeltaPair {
    ToneDeltaPair::new(container, accent, 5.0, TonePolarity::RelativeLighter, true)
        .with_constraint(DeltaConstraint::Farther)
}

fn dim_pair(dim: C, accent: C) -> ToneDeltaPair {
    ToneDeltaPair::new(dim, accent, 5.0, TonePolarity::Darker, true)
        .with_constraint(DeltaConstraint::Farther)
}

fn fixed_dim_pair(fixed_dim: C, fixed: C) -> ToneDeltaPair {
    ToneDeltaPair::new(fixed_dim, fixed, 5.0, TonePolarity::Darker, true)
        .with_constraint(DeltaConstraint::Exact)
}

/// The 2025 definition of `color` minus its tone.
pub(super) fn spec(s: &DynamicScheme, color: C) -> Option<ColorSpec> {
    let hs = s.highest_surface();
    let neutral_bg = ColorSpec::plain(P::Neutral).background();
    Some(match color {
        C::PrimaryPaletteKeyColor
        | C::SecondaryPaletteKeyColor
        | C::TertiaryPaletteKeyColor
        | C::NeutralPaletteKeyColor
        | C::NeutralVariantPaletteKeyColor
        | C::ErrorPaletteKeyColor
        | C::Shadow
        | C::Scrim => return spec_2021::spec(s, color),

        C::Surface | C::Background | C::SurfaceContainerLowest | C::InverseSurface => neutral_bg,
        C::SurfaceDim => neutral_bg.chroma(if s.is_dark {
            1.0
        } else {
            surface_chroma(s, 2.5, 1.7, (2.7, 1.75), 1.36)
        }),
        C::SurfaceBright => neutral_bg.chroma(if s.is_dark {
            surface_chroma(s, 2.5, 1.7, (2.7, 1.75), 1.36)
        } else {
            1.0
        }),
        C::SurfaceContainerLow => {
            neutral_bg.chroma(surface_chroma(s, 1.3, 1.25, (1.3, 1.15), 1.08))
        }
        C::SurfaceContainer => neutral_bg.chroma(surface_chroma(s, 1.6, 1.4, (1.6, 1.3), 1.15)),
        C::SurfaceContainerHigh => {
            neutral_bg.chroma(surface_chroma(s, 1.9, 1.5, (1.95, 1.45), 1.22))
        }
        C::SurfaceContainerHighest | C::SurfaceVariant => {
            neutral_bg.chroma(surface_chroma(s, 2.2, 1.7, (2.3, 1.6), 1.29))
        }
        C::OnSurface | C::OnBackground => ColorSpec::plain(P::Neutral)
            .chroma(on_surface_chroma(s))
            .on(hs, if s.is_dark { curve(11.0) } else { curve(9.0) }),
        C::OnSurfaceVariant => ColorSpec::plain(P::Neutral)
            .chroma(on_surface_chroma(s))
            .on(hs, if s.is_dark { curve(6.0) } else { curve(4.5) }),
        C::Outline => ColorSpec::plain(P::Neutral)
            .chroma(on_surface_chroma(s))
            .on(hs, curve(3.0)),
        C::OutlineVariant => ColorSpec::plain(P::Neutral)
            .chroma(on_surface_chroma(s))
            .on(hs, curve(1.5)),
        C::InverseOnSurface => ColorSpec::plain(P::Neutral).on(C::InverseSurface, curve(7.0)),

        C::Primary | C::SurfaceTint => ColorSpec::plain(P::Primary)
            .background()
            .on(hs, curve(4.5))
            .pair(accent_pair(C::PrimaryContainer, C::Primary)),
        C::PrimaryDim => ColorSpec::plain(P::Primary)
            .background()
            .on(C::SurfaceContainerHigh, curve(4.5))
            .pair(dim_pair(C::PrimaryDim, C::Primary)),
        C::OnPrimary => ColorSpec::plain(P::Primary).on(C::Primary, curve(6.0)),
        C::PrimaryContainer | C::PrimaryFixed => on_opt(
            ColorSpec::plain(P::Primary).background(),
            hs,
            container_curve(s),
        ),
        C::OnPrimaryContainer => ColorSpec::plain(P::Primary).on(C::PrimaryContainer, curve(6.0)),
        C::PrimaryFixedDim => ColorSpec::plain(P::Primary)
            .background()
            .pair(fixed_dim_pair(C::PrimaryFixedDim, C::PrimaryFixed)),
        C::OnPrimaryFixed => ColorSpec::plain(P::Primary).on(C::PrimaryFixedDim, curve(7.0)),
        C::OnPrimaryFixedVariant => ColorSpec::plain(P::Primary).on(C::PrimaryFixedDim, curve(4.5)),
        C::InversePrimary => ColorSpec::plain(P::Primary).on(C::InverseSurface, curve(6.0)),

        C::Secondary => ColorSpec::plain(P::Secondary)
            .background()
            .on(hs, curve(4.5))
            .pair(accent_pair(C::SecondaryContainer, C::Secondary)),
        C::SecondaryDim => ColorSpec::plain(P::Secondary)
            .background()
            .on(C::SurfaceContainerHigh, curve(4.5))
            .pair(dim_pair(C::SecondaryDim, C::Secondary)),
        C::OnSecondary => ColorSpec::plain(P::Secondary).on(C::Secondary, curve(6.0)),
        C::SecondaryContainer | C::SecondaryFixed => on_opt(
            ColorSpec::plain(P::Secondary).background(),
            hs,
            container_curve(s),
        ),
        C::OnSecondaryContainer => {
            ColorSpec::plain(P::Secondary).on(C::SecondaryContainer, curve(6.0))
        }
        C::SecondaryFixedDim => ColorSpec::plain(P::Secondary)
            .background()
            .pair(fixed_dim_pair(C::SecondaryFixedDim, C::SecondaryFixed)),
        C::OnSecondaryFixed => ColorSpec::plain(P::Secondary).on(C::SecondaryFixedDim, curve(7.0)),
        C::OnSecondaryFixedVariant => {
            ColorSpec::plain(P::Secondary).on(C::SecondaryFixedDim, curve(4.5))
        }

        C::Tertiary => ColorSpec::plain(P::Tertiary)
            .background()
            .on(hs, curve(4.5))
            .pair(accent_pair(C::TertiaryContainer, C::Tertiary)),
        C::TertiaryDim => ColorSpec::plain(P::Tertiary)
            .background()
            .on(C::SurfaceContainerHigh, curve(4.5))
            .pair(dim_pair(C::TertiaryDim, C::Tertiary)),
        C::OnTertiary => ColorSpec::plain(P::Tertiary).on(C::Tertiary, curve(6.0)),
        C::TertiaryContainer | C::TertiaryFixed => on_opt(
            ColorSpec::plain(P::Tertiary).background(),
            hs,
            container_curve(s),
        ),
        C::OnTertiaryContainer => {
            ColorSpec::plain(P::Tertiary).on(C::TertiaryContainer, curve(6.0))
        }
        C::TertiaryFixedDim => ColorSpec::plain(P::Tertiary)
            .background()
            .pair(fixed_dim_pair(C::TertiaryFixedDim, C::TertiaryFixed)),
        C::OnTertiaryFixed => ColorSpec::plain(P::Tertiary).on(C::TertiaryFixedDim, curve(7.0)),
        C::OnTertiaryFixedVariant => {
            ColorSpec::plain(P::Tertiary).on(C::TertiaryFixedDim, curve(4.5))
        }

        C::Error => ColorSpec::plain(P::Error)
            .background()
            .on(hs, curve(4.5))
            .pair(accent_pair(C::ErrorContainer, C::Error)),
        C::ErrorDim => ColorSpec::plain(P::Error)
            .background()
            .on(C::SurfaceContainerHigh, curve(4.5))
            .pair(dim_pair(C::ErrorDim, C::Error)),
        C::OnError => ColorSpec::plain(P::Error).on(C::Error, curve(6.0)),
        C::ErrorContainer => on_opt(
            ColorSpec::plain(P::Error).background(),
            hs,
            container_curve(s),
        ),
        C::OnErrorContainer => ColorSpec::plain(P::Error).on(C::ErrorContainer, curve(4.5)),
    })
}

/// MCU's `getInitialToneFromBackground`: the background's resolved tone,
/// or 50 without one.
fn tone_of_background(s: &DynamicScheme, color: C) -> f64 {
    spec(s, color)
        .and_then(|spec| spec.background)
        .map_or(50.0, |bg| s.tone_or_mid(bg))
}

/// The 2025 tone rule of `color`, before contrast and pair constraints.
pub(super) fn tone(s: &DynamicScheme, color: C) -> f64 {
    let v = s.variant;
    let p = &s.palettes.primary;
    let sec = &s.palettes.secondary;
    let ter = &s.palettes.tertiary;
    let err = &s.palettes.error;
    match color {
        C::PrimaryPaletteKeyColor
        | C::SecondaryPaletteKeyColor
        | C::TertiaryPaletteKeyColor
        | C::NeutralPaletteKeyColor
        | C::NeutralVariantPaletteKeyColor
        | C::ErrorPaletteKeyColor
        | C::Shadow
        | C::Scrim => spec_2021::tone(s, color),

        C::Surface | C::Background => dl(s, 4.0, light_surface(s, 99.0, 97.0, 98.0)),
        C::SurfaceDim => dl(s, 4.0, light_surface(s, 90.0, 85.0, 87.0)),
        C::SurfaceBright => dl(s, 18.0, light_surface(s, 99.0, 97.0, 98.0)),
        C::SurfaceContainerLowest => dl(s, 0.0, 100.0),
        C::SurfaceContainerLow => dl(s, 6.0, light_surface(s, 98.0, 95.0, 96.0)),
        C::SurfaceContainer => dl(s, 9.0, light_surface(s, 96.0, 92.0, 94.0)),
        C::SurfaceContainerHigh => dl(s, 12.0, light_surface(s, 94.0, 90.0, 92.0)),
        C::SurfaceContainerHighest | C::SurfaceVariant => {
            dl(s, 15.0, light_surface(s, 92.0, 88.0, 90.0))
        }
        C::OnSurface => {
            if v == Variant::Vibrant {
                t_max_c(&s.palettes.neutral, 0.0, 100.0, 1.1)
            } else {
                s.tone_or_mid(s.highest_surface())
            }
        }
        C::OnBackground => s.tone_or_mid(C::OnSurface),
        C::InverseSurface => dl(s, 98.0, 4.0),

        C::Primary | C::SurfaceTint => {
            if v == Variant::Neutral {
                dl(s, 80.0, 40.0)
            } else if v == Variant::TonalSpot {
                if s.is_dark {
                    80.0
                } else {
                    t_max_c(p, 0.0, 100.0, 1.0)
                }
            } else if v == Variant::Expressive {
                let upper = if s.is_dark {
                    if Hct::is_cyan(p.hue()) {
                        88.0
                    } else {
                        98.0
                    }
                } else if Hct::is_yellow(p.hue()) {
                    25.0
                } else {
                    98.0
                };
                t_max_c(p, 0.0, upper, 1.0)
            } else {
                t_max_c(p, 0.0, if Hct::is_cyan(p.hue()) { 88.0 } else { 98.0 }, 1.0)
            }
        }
        C::PrimaryDim => {
            if v == Variant::Neutral {
                85.0
            } else if v == Variant::TonalSpot {
                t_max_c(p, 0.0, 90.0, 1.0)
            } else {
                t_max_c(p, 0.0, 100.0, 1.0)
            }
        }
        C::PrimaryContainer => {
            if v == Variant::Neutral {
                dl(s, 30.0, 90.0)
            } else if v == Variant::TonalSpot {
                if s.is_dark {
                    t_min_c(p, 35.0, 93.0)
                } else {
                    t_max_c(p, 0.0, 90.0, 1.0)
                }
            } else if v == Variant::Expressive {
                if s.is_dark {
                    t_min_c(p, 30.0, 93.0)
                } else {
                    t_max_c(
                        p,
                        78.0,
                        if Hct::is_cyan(p.hue()) { 88.0 } else { 90.0 },
                        1.0,
                    )
                }
            } else if s.is_dark {
                t_min_c(p, 66.0, 93.0)
            } else {
                t_max_c(
                    p,
                    66.0,
                    if Hct::is_cyan(p.hue()) { 88.0 } else { 93.0 },
                    1.0,
                )
            }
        }
        C::PrimaryFixed => s.light_standard().tone_or_mid(C::PrimaryContainer),
        C::PrimaryFixedDim => s.tone_or_mid(C::PrimaryFixed),
        C::InversePrimary => t_max_c(p, 0.0, 100.0, 1.0),

        C::Secondary => {
            if v == Variant::Neutral {
                if s.is_dark {
                    t_min_c(sec, 0.0, 98.0)
                } else {
                    t_max_c(sec, 0.0, 100.0, 1.0)
                }
            } else if v == Variant::Vibrant {
                t_max_c(sec, 0.0, dl(s, 90.0, 98.0), 1.0)
            } else if s.is_dark {
                80.0
            } else {
                t_max_c(sec, 0.0, 100.0, 1.0)
            }
        }
        C::SecondaryDim => {
            if v == Variant::Neutral {
                85.0
            } else {
                t_max_c(sec, 0.0, 90.0, 1.0)
            }
        }
        C::SecondaryContainer => {
            if v == Variant::Vibrant {
                if s.is_dark {
                    t_min_c(sec, 30.0, 40.0)
                } else {
                    t_max_c(sec, 84.0, 90.0, 1.0)
                }
            } else if v == Variant::Expressive {
                if s.is_dark {
                    15.0
                } else {
                    t_max_c(sec, 90.0, 95.0, 1.0)
                }
            } else {
                dl(s, 25.0, 90.0)
            }
        }
        C::SecondaryFixed => s.light_standard().tone_or_mid(C::SecondaryContainer),
        C::SecondaryFixedDim => s.tone_or_mid(C::SecondaryFixed),

        C::Tertiary => {
            if v == Variant::Expressive || v == Variant::Vibrant {
                let upper = if Hct::is_cyan(ter.hue()) {
                    88.0
                } else {
                    dl(s, 98.0, 100.0)
                };
                t_max_c(ter, 0.0, upper, 1.0)
            } else if s.is_dark {
                t_max_c(ter, 0.0, 98.0, 1.0)
            } else {
                t_max_c(ter, 0.0, 100.0, 1.0)
            }
        }
        C::TertiaryDim => {
            if v == Variant::TonalSpot {
                t_max_c(ter, 0.0, 90.0, 1.0)
            } else {
                t_max_c(ter, 0.0, 100.0, 1.0)
            }
        }
        C::TertiaryContainer => {
            if v == Variant::Neutral {
                t_max_c(ter, 0.0, dl(s, 93.0, 96.0), 1.0)
            } else if v == Variant::TonalSpot {
                t_max_c(ter, 0.0, dl(s, 93.0, 100.0), 1.0)
            } else if v == Variant::Expressive {
                let upper = if Hct::is_cyan(ter.hue()) {
                    88.0
                } else {
                    dl(s, 93.0, 100.0)
                };
                t_max_c(ter, 75.0, upper, 1.0)
            } else if s.is_dark {
                t_max_c(ter, 0.0, 93.0, 1.0)
            } else {
                t_max_c(ter, 72.0, 100.0, 1.0)
            }
        }
        C::TertiaryFixed => s.light_standard().tone_or_mid(C::TertiaryContainer),
        C::TertiaryFixedDim => s.tone_or_mid(C::TertiaryFixed),

        C::Error => {
            if s.is_dark {
                t_min_c(err, 0.0, 98.0)
            } else {
                t_max_c(err, 0.0, 100.0, 1.0)
            }
        }
        C::ErrorDim => t_min_c(err, 0.0, 100.0),
        C::ErrorContainer => {
            if s.is_dark {
                t_min_c(err, 30.0, 93.0)
            } else {
                t_max_c(err, 0.0, 90.0, 1.0)
            }
        }

        // Defined by their background: MCU gives them no tone, so it is the
        // background's.
        C::OnSurfaceVariant
        | C::Outline
        | C::OutlineVariant
        | C::InverseOnSurface
        | C::OnPrimary
        | C::OnPrimaryContainer
        | C::OnPrimaryFixed
        | C::OnPrimaryFixedVariant
        | C::OnSecondary
        | C::OnSecondaryContainer
        | C::OnSecondaryFixed
        | C::OnSecondaryFixedVariant
        | C::OnTertiary
        | C::OnTertiaryContainer
        | C::OnTertiaryFixed
        | C::OnTertiaryFixedVariant
        | C::OnError
        | C::OnErrorContainer => tone_of_background(s, color),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_curves_match_mcu() {
        assert_eq!(curve(4.5), ContrastCurve::new(4.5, 4.5, 7.0, 11.0), "4.5");
        assert_eq!(
            curve(5.0),
            ContrastCurve::new(5.0, 5.0, 7.0, 21.0),
            "fallback"
        );
    }
}
