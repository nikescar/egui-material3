//! Builder methods for MaterialList

use super::{types::*, MaterialList};
use egui::ecolor::Color32;

impl<'a> Default for MaterialList<'a> {
    fn default() -> Self {
        Self::new()
    }
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
    /// Overrides the default transparent background.
    /// By default, non-selected items are transparent to show the parent surface.
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
    /// Overrides the default M3 **primaryContainer** color role.
    /// By default, selected items use primaryContainer (less emphasized fill for selected elements).
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
    /// Overrides the default M3 **onPrimaryContainer** color role.
    /// By default, selected items use onPrimaryContainer (content on primaryContainer).
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
    /// Overrides the default M3 **onSurfaceVariant** color role.
    /// By default, icons use onSurfaceVariant (lower emphasis for icons).
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
    /// Overrides the default M3 **onSurface** color role for primary text.
    /// By default, primary text uses onSurface (standard content color on surface).
    /// Note: Secondary/overline text always uses onSurfaceVariant regardless of this setting.
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
