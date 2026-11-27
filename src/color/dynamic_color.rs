// ============================================================================
// Module:       color::dynamic_color
// Description:  Dynamic colour roles, contrast curves and tone-delta pairs.
//
// Dependencies: super::{contrast, utils}
// ============================================================================

//! The building blocks of a dynamic scheme: each [`DynamicColor`] is a
//! palette plus a rule for its tone, constrained by a background it must
//! contrast with ([`ContrastCurve`]) and, for pairs such as a container and
//! its accent, a minimum tone distance ([`ToneDeltaPair`]).
//!
//! Ported from Material Color Utilities (`dynamiccolor/dynamic_color.ts`,
//! `contrast_curve.ts`, `tone_delta_pair.ts`; Apache-2.0, Copyright
//! 2022-2025 Google LLC). MCU builds each colour from closures; here a
//! colour is an enum variant and each spec version is a pair of functions
//! (see [`super::scheme`]), which keeps every rule a plain `match`.

use crate::tokens::ColorRole;

use super::contrast;
use super::utils;

/// The contrast a colour needs at each contrast level: `low` at -1,
/// `normal` at 0, `medium` at 0.5 and `high` at 1, linear in between.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContrastCurve {
    /// Ratio at contrast level -1.
    pub low: f64,
    /// Ratio at contrast level 0.
    pub normal: f64,
    /// Ratio at contrast level 0.5.
    pub medium: f64,
    /// Ratio at contrast level 1.
    pub high: f64,
}

impl ContrastCurve {
    /// A curve through the four ratios.
    pub const fn new(low: f64, normal: f64, medium: f64, high: f64) -> Self {
        Self {
            low,
            normal,
            medium,
            high,
        }
    }

    /// The ratio at `contrast_level` (-1..1; values outside are clamped).
    pub fn get(&self, contrast_level: f64) -> f64 {
        if contrast_level <= -1.0 {
            self.low
        } else if contrast_level < 0.0 {
            utils::lerp(self.low, self.normal, contrast_level + 1.0)
        } else if contrast_level < 0.5 {
            utils::lerp(self.normal, self.medium, contrast_level / 0.5)
        } else if contrast_level < 1.0 {
            utils::lerp(self.medium, self.high, (contrast_level - 0.5) / 0.5)
        } else {
            self.high
        }
    }
}

/// Which way a [`ToneDeltaPair`] separates its two roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TonePolarity {
    /// Role A is darker than role B.
    Darker,
    /// Role A is lighter than role B.
    Lighter,
    /// Role A is nearer the background's tone than role B.
    Nearer,
    /// Role A is farther from the background's tone than role B.
    Farther,
    /// Role A is darker in light schemes and lighter in dark ones.
    RelativeDarker,
    /// Role A is lighter in light schemes and darker in dark ones.
    RelativeLighter,
}

/// How strictly a 2025 [`ToneDeltaPair`] holds its delta.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeltaConstraint {
    /// Exactly `delta` apart.
    Exact,
    /// At most `delta` apart.
    Nearer,
    /// At least `delta` apart.
    Farther,
}

/// Two roles that must stay a tone distance apart (a container and its
/// accent, a fixed colour and its dim variant).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToneDeltaPair {
    /// The first role.
    pub role_a: DynamicColor,
    /// The second role.
    pub role_b: DynamicColor,
    /// The tone distance.
    pub delta: f64,
    /// Which way they separate.
    pub polarity: TonePolarity,
    /// Whether both move together when one must avoid tones 50..60.
    pub stay_together: bool,
    /// How strictly the delta holds (2025 spec only; 2021 treats it as a
    /// minimum).
    pub constraint: DeltaConstraint,
}

impl ToneDeltaPair {
    /// A pair with the 2021 behaviour (`Exact` constraint, which 2021
    /// ignores).
    pub(crate) const fn new(
        role_a: DynamicColor,
        role_b: DynamicColor,
        delta: f64,
        polarity: TonePolarity,
        stay_together: bool,
    ) -> Self {
        Self {
            role_a,
            role_b,
            delta,
            polarity,
            stay_together,
            constraint: DeltaConstraint::Exact,
        }
    }

