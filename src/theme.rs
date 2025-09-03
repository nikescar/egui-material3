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
//! // Load prepared fonts and themes
//! load_fonts(&egui_ctx);
//! load_themes();
//!
//! // Apply theme background
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
use std::io::Read;
use std::sync::{Arc, Mutex};

// Runtime font management - no more build-time includes

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Auto
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

    let mut schemes = HashMap::new();
    schemes.insert("light".to_string(), light_scheme);
    schemes.insert("dark".to_string(), dark_scheme);

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
        let font_file_path = format!("resources/{}.ttf", font_name.replace(" ", "-").to_lowercase());
        
        let font_data = if std::path::Path::new(&font_file_path).exists() {
            // Use local font file with include_bytes!
            Self::load_local_font(&font_file_path)
        } else {
            // Download font from Google Fonts at runtime
            Self::download_google_font(font_name)
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
    
    fn download_google_font(font_name: &str) -> Option<Vec<u8>> {
        // Convert font name to Google Fonts URL format
        let font_url_name = font_name.replace(" ", "+");
        
        // First, get the CSS file to find the actual font URL
        let css_url = format!("https://fonts.googleapis.com/css2?family={}:wght@400&display=swap", font_url_name);
        
        match ureq::get(&css_url)
            .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
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
                        if font_response.into_reader().read_to_end(&mut font_data).is_ok() {
                            // Save font to resources directory for future use
                            let target_path = format!("resources/{}.ttf", font_name.replace(" ", "-").to_lowercase());
                            if let Ok(()) = std::fs::write(&target_path, &font_data) {
                                eprintln!("Font '{}' downloaded and saved to {}", font_name, target_path);
                            }
                            Some(font_data)
                        } else {
                            eprintln!("Failed to read font data for '{}'", font_name);
                            None
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to download font '{}': {}", font_name, e);
                        None
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch CSS for font '{}': {}", font_name, e);
                None
            }
        }
    }
    
    fn extract_font_url_from_css(css_content: &str) -> Option<String> {
        // Look for TTF URLs in the CSS content
        // Google Fonts CSS contains lines like: src: url(https://fonts.gstatic.com/...) format('truetype');
        for line in css_content.lines() {
            if line.contains("src:") && line.contains("url(") && line.contains("format('truetype')") {
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
        let default_material_symbols_path = "resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf";
        
        // Determine which font to use
        let font_data = if let Some(path) = font_path {
            // Try to load custom font from path
            if std::path::Path::new(path).exists() {
                std::fs::read(path).ok()
            } else {
                // Fall back to default font if custom font doesn't exist
                if std::path::Path::new(default_material_symbols_path).exists() {
                    std::fs::read(default_material_symbols_path).ok()
                } else {
                    // Use include_bytes! as fallback if file exists in resources
                    Self::get_embedded_material_symbols()
                }
            }
        } else {
            // Use default Material Symbols Outlined font
            if std::path::Path::new(default_material_symbols_path).exists() {
                std::fs::read(default_material_symbols_path).ok()
            } else {
                // Use include_bytes! as fallback
                Self::get_embedded_material_symbols()
            }
        };
        
        // Prepare font if available
        if let Some(data) = font_data {
            let prepared_font = PreparedFont {
                name: "MaterialSymbolsOutlined".to_owned(),
                data: Arc::new(FontData::from_owned(data)),
                families: vec![FontFamily::Proportional, FontFamily::Monospace],
            };
            
            if let Ok(mut fonts) = PREPARED_FONTS.lock() {
                // Remove any existing font with the same name
                fonts.retain(|f| f.name != "MaterialSymbolsOutlined");
                fonts.push(prepared_font);
            }
        }
    }
    
    fn get_embedded_material_symbols() -> Option<Vec<u8>> {
        // Use include_bytes! to embed the font if it exists
        if std::path::Path::new("resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf").exists() {
            Some(include_bytes!("../resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf").to_vec())
        } else {
            None
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
                let theme_name = theme_path.and_then(|p| {
                    std::path::Path::new(p).file_stem().map(|s| s.to_string_lossy().to_string())
                }).unwrap_or_else(|| "default".to_string());
                
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
    
    fn get_embedded_theme_data(theme_path: &str) -> Option<String> {
        // Try to embed theme files using include_str! if they exist
        match theme_path {
            "resources/material-theme1.json" => {
                if std::path::Path::new("resources/material-theme1.json").exists() {
                    Some(include_str!("../resources/material-theme1.json").to_string())
                } else { None }
            },
            "resources/material-theme2.json" => {
                if std::path::Path::new("resources/material-theme2.json").exists() {
                    Some(include_str!("../resources/material-theme2.json").to_string())
                } else { None }
            },
            "resources/material-theme3.json" => {
                if std::path::Path::new("resources/material-theme3.json").exists() {
                    Some(include_str!("../resources/material-theme3.json").to_string())
                } else { None }
            },
            "resources/material-theme4.json" => {
                if std::path::Path::new("resources/material-theme4.json").exists() {
                    Some(include_str!("../resources/material-theme4.json").to_string())
                } else { None }
            },
            "resources/material-theme5.json" => {
                if std::path::Path::new("resources/material-theme5.json").exists() {
                    Some(include_str!("../resources/material-theme5.json").to_string())
                } else { None }
            },
            "resources/material-theme6.json" => {
                if std::path::Path::new("resources/material-theme6.json").exists() {
                    Some(include_str!("../resources/material-theme6.json").to_string())
                } else { None }
            },
            "resources/material-theme7.json" => {
                if std::path::Path::new("resources/material-theme7.json").exists() {
                    Some(include_str!("../resources/material-theme7.json").to_string())
                } else { None }
            },
            _ => {
                // For other paths, try to read from file system
                std::fs::read_to_string(theme_path).ok()
            }
        }
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
                fonts.font_data.insert(
                    prepared_font.name.clone(),
                    prepared_font.data.clone(),
                );
                
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
                "onPrimary" => Color32::WHITE, // #FFFFFF
                "primaryContainer" => Color32::from_rgb(200, 238, 168), // #C8EEA8
                "onPrimaryContainer" => Color32::from_rgb(49, 79, 25), // #314F19
                "secondary" => Color32::from_rgb(86, 98, 75), // #56624B
                "onSecondary" => Color32::WHITE, // #FFFFFF
                "secondaryContainer" => Color32::from_rgb(218, 231, 201), // #DAE7C9
                "onSecondaryContainer" => Color32::from_rgb(63, 74, 52), // #3F4A34
                "tertiary" => Color32::from_rgb(56, 102, 101), // #386665
                "onTertiary" => Color32::WHITE, // #FFFFFF
                "tertiaryContainer" => Color32::from_rgb(187, 236, 234), // #BBECEA
                "onTertiaryContainer" => Color32::from_rgb(30, 78, 77), // #1E4E4D
                "error" => Color32::from_rgb(186, 26, 26), // #BA1A1A
                "onError" => Color32::WHITE, // #FFFFFF
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
                "shadow" => Color32::BLACK, // #000000
                "scrim" => Color32::BLACK, // #000000
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
                "surfaceContainerLowest" => Color32::WHITE, // #FFFFFF
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
/// Note: Fonts are only prepared, call load_fonts() to actually load them
pub fn setup_local_fonts(font_path: Option<&str>) {
    MaterialThemeContext::setup_local_fonts(font_path);
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

/// Load all prepared fonts to the egui context
/// Call this after all setup_*_fonts functions to actually load the fonts
pub fn load_fonts(ctx: &egui::Context) {
    MaterialThemeContext::load_fonts(ctx);
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
pub fn update_window_background(ctx: &egui::Context) {
    if let Ok(theme) = GLOBAL_THEME.lock() {
        // Get the appropriate background color from the material theme
        let background_color = match (theme.theme_mode, theme.contrast_level) {
            (ThemeMode::Dark, ContrastLevel::High) => theme.get_color_by_name("surfaceContainerHighest"),
            (ThemeMode::Dark, ContrastLevel::Medium) => theme.get_color_by_name("surfaceContainerHigh"),
            (ThemeMode::Dark, _) => theme.get_color_by_name("surface"),
            (ThemeMode::Light, ContrastLevel::High) => theme.get_color_by_name("surfaceContainerLowest"),
            (ThemeMode::Light, ContrastLevel::Medium) => theme.get_color_by_name("surfaceContainerLow"),
            (ThemeMode::Light, _) => theme.get_color_by_name("surface"),
            (ThemeMode::Auto, _) => theme.get_color_by_name("surface"), // Default to surface for auto mode
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