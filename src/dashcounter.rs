//! Material Design 3 Dashboard Counter Components
//!
//! # M3 Color Role Usage
//!
//! - **surface**: Card background
//! - **onSurface**: Text and content
//! - **surfaceContainerHighest**: Container background
//! - **primary**: Counter text and highlights
//! - **onSurfaceVariant**: Category text

use crate::get_global_color;
use egui::{self, FontId, Pos2, Rect, Response, Sense, Ui, Vec2};
use egui::epaint::CornerRadius;

/// A Material Design 3 Dashboard Counter component.
///
/// Displays a title and horizontally scrollable counter cards.
/// Each card shows a category name and a counter in the format "xx/yy".
///
/// # Example
/// ```rust,no_run
/// # egui::__run_test_ui(|ui| {
/// let mut offset = 0.0f32;
/// ui.add(MaterialDashCounter::new("Dashboard", &mut offset)
///     .card("Apps", 5, 10)
///     .card("Updates", 2, 8)
///     .card("Alerts", 0, 3));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialDashCounter<'a> {
    /// Title displayed at the top
    title: String,
    /// Counter cards to display
    cards: Vec<CounterCard>,
    /// Width of each card
    card_width: f32,
    /// Height of the entire widget
    height: f32,
    /// Padding around elements
    padding: f32,
    /// Corner radius for cards
    corner_radius: f32,
    /// Persistent scroll offset state
    scroll_offset: &'a mut f32,
    /// Optional salt for unique widget IDs
    id_salt: Option<String>,
    /// Click callback - receives card index
    on_click: Option<Box<dyn FnMut(usize) + 'a>>,
    /// Optional color for category text (defaults to onSurfaceVariant)
    category_color: Option<egui::Color32>,
    /// Optional color for counter numbers (defaults to primary)
    counter_color: Option<egui::Color32>,
    /// Optional color for description text (defaults to onSurfaceVariant)
    description_color: Option<egui::Color32>,
    /// Optional callback to add UI controls in the title area
    title_ui: Option<Box<dyn FnMut(&mut Ui) + 'a>>,
}

/// A single counter card with category and counter display.
pub struct CounterCard {
    /// Category name
    category: String,
    /// Current/sub counter value (xx in xx/yy)
    sub_counter: usize,
    /// Total counter value (yy in xx/yy)
    total_counter: usize,
    /// Optional description text below the sub counter (xx)
    sub_description: Option<String>,
    /// Optional description text below the total counter (yy)
    total_description: Option<String>,
    /// Optional color for this card's category text
    category_color: Option<egui::Color32>,
    /// Optional color for this card's counter numbers
    counter_color: Option<egui::Color32>,
    /// Optional color for this card's description text
    description_color: Option<egui::Color32>,
}

impl<'a> MaterialDashCounter<'a> {
    /// Create a new dashboard counter widget.
    ///
    /// # Arguments
    /// * `title` - Title text to display at the top
    /// * `scroll_offset` - Mutable reference to persistent scroll state
    pub fn new(title: impl Into<String>, scroll_offset: &'a mut f32) -> Self {
        Self {
            title: title.into(),
            cards: Vec::new(),
            card_width: 150.0,
            height: 96.0,
            padding: 8.0,
            corner_radius: 12.0,
            scroll_offset,
            id_salt: None,
            on_click: None,
            category_color: None,
            counter_color: None,
            description_color: None,
            title_ui: None,
        }
    }

    /// Add a counter card.
    ///
    /// # Arguments
    /// * `category` - Category name
    /// * `sub_counter` - Current/sub counter value (xx)
    /// * `total_counter` - Total counter value (yy)
    pub fn card(
        mut self,
        category: impl Into<String>,
        sub_counter: usize,
        total_counter: usize,
    ) -> Self {
        self.cards.push(CounterCard {
            category: category.into(),
            sub_counter,
            total_counter,
            sub_description: None,
            total_description: None,
            category_color: None,
            counter_color: None,
            description_color: None,
        });
        self
    }

    /// Add a counter card with descriptions below the counters.
    ///
    /// # Arguments
    /// * `category` - Category name
    /// * `sub_counter` - Current/sub counter value (xx)
    /// * `total_counter` - Total counter value (yy)
    /// * `sub_description` - Description text below xx
    /// * `total_description` - Description text below yy
    pub fn card_with_description(
        mut self,
        category: impl Into<String>,
        sub_counter: usize,
        total_counter: usize,
        sub_description: impl Into<String>,
        total_description: impl Into<String>,
    ) -> Self {
        self.cards.push(CounterCard {
            category: category.into(),
            sub_counter,
            total_counter,
            sub_description: Some(sub_description.into()),
            total_description: Some(total_description.into()),
            category_color: None,
            counter_color: None,
            description_color: None,
        });
        self
    }