    /// The same pair with a 2025 constraint.
    pub(crate) const fn with_constraint(self, constraint: DeltaConstraint) -> Self {
        Self { constraint, ..self }
    }
}

macro_rules! dynamic_colors {
    ($($variant:ident => $doc:literal;)*) => {
        /// A colour a [`DynamicScheme`](super::DynamicScheme) can resolve:
        /// every [`ColorRole`], the six palette key colours, and the four
        /// `*Dim` roles of the 2025 spec.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum DynamicColor {
            $(#[doc = $doc] $variant,)*
        }

        impl DynamicColor {
            /// Every dynamic colour.
            pub const ALL: [DynamicColor; dynamic_colors!(@count $($variant)*)] =
                [$(DynamicColor::$variant,)*];
        }
    };
    (@count) => { 0 };
    (@count $head:ident $($tail:ident)*) => { 1 + dynamic_colors!(@count $($tail)*) };
}

dynamic_colors! {
    PrimaryPaletteKeyColor => "The primary palette's key colour.";
    SecondaryPaletteKeyColor => "The secondary palette's key colour.";
    TertiaryPaletteKeyColor => "The tertiary palette's key colour.";
    NeutralPaletteKeyColor => "The neutral palette's key colour.";
    NeutralVariantPaletteKeyColor => "The neutral variant palette's key colour.";
    ErrorPaletteKeyColor => "The error palette's key colour.";
    Background => "`background`.";
    OnBackground => "`onBackground`.";
    Surface => "`surface`.";
    SurfaceDim => "`surfaceDim`.";
    SurfaceBright => "`surfaceBright`.";
    SurfaceContainerLowest => "`surfaceContainerLowest`.";
    SurfaceContainerLow => "`surfaceContainerLow`.";
    SurfaceContainer => "`surfaceContainer`.";
    SurfaceContainerHigh => "`surfaceContainerHigh`.";
    SurfaceContainerHighest => "`surfaceContainerHighest`.";
    OnSurface => "`onSurface`.";
    SurfaceVariant => "`surfaceVariant`.";
    OnSurfaceVariant => "`onSurfaceVariant`.";
    InverseSurface => "`inverseSurface`.";
    InverseOnSurface => "`inverseOnSurface`.";
    Outline => "`outline`.";
    OutlineVariant => "`outlineVariant`.";
    Shadow => "`shadow`.";
    Scrim => "`scrim`.";
    SurfaceTint => "`surfaceTint`.";
    Primary => "`primary`.";
    PrimaryDim => "`primaryDim` (2025 spec only).";
    OnPrimary => "`onPrimary`.";
    PrimaryContainer => "`primaryContainer`.";
    OnPrimaryContainer => "`onPrimaryContainer`.";
    InversePrimary => "`inversePrimary`.";
    Secondary => "`secondary`.";
    SecondaryDim => "`secondaryDim` (2025 spec only).";
    OnSecondary => "`onSecondary`.";
    SecondaryContainer => "`secondaryContainer`.";
    OnSecondaryContainer => "`onSecondaryContainer`.";
    Tertiary => "`tertiary`.";
    TertiaryDim => "`tertiaryDim` (2025 spec only).";
    OnTertiary => "`onTertiary`.";
    TertiaryContainer => "`tertiaryContainer`.";
    OnTertiaryContainer => "`onTertiaryContainer`.";
    Error => "`error`.";
    ErrorDim => "`errorDim` (2025 spec only).";
    OnError => "`onError`.";
    ErrorContainer => "`errorContainer`.";
    OnErrorContainer => "`onErrorContainer`.";
    PrimaryFixed => "`primaryFixed`.";
    PrimaryFixedDim => "`primaryFixedDim`.";
    OnPrimaryFixed => "`onPrimaryFixed`.";
    OnPrimaryFixedVariant => "`onPrimaryFixedVariant`.";
    SecondaryFixed => "`secondaryFixed`.";
    SecondaryFixedDim => "`secondaryFixedDim`.";
    OnSecondaryFixed => "`onSecondaryFixed`.";
    OnSecondaryFixedVariant => "`onSecondaryFixedVariant`.";
    TertiaryFixed => "`tertiaryFixed`.";
    TertiaryFixedDim => "`tertiaryFixedDim`.";
    OnTertiaryFixed => "`onTertiaryFixed`.";
    OnTertiaryFixedVariant => "`onTertiaryFixedVariant`.";
}

