// ============================================================================
// Module:       color
// Description:  Dynamic colour: seed colour to a full Material 3 scheme.
//
// Dependencies: egui, serde_json; crate::{material_theme, theme, tokens}
// ============================================================================

//! Material 3 dynamic colour: a port of Material Color Utilities (MCU).
//!
//! A seed colour becomes a full [`ColorScheme`] in three steps:
//!
//! 1. **HCT** ([`Hct`]): the seed's CAM16 hue and chroma, with L* as tone.
//! 2. **Palettes** ([`TonalPalette`]): a [`Variant`] derives six palettes
//!    (primary, secondary, tertiary, neutral, neutral variant, error), each
//!    one hue and chroma at every tone.
//! 3. **Roles** ([`DynamicScheme`]): every role picks a palette and a tone,
//!    adjusted to contrast with its background at the requested contrast
//!    level (-1..1, continuous) and kept apart from its partner role.
//!
//! ```
//! use egui::Color32;
//! use egui_material3::color::Variant;
//! use egui_material3::tokens::{ColorRole, ColorScheme};
//!
//! let scheme = ColorScheme::from_seed(Color32::from_rgb(0x76, 0x9C, 0xDF), Variant::TonalSpot, false, 0.0);
//! // Material Theme Builder exports #415F91 as primary for this seed.
//! assert_eq!(scheme.get(ColorRole::Primary), Color32::from_rgb(0x41, 0x5F, 0x91));
//! ```
//!
//! To draw with it, put it in the context's [`MaterialTheme`]:
//!
//! ```
//! # let ctx = egui::Context::default();
//! use egui_material3::color::Variant;
//! use egui_material3::MaterialTheme;
//! MaterialTheme::set(&ctx, MaterialTheme::from_seed(egui::Color32::from_rgb(0x67, 0x50, 0xA4), Variant::TonalSpot, true, 0.0));
//! assert!(MaterialTheme::of(&ctx).is_dark);
//! ```
//!
//! Both MCU spec versions are ported ([`SpecVersion::Spec2021`], which
//! Material Theme Builder uses, and [`SpecVersion::Spec2025`], M3
//! Expressive), for the phone platform. The ported algorithms are MCU's
//! (<https://github.com/material-foundation/material-color-utilities>,
//! TypeScript, Apache-2.0, Copyright 2021-2025 Google LLC); each module
//! names the files it ports. Golden tests check the output against
//! Material Theme Builder exports, role for role.

pub mod blend;
pub mod cam16;
pub mod contrast;
pub mod dislike;
pub mod dynamic_color;
pub mod hct;
mod hct_solver;
pub mod scheme;
pub mod temperature;
pub mod tonal_palette;
mod utils;

use egui::Color32;

pub use cam16::{Cam16, ViewingConditions};
pub use dynamic_color::{ContrastCurve, DynamicColor};
pub use hct::{argb_to_color32, color32_to_argb, Hct};
pub use scheme::{DynamicScheme, SpecVersion, Variant};
pub use tonal_palette::TonalPalette;

use crate::material_theme::MaterialTheme;
use crate::theme::MaterialThemeFile;
use crate::tokens::ColorScheme;

impl ColorScheme {
    /// The scheme MCU generates from `seed` with `variant`, light or dark,
    /// at `contrast` (-1..1: 0 standard, 0.5 medium, 1 high), under the
    /// 2021 spec that Material Theme Builder uses.
    pub fn from_seed(seed: Color32, variant: Variant, is_dark: bool, contrast: f64) -> Self {
        Self::from_seed_with_spec(seed, variant, is_dark, contrast, SpecVersion::Spec2021)
    }

    /// [`ColorScheme::from_seed`] under a chosen spec version. The 2025
    /// spec's `*Dim` roles are not part of a [`ColorScheme`]; read them
    /// from a [`DynamicScheme`].
    pub fn from_seed_with_spec(
        seed: Color32,
        variant: Variant,
        is_dark: bool,
        contrast: f64,
        spec: SpecVersion,
    ) -> Self {
        DynamicScheme::new(Hct::from_color32(seed), variant, is_dark, contrast, spec)
            .to_color_scheme()
    }
}

impl MaterialTheme {
    /// A theme whose colours are generated from `seed` (see
    /// [`ColorScheme::from_seed`]). Set it with [`MaterialTheme::set`] to
    /// make widgets in that context draw with it.
    pub fn from_seed(seed: Color32, variant: Variant, is_dark: bool, contrast: f64) -> Self {
        Self {
            colors: ColorScheme::from_seed(seed, variant, is_dark, contrast),
            ..Self::baseline(is_dark)
        }
    }

    /// A theme from a full [`DynamicScheme`] (any variant, contrast and
    /// spec version).
    pub fn from_dynamic_scheme(scheme: &DynamicScheme) -> Self {
        Self {
            colors: scheme.to_color_scheme(),
            ..Self::baseline(scheme.is_dark())
        }
    }
}

