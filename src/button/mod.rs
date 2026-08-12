//! Material Design 3 Button Components
//!
//! This module implements button controls following Material Design 3 color system.
//!
//! # M3 Color Role Usage
//!
//! ## Filled Button (High Emphasis)
//! - **primary**: Button background
//! - **onPrimary**: Text and icon color on primary background
//! - **State layers**: onPrimary @ 8% (hover), 12% (press)
//! - **Disabled**: surface background, onSurface @ 12% outline, onSurface @ 38% content
//!
//! ## Outlined Button (Medium Emphasis)
//! - **Transparent background**: Shows parent surface
//! - **outline**: Border stroke color
//! - **onSurface**: Text and icon color
//! - **State layers**: onSurface @ 8% (hover), 12% (press)
//! - **Disabled**: onSurface @ 12% outline, onSurface @ 38% content
//!
//! ## Text Button (Low Emphasis)
//! - **Transparent background**: No border, shows parent surface
//! - **onSurface**: Text and icon color
//! - **State layers**: onSurface @ 8% (hover), 12% (press)
//! - **Disabled**: onSurface @ 38% content
//!
//! ## Elevated Button (Medium Emphasis with Shadow)
//! - **surface**: Button background (elevated surface)
//! - **onSurface**: Text and icon color
//! - **Shadow**: 1dp elevation, increases to 3dp on hover
//! - **State layers**: onSurface @ 8% (hover), 12% (press)
//! - **Disabled**: surface background, onSurface @ 38% content
//!
//! ## Filled Tonal Button (Medium Emphasis, Toned Down)
//! - **secondaryContainer**: Tinted container background
//! - **onSecondaryContainer**: Text and icon color on tinted background
//! - **State layers**: onSecondaryContainer @ 8% (hover), 12% (press)
//! - **Disabled**: surface background, onSurface @ 12% outline, onSurface @ 38% content

mod types;
mod builder;
mod rendering;

pub use types::*;
