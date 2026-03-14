use crate::get_global_color;
use egui::{
    Align, Color32, CornerRadius, Layout, Response, Shadow, Stroke, Ui, Vec2, Widget,
};

/// Material Design 3 toolbar component.
///
/// A fixed area at the bottom (or top) of a screen that contains navigation elements.
/// The toolbar serves as a container for navigational links, buttons, and icon buttons.
///
/// # Material Design 3 Color Roles
///
/// This component follows M3 color guidelines:
///
/// ## Top Toolbar (App Bar)
/// - Background: `surface` (at rest) or `surfaceContainerLow` (for subtle elevation)
/// - Content: `onSurface` for text/icons, `onSurfaceVariant` for secondary icons
/// - Border: `outlineVariant` for dividers
///
/// ## Bottom Navigation Bar
/// - Background: `surfaceContainer` (standard for bottom navigation)
/// - Content: `onSurface` for text/icons, `onSurfaceVariant` for secondary elements
/// - Border: `outlineVariant` for top divider
///
/// ## Elevation
/// M3 prefers tone-based surface containers over shadows for visual hierarchy.
/// The component uses surface container color roles to indicate elevation levels
/// without relying heavily on shadows.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{toolbar, MaterialButton, icon_button_standard};
///
/// ui.add(toolbar()
///     .item(MaterialButton::text("Home"))
///     .item_fn(|ui| ui.add(icon_button_standard("search")))
///     .item(MaterialButton::text("Settings")));
/// # });
/// ```
/// Material Design 3 surface elevation levels for toolbar
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarElevation {
    /// No elevation - uses base surface colors
    Level0,
    /// Low elevation - uses surfaceContainerLow
    Level1,
    /// Medium elevation - uses surfaceContainer
    Level2,
    /// High elevation - uses surfaceContainerHigh
    Level3,
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialToolbar<'a> {
    /// Items to display in the toolbar (buttons, icon buttons, etc.)
    items: Vec<ToolbarItem<'a>>,
    /// Position the toolbar at the top of the screen
    top: bool,
    /// Enable tabbar mode with equal-width items
    tabbar: bool,
    /// Show icons in tabbar mode
    tabbar_icons: bool,
    /// Show labels in tabbar mode
    tabbar_labels: bool,
    /// Show outline/border
    outline: bool,
    /// Custom background color (overrides elevation-based colors)
    bg_color: Option<Color32>,
    /// M3 elevation level (determines surface container to use)
    elevation: Option<ToolbarElevation>,
    /// Minimum height of the toolbar
    min_height: f32,
    /// Spacing between items
    item_spacing: f32,
    /// Inner padding
    padding: Vec2,
}

/// Represents an item in the toolbar
enum ToolbarItem<'a> {
    /// A regular widget (button, icon button, etc.)
    Widget(Box<dyn FnOnce(&mut Ui) -> Response + 'a>),
    /// A spacer to push items apart
    Spacer,
}

impl<'a> Default for MaterialToolbar<'a> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            top: false,
            tabbar: false,
            tabbar_icons: false,
            tabbar_labels: false,
            outline: true,
            bg_color: None,
            elevation: None, // Use automatic elevation based on position
            min_height: 56.0, // Material Design standard toolbar height
            item_spacing: 8.0,
            padding: Vec2::new(16.0, 8.0),
        }
    }
}

