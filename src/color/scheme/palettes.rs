// ============================================================================
// Module:       color::scheme::palettes
// Description:  The six tonal palettes each variant derives from a source.
//
// Dependencies: super; super::super::{dislike, hct, temperature, tonal_palette, utils}
// ============================================================================

//! Palette derivation per [`Variant`] and [`SpecVersion`] (phone platform).
//!
//! Ported from Material Color Utilities (`dynamiccolor/dynamic_scheme.ts`,
//! `DynamicSchemePalettesDelegateImpl2021` and `...2025`; Apache-2.0,
//! Copyright 2022-2025 Google LLC). The hue breakpoints and rotations are
//! MCU's tables verbatim.

use super::super::dislike::fix_if_disliked;
use super::super::hct::Hct;
use super::super::temperature::TemperatureCache;
use super::super::tonal_palette::TonalPalette;
use super::super::utils::sanitize_degrees_double;
use super::{SpecVersion, Variant};

/// The six palettes of a scheme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Palettes {
    pub(crate) primary: TonalPalette,
    pub(crate) secondary: TonalPalette,
    pub(crate) tertiary: TonalPalette,
    pub(crate) neutral: TonalPalette,
    pub(crate) neutral_variant: TonalPalette,
    pub(crate) error: TonalPalette,
}

fn fhc(hue: f64, chroma: f64) -> TonalPalette {
    TonalPalette::from_hue_and_chroma(hue, chroma)
}

/// MCU's `DynamicScheme.getPiecewiseHue`: the value of `hues` for the
/// breakpoint band the source hue falls in, or the source hue.
pub(crate) fn piecewise_hue(source: &Hct, breakpoints: &[f64], hues: &[f64]) -> f64 {
    let source_hue = source.hue();
    for (band, value) in breakpoints.windows(2).zip(hues) {
        if let [lo, hi] = band {
            if source_hue >= *lo && source_hue < *hi {
                return sanitize_degrees_double(*value);
            }
        }
    }
    source_hue
}

/// MCU's `DynamicScheme.getRotatedHue`: the source hue rotated by the
/// band's rotation.
pub(crate) fn rotated_hue(source: &Hct, breakpoints: &[f64], rotations: &[f64]) -> f64 {
    let mut rotation = piecewise_hue(source, breakpoints, rotations);
    if breakpoints.len().saturating_sub(1).min(rotations.len()) == 0 {
        rotation = 0.0;
    }
    sanitize_degrees_double(source.hue() + rotation)
}

impl Palettes {
    /// The palettes for `source` under `variant` and `spec` (already
    /// resolved through [`SpecVersion::effective`]).
    pub(crate) fn new(variant: Variant, source: &Hct, is_dark: bool, spec: SpecVersion) -> Self {
        match spec {
            SpecVersion::Spec2021 => Self::spec_2021(variant, source),
            SpecVersion::Spec2025 => Self::spec_2025(variant, source, is_dark),
        }
    }

    fn spec_2021(variant: Variant, s: &Hct) -> Self {
        let hue = s.hue();
        let chroma = s.chroma();
        let error = fhc(25.0, 84.0);
        match variant {
            Variant::Content => Self {
                primary: fhc(hue, chroma),
                secondary: fhc(hue, f64::max(chroma - 32.0, chroma * 0.5)),
                tertiary: TonalPalette::from_hct(fix_if_disliked(
                    TemperatureCache::new(*s)
                        .analogous(3, 6)
                        .get(2)
                        .copied()
                        .unwrap_or(*s),
                )),
                neutral: fhc(hue, chroma / 8.0),
                neutral_variant: fhc(hue, chroma / 8.0 + 4.0),
                error,
            },
            Variant::Fidelity => Self {
                primary: fhc(hue, chroma),
                secondary: fhc(hue, f64::max(chroma - 32.0, chroma * 0.5)),
                tertiary: TonalPalette::from_hct(fix_if_disliked(
                    TemperatureCache::new(*s).complement(),
                )),
                neutral: fhc(hue, chroma / 8.0),
                neutral_variant: fhc(hue, chroma / 8.0 + 4.0),
                error,
            },
            Variant::FruitSalad => Self {
                primary: fhc(sanitize_degrees_double(hue - 50.0), 48.0),
                secondary: fhc(sanitize_degrees_double(hue - 50.0), 36.0),
                tertiary: fhc(hue, 36.0),
                neutral: fhc(hue, 10.0),
                neutral_variant: fhc(hue, 16.0),
                error,
            },
            Variant::Monochrome => Self {
                primary: fhc(hue, 0.0),
                secondary: fhc(hue, 0.0),
                tertiary: fhc(hue, 0.0),
                neutral: fhc(hue, 0.0),
                neutral_variant: fhc(hue, 0.0),
                error,
            },
            Variant::Neutral => Self {
                primary: fhc(hue, 12.0),
                secondary: fhc(hue, 8.0),
                tertiary: fhc(hue, 16.0),
                neutral: fhc(hue, 2.0),
                neutral_variant: fhc(hue, 2.0),
                error,
            },
            Variant::Rainbow => Self {
                primary: fhc(hue, 48.0),
                secondary: fhc(hue, 16.0),
                tertiary: fhc(sanitize_degrees_double(hue + 60.0), 24.0),
                neutral: fhc(hue, 0.0),
                neutral_variant: fhc(hue, 0.0),
                error,
            },
            Variant::TonalSpot => Self {
                primary: fhc(hue, 36.0),
                secondary: fhc(hue, 16.0),
                tertiary: fhc(sanitize_degrees_double(hue + 60.0), 24.0),
                neutral: fhc(hue, 6.0),
                neutral_variant: fhc(hue, 8.0),
                error,
            },
            Variant::Expressive => Self {
                primary: fhc(sanitize_degrees_double(hue + 240.0), 40.0),
                secondary: fhc(
                    rotated_hue(
                        s,
                        &[0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0],
                        &[45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0],
                    ),
                    24.0,
                ),
                tertiary: fhc(
                    rotated_hue(
                        s,
                        &[0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0],
                        &[120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0],
                    ),
                    32.0,
                ),
                neutral: fhc(sanitize_degrees_double(hue + 15.0), 8.0),
                neutral_variant: fhc(sanitize_degrees_double(hue + 15.0), 12.0),
                error,
            },
            Variant::Vibrant => Self {
                primary: fhc(hue, 200.0),
                secondary: fhc(
                    rotated_hue(
                        s,
                        &[0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0],
                        &[18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0],
                    ),
                    24.0,
                ),
                tertiary: fhc(
                    rotated_hue(
                        s,
                        &[0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0],
                        &[35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0],
                    ),
                    32.0,
                ),
                neutral: fhc(hue, 10.0),
                neutral_variant: fhc(hue, 12.0),
                error,
            },
        }
    }

