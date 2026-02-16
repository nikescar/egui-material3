use crate::icons::icon_text;
use crate::theme::get_global_color;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Defines the title font used for ListTile descendants.
///
/// List tiles that appear in a drawer use a smaller text style,
/// while standard list tiles use the default title text style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListTileStyle {
    /// Use a title font appropriate for a list tile in a list.
    List,
    /// Use a title font appropriate for a list tile in a drawer.
    Drawer,
}

/// Defines how leading and trailing widgets are vertically aligned
/// relative to the list tile's titles (title and subtitle).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListTileTitleAlignment {
    /// The top of leading/trailing widgets are placed below the title top
    /// if three-line, otherwise centered relative to title and subtitle.
    /// This is the default for Material 3.
    ThreeLine,
    /// Leading/trailing are placed 16px below title top if tile height > 72,
    /// otherwise centered. This is the default for Material 2.
    TitleHeight,
    /// Leading/trailing tops are placed at min vertical padding below title top.
    Top,
    /// Leading/trailing are centered relative to the titles.
    Center,
    /// Leading/trailing bottoms are placed at min vertical padding above title bottom.
    Bottom,
}

/// Defines the visual density for the list tile layout.
///
/// Visual density allows for compact, comfortable, or spacious layouts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualDensity {
    /// Horizontal density adjustment (-4.0 to 4.0)
    pub horizontal: f32,
    /// Vertical density adjustment (-4.0 to 4.0)
    pub vertical: f32,
}

impl VisualDensity {
    /// Standard density (no adjustment)
    pub const STANDARD: Self = Self {
        horizontal: 0.0,
        vertical: 0.0,
    };

    /// Comfortable density (slightly more spacious)
    pub const COMFORTABLE: Self = Self {
        horizontal: -1.0,
        vertical: -1.0,
    };

    /// Compact density (space-efficient)
    pub const COMPACT: Self = Self {
        horizontal: -2.0,
        vertical: -2.0,
    };

    /// Create a custom visual density
    pub fn new(horizontal: f32, vertical: f32) -> Self {
        Self {
            horizontal: horizontal.clamp(-4.0, 4.0),
            vertical: vertical.clamp(-4.0, 4.0),
        }
    }

    /// Get the base size adjustment as a Vec2
    pub fn base_size_adjustment(&self) -> Vec2 {
        Vec2::new(self.horizontal * 4.0, self.vertical * 4.0)
    }
}

impl Default for VisualDensity {
    fn default() -> Self {
        Self::STANDARD
    }
}

/// Material Design list component.
///
/// Lists are continuous, vertical indexes of text or images.
/// They are composed of items containing primary and related actions.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let list = MaterialList::new()
///     .item(ListItem::new("Inbox")
///         .leading_icon("inbox")
///         .trailing_text("12"))
///     .item(ListItem::new("Starred")
///         .leading_icon("star")
///         .trailing_text("3"))
///     .dividers(true);
///
/// ui.add(list);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialList<'a> {
    /// List of items to display
    items: Vec<ListItem<'a>>,
    /// Whether to show dividers between items
    dividers: bool,
    /// Optional unique ID for this list to avoid widget ID collisions
    id: Option<egui::Id>,
}