impl<'a> MaterialToolbar<'a> {
    /// Create a new toolbar
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a widget item to the toolbar
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, MaterialButton};
    ///
    /// ui.add(toolbar()
    ///     .item(MaterialButton::text("Home"))
    ///     .item(MaterialButton::text("Profile")));
    /// # });
    /// ```
    pub fn item<W>(mut self, widget: W) -> Self
    where
        W: Widget + 'a,
    {
        self.items.push(ToolbarItem::Widget(Box::new(move |ui| {
            ui.add(widget)
        })));
        self
    }

    /// Add a custom closure as an item
    ///
    /// Use this for icon buttons and other widgets that need special handling.
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, icon_button_standard};
    ///
    /// ui.add(toolbar()
    ///     .item_fn(|ui| ui.add(icon_button_standard("home")))
    ///     .item_fn(|ui| ui.add(icon_button_standard("search"))));
    /// # });
    /// ```
    pub fn item_fn<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Ui) -> Response + 'a,
    {
        self.items.push(ToolbarItem::Widget(Box::new(f)));
        self
    }

    /// Add a spacer to push subsequent items to the right
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, MaterialButton};
    ///
    /// ui.add(toolbar()
    ///     .item(MaterialButton::text("Left"))
    ///     .spacer()
    ///     .item(MaterialButton::text("Right")));
    /// # });
    /// ```
    pub fn spacer(mut self) -> Self {
        self.items.push(ToolbarItem::Spacer);
        self
    }

    /// Position the toolbar at the top of the screen
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::toolbar;
    ///
    /// ui.add(toolbar()
    ///     .top(true));
    /// # });
    /// ```
    pub fn top(mut self, top: bool) -> Self {
        self.top = top;
        self
    }

    /// Enable tabbar mode with equal-width items
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::toolbar;
    ///
    /// ui.add(toolbar()
    ///     .tabbar(true));
    /// # });
    /// ```
    pub fn tabbar(mut self, tabbar: bool) -> Self {
        self.tabbar = tabbar;
        self
    }

    /// Show icons in tabbar mode
    pub fn tabbar_icons(mut self, show: bool) -> Self {
        self.tabbar_icons = show;
        self
    }

    /// Show labels in tabbar mode
    pub fn tabbar_labels(mut self, show: bool) -> Self {
        self.tabbar_labels = show;
        self
    }

    /// Show outline/border
    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    /// Set custom background color (overrides elevation-based colors)
    pub fn bg_color(mut self, color: Color32) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set M3 elevation level
    ///
    /// Controls which surface container color role is used for the background.
    /// Higher elevation levels use darker/lighter surface containers (depending on theme).
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, ToolbarElevation};
    ///
    /// ui.add(toolbar()
    ///     .elevation(ToolbarElevation::Level2));
    /// # });
    /// ```
    pub fn elevation(mut self, elevation: ToolbarElevation) -> Self {
        self.elevation = Some(elevation);
        self
    }

    /// Set minimum height of the toolbar
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = height;
        self
    }

    /// Set spacing between items
    pub fn item_spacing(mut self, spacing: f32) -> Self {
        self.item_spacing = spacing;
        self
    }

    /// Set inner padding
    pub fn padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }
}

