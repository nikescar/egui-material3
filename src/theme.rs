use egui::{Color32, FontData, FontDefinitions, FontFamily};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

include!(concat!(env!("OUT_DIR"), "/google_fonts.rs"));

/// Global collection of prepared fonts before loading to context
#[derive(Debug, Clone)]
pub struct PreparedFont {
    pub name: String,
    pub data: Arc<FontData>,
    pub families: Vec<FontFamily>,
}

static PREPARED_FONTS: Mutex<Vec<PreparedFont>> = Mutex::new(Vec::new());

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
    // Create default Material theme programmatically to avoid JSON parsing issues
    let mut light_scheme = MaterialScheme {
        primary: "#8F4C38".to_string(),
        surface_tint: "#8F4C38".to_string(),
        on_primary: "#FFFFFF".to_string(),
        primary_container: "#FFDBD1".to_string(),
        on_primary_container: "#723523".to_string(),
        secondary: "#77574E".to_string(),
        on_secondary: "#FFFFFF".to_string(),
        secondary_container: "#FFDBD1".to_string(),
        on_secondary_container: "#5D4037".to_string(),
        tertiary: "#6C5D2F".to_string(),
        on_tertiary: "#FFFFFF".to_string(),
        tertiary_container: "#F5E1A7".to_string(),
        on_tertiary_container: "#534619".to_string(),
        error: "#BA1A1A".to_string(),
        on_error: "#FFFFFF".to_string(),
        error_container: "#FFDAD6".to_string(),
        on_error_container: "#93000A".to_string(),
        background: "#FFF8F6".to_string(),
        on_background: "#231917".to_string(),
        surface: "#FFF8F6".to_string(),
        on_surface: "#231917".to_string(),
        surface_variant: "#F5DED8".to_string(),
        on_surface_variant: "#53433F".to_string(),
        outline: "#85736E".to_string(),
        outline_variant: "#D8C2BC".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#392E2B".to_string(),
        inverse_on_surface: "#FFEDE8".to_string(),
        inverse_primary: "#FFB5A0".to_string(),
        primary_fixed: "#FFDBD1".to_string(),
        on_primary_fixed: "#3A0B01".to_string(),
        primary_fixed_dim: "#FFB5A0".to_string(),
        on_primary_fixed_variant: "#723523".to_string(),
        secondary_fixed: "#FFDBD1".to_string(),
        on_secondary_fixed: "#2C150F".to_string(),
        secondary_fixed_dim: "#E7BDB2".to_string(),
        on_secondary_fixed_variant: "#5D4037".to_string(),
        tertiary_fixed: "#F5E1A7".to_string(),
        on_tertiary_fixed: "#231B00".to_string(),
        tertiary_fixed_dim: "#D8C58D".to_string(),
        on_tertiary_fixed_variant: "#534619".to_string(),
        surface_dim: "#E8D6D2".to_string(),
        surface_bright: "#FFF8F6".to_string(),
        surface_container_lowest: "#FFFFFF".to_string(),
        surface_container_low: "#FFF1ED".to_string(),
        surface_container: "#FCEAE5".to_string(),
        surface_container_high: "#F7E4E0".to_string(),
        surface_container_highest: "#F1DFDA".to_string(),
    };

    let mut dark_scheme = MaterialScheme {
        primary: "#FFB5A0".to_string(),
        surface_tint: "#FFB5A0".to_string(),
        on_primary: "#561F0F".to_string(),
        primary_container: "#723523".to_string(),
        on_primary_container: "#FFDBD1".to_string(),
        secondary: "#E7BDB2".to_string(),
        on_secondary: "#442A22".to_string(),
        secondary_container: "#5D4037".to_string(),
        on_secondary_container: "#FFDBD1".to_string(),
        tertiary: "#D8C58D".to_string(),
        on_tertiary: "#3B2F05".to_string(),
        tertiary_container: "#534619".to_string(),
        on_tertiary_container: "#F5E1A7".to_string(),
        error: "#FFB4AB".to_string(),
        on_error: "#690005".to_string(),
        error_container: "#93000A".to_string(),
        on_error_container: "#FFDAD6".to_string(),
        background: "#1A110F".to_string(),
        on_background: "#F1DFDA".to_string(),
        surface: "#1A110F".to_string(),
        on_surface: "#F1DFDA".to_string(),
        surface_variant: "#53433F".to_string(),
        on_surface_variant: "#D8C2BC".to_string(),
        outline: "#A08C87".to_string(),
        outline_variant: "#53433F".to_string(),
        shadow: "#000000".to_string(),
        scrim: "#000000".to_string(),
        inverse_surface: "#F1DFDA".to_string(),
        inverse_on_surface: "#392E2B".to_string(),
        inverse_primary: "#8F4C38".to_string(),
        primary_fixed: "#FFDBD1".to_string(),
        on_primary_fixed: "#3A0B01".to_string(),
        primary_fixed_dim: "#FFB5A0".to_string(),
        on_primary_fixed_variant: "#723523".to_string(),
        secondary_fixed: "#FFDBD1".to_string(),
        on_secondary_fixed: "#2C150F".to_string(),
        secondary_fixed_dim: "#E7BDB2".to_string(),
        on_secondary_fixed_variant: "#5D4037".to_string(),
        tertiary_fixed: "#F5E1A7".to_string(),
        on_tertiary_fixed: "#231B00".to_string(),
        tertiary_fixed_dim: "#D8C58D".to_string(),
        on_tertiary_fixed_variant: "#534619".to_string(),
        surface_dim: "#1A110F".to_string(),
        surface_bright: "#423734".to_string(),
        surface_container_lowest: "#140C0A".to_string(),
        surface_container_low: "#231917".to_string(),
        surface_container: "#271D1B".to_string(),
        surface_container_high: "#322825".to_string(),
        surface_container_highest: "#3D322F".to_string(),
    };

    let mut schemes = HashMap::new();
    schemes.insert("light".to_string(), light_scheme);
    schemes.insert("dark".to_string(), dark_scheme);

    let mut core_colors = HashMap::new();
    core_colors.insert("primary".to_string(), "#B33B15".to_string());

    MaterialThemeFile {
        description: "Default Material Theme".to_string(),
        seed: "#B33B15".to_string(),
        core_colors,
        extended_colors: Vec::new(),
        schemes,
        palettes: HashMap::new(),
    }
}

