//! Typed Material 3 colour roles and the [`ColorScheme`] that holds them.
//!
//! Role names and the baseline values come from Compose's
//! `ColorSchemeKeyTokens.kt`, `ColorLightTokens.kt` and `ColorDarkTokens.kt`
//! (generated into `baseline.rs`). Compose has no `Shadow` role; it is
//! black (`PaletteTokens.Neutral0`), which is what Theme Builder exports too.

use egui::Color32;

macro_rules! color_roles {
    ($($variant:ident => $name:literal, $doc:literal;)*) => {
        /// A Material 3 colour role.
        ///
        /// Every role a scheme defines is a variant, so an unknown role is a
        /// compile error rather than a silent grey. The JSON name (as used
        /// by Theme Builder exports and the string API) is [`ColorRole::name`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum ColorRole {
            $(#[doc = $doc] $variant,)*
        }

        impl ColorRole {
            /// Every role, in declaration order.
            pub const ALL: [ColorRole; color_roles!(@count $($variant)*)] = [$(ColorRole::$variant,)*];

            /// The camelCase name used by Theme Builder JSON and
            /// [`get_global_color`](crate::theme::get_global_color).
            pub const fn name(self) -> &'static str {
                match self {
                    $(ColorRole::$variant => $name,)*
                }
            }

            /// Parses a camelCase role name. Returns `None` for an unknown name.
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($name => Some(ColorRole::$variant),)*
                    _ => None,
                }
            }
        }
    };
    (@count) => { 0 };
    (@count $head:ident $($tail:ident)*) => { 1 + color_roles!(@count $($tail)*) };
}

color_roles! {
    Primary => "primary", "Primary brand colour: high-emphasis fills, text and icons.";
    SurfaceTint => "surfaceTint", "Tint applied to surfaces (equal to primary).";
    OnPrimary => "onPrimary", "Content on primary.";
    PrimaryContainer => "primaryContainer", "Lower-emphasis fill for primary elements.";
    OnPrimaryContainer => "onPrimaryContainer", "Content on primary container.";
    Secondary => "secondary", "Secondary accent colour.";
    OnSecondary => "onSecondary", "Content on secondary.";
    SecondaryContainer => "secondaryContainer", "Lower-emphasis fill for secondary elements (tonal buttons).";
    OnSecondaryContainer => "onSecondaryContainer", "Content on secondary container.";
    Tertiary => "tertiary", "Tertiary accent colour.";
    OnTertiary => "onTertiary", "Content on tertiary.";
    TertiaryContainer => "tertiaryContainer", "Lower-emphasis fill for tertiary elements.";
    OnTertiaryContainer => "onTertiaryContainer", "Content on tertiary container.";
    Error => "error", "Error colour.";
    OnError => "onError", "Content on error.";
    ErrorContainer => "errorContainer", "Lower-emphasis error fill.";
    OnErrorContainer => "onErrorContainer", "Content on error container.";
    Background => "background", "Window background (equal to surface in M3).";
    OnBackground => "onBackground", "Content on background.";
    Surface => "surface", "Default surface.";
    OnSurface => "onSurface", "Content on surface.";
    SurfaceVariant => "surfaceVariant", "Alternative surface.";
    OnSurfaceVariant => "onSurfaceVariant", "Medium-emphasis content on surface.";
    Outline => "outline", "Important boundaries (outlined buttons, text fields).";
    OutlineVariant => "outlineVariant", "Decorative boundaries (dividers).";
    Shadow => "shadow", "Shadow colour for elevation.";
    Scrim => "scrim", "Scrim behind modal surfaces.";
    InverseSurface => "inverseSurface", "Inverse surface (snackbars).";
    InverseOnSurface => "inverseOnSurface", "Content on inverse surface.";
    InversePrimary => "inversePrimary", "Primary on inverse surface (snackbar actions).";
    PrimaryFixed => "primaryFixed", "Primary fill that keeps its tone in light and dark.";
    OnPrimaryFixed => "onPrimaryFixed", "Content on primary fixed.";
    PrimaryFixedDim => "primaryFixedDim", "Dimmer primary fixed.";
    OnPrimaryFixedVariant => "onPrimaryFixedVariant", "Lower-emphasis content on primary fixed.";
    SecondaryFixed => "secondaryFixed", "Secondary fill that keeps its tone in light and dark.";
    OnSecondaryFixed => "onSecondaryFixed", "Content on secondary fixed.";
    SecondaryFixedDim => "secondaryFixedDim", "Dimmer secondary fixed.";
    OnSecondaryFixedVariant => "onSecondaryFixedVariant", "Lower-emphasis content on secondary fixed.";
    TertiaryFixed => "tertiaryFixed", "Tertiary fill that keeps its tone in light and dark.";
    OnTertiaryFixed => "onTertiaryFixed", "Content on tertiary fixed.";
    TertiaryFixedDim => "tertiaryFixedDim", "Dimmer tertiary fixed.";
    OnTertiaryFixedVariant => "onTertiaryFixedVariant", "Lower-emphasis content on tertiary fixed.";
    SurfaceDim => "surfaceDim", "Dimmest surface.";
    SurfaceBright => "surfaceBright", "Brightest surface.";
    SurfaceContainerLowest => "surfaceContainerLowest", "Lowest-emphasis container surface.";
    SurfaceContainerLow => "surfaceContainerLow", "Low-emphasis container surface (elevated cards).";
    SurfaceContainer => "surfaceContainer", "Default container surface (menus, navigation bars).";
    SurfaceContainerHigh => "surfaceContainerHigh", "High-emphasis container surface (dialogs).";
    SurfaceContainerHighest => "surfaceContainerHighest", "Highest-emphasis container surface (filled text fields).";
}

