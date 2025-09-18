//! # Material Design Image Lists
//!
//! This module provides Material Design 3 image list components with comprehensive
//! image source support and intelligent caching capabilities.
//!
//! ## Features
//!
//! - **Multiple image sources**: Local files, online URLs, and embedded byte arrays
//! - **Smart caching**: Downloaded images are cached with proper file extensions
//! - **Format detection**: Automatic detection of PNG, JPEG, GIF, and WebP formats
//! - **Performance optimized**: Efficient loading and UI repainting
//! - **Error handling**: Graceful fallback with visual indicators
//!
//! ## Usage
//!
//! ### Local Images
//! ```rust,no_run
//! use egui_material3::image_list;
//!
//! ui.add(image_list()
//!     .columns(3)
//!     .item_spacing(8.0)
//!     .items_from_paths(glob::glob("resources/*.png")?));
//! ```
//!
//! ### Online Images (OnDemand Feature)
//!
//! Enable the `ondemand` feature in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! egui-material3 = { version = "0.0.5", features = ["ondemand"] }
//! ```
//!
//! ```rust,no_run
//! use egui_material3::image_list;
//!
//! ui.add(image_list()
//!     .columns(4)
//!     .item_spacing(8.0)
//!     .items_from_urls(vec![
//!         "https://example.com/image1.jpg".to_string(),
//!         "https://example.com/image2.png".to_string(),
//!     ]));
//! ```
//!
//! ### Embedded Images
//! ```rust,no_run
//! use egui_material3::image_list;
//!
//! ui.add(image_list()
//!     .columns(2)
//!     .item_spacing(8.0)
//!     .items_from_bytes(vec![
//!         include_bytes!("image1.png").to_vec(),
//!         include_bytes!("image2.png").to_vec(),
//!     ]));
//! ```
//!
//! ## OnDemand Feature Details
//!
//! When the `ondemand` feature is enabled, the image list provides:
//!
//! - **Automatic downloading**: Images are downloaded from URLs on first access
//! - **Smart caching**: Downloaded images are saved to `/tmp/egui_material3_img/` with proper extensions
//! - **Format detection**: File extensions are determined from content (PNG, JPEG, GIF, WebP)
//! - **Efficient reuse**: Cached images are reused without re-downloading
//! - **Performance optimization**: UI only repaints when new images are available
//! - **Error handling**: Failed downloads show visual indicators instead of crashing
//!
//! ### Cache Management
//!
//! - Cache directory: `/tmp/egui_material3_img/`
//! - File naming: `img_{hash}.{extension}` (e.g., `img_abc123.png`)
//! - Automatic cleanup: Cache persists between runs for efficiency
//! - Manual cleanup: Remove `/tmp/egui_material3_img/` to clear cache

use crate::theme::get_global_color;
use egui::{
    ecolor::Color32,
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget
};
use std::env;
use image::GenericImageView;

/// Material Design image list variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageListVariant {
    Standard,
    Masonry,
    Woven,
}

/// Material Design image list component.
///
/// Image lists display a collection of images in an organized grid.
/// They're commonly used to display a collection of photos or other images.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let image_list = MaterialImageList::standard()
///     .columns(3)
///     .item("Image 1", "320x240.png")
///     .item("Image 2", "320x240.png")
///     .item("Image 3", "320x240.png");
///
/// ui.add(image_list);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialImageList<'a> {
    variant: ImageListVariant,
    items: Vec<ImageListItem<'a>>,
    columns: usize,
    item_spacing: f32,
    text_protected: bool,
    corner_radius: CornerRadius,
    id_salt: Option<String>,
    tmppath: String,
}

pub struct ImageListItem<'a> {
    pub label: String,
    pub image_source: Option<String>,
    pub supporting_text: Option<String>,
    pub on_click: Option<Box<dyn Fn() + Send + Sync>>,
    pub loaded_image: Option<egui::ColorImage>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> ImageListItem<'a> {
    pub fn new(label: impl Into<String>, image_source: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            image_source: Some(image_source.into()),
            supporting_text: None,
            on_click: None,
            loaded_image: None,
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }
    
    pub fn on_click<F>(mut self, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

/// Load image from a local file path
fn load_image_from_file(file_path: &str) -> Option<egui::ColorImage> {
    if std::path::Path::new(file_path).exists() {
        match image::open(file_path) {
            Ok(image) => {
                let original_size = image.dimensions();
                
                // Resize large images to max 512x512 to avoid memory issues
                let resized_image = if original_size.0 > 512 || original_size.1 > 512 {
                    image.resize(512, 512, image::imageops::FilterType::Lanczos3)
                } else {
                    image
                };
                
                let size = resized_image.dimensions();
                let image_buffer = resized_image.to_rgba8();
                let pixels = image_buffer.into_raw();
                Some(egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], &pixels))
            }
            Err(_) => None
        }
    } else {
        None
    }
}

