use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget,
};

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
///     .item("Image 1", "path/to/image1.jpg")
///     .item("Image 2", "path/to/image2.jpg")
///     .item("Image 3", "path/to/image3.jpg");
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
    pub image_source: String,
    pub supporting_text: Option<String>,
    pub on_click: Option<Box<dyn Fn() + Send + Sync>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> ImageListItem<'a> {
    pub fn new(label: impl Into<String>, image_source: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            image_source: image_source.into(),
            supporting_text: None,
            on_click: None,
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
            for (index, item) in items.iter().enumerate() {
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

                // Draw image icon placeholder (camera icon representation)
                let icon_center = image_rect.center();
                let icon_color = get_global_color("onSurfaceVariant");
                ui.painter().circle_filled(icon_center, 16.0, icon_color);
                ui.painter().circle_filled(icon_center + Vec2::new(0.0, -4.0), 6.0, Color32::WHITE);

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