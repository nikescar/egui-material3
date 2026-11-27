// ============================================================================
// Module:       color::scheme::spec_2021
// Description:  The 2021 (Material You) colour spec: tone rules per role.
//
// Dependencies: super; super::super::{dislike, dynamic_color, hct}
// ============================================================================

//! The 2021 colour spec, role by role.
//!
//! Ported from Material Color Utilities (`dynamiccolor/color_spec_2021.ts`;
//! Apache-2.0, Copyright 2022-2025 Google LLC). [`spec`] is every field of
//! MCU's `DynamicColor.fromPalette` except `tone`, which is [`tone`].

use super::super::dislike::fix_if_disliked;
use super::super::dynamic_color::{
    foreground_tone, ContrastCurve, DynamicColor as C, ToneDeltaPair, TonePolarity,
};
use super::super::hct::Hct;
use super::{ColorSpec, DynamicScheme, PaletteRole as P, Variant};

fn is_fidelity(s: &DynamicScheme) -> bool {
    matches!(s.variant, Variant::Fidelity | Variant::Content)
}

fn is_monochrome(s: &DynamicScheme) -> bool {
    s.variant == Variant::Monochrome
}

/// `a` in dark schemes, `b` in light ones.
fn dl(s: &DynamicScheme, dark: f64, light: f64) -> f64 {
    if s.is_dark {
        dark
    } else {
        light
    }
}

/// MCU 2021's `findDesiredChromaByTone`: walk the tone away from `tone`
/// until the palette's chroma is reached or starts falling.
fn find_desired_chroma_by_tone(hue: f64, chroma: f64, tone: f64, by_decreasing_tone: bool) -> f64 {
    let mut answer = tone;
    let mut closest_to_chroma = Hct::from(hue, chroma, tone);
    if closest_to_chroma.chroma() < chroma {
        let mut chroma_peak = closest_to_chroma.chroma();
        while closest_to_chroma.chroma() < chroma {
            answer += if by_decreasing_tone { -1.0 } else { 1.0 };
            let potential = Hct::from(hue, chroma, answer);
            if chroma_peak > potential.chroma() {
                break;
            }
            if (potential.chroma() - chroma).abs() < 0.4 {
                break;
            }
            let potential_delta = (potential.chroma() - chroma).abs();
            let current_delta = (closest_to_chroma.chroma() - chroma).abs();
            if potential_delta < current_delta {
                closest_to_chroma = potential;
            }
            chroma_peak = f64::max(chroma_peak, potential.chroma());
        }
    }
    answer
}

const ACCENT: ContrastCurve = ContrastCurve::new(3.0, 4.5, 7.0, 7.0);
const CONTAINER: ContrastCurve = ContrastCurve::new(1.0, 1.0, 3.0, 4.5);
const ON_ACCENT: ContrastCurve = ContrastCurve::new(4.5, 7.0, 11.0, 21.0);
const ON_CONTAINER: ContrastCurve = ContrastCurve::new(3.0, 4.5, 7.0, 11.0);

fn container_pair(container: C, accent: C) -> ToneDeltaPair {
    ToneDeltaPair::new(container, accent, 10.0, TonePolarity::Nearer, false)
}

fn fixed_pair(fixed: C, fixed_dim: C) -> ToneDeltaPair {
    ToneDeltaPair::new(fixed, fixed_dim, 10.0, TonePolarity::Lighter, true)
}