/// Load image from URL (requires ondemand feature)
#[cfg(feature = "ondemand")]
fn load_image_from_url(url: &str, tmppath: &str) -> Option<egui::ColorImage> {
    use std::io::Read;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
    let url_hash = format!("{:x}", hasher.finish());
    let filename = format!("img_{}", url_hash);
    let filepath = std::path::Path::new(tmppath).join(&filename);

    // Check if file already exists with any extension
    let possible_files = [
        filepath.with_extension("png"),
        filepath.with_extension("jpg"),
        filepath.with_extension("gif"),
        filepath.with_extension("webp"),
        filepath.clone()
    ];

    let existing_file = possible_files.iter().find(|f| f.exists());

    if existing_file.is_none() {
        // Try to download the image with timeout and user agent
        let agent = ureq::AgentBuilder::new()
            .timeout_read(std::time::Duration::from_secs(10))
            .timeout_write(std::time::Duration::from_secs(10))
            .user_agent("egui-material3/1.0")
            .build();

        match agent.get(url).call() {
            Ok(response) => {
                let status = response.status();
                if status == 200 {
                    let mut bytes = Vec::new();
                    if let Ok(_) = response.into_reader().read_to_end(&mut bytes) {
                        if !bytes.is_empty() {
                            // Detect image format from content and add appropriate extension
                            let extension = if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
                                "png"
                            } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
                                "jpg"
                            } else if bytes.starts_with(&[0x47, 0x49, 0x46]) {
                                "gif"
                            } else if bytes.starts_with(&[0x52, 0x49, 0x46, 0x46]) && bytes.len() > 12 && &bytes[8..12] == b"WEBP" {
                                "webp"
                            } else {
                                "png"
                            };

                            let filepath_with_ext = filepath.with_extension(extension);
                            let _ = std::fs::write(&filepath_with_ext, &bytes);
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    // Try to load the image from cache
    if let Some(existing_filepath) = possible_files.iter().find(|f| f.exists()) {
        match image::open(existing_filepath) {
            Ok(image) => {
                let original_size = image.dimensions();

                // Resize large images to max 512x512 to avoid memory issues
                let resized_image = if original_size.0 > 512 || original_size.1 > 512 {
                    image.resize(512, 512, image::imageops::FilterType::Lanczos3)
                } else {
                    image
                };

                let size = resized_image.dimensions();
                let image_buffer = resized_image.to_rgba8();
                let pixels = image_buffer.into_raw();
                Some(egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], &pixels))
            }
            Err(_) => None
        }
    } else {
        None
    }
}

/// Load image from data URL (base64 encoded)
fn load_image_from_data_url(data_url: &str) -> Option<egui::ColorImage> {
    if let Some(comma_pos) = data_url.find(',') {
        let data_part = &data_url[comma_pos + 1..];
        if let Ok(bytes) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data_part) {
            if let Ok(image) = image::load_from_memory(&bytes) {
                let size = image.dimensions();
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.into_raw();
                return Some(egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], &pixels));
            }
        }
    }
    None
}

/// Load image from bytes (hex encoded)
fn load_image_from_bytes(bytes_str: &str) -> Option<egui::ColorImage> {
    if let Ok(bytes) = hex::decode(bytes_str) {
        if let Ok(image) = image::load_from_memory(&bytes) {
            let size = image.dimensions();
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.into_raw();
            return Some(egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], &pixels));
        }
    }
    None
}

impl<'a> MaterialImageList<'a> {
    /// Create a new standard image list.
    pub fn standard() -> Self {
        Self::new(ImageListVariant::Standard)
    }

    /// Create a new masonry image list.
    pub fn masonry() -> Self {
        Self::new(ImageListVariant::Masonry)
    }

    /// Create a new woven image list.
    pub fn woven() -> Self {
        Self::new(ImageListVariant::Woven)
    }

    fn new(variant: ImageListVariant) -> Self {
        // create img folder in tmp dir using let dir = env::temp_dir(), save the path to tmppath on MaterialImageList
        let tmppath = env::temp_dir().join("egui_material3_img");
        let _ = std::fs::create_dir_all(&tmppath);

        Self {
            variant,
            items: Vec::new(),
            columns: 3,
            item_spacing: 8.0,
            text_protected: false,
            corner_radius: CornerRadius::from(4.0),
            id_salt: None,
            tmppath: tmppath.to_string_lossy().to_string(),
        }
    }

    /// Set number of columns.
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    /// Add an image item.
    pub fn item(mut self, label: impl Into<String>, image_source: impl Into<String>) -> Self {
        self.items.push(ImageListItem::new(label, image_source));
        self
    }

