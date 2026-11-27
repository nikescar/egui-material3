// ============================================================================
// Module:       color::scheme
// Description:  DynamicScheme: a source colour, variant and contrast resolved.
//
// Dependencies: egui; crate::tokens; super::{contrast, dynamic_color, hct, tonal_palette, utils}
// ============================================================================

//! [`DynamicScheme`]: palettes derived from a source colour by a
//! [`Variant`], and every [`DynamicColor`] resolved against them at a
//! contrast level and in light or dark.
//!
//! Ported from Material Color Utilities (`dynamiccolor/dynamic_scheme.ts`,
//! `dynamic_color.ts` `ColorCalculationDelegateImpl2021`/`...2025`,
//! `color_spec_2021.ts`, `color_spec_2025.ts`, and the `scheme/scheme_*.ts`
//! variants; Apache-2.0, Copyright 2022-2025 Google LLC). Only the phone
//! platform is ported; MCU's watch rules are left out.

mod palettes;
mod spec_2021;
mod spec_2025;

use egui::Color32;

use crate::tokens::{ColorRole, ColorScheme};

use super::contrast;
use super::dynamic_color::{
    foreground_tone, tone_prefers_light_foreground, ContrastCurve, DeltaConstraint, DynamicColor,
    ToneDeltaPair, TonePolarity,
};
use super::hct::{argb_to_color32, Hct};
use super::tonal_palette::TonalPalette;
use super::utils::clamp_double;

use palettes::Palettes;

/// How a scheme derives its palettes from the source colour.
///
/// The doc line of each variant is MCU's description of its scheme class.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Variant {
    /// All colours are grayscale, no chroma.
    Monochrome,
    /// Close to grayscale, a hint of chroma.
    Neutral,
    /// Pastel tokens, low chroma palettes: the default Material You theme.
    #[default]
    TonalSpot,
    /// Maxes out colourfulness at each position in the primary palette.
    Vibrant,
    /// A playful theme: the source colour's hue does not appear in it.
    Expressive,
    /// The source colour is the primary container; palettes keep its
    /// chroma (for content-derived themes that must match the source).
    Fidelity,
    /// Like Fidelity, with an analogous tertiary.
    Content,
    /// A playful theme: the source colour's hue does not appear in it.
    Rainbow,
    /// A playful theme: the source colour's hue does not appear in it.
    FruitSalad,
}

/// Which revision of the Material colour spec to resolve with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpecVersion {
    /// The 2021 Material You spec. This is what Material Theme Builder
    /// exports and what Compose's `dynamicColorScheme` used before M3
    /// Expressive.
    #[default]
    Spec2021,
    /// The 2025 M3 Expressive spec: new tone rules, chroma multipliers on
    /// surfaces, and the `*Dim` roles. Only TonalSpot, Neutral, Vibrant and
    /// Expressive have 2025 rules; other variants resolve with 2021.
    Spec2025,
}

impl SpecVersion {
    /// The version `variant` actually resolves with (MCU's
    /// `maybeFallbackSpecVersion` for the phone platform).
    pub fn effective(self, variant: Variant) -> SpecVersion {
        match variant {
            Variant::Expressive | Variant::Vibrant | Variant::TonalSpot | Variant::Neutral => self,
            Variant::Monochrome
            | Variant::Fidelity
            | Variant::Content
            | Variant::Rainbow
            | Variant::FruitSalad => SpecVersion::Spec2021,
        }
    }
}

/// Which of a scheme's palettes a colour takes its hue and chroma from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PaletteRole {
    Primary,
    Secondary,
    Tertiary,
    Neutral,
    NeutralVariant,
    Error,
}

/// Everything about a colour except its tone rule, as one spec version
/// defines it for one scheme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ColorSpec {
    pub(crate) palette: PaletteRole,
    pub(crate) is_background: bool,
    pub(crate) chroma_multiplier: f64,
    pub(crate) background: Option<DynamicColor>,
    pub(crate) second_background: Option<DynamicColor>,
    pub(crate) contrast_curve: Option<ContrastCurve>,
    pub(crate) tone_delta_pair: Option<ToneDeltaPair>,
}

impl ColorSpec {
    /// A foreground or background with no contrast requirements.
    pub(crate) const fn plain(palette: PaletteRole) -> Self {
        Self {
            palette,
            is_background: false,
            chroma_multiplier: 1.0,
            background: None,
            second_background: None,
            contrast_curve: None,
            tone_delta_pair: None,
        }
    }

