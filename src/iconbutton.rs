use crate::get_global_color;
use eframe::egui::{Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Visual variants for the icon button component.
#[derive(Clone, Copy, PartialEq)]
pub enum IconButtonVariant {
    /// Standard icon button (minimal visual emphasis)
    Standard,
    /// Filled icon button (high emphasis with filled background)
    Filled,
    /// Filled tonal icon button (medium emphasis with tonal background)
    FilledTonal,
    /// Outlined icon button (medium emphasis with border)
    Outlined,
}

/// Material Design icon button component.
///
/// Icon buttons help users take supplementary actions with a single tap.
/// They're used when a compact button is required.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// // Standard icon button
/// if ui.add(MaterialIconButton::standard("favorite")).clicked() {
///     println!("Favorite clicked!");
/// }
///
/// // Filled icon button with toggle state
/// let mut liked = false;
/// ui.add(MaterialIconButton::filled("favorite")
///     .toggle(&mut liked)
///     .size(48.0));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialIconButton<'a> {
    /// Icon identifier (e.g., "favorite", "settings", "delete")
    icon: String,
    /// Visual variant of the button
    variant: IconButtonVariant,
    /// Optional toggle state for the button
    selected: Option<&'a mut bool>,
    /// Whether the button is enabled for interaction
    enabled: bool,
    /// Size of the button (width and height)
    size: f32,
    /// Whether to use rectangular container (true) or circular (false)
    container: bool,
    /// Optional callback to execute when clicked
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialIconButton<'a> {
    /// Create a new icon button with the specified variant.
    ///
    /// # Arguments
    /// * `icon` - Icon identifier (e.g., "home", "settings", "delete")
    /// * `variant` - Visual variant of the button
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let button = MaterialIconButton::new("settings", IconButtonVariant::Outlined);
    /// # });
    /// ```
    pub fn new(icon: impl Into<String>, variant: IconButtonVariant) -> Self {
        Self {
            icon: icon.into(),
            variant,
            selected: None,
            enabled: true,
            size: 40.0,
            container: false, // circular by default
            action: None,
        }
    }

    /// Create a standard icon button (minimal visual emphasis).
    ///
    /// # Arguments
    /// * `icon` - Icon identifier
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::standard("menu"));
    /// # });
    /// ```
    pub fn standard(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::Standard)
    }

    /// Create a filled icon button (high emphasis with filled background).
    ///
    /// # Arguments
    /// * `icon` - Icon identifier
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::filled("add"));
    /// # });
    /// ```
    pub fn filled(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::Filled)
    }

    /// Create a filled tonal icon button (medium emphasis with tonal background).
    ///
    /// # Arguments
    /// * `icon` - Icon identifier
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::filled_tonal("edit"));
    /// # });
    /// ```
    pub fn filled_tonal(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::FilledTonal)
    }

    /// Create an outlined icon button (medium emphasis with border).
    ///
    /// # Arguments
    /// * `icon` - Icon identifier
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::outlined("delete"));
    /// # });
    /// ```
    pub fn outlined(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::Outlined)
    }

    /// Create a toggleable icon button.
    ///
    /// The button's appearance will change based on the `selected` state.
    ///
    /// # Arguments
    /// * `icon` - Icon identifier
    /// * `selected` - Mutable reference to the toggle state
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut is_favorite = false;
    /// ui.add(MaterialIconButton::toggle("favorite", &mut is_favorite));
    /// # });
    /// ```
    pub fn toggle(icon: impl Into<String>, selected: &'a mut bool) -> Self {
        let mut button = Self::standard(icon);
        button.selected = Some(selected);
        button
    }

    /// Set the size of the icon button.
    ///
    /// # Arguments
    /// * `size` - Desired size (width and height) of the button
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::standard("settings").size(48.0));
    /// # });
    /// ```
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Enable or disable the icon button.
    ///
    /// # Arguments
    /// * `enabled` - `true` to enable the button, `false` to disable
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::standard("download").enabled(false));
    /// # });
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the container style of the icon button.
    ///
    /// # Arguments
    /// * `container` - `true` for rectangular container, `false` for circular
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::standard("share").container(true));
    /// # });
    /// ```
    pub fn container(mut self, container: bool) -> Self {
        self.container = container;
        self
    }

    /// Set the click action for the icon button.
    ///
    /// # Arguments
    /// * `f` - Function to execute when the button is clicked
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// ui.add(MaterialIconButton::standard("info").on_click(|| {
    ///     println!("Info button clicked!");
    /// }));
    /// # });
    /// ```
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

