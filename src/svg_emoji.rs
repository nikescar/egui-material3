//! SVG Emoji and Icon Collections
//!
//! This module provides three comprehensive SVG collections embedded at compile time:
//!
//! - **Solar Icons** (~1200 icons): UI/UX icon set with variants
//! - **Noto Emoji** (~3600 emoji): Google's emoji collection with skin tone and gender variants
//! - **Twemoji** (~3700 emoji): Twitter's emoji collection
//!
//! ## Usage
//!
//! Icons and emojis are accessible through HashMaps for O(1) lookup:
//!
//! ```rust
//! use egui_material3::svg_emoji::{SOLAR_ICONS, NOTO_EMOJIS, TWEMOJI};
//!
//! // Get a Solar icon
//! if let Some(svg) = SOLAR_ICONS.get("home") {
//!     // Use the SVG data
//! }
//!
//! // Get a Noto emoji (filename without .svg suffix)
//! if let Some(svg) = NOTO_EMOJIS.get("emoji_u1f600") {
//!     // Grinning face emoji
//! }
//!
//! // Get a Twemoji (Unicode codepoint)
//! if let Some(svg) = TWEMOJI.get("1f600") {
//!     // Grinning face emoji
//! }
//! ```

// Include generated constants and HashMaps from build script
include!(concat!(env!("OUT_DIR"), "/svg_emoji_generated.rs"));

// Helper structures for organized access to icon collections
#[derive(Debug, Clone)]
pub struct SvgIcon {
    pub name: &'static str,
    pub svg_data: &'static str,
}

#[derive(Debug, Clone)]
pub struct SvgCollection {
    pub icons: Vec<SvgIcon>,
}

impl SvgCollection {
    /// Get all Solar icons
    pub fn solar_icons() -> Self {
        let icons = SOLAR_ICONS
            .iter()
            .map(|(name, svg_data)| SvgIcon {
                name,
                svg_data,
            })
            .collect();
        Self { icons }
    }

    /// Get all Noto emoji
    pub fn noto_emoji() -> Self {
        let icons = NOTO_EMOJIS
            .iter()
            .map(|(name, svg_data)| SvgIcon {
                name,
                svg_data,
            })
            .collect();
        Self { icons }
    }

    /// Get all Twemoji
    pub fn twemoji() -> Self {
        let icons = TWEMOJI
            .iter()
            .map(|(name, svg_data)| SvgIcon {
                name,
                svg_data,
            })
            .collect();
        Self { icons }
    }
}