    pub(crate) const fn background(mut self) -> Self {
        self.is_background = true;
        self
    }

    pub(crate) const fn on(mut self, background: DynamicColor, curve: ContrastCurve) -> Self {
        self.background = Some(background);
        self.contrast_curve = Some(curve);
        self
    }

    pub(crate) const fn second(mut self, background: DynamicColor) -> Self {
        self.second_background = Some(background);
        self
    }

    pub(crate) const fn pair(mut self, pair: ToneDeltaPair) -> Self {
        self.tone_delta_pair = Some(pair);
        self
    }

    pub(crate) const fn chroma(mut self, multiplier: f64) -> Self {
        self.chroma_multiplier = multiplier;
        self
    }
}

/// A resolved dynamic colour scheme.
///
/// ```
/// use egui_material3::color::{DynamicScheme, Hct, SpecVersion, Variant};
/// use egui_material3::tokens::ColorRole;
/// let scheme = DynamicScheme::new(
///     Hct::from_argb(0xFF76_9CDF),
///     Variant::TonalSpot,
///     false,
///     0.0,
///     SpecVersion::Spec2021,
/// );
/// // Matches Material Theme Builder's export for seed #769CDF.
/// assert_eq!(scheme.argb(ColorRole::Primary.into()), Some(0xFF41_5F91));
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicScheme {
    source: Hct,
    variant: Variant,
    is_dark: bool,
    contrast_level: f64,
    spec_version: SpecVersion,
    palettes: Palettes,
}

impl DynamicScheme {
    /// Derives the palettes of `source` for `variant`, to be resolved in
    /// light or dark (`is_dark`) at `contrast_level` (-1 lowest, 0 standard,
    /// 0.5 medium, 1 high; clamped to -1..1).
    pub fn new(
        source: Hct,
        variant: Variant,
        is_dark: bool,
        contrast_level: f64,
        spec_version: SpecVersion,
    ) -> Self {
        let spec_version = spec_version.effective(variant);
        let contrast_level = if contrast_level.is_finite() {
            clamp_double(-1.0, 1.0, contrast_level)
        } else {
            0.0
        };
        Self {
            source,
            variant,
            is_dark,
            contrast_level,
            spec_version,
            palettes: Palettes::new(variant, &source, is_dark, spec_version),
        }
    }

    /// The source colour.
    pub fn source(&self) -> Hct {
        self.source
    }

    /// The variant.
    pub fn variant(&self) -> Variant {
        self.variant
    }

    /// Whether this is a dark scheme.
    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    /// The contrast level, -1..1.
    pub fn contrast_level(&self) -> f64 {
        self.contrast_level
    }

    /// The spec version colours resolve with (after the variant fallback).
    pub fn spec_version(&self) -> SpecVersion {
        self.spec_version
    }

    /// The primary palette.
    pub fn primary_palette(&self) -> TonalPalette {
        self.palettes.primary
    }

    /// The secondary palette.
    pub fn secondary_palette(&self) -> TonalPalette {
        self.palettes.secondary
    }

    /// The tertiary palette.
    pub fn tertiary_palette(&self) -> TonalPalette {
        self.palettes.tertiary
    }

    /// The neutral palette (surfaces).
    pub fn neutral_palette(&self) -> TonalPalette {
        self.palettes.neutral
    }

    /// The neutral variant palette (outlines, surface variants).
    pub fn neutral_variant_palette(&self) -> TonalPalette {
        self.palettes.neutral_variant
    }

    /// The error palette.
    pub fn error_palette(&self) -> TonalPalette {
        self.palettes.error
    }

    pub(crate) fn palette(&self, role: PaletteRole) -> &TonalPalette {
        match role {
            PaletteRole::Primary => &self.palettes.primary,
            PaletteRole::Secondary => &self.palettes.secondary,
            PaletteRole::Tertiary => &self.palettes.tertiary,
            PaletteRole::Neutral => &self.palettes.neutral,
            PaletteRole::NeutralVariant => &self.palettes.neutral_variant,
            PaletteRole::Error => &self.palettes.error,
        }
    }