impl<'a> Widget for MaterialIconButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        let is_selected = self.selected.as_ref().map_or(false, |s| **s);

        if response.clicked() && self.enabled {
            if let Some(selected) = self.selected {
                *selected = !*selected;
                response.mark_changed();
            }
            if let Some(action) = self.action {
                action();
            }
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
        let secondary_container = get_global_color("secondaryContainer");
        let on_secondary_container = get_global_color("onSecondaryContainer");
        let _surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let (bg_color, icon_color, border_color) = if !self.enabled {
            (
                get_global_color("surfaceContainer"),
                get_global_color("outline"),
                Color32::TRANSPARENT,
            )
        } else {
            match self.variant {
                IconButtonVariant::Standard => {
                    if is_selected {
                        (Color32::TRANSPARENT, primary_color, Color32::TRANSPARENT)
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                on_surface.r(),
                                on_surface.g(),
                                on_surface.b(),
                                20,
                            ),
                            on_surface,
                            Color32::TRANSPARENT,
                        )
                    } else {
                        (
                            Color32::TRANSPARENT,
                            on_surface_variant,
                            Color32::TRANSPARENT,
                        )
                    }
                }
                IconButtonVariant::Filled => {
                    if is_selected {
                        (
                            primary_color,
                            get_global_color("onPrimary"),
                            Color32::TRANSPARENT,
                        )
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                primary_color.r().saturating_add(20),
                                primary_color.g().saturating_add(20),
                                primary_color.b().saturating_add(20),
                                255,
                            ),
                            get_global_color("onPrimary"),
                            Color32::TRANSPARENT,
                        )
                    } else {
                        (primary_color, Color32::WHITE, Color32::TRANSPARENT)
                    }
                }
                IconButtonVariant::FilledTonal => {
                    if is_selected {
                        (
                            secondary_container,
                            on_secondary_container,
                            Color32::TRANSPARENT,
                        )
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                secondary_container.r().saturating_sub(10),
                                secondary_container.g().saturating_sub(10),
                                secondary_container.b().saturating_sub(10),
                                255,
                            ),
                            on_secondary_container,
                            Color32::TRANSPARENT,
                        )
                    } else {
                        (
                            secondary_container,
                            on_secondary_container,
                            Color32::TRANSPARENT,
                        )
                    }
                }
                IconButtonVariant::Outlined => {
                    if is_selected {
                        (
                            Color32::from_rgba_premultiplied(
                                primary_color.r(),
                                primary_color.g(),
                                primary_color.b(),
                                24,
                            ),
                            primary_color,
                            primary_color,
                        )
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                on_surface.r(),
                                on_surface.g(),
                                on_surface.b(),
                                20,
                            ),
                            on_surface_variant,
                            outline,
                        )
                    } else {
                        (Color32::TRANSPARENT, on_surface_variant, outline)
                    }
                }
            }
        };

        // Calculate corner radius based on container style
        let corner_radius = if self.container {
            // Rectangular container: smaller radius for more rectangular shape
            rect.height() * 0.2 // About 8px for 40px button
        } else {
            // Circular container: full radius
            rect.height() / 2.0
        };

        // Draw background
        if bg_color != Color32::TRANSPARENT {
            ui.painter().rect_filled(rect, corner_radius, bg_color);
        }

        // Draw border for outlined variant
        if border_color != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                corner_radius,
                Stroke::new(1.0, border_color),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Draw icon using our icon system
        let icon_size = self.size * 0.6;
        let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));

        let icon_widget = crate::icon::MaterialIcon::new(&self.icon)
            .size(icon_size)
            .color(icon_color);

        ui.scope_builder(egui::UiBuilder::new().max_rect(icon_rect), |ui| {
            ui.add(icon_widget);
        });

        // Add ripple effect on hover
        if response.hovered() && self.enabled {
            let ripple_color = Color32::from_rgba_premultiplied(
                icon_color.r(),
                icon_color.g(),
                icon_color.b(),
                30,
            );
            ui.painter().rect_filled(rect, corner_radius, ripple_color);
        }

        response
    }
}

/// Convenience function to create a standard icon button.
///
/// # Arguments
/// * `icon` - Icon identifier
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(icon_button_standard("menu"));
/// # });
/// ```
pub fn icon_button_standard(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::standard(icon)
}

/// Convenience function to create a filled icon button.
///
/// # Arguments
/// * `icon` - Icon identifier
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(icon_button_filled("add"));
/// # });
/// ```
pub fn icon_button_filled(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::filled(icon)
}

/// Convenience function to create a filled tonal icon button.
///
/// # Arguments
/// * `icon` - Icon identifier
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(icon_button_filled_tonal("edit"));
/// # });
/// ```
pub fn icon_button_filled_tonal(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::filled_tonal(icon)
}

/// Convenience function to create an outlined icon button.
///
/// # Arguments
/// * `icon` - Icon identifier
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(icon_button_outlined("delete"));
/// # });
/// ```
pub fn icon_button_outlined(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::outlined(icon)
}

/// Convenience function to create a toggleable icon button.
///
/// # Arguments
/// * `icon` - Icon identifier
/// * `selected` - Mutable reference to the toggle state
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut is_liked = false;
/// ui.add(icon_button_toggle("favorite", &mut is_liked));
/// # });
/// ```
pub fn icon_button_toggle(icon: impl Into<String>, selected: &mut bool) -> MaterialIconButton<'_> {
    MaterialIconButton::toggle(icon, selected)
}
