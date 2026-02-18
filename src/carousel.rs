use crate::get_global_color;
use egui::{self, FontId, Pos2, Rect, Response, Sense, Ui, Vec2};
use egui::epaint::CornerRadius;

/// A Material Design 3 Carousel component.
///
/// Carousels display a horizontally scrollable list of items where edge items
/// compress to a smaller size, creating a peek effect.
///
/// # Example
/// ```rust,no_run
/// # egui::__run_test_ui(|ui| {
/// let mut offset = 0.0f32;
/// ui.add(MaterialCarousel::new(&mut offset)
///     .item_text("Item 0")
///     .item_text("Item 1")
///     .item_text("Item 2"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialCarousel<'a> {
    /// Items to display in the carousel
    items: Vec<CarouselItem<'a>>,
    /// Width of each item at full size
    item_extent: f32,
    /// Minimum width for edge items (shrink size)
    shrink_extent: f32,
    /// Height of the carousel
    height: f32,
    /// Padding around each item
    padding: f32,
    /// Corner radius for item shapes
    corner_radius: f32,
    /// Whether items snap to boundaries when scrolling stops
    item_snapping: bool,
    /// Persistent scroll offset state
    scroll_offset: &'a mut f32,
    /// Optional salt for unique widget IDs
    id_salt: Option<String>,
}

/// A single item in a carousel.
pub struct CarouselItem<'a> {
    content: Box<dyn FnOnce(&mut Ui, Rect) + 'a>,
}

impl<'a> MaterialCarousel<'a> {
    /// Create a new carousel widget.
    ///
    /// # Arguments
    /// * `scroll_offset` - Mutable reference to persistent scroll state
    pub fn new(scroll_offset: &'a mut f32) -> Self {
        Self {
            items: Vec::new(),
            item_extent: 180.0,
            shrink_extent: 100.0,
            height: 150.0,
            padding: 4.0,
            corner_radius: 10.0,
            item_snapping: false,
            scroll_offset,
            id_salt: None,
        }
    }

    /// Add a custom item with a rendering closure.
    ///
    /// The closure receives `(&mut Ui, Rect)` where `Rect` is the available area.
    pub fn item(mut self, content: impl FnOnce(&mut Ui, Rect) + 'a) -> Self {
        self.items.push(CarouselItem {
            content: Box::new(content),
        });
        self
    }

    /// Add a simple text-label item.
    pub fn item_text(self, label: impl Into<String>) -> Self {
        let label = label.into();
        self.item(move |ui, rect| {
            let on_surface = get_global_color("onSurface");
            let center = rect.center();
            ui.painter().text(
                center,
                egui::Align2::CENTER_CENTER,
                &label,
                FontId::proportional(14.0),
                on_surface,
            );
        })
    }

    /// Set the full-size width of each item (default: 180.0).
    pub fn item_extent(mut self, extent: f32) -> Self {
        self.item_extent = extent;
        self
    }

    /// Set the minimum width for edge items (default: 100.0).
    pub fn shrink_extent(mut self, extent: f32) -> Self {
        self.shrink_extent = extent;
        self
    }

    /// Set the height of the carousel (default: 150.0).
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the padding around each item (default: 4.0).
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the corner radius for item shapes (default: 10.0).
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Enable or disable item snapping (default: false).
    pub fn item_snapping(mut self, snapping: bool) -> Self {
        self.item_snapping = snapping;
        self
    }

    /// Set an ID salt for unique widget identification.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Compute the width of an item based on its position relative to viewport edges.
    ///
    /// Items fully within the viewport get `item_extent`.
    /// Items partially outside shrink towards `shrink_extent`.
    fn compute_item_width(&self, item_left: f32, item_right: f32, viewport_left: f32, viewport_right: f32) -> f32 {
        let full = self.item_extent;
        let min = self.shrink_extent;

        // How much of the item is clipped on the left
        let left_clip = (viewport_left - item_left).max(0.0);
        // How much of the item is clipped on the right
        let right_clip = (item_right - viewport_right).max(0.0);

        let total_clip = left_clip + right_clip;
        if total_clip <= 0.0 {
            return full;
        }

        // Proportion clipped
        let clip_ratio = (total_clip / full).min(1.0);
        // Lerp from full to min
        let width = full - (full - min) * clip_ratio;
        width.max(min)
    }
}

