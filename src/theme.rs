//! Material Design 3 theming system
//!
//! This module provides a comprehensive theming system for Material Design 3 components,
//! including support for build-time theme inclusion, runtime theme loading, and dynamic
//! theme switching with multiple modes and contrast levels.
//!
//! # Overview
//!
//! The theme system consists of several key components:
//!
//! - **Theme Preparation**: Load and parse Material Design theme JSON files
//! - **Theme Loading**: Apply prepared themes to the global context
//! - **Font Management**: Handle Google Fonts and local font loading
//! - **Background Updates**: Apply theme-appropriate background colors
//! - **Runtime Switching**: Change themes, modes, and contrast levels dynamically
//!
//! # Basic Usage
//!
//! ```rust,no_run
//! use egui_material3::theme::{
//!     setup_google_fonts, setup_local_fonts, setup_local_theme,
//!     load_fonts, load_themes, update_window_background
//! };
//!
//! // Setup fonts and themes (typically during app initialization)
//! setup_google_fonts(Some("Roboto"));
//! setup_local_fonts(Some("path/to/MaterialSymbols.ttf"));
//! setup_local_theme(None); // Use build-time included themes
//!
//! // Load prepared fonts and themes (accepts both &egui::Context and egui::Context)
//! load_fonts(&egui_ctx);  // With reference
//! load_fonts(egui_ctx);   // With owned context  
//! load_themes();
//!
//! // Apply theme background (also flexible with context types)
//! update_window_background(&egui_ctx);
//! ```
//!
//! # Build-time Theme Inclusion
//!
//! The build script automatically scans for theme JSON files in:
//! - `resources/` directory  
//! - `examples/` directory
//!
//! Files matching patterns like `*theme*.json` or `*material-theme*.json` are
//! included as string constants for optimal performance.
//!
//! # Theme JSON Format
//!
//! Themes should follow the Material Design Theme Builder export format:
//!
//! ```json
//! {
//!   "description": "My Custom Theme",
//!   "seed": "#6750A4",
//!   "coreColors": {
//!     "primary": "#6750A4"
//!   },
//!   "schemes": {
//!     "light": {
//!       "primary": "#6750A4",
//!       "onPrimary": "#FFFFFF",
//!       "surface": "#FEF7FF",
//!       // ... more colors
//!     },
//!     "dark": {
//!       "primary": "#D0BCFF",
//!       "onPrimary": "#381E72",
//!       "surface": "#141218",
//!       // ... more colors
//!     }
//!   }
//! }
//! ```

use egui::{Color32, FontData, FontDefinitions, FontFamily};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(feature = "ondemand")]
use std::io::Read;

// Font runtime management system - replaced build-time font inclusion with runtime loading for better flexibility

// Runtime font management system with support for both local and on-demand font loading

/// Global collection of prepared fonts before loading to context
#[derive(Debug, Clone)]
pub struct PreparedFont {
    pub name: String,
    pub data: Arc<FontData>,
    pub families: Vec<FontFamily>,
}

static PREPARED_FONTS: Mutex<Vec<PreparedFont>> = Mutex::new(Vec::new());

/// A prepared Material Design theme ready for loading
///
/// This struct represents a Material Design theme that has been loaded and parsed
/// from a JSON file, but not yet applied to the global theme context. Themes are
/// stored in this prepared state to allow multiple themes to be loaded and then
/// selectively applied.
///
/// # Fields
/// * `name` - Human-readable name for the theme (derived from filename or "default")
/// * `theme_data` - The complete Material Design theme specification parsed from JSON
///
/// # Usage
/// This struct is primarily used internally by the theme system. Themes are prepared
/// by `setup_local_theme()` and stored in the static `PREPARED_THEMES` collection,
/// then activated by `load_themes()`.
#[derive(Debug, Clone)]
pub struct PreparedTheme {
    pub name: String,
    pub theme_data: MaterialThemeFile,
}

static PREPARED_THEMES: Mutex<Vec<PreparedTheme>> = Mutex::new(Vec::new());

/// Material Design color scheme structure from JSON
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaterialScheme {
    pub primary: String,
    #[serde(rename = "surfaceTint")]
    pub surface_tint: String,
    #[serde(rename = "onPrimary")]
    pub on_primary: String,
    #[serde(rename = "primaryContainer")]
    pub primary_container: String,
    #[serde(rename = "onPrimaryContainer")]
    pub on_primary_container: String,
    pub secondary: String,
    #[serde(rename = "onSecondary")]
    pub on_secondary: String,
    #[serde(rename = "secondaryContainer")]
    pub secondary_container: String,
    #[serde(rename = "onSecondaryContainer")]
    pub on_secondary_container: String,
    pub tertiary: String,
    #[serde(rename = "onTertiary")]
    pub on_tertiary: String,
    #[serde(rename = "tertiaryContainer")]
    pub tertiary_container: String,
    #[serde(rename = "onTertiaryContainer")]
    pub on_tertiary_container: String,
    pub error: String,
    #[serde(rename = "onError")]
    pub on_error: String,
    #[serde(rename = "errorContainer")]
    pub error_container: String,
    #[serde(rename = "onErrorContainer")]
    pub on_error_container: String,
    pub background: String,
    #[serde(rename = "onBackground")]
    pub on_background: String,
    pub surface: String,
    #[serde(rename = "onSurface")]
    pub on_surface: String,
    #[serde(rename = "surfaceVariant")]
    pub surface_variant: String,
    #[serde(rename = "onSurfaceVariant")]
    pub on_surface_variant: String,
    pub outline: String,
    #[serde(rename = "outlineVariant")]
    pub outline_variant: String,
    pub shadow: String,
    pub scrim: String,
    #[serde(rename = "inverseSurface")]
    pub inverse_surface: String,
    #[serde(rename = "inverseOnSurface")]
    pub inverse_on_surface: String,
    #[serde(rename = "inversePrimary")]
    pub inverse_primary: String,
    #[serde(rename = "primaryFixed")]
    pub primary_fixed: String,
    #[serde(rename = "onPrimaryFixed")]
    pub on_primary_fixed: String,
    #[serde(rename = "primaryFixedDim")]
    pub primary_fixed_dim: String,
    #[serde(rename = "onPrimaryFixedVariant")]
    pub on_primary_fixed_variant: String,
    #[serde(rename = "secondaryFixed")]
    pub secondary_fixed: String,
    #[serde(rename = "onSecondaryFixed")]
    pub on_secondary_fixed: String,
    #[serde(rename = "secondaryFixedDim")]
    pub secondary_fixed_dim: String,
    #[serde(rename = "onSecondaryFixedVariant")]
    pub on_secondary_fixed_variant: String,
    #[serde(rename = "tertiaryFixed")]
    pub tertiary_fixed: String,
    #[serde(rename = "onTertiaryFixed")]
    pub on_tertiary_fixed: String,
    #[serde(rename = "tertiaryFixedDim")]
    pub tertiary_fixed_dim: String,
    #[serde(rename = "onTertiaryFixedVariant")]
    pub on_tertiary_fixed_variant: String,
    #[serde(rename = "surfaceDim")]
    pub surface_dim: String,
    #[serde(rename = "surfaceBright")]
    pub surface_bright: String,
    #[serde(rename = "surfaceContainerLowest")]
    pub surface_container_lowest: String,
    #[serde(rename = "surfaceContainerLow")]
    pub surface_container_low: String,
    #[serde(rename = "surfaceContainer")]
    pub surface_container: String,
    #[serde(rename = "surfaceContainerHigh")]
    pub surface_container_high: String,
    #[serde(rename = "surfaceContainerHighest")]
    pub surface_container_highest: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaterialThemeFile {
    pub description: String,
    pub seed: String,
    #[serde(rename = "coreColors")]
    pub core_colors: HashMap<String, String>,
    #[serde(rename = "extendedColors")]
    pub extended_colors: Vec<serde_json::Value>,
    pub schemes: HashMap<String, MaterialScheme>,
    pub palettes: HashMap<String, HashMap<String, String>>,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ContrastLevel {
    Normal,
    Medium,
    High,
}

impl std::fmt::Display for ContrastLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContrastLevel::Normal => write!(f, "Normal"),
            ContrastLevel::Medium => write!(f, "Medium"),
            ContrastLevel::High => write!(f, "High"),
        }
    }
}

impl std::str::FromStr for ContrastLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(ContrastLevel::Normal),
            "Medium" => Ok(ContrastLevel::Medium),
            "High" => Ok(ContrastLevel::High),
            _ => Ok(ContrastLevel::Normal), // Default to Normal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    Auto,
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::Dark => write!(f, "Dark"),
            ThemeMode::Auto => write!(f, "Auto"),
        }
    }
}

