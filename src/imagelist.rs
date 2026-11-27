// ============================================================================
// Module:       imagelist
// Description:  Material Design image lists, with images loaded off the UI
//               thread under size and decode limits.
//
// Dependencies: egui, image, base64, hex, ureq (ondemand)
// ============================================================================

//! # Material Design Image Lists
//!
//! This module provides Material Design 3 image list components with comprehensive
//! image source support and intelligent caching capabilities.
//!
//! ## Features
//!
//! - **Multiple image sources**: Local files, online URLs, and embedded byte arrays
//! - **Never blocks a frame**: every source (file, URL, data URL, embedded
//!   bytes) is read and decoded on a background worker; the list draws a
//!   placeholder until the image is ready, then repaints
//! - **Bounded**: a source larger than 16 MiB, or an image larger than
//!   8192 x 8192, is rejected rather than read or decoded; decoded images are
//!   scaled down to at most 512 x 512
//! - **Caching**: each source is loaded once per [`egui::Context`] and kept in
//!   memory as a texture; nothing is written to disk
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
//! With the `ondemand` feature, `http://` and `https://` sources are
//! downloaded on the same background workers, with 10 s connect and read
//! timeouts. A failed download is not retried. Without the feature, URL
//! sources show the failed-image mark.
//!
//! ## Colours
//!
//! Colours come from the context's [`MaterialTheme`]: `surface` behind the
//! grid, `surfaceContainerHighest` behind each image while it loads,
//! `outlineVariant` around it, `onSurface` for labels and the state layer,
//! `onSurfaceVariant` for supporting text, and `error` for the failed mark.

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use crate::tokens::{ColorRole, InteractionState, TypeRole};
use crate::MaterialTheme;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke},
    Rect, Response, Sense, Ui, Vec2, Widget,
};
use image::GenericImageView;

/// Material Design image list variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageListVariant {
    /// Square tiles.
    Standard,
    /// Tiles 1.2 times as tall as they are wide.
    Masonry,
    /// Tiles 0.8 times as tall as they are wide.
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

/// One tile of a [`MaterialImageList`].
pub struct ImageListItem<'a> {
    /// The tile's label, also its accessible name.
    pub label: String,
    /// Where the image comes from: a file path, an `http(s)://` URL (with the
    /// `ondemand` feature), a `data:` URL, or `bytes:` followed by hex.
    pub image_source: Option<String>,
    /// A second line under the label.
    pub supporting_text: Option<String>,
    /// Called when the tile is clicked.
    pub on_click: Option<Box<dyn Fn() + Send + Sync>>,
    /// An already-decoded image. When set, it is drawn instead of loading
    /// `image_source`.
    pub loaded_image: Option<egui::ColorImage>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> ImageListItem<'a> {
    /// A tile labelled `label` showing the image at `image_source`.
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

    /// Sets the supporting text.
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Sets the click callback.
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

// ---------------------------------------------------------------------------
// Loading: every source is read and decoded on a background worker.

/// Largest source accepted, in bytes: a file, a response body, or the bytes
/// a `data:` or `bytes:` source decodes to.
const MAX_SOURCE_BYTES: u64 = 16 * 1024 * 1024;

/// Largest width or height accepted when decoding.
const MAX_DECODED_DIMENSION: u32 = 8192;

/// Most memory the decoder may allocate for one image.
const MAX_DECODE_ALLOC: u64 = 512 * 1024 * 1024;

/// Decoded images are scaled down to fit this square.
const MAX_DISPLAY_DIMENSION: u32 = 512;

/// Background workers shared by every image list in the process.
const WORKERS: usize = 4;

/// Load state of one source in one context.
#[derive(Clone)]
enum Slot {
    /// Queued or being read on a worker.
    Loading,
    /// Decoded on a worker, waiting to be uploaded as a texture.
    Decoded(egui::ColorImage),
    /// Uploaded.
    Ready(egui::TextureHandle),
    /// Could not be read or decoded; not retried.
    Failed,
}