    /// The same palettes resolved as if light and at standard contrast
    /// (MCU's `Object.assign({}, s, {isDark: false, contrastLevel: 0})`,
    /// which the 2025 fixed roles use).
    pub(crate) fn light_standard(&self) -> Self {
        Self {
            is_dark: false,
            contrast_level: 0.0,
            ..*self
        }
    }

    /// `surfaceBright` in dark schemes, `surfaceDim` in light: the surface
    /// accents must contrast with whichever is closest to them.
    pub(crate) fn highest_surface(&self) -> DynamicColor {
        if self.is_dark {
            DynamicColor::SurfaceBright
        } else {
            DynamicColor::SurfaceDim
        }
    }

    pub(crate) fn spec(&self, color: DynamicColor) -> Option<ColorSpec> {
        match self.spec_version {
            SpecVersion::Spec2021 => spec_2021::spec(self, color),
            SpecVersion::Spec2025 => spec_2025::spec(self, color),
        }
    }

    /// The colour's tone before contrast and pair constraints.
    pub(crate) fn raw_tone(&self, color: DynamicColor) -> f64 {
        match self.spec_version {
            SpecVersion::Spec2021 => spec_2021::tone(self, color),
            SpecVersion::Spec2025 => spec_2025::tone(self, color),
        }
    }

    /// The resolved tone of `color`, or `None` if this spec version does
    /// not define it (the `*Dim` roles before 2025).
    pub fn tone(&self, color: DynamicColor) -> Option<f64> {
        let spec = self.spec(color)?;
        Some(match self.spec_version {
            SpecVersion::Spec2021 => self.tone_2021(color, &spec),
            SpecVersion::Spec2025 => self.tone_2025(color, &spec),
        })
    }

    /// The tone of a colour the spec defines (falls back to 50 otherwise,
    /// which no caller in the specs reaches).
    pub(crate) fn tone_or_mid(&self, color: DynamicColor) -> f64 {
        self.tone(color).unwrap_or(50.0)
    }

    /// The resolved colour, or `None` if this spec version does not define
    /// it (the `*Dim` roles before 2025).
    pub fn hct(&self, color: DynamicColor) -> Option<Hct> {
        let spec = self.spec(color)?;
        let tone = self.tone(color)?;
        let palette = self.palette(spec.palette);
        Some(match self.spec_version {
            SpecVersion::Spec2021 => palette.hct(tone),
            SpecVersion::Spec2025 => {
                let multiplier = spec.chroma_multiplier;
                if multiplier == 1.0 {
                    palette.hct(tone)
                } else {
                    let chroma = palette.chroma() * multiplier;
                    if tone == 99.0 && Hct::is_yellow(palette.hue()) {
                        TonalPalette::from_hue_and_chroma(palette.hue(), chroma).hct(tone)
                    } else {
                        Hct::from(palette.hue(), chroma, tone)
                    }
                }
            }
        })
    }

    /// The resolved colour as packed `0xFFRRGGBB`, or `None` if this spec
    /// version does not define it.
    pub fn argb(&self, color: DynamicColor) -> Option<u32> {
        self.hct(color).map(|hct| hct.to_argb())
    }

    /// The colour of a [`ColorRole`]: every role is defined in every spec.
    pub fn color(&self, role: ColorRole) -> Color32 {
        // Every `ColorRole` maps to a colour both specs define.
        self.argb(role.into())
            .map(argb_to_color32)
            .unwrap_or(Color32::MAGENTA)
    }

    /// Every role resolved into a [`ColorScheme`].
    pub fn to_color_scheme(&self) -> ColorScheme {
        let mut scheme = ColorScheme::baseline(self.is_dark);
        for role in ColorRole::ALL {
            scheme.set(role, self.color(role));
        }
        scheme
    }

    /// MCU's `ColorCalculationDelegateImpl2021.getTone`.
    fn tone_2021(&self, color: DynamicColor, spec: &ColorSpec) -> f64 {
        let decreasing_contrast = self.contrast_level < 0.0;
        if let Some(pair) = spec.tone_delta_pair {
            return self.pair_tone_2021(color, spec, &pair, decreasing_contrast);
        }
        let mut answer = self.raw_tone(color);
        let (Some(bg), Some(curve)) = (spec.background, spec.contrast_curve) else {
            return answer;
        };
        let bg_tone = self.tone_or_mid(bg);
        let desired_ratio = curve.get(self.contrast_level);
        if contrast::ratio_of_tones(bg_tone, answer) < desired_ratio {
            answer = foreground_tone(bg_tone, desired_ratio);
        }
        if decreasing_contrast {
            answer = foreground_tone(bg_tone, desired_ratio);
        }
        if spec.is_background && (50.0..60.0).contains(&answer) {
            answer = if contrast::ratio_of_tones(49.0, bg_tone) >= desired_ratio {
                49.0
            } else {
                60.0
            };
        }
        self.second_background_tone(spec, bg_tone, answer, desired_ratio)
    }