/// A complete Material 3 colour scheme: one colour per [`ColorRole`].
///
/// ```
/// use egui_material3::tokens::{ColorRole, ColorScheme};
/// let scheme = ColorScheme::baseline_light();
/// assert_eq!(scheme.get(ColorRole::Primary), egui::Color32::from_rgb(0x67, 0x50, 0xA4));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ColorScheme {
    colors: [Color32; ColorRole::ALL.len()],
}

impl ColorScheme {
    fn from_hex_table(table: &[u32; ColorRole::ALL.len()]) -> Self {
        let colors = table.map(|rgb| {
            let [_, r, g, b] = rgb.to_be_bytes();
            Color32::from_rgb(r, g, b)
        });
        Self { colors }
    }

    /// Compose's baseline light scheme (`ColorLightTokens`).
    pub fn baseline_light() -> Self {
        Self::from_hex_table(&super::baseline::LIGHT)
    }

    /// Compose's baseline dark scheme (`ColorDarkTokens`).
    pub fn baseline_dark() -> Self {
        Self::from_hex_table(&super::baseline::DARK)
    }

    /// Baseline light or dark.
    pub fn baseline(dark: bool) -> Self {
        if dark {
            Self::baseline_dark()
        } else {
            Self::baseline_light()
        }
    }

    /// The colour of `role`.
    #[inline]
    pub fn get(&self, role: ColorRole) -> Color32 {
        // `ColorRole as usize` is always in bounds: the array has one slot per variant.
        self.colors
            .get(role as usize)
            .copied()
            .unwrap_or(Color32::MAGENTA)
    }

    /// Replaces the colour of `role`.
    pub fn set(&mut self, role: ColorRole, color: Color32) {
        if let Some(slot) = self.colors.get_mut(role as usize) {
            *slot = color;
        }
    }

    /// Builds a scheme by looking up each role's camelCase name in `lookup`.
    /// Roles the lookup does not resolve keep their value from `fallback`.
    pub fn from_lookup(
        fallback: &ColorScheme,
        mut lookup: impl FnMut(&str) -> Option<Color32>,
    ) -> Self {
        let mut scheme = fallback.clone();
        for role in ColorRole::ALL {
            if let Some(color) = lookup(role.name()) {
                scheme.set(role, color);
            }
        }
        scheme
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::baseline_light()
    }
}

impl std::ops::Index<ColorRole> for ColorScheme {
    type Output = Color32;
    fn index(&self, role: ColorRole) -> &Color32 {
        // One slot per variant, so this never falls through.
        self.colors.get(role as usize).unwrap_or(&Color32::MAGENTA)
    }
}
