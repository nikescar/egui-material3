use egui::{
    ecolor::Color32, 
    epaint::CornerRadius,
    Rect, Response, Sense, TextStyle, Ui, Vec2, Widget,
};

/// Material Design navigation bar component.
///
/// Navigation bars offer a persistent and convenient way to switch between
/// primary destinations in an app. They typically contain 3-5 navigation tabs.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let mut active_index = 0usize;
/// ui.add(MaterialNavigationBar::new(&mut active_index)
///     .tab("Home", "🏠")
///     .tab("Search", "🔍")
///     .tab("Profile", "👤"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialNavigationBar<'a> {
    active_index: &'a mut usize,
    tabs: Vec<NavigationTab>,
    hide_inactive_labels: bool,
    width: Option<f32>,
}

/// A single navigation tab within a navigation bar.
#[derive(Clone)]
pub struct NavigationTab {
    label: String,
    active_icon: String,
    inactive_icon: Option<String>,
    show_badge: bool,
    badge_value: String,
}

impl NavigationTab {
    /// Create a new navigation tab.
    pub fn new(label: impl Into<String>, icon: impl Into<String>) -> Self {
        let icon_str = icon.into();
        Self {
            label: label.into(),
            active_icon: icon_str.clone(),
            inactive_icon: None,
            show_badge: false,
            badge_value: String::new(),
        }
    }

    /// Set different icons for active and inactive states.
    pub fn icons(mut self, active_icon: impl Into<String>, inactive_icon: impl Into<String>) -> Self {
        self.active_icon = active_icon.into();
        self.inactive_icon = Some(inactive_icon.into());
        self
    }

    /// Show a badge on this tab.
    pub fn badge(mut self, value: impl Into<String>) -> Self {
        self.show_badge = true;
        self.badge_value = value.into();
        self
    }

    /// Hide the badge on this tab.
    pub fn no_badge(mut self) -> Self {
        self.show_badge = false;
        self.badge_value.clear();
        self
    }
}

impl<'a> MaterialNavigationBar<'a> {
    /// Create a new material navigation bar.
    pub fn new(active_index: &'a mut usize) -> Self {
        Self {
            active_index,
            tabs: Vec::new(),
            hide_inactive_labels: false,
            width: None,
        }
    }

    /// Add a navigation tab with label and icon.
    pub fn tab(mut self, label: impl Into<String>, icon: impl Into<String>) -> Self {
        self.tabs.push(NavigationTab::new(label, icon));
        self
    }

    /// Add a navigation tab with separate active/inactive icons.
    pub fn tab_with_icons(mut self, label: impl Into<String>, active_icon: impl Into<String>, inactive_icon: impl Into<String>) -> Self {
        let active_icon_str = active_icon.into();
        let inactive_icon_str = inactive_icon.into();
        self.tabs.push(NavigationTab::new(label, active_icon_str.clone()).icons(active_icon_str, inactive_icon_str));
        self
    }

    /// Add a custom navigation tab.
    pub fn add_tab(mut self, tab: NavigationTab) -> Self {
        self.tabs.push(tab);
        self
    }

    /// Hide labels for inactive tabs.
    pub fn hide_inactive_labels(mut self, hide: bool) -> Self {
        self.hide_inactive_labels = hide;
        self
    }

