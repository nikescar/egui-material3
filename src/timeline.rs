use crate::get_global_color;
use egui::{self, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Position where timeline content appears relative to the timeline axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelinePosition {
    /// Content appears on the left side of the timeline
    Left,
    /// Content appears on the right side of the timeline
    Right,
    /// Content alternates between left and right sides
    Alternate,
    /// Content alternates between right and left sides (starts on right)
    AlternateReverse,
}

impl Default for TimelinePosition {
    fn default() -> Self {
        Self::Right
    }
}

/// Variant for timeline dot appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineDotVariant {
    /// Filled solid dot
    Filled,
    /// Outlined dot with border
    Outlined,
}

/// Color scheme for timeline dot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineDotColor {
    /// Grey color (default)
    Grey,
    /// Inherit color from context
    Inherit,
    /// Primary theme color
    Primary,
    /// Secondary theme color
    Secondary,
    /// Error/danger color
    Error,
    /// Info color
    Info,
    /// Success color
    Success,
    /// Warning color
    Warning,
}

impl Default for TimelineDotColor {
    fn default() -> Self {
        Self::Grey
    }
}

impl Default for TimelineDotVariant {
    fn default() -> Self {
        Self::Filled
    }
}

/// Material Design timeline component.
///
/// Timelines display a list of events in chronological order.
/// They can be used to show a sequence of events, process steps, or historical data.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(MaterialTimeline::new()
///     .position(TimelinePosition::Right)
///     .item(TimelineItem::new()
///         .content("First event")
///         .dot(TimelineDot::new()
///             .color(TimelineDotColor::Primary)))
///     .item(TimelineItem::new()
///         .content("Second event")
///         .dot(TimelineDot::new()
///             .color(TimelineDotColor::Success))));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialTimeline<'a> {
    /// Position of content relative to timeline
    position: TimelinePosition,
    /// List of timeline items
    items: Vec<TimelineItem<'a>>,
    /// Optional unique ID for this timeline
    id: Option<egui::Id>,
}

/// Individual item in a timeline.
pub struct TimelineItem<'a> {
    /// Main content text
    content: Option<String>,
    /// Custom content renderer (takes precedence over content text)
    content_custom: Option<Box<dyn Fn(&mut Ui) + 'a>>,
    /// Optional opposite side content
    opposite_content: Option<String>,
    /// Timeline dot configuration
    dot: Option<TimelineDot>,
    /// Whether to show connector line below this item
    show_connector: bool,
    /// Optional callback when item is clicked
    action: Option<Box<dyn Fn() + 'a>>,
    /// Optional custom content color
    content_color: Option<Color32>,
    /// Optional custom opposite content color
    opposite_content_color: Option<Color32>,
    /// Custom min height for this item (useful for cards/complex content)
    min_height: Option<f32>,
}

/// Timeline dot/indicator configuration.
pub struct TimelineDot {
    /// Visual variant (filled or outlined)
    variant: TimelineDotVariant,
    /// Color scheme
    color: TimelineDotColor,
    /// Optional icon text to display in the dot
    icon: Option<String>,
    /// Optional custom color
    custom_color: Option<Color32>,
    /// Optional custom dot size (defaults to DOT_SIZE constant)
    size: Option<f32>,
}

impl<'a> MaterialTimeline<'a> {
    /// Create a new timeline with default right-aligned position.
    ///
    /// # Example
    /// ```rust
    /// let timeline = MaterialTimeline::new();
    /// ```
    pub fn new() -> Self {
        Self {
            position: TimelinePosition::default(),
            items: Vec::new(),
            id: None,
        }
    }

    /// Set the position where content appears relative to the timeline axis.
    ///
    /// # Arguments
    /// * `position` - The position (Left, Right, Alternate, or AlternateReverse)
    ///
    /// # Example
    /// ```rust
    /// let timeline = MaterialTimeline::new()
    ///     .position(TimelinePosition::Alternate);
    /// ```
    pub fn position(mut self, position: TimelinePosition) -> Self {
        self.position = position;
        self
    }

