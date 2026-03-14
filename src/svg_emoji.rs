//! SVG Emoji and Icon Collections
//!
//! This module provides three comprehensive SVG collections available as optional features:
//!
//! - **Solar Icons** (~1200 icons): UI/UX icon set with variants (feature: `svg_solar`)
//! - **Noto Emoji** (~3600 emoji): Google's emoji collection with skin tone and gender variants (feature: `svg_noto`)
//! - **Twemoji** (~3700 emoji): Twitter's emoji collection (feature: `svg_twemoji`)
//!
//! ## Features
//!
//! Add the features you need to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! egui-material3 = { version = "0.0.8", features = ["svg_solar"] }
//! # Or enable all: features = ["svg_emoji"]
//! ```
//!
//! ## Usage
//!
//! Icons and emojis are accessible through HashMaps for O(1) lookup:
//!
//! ```rust
//! use egui_material3::svg_emoji::{SOLAR_ICONS, NOTO_EMOJIS, TWEMOJI};
//!
//! // Get a Solar icon (requires svg_solar feature)
//! if let Some(svg) = SOLAR_ICONS.get("home") {
//!     // Use the SVG data
//! }
//!
//! // Get a Noto emoji (requires svg_noto feature)
//! if let Some(svg) = NOTO_EMOJIS.get("emoji_u1f600") {
//!     // Grinning face emoji
//! }
//!
//! // Get a Twemoji (requires svg_twemoji feature)
//! if let Some(svg) = TWEMOJI.get("1f600") {
//!     // Grinning face emoji
//! }
//! ```
//!
//! ## Notes
//!
//! - SVG files are embedded at compile time when features are enabled
//! - Files are downloaded automatically during build if not in local checkout
//! - Each collection is ~5-40 MiB when embedded, so only enable what you need

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
    /// Get all Solar icons (requires `svg_solar` feature)
    #[cfg(feature = "svg_solar")]
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

    /// Get all Noto emoji (requires `svg_noto` feature)
    #[cfg(feature = "svg_noto")]
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

    /// Get all Twemoji (requires `svg_twemoji` feature)
    #[cfg(feature = "svg_twemoji")]
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