/// The load states of one context's sources, by source string.
type Slots = Arc<Mutex<HashMap<String, Slot>>>;

/// One load for a worker.
struct Job {
    source: String,
    slots: Slots,
    ctx: egui::Context,
}

/// What a tile can draw this frame.
enum ImageState {
    Loading,
    Ready(egui::TextureId),
    Failed,
}

/// The slots of `ctx`, created on first use.
fn slots(ctx: &egui::Context) -> Slots {
    let id = egui::Id::unique("egui_material3::imagelist::slots");
    ctx.data_mut(|d| d.get_temp_mut_or_default::<Slots>(id).clone())
}

/// The queue into the worker pool, or `None` where threads cannot be
/// spawned (for example on the web).
fn queue() -> Option<&'static Mutex<Sender<Job>>> {
    static QUEUE: OnceLock<Option<Mutex<Sender<Job>>>> = OnceLock::new();
    QUEUE
        .get_or_init(|| {
            let (sender, receiver) = std::sync::mpsc::channel::<Job>();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut spawned = 0;
            for n in 0..WORKERS {
                let receiver = Arc::clone(&receiver);
                let worker = std::thread::Builder::new()
                    .name(format!("egui-material3-image-{n}"))
                    .spawn(move || work(&receiver));
                if worker.is_ok() {
                    spawned += 1;
                }
            }
            (spawned > 0).then(|| Mutex::new(sender))
        })
        .as_ref()
}

/// A worker: loads jobs until the queue closes.
fn work(receiver: &Mutex<Receiver<Job>>) {
    loop {
        let next = receiver
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .recv();
        let Ok(job) = next else {
            return;
        };
        let slot = load_source(&job.source).map_or(Slot::Failed, Slot::Decoded);
        job.slots
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(job.source, slot);
        job.ctx.request_repaint();
    }
}

/// What to draw for `source` this frame. The first call for a source queues
/// it on a worker and returns [`ImageState::Loading`]; nothing here reads a
/// file, the network or decodes an image on the calling thread, except where
/// no worker can run, and then only for in-memory sources.
fn image_for(ctx: &egui::Context, source: &str) -> ImageState {
    let slots = slots(ctx);
    let mut map = slots.lock().unwrap_or_else(PoisonError::into_inner);
    match map.get(source).cloned() {
        Some(Slot::Ready(texture)) => return ImageState::Ready(texture.id()),
        Some(Slot::Failed) => return ImageState::Failed,
        Some(Slot::Loading) => return ImageState::Loading,
        Some(Slot::Decoded(image)) => {
            let texture = ctx.load_texture(
                "egui_material3::imagelist",
                image,
                egui::TextureOptions::default(),
            );
            let id = texture.id();
            map.insert(source.to_owned(), Slot::Ready(texture));
            return ImageState::Ready(id);
        }
        None => {}
    }

    let job = Job {
        source: source.to_owned(),
        slots: Arc::clone(&slots),
        ctx: ctx.clone(),
    };
    let queued = queue().is_some_and(|queue| {
        queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .send(job)
            .is_ok()
    });
    if queued {
        map.insert(source.to_owned(), Slot::Loading);
        return ImageState::Loading;
    }

    // No workers: decode in-memory sources here (no I/O), fail the rest.
    let slot = if is_in_memory(source) {
        load_source(source).map_or(Slot::Failed, Slot::Decoded)
    } else {
        Slot::Failed
    };
    map.insert(source.to_owned(), slot);
    drop(map);
    ctx.request_repaint();
    ImageState::Loading
}

fn is_in_memory(source: &str) -> bool {
    source.starts_with("data:") || source.starts_with("bytes:")
}