impl DynamicColor {
    /// MCU's `name.endsWith('_fixed_dim')`, which exempts these from the
    /// 2025 spec's avoidance of tones 49..65 for backgrounds.
    pub(crate) fn is_fixed_dim(self) -> bool {
        matches!(
            self,
            DynamicColor::PrimaryFixedDim
                | DynamicColor::SecondaryFixedDim
                | DynamicColor::TertiaryFixedDim
        )
    }
}

impl From<ColorRole> for DynamicColor {
    fn from(role: ColorRole) -> Self {
        match role {
            ColorRole::Primary => DynamicColor::Primary,
            ColorRole::SurfaceTint => DynamicColor::SurfaceTint,
            ColorRole::OnPrimary => DynamicColor::OnPrimary,
            ColorRole::PrimaryContainer => DynamicColor::PrimaryContainer,
            ColorRole::OnPrimaryContainer => DynamicColor::OnPrimaryContainer,
            ColorRole::Secondary => DynamicColor::Secondary,
            ColorRole::OnSecondary => DynamicColor::OnSecondary,
            ColorRole::SecondaryContainer => DynamicColor::SecondaryContainer,
            ColorRole::OnSecondaryContainer => DynamicColor::OnSecondaryContainer,
            ColorRole::Tertiary => DynamicColor::Tertiary,
            ColorRole::OnTertiary => DynamicColor::OnTertiary,
            ColorRole::TertiaryContainer => DynamicColor::TertiaryContainer,
            ColorRole::OnTertiaryContainer => DynamicColor::OnTertiaryContainer,
            ColorRole::Error => DynamicColor::Error,
            ColorRole::OnError => DynamicColor::OnError,
            ColorRole::ErrorContainer => DynamicColor::ErrorContainer,
            ColorRole::OnErrorContainer => DynamicColor::OnErrorContainer,
            ColorRole::Background => DynamicColor::Background,
            ColorRole::OnBackground => DynamicColor::OnBackground,
            ColorRole::Surface => DynamicColor::Surface,
            ColorRole::OnSurface => DynamicColor::OnSurface,
            ColorRole::SurfaceVariant => DynamicColor::SurfaceVariant,
            ColorRole::OnSurfaceVariant => DynamicColor::OnSurfaceVariant,
            ColorRole::Outline => DynamicColor::Outline,
            ColorRole::OutlineVariant => DynamicColor::OutlineVariant,
            ColorRole::Shadow => DynamicColor::Shadow,
            ColorRole::Scrim => DynamicColor::Scrim,
            ColorRole::InverseSurface => DynamicColor::InverseSurface,
            ColorRole::InverseOnSurface => DynamicColor::InverseOnSurface,
            ColorRole::InversePrimary => DynamicColor::InversePrimary,
            ColorRole::PrimaryFixed => DynamicColor::PrimaryFixed,
            ColorRole::OnPrimaryFixed => DynamicColor::OnPrimaryFixed,
            ColorRole::PrimaryFixedDim => DynamicColor::PrimaryFixedDim,
            ColorRole::OnPrimaryFixedVariant => DynamicColor::OnPrimaryFixedVariant,
            ColorRole::SecondaryFixed => DynamicColor::SecondaryFixed,
            ColorRole::OnSecondaryFixed => DynamicColor::OnSecondaryFixed,
            ColorRole::SecondaryFixedDim => DynamicColor::SecondaryFixedDim,
            ColorRole::OnSecondaryFixedVariant => DynamicColor::OnSecondaryFixedVariant,
            ColorRole::TertiaryFixed => DynamicColor::TertiaryFixed,
            ColorRole::OnTertiaryFixed => DynamicColor::OnTertiaryFixed,
            ColorRole::TertiaryFixedDim => DynamicColor::TertiaryFixedDim,
            ColorRole::OnTertiaryFixedVariant => DynamicColor::OnTertiaryFixedVariant,
            ColorRole::SurfaceDim => DynamicColor::SurfaceDim,
            ColorRole::SurfaceBright => DynamicColor::SurfaceBright,
            ColorRole::SurfaceContainerLowest => DynamicColor::SurfaceContainerLowest,
            ColorRole::SurfaceContainerLow => DynamicColor::SurfaceContainerLow,
            ColorRole::SurfaceContainer => DynamicColor::SurfaceContainer,
            ColorRole::SurfaceContainerHigh => DynamicColor::SurfaceContainerHigh,
            ColorRole::SurfaceContainerHighest => DynamicColor::SurfaceContainerHighest,
        }
    }
}