    /// Add an item to the timeline.
    ///
    /// # Arguments
    /// * `item` - The timeline item to add
    ///
    /// # Example
    /// ```rust
    /// let timeline = MaterialTimeline::new()
    ///     .item(TimelineItem::new().content("Event"));
    /// ```
    pub fn item(mut self, item: TimelineItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    /// Set a unique ID for this timeline to avoid widget ID collisions.
    ///
    /// # Arguments
    /// * `id` - Unique identifier
    ///
    /// # Example
    /// ```rust
    /// let timeline = MaterialTimeline::new()
    ///     .id(egui::Id::new("my_timeline"));
    /// ```
    pub fn id(mut self, id: egui::Id) -> Self {
        self.id = Some(id);
        self
    }
}

impl<'a> TimelineItem<'a> {
    /// Create a new timeline item.
    ///
    /// # Example
    /// ```rust
    /// let item = TimelineItem::new();
    /// ```
    pub fn new() -> Self {
        Self {
            content: None,
            content_custom: None,
            opposite_content: None,
            dot: None,
            show_connector: true,
            action: None,
            content_color: None,
            opposite_content_color: None,
            min_height: None,
        }
    }

    /// Set the main content text.
    ///
    /// # Arguments
    /// * `text` - Content text to display
    ///
    /// # Example
    /// ```rust
    /// let item = TimelineItem::new()
    ///     .content("Event description");
    /// ```
    pub fn content(mut self, text: impl Into<String>) -> Self {
        self.content = Some(text.into());
        self
    }

    /// Set custom content renderer with a closure.
    ///
    /// This takes precedence over the text-based `content()` method.
    ///
    /// # Arguments
    /// * `render` - Closure that renders custom UI
    ///
    /// # Example
    /// ```rust
    /// let item = TimelineItem::new()
    ///     .content_custom(|ui| {
    ///         ui.label("Custom content");
    ///         ui.button("Click me");
    ///     });
    /// ```
    pub fn content_custom<F: Fn(&mut Ui) + 'a>(mut self, render: F) -> Self {
        self.content_custom = Some(Box::new(render));
        self
    }

    /// Set minimum height for this timeline item.
    ///
    /// Useful when using custom content that needs more vertical space.
    ///
    /// # Arguments
    /// * `height` - Minimum height in pixels
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    /// Set the opposite side content text.
    ///
    /// This appears on the opposite side of the timeline axis from the main content.
    ///
    /// # Arguments
    /// * `text` - Opposite content text to display
    ///
    /// # Example
    /// ```rust
    /// let item = TimelineItem::new()
    ///     .content("Event description")
    ///     .opposite_content("09:30 am");
    /// ```
    pub fn opposite_content(mut self, text: impl Into<String>) -> Self {
        self.opposite_content = Some(text.into());
        self
    }

    /// Set the timeline dot configuration.
    ///
    /// # Arguments
    /// * `dot` - TimelineDot configuration
    ///
    /// # Example
    /// ```rust
    /// let item = TimelineItem::new()
    ///     .content("Event")
    ///     .dot(TimelineDot::new()
    ///         .color(TimelineDotColor::Primary));
    /// ```
    pub fn dot(mut self, dot: TimelineDot) -> Self {
        self.dot = Some(dot);
        self
    }

    /// Set whether to show the connector line below this item.
    ///
    /// # Arguments
    /// * `show` - true to show connector, false to hide
    ///
    /// # Example
    /// ```rust
    /// let item = TimelineItem::new()
    ///     .content("Final event")
    ///     .show_connector(false); // Last item doesn't need connector
    /// ```
    pub fn show_connector(mut self, show: bool) -> Self {
        self.show_connector = show;
        self
    }

    /// Set a callback to execute when this item is clicked.
    ///
    /// # Arguments
    /// * `action` - Callback function
    ///
    /// # Example
    /// ```rust
    /// let item = TimelineItem::new()
    ///     .content("Clickable event")
    ///     .on_click(|| println!("Item clicked"));
    /// ```
    pub fn on_click<F: Fn() + 'a>(mut self, action: F) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    /// Set custom color for the main content.
    ///
    /// # Arguments
    /// * `color` - Custom color
    pub fn content_color(mut self, color: Color32) -> Self {
        self.content_color = Some(color);
        self
    }