/// The 2021 definition of `color` minus its tone, or `None` for the
/// `*Dim` roles, which 2021 does not have.
pub(super) fn spec(s: &DynamicScheme, color: C) -> Option<ColorSpec> {
    let hs = s.highest_surface();
    let neutral_bg = ColorSpec::plain(P::Neutral).background();
    Some(match color {
        C::PrimaryPaletteKeyColor => ColorSpec::plain(P::Primary),
        C::SecondaryPaletteKeyColor => ColorSpec::plain(P::Secondary),
        C::TertiaryPaletteKeyColor => ColorSpec::plain(P::Tertiary),
        C::NeutralPaletteKeyColor => ColorSpec::plain(P::Neutral),
        C::NeutralVariantPaletteKeyColor => ColorSpec::plain(P::NeutralVariant),
        C::ErrorPaletteKeyColor => ColorSpec::plain(P::Error),
        C::Background
        | C::Surface
        | C::SurfaceDim
        | C::SurfaceBright
        | C::SurfaceContainerLowest
        | C::SurfaceContainerLow
        | C::SurfaceContainer
        | C::SurfaceContainerHigh
        | C::SurfaceContainerHighest
        | C::InverseSurface => neutral_bg,
        C::OnBackground => {
            ColorSpec::plain(P::Neutral).on(C::Background, ContrastCurve::new(3.0, 3.0, 4.5, 7.0))
        }
        C::OnSurface => ColorSpec::plain(P::Neutral).on(hs, ON_ACCENT),
        C::SurfaceVariant => ColorSpec::plain(P::NeutralVariant).background(),
        C::OnSurfaceVariant => ColorSpec::plain(P::NeutralVariant).on(hs, ON_CONTAINER),
        C::InverseOnSurface => ColorSpec::plain(P::Neutral).on(C::InverseSurface, ON_ACCENT),
        C::Outline => {
            ColorSpec::plain(P::NeutralVariant).on(hs, ContrastCurve::new(1.5, 3.0, 4.5, 7.0))
        }
        C::OutlineVariant => ColorSpec::plain(P::NeutralVariant).on(hs, CONTAINER),
        C::Shadow | C::Scrim => ColorSpec::plain(P::Neutral),
        C::SurfaceTint => ColorSpec::plain(P::Primary).background(),

        C::Primary => ColorSpec::plain(P::Primary)
            .background()
            .on(hs, ACCENT)
            .pair(container_pair(C::PrimaryContainer, C::Primary)),
        C::OnPrimary => ColorSpec::plain(P::Primary).on(C::Primary, ON_ACCENT),
        C::PrimaryContainer => ColorSpec::plain(P::Primary)
            .background()
            .on(hs, CONTAINER)
            .pair(container_pair(C::PrimaryContainer, C::Primary)),
        C::OnPrimaryContainer => ColorSpec::plain(P::Primary).on(C::PrimaryContainer, ON_CONTAINER),
        C::InversePrimary => ColorSpec::plain(P::Primary).on(C::InverseSurface, ACCENT),

        C::Secondary => ColorSpec::plain(P::Secondary)
            .background()
            .on(hs, ACCENT)
            .pair(container_pair(C::SecondaryContainer, C::Secondary)),
        C::OnSecondary => ColorSpec::plain(P::Secondary).on(C::Secondary, ON_ACCENT),
        C::SecondaryContainer => ColorSpec::plain(P::Secondary)
            .background()
            .on(hs, CONTAINER)
            .pair(container_pair(C::SecondaryContainer, C::Secondary)),
        C::OnSecondaryContainer => {
            ColorSpec::plain(P::Secondary).on(C::SecondaryContainer, ON_CONTAINER)
        }

        C::Tertiary => ColorSpec::plain(P::Tertiary)
            .background()
            .on(hs, ACCENT)
            .pair(container_pair(C::TertiaryContainer, C::Tertiary)),
        C::OnTertiary => ColorSpec::plain(P::Tertiary).on(C::Tertiary, ON_ACCENT),
        C::TertiaryContainer => ColorSpec::plain(P::Tertiary)
            .background()
            .on(hs, CONTAINER)
            .pair(container_pair(C::TertiaryContainer, C::Tertiary)),
        C::OnTertiaryContainer => {
            ColorSpec::plain(P::Tertiary).on(C::TertiaryContainer, ON_CONTAINER)
        }

        C::Error => ColorSpec::plain(P::Error)
            .background()
            .on(hs, ACCENT)
            .pair(container_pair(C::ErrorContainer, C::Error)),
        C::OnError => ColorSpec::plain(P::Error).on(C::Error, ON_ACCENT),
        C::ErrorContainer => ColorSpec::plain(P::Error)
            .background()
            .on(hs, CONTAINER)
            .pair(container_pair(C::ErrorContainer, C::Error)),
        C::OnErrorContainer => ColorSpec::plain(P::Error).on(C::ErrorContainer, ON_CONTAINER),

        C::PrimaryFixed | C::PrimaryFixedDim => ColorSpec::plain(P::Primary)
            .background()
            .on(hs, CONTAINER)
            .pair(fixed_pair(C::PrimaryFixed, C::PrimaryFixedDim)),
        C::OnPrimaryFixed => ColorSpec::plain(P::Primary)
            .on(C::PrimaryFixedDim, ON_ACCENT)
            .second(C::PrimaryFixed),
        C::OnPrimaryFixedVariant => ColorSpec::plain(P::Primary)
            .on(C::PrimaryFixedDim, ON_CONTAINER)
            .second(C::PrimaryFixed),

        C::SecondaryFixed | C::SecondaryFixedDim => ColorSpec::plain(P::Secondary)
            .background()
            .on(hs, CONTAINER)
            .pair(fixed_pair(C::SecondaryFixed, C::SecondaryFixedDim)),
        C::OnSecondaryFixed => ColorSpec::plain(P::Secondary)
            .on(C::SecondaryFixedDim, ON_ACCENT)
            .second(C::SecondaryFixed),
        C::OnSecondaryFixedVariant => ColorSpec::plain(P::Secondary)
            .on(C::SecondaryFixedDim, ON_CONTAINER)
            .second(C::SecondaryFixed),

        C::TertiaryFixed | C::TertiaryFixedDim => ColorSpec::plain(P::Tertiary)
            .background()
            .on(hs, CONTAINER)
            .pair(fixed_pair(C::TertiaryFixed, C::TertiaryFixedDim)),
        C::OnTertiaryFixed => ColorSpec::plain(P::Tertiary)
            .on(C::TertiaryFixedDim, ON_ACCENT)
            .second(C::TertiaryFixed),
        C::OnTertiaryFixedVariant => ColorSpec::plain(P::Tertiary)
            .on(C::TertiaryFixedDim, ON_CONTAINER)
            .second(C::TertiaryFixed),

        C::PrimaryDim | C::SecondaryDim | C::TertiaryDim | C::ErrorDim => return None,
    })
}

