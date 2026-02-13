use crate::get_global_color;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Material Design tabs component.
///
/// Tabs organize content across different screens, data sets, and other interactions.
/// They allow users to navigate between related groups of content.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selected_tab = 0;
///
/// ui.add(MaterialTabs::primary(&mut selected_tab)
///     .tab("Home")
///     .tab("Profile")
///     .tab("Settings"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialTabs<'a> {
    /// Reference to the currently selected tab index
    selected: &'a mut usize,
    /// List of tab items
    tabs: Vec<TabItem>,
    /// Whether the tabs are enabled for interaction
    enabled: bool,
    /// Visual variant of the tabs (primary or secondary)
    variant: TabVariant,
    /// Optional salt for generating unique IDs
    id_salt: Option<String>,
    /// Optional custom height for the tab bar
    height: Option<f32>,
}

/// Individual tab item data.
pub struct TabItem {
    /// Display label for the tab
    label: String,
    /// Optional icon for the tab
    icon: Option<String>,
}

/// Visual variants for tabs component.
#[derive(Clone, Copy, PartialEq)]
pub enum TabVariant {
    /// Primary tabs (filled background, more prominent)
    Primary,
    /// Secondary tabs (outlined style, less prominent)
    Secondary,
}

impl<'a> MaterialTabs<'a> {
    /// Create a new tabs component.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected tab index
    /// * `variant` - Visual variant (Primary or Secondary)
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// let tabs = MaterialTabs::new(&mut tab_index, TabVariant::Primary);
    /// # });
    /// ```
    pub fn new(selected: &'a mut usize, variant: TabVariant) -> Self {
        Self {
            selected,
            tabs: Vec::new(),
            enabled: true,
            variant,
            id_salt: None,
            height: None,
        }
    }

    /// Create a primary tabs component.
    ///
    /// Primary tabs have a filled background and are more prominent.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected tab index
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// let tabs = MaterialTabs::primary(&mut tab_index);
    /// # });
    /// ```
    pub fn primary(selected: &'a mut usize) -> Self {
        Self::new(selected, TabVariant::Primary)
    }

    /// Create a secondary tabs component.
    ///
    /// Secondary tabs have an outlined style and are less prominent than primary tabs.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected tab index
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// let tabs = MaterialTabs::secondary(&mut tab_index);
    /// # });
    /// ```
    pub fn secondary(selected: &'a mut usize) -> Self {
        Self::new(selected, TabVariant::Secondary)
    }

    /// Add a tab with just a text label.
    ///
    /// # Arguments
    /// * `label` - Text label for the tab
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab("Home")
    ///     .tab("About"));
    /// # });
    /// ```
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.tabs.push(TabItem {
            label: label.into(),
            icon: None,
        });
        self
    }

    /// Add a tab with both an icon and text label.
    ///
    /// # Arguments
    /// * `label` - Text label for the tab
    /// * `icon` - Icon identifier (e.g., "home", "person", "settings")
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab_with_icon("Home", "home")
    ///     .tab_with_icon("Profile", "person"));
    /// # });
    /// ```
    pub fn tab_with_icon(mut self, label: impl Into<String>, icon: impl Into<String>) -> Self {
        self.tabs.push(TabItem {
            label: label.into(),
            icon: Some(icon.into()),
        });
        self
    }

    /// Set whether the tabs are enabled for interaction.
    ///
    /// # Arguments
    /// * `enabled` - `true` to enable tabs, `false` to disable
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab("Home")
    ///     .tab("Profile")
    ///     .tab("Settings")
    ///     .enabled(false));
    /// # });
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set an optional salt for generating unique IDs for the tabs.
    ///
    /// This is useful if you have multiple instances of tabs and want to avoid ID collisions.
    ///
    /// # Arguments
    /// * `salt` - Salt string to use in ID generation
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab("Home")
    ///     .tab("Profile")
    ///     .tab("Settings")
    ///     .id_salt("unique_salt"));
    /// # });
    /// ```
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Set a custom height for the tab bar.
    ///
    /// Default is 48.0 pixels if not specified.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
}