impl std::str::FromStr for ThemeMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Light" => Ok(ThemeMode::Light),
            "Dark" => Ok(ThemeMode::Dark),
            "Auto" => Ok(ThemeMode::Auto),
            _ => Ok(ThemeMode::Auto), // Default to Auto
        }
    }
}


/// Global theme context that can be shared across all Material components
#[derive(Clone, Debug)]
pub struct MaterialThemeContext {
    pub theme_mode: ThemeMode,
    pub contrast_level: ContrastLevel,
    pub material_theme: Option<MaterialThemeFile>,
    pub selected_colors: HashMap<String, Color32>,
}

impl Default for MaterialThemeContext {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Auto,
            contrast_level: ContrastLevel::Normal,
            material_theme: Some(get_default_material_theme()),
            selected_colors: HashMap::new(),
        }
    }
}

fn get_default_material_theme() -> MaterialThemeFile {
    // Create default Material theme programmatically using colors from material-theme4.json
    let light_scheme = MaterialScheme {
        primary: "#48672F".to_string(),
        surface_tint: "#48672F".to_string(),
        on_primary: "#FFFFFF".to_string(),
        primary_container: "#C8EEA8".to_string(),
        on_primary_container: "#314F19".to_string(),
        secondary: "#56624B".to_string(),
        on_secondary: "#FFFFFF".to_string(),
        secondary_container: "#DAE7C9".to_string(),
        on_secondary_container: "#3F4A34".to_string(),
        tertiary: "#386665".to_string(),
        on_tertiary: "#FFFFFF".to_string(),
        tertiary_container: "#BBECEA".to_string(),
        on_tertiary_container: "#1E4E4D".to_string(),
        error: "#BA1A1A".to_string(),
        on_error: "#FFFFFF".to_string(),
        error_container: "#FFDAD6".to_string(),
        on_error_container: "#93000A".to_string(),
        background: "#F9FAEF".to_string(),
        on_background: "#191D16".to_string(),
        surface: "#F9FAEF".to_string(),
        on_surface: "#191D16".to_string(),
        surface_variant: "#E0E4D6".to_string(),
        on_surface_variant: "#44483E".to_string(),
        outline: "#74796D".to_string(),
        outline_variant: "#C4C8BA".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#2E312A".to_string(),
        inverse_on_surface: "#F0F2E7".to_string(),
        inverse_primary: "#ADD28E".to_string(),
        primary_fixed: "#C8EEA8".to_string(),
        on_primary_fixed: "#0B2000".to_string(),
        primary_fixed_dim: "#ADD28E".to_string(),
        on_primary_fixed_variant: "#314F19".to_string(),
        secondary_fixed: "#DAE7C9".to_string(),
        on_secondary_fixed: "#141E0C".to_string(),
        secondary_fixed_dim: "#BECBAE".to_string(),
        on_secondary_fixed_variant: "#3F4A34".to_string(),
        tertiary_fixed: "#BBECEA".to_string(),
        on_tertiary_fixed: "#00201F".to_string(),
        tertiary_fixed_dim: "#A0CFCE".to_string(),
        on_tertiary_fixed_variant: "#1E4E4D".to_string(),
        surface_dim: "#D9DBD1".to_string(),
        surface_bright: "#F9FAEF".to_string(),
        surface_container_lowest: "#FFFFFF".to_string(),
        surface_container_low: "#F3F5EA".to_string(),
        surface_container: "#EDEFE4".to_string(),
        surface_container_high: "#E7E9DE".to_string(),
        surface_container_highest: "#E2E3D9".to_string(),
    };

    let dark_scheme = MaterialScheme {
        primary: "#ADD28E".to_string(),
        surface_tint: "#ADD28E".to_string(),
        on_primary: "#1B3704".to_string(),
        primary_container: "#314F19".to_string(),
        on_primary_container: "#C8EEA8".to_string(),
        secondary: "#BECBAE".to_string(),
        on_secondary: "#29341F".to_string(),
        secondary_container: "#3F4A34".to_string(),
        on_secondary_container: "#DAE7C9".to_string(),
        tertiary: "#A0CFCE".to_string(),
        on_tertiary: "#003736".to_string(),
        tertiary_container: "#1E4E4D".to_string(),
        on_tertiary_container: "#BBECEA".to_string(),
        error: "#FFB4AB".to_string(),
        on_error: "#690005".to_string(),
        error_container: "#93000A".to_string(),
        on_error_container: "#FFDAD6".to_string(),
        background: "#11140E".to_string(),
        on_background: "#E2E3D9".to_string(),
        surface: "#11140E".to_string(),
        on_surface: "#E2E3D9".to_string(),
        surface_variant: "#44483E".to_string(),
        on_surface_variant: "#C4C8BA".to_string(),
        outline: "#8E9286".to_string(),
        outline_variant: "#44483E".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#E2E3D9".to_string(),
        inverse_on_surface: "#2E312A".to_string(),
        inverse_primary: "#48672F".to_string(),
        primary_fixed: "#C8EEA8".to_string(),
        on_primary_fixed: "#0B2000".to_string(),
        primary_fixed_dim: "#ADD28E".to_string(),
        on_primary_fixed_variant: "#314F19".to_string(),
        secondary_fixed: "#DAE7C9".to_string(),
        on_secondary_fixed: "#141E0C".to_string(),
        secondary_fixed_dim: "#BECBAE".to_string(),
        on_secondary_fixed_variant: "#3F4A34".to_string(),
        tertiary_fixed: "#BBECEA".to_string(),
        on_tertiary_fixed: "#00201F".to_string(),
        tertiary_fixed_dim: "#A0CFCE".to_string(),
        on_tertiary_fixed_variant: "#1E4E4D".to_string(),
        surface_dim: "#11140E".to_string(),
        surface_bright: "#373A33".to_string(),
        surface_container_lowest: "#0C0F09".to_string(),
        surface_container_low: "#191D16".to_string(),
        surface_container: "#1E211A".to_string(),
        surface_container_high: "#282B24".to_string(),
        surface_container_highest: "#33362F".to_string(),
    };

    let light_medium_contrast_scheme = MaterialScheme {
        primary: "#253D05".to_string(),
        surface_tint: "#4C662B".to_string(),
        on_primary: "#FFFFFF".to_string(),
        primary_container: "#5A7539".to_string(),
        on_primary_container: "#FFFFFF".to_string(),
        secondary: "#303924".to_string(),
        on_secondary: "#FFFFFF".to_string(),
        secondary_container: "#667157".to_string(),
        on_secondary_container: "#FFFFFF".to_string(),
        tertiary: "#083D3A".to_string(),
        on_tertiary: "#FFFFFF".to_string(),
        tertiary_container: "#477572".to_string(),
        on_tertiary_container: "#FFFFFF".to_string(),
        error: "#740006".to_string(),
        on_error: "#FFFFFF".to_string(),
        error_container: "#CF2C27".to_string(),
        on_error_container: "#FFFFFF".to_string(),
        background: "#F9FAEF".to_string(),
        on_background: "#1A1C16".to_string(),
        surface: "#F9FAEF".to_string(),
        on_surface: "#0F120C".to_string(),
        surface_variant: "#E1E4D5".to_string(),
        on_surface_variant: "#34382D".to_string(),
        outline: "#505449".to_string(),
        outline_variant: "#6B6F62".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#2F312A".to_string(),
        inverse_on_surface: "#F1F2E6".to_string(),
        inverse_primary: "#B1D18A".to_string(),
        primary_fixed: "#5A7539".to_string(),
        on_primary_fixed: "#FFFFFF".to_string(),
        primary_fixed_dim: "#425C23".to_string(),
        on_primary_fixed_variant: "#FFFFFF".to_string(),
        secondary_fixed: "#667157".to_string(),
        on_secondary_fixed: "#FFFFFF".to_string(),
        secondary_fixed_dim: "#4E5840".to_string(),
        on_secondary_fixed_variant: "#FFFFFF".to_string(),
        tertiary_fixed: "#477572".to_string(),
        on_tertiary_fixed: "#FFFFFF".to_string(),
        tertiary_fixed_dim: "#2E5C59".to_string(),
        on_tertiary_fixed_variant: "#FFFFFF".to_string(),
        surface_dim: "#C6C7BD".to_string(),
        surface_bright: "#F9FAEF".to_string(),
        surface_container_lowest: "#FFFFFF".to_string(),
        surface_container_low: "#F3F4E9".to_string(),
        surface_container: "#E8E9DE".to_string(),
        surface_container_high: "#DCDED3".to_string(),
        surface_container_highest: "#D1D3C8".to_string(),
    };

    let light_high_contrast_scheme = MaterialScheme {
        primary: "#1C3200".to_string(),
        surface_tint: "#4C662B".to_string(),
        on_primary: "#FFFFFF".to_string(),
        primary_container: "#375018".to_string(),
        on_primary_container: "#FFFFFF".to_string(),
        secondary: "#262F1A".to_string(),
        on_secondary: "#FFFFFF".to_string(),
        secondary_container: "#434C35".to_string(),
        on_secondary_container: "#FFFFFF".to_string(),
        tertiary: "#003230".to_string(),
        on_tertiary: "#FFFFFF".to_string(),
        tertiary_container: "#21504E".to_string(),
        on_tertiary_container: "#FFFFFF".to_string(),
        error: "#600004".to_string(),
        on_error: "#FFFFFF".to_string(),
        error_container: "#98000A".to_string(),
        on_error_container: "#FFFFFF".to_string(),
        background: "#F9FAEF".to_string(),
        on_background: "#1A1C16".to_string(),
        surface: "#F9FAEF".to_string(),
        on_surface: "#000000".to_string(),
        surface_variant: "#E1E4D5".to_string(),
        on_surface_variant: "#000000".to_string(),
        outline: "#2A2D24".to_string(),
        outline_variant: "#474B40".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#2F312A".to_string(),
        inverse_on_surface: "#FFFFFF".to_string(),
        inverse_primary: "#B1D18A".to_string(),
        primary_fixed: "#375018".to_string(),
        on_primary_fixed: "#FFFFFF".to_string(),
        primary_fixed_dim: "#213903".to_string(),
        on_primary_fixed_variant: "#FFFFFF".to_string(),
        secondary_fixed: "#434C35".to_string(),
        on_secondary_fixed: "#FFFFFF".to_string(),
        secondary_fixed_dim: "#2C3620".to_string(),
        on_secondary_fixed_variant: "#FFFFFF".to_string(),
        tertiary_fixed: "#21504E".to_string(),
        on_tertiary_fixed: "#FFFFFF".to_string(),
        tertiary_fixed_dim: "#033937".to_string(),
        on_tertiary_fixed_variant: "#FFFFFF".to_string(),
        surface_dim: "#B8BAAF".to_string(),
        surface_bright: "#F9FAEF".to_string(),
        surface_container_lowest: "#FFFFFF".to_string(),
        surface_container_low: "#F1F2E6".to_string(),
        surface_container: "#E2E3D8".to_string(),
        surface_container_high: "#D4D5CA".to_string(),
        surface_container_highest: "#C6C7BD".to_string(),
    };

    let dark_medium_contrast_scheme = MaterialScheme {
        primary: "#C7E79E".to_string(),
        surface_tint: "#B1D18A".to_string(),
        on_primary: "#172B00".to_string(),
        primary_container: "#7D9A59".to_string(),
        on_primary_container: "#000000".to_string(),
        secondary: "#D5E1C2".to_string(),
        on_secondary: "#1F2814".to_string(),
        secondary_container: "#8A9579".to_string(),
        on_secondary_container: "#000000".to_string(),
        tertiary: "#B5E6E1".to_string(),
        on_tertiary: "#002B29".to_string(),
        tertiary_container: "#6B9995".to_string(),
        on_tertiary_container: "#000000".to_string(),
        error: "#FFD2CC".to_string(),
        on_error: "#540003".to_string(),
        error_container: "#FF5449".to_string(),
        on_error_container: "#000000".to_string(),
        background: "#12140E".to_string(),
        on_background: "#E2E3D8".to_string(),
        surface: "#12140E".to_string(),
        on_surface: "#FFFFFF".to_string(),
        surface_variant: "#44483D".to_string(),
        on_surface_variant: "#DBDECF".to_string(),
        outline: "#B0B3A6".to_string(),
        outline_variant: "#8E9285".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#E2E3D8".to_string(),
        inverse_on_surface: "#282B24".to_string(),
        inverse_primary: "#364F17".to_string(),
        primary_fixed: "#CDEDA3".to_string(),
        on_primary_fixed: "#081400".to_string(),
        primary_fixed_dim: "#B1D18A".to_string(),
        on_primary_fixed_variant: "#253D05".to_string(),
        secondary_fixed: "#DCE7C8".to_string(),
        on_secondary_fixed: "#0B1403".to_string(),
        secondary_fixed_dim: "#BFCBAD".to_string(),
        on_secondary_fixed_variant: "#303924".to_string(),
        tertiary_fixed: "#BCECE7".to_string(),
        on_tertiary_fixed: "#001413".to_string(),
        tertiary_fixed_dim: "#A0D0CB".to_string(),
        on_tertiary_fixed_variant: "#083D3A".to_string(),
        surface_dim: "#12140E".to_string(),
        surface_bright: "#43453D".to_string(),
        surface_container_lowest: "#060804".to_string(),
        surface_container_low: "#1C1E18".to_string(),
        surface_container: "#262922".to_string(),
        surface_container_high: "#31342C".to_string(),
        surface_container_highest: "#3C3F37".to_string(),
    };

    let dark_high_contrast_scheme = MaterialScheme {
        primary: "#DAFBB0".to_string(),
        surface_tint: "#B1D18A".to_string(),
        on_primary: "#000000".to_string(),
        primary_container: "#ADCD86".to_string(),
        on_primary_container: "#050E00".to_string(),
        secondary: "#E9F4D5".to_string(),
        on_secondary: "#000000".to_string(),
        secondary_container: "#BCC7A9".to_string(),
        on_secondary_container: "#060D01".to_string(),
        tertiary: "#C9F9F5".to_string(),
        on_tertiary: "#000000".to_string(),
        tertiary_container: "#9CCCC7".to_string(),
        on_tertiary_container: "#000E0D".to_string(),
        error: "#FFECE9".to_string(),
        on_error: "#000000".to_string(),
        error_container: "#FFAEA4".to_string(),
        on_error_container: "#220001".to_string(),
        background: "#12140E".to_string(),
        on_background: "#E2E3D8".to_string(),
        surface: "#12140E".to_string(),
        on_surface: "#FFFFFF".to_string(),
        surface_variant: "#44483D".to_string(),
        on_surface_variant: "#FFFFFF".to_string(),
        outline: "#EEF2E2".to_string(),
        outline_variant: "#C1C4B6".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#E2E3D8".to_string(),
        inverse_on_surface: "#000000".to_string(),
        inverse_primary: "#364F17".to_string(),
        primary_fixed: "#CDEDA3".to_string(),
        on_primary_fixed: "#000000".to_string(),
        primary_fixed_dim: "#B1D18A".to_string(),
        on_primary_fixed_variant: "#081400".to_string(),
        secondary_fixed: "#DCE7C8".to_string(),
        on_secondary_fixed: "#000000".to_string(),
        secondary_fixed_dim: "#BFCBAD".to_string(),
        on_secondary_fixed_variant: "#0B1403".to_string(),
        tertiary_fixed: "#BCECE7".to_string(),
        on_tertiary_fixed: "#000000".to_string(),
        tertiary_fixed_dim: "#A0D0CB".to_string(),
        on_tertiary_fixed_variant: "#001413".to_string(),
        surface_dim: "#12140E".to_string(),
        surface_bright: "#4F5149".to_string(),
        surface_container_lowest: "#000000".to_string(),
        surface_container_low: "#1E201A".to_string(),
        surface_container: "#2F312A".to_string(),
        surface_container_high: "#3A3C35".to_string(),
        surface_container_highest: "#454840".to_string(),
    };

    let mut schemes = HashMap::new();
    schemes.insert("light".to_string(), light_scheme);
    schemes.insert(
        "light-medium-contrast".to_string(),
        light_medium_contrast_scheme,
    );
    schemes.insert(
        "light-high-contrast".to_string(),
        light_high_contrast_scheme,
    );
    schemes.insert("dark".to_string(), dark_scheme);
    schemes.insert(
        "dark-medium-contrast".to_string(),
        dark_medium_contrast_scheme,
    );
    schemes.insert("dark-high-contrast".to_string(), dark_high_contrast_scheme);

    let mut core_colors = HashMap::new();
    core_colors.insert("primary".to_string(), "#5C883A".to_string());

    MaterialThemeFile {
        description: "TYPE: CUSTOM Material Theme Builder export 2025-08-21 11:51:45".to_string(),
        seed: "#5C883A".to_string(),
        core_colors,
        extended_colors: Vec::new(),
        schemes,
        palettes: HashMap::new(),
    }
}

