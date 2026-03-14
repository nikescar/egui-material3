use crate::get_global_color;
use egui::{
    Align, Color32, Layout, Response, Rounding, Shadow, Stroke, Ui, Vec2, Widget,
};

/// Material Design toolbar component.
///
/// A fixed area at the bottom (or top) of a screen that contains navigation elements.
/// The toolbar serves as a container for navigational links, buttons, and icon buttons.
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
    /// Custom background color
    bg_color: Option<Color32>,
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

    /// Set custom background color
    pub fn bg_color(mut self, color: Color32) -> Self {
        self.bg_color = Some(color);
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
        // Material Design colors for toolbar/bottom navigation
        let surface = get_global_color("surface");
        let surface_container = get_global_color("surfaceContainer");
        let outline_variant = get_global_color("outlineVariant");
        let surface_tint = get_global_color("surfaceTint");

        // Determine background color based on position and state
        let bg_color = self.bg_color.unwrap_or_else(|| {
            if self.top {
                // Top toolbar uses surface with elevation
                surface
            } else {
                // Bottom navigation uses surface container
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
            // Draw elevation shadow for top toolbar (behind background)
            if self.top {
                // Material Design 3 elevation level 2
                let shadow = Shadow {
                    offset: [0, 2],
                    blur: 6,
                    spread: 0,
                    color: Color32::from_black_alpha(16),
                };
                let shadow_offset = Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32);
                let shadow_rect = rect.translate(shadow_offset);

                // Draw shadow with blur simulation
                for i in 0..3 {
                    let blur_offset = i as f32 * 1.5;
                    let alpha = (16 / (i + 1)) as u8;
                    ui.painter().rect_filled(
                        shadow_rect.expand(blur_offset),
                        Rounding::ZERO,
                        Color32::from_black_alpha(alpha),
                    );
                }
            }

            // Draw background with surface tint for elevation
            if self.top {
                // Top toolbar with elevation tint
                ui.painter().rect_filled(
                    rect,
                    Rounding::ZERO,
                    bg_color,
                );

                // Apply surface tint overlay for elevated appearance
                let tint_overlay = Color32::from_rgba_unmultiplied(
                    surface_tint.r(),
                    surface_tint.g(),
                    surface_tint.b(),
                    8, // 3% opacity for elevation level 2
                );
                ui.painter().rect_filled(
                    rect,
                    Rounding::ZERO,
                    tint_overlay,
                );
            } else {
                // Bottom navigation - flat surface
                ui.painter().rect_filled(
                    rect,
                    Rounding::ZERO,
                    bg_color,
                );
            }

            // Draw outline/border with proper Material Design colors
            if self.outline {
                let border_color = if self.top {
                    outline_variant // Subtle for top
                } else {
                    outline_variant // Subtle for bottom
                };

                let border_y = if self.top {
                    rect.max.y
                } else {
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
            let mut child_ui = ui.child_ui(
                rect.shrink2(self.padding),
                Layout::left_to_right(Align::Center),
                None,
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
