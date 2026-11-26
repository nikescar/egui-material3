//! # Material Design Image Lists
//!
//! This module provides Material Design 3 image list components with comprehensive
//! image source support and intelligent caching capabilities.
//!
//! ## Features
//!
//! - **Multiple image sources**: Local files, online URLs, and embedded byte arrays
//! - **Caching**: Downloaded images are kept in memory for the life of the process
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
//! egui-material3 = { version = "0.0.6", features = ["ondemand"] }
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
//! - **Background downloading**: Images are downloaded on a background thread on
//!   first access; the UI never waits for the network, and repaints when an
//!   image arrives
//! - **Bounded**: A response larger than 16 MiB, or an image larger than
//!   8192 x 8192, is rejected
//! - **In-memory cache**: Each URL is downloaded once per process; a failed
//!   download is not retried, and nothing is written to disk
//! - **Error handling**: Failed downloads show visual indicators instead of crashing

use crate::theme::get_global_color;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke},
    Rect, Response, Sense, Ui, Vec2, Widget,
};
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
                Some(egui::ColorImage::from_rgba_unmultiplied(
                    [size.0 as usize, size.1 as usize],
                    &pixels,
                ))
            }
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Largest response body accepted for a remote image.
#[cfg(feature = "ondemand")]
const MAX_DOWNLOAD_BYTES: u64 = 16 * 1024 * 1024;

/// Largest width or height accepted when decoding a remote image.
#[cfg(feature = "ondemand")]
const MAX_DECODED_DIMENSION: u32 = 8192;

/// State of one remote image download.
#[cfg(feature = "ondemand")]
#[derive(Clone)]
enum Download {
    Loading,
    Ready(egui::ColorImage),
    Failed,
}

/// The remote image cache type: download state by URL.
#[cfg(feature = "ondemand")]
type DownloadMap = std::sync::Mutex<std::collections::HashMap<String, Download>>;

/// Remote images by URL, shared by every image list in the process.
#[cfg(feature = "ondemand")]
fn downloads() -> &'static DownloadMap {
    static DOWNLOADS: std::sync::OnceLock<DownloadMap> = std::sync::OnceLock::new();
    DOWNLOADS.get_or_init(Default::default)
}

/// Returns the image for `url` if it has been downloaded. The first call
/// for a URL starts the download on a background thread and returns
/// `Ok(None)`; a failed download returns `Err(())` and is not retried.
#[cfg(feature = "ondemand")]
fn load_image_from_url(ctx: &egui::Context, url: &str) -> Result<Option<egui::ColorImage>, ()> {
    let mut map = downloads()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match map.get(url) {
        Some(Download::Ready(image)) => return Ok(Some(image.clone())),
        Some(Download::Failed) => return Err(()),
        Some(Download::Loading) => return Ok(None),
        None => {}
    }
    map.insert(url.to_owned(), Download::Loading);
    drop(map);

    let ctx = ctx.clone();
    let url = url.to_owned();
    let spawned = std::thread::Builder::new()
        .name("egui-material3-image".to_owned())
        .spawn(move || {
            let state = download_image(&url).map_or(Download::Failed, Download::Ready);
            downloads()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(url, state);
            ctx.request_repaint();
        });
    if spawned.is_err() {
        return Err(());
    }
    Ok(None)
}

/// Downloads and decodes one image. Runs on a background thread.
#[cfg(feature = "ondemand")]
fn download_image(url: &str) -> Option<egui::ColorImage> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(10))
        .timeout_read(std::time::Duration::from_secs(10))
        .timeout_write(std::time::Duration::from_secs(10))
        .user_agent("egui-material3/1.0")
        .build();
    let response = agent.get(url).call().ok()?;
    if response.status() != 200 {
        return None;
    }
    let bytes = read_capped(response.into_reader(), MAX_DOWNLOAD_BYTES)?;
    decode_limited(&bytes)
}

/// Reads at most `limit` bytes; a longer or empty body is rejected.
#[cfg(feature = "ondemand")]
fn read_capped(reader: impl std::io::Read, limit: u64) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut bytes = Vec::new();
    reader
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.is_empty() || bytes.len() as u64 > limit {
        return None;
    }
    Some(bytes)
}

/// Decodes an image with dimension limits, resized to at most 512 x 512.
#[cfg(feature = "ondemand")]
fn decode_limited(bytes: &[u8]) -> Option<egui::ColorImage> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_DECODED_DIMENSION);
    limits.max_image_height = Some(MAX_DECODED_DIMENSION);
    reader.limits(limits);
    let image = reader.decode().ok()?;
    let image = if image.width() > 512 || image.height() > 512 {
        image.resize(512, 512, image::imageops::FilterType::Lanczos3)
    } else {
        image
    };
    let size = image.dimensions();
    let pixels = image.to_rgba8().into_raw();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [size.0 as usize, size.1 as usize],
        &pixels,
    ))
}