/// Individual item in a Material Design list.
///
/// List items can contain primary text, secondary text, overline text,
/// leading and trailing icons, and custom actions.
///
/// # Example
/// ```rust
/// let item = ListItem::new("Primary Text")
///     .secondary_text("Secondary supporting text")
///     .leading_icon("person")
///     .trailing_icon("more_vert")
///     .on_click(|| println!("Item clicked"));
/// ```
pub struct ListItem<'a> {
    /// Main text displayed for this item
    primary_text: String,
    /// Optional secondary text displayed below primary text
    secondary_text: Option<String>,
    /// Optional overline text displayed above primary text
    overline_text: Option<String>,
    /// Optional icon displayed at the start of the item
    leading_icon: Option<String>,
    /// Optional icon displayed at the end of the item
    trailing_icon: Option<String>,
    /// Optional text displayed at the end of the item
    trailing_text: Option<String>,
    /// Whether the item is enabled and interactive
    enabled: bool,
    /// Whether the item is selected
    selected: bool,
    /// Whether this list tile is part of a vertically dense list
    dense: Option<bool>,
    /// Whether this list tile is intended to display three lines of text
    is_three_line: Option<bool>,
    /// Defines how compact the list tile's layout will be
    visual_density: Option<VisualDensity>,
    /// Defines the font used for the title
    style: Option<ListTileStyle>,
    /// Defines how leading and trailing are vertically aligned
    title_alignment: Option<ListTileTitleAlignment>,
    /// The horizontal gap between the titles and the leading/trailing widgets
    horizontal_title_gap: Option<f32>,
    /// The minimum padding on the top and bottom of the title and subtitle widgets
    min_vertical_padding: Option<f32>,
    /// The minimum width allocated for the leading widget
    min_leading_width: Option<f32>,
    /// The minimum height allocated for the list tile widget
    min_tile_height: Option<f32>,
    /// Background color when selected is false
    tile_color: Option<Color32>,
    /// Background color when selected is true
    selected_tile_color: Option<Color32>,
    /// Color for icons and text when selected
    selected_color: Option<Color32>,
    /// Default color for leading and trailing icons
    icon_color: Option<Color32>,
    /// Text color for title, subtitle, leading, and trailing
    text_color: Option<Color32>,
    /// Callback function to execute when the item is clicked
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialList<'a> {
    /// Create a new empty list.
    ///
    /// # Example
    /// ```rust
    /// let list = MaterialList::new();
    /// ```
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            dividers: true,
            id: None,
        }
    }

    /// Add an item to the list.
    ///
    /// # Arguments
    /// * `item` - The list item to add
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let item = ListItem::new("Sample Item");
    /// let list = MaterialList::new().item(item);
    /// # });
    /// ```
    pub fn item(mut self, item: ListItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    /// Set whether to show dividers between items.
    ///
    /// # Arguments
    /// * `dividers` - Whether to show divider lines between items
    ///
    /// # Example
    /// ```rust
    /// let list = MaterialList::new().dividers(false); // No dividers
    /// ```
    pub fn dividers(mut self, dividers: bool) -> Self {
        self.dividers = dividers;
        self
    }

    /// Set a custom ID for this list to avoid widget ID collisions.
    ///
    /// Use this when you have multiple lists with similar content in the same UI.
    ///
    /// # Arguments
    /// * `id` - A unique identifier for this list
    ///
    /// # Example
    /// ```rust
    /// let list = MaterialList::new()
    ///     .id(egui::Id::new("my_list"))
    ///     .dividers(false);
    /// ```
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a> ListItem<'a> {
    /// Create a new list item with primary text.
    ///
    /// # Arguments
    /// * `primary_text` - The main text to display
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("My List Item");
    /// ```
    pub fn new(primary_text: impl Into<String>) -> Self {
        Self {
            primary_text: primary_text.into(),
            secondary_text: None,
            overline_text: None,
            leading_icon: None,
            trailing_icon: None,
            trailing_text: None,
            enabled: true,
            selected: false,
            dense: None,
            is_three_line: None,
            visual_density: None,
            style: None,
            title_alignment: None,
            horizontal_title_gap: None,
            min_vertical_padding: None,
            min_leading_width: None,
            min_tile_height: None,
            tile_color: None,
            selected_tile_color: None,
            selected_color: None,
            icon_color: None,
            text_color: None,
            action: None,
        }
    }

    /// Set the secondary text for the item.
    ///
    /// Secondary text is displayed below the primary text.
    ///
    /// # Arguments
    /// * `text` - The secondary text to display
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .secondary_text("This is some secondary text");
    /// ```
    pub fn secondary_text(mut self, text: impl Into<String>) -> Self {
        self.secondary_text = Some(text.into());
        self
    }

    /// Set the overline text for the item.
    ///
    /// Overline text is displayed above the primary text.
    ///
    /// # Arguments
    /// * `text` - The overline text to display
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .overline("Important")
    ///     .secondary_text("This is some secondary text");
    /// ```
    pub fn overline(mut self, text: impl Into<String>) -> Self {
        self.overline_text = Some(text.into());
        self
    }

    /// Set a leading icon for the item.
    ///
    /// A leading icon is displayed at the start of the item, before the text.
    ///
    /// # Arguments
    /// * `icon` - The name of the icon to display
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .leading_icon("check");
    /// ```
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set a trailing icon for the item.
    ///
    /// A trailing icon is displayed at the end of the item, after the text.
    ///
    /// # Arguments
    /// * `icon` - The name of the icon to display
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .trailing_icon("more_vert");
    /// ```
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set trailing text for the item.
    ///
    /// Trailing text is displayed at the end of the item, after the icons.
    ///
    /// # Arguments
    /// * `text` - The trailing text to display
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .trailing_text("99+");
    /// ```
    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    /// Enable or disable the item.
    ///
    /// Disabled items are not interactive and are typically displayed with
    /// reduced opacity.
    ///
    /// # Arguments
    /// * `enabled` - Whether the item should be enabled
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .enabled(false); // This item is disabled
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the selected state of the item.
    ///
    /// Selected items are highlighted with a different background color
    /// and may use different text/icon colors.
    ///
    /// # Arguments
    /// * `selected` - Whether the item should appear selected
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .selected(true); // This item appears selected
    /// ```
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set whether this list tile is part of a vertically dense list.
    ///
    /// Dense list tiles default to a smaller height.
    ///
    /// # Arguments
    /// * `dense` - Whether to use dense layout
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .dense(true); // Compact layout
    /// ```
    pub fn dense(mut self, dense: bool) -> Self {
        self.dense = Some(dense);
        self
    }

    /// Set whether this list tile is intended to display three lines of text.
    ///
    /// # Arguments
    /// * `is_three_line` - Whether to use three-line layout
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .is_three_line(true);
    /// ```
    pub fn is_three_line(mut self, is_three_line: bool) -> Self {
        self.is_three_line = Some(is_three_line);
        self
    }

    /// Set the visual density for compact/comfortable/spacious layouts.
    ///
    /// # Arguments
    /// * `density` - The visual density to apply
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .visual_density(VisualDensity::COMPACT);
    /// ```
    pub fn visual_density(mut self, density: VisualDensity) -> Self {
        self.visual_density = Some(density);
        self
    }

    /// Set the title style (List or Drawer).
    ///
    /// # Arguments
    /// * `style` - The list tile style
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .style(ListTileStyle::Drawer);
    /// ```
    pub fn style(mut self, style: ListTileStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Set how leading and trailing widgets are vertically aligned.
    ///
    /// # Arguments
    /// * `alignment` - The title alignment mode
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .title_alignment(ListTileTitleAlignment::Center);
    /// ```
    pub fn title_alignment(mut self, alignment: ListTileTitleAlignment) -> Self {
        self.title_alignment = Some(alignment);
        self
    }

    /// Set the horizontal gap between titles and leading/trailing widgets.
    ///
    /// # Arguments
    /// * `gap` - The gap in pixels
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .horizontal_title_gap(20.0);
    /// ```
    pub fn horizontal_title_gap(mut self, gap: f32) -> Self {
        self.horizontal_title_gap = Some(gap);
        self
    }

    /// Set the minimum padding on top and bottom of title/subtitle.
    ///
    /// # Arguments
    /// * `padding` - The minimum vertical padding in pixels
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .min_vertical_padding(8.0);
    /// ```
    pub fn min_vertical_padding(mut self, padding: f32) -> Self {
        self.min_vertical_padding = Some(padding);
        self
    }

    /// Set the minimum width allocated for the leading widget.
    ///
    /// # Arguments
    /// * `width` - The minimum leading width in pixels
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .min_leading_width(48.0);
    /// ```
    pub fn min_leading_width(mut self, width: f32) -> Self {
        self.min_leading_width = Some(width);
        self
    }

    /// Set the minimum height allocated for the list tile.
    ///
    /// # Arguments
    /// * `height` - The minimum tile height in pixels
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .min_tile_height(64.0);
    /// ```
    pub fn min_tile_height(mut self, height: f32) -> Self {
        self.min_tile_height = Some(height);
        self
    }

    /// Set the background color when not selected.
    ///
    /// # Arguments
    /// * `color` - The tile background color
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .tile_color(Color32::from_rgb(240, 240, 240));
    /// ```
    pub fn tile_color(mut self, color: Color32) -> Self {
        self.tile_color = Some(color);
        self
    }

    /// Set the background color when selected.
    ///
    /// # Arguments
    /// * `color` - The selected tile background color
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .selected_tile_color(Color32::from_rgb(200, 230, 255));
    /// ```
    pub fn selected_tile_color(mut self, color: Color32) -> Self {
        self.selected_tile_color = Some(color);
        self
    }

    /// Set the color for icons and text when selected.
    ///
    /// # Arguments
    /// * `color` - The selected content color
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .selected_color(Color32::from_rgb(0, 100, 200));
    /// ```
    pub fn selected_color(mut self, color: Color32) -> Self {
        self.selected_color = Some(color);
        self
    }

    /// Set the default color for leading and trailing icons.
    ///
    /// # Arguments
    /// * `color` - The icon color
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .icon_color(Color32::from_rgb(100, 100, 100));
    /// ```
    pub fn icon_color(mut self, color: Color32) -> Self {
        self.icon_color = Some(color);
        self
    }

    /// Set the text color for title, subtitle, leading, and trailing.
    ///
    /// # Arguments
    /// * `color` - The text color
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .text_color(Color32::from_rgb(0, 0, 0));
    /// ```
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set a click action for the item.
    ///
    /// # Arguments
    /// * `f` - A function to call when the item is clicked
    ///
    /// # Example
    /// ```rust
    /// let item = ListItem::new("Item")
    ///     .on_click(|| {
    ///         println!("Item was clicked!");
    ///     });
    /// ```
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

impl<'a> Widget for MaterialList<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Material Design colors
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline_variant = get_global_color("outlineVariant");
        let primary = get_global_color("primary");
        let on_primary_container = get_global_color("onPrimaryContainer");
        let primary_container = get_global_color("primaryContainer");

        // Calculate total height and max width
        let mut total_height = 0.0;
        let mut max_content_width = 200.0;

        for item in &self.items {
            // Calculate item height based on configuration
            let visual_density = item.visual_density.unwrap_or_default();
            let density_adjustment = visual_density.base_size_adjustment().y;
            let is_dense = item.dense.unwrap_or(false);

            let base_height = if item.is_three_line.unwrap_or(false)
                || (item.overline_text.is_some() && item.secondary_text.is_some())
            {
                if is_dense {
                    76.0
                } else {
                    88.0
                }
            } else if item.secondary_text.is_some() || item.overline_text.is_some() {
                if is_dense {
                    64.0
                } else {
                    72.0
                }
            } else {
                if is_dense {
                    48.0
                } else {
                    56.0
                }
            };

            let item_height = item
                .min_tile_height
                .unwrap_or(base_height + density_adjustment);
            total_height += item_height;

            // Calculate item width
            let mut item_width = 32.0; // base padding
            if item.leading_icon.is_some() {
                item_width += item.min_leading_width.unwrap_or(40.0);
            }
            let primary_text_width = item.primary_text.len() as f32 * 8.0;
            let secondary_text_width = item
                .secondary_text
                .as_ref()
                .map_or(0.0, |s| s.len() as f32 * 6.0);
            let overline_text_width = item
                .overline_text
                .as_ref()
                .map_or(0.0, |s| s.len() as f32 * 5.5);
            let max_text_width = primary_text_width
                .max(secondary_text_width)
                .max(overline_text_width);
            item_width += max_text_width;
            if let Some(ref trailing_text) = item.trailing_text {
                item_width += trailing_text.len() as f32 * 6.0;
            }
            if item.trailing_icon.is_some() {
                item_width += 40.0;
            }
            item_width += 32.0;

            if item_width > max_content_width {
                max_content_width = item_width;
            }
        }

        if self.dividers && self.items.len() > 1 {
            total_height += (self.items.len() - 1) as f32;
        }

        let list_width = max_content_width.min(ui.available_width());
        let desired_size = Vec2::new(list_width, total_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Draw list background
        ui.painter().rect_filled(rect, 8.0, surface);
        ui.painter().rect_stroke(
            rect,
            8.0,
            Stroke::new(1.0, outline_variant),
            egui::epaint::StrokeKind::Outside,
        );

        let mut current_y = rect.min.y;
        let mut pending_actions = Vec::new();
        let items_len = self.items.len();

        for (index, item) in self.items.into_iter().enumerate() {
            // Calculate item-specific dimensions
            let visual_density = item.visual_density.unwrap_or_default();
            let density_adjustment = visual_density.base_size_adjustment().y;
            let is_dense = item.dense.unwrap_or(false);

            let base_height = if item.is_three_line.unwrap_or(false)
                || (item.overline_text.is_some() && item.secondary_text.is_some())
            {
                if is_dense {
                    76.0
                } else {
                    88.0
                }
            } else if item.secondary_text.is_some() || item.overline_text.is_some() {
                if is_dense {
                    64.0
                } else {
                    72.0
                }
            } else {
                if is_dense {
                    48.0
                } else {
                    56.0
                }
            };

            let item_height = item
                .min_tile_height
                .unwrap_or(base_height + density_adjustment);

            let item_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, current_y),
                Vec2::new(rect.width(), item_height),
            );

            // Use list's ID (or auto-generate one) to scope item IDs and avoid collisions
            let list_id = self.id.unwrap_or_else(|| ui.id().with("material_list"));
            let unique_id = list_id.with(("item", index));
            let item_response = ui.interact(item_rect, unique_id, Sense::click());

            // Determine background color
            let bg_color = if item.selected {
                item.selected_tile_color.unwrap_or_else(|| {
                    Color32::from_rgba_premultiplied(
                        primary_container.r(),
                        primary_container.g(),
                        primary_container.b(),
                        255,
                    )
                })
            } else {
                item.tile_color.unwrap_or(Color32::TRANSPARENT)
            };

            // Draw background
            if bg_color != Color32::TRANSPARENT {
                ui.painter().rect_filled(item_rect, 0.0, bg_color);
            }

            // Draw hover effect
            if item_response.hovered() && item.enabled {
                let hover_color = Color32::from_rgba_premultiplied(
                    on_surface.r(),
                    on_surface.g(),
                    on_surface.b(),
                    20,
                );
                ui.painter().rect_filled(item_rect, 0.0, hover_color);
            }

            // Handle click
            if item_response.clicked() && item.enabled {
                if let Some(action) = item.action {
                    pending_actions.push(action);
                }
            }

            // Calculate colors
            let icon_color = if item.selected {
                item.selected_color.unwrap_or(on_primary_container)
            } else if item.enabled {
                item.icon_color.unwrap_or(on_surface_variant)
            } else {
                on_surface_variant.linear_multiply(0.38)
            };

            let text_color = if item.selected {
                item.selected_color.unwrap_or(on_primary_container)
            } else if item.enabled {
                item.text_color.unwrap_or(on_surface)
            } else {
                on_surface.linear_multiply(0.38)
            };

            // Layout constants
            let horizontal_title_gap = item.horizontal_title_gap.unwrap_or(16.0)
                + visual_density.horizontal * 2.0;
            let min_vertical_padding = item.min_vertical_padding.unwrap_or(8.0);
            let min_leading_width = item.min_leading_width.unwrap_or(40.0);
            
            let mut content_x = item_rect.min.x + 16.0;
            let content_y = item_rect.center().y;

            // Draw leading icon
            if let Some(icon_name) = &item.leading_icon {
                let leading_width = min_leading_width;
                let icon_pos = Pos2::new(content_x + leading_width / 2.0, content_y);

                let icon_string = icon_text(icon_name);
                ui.painter().text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    &icon_string,
                    egui::FontId::proportional(20.0),
                    icon_color,
                );
                content_x += leading_width + horizontal_title_gap;
            }

            // Calculate trailing width
            let trailing_icon_width = if item.trailing_icon.is_some() {
                40.0
            } else {
                0.0
            };
            let trailing_text_width = if item.trailing_text.is_some() {
                80.0
            } else {
                0.0
            };
            let total_trailing_width = trailing_icon_width + trailing_text_width;

            // Draw text content based on configuration
            match (&item.overline_text, &item.secondary_text) {
                (Some(overline), Some(secondary)) => {
                    // Three-line layout
                    let overline_pos = Pos2::new(content_x, content_y - 20.0);
                    let primary_pos = Pos2::new(content_x, content_y);
                    let secondary_pos = Pos2::new(content_x, content_y + 20.0);

                    ui.painter().text(
                        overline_pos,
                        egui::Align2::LEFT_CENTER,
                        overline,
                        egui::FontId::proportional(if is_dense { 10.0 } else { 11.0 }),
                        on_surface_variant,
                    );

                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );

                    ui.painter().text(
                        secondary_pos,
                        egui::Align2::LEFT_CENTER,
                        secondary,
                        egui::FontId::proportional(if is_dense { 11.0 } else { 12.0 }),
                        on_surface_variant,
                    );
                }
                (Some(overline), None) => {
                    // Two-line layout: overline + primary
                    let overline_pos = Pos2::new(content_x, content_y - 10.0);
                    let primary_pos = Pos2::new(content_x, content_y + 10.0);

                    ui.painter().text(
                        overline_pos,
                        egui::Align2::LEFT_CENTER,
                        overline,
                        egui::FontId::proportional(if is_dense { 10.0 } else { 11.0 }),
                        on_surface_variant,
                    );

                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );
                }
                (None, Some(secondary)) => {
                    // Two-line layout: primary + secondary
                    let primary_pos = Pos2::new(content_x, content_y - 10.0);
                    let secondary_pos = Pos2::new(content_x, content_y + 10.0);

                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );

                    ui.painter().text(
                        secondary_pos,
                        egui::Align2::LEFT_CENTER,
                        secondary,
                        egui::FontId::proportional(if is_dense { 11.0 } else { 12.0 }),
                        on_surface_variant,
                    );
                }
                (None, None) => {
                    // Single-line layout
                    let text_pos = Pos2::new(content_x, content_y);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );
                }
            }

            // Draw trailing text
            if let Some(ref trailing_text) = item.trailing_text {
                let trailing_text_pos = Pos2::new(
                    item_rect.max.x - trailing_icon_width - trailing_text_width + 10.0,
                    content_y,
                );

                ui.painter().text(
                    trailing_text_pos,
                    egui::Align2::LEFT_CENTER,
                    trailing_text,
                    egui::FontId::proportional(12.0),
                    on_surface_variant,
                );
            }

            // Draw trailing icon
            if let Some(icon_name) = &item.trailing_icon {
                let icon_pos = Pos2::new(item_rect.max.x - 28.0, content_y);

                let icon_string = icon_text(icon_name);
                ui.painter().text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    &icon_string,
                    egui::FontId::proportional(20.0),
                    icon_color,
                );
            }

            current_y += item_height;

            // Draw divider
            if self.dividers && index < items_len - 1 {
                let divider_y = current_y;
                let divider_start = Pos2::new(rect.min.x + 16.0, divider_y);
                let divider_end = Pos2::new(rect.max.x - 16.0, divider_y);

                ui.painter().line_segment(
                    [divider_start, divider_end],
                    Stroke::new(1.0, outline_variant),
                );
                current_y += 1.0;
            }
        }

        // Execute pending actions
        for action in pending_actions {
            action();
        }

        response
    }
}

pub fn list_item(primary_text: impl Into<String>) -> ListItem<'static> {
    ListItem::new(primary_text)
}

pub fn list() -> MaterialList<'static> {
    MaterialList::new()
}