impl MaterialThemeContext {
    pub fn setup_fonts(font_name: Option<&str>) {
        let font_name = font_name.unwrap_or("Google Sans Code");

        // Check if font exists in resources directory first
        let font_file_path = format!(
            "resources/{}.ttf",
            font_name.replace(" ", "-").to_lowercase()
        );

        let font_data = if std::path::Path::new(&font_file_path).exists() {
            // Use local font file with include_bytes!
            Self::load_local_font(&font_file_path)
        } else {
            // Download font from Google Fonts at runtime (only if ondemand feature is enabled)
            #[cfg(feature = "ondemand")]
            {
                Self::download_google_font(font_name)
            }
            #[cfg(not(feature = "ondemand"))]
            {
                eprintln!(
                    "Font '{}' not found locally and ondemand feature is not enabled",
                    font_name
                );
                None
            }
        };

        if let Some(data) = font_data {
            let font_family_name = font_name.replace(" ", "");

            let prepared_font = PreparedFont {
                name: font_family_name.clone(),
                data: Arc::new(FontData::from_owned(data)),
                families: vec![FontFamily::Proportional, FontFamily::Monospace],
            };

            if let Ok(mut fonts) = PREPARED_FONTS.lock() {
                // Remove any existing font with the same name
                fonts.retain(|f| f.name != font_family_name);
                fonts.push(prepared_font);
            }
        }
    }