    /// Add a counter card with custom colors.
    ///
    /// # Arguments
    /// * `category` - Category name
    /// * `sub_counter` - Current/sub counter value (xx)
    /// * `total_counter` - Total counter value (yy)
    /// * `category_color` - Color for category text
    /// * `counter_color` - Color for counter numbers
    /// * `description_color` - Color for description text
    pub fn card_colored(
        mut self,
        category: impl Into<String>,
        sub_counter: usize,
        total_counter: usize,
        category_color: egui::Color32,
        counter_color: egui::Color32,
        description_color: egui::Color32,
    ) -> Self {
        self.cards.push(CounterCard {
            category: category.into(),
            sub_counter,
            total_counter,
            sub_description: None,
            total_description: None,
            category_color: Some(category_color),
            counter_color: Some(counter_color),
            description_color: Some(description_color),
        });
        self
    }

    /// Add a counter card with descriptions and custom colors.
    ///
    /// # Arguments
    /// * `category` - Category name
    /// * `sub_counter` - Current/sub counter value (xx)
    /// * `total_counter` - Total counter value (yy)
    /// * `sub_description` - Description text below xx
    /// * `total_description` - Description text below yy
    /// * `category_color` - Color for category text
    /// * `counter_color` - Color for counter numbers
    /// * `description_color` - Color for description text
    pub fn card_with_description_colored(
        mut self,
        category: impl Into<String>,
        sub_counter: usize,
        total_counter: usize,
        sub_description: impl Into<String>,
        total_description: impl Into<String>,
        category_color: egui::Color32,
        counter_color: egui::Color32,
        description_color: egui::Color32,
    ) -> Self {
        self.cards.push(CounterCard {
            category: category.into(),
            sub_counter,
            total_counter,
            sub_description: Some(sub_description.into()),
            total_description: Some(total_description.into()),
            category_color: Some(category_color),
            counter_color: Some(counter_color),
            description_color: Some(description_color),
        });
        self
    }

    /// Set the width of each card (default: 150.0).
    pub fn card_width(mut self, width: f32) -> Self {
        self.card_width = width;
        self
    }

    /// Set the height of the widget (default: 120.0).
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the padding (default: 8.0).
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the corner radius for cards (default: 12.0).
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set an ID salt for unique widget identification.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Set a click callback that receives the card index.
    pub fn on_click(mut self, callback: impl FnMut(usize) + 'a) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }

    /// Set the color for category text (default: onSurfaceVariant).
    pub fn category_color(mut self, color: egui::Color32) -> Self {
        self.category_color = Some(color);
        self
    }

    /// Set the color for counter numbers (default: primary).
    pub fn counter_color(mut self, color: egui::Color32) -> Self {
        self.counter_color = Some(color);
        self
    }

    /// Set the color for description text (default: onSurfaceVariant).
    pub fn description_color(mut self, color: egui::Color32) -> Self {
        self.description_color = Some(color);
        self
    }

    /// Set a callback to add custom UI controls in the title area.
    /// The callback receives a mutable UI reference to add widgets.
    pub fn title_ui(mut self, callback: impl FnMut(&mut Ui) + 'a) -> Self {
        self.title_ui = Some(Box::new(callback));
        self
    }
}

impl<'a> egui::Widget for MaterialDashCounter<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let id_salt = self.id_salt.as_deref().unwrap_or("material_dashcounter");
        let _id = ui.make_persistent_id(id_salt);

        let available_width = ui.available_width();

        // Title height + card area height + bottom padding
        let title_height = 30.0;
        let desired_size = Vec2::new(available_width, title_height + self.height + self.padding * 2.0);

        let (outer_rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if !ui.is_rect_visible(outer_rect) {
            return response;
        }

        // Theme colors
        let on_surface = get_global_color("onSurface");
        let surface = get_global_color("surface");
        let outline = get_global_color("outline");

        // Use custom colors if provided, otherwise use theme defaults
        let category_color = self.category_color.unwrap_or_else(|| get_global_color("onSurfaceVariant"));
        let counter_color = self.counter_color.unwrap_or_else(|| get_global_color("primary"));
        let description_color = self.description_color.unwrap_or_else(|| get_global_color("onSurfaceVariant"));

        let painter = ui.painter_at(outer_rect);

        // Draw title area with optional custom UI
        let title_rect = Rect::from_min_size(
            Pos2::new(outer_rect.left() + self.padding, outer_rect.top() + self.padding),
            Vec2::new(available_width - self.padding * 2.0, title_height),
        );

        let mut title_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(title_rect)
                .layout(egui::Layout::left_to_right(egui::Align::BOTTOM))
        );
        title_ui.label(egui::RichText::new(&self.title).size(18.0).color(on_surface));

