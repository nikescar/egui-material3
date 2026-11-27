//! Image and SVG Utilities
//!
//! Helper functions for loading and caching images and SVG graphics.
//! Icons rendered from these utilities should use M3 color roles.

use egui::{self, ColorImage, Context, TextureHandle};
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Tree},
};
use std::collections::HashMap;
use std::sync::Mutex;

/// Largest width or height an SVG is rasterised at, or a PNG is decoded at.
/// An SVG declaring `width="100000"` would otherwise allocate tens of
/// gigabytes for its pixmap.
const MAX_DIMENSION: u32 = 4096;

lazy_static::lazy_static! {
    static ref TEXTURE_CACHE: Mutex<HashMap<String, TextureHandle>> = Mutex::new(HashMap::new());
}

pub fn svg_to_png_bytes(svg_data: &str) -> Result<Vec<u8>, String> {
    // Parse the SVG data into a usvg::Tree
    let tree = Tree::from_str(svg_data, &Options::default())
        .map_err(|e| format!("Failed to parse SVG: {}", e))?;

    // Get the dimensions from the SVG or define a default
    let size = tree.size().to_int_size();
    let width = size.width();
    let height = size.height();
    if width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(format!(
            "SVG is {width} x {height}; the limit is {MAX_DIMENSION} x {MAX_DIMENSION}"
        ));
    }

    // Create a new Pixmap to draw on
    let mut pixmap =
        Pixmap::new(width, height).ok_or_else(|| "Failed to create pixmap".to_string())?;

    // Render the SVG onto the pixmap
    resvg::render(
        &tree,
        resvg::usvg::Transform::default(),
        &mut pixmap.as_mut(),
    );

    // Encode the pixmap to PNG bytes
    let png_bytes = pixmap
        .encode_png()
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    Ok(png_bytes)
}

pub fn create_texture_from_svg(
    ctx: &Context,
    svg_data: &str,
    name: &str,
) -> Result<TextureHandle, String> {
    // Check if texture is already cached
    {
        let cache = TEXTURE_CACHE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(texture) = cache.get(name) {
            return Ok(texture.clone());
        }
    }

    // Convert SVG to PNG bytes
    let png_bytes = svg_to_png_bytes(svg_data)?;

    // Decode PNG to image data
    let image = decode_png_limited(&png_bytes)?;

    let size = [image.width() as usize, image.height() as usize];
    let rgba_image = image.to_rgba8();
    let pixels = rgba_image.as_flat_samples();

    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    let texture = ctx.load_texture(name, color_image, egui::TextureOptions::default());

    // Cache the texture
    {
        let mut cache = TEXTURE_CACHE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        cache.insert(name.to_string(), texture.clone());
    }

    Ok(texture)
}

pub fn create_texture_from_png_bytes(
    ctx: &Context,
    png_bytes: &[u8],
    name: &str,
) -> Result<TextureHandle, String> {
    // Check if texture is already cached
    {
        let cache = TEXTURE_CACHE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(texture) = cache.get(name) {
            return Ok(texture.clone());
        }
    }

    // Decode PNG to image data
    let image = decode_png_limited(png_bytes)?;

    let size = [image.width() as usize, image.height() as usize];
    let rgba_image = image.to_rgba8();
    let pixels = rgba_image.as_flat_samples();

    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    let texture = ctx.load_texture(name, color_image, egui::TextureOptions::default());

    // Cache the texture
    {
        let mut cache = TEXTURE_CACHE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        cache.insert(name.to_string(), texture.clone());
    }

    Ok(texture)
}

/// Decodes a PNG no larger than [`MAX_DIMENSION`] on either side.
fn decode_png_limited(png_bytes: &[u8]) -> Result<image::DynamicImage, String> {
    let mut reader =
        image::ImageReader::with_format(std::io::Cursor::new(png_bytes), image::ImageFormat::Png);
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_DIMENSION);
    limits.max_image_height = Some(MAX_DIMENSION);
    reader.limits(limits);
    reader
        .decode()
        .map_err(|e| format!("Failed to decode PNG: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oversized_svg_is_refused() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100000" height="100000"/>"#;
        assert!(
            svg_to_png_bytes(svg).is_err(),
            "an SVG over the size limit should be refused, not rasterised"
        );
        let small = r#"<svg xmlns="http://www.w3.org/2000/svg" width="8" height="8"/>"#;
        assert!(
            svg_to_png_bytes(small).is_ok(),
            "a small SVG should rasterise"
        );
    }
}

// Material Icons support - using Unicode characters
pub mod material_icons {
    pub const LOCAL_LAUNDRY_SERVICE: &str = "\u{e950}";
    pub const ACCOUNT_CIRCLE: &str = "\u{e853}";
    pub const LINK: &str = "\u{e157}";
    pub const GOOGLE: &str = "\u{ea88}";
    pub const EVENT: &str = "\u{e878}";
}

// Include static assets
pub const GOOGLE_LOGO_SVG: &str = include_str!("../resources/google_logo.svg");
pub const AVATAR_SVG: &str = include_str!("../resources/avatar.svg");