    fn pair_tone_2021(
        &self,
        color: DynamicColor,
        spec: &ColorSpec,
        pair: &ToneDeltaPair,
        decreasing_contrast: bool,
    ) -> f64 {
        let a_is_nearer = match pair.polarity {
            TonePolarity::Nearer => true,
            TonePolarity::Lighter => !self.is_dark,
            TonePolarity::Darker => self.is_dark,
            TonePolarity::Farther
            | TonePolarity::RelativeDarker
            | TonePolarity::RelativeLighter => false,
        };
        let (nearer, farther) = if a_is_nearer {
            (pair.role_a, pair.role_b)
        } else {
            (pair.role_b, pair.role_a)
        };
        let am_nearer = color == nearer;
        let expansion_dir = if self.is_dark { 1.0 } else { -1.0 };
        let delta = pair.delta;
        let mut n_tone = self.raw_tone(nearer);
        let mut f_tone = self.raw_tone(farther);

        let n_curve = self.spec(nearer).and_then(|s| s.contrast_curve);
        let f_curve = self.spec(farther).and_then(|s| s.contrast_curve);
        if let (Some(bg), Some(n_curve), Some(f_curve)) = (spec.background, n_curve, f_curve) {
            let bg_tone = self.tone_or_mid(bg);
            let n_contrast = n_curve.get(self.contrast_level);
            let f_contrast = f_curve.get(self.contrast_level);
            if contrast::ratio_of_tones(bg_tone, n_tone) < n_contrast {
                n_tone = foreground_tone(bg_tone, n_contrast);
            }
            if contrast::ratio_of_tones(bg_tone, f_tone) < f_contrast {
                f_tone = foreground_tone(bg_tone, f_contrast);
            }
            if decreasing_contrast {
                n_tone = foreground_tone(bg_tone, n_contrast);
                f_tone = foreground_tone(bg_tone, f_contrast);
            }
        }

        if (f_tone - n_tone) * expansion_dir < delta {
            f_tone = clamp_double(0.0, 100.0, n_tone + delta * expansion_dir);
            if (f_tone - n_tone) * expansion_dir < delta {
                n_tone = clamp_double(0.0, 100.0, f_tone - delta * expansion_dir);
            }
        }

        if (50.0..60.0).contains(&n_tone) {
            if expansion_dir > 0.0 {
                n_tone = 60.0;
                f_tone = f64::max(f_tone, n_tone + delta * expansion_dir);
            } else {
                n_tone = 49.0;
                f_tone = f64::min(f_tone, n_tone + delta * expansion_dir);
            }
        } else if (50.0..60.0).contains(&f_tone) {
            if pair.stay_together {
                if expansion_dir > 0.0 {
                    n_tone = 60.0;
                    f_tone = f64::max(f_tone, n_tone + delta * expansion_dir);
                } else {
                    n_tone = 49.0;
                    f_tone = f64::min(f_tone, n_tone + delta * expansion_dir);
                }
            } else if expansion_dir > 0.0 {
                f_tone = 60.0;
            } else {
                f_tone = 49.0;
            }
        }

        if am_nearer {
            n_tone
        } else {
            f_tone
        }
    }