/// Reads and decodes one source. Runs on a worker.
fn load_source(source: &str) -> Option<egui::ColorImage> {
    let bytes = if source.starts_with("http://") || source.starts_with("https://") {
        download(source)?
    } else if let Some(data_url) = source.strip_prefix("data:") {
        decode_data_url(data_url)?
    } else if let Some(hex_bytes) = source.strip_prefix("bytes:") {
        decode_hex(hex_bytes)?
    } else {
        read_file(std::path::Path::new(source))?
    };
    decode_limited(&bytes)
}

/// Reads at most `limit` bytes; a longer or empty input is rejected.
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

fn read_file(path: &std::path::Path) -> Option<Vec<u8>> {
    let file = std::fs::File::open(path).ok()?;
    if file.metadata().ok()?.len() > MAX_SOURCE_BYTES {
        return None;
    }
    read_capped(file, MAX_SOURCE_BYTES)
}

/// Decodes the part after `data:`: `<mediatype>[;base64],<payload>`. The
/// payload length is checked before decoding.
fn decode_data_url(data_url: &str) -> Option<Vec<u8>> {
    let (_, payload) = data_url.split_once(',')?;
    // Base64 is 4 characters per 3 bytes.
    if payload.len() as u64 > MAX_SOURCE_BYTES.saturating_mul(4) / 3 + 4 {
        return None;
    }
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, payload).ok()
}

fn decode_hex(hex_bytes: &str) -> Option<Vec<u8>> {
    if hex_bytes.len() as u64 > MAX_SOURCE_BYTES.saturating_mul(2) {
        return None;
    }
    hex::decode(hex_bytes).ok()
}

/// Downloads one image body. Runs on a worker.
#[cfg(feature = "ondemand")]
fn download(url: &str) -> Option<Vec<u8>> {
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
    read_capped(response.into_reader(), MAX_SOURCE_BYTES)
}

/// Without the `ondemand` feature there is no HTTP client.
#[cfg(not(feature = "ondemand"))]
fn download(_url: &str) -> Option<Vec<u8>> {
    None
}

/// Decodes an image under dimension and allocation limits, scaled down to fit
/// [`MAX_DISPLAY_DIMENSION`].
fn decode_limited(bytes: &[u8]) -> Option<egui::ColorImage> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_DECODED_DIMENSION);
    limits.max_image_height = Some(MAX_DECODED_DIMENSION);
    limits.max_alloc = Some(MAX_DECODE_ALLOC);
    reader.limits(limits);
    let image = reader.decode().ok()?;
    let image = if image.width() > MAX_DISPLAY_DIMENSION || image.height() > MAX_DISPLAY_DIMENSION {
        image.resize(
            MAX_DISPLAY_DIMENSION,
            MAX_DISPLAY_DIMENSION,
            image::imageops::FilterType::Lanczos3,
        )
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

// ---------------------------------------------------------------------------
// The widget.

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
            // ShapeKeyTokens.CornerExtraSmall (4 dp).
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
}

impl<'a> Default for MaterialImageList<'a> {
    fn default() -> Self {
        Self::standard()
    }
}