    /// Add an image item with callback.
    pub fn item_with_callback<F>(
        mut self, 
        label: impl Into<String>, 
        image_source: impl Into<String>,
        callback: F
    ) -> Self 
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.items.push(
            ImageListItem::new(label, image_source)
                .on_click(callback)
        );
        self
    }

    /// Set item spacing.
    pub fn item_spacing(mut self, spacing: f32) -> Self {
        self.item_spacing = spacing;
        self
    }

    /// Enable text protection overlay.
    pub fn text_protected(mut self, protected: bool) -> Self {
        self.text_protected = protected;
        self
    }

    /// Set corner radius.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set unique ID salt to prevent ID clashes.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Add items from a collection of file paths.
    pub fn items_from_paths<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<std::path::Path>,
    {
        for (i, path) in paths.into_iter().enumerate() {
            let path_str = path.as_ref().to_string_lossy().to_string();
            let filename = if let Some(file_name) = path.as_ref().file_name() {
                file_name.to_string_lossy().to_string()
            } else {
                format!("Image {}", i + 1)
            };
            self.items.push(ImageListItem::new(filename, path_str));
        }
        self
    }

    /// Add items from a collection of URLs.
    pub fn items_from_urls<I, S>(mut self, urls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for (i, url) in urls.into_iter().enumerate() {
            let url_str = url.into();
            let label = format!("Online {}", i + 1);
            self.items.push(ImageListItem::new(label, url_str));
        }
        self
    }

    /// Add items from a collection of byte arrays (for embedded images).
    pub fn items_from_bytes<I>(mut self, bytes_collection: I) -> Self
    where
        I: IntoIterator<Item = Vec<u8>>,
    {
        for (i, bytes) in bytes_collection.into_iter().enumerate() {
            let label = format!("Embedded {}", i + 1);
            // Convert bytes to hex string for the bytes: protocol
            let hex_string = format!("bytes:{}", hex::encode(bytes));
            self.items.push(ImageListItem::new(label, hex_string));
        }
        self
    }

    fn get_image_list_style(&self) -> Color32 {
        get_global_color("surface")
    }
}

impl<'a> Default for MaterialImageList<'a> {
    fn default() -> Self {
        Self::standard()
    }
}

