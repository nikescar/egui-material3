use crate::get_global_color;
use eframe::egui::{self, Color32, FontId, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};
use eframe::egui::epaint::CornerRadius;

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
    /// Default height is 46.0 for text-only tabs, 72.0 for tabs with icons.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
}

/// M3 tab height constants
const TAB_HEIGHT_TEXT_ONLY: f32 = 46.0;
const TAB_HEIGHT_WITH_ICON: f32 = 72.0;
/// M3 indicator constants
const PRIMARY_INDICATOR_HEIGHT: f32 = 3.0;
const SECONDARY_INDICATOR_HEIGHT: f32 = 2.0;
const INDICATOR_TOP_ROUNDING: f32 = 3.0;
/// M3 divider
const DIVIDER_HEIGHT: f32 = 1.0;
/// M3 label font size
const LABEL_FONT_SIZE: f32 = 14.0;
const ICON_FONT_SIZE: f32 = 18.0;

impl<'a> Widget for MaterialTabs<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let has_icons = self.tabs.iter().any(|t| t.icon.is_some());
        let tab_height = self
            .height
            .unwrap_or(if has_icons { TAB_HEIGHT_WITH_ICON } else { TAB_HEIGHT_TEXT_ONLY });
        let tab_width = ui.available_width() / self.tabs.len().max(1) as f32;

        let desired_size = Vec2::new(ui.available_width(), tab_height);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Material Design 3 colors
        let primary_color = get_global_color("primary");
        let surface_container = get_global_color("surfaceContainer");
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline_variant = get_global_color("outlineVariant");

        // Draw tab bar background
        let bg_color = match self.variant {
            TabVariant::Primary => surface_container,
            TabVariant::Secondary => surface,
        };
        ui.painter().rect_filled(rect, 0.0, bg_color);

        // Draw tabs
        let mut any_clicked = false;
        let label_font = FontId::proportional(LABEL_FONT_SIZE);
        let icon_font = FontId::proportional(ICON_FONT_SIZE);

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

            // M3 label colors per variant
            let text_color = match self.variant {
                TabVariant::Primary => {
                    if is_selected {
                        primary_color
                    } else {
                        on_surface_variant
                    }
                }
                TabVariant::Secondary => {
                    if is_selected {
                        on_surface
                    } else {
                        on_surface_variant
                    }
                }
            };

            // Apply disabled opacity
            let text_color = if self.enabled {
                text_color
            } else {
                text_color.linear_multiply(0.38)
            };

            // M3 state layer (hover overlay)
            if is_hovered && self.enabled {
                let state_layer_color = match self.variant {
                    TabVariant::Primary => primary_color,
                    TabVariant::Secondary => on_surface,
                };
                let hover_color = Color32::from_rgba_unmultiplied(
                    state_layer_color.r(),
                    state_layer_color.g(),
                    state_layer_color.b(),
                    20, // ~8% opacity
                );
                ui.painter().rect_filled(tab_rect, 0.0, hover_color);
            }

            // Handle click
            if tab_response.clicked() && self.enabled {
                *self.selected = index;
                any_clicked = true;
            }

            // Layout and draw tab content
            if let Some(icon) = &tab.icon {
                // Icon + text layout: icon above label
                let icon_y = tab_rect.center().y - 10.0;
                let label_y = tab_rect.center().y + 12.0;

                // Draw icon as text (emoji/unicode)
                ui.painter().text(
                    Pos2::new(tab_rect.center().x, icon_y),
                    egui::Align2::CENTER_CENTER,
                    icon,
                    icon_font.clone(),
                    text_color,
                );

                // Draw label text
                ui.painter().text(
                    Pos2::new(tab_rect.center().x, label_y),
                    egui::Align2::CENTER_CENTER,
                    &tab.label,
                    label_font.clone(),
                    text_color,
                );
            } else {
                // Text-only layout: centered
                ui.painter().text(
                    tab_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &tab.label,
                    label_font.clone(),
                    text_color,
                );
            }

            // Draw indicator for selected tab
            if is_selected && self.enabled {
                match self.variant {
                    TabVariant::Primary => {
                        // M3: indicator width matches label, top-rounded corners
                        let galley = ui.painter().layout_no_wrap(
                            tab.label.clone(),
                            label_font.clone(),
                            text_color,
                        );
                        let label_width = galley.size().x + 16.0; // add padding
                        let indicator_x =
                            tab_rect.center().x - label_width / 2.0;
                        let indicator_rect = Rect::from_min_size(
                            Pos2::new(indicator_x, tab_rect.max.y - PRIMARY_INDICATOR_HEIGHT),
                            Vec2::new(label_width, PRIMARY_INDICATOR_HEIGHT),
                        );
                        let rounding = CornerRadius {
                            nw: INDICATOR_TOP_ROUNDING as u8,
                            ne: INDICATOR_TOP_ROUNDING as u8,
                            sw: 0,
                            se: 0,
                        };
                        ui.painter()
                            .rect_filled(indicator_rect, rounding, primary_color);
                    }
                    TabVariant::Secondary => {
                        // M3: full tab width underline, primary color
                        let indicator_rect = Rect::from_min_size(
                            Pos2::new(tab_rect.min.x, tab_rect.max.y - SECONDARY_INDICATOR_HEIGHT),
                            Vec2::new(tab_width, SECONDARY_INDICATOR_HEIGHT),
                        );
                        ui.painter()
                            .rect_filled(indicator_rect, 0.0, primary_color);
                    }
                }
            }
        }

        // M3: Draw bottom divider for both variants
        let divider_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.max.y - DIVIDER_HEIGHT),
            Vec2::new(rect.width(), DIVIDER_HEIGHT),
        );
        ui.painter().rect_filled(divider_rect, 0.0, outline_variant);

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
