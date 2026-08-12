//! Type definitions for Material Design 3 List Components

use egui::{ecolor::Color32, Vec2};

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
    pub(crate) items: Vec<ListItem<'a>>,
    /// Whether to show dividers between items
    pub(crate) dividers: bool,
    /// Optional unique ID for this list to avoid widget ID collisions
    pub(crate) id: Option<egui::Id>,
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
    pub(crate) primary_text: String,
    /// Optional secondary text displayed below primary text
    pub(crate) secondary_text: Option<String>,
    /// Optional overline text displayed above primary text
    pub(crate) overline_text: Option<String>,
    /// Optional icon displayed at the start of the item
    pub(crate) leading_icon: Option<String>,
    /// Optional icon displayed at the end of the item
    pub(crate) trailing_icon: Option<String>,
    /// Optional text displayed at the end of the item
    pub(crate) trailing_text: Option<String>,
    /// Whether the item is enabled and interactive
    pub(crate) enabled: bool,
    /// Whether the item is selected
    pub(crate) selected: bool,
    /// Whether this list tile is part of a vertically dense list
    pub(crate) dense: Option<bool>,
    /// Whether this list tile is intended to display three lines of text
    pub(crate) is_three_line: Option<bool>,
    /// Defines how compact the list tile's layout will be
    pub(crate) visual_density: Option<VisualDensity>,
    /// Defines the font used for the title
    pub(crate) style: Option<ListTileStyle>,
    /// Defines how leading and trailing are vertically aligned
    pub(crate) title_alignment: Option<ListTileTitleAlignment>,
    /// The horizontal gap between the titles and the leading/trailing widgets
    pub(crate) horizontal_title_gap: Option<f32>,
    /// The minimum padding on the top and bottom of the title and subtitle widgets
    pub(crate) min_vertical_padding: Option<f32>,
    /// The minimum width allocated for the leading widget
    pub(crate) min_leading_width: Option<f32>,
    /// The minimum height allocated for the list tile widget
    pub(crate) min_tile_height: Option<f32>,
    /// Background color when selected is false
    pub(crate) tile_color: Option<Color32>,
    /// Background color when selected is true
    pub(crate) selected_tile_color: Option<Color32>,
    /// Color for icons and text when selected
    pub(crate) selected_color: Option<Color32>,
    /// Default color for leading and trailing icons
    pub(crate) icon_color: Option<Color32>,
    /// Text color for title, subtitle, leading, and trailing
    pub(crate) text_color: Option<Color32>,
    /// Callback function to execute when the item is clicked
    pub(crate) action: Option<Box<dyn Fn() + 'a>>,
}