/// Load image from data URL (base64 encoded)
fn load_image_from_data_url(data_url: &str) -> Option<egui::ColorImage> {
    if let Some((_, data_part)) = data_url.split_once(',') {
        if let Ok(bytes) =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data_part)
        {
            if let Ok(image) = image::load_from_memory(&bytes) {
                let size = image.dimensions();
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.into_raw();
                return Some(egui::ColorImage::from_rgba_unmultiplied(
                    [size.0 as usize, size.1 as usize],
                    &pixels,
                ));
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
            return Some(egui::ColorImage::from_rgba_unmultiplied(
                [size.0 as usize, size.1 as usize],
                &pixels,
            ));
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
        Self {
            variant,
            items: Vec::new(),
            columns: 3,
            item_spacing: 8.0,
            text_protected: false,
            corner_radius: CornerRadius::from(4.0),
            id_salt: None,
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
        callback: F,
    ) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.items
            .push(ImageListItem::new(label, image_source).on_click(callback));
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

        let rows = items.len().div_ceil(columns);
        let total_height = rows as f32 * (item_height + item_spacing) - item_spacing;
        let total_width = available_width;

        let response = ui.allocate_response(Vec2::new(total_width, total_height), Sense::hover());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw background
            ui.painter()
                .rect_filled(rect, corner_radius, background_color);

            // Draw items in grid
            for (index, item) in items.iter_mut().enumerate() {
                let row = index / columns;
                let col = index % columns;

                let item_x = rect.min.x + col as f32 * (item_width + item_spacing);
                let item_y = rect.min.y + row as f32 * (item_height + item_spacing);

                let item_rect = Rect::from_min_size(
                    egui::pos2(item_x, item_y),
                    Vec2::new(item_width, item_height),
                );

                // Handle item interaction with unique ID
                let item_id = if let Some(ref salt) = id_salt {
                    egui::Id::unique((salt, "image_item", index))
                } else {
                    egui::Id::unique(("image_item", index, &item.label))
                };

                let item_response = ui.interact(item_rect, item_id, Sense::click());
                if item_response.hovered() {
                    let hover_color = get_global_color("primary").linear_multiply(0.08);
                    ui.painter()
                        .rect_filled(item_rect, corner_radius, hover_color);
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

                ui.painter()
                    .rect_filled(image_rect, corner_radius, image_bg);
                ui.painter().rect_stroke(
                    image_rect,
                    corner_radius,
                    image_border,
                    egui::epaint::StrokeKind::Outside,
                );

                // Load and cache image if not already loaded
                let mut loading = false;
                if item.loaded_image.is_none() {
                    if let Some(ref image_source) = item.image_source {
                        let loaded_image = if image_source.starts_with("http://")
                            || image_source.starts_with("https://")
                        {
                            #[cfg(feature = "ondemand")]
                            {
                                match load_image_from_url(ui.ctx(), image_source) {
                                    Ok(Some(image)) => Some(image),
                                    // Still downloading: draw the placeholder only.
                                    Ok(None) => {
                                        loading = true;
                                        None
                                    }
                                    Err(()) => None,
                                }
                            }
                            #[cfg(not(feature = "ondemand"))]
                            {
                                loading = false;
                                None
                            }
                        } else if image_source.starts_with("data:") {
                            load_image_from_data_url(image_source)
                        } else if let Some(bytes_str) = image_source.strip_prefix("bytes:") {
                            // Remove "bytes:" prefix
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
                        Default::default(),
                    );
                    ui.painter().image(
                        texture_id.id(),
                        image_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                } else {
                    failed = !loading;
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
                        [
                            egui::pos2(image_rect.min.x, image_rect.max.y),
                            egui::pos2(image_rect.max.x, image_rect.min.y),
                        ],
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
                        Vec2::new(image_rect.width(), 40.0),
                    );
                    let overlay_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);
                    ui.painter()
                        .rect_filled(overlay_rect, CornerRadius::ZERO, overlay_color);

                    // Draw text on overlay
                    let text_pos = egui::pos2(image_rect.min.x + 8.0, image_rect.max.y - 30.0);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_TOP,
                        &item.label,
                        egui::FontId::proportional(12.0),
                        text_color,
                    );

                    if let Some(supporting_text) = &item.supporting_text {
                        let support_text_pos =
                            egui::pos2(image_rect.min.x + 8.0, image_rect.max.y - 16.0);
                        ui.painter().text(
                            support_text_pos,
                            egui::Align2::LEFT_TOP,
                            supporting_text,
                            egui::FontId::proportional(10.0),
                            get_global_color("onSurfaceVariant"),
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
                        text_color,
                    );
                    // draw image_source if avalilable
                    if let Some(image_source) = &item.image_source {
                        let source_pos = egui::pos2(item_rect.min.x + 4.0, text_y + 14.0);
                        ui.painter().text(
                            source_pos,
                            egui::Align2::LEFT_TOP,
                            image_source,
                            egui::FontId::proportional(10.0),
                            get_global_color("onSurfaceVariant"),
                        );
                    }

                    if let Some(supporting_text) = &item.supporting_text {
                        let support_text_pos = egui::pos2(item_rect.min.x + 4.0, text_y + 14.0);
                        ui.painter().text(
                            support_text_pos,
                            egui::Align2::LEFT_TOP,
                            supporting_text,
                            egui::FontId::proportional(10.0),
                            get_global_color("onSurfaceVariant"),
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

#[cfg(all(test, feature = "ondemand"))]
mod download_tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// Regression test for nikescar/egui-material3#19: drawing an image list
    /// with a remote image does not wait for the server.
    #[test]
    fn test_remote_image_does_not_block_the_frame() -> std::io::Result<()> {
        // A server that accepts the connection and never answers.
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let url = format!("http://{}/image.png", listener.local_addr()?);
        let ctx = egui::Context::default();
        let started = Instant::now();
        for _ in 0..3 {
            ctx.run_ui(Default::default(), |ui| {
                ui.add(image_list().item("Remote", url.clone()));
            })
            .textures_delta
            .clear();
        }
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "drawing the list should not wait for the download"
        );
        Ok(())
    }

    /// Regression test for nikescar/egui-material3#19: a body over the cap is
    /// rejected rather than buffered.
    #[test]
    fn test_body_over_the_cap_is_rejected() {
        let body = vec![0_u8; 101];
        assert!(
            read_capped(body.as_slice(), 100).is_none(),
            "a body over the cap should be rejected"
        );
        assert_eq!(
            read_capped(body.get(..100).unwrap_or_default(), 100).map(|b| b.len()),
            Some(100),
            "a body at the cap should be read"
        );
    }
}
