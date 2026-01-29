use eframe::egui::{self, ColorImage, Context, TextureHandle};
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Tree},
};
use std::collections::HashMap;
use std::sync::Mutex;

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
        let cache = TEXTURE_CACHE.lock().unwrap();
        if let Some(texture) = cache.get(name) {
            return Ok(texture.clone());
        }
    }

    // Convert SVG to PNG bytes
    let png_bytes = svg_to_png_bytes(svg_data)?;

    // Decode PNG to image data
    let image =
        image::load_from_memory(&png_bytes).map_err(|e| format!("Failed to decode PNG: {}", e))?;

    let size = [image.width() as usize, image.height() as usize];
    let rgba_image = image.to_rgba8();
    let pixels = rgba_image.as_flat_samples();

    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    let texture = ctx.load_texture(name, color_image, egui::TextureOptions::default());

    // Cache the texture
    {
        let mut cache = TEXTURE_CACHE.lock().unwrap();
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
        let cache = TEXTURE_CACHE.lock().unwrap();
        if let Some(texture) = cache.get(name) {
            return Ok(texture.clone());
        }
    }

    // Decode PNG to image data
    let image =
        image::load_from_memory(png_bytes).map_err(|e| format!("Failed to decode PNG: {}", e))?;

    let size = [image.width() as usize, image.height() as usize];
    let rgba_image = image.to_rgba8();
    let pixels = rgba_image.as_flat_samples();

    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    let texture = ctx.load_texture(name, color_image, egui::TextureOptions::default());

    // Cache the texture
    {
        let mut cache = TEXTURE_CACHE.lock().unwrap();
        cache.insert(name.to_string(), texture.clone());
    }

    Ok(texture)
}

// Material Icons support - using Unicode characters
pub mod material_icons {
    pub const LOCAL_LAUNDRY_SERVICE: &str = "\u{e950}";
    pub const ACCOUNT_CIRCLE: &str = "\u{e853}";
    pub const LINK: &str = "\u{e157}";
    pub const GOOGLE: &str = "\u{ea88}";
}

// Include static assets
pub const GOOGLE_LOGO_SVG: &str = include_str!("../resources/google_logo.svg");
pub const AVATAR_SVG: &str = include_str!("../resources/avatar.svg");