    fn load_local_font(font_path: &str) -> Option<Vec<u8>> {
        std::fs::read(font_path).ok()
    }

    // On-demand font downloading feature - downloads Google Fonts at runtime when ondemand feature is enabled
    #[cfg(feature = "ondemand")]
    fn download_google_font(font_name: &str) -> Option<Vec<u8>> {
        // Convert font name to Google Fonts URL format
        let font_url_name = font_name.replace(" ", "+");

        // First, get the CSS file to find the actual font URL
        let css_url = format!(
            "https://fonts.googleapis.com/css2?family={}:wght@400&display=swap",
            font_url_name
        );

        match ureq::get(&css_url)
            .set(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .call()
        {
            Ok(response) => {
                let css_content = response.into_string().ok()?;

                // Parse CSS to find TTF URL
                let font_url = Self::extract_font_url_from_css(&css_content)?;

                // Download the actual font file
                match ureq::get(&font_url).call() {
                    Ok(font_response) => {
                        let mut font_data = Vec::new();
                        if font_response
                            .into_reader()
                            .read_to_end(&mut font_data)
                            .is_ok()
                        {
                            // Save font to resources directory for future use
                            let target_path = format!(
                                "resources/{}.ttf",
                                font_name.replace(" ", "-").to_lowercase()
                            );
                            if let Ok(()) = std::fs::write(&target_path, &font_data) {
                                eprintln!(
                                    "Font '{}' downloaded and saved to {}",
                                    font_name, target_path
                                );
                            }
                            Some(font_data)
                        } else {
                            eprintln!("Failed to read font data for '{}'", font_name);
                            None
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to download font '{}': {}", font_name, e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch CSS for font '{}': {}", font_name, e);
                None
            }
        }
    }

    #[cfg(feature = "ondemand")]
    fn extract_font_url_from_css(css_content: &str) -> Option<String> {
        // Look for TTF URLs in the CSS content
        // Google Fonts CSS contains lines like: src: url(https://fonts.gstatic.com/...) format('truetype');
        for line in css_content.lines() {
            if line.contains("src:") && line.contains("url(") && line.contains("format('truetype')")
            {
                if let Some(start) = line.find("url(") {
                    let start = start + 4; // Skip "url("
                    if let Some(end) = line[start..].find(")") {
                        let url = &line[start..start + end];
                        return Some(url.to_string());
                    }
                }
            }
        }
        None
    }

    pub fn setup_local_fonts(font_path: Option<&str>) {
        // let default_material_symbols_path =
        //     "resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf";

        // Load custom text font if provided
        if let Some(path) = font_path {
            if std::path::Path::new(path).exists() {
                if let Ok(data) = std::fs::read(path) {
                    // Derive font name from filename (e.g. "noto-sans-kr.ttf" -> "NotoSansKr")
                    let font_name = std::path::Path::new(path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("CustomFont")
                        .split(['-', '_'])
                        .map(|part| {
                            let mut chars = part.chars();
                            match chars.next() {
                                Some(first) => {
                                    let upper: String = first.to_uppercase().collect();
                                    format!("{}{}", upper, chars.as_str())
                                }
                                None => String::new(),
                            }
                        })
                        .collect::<String>();

                    let prepared_font = PreparedFont {
                        name: font_name.clone(),
                        data: Arc::new(FontData::from_owned(data)),
                        families: vec![FontFamily::Proportional, FontFamily::Monospace],
                    };

                    if let Ok(mut fonts) = PREPARED_FONTS.lock() {
                        fonts.retain(|f| f.name != font_name);
                        fonts.push(prepared_font);
                    }
                }
            }
        }

        // Always load Material Symbols icon font
        // let icon_font_data = if std::path::Path::new(default_material_symbols_path).exists() {
        //     std::fs::read(default_material_symbols_path).ok()
        // } else {
        //     Self::get_embedded_material_symbols()
        // };

        // if let Some(data) = icon_font_data {
        //     let prepared_font = PreparedFont {
        //         name: "MaterialSymbolsOutlined".to_owned(),
        //         data: Arc::new(FontData::from_owned(data)),
        //         families: vec![FontFamily::Proportional, FontFamily::Monospace],
        //     };

        //     if let Ok(mut fonts) = PREPARED_FONTS.lock() {
        //         fonts.retain(|f| f.name != "MaterialSymbolsOutlined");
        //         fonts.push(prepared_font);
        //     }
        // }
    }

    pub fn setup_local_fonts_from_bytes(font_name: &str, font_data: &[u8]) {
        let prepared_font = PreparedFont {
            name: font_name.to_owned(),
            data: Arc::new(FontData::from_owned(font_data.to_vec())),
            families: vec![FontFamily::Proportional, FontFamily::Monospace],
        };

        if let Ok(mut fonts) = PREPARED_FONTS.lock() {
            fonts.retain(|f| f.name != font_name);
            fonts.push(prepared_font);
        }
    }


    /// Internal implementation for preparing local themes from JSON files
    ///
    /// This function handles the loading and parsing of Material Design theme JSON files.
    /// It supports both runtime file loading and build-time constant inclusion.
    ///
    /// # Arguments
    /// * `theme_path` - Optional path to theme JSON file. If None, uses build-time constants.
    ///
    /// # Implementation Details
    /// - First attempts to load from specified file path (if provided)
    /// - Falls back to build-time included theme constants
    /// - Finally falls back to default built-in theme
    /// - Parses JSON and stores in static PREPARED_THEMES collection
    /// - Replaces any existing theme with the same name
    pub fn setup_local_theme(theme_path: Option<&str>) {
        let theme_data = if let Some(path) = theme_path {
            // Try to load custom theme from path
            if std::path::Path::new(path).exists() {
                std::fs::read_to_string(path).ok()
            } else {
                // Fall back to embedded theme files or default theme
                Self::get_embedded_theme_data(path).or_else(|| {
                    Some(serde_json::to_string(&get_default_material_theme()).unwrap_or_default())
                })
            }
        } else {
            // Use embedded theme data first, then fall back to default
            Self::get_embedded_theme_data("resources/material-theme1.json").or_else(|| {
                Some(serde_json::to_string(&get_default_material_theme()).unwrap_or_default())
            })
        };

        // Parse and prepare theme if available
        if let Some(data) = theme_data {
            if let Ok(theme_file) = serde_json::from_str::<MaterialThemeFile>(&data) {
                let theme_name = theme_path
                    .and_then(|p| {
                        std::path::Path::new(p)
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                    })
                    .unwrap_or_else(|| "default".to_string());

                let prepared_theme = PreparedTheme {
                    name: theme_name.clone(),
                    theme_data: theme_file,
                };

                if let Ok(mut themes) = PREPARED_THEMES.lock() {
                    // Remove any existing theme with the same name
                    themes.retain(|t| t.name != theme_name);
                    themes.push(prepared_theme);
                }
            }
        }
    }

    // Build-time theme embedding system - includes theme JSON files as string constants for optimal performance
    fn get_embedded_theme_data(theme_path: &str) -> Option<String> {
        // For published packages, theme files are not included so we fallback to runtime loading
        // Users should provide their own theme files or use the default programmatic theme
        std::fs::read_to_string(theme_path).ok()
    }

    /// Internal implementation for loading prepared themes to the global theme context
    ///
    /// This function applies the first prepared theme from the PREPARED_THEMES collection
    /// as the active global theme. It creates a new MaterialThemeContext with the theme
    /// data and updates the global theme state.
    ///
    /// # Behavior
    /// - Takes the first theme from prepared themes collection
    /// - Creates a MaterialThemeContext with default settings (Light mode, Normal contrast)
    /// - Updates the global GLOBAL_THEME with the new context
    /// - If no themes were prepared, the global theme remains unchanged
    pub fn load_themes() {
        if let Ok(prepared_themes) = PREPARED_THEMES.lock() {
            if let Some(theme) = prepared_themes.first() {
                // Load the first prepared theme as the active theme
                let theme_context = MaterialThemeContext {
                    material_theme: Some(theme.theme_data.clone()),
                    ..Default::default()
                };
                update_global_theme(theme_context);
            }
        }
    }

    /// Load all prepared fonts to the egui context
    pub fn load_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();

        if let Ok(prepared_fonts) = PREPARED_FONTS.lock() {
            for prepared_font in prepared_fonts.iter() {
                // Add font data
                fonts
                    .font_data
                    .insert(prepared_font.name.clone(), prepared_font.data.clone());

                // Add to font families
                for family in &prepared_font.families {
                    match family {
                        FontFamily::Proportional => {
                            // Google fonts go to the front, icon fonts go to the back
                            if prepared_font.name.contains("MaterialSymbols") {
                                fonts
                                    .families
                                    .entry(FontFamily::Proportional)
                                    .or_default()
                                    .push(prepared_font.name.clone());
                            } else {
                                fonts
                                    .families
                                    .entry(FontFamily::Proportional)
                                    .or_default()
                                    .insert(0, prepared_font.name.clone());
                            }
                        }
                        FontFamily::Monospace => {
                            fonts
                                .families
                                .entry(FontFamily::Monospace)
                                .or_default()
                                .push(prepared_font.name.clone());
                        }
                        _ => {}
                    }
                }
            }
        }

        ctx.set_fonts(fonts);
    }

    pub fn get_current_scheme(&self) -> Option<&MaterialScheme> {
        if let Some(ref theme) = self.material_theme {
            let scheme_key = match (self.theme_mode, self.contrast_level) {
                (ThemeMode::Light, ContrastLevel::Normal) => "light",
                (ThemeMode::Light, ContrastLevel::Medium) => "light-medium-contrast",
                (ThemeMode::Light, ContrastLevel::High) => "light-high-contrast",
                (ThemeMode::Dark, ContrastLevel::Normal) => "dark",
                (ThemeMode::Dark, ContrastLevel::Medium) => "dark-medium-contrast",
                (ThemeMode::Dark, ContrastLevel::High) => "dark-high-contrast",
                (ThemeMode::Auto, contrast) => {
                    // For auto mode, we'll default to light for now
                    match contrast {
                        ContrastLevel::Normal => "light",
                        ContrastLevel::Medium => "light-medium-contrast",
                        ContrastLevel::High => "light-high-contrast",
                    }
                }
            };
            theme.schemes.get(scheme_key)
        } else {
            None
        }
    }

    pub fn hex_to_color32(hex: &str) -> Option<Color32> {
        if hex.starts_with('#') && hex.len() == 7 {
            if let Ok(r) = u8::from_str_radix(&hex[1..3], 16) {
                if let Ok(g) = u8::from_str_radix(&hex[3..5], 16) {
                    if let Ok(b) = u8::from_str_radix(&hex[5..7], 16) {
                        return Some(Color32::from_rgb(r, g, b));
                    }
                }
            }
        }
        None
    }

    pub fn color32_to_hex(color: Color32) -> String {
        format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
    }

    pub fn get_color_by_name(&self, name: &str) -> Color32 {
        if let Some(color) = self.selected_colors.get(name) {
            return *color;
        }

        if let Some(scheme) = self.get_current_scheme() {
            let hex = match name {
                "primary" => &scheme.primary,
                "surfaceTint" => &scheme.surface_tint,
                "onPrimary" => &scheme.on_primary,
                "primaryContainer" => &scheme.primary_container,
                "onPrimaryContainer" => &scheme.on_primary_container,
                "secondary" => &scheme.secondary,
                "onSecondary" => &scheme.on_secondary,
                "secondaryContainer" => &scheme.secondary_container,
                "onSecondaryContainer" => &scheme.on_secondary_container,
                "tertiary" => &scheme.tertiary,
                "onTertiary" => &scheme.on_tertiary,
                "tertiaryContainer" => &scheme.tertiary_container,
                "onTertiaryContainer" => &scheme.on_tertiary_container,
                "error" => &scheme.error,
                "onError" => &scheme.on_error,
                "errorContainer" => &scheme.error_container,
                "onErrorContainer" => &scheme.on_error_container,
                "background" => &scheme.background,
                "onBackground" => &scheme.on_background,
                "surface" => &scheme.surface,
                "onSurface" => &scheme.on_surface,
                "surfaceVariant" => &scheme.surface_variant,
                "onSurfaceVariant" => &scheme.on_surface_variant,
                "outline" => &scheme.outline,
                "outlineVariant" => &scheme.outline_variant,
                "shadow" => &scheme.shadow,
                "scrim" => &scheme.scrim,
                "inverseSurface" => &scheme.inverse_surface,
                "inverseOnSurface" => &scheme.inverse_on_surface,
                "inversePrimary" => &scheme.inverse_primary,
                "primaryFixed" => &scheme.primary_fixed,
                "onPrimaryFixed" => &scheme.on_primary_fixed,
                "primaryFixedDim" => &scheme.primary_fixed_dim,
                "onPrimaryFixedVariant" => &scheme.on_primary_fixed_variant,
                "secondaryFixed" => &scheme.secondary_fixed,
                "onSecondaryFixed" => &scheme.on_secondary_fixed,
                "secondaryFixedDim" => &scheme.secondary_fixed_dim,
                "onSecondaryFixedVariant" => &scheme.on_secondary_fixed_variant,
                "tertiaryFixed" => &scheme.tertiary_fixed,
                "onTertiaryFixed" => &scheme.on_tertiary_fixed,
                "tertiaryFixedDim" => &scheme.tertiary_fixed_dim,
                "onTertiaryFixedVariant" => &scheme.on_tertiary_fixed_variant,
                "surfaceDim" => &scheme.surface_dim,
                "surfaceBright" => &scheme.surface_bright,
                "surfaceContainerLowest" => &scheme.surface_container_lowest,
                "surfaceContainerLow" => &scheme.surface_container_low,
                "surfaceContainer" => &scheme.surface_container,
                "surfaceContainerHigh" => &scheme.surface_container_high,
                "surfaceContainerHighest" => &scheme.surface_container_highest,
                _ => return Color32::GRAY, // fallback
            };

            Self::hex_to_color32(hex).unwrap_or(Color32::GRAY)
        } else {
            // Fallback colors when no theme is loaded (using material-theme4.json light values)
            match name {
                "primary" => Color32::from_rgb(72, 103, 47), // #48672F
                "surfaceTint" => Color32::from_rgb(72, 103, 47), // #48672F
                "onPrimary" => Color32::WHITE,               // #FFFFFF
                "primaryContainer" => Color32::from_rgb(200, 238, 168), // #C8EEA8
                "onPrimaryContainer" => Color32::from_rgb(49, 79, 25), // #314F19
                "secondary" => Color32::from_rgb(86, 98, 75), // #56624B
                "onSecondary" => Color32::WHITE,             // #FFFFFF
                "secondaryContainer" => Color32::from_rgb(218, 231, 201), // #DAE7C9
                "onSecondaryContainer" => Color32::from_rgb(63, 74, 52), // #3F4A34
                "tertiary" => Color32::from_rgb(56, 102, 101), // #386665
                "onTertiary" => Color32::WHITE,              // #FFFFFF
                "tertiaryContainer" => Color32::from_rgb(187, 236, 234), // #BBECEA
                "onTertiaryContainer" => Color32::from_rgb(30, 78, 77), // #1E4E4D
                "error" => Color32::from_rgb(186, 26, 26),   // #BA1A1A
                "onError" => Color32::WHITE,                 // #FFFFFF
                "errorContainer" => Color32::from_rgb(255, 218, 214), // #FFDAD6
                "onErrorContainer" => Color32::from_rgb(147, 0, 10), // #93000A
                "background" => Color32::from_rgb(249, 250, 239), // #F9FAEF
                "onBackground" => Color32::from_rgb(25, 29, 22), // #191D16
                "surface" => Color32::from_rgb(249, 250, 239), // #F9FAEF
                "onSurface" => Color32::from_rgb(25, 29, 22), // #191D16
                "surfaceVariant" => Color32::from_rgb(224, 228, 214), // #E0E4D6
                "onSurfaceVariant" => Color32::from_rgb(68, 72, 62), // #44483E
                "outline" => Color32::from_rgb(116, 121, 109), // #74796D
                "outlineVariant" => Color32::from_rgb(196, 200, 186), // #C4C8BA
                "shadow" => Color32::BLACK,                  // #000000
                "scrim" => Color32::BLACK,                   // #000000
                "inverseSurface" => Color32::from_rgb(46, 49, 42), // #2E312A
                "inverseOnSurface" => Color32::from_rgb(240, 242, 231), // #F0F2E7
                "inversePrimary" => Color32::from_rgb(173, 210, 142), // #ADD28E
                "primaryFixed" => Color32::from_rgb(200, 238, 168), // #C8EEA8
                "onPrimaryFixed" => Color32::from_rgb(11, 32, 0), // #0B2000
                "primaryFixedDim" => Color32::from_rgb(173, 210, 142), // #ADD28E
                "onPrimaryFixedVariant" => Color32::from_rgb(49, 79, 25), // #314F19
                "secondaryFixed" => Color32::from_rgb(218, 231, 201), // #DAE7C9
                "onSecondaryFixed" => Color32::from_rgb(20, 30, 12), // #141E0C
                "secondaryFixedDim" => Color32::from_rgb(190, 203, 174), // #BECBAE
                "onSecondaryFixedVariant" => Color32::from_rgb(63, 74, 52), // #3F4A34
                "tertiaryFixed" => Color32::from_rgb(187, 236, 234), // #BBECEA
                "onTertiaryFixed" => Color32::from_rgb(0, 32, 31), // #00201F
                "tertiaryFixedDim" => Color32::from_rgb(160, 207, 206), // #A0CFCE
                "onTertiaryFixedVariant" => Color32::from_rgb(30, 78, 77), // #1E4E4D
                "surfaceDim" => Color32::from_rgb(217, 219, 209), // #D9DBD1
                "surfaceBright" => Color32::from_rgb(249, 250, 239), // #F9FAEF
                "surfaceContainerLowest" => Color32::WHITE,  // #FFFFFF
                "surfaceContainerLow" => Color32::from_rgb(243, 245, 234), // #F3F5EA
                "surfaceContainer" => Color32::from_rgb(237, 239, 228), // #EDEFE4
                "surfaceContainerHigh" => Color32::from_rgb(231, 233, 222), // #E7E9DE
                "surfaceContainerHighest" => Color32::from_rgb(226, 227, 217), // #E2E3D9
                _ => Color32::GRAY,
            }
        }
    }

    pub fn get_primary_color(&self) -> Color32 {
        self.get_color_by_name("primary")
    }

    pub fn get_secondary_color(&self) -> Color32 {
        self.get_color_by_name("secondary")
    }

    pub fn get_tertiary_color(&self) -> Color32 {
        self.get_color_by_name("tertiary")
    }

    pub fn get_surface_color(&self, _dark_mode: bool) -> Color32 {
        self.get_color_by_name("surface")
    }

    pub fn get_on_primary_color(&self) -> Color32 {
        self.get_color_by_name("onPrimary")
    }
}

// Global theme context accessible by all components
static GLOBAL_THEME: std::sync::LazyLock<Arc<Mutex<MaterialThemeContext>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(MaterialThemeContext::default())));

pub fn get_global_theme() -> Arc<Mutex<MaterialThemeContext>> {
    GLOBAL_THEME.clone()
}

/// Update the global theme context with a new theme configuration
///
/// This function replaces the current global theme context with a new one.
/// It's used internally by the theme system and can be used by applications
/// to programmatically change theme settings at runtime.
///
/// # Arguments
/// * `theme` - The new MaterialThemeContext to set as the global theme
///
/// # Usage
/// This function is typically called by:
/// - `load_themes()` - To apply a loaded theme as the global theme
/// - Application code - To change theme mode, contrast level, or selected colors at runtime
///
/// # Thread Safety
/// This function is thread-safe and uses a mutex to ensure exclusive access
/// to the global theme state.
///
/// # Example
/// ```rust
/// let mut theme_context = MaterialThemeContext::default();
/// theme_context.theme_mode = ThemeMode::Dark;
/// theme_context.contrast_level = ContrastLevel::High;
/// update_global_theme(theme_context);
/// ```
pub fn update_global_theme(theme: MaterialThemeContext) {
    if let Ok(mut global_theme) = GLOBAL_THEME.lock() {
        *global_theme = theme;
    }
}

/// Helper function to prepare Material Design fonts for the application
/// Default font is "Google Sans Code" if not specified
/// Note: Fonts are only prepared, call load_fonts() to actually load them
pub fn setup_google_fonts(font_name: Option<&str>) {
    MaterialThemeContext::setup_fonts(font_name);
}

/// Helper function to prepare local fonts from the resources directory
///
/// # Arguments
/// * `font_path` - Optional path to a TTF font file. If None, uses the default MaterialSymbolsOutlined font
///
///   Note: Fonts are only prepared, call load_fonts() to actually load them
pub fn setup_local_fonts(font_path: Option<&str>) {
    MaterialThemeContext::setup_local_fonts(font_path);
}

/// Prepare a local font from pre-loaded byte data
///
/// # Arguments
/// * `font_name` - Name to register the font under (must not contain "MaterialSymbols" for text fonts)
/// * `font_data` - Raw TTF/OTF font bytes (e.g. from `include_bytes!`)
///
/// Note: Fonts are only prepared, call load_fonts() to actually load them
pub fn setup_local_fonts_from_bytes(font_name: &str, font_data: &[u8]) {
    MaterialThemeContext::setup_local_fonts_from_bytes(font_name, font_data);
}

/// Prepare local Material Design themes for the application from JSON files
///
/// This function loads Material Design theme data from JSON files and prepares them for use.
/// Theme data is included at build-time when using the default behavior (None path), or loaded
/// at runtime when a specific path is provided.
///
/// # Arguments
/// * `theme_path` - Optional path to a Material Design theme JSON file:
///   - `Some(path)` - Load theme from the specified file path at runtime
///   - `None` - Use themes that were included at build-time from the build script scan
///
/// # Build-time Theme Inclusion
/// When `theme_path` is `None`, the build script automatically scans for JSON files in:
/// - `resources/` directory
/// - `examples/` directory
///
/// Files matching `*theme*.json` or `*material-theme*.json` patterns are included as constants.
///
/// # Example
/// ```rust
/// // Use build-time included themes (recommended for production)
/// setup_local_theme(None);
///
/// // Load specific theme file at runtime (useful for development/testing)
/// setup_local_theme(Some("resources/my-custom-theme.json"));
/// setup_local_theme(Some("examples/material-theme6.json"));
/// ```
///
/// # Note
/// Themes are only prepared by this function. Call `load_themes()` after this to actually
/// apply the prepared themes to the global theme context.
pub fn setup_local_theme(theme_path: Option<&str>) {
    MaterialThemeContext::setup_local_theme(theme_path);
}

/// Load all prepared themes to the global theme context
///
/// This function takes themes that were prepared by `setup_local_theme()` and applies
/// the first prepared theme as the active global theme. This makes the theme available
/// to all Material Design components throughout the application.
///
/// # Usage
/// This should be called after all `setup_local_theme()` calls and typically during
/// application initialization.
///
/// # Example
/// ```rust
/// // Setup and load themes during app initialization
/// setup_local_theme(Some("resources/my-theme.json"));
/// load_themes();  // Apply the prepared theme globally
/// ```
///
/// # Behavior
/// - If multiple themes were prepared, only the first one becomes active
/// - If no themes were prepared, the default built-in theme is used
/// - The active theme becomes available via `get_global_color()` and other theme functions
pub fn load_themes() {
    MaterialThemeContext::load_themes();
}

/// Load a Material Design theme directly from a JSON string
///
/// This function parses a Material Design theme JSON string and applies it to the global theme context.
/// This is useful for loading embedded theme data or dynamically generated themes.
///
/// # Parameters
/// - `json_data`: A JSON string containing the Material Design theme data
///
/// # Returns
/// - `Ok(())` if the theme was successfully loaded
/// - `Err(String)` with an error message if parsing or loading failed
///
/// # Example
/// ```rust,no_run
/// use egui_material3::theme::load_theme_from_json_str;
///
/// const MY_THEME: &str = include_str!("../resources/my-theme.json");
///
/// // Load the theme
/// if let Err(e) = load_theme_from_json_str(MY_THEME) {
///     eprintln!("Failed to load theme: {}", e);
/// }
/// ```
pub fn load_theme_from_json_str(json_data: &str) -> Result<(), String> {
    let theme_file = serde_json::from_str::<MaterialThemeFile>(json_data)
        .map_err(|e| format!("Failed to parse theme JSON: {}", e))?;

    if let Ok(mut global_theme) = get_global_theme().lock() {
        global_theme.material_theme = Some(theme_file);
        global_theme.selected_colors.clear();
        Ok(())
    } else {
        Err("Failed to acquire theme lock".to_string())
    }
}

/// Trait to provide a unified interface for accessing egui Context
pub trait ContextRef {
    fn context_ref(&self) -> &egui::Context;
}

impl ContextRef for egui::Context {
    fn context_ref(&self) -> &egui::Context {
        self
    }
}

impl ContextRef for &egui::Context {
    fn context_ref(&self) -> &egui::Context {
        self
    }
}

/// Load all prepared fonts to the egui context
/// Call this after all setup_*_fonts functions to actually load the fonts
pub fn load_fonts<C: ContextRef>(ctx: C) {
    MaterialThemeContext::load_fonts(ctx.context_ref());
}

/// Update the window/panel background colors based on the current theme
///
/// This function automatically applies the appropriate background colors from the current
/// Material Design theme to the egui context. The background color is selected based on
/// the current theme mode (Light/Dark/Auto) and contrast level (Normal/Medium/High).
///
/// # Arguments
/// * `ctx` - The egui context to update with new background colors
///
/// # Background Color Selection
/// The function selects background colors according to Material Design guidelines:
///
/// **Dark Theme:**
/// - High contrast: `surfaceContainerHighest`
/// - Medium contrast: `surfaceContainerHigh`
/// - Normal contrast: `surface`
///
/// **Light Theme:**
/// - High contrast: `surfaceContainerLowest`
/// - Medium contrast: `surfaceContainerLow`
/// - Normal contrast: `surface`
///
/// **Auto Theme:** Uses `surface` as default
///
/// # Usage
/// This function should be called:
/// - Once during application initialization (after `load_themes()`)
/// - Whenever theme settings change (mode or contrast level)
///
/// # Example
/// ```rust
/// // During app initialization in eframe::run_native
/// setup_local_theme(Some("my-theme.json"));
/// load_themes();
/// update_window_background(&cc.egui_ctx);  // Apply initial background
///
/// // When theme settings change at runtime
/// fn change_theme_mode(&mut self, ctx: &egui::Context) {
///     // Update theme mode in global context...
///     update_window_background(ctx);  // Apply new background
/// }
/// ```
///
/// # Effects
/// This function updates the following egui visual properties:
/// - `window_fill` - Background color for floating windows
/// - `panel_fill` - Background color for side panels and central panel  
/// - `extreme_bg_color` - Background color for extreme contrast areas
pub fn update_window_background<C: ContextRef>(ctx: C) {
    let ctx = ctx.context_ref();
    if let Ok(theme) = GLOBAL_THEME.lock() {
        // Resolve Auto mode to actual mode based on context
        let mut resolved_theme = theme.clone();
        if resolved_theme.theme_mode == ThemeMode::Auto {
            if ctx.style().visuals.dark_mode {
                resolved_theme.theme_mode = ThemeMode::Dark;
            } else {
                resolved_theme.theme_mode = ThemeMode::Light;
            }
        }

        // Get the appropriate background color from the material theme
        let background_color = match (resolved_theme.theme_mode, resolved_theme.contrast_level) {
            (ThemeMode::Dark, ContrastLevel::High) => {
                resolved_theme.get_color_by_name("surfaceContainerHighest")
            }
            (ThemeMode::Dark, ContrastLevel::Medium) => {
                resolved_theme.get_color_by_name("surfaceContainerHigh")
            }
            (ThemeMode::Dark, _) => resolved_theme.get_color_by_name("surface"),
            (ThemeMode::Light, ContrastLevel::High) => {
                resolved_theme.get_color_by_name("surfaceContainerLowest")
            }
            (ThemeMode::Light, ContrastLevel::Medium) => {
                resolved_theme.get_color_by_name("surfaceContainerLow")
            }
            (ThemeMode::Light, _) => resolved_theme.get_color_by_name("surface"),
            (ThemeMode::Auto, _) => resolved_theme.get_color_by_name("surface"), // Should be unreachable
        };

        // Apply the background color to the context
        let mut visuals = ctx.style().visuals.clone();
        visuals.window_fill = background_color;
        visuals.panel_fill = background_color;
        visuals.extreme_bg_color = background_color;

        let mut style = (*ctx.style()).clone();
        style.visuals = visuals;
        ctx.set_style(style);
    }
}

/// Helper function to get a color by name from the global theme
pub fn get_global_color(name: &str) -> Color32 {
    if let Ok(theme) = GLOBAL_THEME.lock() {
        theme.get_color_by_name(name)
    } else {
        // Fallback colors when theme is not accessible
        match name {
            "primary" => Color32::from_rgb(103, 80, 164),
            "onPrimary" => Color32::WHITE,
            "surface" => Color32::from_rgb(254, 247, 255),
            "onSurface" => Color32::from_rgb(28, 27, 31),
            "surfaceContainer" => Color32::from_rgb(247, 243, 249),
            "surfaceContainerHigh" => Color32::from_rgb(237, 231, 246),
            "surfaceContainerHighest" => Color32::from_rgb(230, 224, 233),
            "surfaceContainerLow" => Color32::from_rgb(247, 243, 249),
            "surfaceContainerLowest" => Color32::from_rgb(255, 255, 255),
            "outline" => Color32::from_rgb(121, 116, 126),
            "outlineVariant" => Color32::from_rgb(196, 199, 197),
            "surfaceVariant" => Color32::from_rgb(232, 222, 248),
            "secondary" => Color32::from_rgb(125, 82, 96),
            "tertiary" => Color32::from_rgb(125, 82, 96),
            "error" => Color32::from_rgb(186, 26, 26),
            "background" => Color32::from_rgb(255, 251, 254),
            "onBackground" => Color32::from_rgb(28, 27, 31),
            _ => Color32::GRAY,
        }
    }
}

/// Detect OS theme preference using the dark-light crate (desktop platforms only)
///
/// On Android, this function will return `ThemeMode::Light` as a fallback.
/// Android apps should provide their own theme detection using JNI.
///
/// # Returns
/// - `ThemeMode::Dark` if OS dark mode is detected
/// - `ThemeMode::Light` if OS light mode is detected or on unsupported platforms
pub fn detect_os_theme() -> ThemeMode {
    #[cfg(not(target_os = "android"))]
    {
        match dark_light::detect() {
            dark_light::Mode::Dark => ThemeMode::Dark,
            dark_light::Mode::Light => ThemeMode::Light,
            dark_light::Mode::Default => ThemeMode::Light,
        }
    }

    #[cfg(target_os = "android")]
    {
        // Default to Light on Android - apps should provide their own detector
        ThemeMode::Light
    }
}

/// Apply the Material Design 3 theme to the egui context
///
/// This function applies the current global theme to the egui visual system,
/// comprehensively mapping Material Design 3 color roles to egui visuals.
///
/// # Material Design 3 Color Mappings
///
/// ## Widget States
/// - **Noninteractive** (disabled): surface, surfaceVariant, outlineVariant, onSurfaceVariant
/// - **Inactive** (default): primary with low opacity, outline, onSurface
/// - **Hovered**: primary with medium opacity, outline, onSurface
/// - **Active** (pressed/selected): primary, onPrimary, primaryContainer
/// - **Open** (menus/dropdowns): primaryContainer, onPrimaryContainer, outline
///
/// ## Backgrounds
/// - **window_fill**: surface
/// - **panel_fill**: surfaceContainer
/// - **faint_bg_color**: surfaceContainerLow
/// - **extreme_bg_color**: surfaceContainerLowest
/// - **code_bg_color**: surfaceContainerHighest
///
/// ## Text & Interaction
/// - **override_text_color**: onSurface
/// - **text_cursor**: primary
/// - **hyperlink_color**: primary
/// - **selection**: primary
///
/// ## Feedback Colors
/// - **error_fg_color**: error
/// - **warn_fg_color**: tertiary
///
/// ## Strokes & Shadows
/// - **window_stroke**: outlineVariant
/// - **window_shadow**: shadow
/// - **popup_shadow**: shadow
///
/// # Parameters
/// - `ctx`: The egui context to apply the theme to
/// - `os_theme_detector`: Optional function to detect OS theme mode when `ThemeMode::Auto` is set.
///   If not provided, uses the default `detect_os_theme()` function.
///
/// # Example
/// ```rust,no_run
/// use egui_material3::theme::{apply_theme, ThemeMode};
///
/// // Use default OS theme detection
/// apply_theme(&egui_ctx, None);
///
/// // Provide custom theme detection (e.g., for Android)
/// apply_theme(&egui_ctx, Some(|| {
///     // Custom Android theme detection logic
///     ThemeMode::Dark
/// }));
/// ```
pub fn apply_theme<C, F>(ctx: C, os_theme_detector: Option<F>)
where
    C: ContextRef,
    F: FnOnce() -> ThemeMode,
{
    let ctx = ctx.context_ref();

    let mut theme = if let Ok(theme) = GLOBAL_THEME.lock() {
        theme.clone()
    } else {
        return;
    };

    let mut visuals = match theme.theme_mode {
        ThemeMode::Light => egui::Visuals::light(),
        ThemeMode::Dark => egui::Visuals::dark(),
        ThemeMode::Auto => {
            // Detect OS theme preference
            let detected_mode = if let Some(detector) = os_theme_detector {
                detector()
            } else {
                detect_os_theme()
            };
            theme.theme_mode = detected_mode; // Resolve Auto to detected OS theme
            match detected_mode {
                ThemeMode::Dark => egui::Visuals::dark(),
                _ => egui::Visuals::light(),
            }
        }
    };

    // Extract Material Design 3 color roles
    let primary = theme.get_primary_color();
    let on_primary = theme.get_on_primary_color();
    let primary_container = theme.get_color_by_name("primaryContainer");
    let on_primary_container = theme.get_color_by_name("onPrimaryContainer");

    let secondary = theme.get_color_by_name("secondary");
    let on_secondary = theme.get_color_by_name("onSecondary");
    let secondary_container = theme.get_color_by_name("secondaryContainer");

    let tertiary = theme.get_color_by_name("tertiary");

    let error = theme.get_color_by_name("error");
    let on_error = theme.get_color_by_name("onError");
    let error_container = theme.get_color_by_name("errorContainer");

    let surface = theme.get_surface_color(visuals.dark_mode);
    let on_surface = theme.get_color_by_name("onSurface");
    let surface_variant = theme.get_color_by_name("surfaceVariant");
    let on_surface_variant = theme.get_color_by_name("onSurfaceVariant");

    let surface_container = theme.get_color_by_name("surfaceContainer");
    let surface_container_high = theme.get_color_by_name("surfaceContainerHigh");
    let surface_container_highest = theme.get_color_by_name("surfaceContainerHighest");
    let surface_container_low = theme.get_color_by_name("surfaceContainerLow");
    let surface_container_lowest = theme.get_color_by_name("surfaceContainerLowest");

    let outline = theme.get_color_by_name("outline");
    let outline_variant = theme.get_color_by_name("outlineVariant");

    let inverse_surface = theme.get_color_by_name("inverseSurface");
    let inverse_on_surface = theme.get_color_by_name("inverseOnSurface");
    let inverse_primary = theme.get_color_by_name("inversePrimary");

    let shadow = theme.get_color_by_name("shadow");

    // === Selection colors ===
    visuals.selection.bg_fill = primary;
    visuals.selection.stroke.color = primary;

    // === Hyperlink ===
    visuals.hyperlink_color = primary;

    // === Widget colors (noninteractive, inactive, hovered, active, open) ===

    // Noninteractive widgets (disabled state)
    visuals.widgets.noninteractive.bg_fill = surface;
    visuals.widgets.noninteractive.weak_bg_fill = surface_variant;
    visuals.widgets.noninteractive.bg_stroke.color = outline_variant;
    visuals.widgets.noninteractive.fg_stroke.color = on_surface_variant;

    // Inactive widgets (default state)
    visuals.widgets.inactive.weak_bg_fill = surface_container_highest;
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(
        primary.r(),
        primary.g(),
        primary.b(),
        20,
    );
    visuals.widgets.inactive.bg_stroke.color = outline;
    visuals.widgets.inactive.fg_stroke.color = on_surface;

    // Hovered widgets
    visuals.widgets.hovered.weak_bg_fill = surface_container_high;
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(
        primary.r(),
        primary.g(),
        primary.b(),
        40,
    );
    visuals.widgets.hovered.bg_stroke.color = outline;
    visuals.widgets.hovered.fg_stroke.color = on_surface;

    // Active widgets (pressed/selected state)
    visuals.widgets.active.weak_bg_fill = primary_container;
    visuals.widgets.active.bg_fill = primary;
    visuals.widgets.active.bg_stroke.color = primary;
    visuals.widgets.active.fg_stroke.color = on_primary;

    // Open widgets (menus, dropdowns)
    visuals.widgets.open.weak_bg_fill = surface_container_low;
    visuals.widgets.open.bg_fill = primary_container;
    visuals.widgets.open.bg_stroke.color = outline;
    visuals.widgets.open.fg_stroke.color = on_primary_container;

    // === Background colors ===
    visuals.window_fill = surface;
    visuals.panel_fill = surface_container;
    visuals.faint_bg_color = surface_container_low;
    visuals.extreme_bg_color = surface_container_lowest;
    visuals.code_bg_color = surface_container_highest;

    // === Text colors ===
    visuals.override_text_color = Some(on_surface);
    visuals.text_cursor.stroke.color = primary;

    // === Error and warning colors ===
    visuals.error_fg_color = error;
    visuals.warn_fg_color = tertiary;

    // === Window stroke ===
    visuals.window_stroke.color = outline_variant;
    visuals.window_stroke.width = 1.0;

    // === Window shadow ===
    visuals.window_shadow.color = shadow;
    visuals.popup_shadow.color = shadow;

    ctx.set_visuals(visuals);
}

// ============================================================================
// Theme Management Utilities
// ============================================================================

/// Get the current theme mode from the global theme
///
/// # Returns
/// The current theme mode (Light, Dark, or Auto)
///
/// # Example
/// ```rust,no_run
/// use egui_material3::theme::get_theme_mode;
///
/// let mode = get_theme_mode();
/// println!("Current theme mode: {}", mode);
/// ```
pub fn get_theme_mode() -> ThemeMode {
    if let Ok(theme) = get_global_theme().lock() {
        theme.theme_mode
    } else {
        ThemeMode::Auto
    }
}

/// Set the theme mode in the global theme
///
/// # Parameters
/// - `mode`: The theme mode to set (Light, Dark, or Auto)
///
/// # Example
/// ```rust,no_run
/// use egui_material3::theme::{set_theme_mode, ThemeMode};
///
/// set_theme_mode(ThemeMode::Dark);
/// ```
pub fn set_theme_mode(mode: ThemeMode) {
    if let Ok(mut theme) = get_global_theme().lock() {
        theme.theme_mode = mode;
    }
}

/// Get the current contrast level from the global theme
///
/// # Returns
/// The current contrast level (Normal, Medium, or High)
///
/// # Example
/// ```rust,no_run
/// use egui_material3::theme::get_contrast_level;
///
/// let level = get_contrast_level();
/// println!("Current contrast level: {}", level);
/// ```
pub fn get_contrast_level() -> ContrastLevel {
    if let Ok(theme) = get_global_theme().lock() {
        theme.contrast_level
    } else {
        ContrastLevel::Normal
    }
}

/// Set the contrast level in the global theme
///
/// # Parameters
/// - `level`: The contrast level to set (Normal, Medium, or High)
///
/// # Example
/// ```rust,no_run
/// use egui_material3::theme::{set_contrast_level, ContrastLevel};
///
/// set_contrast_level(ContrastLevel::High);
/// ```
pub fn set_contrast_level(level: ContrastLevel) {
    if let Ok(mut theme) = get_global_theme().lock() {
        theme.contrast_level = level;
    }
}