impl Widget for MaterialImageList<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let m3 = MaterialTheme::of(ui.ctx());

        let MaterialImageList {
            variant,
            items,
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

        if !ui.is_rect_visible(rect) {
            return response;
        }

        let on_surface = m3.color(ColorRole::OnSurface);
        let on_surface_variant = m3.color(ColorRole::OnSurfaceVariant);
        let label_font = TypeRole::LabelMedium.style().font_id(ui.ctx());
        let supporting_font = TypeRole::LabelSmall.style().font_id(ui.ctx());

        ui.painter()
            .rect_filled(rect, corner_radius, m3.color(ColorRole::Surface));

        for (index, item) in items.iter().enumerate() {
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
            crate::a11y::button(&item_response, true, &item.label);

            if item_response.clicked() {
                if let Some(callback) = &item.on_click {
                    callback();
                }
            }

            // Placeholder behind the image while it loads.
            let image_rect = item_rect.shrink(2.0);
            ui.painter().rect_filled(
                image_rect,
                corner_radius,
                m3.color(ColorRole::SurfaceContainerHighest),
            );
            ui.painter().rect_stroke(
                image_rect,
                corner_radius,
                Stroke::new(1.0, m3.color(ColorRole::OutlineVariant)),
                egui::epaint::StrokeKind::Inside,
            );

            let state = match (&item.loaded_image, &item.image_source) {
                (Some(image), _) => {
                    // A caller-decoded image, uploaded under this tile's id.
                    let texture = ui.ctx().load_texture(
                        format!("egui_material3::imagelist::{}", item_id.value()),
                        image.clone(),
                        egui::TextureOptions::default(),
                    );
                    let id = texture.id();
                    // Keep the handle alive until the frame is painted.
                    ui.data_mut(|d| d.insert_temp(item_id, texture));
                    ImageState::Ready(id)
                }
                (None, Some(source)) => image_for(ui.ctx(), source),
                (None, None) => ImageState::Failed,
            };

            match state {
                ImageState::Ready(texture_id) => {
                    ui.painter().image(
                        texture_id,
                        image_rect,
                        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
                ImageState::Loading => {}
                ImageState::Failed => {
                    let stroke = Stroke::new(2.0, m3.color(ColorRole::Error));
                    ui.painter()
                        .line_segment([image_rect.min, image_rect.max], stroke);
                    ui.painter().line_segment(
                        [
                            egui::pos2(image_rect.min.x, image_rect.max.y),
                            egui::pos2(image_rect.max.x, image_rect.min.y),
                        ],
                        stroke,
                    );
                }
            }

            // M3 state layer (StateTokens) of onSurface over the tile.
            crate::paint::paint_state_layer(
                ui.painter(),
                item_rect,
                corner_radius,
                on_surface,
                InteractionState::from_response(&item_response),
            );

            if text_protected {
                // A dark band keeps white text readable over any image.
                let overlay_rect = Rect::from_min_size(
                    egui::pos2(image_rect.min.x, image_rect.max.y - 40.0),
                    Vec2::new(image_rect.width(), 40.0),
                );
                ui.painter().rect_filled(
                    overlay_rect,
                    CornerRadius::ZERO,
                    Color32::from_black_alpha(128),
                );

                ui.painter().text(
                    egui::pos2(image_rect.min.x + 8.0, image_rect.max.y - 30.0),
                    egui::Align2::LEFT_TOP,
                    &item.label,
                    label_font.clone(),
                    Color32::WHITE,
                );

                if let Some(supporting_text) = &item.supporting_text {
                    ui.painter().text(
                        egui::pos2(image_rect.min.x + 8.0, image_rect.max.y - 16.0),
                        egui::Align2::LEFT_TOP,
                        supporting_text,
                        supporting_font.clone(),
                        Color32::from_white_alpha(200),
                    );
                }
            } else {
                // Draw text below image
                let text_y = item_rect.max.y - 30.0;
                ui.painter().text(
                    egui::pos2(item_rect.min.x + 4.0, text_y),
                    egui::Align2::LEFT_TOP,
                    &item.label,
                    label_font.clone(),
                    on_surface,
                );

                // Supporting text, or else the source for anything but
                // embedded data (a hex or base64 blob is no use as a label).
                let second_line = item.supporting_text.as_deref().or_else(|| {
                    item.image_source
                        .as_deref()
                        .filter(|source| !is_in_memory(source))
                });
                if let Some(text) = second_line {
                    ui.painter().text(
                        egui::pos2(item_rect.min.x + 4.0, text_y + 14.0),
                        egui::Align2::LEFT_TOP,
                        text,
                        supporting_font.clone(),
                        on_surface_variant,
                    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn frame(ctx: &egui::Context, mut app: impl FnMut(&mut Ui)) {
        ctx.run_ui(Default::default(), |ui| app(ui))
            .textures_delta
            .clear();
    }

    fn png(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        let image = image::RgbaImage::from_pixel(width, height, image::Rgba([10, 20, 30, 255]));
        let written = image::DynamicImage::ImageRgba8(image).write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        );
        assert!(written.is_ok(), "encoding the test PNG should work");
        bytes
    }

    /// Runs frames until `source` leaves the loading state, or 10 s pass.
    fn settle(ctx: &egui::Context, source: &str) -> ImageState {
        let started = Instant::now();
        loop {
            frame(ctx, |ui| {
                ui.add(image_list().item("Image", source));
            });
            match image_for(ctx, source) {
                ImageState::Loading if started.elapsed() < Duration::from_secs(10) => {
                    std::thread::sleep(Duration::from_millis(10));
                }
                state => return state,
            }
        }
    }

    /// Regression test for nikescar/egui-material3#19: drawing an image list
    /// with a remote image does not wait for the server.
    #[cfg(feature = "ondemand")]
    #[test]
    fn test_remote_image_does_not_block_the_frame() -> std::io::Result<()> {
        // A server that accepts the connection and never answers.
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let url = format!("http://{}/image.png", listener.local_addr()?);
        let ctx = egui::Context::default();
        let started = Instant::now();
        for _ in 0..3 {
            frame(&ctx, |ui| {
                ui.add(image_list().item("Remote", url.clone()));
            });
        }
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "drawing the list should not wait for the download"
        );
        assert!(
            matches!(image_for(&ctx, &url), ImageState::Loading),
            "the stalled download should still be loading"
        );
        Ok(())
    }

    /// A local file is read on a worker and shows up on a later frame.
    #[test]
    fn test_local_file_loads_off_the_frame() -> std::io::Result<()> {
        let dir = std::env::temp_dir().join(format!(
            "egui-material3-imagelist-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("tile.png");
        std::fs::write(&path, png(4, 4))?;
        let source = path.to_string_lossy().to_string();

        let ctx = egui::Context::default();
        let state = settle(&ctx, &source);
        // Best-effort cleanup of the temporary directory.
        std::fs::remove_dir_all(&dir).ok();
        assert!(
            matches!(state, ImageState::Ready(_)),
            "the local file should load"
        );
        Ok(())
    }

    #[test]
    fn test_missing_file_fails() {
        let ctx = egui::Context::default();
        let source = "this/file/does/not/exist.png";
        assert!(
            matches!(settle(&ctx, source), ImageState::Failed),
            "a missing file should fail"
        );
    }

    #[test]
    fn test_embedded_bytes_load() {
        let ctx = egui::Context::default();
        let source = format!("bytes:{}", hex::encode(png(3, 2)));
        assert!(
            matches!(settle(&ctx, &source), ImageState::Ready(_)),
            "embedded bytes should load"
        );
    }

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

    #[test]
    fn test_image_over_the_dimension_limit_is_rejected() {
        let too_wide = png(MAX_DECODED_DIMENSION + 1, 1);
        assert!(
            decode_limited(&too_wide).is_none(),
            "an image wider than the limit should not be decoded"
        );
        let decoded = decode_limited(&png(1024, 8));
        assert_eq!(
            decoded.map(|image| image.size),
            Some([512, 4]),
            "a large image should be scaled to fit 512 x 512"
        );
    }

    #[test]
    fn test_oversized_inline_sources_are_rejected_before_decoding() {
        let hex_len = usize::try_from(MAX_SOURCE_BYTES * 2 + 2).unwrap_or(usize::MAX);
        assert!(
            decode_hex(&"0".repeat(hex_len)).is_none(),
            "hex over the cap should be rejected"
        );
        let b64_len = usize::try_from(MAX_SOURCE_BYTES * 2).unwrap_or(usize::MAX);
        assert!(
            decode_data_url(&format!("image/png;base64,{}", "A".repeat(b64_len))).is_none(),
            "a data URL over the cap should be rejected"
        );
    }
}