    /// Set custom color for the opposite content.
    ///
    /// # Arguments
    /// * `color` - Custom color
    pub fn opposite_content_color(mut self, color: Color32) -> Self {
        self.opposite_content_color = Some(color);
        self
    }
}

impl TimelineDot {
    /// Create a new timeline dot with default settings.
    ///
    /// # Example
    /// ```rust
    /// let dot = TimelineDot::new();
    /// ```
    pub fn new() -> Self {
        Self {
            variant: TimelineDotVariant::default(),
            color: TimelineDotColor::default(),
            icon: None,
            custom_color: None,
            size: None,
        }
    }

    /// Set the visual variant (filled or outlined).
    ///
    /// # Arguments
    /// * `variant` - Dot variant
    ///
    /// # Example
    /// ```rust
    /// let dot = TimelineDot::new()
    ///     .variant(TimelineDotVariant::Outlined);
    /// ```
    pub fn variant(mut self, variant: TimelineDotVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the color scheme.
    ///
    /// # Arguments
    /// * `color` - Color scheme
    ///
    /// # Example
    /// ```rust
    /// let dot = TimelineDot::new()
    ///     .color(TimelineDotColor::Primary);
    /// ```
    pub fn color(mut self, color: TimelineDotColor) -> Self {
        self.color = color;
        self
    }

    /// Set an icon to display in the dot.
    ///
    /// # Arguments
    /// * `icon` - Icon text (emoji or character)
    ///
    /// # Example
    /// ```rust
    /// let dot = TimelineDot::new()
    ///     .icon("✓");
    /// ```
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a custom color for the dot.
    ///
    /// # Arguments
    /// * `color` - Custom color
    pub fn custom_color(mut self, color: Color32) -> Self {
        self.custom_color = Some(color);
        self
    }

    /// Set a custom size for the dot.
    ///
    /// # Arguments
    /// * `size` - Dot diameter in pixels
    ///
    /// # Example
    /// ```rust
    /// let dot = TimelineDot::new()
    ///     .size(40.0)  // Large dot
    ///     .icon("🚀");
    /// ```
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

impl Default for TimelineDot {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Default for TimelineItem<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Default for MaterialTimeline<'a> {
    fn default() -> Self {
        Self::new()
    }
}

// Constants for Material Design 3 timeline styling
const DOT_SIZE: f32 = 12.0;
const DOT_ICON_SIZE: f32 = 16.0;
const CONNECTOR_WIDTH: f32 = 2.0;
const CONTENT_PADDING: f32 = 32.0;  // Increased padding to prevent icon overlap with text
const MIN_ITEM_SPACING: f32 = 24.0;  // Minimum spacing between items
const OPPOSITE_CONTENT_WIDTH: f32 = 80.0;

impl<'a> Widget for MaterialTimeline<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let base_id = self.id.unwrap_or_else(|| ui.make_persistent_id("timeline"));

        let mut total_height = 0.0;
        let item_count = self.items.len();

        // Calculate total height needed based on actual dot sizes
        for (index, item) in self.items.iter_mut().enumerate() {
            let dot_size = item.dot.as_ref().and_then(|d| d.size).unwrap_or(DOT_SIZE);
            // Spacing should be at least dot_size + padding, or MIN_ITEM_SPACING, whichever is larger
            let item_spacing = (dot_size + CONTENT_PADDING).max(MIN_ITEM_SPACING);
            total_height += item_spacing;

            if index == item_count - 1 {
                // Last item doesn't need connector
                item.show_connector = false;
            }
        }

        let available_width = ui.available_width();
        let desired_size = Vec2::new(available_width, total_height.max(50.0));
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Material Design 3 colors
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let mut current_y = rect.min.y;

        // Check if we're in alternate mode - if so, center the separator
        let is_alternate_mode = matches!(self.position, TimelinePosition::Alternate | TimelinePosition::AlternateReverse);

        for (index, item) in self.items.iter().enumerate() {
            // Determine position for this item
            let item_position = match self.position {
                TimelinePosition::Left => TimelinePosition::Left,
                TimelinePosition::Right => TimelinePosition::Right,
                TimelinePosition::Alternate => {
                    if index % 2 == 0 {
                        TimelinePosition::Right
                    } else {
                        TimelinePosition::Left
                    }
                }
                TimelinePosition::AlternateReverse => {
                    if index % 2 == 0 {
                        TimelinePosition::Left
                    } else {
                        TimelinePosition::Right
                    }
                }
            };

            let has_opposite = item.opposite_content.is_some();

            // Calculate dot size and spacing early for use throughout
            let dot_config = item.dot.as_ref();
            let dot_size = dot_config.and_then(|d| d.size).unwrap_or(DOT_SIZE);
            // Icon size should be smaller than dot for better fit
            let icon_size = (dot_size * 0.7).max(10.0);
            // Use custom min_height if provided, otherwise calculate from dot size
            let base_spacing = (dot_size + CONTENT_PADDING).max(MIN_ITEM_SPACING);
            let item_spacing = item.min_height.unwrap_or(base_spacing).max(base_spacing);

            // Calculate layout positions
            let (opposite_x, separator_x, content_x, is_content_right) = if is_alternate_mode {
                // For alternate mode, center the separator
                let center_x = rect.center().x;

                match item_position {
                    TimelinePosition::Right => {
                        if has_opposite {
                            (
                                center_x - OPPOSITE_CONTENT_WIDTH - CONTENT_PADDING - DOT_SIZE / 2.0,
                                center_x,
                                center_x + DOT_SIZE / 2.0 + CONTENT_PADDING,
                                true,
                            )
                        } else {
                            (
                                center_x - DOT_SIZE / 2.0,
                                center_x,
                                center_x + DOT_SIZE / 2.0 + CONTENT_PADDING,
                                true,
                            )
                        }
                    }
                    TimelinePosition::Left => {
                        // For left-positioned items in alternate mode, calculate content_x
                        // so the content rect ends near the separator
                        let half_width = available_width / 2.0;
                        let left_content_width = if has_opposite {
                            half_width - OPPOSITE_CONTENT_WIDTH - CONTENT_PADDING * 2.0 - DOT_SIZE / 2.0
                        } else {
                            half_width - CONTENT_PADDING - DOT_SIZE / 2.0
                        };
                        let content_start_x = center_x - DOT_SIZE / 2.0 - CONTENT_PADDING - left_content_width;

                        if has_opposite {
                            (
                                center_x + DOT_SIZE / 2.0 + CONTENT_PADDING,
                                center_x,
                                content_start_x,
                                false,
                            )
                        } else {
                            (
                                center_x + DOT_SIZE / 2.0,
                                center_x,
                                content_start_x,
                                false,
                            )
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                // For non-alternate mode (Left or Right), use edge-based layout
                match item_position {
                    TimelinePosition::Right => {
                        if has_opposite {
                            (
                                rect.min.x,
                                rect.min.x + OPPOSITE_CONTENT_WIDTH + CONTENT_PADDING,
                                rect.min.x + OPPOSITE_CONTENT_WIDTH + CONTENT_PADDING * 2.0 + DOT_SIZE,
                                true,
                            )
                        } else {
                            (rect.min.x, rect.min.x, rect.min.x + DOT_SIZE + CONTENT_PADDING, true)
                        }
                    }
                    TimelinePosition::Left => {
                        if has_opposite {
                            (
                                rect.max.x - OPPOSITE_CONTENT_WIDTH,
                                rect.max.x - OPPOSITE_CONTENT_WIDTH - CONTENT_PADDING - DOT_SIZE,
                                rect.min.x,
                                false,
                            )
                        } else {
                            (rect.max.x, rect.max.x, rect.min.x, false)
                        }
                    }
                    _ => unreachable!(),
                }
            };

            // Draw opposite content (e.g., timestamp)
            if let Some(opposite_text) = &item.opposite_content {
                let opposite_color = item.opposite_content_color.unwrap_or(on_surface_variant);

                let opposite_width = if has_opposite {
                    OPPOSITE_CONTENT_WIDTH
                } else {
                    100.0
                };

                // Calculate spacing for vertical centering
                let item_spacing = (dot_size + CONTENT_PADDING).max(MIN_ITEM_SPACING);

                // Use allocate_ui_at_rect for proper text rendering with unique ID
                // Rect spans full item height for proper vertical centering
                let opposite_rect = Rect::from_min_size(
                    Pos2::new(opposite_x, current_y),
                    Vec2::new(opposite_width, item_spacing),
                );

                ui.allocate_ui_at_rect(opposite_rect, |ui| {
                    // Properly clip to both the rect and parent's clip rect
                    let parent_clip = ui.clip_rect();
                    let clipped = opposite_rect.intersect(parent_clip);
                    ui.set_clip_rect(clipped);

                    // Right-align when content is on the left (opposite on right)
                    // Use Center for vertical alignment with dot
                    let layout = if is_content_right {
                        egui::Layout::left_to_right(egui::Align::Center)
                    } else {
                        egui::Layout::right_to_left(egui::Align::Center)
                    };
                    ui.with_layout(layout, |ui| {
                        let label = egui::Label::new(
                            egui::RichText::new(opposite_text)
                                .size(14.0)
                                .color(opposite_color)
                        ).wrap_mode(egui::TextWrapMode::Truncate);
                        ui.add(label);
                    });
                }).response.context_menu(|_ui| {});  // Add context menu to force unique ID
            }

            // Draw dot
            let dot_center = Pos2::new(separator_x, current_y + dot_size / 2.0);

            let dot_color = if let Some(dot) = dot_config {
                if let Some(custom) = dot.custom_color {
                    custom
                } else {
                    match dot.color {
                        TimelineDotColor::Grey => get_global_color("outline"),
                        TimelineDotColor::Inherit => on_surface,
                        TimelineDotColor::Primary => get_global_color("primary"),
                        TimelineDotColor::Secondary => get_global_color("secondary"),
                        TimelineDotColor::Error => get_global_color("error"),
                        TimelineDotColor::Info => get_global_color("tertiary"),
                        TimelineDotColor::Success => Color32::from_rgb(76, 175, 80),
                        TimelineDotColor::Warning => Color32::from_rgb(255, 152, 0),
                    }
                }
            } else {
                outline
            };

            // Draw dot based on variant
            if let Some(dot) = dot_config {
                match dot.variant {
                    TimelineDotVariant::Filled => {
                        ui.painter().circle_filled(dot_center, dot_size / 2.0, dot_color);

                        // Draw icon if present - use allocate_at_rect for unique ID
                        if let Some(icon_text) = &dot.icon {
                            let icon_color = if dot_color.r() as u32 + dot_color.g() as u32 + dot_color.b() as u32 > 384 {
                                Color32::BLACK
                            } else {
                                Color32::WHITE
                            };
                            let icon_rect = Rect::from_center_size(dot_center, Vec2::splat(icon_size));
                            ui.allocate_ui_at_rect(icon_rect, |ui| {
                                // Clip icon to parent's clip rect
                                let parent_clip = ui.clip_rect();
                                let clipped = icon_rect.intersect(parent_clip);
                                ui.set_clip_rect(clipped);

                                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                    let label = egui::Label::new(
                                        egui::RichText::new(icon_text)
                                            .size(icon_size)
                                            .color(icon_color)
                                    );
                                    ui.add(label);
                                });
                            });
                        }
                    }
                    TimelineDotVariant::Outlined => {
                        let stroke_width = (dot_size / 6.0).max(2.0); // Scale stroke with dot size
                        ui.painter().circle_stroke(
                            dot_center,
                            dot_size / 2.0,
                            Stroke::new(stroke_width, dot_color),
                        );

                        // Draw icon if present - use allocate_at_rect for unique ID
                        if let Some(icon_text) = &dot.icon {
                            let icon_rect = Rect::from_center_size(dot_center, Vec2::splat(icon_size));
                            ui.allocate_ui_at_rect(icon_rect, |ui| {
                                // Clip icon to parent's clip rect
                                let parent_clip = ui.clip_rect();
                                let clipped = icon_rect.intersect(parent_clip);
                                ui.set_clip_rect(clipped);

                                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                    let label = egui::Label::new(
                                        egui::RichText::new(icon_text)
                                            .size(icon_size)
                                            .color(dot_color)
                                    );
                                    ui.add(label);
                                });
                            });
                        }
                    }
                }
            } else {
                // Default dot
                ui.painter().circle_filled(dot_center, dot_size / 2.0, dot_color);
            }

            // Draw connector line if not the last item
            if item.show_connector {
                let connector_start = Pos2::new(separator_x, current_y + dot_size);
                let connector_end = Pos2::new(separator_x, current_y + item_spacing);
                ui.painter().line_segment(
                    [connector_start, connector_end],
                    Stroke::new(CONNECTOR_WIDTH, outline),
                );
            }

            // Draw content (custom or text-based)
            if item.content_custom.is_some() || item.content.is_some() {
                let content_color = item.content_color.unwrap_or(on_surface);
                let content_width = if is_alternate_mode {
                    // For alternate mode, content takes up half the width minus padding and dot
                    let half_width = available_width / 2.0;
                    if has_opposite {
                        half_width - OPPOSITE_CONTENT_WIDTH - CONTENT_PADDING * 2.0 - DOT_SIZE / 2.0
                    } else {
                        half_width - CONTENT_PADDING - DOT_SIZE / 2.0
                    }
                } else {
                    // For Left/Right mode, content can use most of the width
                    if has_opposite {
                        available_width - OPPOSITE_CONTENT_WIDTH - CONTENT_PADDING * 3.0 - DOT_SIZE
                    } else {
                        available_width - DOT_SIZE - CONTENT_PADDING * 2.0
                    }
                };

                // Use full item height for proper vertical centering with dot
                let content_rect = Rect::from_min_size(
                    Pos2::new(content_x, current_y),
                    Vec2::new(content_width, item_spacing),
                );

                // Use allocate_ui_at_rect for proper rendering with interaction
                let content_inner = ui.allocate_ui_at_rect(content_rect, |ui| {
                    // Properly clip to both the rect and parent's clip rect
                    let parent_clip = ui.clip_rect();
                    let clipped = content_rect.intersect(parent_clip);
                    ui.set_clip_rect(clipped);

                    let has_action = item.action.is_some();
                    let item_id = base_id.with(("content", index));
                    let sense = if has_action { Sense::click() } else { Sense::hover() };
                    let interact_response = ui.interact(content_rect, item_id, sense);

                    // Draw hover effect
                    if interact_response.hovered() && has_action {
                        let hover_color = Color32::from_rgba_unmultiplied(
                            on_surface.r(),
                            on_surface.g(),
                            on_surface.b(),
                            10,
                        );
                        ui.painter().rect_filled(content_rect, 4.0, hover_color);
                    }

                    // Render custom content or text label
                    if let Some(custom_render) = &item.content_custom {
                        // Custom content rendering - use vertical layout for cards/complex content
                        let align = if is_content_right {
                            egui::Align::LEFT
                        } else {
                            egui::Align::RIGHT
                        };
                        let layout = egui::Layout::top_down(align);

                        ui.with_layout(layout, |ui| {
                            custom_render(ui);
                        });
                    } else if let Some(content_text) = &item.content {
                        // Text-based content rendering - use center alignment
                        let layout = if is_content_right {
                            egui::Layout::left_to_right(egui::Align::Center)
                        } else {
                            egui::Layout::right_to_left(egui::Align::Center)
                        };

                        ui.with_layout(layout, |ui| {
                            let label = egui::Label::new(
                                egui::RichText::new(content_text)
                                    .size(16.0)
                                    .color(content_color)
                            ).wrap_mode(egui::TextWrapMode::Wrap);
                            ui.add(label);
                        });
                    }

                    (interact_response, has_action)
                });

                // Handle click
                if content_inner.inner.0.clicked() && content_inner.inner.1 {
                    if let Some(action) = &item.action {
                        action();
                    }
                }
            }

            current_y += item_spacing;
        }

        response
    }
}

/// Convenience function to create a timeline.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(timeline()
///     .item(TimelineItem::new().content("Event 1"))
///     .item(TimelineItem::new().content("Event 2")));
/// # });
/// ```
pub fn timeline<'a>() -> MaterialTimeline<'a> {
    MaterialTimeline::new()
}