    /// The 2025 phone palettes. Only TonalSpot, Neutral, Vibrant and
    /// Expressive use the 2025 spec; the others fall back to 2021 before
    /// getting here (see [`SpecVersion::effective`]).
    fn spec_2025(variant: Variant, s: &Hct, is_dark: bool) -> Self {
        let hue = s.hue();
        let error_hue = piecewise_hue(
            s,
            &[0.0, 3.0, 13.0, 23.0, 33.0, 43.0, 153.0, 273.0, 360.0],
            &[12.0, 22.0, 32.0, 12.0, 22.0, 32.0, 22.0, 12.0],
        );
        match variant {
            Variant::Neutral => Self {
                primary: fhc(hue, if Hct::is_blue(hue) { 12.0 } else { 8.0 }),
                secondary: fhc(hue, if Hct::is_blue(hue) { 6.0 } else { 4.0 }),
                tertiary: fhc(
                    rotated_hue(
                        s,
                        &[0.0, 38.0, 105.0, 161.0, 204.0, 278.0, 333.0, 360.0],
                        &[-32.0, 26.0, 10.0, -39.0, 24.0, -15.0, -32.0],
                    ),
                    20.0,
                ),
                neutral: fhc(hue, 1.4),
                neutral_variant: fhc(hue, 1.4 * 2.2),
                error: fhc(error_hue, 50.0),
            },
            Variant::TonalSpot => Self {
                primary: fhc(hue, if is_dark { 26.0 } else { 32.0 }),
                secondary: fhc(hue, 16.0),
                tertiary: fhc(
                    rotated_hue(
                        s,
                        &[0.0, 20.0, 71.0, 161.0, 333.0, 360.0],
                        &[-40.0, 48.0, -32.0, 40.0, -32.0],
                    ),
                    28.0,
                ),
                neutral: fhc(hue, 5.0),
                neutral_variant: fhc(hue, 5.0 * 1.7),
                error: fhc(error_hue, 60.0),
            },
            Variant::Expressive => {
                let neutral_hue = rotated_hue(
                    s,
                    &[0.0, 71.0, 124.0, 253.0, 278.0, 300.0, 360.0],
                    &[10.0, 0.0, 10.0, 0.0, 10.0, 0.0],
                );
                let neutral_chroma = if is_dark {
                    if Hct::is_yellow(neutral_hue) {
                        6.0
                    } else {
                        14.0
                    }
                } else {
                    18.0
                };
                let nv_factor = if (105.0..125.0).contains(&neutral_hue) {
                    1.6
                } else {
                    2.3
                };
                Self {
                    primary: fhc(hue, if is_dark { 36.0 } else { 48.0 }),
                    secondary: fhc(
                        rotated_hue(
                            s,
                            &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0],
                            &[-160.0, 155.0, -100.0, 96.0, -96.0, -156.0, -165.0, -160.0],
                        ),
                        if is_dark { 16.0 } else { 24.0 },
                    ),
                    tertiary: fhc(
                        rotated_hue(
                            s,
                            &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0],
                            &[-165.0, 160.0, -105.0, 101.0, -101.0, -160.0, -170.0, -165.0],
                        ),
                        48.0,
                    ),
                    neutral: fhc(neutral_hue, neutral_chroma),
                    neutral_variant: fhc(neutral_hue, neutral_chroma * nv_factor),
                    error: fhc(error_hue, 64.0),
                }
            }
            Variant::Vibrant => {
                let neutral_hue = rotated_hue(
                    s,
                    &[0.0, 38.0, 105.0, 140.0, 333.0, 360.0],
                    &[-14.0, 10.0, -14.0, 10.0, -14.0],
                );
                Self {
                    primary: fhc(hue, 74.0),
                    secondary: fhc(neutral_hue, 56.0),
                    tertiary: fhc(
                        rotated_hue(
                            s,
                            &[0.0, 38.0, 71.0, 105.0, 140.0, 161.0, 253.0, 333.0, 360.0],
                            &[-72.0, 35.0, 24.0, -24.0, 62.0, 50.0, 62.0, -72.0],
                        ),
                        56.0,
                    ),
                    neutral: fhc(neutral_hue, 28.0),
                    neutral_variant: fhc(neutral_hue, 28.0 * 1.29),
                    error: fhc(error_hue, 80.0),
                }
            }
            Variant::Monochrome
            | Variant::Fidelity
            | Variant::Content
            | Variant::Rainbow
            | Variant::FruitSalad => Self::spec_2021(variant, s),
        }
    }
}