impl<'a> Widget for MaterialToolbar<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Material Design 3 color roles for toolbar/bottom navigation
        // Following M3 guidelines for surface containers and elevation
        let surface = get_global_color("surface");
        let surface_container_low = get_global_color("surfaceContainerLow");
        let surface_container = get_global_color("surfaceContainer");
        let surface_container_high = get_global_color("surfaceContainerHigh");
        let outline_variant = get_global_color("outlineVariant");
        let _on_surface = get_global_color("onSurface");
        let _on_surface_variant = get_global_color("onSurfaceVariant");

        // Determine background color based on position, elevation, and M3 guidelines
        // M3 uses surface container levels to create visual hierarchy without relying on shadows
        let bg_color = self.bg_color.unwrap_or_else(|| {
            // If elevation is explicitly set, use it to determine surface container
            if let Some(elev) = self.elevation {
                match elev {
                    ToolbarElevation::Level0 => surface,
                    ToolbarElevation::Level1 => surface_container_low,
                    ToolbarElevation::Level2 => surface_container,
                    ToolbarElevation::Level3 => surface_container_high,
                }
            } else if self.top {
                // Top toolbar default: surface (M3 standard for top app bars at rest)
                // Top app bars use surface with minimal or no shadow in modern M3
                surface
            } else {
                // Bottom navigation default: surfaceContainer (M3 standard for bottom nav)
                surface_container
            }
        });

        // Calculate available width
        let available_width = ui.available_width();

        // Allocate space for the toolbar
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(available_width, self.min_height),
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            // Material Design 3 elevation approach:
            // M3 prefers tone-based surface containers over shadows for elevation
            // Surface containers (surfaceContainerLowest through surfaceContainerHighest)
            // provide visual hierarchy without relying solely on shadows

            // Optional: Draw subtle shadow for top toolbar (legacy elevation support)
            // Modern M3 apps may omit this in favor of pure surface container tones
            if self.top {
                // Subtle shadow for top app bar (elevation level 2)
                // Note: M3 de-emphasizes shadows in favor of surface container tones
                let shadow = Shadow {
                    offset: [0, 1],
                    blur: 3,
                    spread: 0,
                    color: Color32::from_black_alpha(12), // Reduced from 16 for subtlety
                };
                let shadow_offset = Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32);
                let shadow_rect = rect.translate(shadow_offset);

                // Draw minimal shadow with reduced blur for modern M3 look
                for i in 0..2 {
                    let blur_offset = i as f32 * 1.0;
                    let alpha = (12 / (i + 1)) as u8;
                    ui.painter().rect_filled(
                        shadow_rect.expand(blur_offset),
                        CornerRadius::ZERO,
                        Color32::from_black_alpha(alpha),
                    );
                }
            }

            // Draw background using M3 surface color roles
            // No additional tint overlay needed - the surface container colors
            // already provide the correct tonal values for elevation hierarchy
            ui.painter().rect_filled(
                rect,
                CornerRadius::ZERO,
                bg_color,
            );

            // Draw outline/border using M3 outline color roles
            // outlineVariant: A less emphasized version of the outline color
            // Used for borders or dividers that provide structure to UI elements
            if self.outline {
                let border_color = outline_variant;

                let border_y = if self.top {
                    // Top toolbar: border at bottom edge
                    rect.max.y
                } else {
                    // Bottom navigation: border at top edge
                    rect.min.y
                };

                ui.painter().line_segment(
                    [
                        egui::pos2(rect.min.x, border_y),
                        egui::pos2(rect.max.x, border_y),
                    ],
                    Stroke::new(1.0, border_color),
                );
            }

            // Create a child UI for content
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(rect.shrink2(self.padding))
                    .layout(Layout::left_to_right(Align::Center))
            );

            child_ui.spacing_mut().item_spacing.x = self.item_spacing;

            if self.tabbar {
                // Tabbar mode: equal-width items
                let item_count = self.items.len();
                if item_count > 0 {
                    let item_width = (rect.width() - self.padding.x * 2.0) / item_count as f32;

                    for item in self.items {
                        match item {
                            ToolbarItem::Widget(widget_fn) => {
                                child_ui.allocate_ui_with_layout(
                                    Vec2::new(item_width, rect.height() - self.padding.y * 2.0),
                                    Layout::centered_and_justified(egui::Direction::TopDown),
                                    |ui| {
                                        widget_fn(ui);
                                    },
                                );
                            }
                            ToolbarItem::Spacer => {
                                // Spacers don't make sense in tabbar mode
                            }
                        }
                    }
                }
            } else {
                // Normal mode: flexible layout
                for item in self.items {
                    match item {
                        ToolbarItem::Widget(widget_fn) => {
                            widget_fn(&mut child_ui);
                        }
                        ToolbarItem::Spacer => {
                            child_ui.with_layout(
                                Layout::right_to_left(Align::Center),
                                |_ui| {},
                            );
                        }
                    }
                }
            }
        }

        response
    }
}

/// Convenience function to create a toolbar
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{toolbar, MaterialButton, icon_button_standard};
///
/// ui.add(toolbar()
///     .item(MaterialButton::text("Home"))
///     .item_fn(|ui| ui.add(icon_button_standard("settings"))));
/// # });
/// ```
pub fn toolbar<'a>() -> MaterialToolbar<'a> {
    MaterialToolbar::new()
}