        // Call custom title UI callback if provided
        if let Some(mut callback) = self.title_ui {
            callback(&mut title_ui);
        }

        // Card area starts below title
        let cards_top = outer_rect.top() + title_height;
        let cards_rect = Rect::from_min_size(
            Pos2::new(outer_rect.left(), cards_top),
            Vec2::new(available_width, self.height + self.padding),
        );

        let card_count = self.cards.len();
        if card_count == 0 {
            return response;
        }

        // Calculate scrolling
        let card_step = self.card_width + self.padding;
        let total_content_width = card_step * card_count as f32;
        let max_scroll = (total_content_width - available_width).max(0.0);

        // Handle scroll input
        let mut scroll_delta = ui.input(|i| {
            let mut delta = 0.0;
            if let Some(hover_pos) = i.pointer.hover_pos() {
                if cards_rect.contains(hover_pos) {
                    delta -= i.smooth_scroll_delta.y;
                    delta -= i.smooth_scroll_delta.x;
                }
            }
            delta
        });

        // Handle dragging on the cards area
        let drag_response = ui.allocate_rect(cards_rect, Sense::click_and_drag());

        if drag_response.dragged() {
            scroll_delta -= drag_response.drag_delta().x;
        }

        *self.scroll_offset = (*self.scroll_offset + scroll_delta).clamp(0.0, max_scroll);
        let scroll = *self.scroll_offset;

        // Determine visible card range
        let first_visible = ((scroll / card_step).floor() as usize).min(card_count.saturating_sub(1));
        let last_visible = (((scroll + available_width) / card_step).ceil() as usize).min(card_count);