impl<'a> egui::Widget for MaterialCarousel<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id_salt = self.id_salt.as_deref().unwrap_or("material_carousel");
        let id = ui.make_persistent_id(id_salt);

        let available_width = ui.available_width();
        let desired_size = Vec2::new(available_width, self.height);

        let (outer_rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if !ui.is_rect_visible(outer_rect) {
            return response;
        }

        // Theme colors
        let outline_color = get_global_color("outline");
        let surface_color = get_global_color("surface");

        let item_count = self.items.len();
        if item_count == 0 {
            return response;
        }

        // Total content width (all items at full extent + padding)
        let item_step = self.item_extent + self.padding * 2.0;
        let total_content_width = item_step * item_count as f32;
        let max_scroll = (total_content_width - available_width).max(0.0);

        // Handle scroll input
        let scroll_delta = ui.input(|i| {
            // Horizontal scroll or shift+vertical scroll
            let mut delta = 0.0;
            if let Some(hover_pos) = i.pointer.hover_pos() {
                if outer_rect.contains(hover_pos) {
                    delta -= i.smooth_scroll_delta.y;
                    delta -= i.smooth_scroll_delta.x;
                }
            }
            delta
        });

        *self.scroll_offset = (*self.scroll_offset + scroll_delta).clamp(0.0, max_scroll);

        // Item snapping: animate towards nearest item boundary
        if self.item_snapping && scroll_delta == 0.0 {
            let nearest_item = (*self.scroll_offset / item_step).round();
            let target = (nearest_item * item_step).clamp(0.0, max_scroll);
            let diff = target - *self.scroll_offset;
            if diff.abs() > 0.5 {
                *self.scroll_offset += diff * 0.15;
                ui.ctx().request_repaint();
            } else {
                *self.scroll_offset = target;
            }
        }

        let scroll = *self.scroll_offset;
        let viewport_left = scroll;
        let viewport_right = scroll + available_width;
        let painter = ui.painter_at(outer_rect);

        // Store values before moving self.items
        let item_extent = self.item_extent;
        let shrink_extent = self.shrink_extent;
        let padding = self.padding;
        let height = self.height;
        let corner_radius = self.corner_radius;

        // Determine visible item range
        let first_visible = ((scroll / item_step).floor() as i32).max(0) as usize;
        let last_visible = (((scroll + available_width) / item_step).ceil() as usize).min(item_count);

        // We need to consume items, so iterate with index tracking
        let mut items_vec: Vec<Option<CarouselItem<'a>>> = self.items.into_iter().map(Some).collect();

        for i in first_visible..last_visible {
            let item_content_left = i as f32 * item_step + padding;
            let item_content_right = item_content_left + item_extent;

            // Compute compressed width based on edge proximity
            let left_clip_calc = (viewport_left - item_content_left).max(0.0);
            let right_clip_calc = (item_content_right - viewport_right).max(0.0);
            let total_clip = left_clip_calc + right_clip_calc;
            let display_width = if total_clip <= 0.0 {
                item_extent
            } else {
                let clip_ratio = (total_clip / item_extent).min(1.0);
                let width = item_extent - (item_extent - shrink_extent) * clip_ratio;
                width.max(shrink_extent)
            };

            // Position relative to the outer_rect
            let screen_x = item_content_left - scroll + outer_rect.left();

            // Adjust position for clipped items:
            // If item is being clipped on the left, shift it right
            let left_clip = (viewport_left - item_content_left).max(0.0);
            let adjusted_x = if left_clip > 0.0 {
                outer_rect.left()
            } else {
                screen_x
            };

            let item_rect = Rect::from_min_size(
                Pos2::new(adjusted_x, outer_rect.top() + padding),
                Vec2::new(display_width, height - padding * 2.0),
            );

            // Clip to the outer rect
            let clipped_rect = item_rect.intersect(outer_rect);
            if clipped_rect.width() <= 0.0 || clipped_rect.height() <= 0.0 {
                continue;
            }

            let rounding = CornerRadius::same(corner_radius as u8);

            // Draw item background
            painter.rect_filled(clipped_rect, rounding, surface_color);

            // Draw item border
            painter.rect_stroke(
                clipped_rect,
                rounding,
                egui::Stroke::new(1.0, outline_color),
                egui::epaint::StrokeKind::Outside,
            );

            // Render content
            if let Some(item) = items_vec[i].take() {
                // Create a child UI clipped to the item rect
                let mut child_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(clipped_rect)
                        .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown)),
                );
                child_ui.set_clip_rect(clipped_rect);
                (item.content)(&mut child_ui, clipped_rect);
            }
        }

        response
    }
}

/// Create a new carousel widget.
///
/// # Arguments
/// * `scroll_offset` - Mutable reference to persistent scroll state
///
/// # Example
/// ```rust,no_run
/// # egui::__run_test_ui(|ui| {
/// let mut offset = 0.0f32;
/// ui.add(carousel(&mut offset)
///     .item_text("First")
///     .item_text("Second"));
/// # });
/// ```
pub fn carousel<'a>(scroll_offset: &'a mut f32) -> MaterialCarousel<'a> {
    MaterialCarousel::new(scroll_offset)
}