/// A role's four colours, as MCU's `customColor` produces them for an
/// extended (custom) colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorGroup {
    /// The colour itself (tone 40 light, 80 dark).
    pub color: Color32,
    /// Content on it (tone 100 light, 20 dark).
    pub on_color: Color32,
    /// Its container (tone 90 light, 30 dark).
    pub color_container: Color32,
    /// Content on the container (tone 10 light, 90 dark).
    pub on_color_container: Color32,
}

/// An extended colour resolved for light and dark schemes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomColorGroup {
    /// The colour's name.
    pub name: String,
    /// The colour after harmonisation (the input if not harmonised).
    pub value: Color32,
    /// Its roles in a light scheme.
    pub light: ColorGroup,
    /// Its roles in a dark scheme.
    pub dark: ColorGroup,
}

/// MCU's `customColor`: `color`, optionally harmonised towards `source`
/// ([`blend::harmonize`]), as a palette of chroma at least 48 and its
/// light and dark roles.
pub fn custom_color(
    name: &str,
    source: Color32,
    color: Color32,
    harmonize: bool,
) -> CustomColorGroup {
    let mut value = color32_to_argb(color);
    if harmonize {
        value = blend::harmonize(value, color32_to_argb(source));
    }
    let hct = Hct::from_argb(value);
    // MCU's `CorePalette.of(value).a1`.
    let tones = TonalPalette::from_hue_and_chroma(hct.hue(), hct.chroma().max(48.0));
    let group = |c: f64, on: f64, container: f64, on_container: f64| ColorGroup {
        color: tones.color32(c),
        on_color: tones.color32(on),
        color_container: tones.color32(container),
        on_color_container: tones.color32(on_container),
    };
    CustomColorGroup {
        name: name.to_owned(),
        value: argb_to_color32(value),
        light: group(40.0, 100.0, 90.0, 10.0),
        dark: group(80.0, 20.0, 30.0, 90.0),
    }
}

/// Parses `#RRGGBB` or `RRGGBB` (case-insensitive) into an opaque colour.
pub fn parse_hex_color(hex: &str) -> Option<Color32> {
    let digits = hex.trim().trim_start_matches('#');
    if digits.len() != 6 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let value = u32::from_str_radix(digits, 16).ok()?;
    Some(argb_to_color32(value))
}

impl MaterialThemeFile {
    /// The export's `seed` colour, if it parses.
    pub fn seed_color(&self) -> Option<Color32> {
        parse_hex_color(&self.seed)
    }

    /// The export's `extendedColors`, resolved against its seed. Entries
    /// without a parsable `color` are skipped; `harmonized` defaults to
    /// false and `name` to empty, as Theme Builder writes them.
    pub fn custom_colors(&self) -> Vec<CustomColorGroup> {
        let Some(seed) = self.seed_color() else {
            return Vec::new();
        };
        self.extended_colors
            .iter()
            .filter_map(|entry| {
                let color = parse_hex_color(entry.get("color")?.as_str()?)?;
                let name = entry.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let harmonized = entry
                    .get("harmonized")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
                Some(custom_color(name, seed, color, harmonized))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_colors_parse() {
        assert_eq!(
            parse_hex_color("#769CDF"),
            Some(Color32::from_rgb(0x76, 0x9C, 0xDF)),
            "with #"
        );
        assert_eq!(
            parse_hex_color("769cdf"),
            Some(Color32::from_rgb(0x76, 0x9C, 0xDF)),
            "bare"
        );
        assert_eq!(parse_hex_color("#769CD"), None, "too short");
        assert_eq!(parse_hex_color("#7G9CDF"), None, "not hex");
        assert_eq!(parse_hex_color("#+69CDF"), None, "a sign is not a digit");
    }

    #[test]
    fn custom_color_without_harmonising_keeps_the_hue() {
        let red = Color32::from_rgb(0xF1, 0x23, 0x3C);
        let group = custom_color("Red", Color32::from_rgb(0x63, 0xA0, 0x02), red, false);
        assert_eq!(group.value, red, "not harmonised");
        let hue = Hct::from_color32(group.light.color).hue();
        let expected = Hct::from_color32(red).hue();
        assert!(
            utils::difference_degrees(hue, expected) < 3.0,
            "hue {hue} vs {expected}"
        );
    }

    #[test]
    fn harmonising_moves_towards_the_source() {
        let source = Color32::from_rgb(0x00, 0x00, 0xFF);
        let red = Color32::from_rgb(0xFF, 0x00, 0x00);
        let group = custom_color("Red", source, red, true);
        assert_eq!(
            group.value,
            argb_to_color32(0xFFFB_0057),
            "MCU blend_test red to blue"
        );
    }
}