        // Draw cards
        for i in first_visible..last_visible {
            let card = &self.cards[i];
            let card_left = i as f32 * card_step + self.padding - scroll;
            let screen_x = outer_rect.left() + card_left;

            let card_rect = Rect::from_min_size(
                Pos2::new(screen_x, cards_top + self.padding),
                Vec2::new(self.card_width, self.height),
            );

            // Clip to visible area
            let clipped_rect = card_rect.intersect(cards_rect);
            if clipped_rect.width() <= 0.0 || clipped_rect.height() <= 0.0 {
                continue;
            }

            let rounding = CornerRadius::same(self.corner_radius as u8);

            // Draw card background
            painter.rect_filled(card_rect, rounding, surface);
            painter.rect_stroke(
                card_rect,
                rounding,
                egui::Stroke::new(1.0, outline),
                egui::epaint::StrokeKind::Outside,
            );

            // Card content layout - vertically centered
            let content_padding = 12.0;

            // Check if this card has any descriptions
            let has_descriptions = card.sub_description.is_some() || card.total_description.is_some();

            // Determine colors for this card: card-specific > control-level > theme default
            let card_category_color = card.category_color.unwrap_or(category_color);
            let card_counter_color = card.counter_color.unwrap_or(counter_color);
            let card_description_color = card.description_color.unwrap_or(description_color);

            // Calculate total content height for vertical centering
            let category_height = 15.0;  // Approximate height for 12pt font
            let gap_after_category = 10.0;  // Space between category and counter
            let counter_height = 32.0;  // Approximate height for 32pt font
            let desc_gap = 3.0;  // Gap before descriptions
            let desc_height = 12.0;  // Approximate height for 10pt font

            let total_content_height = if has_descriptions {
                category_height + gap_after_category + counter_height + desc_gap + desc_height
            } else {
                category_height + gap_after_category + counter_height
            };

            // Center content vertically within card
            let content_start_y = card_rect.center().y - total_content_height / 2.0;

            // Position elements from top to bottom, starting from centered position
            let category_y = content_start_y;
            // counter_y is now the BOTTOM position since we use BOTTOM alignment
            let counter_y = category_y + category_height + gap_after_category + counter_height;

            // Draw category name
            painter.text(
                Pos2::new(card_rect.center().x, category_y),
                egui::Align2::CENTER_TOP,
                &card.category,
                FontId::proportional(12.0),
                card_category_color,
            );

            // Draw counter with xx in larger text and yy in smaller text
            let sub_text = format!("{}", card.sub_counter);
            let total_text = format!("/{}", card.total_counter);

            // Measure text widths to position them correctly
            let sub_font = FontId::proportional(32.0);
            let total_font = FontId::proportional(20.0);
            let desc_font = FontId::proportional(10.0);

            let sub_galley = painter.layout_no_wrap(sub_text.clone(), sub_font.clone(), card_counter_color);
            let total_galley = painter.layout_no_wrap(total_text.clone(), total_font.clone(), card_counter_color);

            let total_width = sub_galley.rect.width() + total_galley.rect.width();
            let start_x = card_rect.center().x - total_width / 2.0;

            // Position for xx
            let sub_x = start_x + sub_galley.rect.width() / 2.0;
            // Position for /yy
            let total_x = start_x + sub_galley.rect.width() + total_galley.rect.width() / 2.0;

            // Draw larger xx (using BOTTOM alignment for baseline alignment)
            painter.text(
                Pos2::new(sub_x, counter_y),
                egui::Align2::CENTER_BOTTOM,
                sub_text,
                sub_font,
                card_counter_color,
            );

            // Draw smaller /yy (bottom-aligned with larger number for proper baseline)
            painter.text(
                Pos2::new(total_x, counter_y),
                egui::Align2::CENTER_BOTTOM,
                total_text,
                total_font,
                card_counter_color,
            );

            // Draw descriptions below their respective numbers (only if they exist)
            if has_descriptions {
                let desc_y = counter_y + desc_gap;  // Position below the counter (counter_y is now bottom of counter)
                let desc_padding = 4.0;  // Small padding between the two descriptions

                // If both descriptions exist, center them as a pair
                if card.sub_description.is_some() && card.total_description.is_some() {
                    let sub_desc = card.sub_description.as_ref().unwrap();
                    let total_desc = card.total_description.as_ref().unwrap();

                    // Measure description text widths
                    let sub_desc_galley = painter.layout_no_wrap(sub_desc.clone(), desc_font.clone(), card_description_color);
                    let total_desc_galley = painter.layout_no_wrap(total_desc.clone(), desc_font.clone(), card_description_color);

                    // Calculate total width and center position
                    let total_desc_width = sub_desc_galley.rect.width() + desc_padding + total_desc_galley.rect.width();
                    let desc_start_x = card_rect.center().x - total_desc_width / 2.0;

                    // Draw left description (extends rightward from start)
                    painter.text(
                        Pos2::new(desc_start_x, desc_y),
                        egui::Align2::LEFT_TOP,
                        sub_desc,
                        desc_font.clone(),
                        card_description_color,
                    );

                    // Draw right description (extends rightward after padding)
                    painter.text(
                        Pos2::new(desc_start_x + sub_desc_galley.rect.width() + desc_padding, desc_y),
                        egui::Align2::LEFT_TOP,
                        total_desc,
                        desc_font.clone(),
                        card_description_color,
                    );
                } else {
                    // If only one description exists, center it individually
                    if let Some(ref sub_desc) = card.sub_description {
                        painter.text(
                            Pos2::new(card_rect.center().x, desc_y),
                            egui::Align2::CENTER_TOP,
                            sub_desc,
                            desc_font.clone(),
                            card_description_color,
                        );
                    }

                    if let Some(ref total_desc) = card.total_description {
                        painter.text(
                            Pos2::new(card_rect.center().x, desc_y),
                            egui::Align2::CENTER_TOP,
                            total_desc,
                            desc_font,
                            card_description_color,
                        );
                    }
                }
            }

            // Check for click on counter (expanded area to cover both parts)
            let counter_rect = Rect::from_center_size(
                Pos2::new(card_rect.center().x, counter_y),
                Vec2::new(self.card_width - content_padding * 2.0, 50.0),
            );

            let counter_sense = ui.interact(
                counter_rect,
                ui.make_persistent_id(format!("{}_{}", id_salt, i)),
                Sense::click(),
            );

            if counter_sense.clicked() {
                if let Some(callback) = &mut self.on_click {
                    callback(i);
                }
                response.mark_changed();
            }

            // Visual feedback on hover
            if counter_sense.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        response
    }
}

/// Create a new dashboard counter widget.
///
/// # Arguments
/// * `title` - Title text to display at the top
/// * `scroll_offset` - Mutable reference to persistent scroll state
///
/// # Example
/// ```rust,no_run
/// # egui::__run_test_ui(|ui| {
/// let mut offset = 0.0f32;
/// ui.add(dashcounter("Stats", &mut offset)
///     .card("Active", 3, 10)
///     .card("Pending", 7, 15));
/// # });
/// ```
pub fn dashcounter<'a>(title: impl Into<String>, scroll_offset: &'a mut f32) -> MaterialDashCounter<'a> {
    MaterialDashCounter::new(title, scroll_offset)
}
