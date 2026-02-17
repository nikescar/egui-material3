use crate::material_symbol::material_symbol_text;
use crate::theme::get_global_color;
use egui::{
    ecolor::Color32,
    epaint::CornerRadius,
    Rect, Response, Sense, Ui, Vec2, Widget,
};

/// Material Design bottom app bar component.
///
/// Bottom app bars provide access to a bottom navigation drawer and up to four actions,
/// including the floating action button. They are typically used for mobile layouts.
///
/// In Material Design 3, bottom app bars use `surfaceContainer` as the background color
/// and have a default elevation of 3.0dp with a height of 80.0dp.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let bottom_bar = MaterialBottomAppBar::new()
///     .action_icon("search", || println!("Search clicked!"))
///     .action_icon("share", || println!("Share clicked!"))
///     .action_icon("more_vert", || println!("More clicked!"));
///
/// ui.add(bottom_bar);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialBottomAppBar<'a> {
    height: f32,
    padding_horizontal: f32,
    padding_vertical: f32,
    navigation_icon: Option<(String, Box<dyn Fn() + Send + Sync + 'a>)>,
    action_icons: Vec<(String, Box<dyn Fn() + Send + Sync + 'a>)>,
    elevation: f32,
    corner_radius: CornerRadius,
    id_salt: Option<String>,
    background_color: Option<Color32>,
    foreground_color: Option<Color32>,
    surface_tint_color: Option<Color32>,
    shadow_color: Option<Color32>,
    notch_margin: f32,
    show_fab_notch: bool,
}

impl<'a> MaterialBottomAppBar<'a> {
    /// Create a new bottom app bar with default Material 3 styling.
    pub fn new() -> Self {
        Self {
            height: 80.0,
            padding_horizontal: 16.0,
            padding_vertical: 12.0,
            navigation_icon: None,
            action_icons: Vec::new(),
            elevation: 3.0,
            corner_radius: CornerRadius::ZERO,
            id_salt: None,
            background_color: None,
            foreground_color: None,
            surface_tint_color: None,
            shadow_color: None,
            notch_margin: 4.0,
            show_fab_notch: false,
        }
    }

    /// Set a custom height for the bottom app bar.
    /// Default is 80.0 in Material 3.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set horizontal and vertical padding.
    /// Default is (16.0, 12.0) in Material 3.
    pub fn padding(mut self, horizontal: f32, vertical: f32) -> Self {
        self.padding_horizontal = horizontal;
        self.padding_vertical = vertical;
        self
    }

    /// Add a navigation icon (typically on the left side).
    pub fn navigation_icon(
        mut self,
        icon: impl Into<String>,
        on_click: impl Fn() + Send + Sync + 'a,
    ) -> Self {
        self.navigation_icon = Some((icon.into(), Box::new(on_click)));
        self
    }

    /// Add an action icon button.
    /// Actions are displayed from left to right.
    pub fn action_icon(
        mut self,
        icon: impl Into<String>,
        on_click: impl Fn() + Send + Sync + 'a,
    ) -> Self {
        self.action_icons.push((icon.into(), Box::new(on_click)));
        self
    }

    /// Set the elevation (shadow depth).
    /// Default is 3.0 in Material 3.
    pub fn elevation(mut self, elevation: f32) -> Self {
        self.elevation = elevation;
        self
    }

    /// Set a custom background color.
    /// Default is `surfaceContainer` from the theme.
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set a custom foreground color for icons and text.
    /// Default is `onSurface` from the theme.
    pub fn foreground_color(mut self, color: Color32) -> Self {
        self.foreground_color = Some(color);
        self
    }

    /// Set an ID salt for this widget to allow multiple instances.
    pub fn id_salt(mut self, id: impl Into<String>) -> Self {
        self.id_salt = Some(id.into());
        self
    }

    /// Set the surface tint color for elevation overlay.
    /// Default is transparent in Material 3.
    pub fn surface_tint_color(mut self, color: Color32) -> Self {
        self.surface_tint_color = Some(color);
        self
    }

    /// Set the shadow color.
    /// Default is transparent in Material 3.
    pub fn shadow_color(mut self, color: Color32) -> Self {
        self.shadow_color = Some(color);
        self
    }

    /// Enable FAB notch rendering (visual indication for floating action button).
    pub fn fab_notch(mut self, enabled: bool) -> Self {
        self.show_fab_notch = enabled;
        self
    }

    /// Set the margin around the FAB notch.
    /// Default is 4.0.
    pub fn notch_margin(mut self, margin: f32) -> Self {
        self.notch_margin = margin;
        self
    }

    fn get_background_color(&self) -> Color32 {
        self.background_color
            .unwrap_or_else(|| get_global_color("surfaceContainer"))
    }

    fn get_foreground_color(&self) -> Color32 {
        self.foreground_color
            .unwrap_or_else(|| get_global_color("onSurface"))
    }
}

impl<'a> Default for MaterialBottomAppBar<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Widget for MaterialBottomAppBar<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id_source = self.id_salt.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("bottom_app_bar");
        let id = ui.make_persistent_id(id_source);

        let background_color = self.get_background_color();
        let foreground_color = self.get_foreground_color();

        // Calculate total width and height
        let available_width = ui.available_width();
        let total_height = self.height;

        // Allocate space for the bottom app bar
        let (rect, mut response) = ui.allocate_exact_size(
            Vec2::new(available_width, total_height),
            Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            // Draw elevation shadow if enabled
            if self.elevation > 0.0 {
                let shadow_offset = -2.0;
                let shadow_rect = rect.translate(Vec2::new(0.0, shadow_offset));
                let shadow_color = self.shadow_color.unwrap_or(
                    Color32::from_rgba_unmultiplied(0, 0, 0, (self.elevation * 7.0).min(50.0) as u8)
                );
                ui.painter().rect_filled(
                    shadow_rect,
                    self.corner_radius,
                    shadow_color,
                );
            }

            // Draw background
            ui.painter().rect_filled(rect, self.corner_radius, background_color);

            // Content area with padding
            let content_rect = Rect::from_min_max(
                rect.min + Vec2::new(self.padding_horizontal, self.padding_vertical),
                rect.max - Vec2::new(self.padding_horizontal, self.padding_vertical),
            );

            let mut ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
            ui.set_clip_rect(content_rect);

            // Layout: [navigation_icon] [spacer] [actions...]
            let icon_size = 48.0;
            let icon_spacing = 8.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = icon_spacing;

                // Navigation icon (left side)
                if let Some((icon_name, on_click)) = self.navigation_icon {
                    let icon_response = ui.allocate_ui(Vec2::new(icon_size, icon_size), |ui| {
                        let (rect, sense_response) = ui.allocate_exact_size(
                            Vec2::splat(icon_size),
                            Sense::click(),
                        );

                        // Draw icon button background on hover/click
                        if sense_response.hovered() || sense_response.is_pointer_button_down_on() {
                            let bg_color = if sense_response.is_pointer_button_down_on() {
                                Color32::from_rgba_unmultiplied(
                                    foreground_color.r(),
                                    foreground_color.g(),
                                    foreground_color.b(),
                                    30,
                                )
                            } else {
                                Color32::from_rgba_unmultiplied(
                                    foreground_color.r(),
                                    foreground_color.g(),
                                    foreground_color.b(),
                                    20,
                                )
                            };
                            ui.painter()
                                .circle_filled(rect.center(), icon_size / 2.0, bg_color);
                        }

                        // Draw icon
                        let icon_text = material_symbol_text(&icon_name);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            icon_text,
                            egui::FontId::proportional(24.0),
                            foreground_color,
                        );

                        sense_response
                    });

                    if icon_response.inner.clicked() {
                        on_click();
                    }
                }

                // Spacer to push actions to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Action icons (right side)
                    for (icon_name, on_click) in self.action_icons.into_iter().rev() {
                        let icon_response = ui.allocate_ui(Vec2::new(icon_size, icon_size), |ui| {
                            let (rect, sense_response) = ui.allocate_exact_size(
                                Vec2::splat(icon_size),
                                Sense::click(),
                            );

                            // Draw icon button background on hover/click
                            if sense_response.hovered() || sense_response.is_pointer_button_down_on()
                            {
                                let bg_color = if sense_response.is_pointer_button_down_on() {
                                    Color32::from_rgba_unmultiplied(
                                        foreground_color.r(),
                                        foreground_color.g(),
                                        foreground_color.b(),
                                        30,
                                    )
                                } else {
                                    Color32::from_rgba_unmultiplied(
                                        foreground_color.r(),
                                        foreground_color.g(),
                                        foreground_color.b(),
                                        20,
                                    )
                                };
                                ui.painter()
                                    .circle_filled(rect.center(), icon_size / 2.0, bg_color);
                            }

                            // Draw icon
                            let icon_text = material_symbol_text(&icon_name);
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                icon_text,
                                egui::FontId::proportional(24.0),
                                foreground_color,
                            );

                            sense_response
                        });

                        if icon_response.inner.clicked() {
                            on_click();
                            response.mark_changed();
                        }
                    }

                    // FAB notch indicator (if enabled)
                    if self.show_fab_notch {
                        ui.add_space(16.0);
                        let notch_size = 56.0 + self.notch_margin * 2.0;
                        let (notch_rect, _) = ui.allocate_exact_size(
                            Vec2::new(notch_size, icon_size),
                            Sense::hover(),
                        );
                        
                        // Draw a subtle cutout indicator
                        ui.painter().circle_stroke(
                            notch_rect.center(),
                            28.0,
                            (1.0, foreground_color.linear_multiply(0.3)),
                        );
                    }
                });
            });
        }

        response
    }
}

/// Helper function to create a bottom app bar.
pub fn bottom_app_bar<'a>() -> MaterialBottomAppBar<'a> {
    MaterialBottomAppBar::new()
}