    /// MCU's `ColorCalculationDelegateImpl2025.getTone` (phone platform).
    fn tone_2025(&self, color: DynamicColor, spec: &ColorSpec) -> f64 {
        if let Some(pair) = spec.tone_delta_pair {
            let absolute_delta = match pair.polarity {
                TonePolarity::Darker => -pair.delta,
                TonePolarity::RelativeLighter if self.is_dark => -pair.delta,
                TonePolarity::RelativeDarker if !self.is_dark => -pair.delta,
                TonePolarity::RelativeLighter
                | TonePolarity::RelativeDarker
                | TonePolarity::Lighter
                | TonePolarity::Nearer
                | TonePolarity::Farther => pair.delta,
            };
            let am_role_a = color == pair.role_a;
            let (self_role, ref_role) = if am_role_a {
                (pair.role_a, pair.role_b)
            } else {
                (pair.role_b, pair.role_a)
            };
            let mut self_tone = self.raw_tone(self_role);
            let ref_tone = self.tone_or_mid(ref_role);
            let relative_delta = absolute_delta * if am_role_a { 1.0 } else { -1.0 };
            match pair.constraint {
                DeltaConstraint::Exact => {
                    self_tone = clamp_double(0.0, 100.0, ref_tone + relative_delta);
                }
                DeltaConstraint::Nearer => {
                    self_tone = if relative_delta > 0.0 {
                        clamp_double(
                            0.0,
                            100.0,
                            clamp_double(ref_tone, ref_tone + relative_delta, self_tone),
                        )
                    } else {
                        clamp_double(
                            0.0,
                            100.0,
                            clamp_double(ref_tone + relative_delta, ref_tone, self_tone),
                        )
                    };
                }
                DeltaConstraint::Farther => {
                    self_tone = if relative_delta > 0.0 {
                        clamp_double(ref_tone + relative_delta, 100.0, self_tone)
                    } else {
                        clamp_double(0.0, ref_tone + relative_delta, self_tone)
                    };
                }
            }
            if let (Some(bg), Some(curve)) = (spec.background, spec.contrast_curve) {
                let bg_tone = self.tone_or_mid(bg);
                let self_contrast = curve.get(self.contrast_level);
                if contrast::ratio_of_tones(bg_tone, self_tone) < self_contrast
                    || self.contrast_level < 0.0
                {
                    self_tone = foreground_tone(bg_tone, self_contrast);
                }
            }
            if spec.is_background && !color.is_fixed_dim() {
                self_tone = avoid_mid_tones(self_tone);
            }
            return self_tone;
        }

        let mut answer = self.raw_tone(color);
        let (Some(bg), Some(curve)) = (spec.background, spec.contrast_curve) else {
            return answer;
        };
        let bg_tone = self.tone_or_mid(bg);
        let desired_ratio = curve.get(self.contrast_level);
        if contrast::ratio_of_tones(bg_tone, answer) < desired_ratio || self.contrast_level < 0.0 {
            answer = foreground_tone(bg_tone, desired_ratio);
        }
        if spec.is_background && !color.is_fixed_dim() {
            answer = avoid_mid_tones(answer);
        }
        self.second_background_tone(spec, bg_tone, answer, desired_ratio)
    }

    /// The shared tail of both specs: a foreground over two backgrounds
    /// (`onPrimaryFixed` over `primaryFixed` and `primaryFixedDim`) must
    /// contrast with both.
    fn second_background_tone(
        &self,
        spec: &ColorSpec,
        bg_tone1: f64,
        answer: f64,
        desired_ratio: f64,
    ) -> f64 {
        let Some(second) = spec.second_background else {
            return answer;
        };
        let bg_tone2 = self.tone_or_mid(second);
        let upper = f64::max(bg_tone1, bg_tone2);
        let lower = f64::min(bg_tone1, bg_tone2);
        if contrast::ratio_of_tones(upper, answer) >= desired_ratio
            && contrast::ratio_of_tones(lower, answer) >= desired_ratio
        {
            return answer;
        }
        let light_option = contrast::lighter(upper, desired_ratio);
        let dark_option = contrast::darker(lower, desired_ratio);
        let prefers_light =
            tone_prefers_light_foreground(bg_tone1) || tone_prefers_light_foreground(bg_tone2);
        if prefers_light {
            return light_option.unwrap_or(100.0);
        }
        match (light_option, dark_option) {
            (Some(only), None) | (None, Some(only)) => only,
            (Some(_), Some(dark)) => dark,
            (None, None) => 0.0,
        }
    }
}

/// The 2025 phone rule: backgrounds avoid tones 50..65 (49 or below, or
/// 65 and up), so text on them can always be either black or white.
fn avoid_mid_tones(tone: f64) -> f64 {
    if tone >= 57.0 {
        clamp_double(65.0, 100.0, tone)
    } else {
        clamp_double(0.0, 49.0, tone)
    }
}