impl MaterialThemeContext {
    pub fn setup_fonts(font_name: Option<&str>) {
        let font_name = font_name.unwrap_or("Google Sans Code");
        
        // Get font data from the generated constants
        let font_data = Self::get_font_data(font_name);
        
        if let Some(data) = font_data {
            if !data.is_empty() {
                let font_family_name = font_name.replace(" ", "");
                
                let prepared_font = PreparedFont {
                    name: font_family_name.clone(),
                    data: Arc::new(FontData::from_static(data)),
                    families: vec![FontFamily::Proportional, FontFamily::Monospace],
                };
                
                if let Ok(mut fonts) = PREPARED_FONTS.lock() {
                    // Remove any existing font with the same name
                    fonts.retain(|f| f.name != font_family_name);
                    fonts.push(prepared_font);
                }
            }
        }
    }
    
    fn get_font_data(font_name: &str) -> Option<&'static [u8]> {
        match font_name {
            "Google Sans Code" => Some(GOOGLE_SANS_CODE_REGULAR),
            _ => {
                // Try to find the font constant dynamically
                Self::get_font_data_dynamic(font_name)
            }
        }
    }
    
    fn get_font_data_dynamic(font_name: &str) -> Option<&'static [u8]> {
        // This function will be extended to handle dynamically generated font constants
        // For now, return None for unknown fonts
        eprintln!("Warning: Font '{}' not found in generated constants", font_name);
        None
    }
    
    pub fn setup_local_fonts(font_path: Option<&str>) {
        // Determine which font to use
        let font_data = if let Some(path) = font_path {
            // Try to load custom font from path
            match std::fs::read(path) {
                Ok(data) => Some(data),
                Err(_) => {
                    // Fall back to default font if custom font fails to load
                    if !MATERIALSYMBOLSOUTLINED_FILL_GRAD_OPSZ_WGHT_.is_empty() {
                        Some(MATERIALSYMBOLSOUTLINED_FILL_GRAD_OPSZ_WGHT_.to_vec())
                    } else {
                        None
                    }
                }
            }
        } else {
            // Use default Material Symbols Outlined font
            if !MATERIALSYMBOLSOUTLINED_FILL_GRAD_OPSZ_WGHT_.is_empty() {
                Some(MATERIALSYMBOLSOUTLINED_FILL_GRAD_OPSZ_WGHT_.to_vec())
            } else {
                None
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
            // Fallback colors when no theme is loaded (using material-theme1.json light values)
            match name {
                "primary" => Color32::from_rgb(143, 76, 56), // #8F4C38
                "onPrimary" => Color32::WHITE, // #FFFFFF
                "surface" => Color32::from_rgb(255, 248, 246), // #FFF8F6
                "onSurface" => Color32::from_rgb(35, 25, 23), // #231917
                "surfaceContainer" => Color32::from_rgb(252, 234, 229), // #FCEAE5
                "surfaceContainerHigh" => Color32::from_rgb(247, 228, 224), // #F7E4E0
                "surfaceContainerHighest" => Color32::from_rgb(241, 223, 218), // #F1DFDA
                "surfaceContainerLow" => Color32::from_rgb(255, 241, 237), // #FFF1ED
                "surfaceContainerLowest" => Color32::from_rgb(255, 255, 255), // #FFFFFF
                "outline" => Color32::from_rgb(133, 115, 110), // #85736E
                "outlineVariant" => Color32::from_rgb(216, 194, 188), // #D8C2BC
                "surfaceVariant" => Color32::from_rgb(245, 222, 216), // #F5DED8
                "onSurfaceVariant" => Color32::from_rgb(83, 67, 63), // #53433F
                "secondary" => Color32::from_rgb(119, 87, 78), // #77574E
                "onSecondary" => Color32::WHITE, // #FFFFFF
                "tertiary" => Color32::from_rgb(108, 93, 47), // #6C5D2F
                "onTertiary" => Color32::WHITE, // #FFFFFF
                "error" => Color32::from_rgb(186, 26, 26), // #BA1A1A
                "onError" => Color32::WHITE, // #FFFFFF
                "background" => Color32::from_rgb(255, 248, 246), // #FFF8F6
                "onBackground" => Color32::from_rgb(35, 25, 23), // #231917
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

/// Load all prepared fonts to the egui context
/// Call this after all setup_*_fonts functions to actually load the fonts
pub fn load_fonts(ctx: &egui::Context) {
    MaterialThemeContext::load_fonts(ctx);
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