/// The tone of a foreground with at least `ratio` contrast against
/// `bg_tone`, preferring light text on dark backgrounds (MCU's
/// `DynamicColor.foregroundTone`).
pub fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
    let lighter_tone = contrast::lighter_unsafe(bg_tone, ratio);
    let darker_tone = contrast::darker_unsafe(bg_tone, ratio);
    let lighter_ratio = contrast::ratio_of_tones(lighter_tone, bg_tone);
    let darker_ratio = contrast::ratio_of_tones(darker_tone, bg_tone);
    if tone_prefers_light_foreground(bg_tone) {
        let negligible_difference = (lighter_ratio - darker_ratio).abs() < 0.1
            && lighter_ratio < ratio
            && darker_ratio < ratio;
        if lighter_ratio >= ratio || lighter_ratio >= darker_ratio || negligible_difference {
            lighter_tone
        } else {
            darker_tone
        }
    } else if darker_ratio >= ratio || darker_ratio >= lighter_ratio {
        darker_tone
    } else {
        lighter_tone
    }
}

/// Whether text on `tone` reads better light (tone below 60, rounded).
pub fn tone_prefers_light_foreground(tone: f64) -> bool {
    utils::js_round(tone) < 60.0
}

/// Whether light text on `tone` meets 4.5:1 (tone 49 or below, rounded).
pub fn tone_allows_light_foreground(tone: f64) -> bool {
    utils::js_round(tone) <= 49.0
}

/// `tone`, moved to 49 if light text would be preferred but would not
/// meet contrast.
pub fn enable_light_foreground(tone: f64) -> f64 {
    if tone_prefers_light_foreground(tone) && !tone_allows_light_foreground(tone) {
        49.0
    } else {
        tone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_curve_interpolates() {
        let curve = ContrastCurve::new(1.0, 3.0, 5.0, 9.0);
        assert_eq!(curve.get(-2.0), 1.0, "below -1 is low");
        assert_eq!(curve.get(-0.5), 2.0, "between low and normal");
        assert_eq!(curve.get(0.0), 3.0, "0 is normal");
        assert_eq!(curve.get(0.25), 4.0, "between normal and medium");
        assert_eq!(curve.get(0.5), 5.0, "0.5 is medium");
        assert_eq!(curve.get(0.75), 7.0, "between medium and high");
        assert_eq!(curve.get(1.0), 9.0, "1 is high");
    }

    #[test]
    fn every_color_role_has_a_dynamic_color() {
        let mut seen = std::collections::HashSet::new();
        for role in ColorRole::ALL {
            assert!(
                seen.insert(DynamicColor::from(role)),
                "{role:?} maps to a duplicate"
            );
        }
    }

    #[test]
    fn foreground_tone_meets_the_ratio() {
        for bg in [0.0, 20.0, 49.0, 50.0, 60.0, 80.0, 100.0] {
            let fg = foreground_tone(bg, 4.5);
            let ratio = contrast::ratio_of_tones(bg, fg);
            assert!(ratio >= 4.4, "bg {bg}: fg {fg} only {ratio}:1");
        }
    }
}