    /// Set the width of the navigation bar.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for MaterialNavigationBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialNavigationBar {
            active_index,
            tabs,
            hide_inactive_labels,
            width,
        } = self;

        if tabs.is_empty() {
            return ui.label("No tabs").into();
        }

        // Material Design colors
        let md_surface = Color32::from_rgb(254, 247, 255); // md-sys-color-surface
        let md_surface_container = Color32::from_rgb(243, 237, 247); // md-sys-color-surface-container
        let md_on_surface = Color32::from_rgb(28, 27, 31); // md-sys-color-on-surface
        let md_on_surface_variant = Color32::from_rgb(73, 69, 79); // md-sys-color-on-surface-variant
        let md_primary = Color32::from_rgb(103, 80, 164); // md-sys-color-primary
        let md_on_secondary_container = Color32::from_rgb(31, 31, 35); // md-sys-color-on-secondary-container

        // Navigation bar height
        let nav_height = 80.0;
        let tab_width = width.unwrap_or(ui.available_width()) / tabs.len() as f32;

        let desired_size = Vec2::new(width.unwrap_or(ui.available_width()), nav_height);
        let mut response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw background
            ui.painter().rect_filled(rect, CornerRadius::ZERO, md_surface_container);

            // Draw elevation (subtle shadow at top)
            let shadow_rect = Rect::from_min_size(
                rect.min - Vec2::new(0.0, 1.0),
                Vec2::new(rect.width(), 1.0),
            );
            ui.painter().rect_filled(shadow_rect, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(0, 0, 0, 20));

            // Draw tabs
            for (i, tab) in tabs.iter().enumerate() {
                let is_active = i == *active_index;
                let tab_rect = Rect::from_min_size(
                    rect.min + Vec2::new(i as f32 * tab_width, 0.0),
                    Vec2::new(tab_width, nav_height),
                );

                // Check if this tab was clicked
                let tab_response = ui.allocate_rect(tab_rect, Sense::click());
                if tab_response.clicked() {
                    *active_index = i;
                    response = response.union(tab_response.clone());
                }

                // Draw tab background on hover/active
                if is_active {
                    // Active state indicator (rounded rectangle)
                    let indicator_rect = Rect::from_center_size(
                        tab_rect.center() + Vec2::new(0.0, -12.0),
                        Vec2::new(64.0, 32.0),
                    );
                    ui.painter().rect_filled(
                        indicator_rect,
                        CornerRadius::from(16),
                        Color32::from_rgba_unmultiplied(md_primary.r(), md_primary.g(), md_primary.b(), 31),
                    );
                } else if tab_response.hovered() {
                    // Hover state
                    let hover_rect = Rect::from_center_size(
                        tab_rect.center(),
                        Vec2::new(64.0, 32.0),
                    );
                    ui.painter().rect_filled(
                        hover_rect,
                        CornerRadius::from(16),
                        Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 20),
                    );
                }

                // Draw tab content
                ui.scope_builder(egui::UiBuilder::new().max_rect(tab_rect.shrink(8.0)), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);

                        // Icon
                        let icon = if is_active {
                            &tab.active_icon
                        } else {
                            tab.inactive_icon.as_ref().unwrap_or(&tab.active_icon)
                        };

                        let icon_color = if is_active { md_on_secondary_container } else { md_on_surface_variant };
                        
                        ui.horizontal(|ui| {
                            ui.style_mut().override_text_style = Some(TextStyle::Heading);
                            ui.colored_label(icon_color, icon);

                            // Badge
                            if tab.show_badge && !tab.badge_value.is_empty() {
                                ui.add_space(2.0);
                                
                                // Badge background
                                let badge_size = Vec2::new(16.0, 16.0);
                                let (badge_rect, _) = ui.allocate_exact_size(badge_size, Sense::hover());
                                ui.painter().circle_filled(
                                    badge_rect.center(),
                                    8.0,
                                    Color32::from_rgb(186, 26, 26), // md-sys-color-error
                                );
                                
                                // Badge text
                                ui.scope_builder(egui::UiBuilder::new().max_rect(badge_rect), |ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.style_mut().override_text_style = Some(TextStyle::Small);
                                        ui.colored_label(Color32::WHITE, &tab.badge_value);
                                    });
                                });
                            }
                        });

                        ui.add_space(4.0);

                        // Label
                        let show_label = is_active || !hide_inactive_labels;
                        if show_label {
                            ui.style_mut().override_text_style = Some(TextStyle::Small);
                            let label_color = if is_active { md_on_surface } else { md_on_surface_variant };
                            ui.colored_label(label_color, &tab.label);
                        }
                    });
                });
            }
        }

        response
    }
}

/// Convenience function to create a navigation tab.
pub fn navigation_tab(label: impl Into<String>, icon: impl Into<String>) -> NavigationTab {
    NavigationTab::new(label, icon)
}