/// The 2021 tone rule of `color`, before contrast and pair constraints.
pub(super) fn tone(s: &DynamicScheme, color: C) -> f64 {
    let mono = is_monochrome(s);
    let fidelity = is_fidelity(s);
    match color {
        C::PrimaryPaletteKeyColor => s.palettes.primary.key_color().tone(),
        C::SecondaryPaletteKeyColor => s.palettes.secondary.key_color().tone(),
        C::TertiaryPaletteKeyColor => s.palettes.tertiary.key_color().tone(),
        C::NeutralPaletteKeyColor => s.palettes.neutral.key_color().tone(),
        C::NeutralVariantPaletteKeyColor => s.palettes.neutral_variant.key_color().tone(),
        C::ErrorPaletteKeyColor => s.palettes.error.key_color().tone(),
        C::Background | C::Surface => dl(s, 6.0, 98.0),
        C::OnBackground | C::OnSurface => dl(s, 90.0, 10.0),
        C::SurfaceDim => dl(
            s,
            6.0,
            ContrastCurve::new(87.0, 87.0, 80.0, 75.0).get(s.contrast_level),
        ),
        C::SurfaceBright => dl(
            s,
            ContrastCurve::new(24.0, 24.0, 29.0, 34.0).get(s.contrast_level),
            98.0,
        ),
        C::SurfaceContainerLowest => dl(
            s,
            ContrastCurve::new(4.0, 4.0, 2.0, 0.0).get(s.contrast_level),
            100.0,
        ),
        C::SurfaceContainerLow => dl(
            s,
            ContrastCurve::new(10.0, 10.0, 11.0, 12.0).get(s.contrast_level),
            ContrastCurve::new(96.0, 96.0, 96.0, 95.0).get(s.contrast_level),
        ),
        C::SurfaceContainer => dl(
            s,
            ContrastCurve::new(12.0, 12.0, 16.0, 20.0).get(s.contrast_level),
            ContrastCurve::new(94.0, 94.0, 92.0, 90.0).get(s.contrast_level),
        ),
        C::SurfaceContainerHigh => dl(
            s,
            ContrastCurve::new(17.0, 17.0, 21.0, 25.0).get(s.contrast_level),
            ContrastCurve::new(92.0, 92.0, 88.0, 85.0).get(s.contrast_level),
        ),
        C::SurfaceContainerHighest => dl(
            s,
            ContrastCurve::new(22.0, 22.0, 26.0, 30.0).get(s.contrast_level),
            ContrastCurve::new(90.0, 90.0, 84.0, 80.0).get(s.contrast_level),
        ),
        C::SurfaceVariant => dl(s, 30.0, 90.0),
        C::OnSurfaceVariant => dl(s, 80.0, 30.0),
        C::InverseSurface => dl(s, 90.0, 20.0),
        C::InverseOnSurface => dl(s, 20.0, 95.0),
        C::Outline => dl(s, 60.0, 50.0),
        C::OutlineVariant => dl(s, 30.0, 80.0),
        C::Shadow | C::Scrim => 0.0,
        C::SurfaceTint => dl(s, 80.0, 40.0),

        C::Primary => {
            if mono {
                dl(s, 100.0, 0.0)
            } else {
                dl(s, 80.0, 40.0)
            }
        }
        C::OnPrimary => {
            if mono {
                dl(s, 10.0, 90.0)
            } else {
                dl(s, 20.0, 100.0)
            }
        }
        C::PrimaryContainer => {
            if fidelity {
                s.source.tone()
            } else if mono {
                dl(s, 85.0, 25.0)
            } else {
                dl(s, 30.0, 90.0)
            }
        }
        C::OnPrimaryContainer => {
            if fidelity {
                foreground_tone(tone(s, C::PrimaryContainer), 4.5)
            } else if mono {
                dl(s, 0.0, 100.0)
            } else {
                dl(s, 90.0, 30.0)
            }
        }
        C::InversePrimary => dl(s, 40.0, 80.0),

        C::Secondary => dl(s, 80.0, 40.0),
        C::OnSecondary => {
            if mono {
                dl(s, 10.0, 100.0)
            } else {
                dl(s, 20.0, 100.0)
            }
        }
        C::SecondaryContainer => {
            let initial = dl(s, 30.0, 90.0);
            if mono {
                dl(s, 30.0, 85.0)
            } else if !fidelity {
                initial
            } else {
                find_desired_chroma_by_tone(
                    s.palettes.secondary.hue(),
                    s.palettes.secondary.chroma(),
                    initial,
                    !s.is_dark,
                )
            }
        }
        C::OnSecondaryContainer => {
            if mono {
                dl(s, 90.0, 10.0)
            } else if !fidelity {
                dl(s, 90.0, 30.0)
            } else {
                foreground_tone(tone(s, C::SecondaryContainer), 4.5)
            }
        }

        C::Tertiary => {
            if mono {
                dl(s, 90.0, 25.0)
            } else {
                dl(s, 80.0, 40.0)
            }
        }
        C::OnTertiary => {
            if mono {
                dl(s, 10.0, 90.0)
            } else {
                dl(s, 20.0, 100.0)
            }
        }
        C::TertiaryContainer => {
            if mono {
                dl(s, 60.0, 49.0)
            } else if !fidelity {
                dl(s, 30.0, 90.0)
            } else {
                let proposed = s.palettes.tertiary.hct(s.source.tone());
                fix_if_disliked(proposed).tone()
            }
        }
        C::OnTertiaryContainer => {
            if mono {
                dl(s, 0.0, 100.0)
            } else if !fidelity {
                dl(s, 90.0, 30.0)
            } else {
                foreground_tone(tone(s, C::TertiaryContainer), 4.5)
            }
        }

        C::Error => dl(s, 80.0, 40.0),
        C::OnError => dl(s, 20.0, 100.0),
        C::ErrorContainer => dl(s, 30.0, 90.0),
        C::OnErrorContainer => {
            if mono {
                dl(s, 90.0, 10.0)
            } else {
                dl(s, 90.0, 30.0)
            }
        }

        C::PrimaryFixed => {
            if mono {
                40.0
            } else {
                90.0
            }
        }
        C::PrimaryFixedDim => {
            if mono {
                30.0
            } else {
                80.0
            }
        }
        C::OnPrimaryFixed => {
            if mono {
                100.0
            } else {
                10.0
            }
        }
        C::OnPrimaryFixedVariant => {
            if mono {
                90.0
            } else {
                30.0
            }
        }
        C::SecondaryFixed => {
            if mono {
                80.0
            } else {
                90.0
            }
        }
        C::SecondaryFixedDim => {
            if mono {
                70.0
            } else {
                80.0
            }
        }
        C::OnSecondaryFixed => 10.0,
        C::OnSecondaryFixedVariant => {
            if mono {
                25.0
            } else {
                30.0
            }
        }
        C::TertiaryFixed => {
            if mono {
                40.0
            } else {
                90.0
            }
        }
        C::TertiaryFixedDim => {
            if mono {
                30.0
            } else {
                80.0
            }
        }
        C::OnTertiaryFixed => {
            if mono {
                100.0
            } else {
                10.0
            }
        }
        C::OnTertiaryFixedVariant => {
            if mono {
                90.0
            } else {
                30.0
            }
        }

        // Not defined in 2021; `spec` returns `None` for these, so the
        // resolver never asks. MCU's default tone for a colour without a
        // background is 50.
        C::PrimaryDim | C::SecondaryDim | C::TertiaryDim | C::ErrorDim => 50.0,
    }
}