impl<'a> Widget for MaterialTabs<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tab_height = self.height.unwrap_or(48.0);
        let tab_width = ui.available_width() / self.tabs.len().max(1) as f32;

        let desired_size = Vec2::new(ui.available_width(), tab_height);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline_variant = get_global_color("outlineVariant");

        // Draw tab bar background
        ui.painter().rect_filled(rect, 0.0, surface);

        // Draw tabs
        let mut any_clicked = false;
        for (index, tab) in self.tabs.iter().enumerate() {
            let tab_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + index as f32 * tab_width, rect.min.y),
                Vec2::new(tab_width, tab_height),
            );

            // Create unique ID for each tab using optional salt
            let tab_id = if let Some(ref salt) = self.id_salt {
                egui::Id::new((salt, "tab", index))
            } else {
                egui::Id::new(("tab", index))
            };

            let tab_response = ui.interact(tab_rect, tab_id, Sense::click());

            let is_selected = *self.selected == index;
            let is_hovered = tab_response.hovered();

            // Determine colors
            let (text_color, indicator_color) = match self.variant {
                TabVariant::Primary => {
                    if is_selected {
                        (primary_color, primary_color)
                    } else if is_hovered && self.enabled {
                        (on_surface, on_surface_variant)
                    } else {
                        (on_surface_variant, Color32::TRANSPARENT)
                    }
                }
                TabVariant::Secondary => {
                    if is_selected {
                        (on_surface, outline_variant)
                    } else if is_hovered && self.enabled {
                        (on_surface, Color32::TRANSPARENT)
                    } else {
                        (on_surface_variant, Color32::TRANSPARENT)
                    }
                }
            };

            // Draw hover background
            if is_hovered && self.enabled {
                let hover_color = Color32::from_rgba_premultiplied(
                    text_color.r(),
                    text_color.g(),
                    text_color.b(),
                    20,
                );
                ui.painter().rect_filled(tab_rect, 0.0, hover_color);
            }

            // Handle click
            if tab_response.clicked() && self.enabled {
                *self.selected = index;
                any_clicked = true;
            }

            // Layout tab content
            let mut content_y = tab_rect.center().y;

            // Draw icon if present
            if let Some(_icon) = &tab.icon {
                let icon_rect = Rect::from_min_size(
                    Pos2::new(tab_rect.center().x - 12.0, content_y - 20.0),
                    Vec2::splat(24.0),
                );

                ui.painter()
                    .circle_filled(icon_rect.center(), 8.0, text_color);
                content_y += 12.0;
            }

            // Draw tab text
            let text_pos = Pos2::new(tab_rect.center().x, content_y);
            let font_size = if tab.icon.is_some() {
                egui::FontId::proportional(12.0)
            } else {
                egui::FontId::default()
            };

            ui.painter().text(
                text_pos,
                egui::Align2::CENTER_CENTER,
                &tab.label,
                font_size,
                text_color,
            );

            // Draw indicator
            match self.variant {
                TabVariant::Primary => {
                    if is_selected && indicator_color != Color32::TRANSPARENT {
                        let indicator_rect = Rect::from_min_size(
                            Pos2::new(tab_rect.min.x + 8.0, tab_rect.max.y - 3.0),
                            Vec2::new(tab_width - 16.0, 3.0),
                        );
                        ui.painter()
                            .rect_filled(indicator_rect, 1.5, indicator_color);
                    }
                }
                TabVariant::Secondary => {
                    if is_selected && indicator_color != Color32::TRANSPARENT {
                        ui.painter().rect_stroke(
                            tab_rect,
                            0.0,
                            Stroke::new(1.0, indicator_color),
                            egui::epaint::StrokeKind::Outside,
                        );
                    }
                }
            }
        }

        // Draw bottom border for secondary variant
        if self.variant == TabVariant::Secondary {
            let border_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.max.y - 1.0),
                Vec2::new(rect.width(), 1.0),
            );
            ui.painter().rect_filled(border_rect, 0.0, outline_variant);
        }

        if any_clicked {
            response.mark_changed();
        }
        response
    }
}

/// Convenience function to create primary tabs.
///
/// Shorthand for `MaterialTabs::primary()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected tab index
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut tab_index = 0;
/// ui.add(tabs_primary(&mut tab_index)
///     .tab("Tab 1")
///     .tab("Tab 2"));
/// # });
/// ```
pub fn tabs_primary<'a>(selected: &'a mut usize) -> MaterialTabs<'a> {
    MaterialTabs::primary(selected)
}

/// Convenience function to create secondary tabs.
///
/// Shorthand for `MaterialTabs::secondary()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected tab index
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut tab_index = 0;
/// ui.add(tabs_secondary(&mut tab_index)
///     .tab("Tab 1")
///     .tab("Tab 2"));
/// # });
/// ```
pub fn tabs_secondary<'a>(selected: &'a mut usize) -> MaterialTabs<'a> {
    MaterialTabs::secondary(selected)
}