impl Widget for MaterialImageList<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let background_color = self.get_image_list_style();

        let MaterialImageList {
            variant,
            mut items,
            columns,
            item_spacing,
            text_protected,
            corner_radius,
            id_salt,
            #[cfg_attr(not(feature = "ondemand"), allow(unused_variables))]
            tmppath,
        } = self;

        if items.is_empty() {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        // Calculate grid dimensions
        let available_width = ui.available_width();
        let item_width = (available_width - (columns - 1) as f32 * item_spacing) / columns as f32;
        let item_height = match variant {
            ImageListVariant::Standard => item_width, // Square items
            ImageListVariant::Masonry => item_width * 1.2, // Slightly taller
            ImageListVariant::Woven => item_width * 0.8, // Slightly shorter
        };
        
        let rows = (items.len() + columns - 1) / columns;
        let total_height = rows as f32 * (item_height + item_spacing) - item_spacing;
        let total_width = available_width;

        let response = ui.allocate_response(
            Vec2::new(total_width, total_height), 
            Sense::hover()
        );
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw background
            ui.painter().rect_filled(rect, corner_radius, background_color);

            // Draw items in grid
            for (index, item) in items.iter_mut().enumerate() {
                let row = index / columns;
                let col = index % columns;
                
                let item_x = rect.min.x + col as f32 * (item_width + item_spacing);
                let item_y = rect.min.y + row as f32 * (item_height + item_spacing);
                
                let item_rect = Rect::from_min_size(
                    egui::pos2(item_x, item_y),
                    Vec2::new(item_width, item_height)
                );

                // Handle item interaction with unique ID
                let item_id = if let Some(ref salt) = id_salt {
                    egui::Id::new((salt, "image_item", index))
                } else {
                    egui::Id::new(("image_item", index, &item.label))
                };
                
                let item_response = ui.interact(item_rect, item_id, Sense::click());
                if item_response.hovered() {
                    let hover_color = get_global_color("primary").linear_multiply(0.08);
                    ui.painter().rect_filled(item_rect, corner_radius, hover_color);
                }

                if item_response.clicked() {
                    if let Some(callback) = &item.on_click {
                        callback();
                    }
                }

                // Draw placeholder image (rectangle with border)
                let image_rect = item_rect.shrink(2.0);
                let image_bg = get_global_color("surfaceVariant");
                let image_border = Stroke::new(1.0, get_global_color("outline"));
                
                ui.painter().rect_filled(image_rect, corner_radius, image_bg);
                ui.painter().rect_stroke(image_rect, corner_radius, image_border, egui::epaint::StrokeKind::Outside);

                // Load and cache image if not already loaded
                if item.loaded_image.is_none() {
                    if let Some(ref image_source) = item.image_source {
                        let loaded_image = if image_source.starts_with("http://") || image_source.starts_with("https://") {
                            #[cfg(feature = "ondemand")]
                            {
                                load_image_from_url(image_source, &tmppath)
                            }
                            #[cfg(not(feature = "ondemand"))]
                            {
                                None
                            }
                        } else if image_source.starts_with("data:") {
                            load_image_from_data_url(image_source)
                        } else if image_source.starts_with("bytes:") {
                            let bytes_str = &image_source[6..]; // Remove "bytes:" prefix
                            load_image_from_bytes(bytes_str)
                        } else {
                            load_image_from_file(image_source)
                        };
                        
                        // Cache the loaded image (even if None for failed loads)
                        item.loaded_image = loaded_image;
                    }
                }

                // Render the image if available
                let mut failed = false;
                if let Some(ref color_image) = item.loaded_image {
                    let texture_name = format!("image_texture_{}_{}", item_id.value(), item.label);
                    let texture_id = ui.ctx().load_texture(
                        texture_name,
                        color_image.clone(),
                        Default::default()
                    );
                    ui.painter().image(
                        texture_id.id(),
                        image_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                } else {
                    failed = true;
                }

                if failed {
                    // Debug: Print when showing X marks
                    println!("SHOWING X MARK for item: {}", item.label);
                    // Draw a simple "X" to indicate failed image load
                    let line_color = get_global_color("error");
                    ui.painter().line_segment(
                        [image_rect.min, image_rect.max],
                        Stroke::new(2.0, line_color),
                    );
                    ui.painter().line_segment(
                        [egui::pos2(image_rect.min.x, image_rect.max.y), egui::pos2(image_rect.max.x, image_rect.min.y)],
                        Stroke::new(2.0, line_color),
                    );
                }

                // Draw text overlay or below image
                let text_color = if text_protected {
                    Color32::WHITE
                } else {
                    get_global_color("onSurface")
                };

                if text_protected {
                    // Draw dark overlay for text protection
                    let overlay_rect = Rect::from_min_size(
                        egui::pos2(image_rect.min.x, image_rect.max.y - 40.0),
                        Vec2::new(image_rect.width(), 40.0)
                    );
                    let overlay_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);
                    ui.painter().rect_filled(overlay_rect, CornerRadius::ZERO, overlay_color);
                    
                    // Draw text on overlay
                    let text_pos = egui::pos2(image_rect.min.x + 8.0, image_rect.max.y - 30.0);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_TOP,
                        &item.label,
                        egui::FontId::proportional(12.0),
                        text_color
                    );
                    
                    if let Some(supporting_text) = &item.supporting_text {
                        let support_text_pos = egui::pos2(image_rect.min.x + 8.0, image_rect.max.y - 16.0);
                        ui.painter().text(
                            support_text_pos,
                            egui::Align2::LEFT_TOP,
                            supporting_text,
                            egui::FontId::proportional(10.0),
                            get_global_color("onSurfaceVariant")
                        );
                    }
                } else {
                    // Draw text below image
                    let text_y = item_rect.max.y - 30.0;
                    let text_pos = egui::pos2(item_rect.min.x + 4.0, text_y);
                    
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_TOP,
                        &item.label,
                        egui::FontId::proportional(12.0),
                        text_color
                    );
                    // draw image_source if avalilable
                    if let Some(image_source) = &item.image_source {
                        let source_pos = egui::pos2(item_rect.min.x + 4.0, text_y + 14.0);
                        ui.painter().text(
                            source_pos,
                            egui::Align2::LEFT_TOP,
                            image_source,
                            egui::FontId::proportional(10.0),
                            get_global_color("onSurfaceVariant")
                        );
                    }
                    
                    if let Some(supporting_text) = &item.supporting_text {
                        let support_text_pos = egui::pos2(item_rect.min.x + 4.0, text_y + 14.0);
                        ui.painter().text(
                            support_text_pos,
                            egui::Align2::LEFT_TOP,
                            supporting_text,
                            egui::FontId::proportional(10.0),
                            get_global_color("onSurfaceVariant")
                        );
                    }
                }
            }
        }

        response
    }
}

/// Convenience function to create a standard image list.
pub fn image_list() -> MaterialImageList<'static> {
    MaterialImageList::standard()
}

/// Convenience function to create a masonry image list.
pub fn masonry_image_list() -> MaterialImageList<'static> {
    MaterialImageList::masonry()
}

/// Convenience function to create a woven image list.
pub fn woven_image_list() -> MaterialImageList<'static> {
    MaterialImageList::woven()
}