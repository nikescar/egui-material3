use crate::get_global_color;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget, FontId};

/// Material Design radio button component.
///
/// Radio buttons allow users to select one option from a set.
/// Use radio buttons when only one option may be selected.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selected = Some(0);
///
/// ui.add(MaterialRadio::new(&mut selected, 0, "Option 1"));
/// ui.add(MaterialRadio::new(&mut selected, 1, "Option 2"));
/// ui.add(MaterialRadio::new(&mut selected, 2, "Option 3"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialRadio<'a, T: PartialEq + Clone> {
    /// Reference to the selected value
    selected: &'a mut Option<T>,
    /// Value this radio button represents
    value: T,
    /// Text label for the radio button
    text: String,
    /// Whether the radio button is enabled
    enabled: bool,
    /// Whether the radio can be toggled off when clicked while selected
    toggleable: bool,
    /// Custom fill color for the radio button
    fill_color: Option<Color32>,
    /// Custom overlay color for hover/press effects
    overlay_color: Option<Color32>,
    /// Custom background color
    background_color: Option<Color32>,
    /// Custom inner radius
    inner_radius: Option<f32>,
    /// Custom splash radius for ripple effect
    splash_radius: Option<f32>,
}

/// Material Design radio button group component.
///
/// A convenience wrapper for managing multiple radio buttons as a group.
/// Ensures only one option can be selected at a time.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selected = Some(0);
/// let mut group = MaterialRadioGroup::new(&mut selected)
///     .option(0, "First Option")
///     .option(1, "Second Option")
///     .option(2, "Third Option");
///
/// ui.add(group);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialRadioGroup<'a, T: PartialEq + Clone> {
    /// Reference to the selected value
    selected: &'a mut Option<T>,
    /// List of available options
    options: Vec<RadioOption<T>>,
    /// Whether the entire group is enabled
    enabled: bool,
    /// Whether radios can be toggled off
    toggleable: bool,
}

/// Individual radio option data.
pub struct RadioOption<T: PartialEq + Clone> {
    /// Display text for the option
    text: String,
    /// Unique value identifying this option
    value: T,
}

impl<'a, T: PartialEq + Clone> MaterialRadio<'a, T> {
    /// Create a new radio button.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected value
    /// * `value` - The value this radio button represents
    /// * `text` - The text label to display
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = Some(1);
    /// ui.add(MaterialRadio::new(&mut selection, 1, "My Option"));
    /// # });
    /// ```
    pub fn new(selected: &'a mut Option<T>, value: T, text: impl Into<String>) -> Self {
        Self {
            selected,
            value,
            text: text.into(),
            enabled: true,
            toggleable: false,
            fill_color: None,
            overlay_color: None,
            background_color: None,
            inner_radius: None,
            splash_radius: None,
        }
    }

    /// Set whether the radio button is enabled.
    ///
    /// # Arguments
    /// * `enabled` - Whether the radio button should respond to interactions
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialRadio::new(&mut selection, 0, "Disabled Option")
    ///     .enabled(false));
    /// # });
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set whether the radio can be deselected by clicking when already selected.
    ///
    /// When true, clicking a selected radio button will deselect it.
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    /// Set custom fill color for the radio button.
    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set custom overlay color for hover/press effects.
    pub fn overlay_color(mut self, color: Color32) -> Self {
        self.overlay_color = Some(color);
        self
    }

    /// Set custom background color.
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set custom inner circle radius.
    pub fn inner_radius(mut self, radius: f32) -> Self {
        self.inner_radius = Some(radius);
        self
    }

    /// Set custom splash radius for ripple effects.
    pub fn splash_radius(mut self, radius: f32) -> Self {
        self.splash_radius = Some(radius);
        self
    }
}

impl<'a, T: PartialEq + Clone> Widget for MaterialRadio<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(ui.available_width().min(300.0), 24.0);

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        let is_selected = self.selected.as_ref().map_or(false, |s| s == &self.value);

        if response.clicked() && self.enabled {
            if self.toggleable && is_selected {
                // Deselect if toggleable and already selected
                *self.selected = None;
            } else {
                // Select this value
                *self.selected = Some(self.value.clone());
            }
            response.mark_changed();
        }

        // Material Design colors
        let primary_color = self.fill_color.unwrap_or_else(|| get_global_color("primary"));
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let radio_size = 20.0;
        let radio_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - radio_size / 2.0),
            Vec2::splat(radio_size),
        );

        let (border_color, fill_color, inner_color) = if !self.enabled {
            let disabled_color = get_global_color("onSurfaceVariant").linear_multiply(0.38);
            (disabled_color, Color32::TRANSPARENT, disabled_color)
        } else if is_selected {
            (primary_color, self.background_color.unwrap_or(Color32::TRANSPARENT), primary_color)
        } else if response.hovered() {
            let hover_overlay = self.overlay_color.unwrap_or_else(|| 
                Color32::from_rgba_premultiplied(
                    on_surface.r(),
                    on_surface.g(),
                    on_surface.b(),
                    20,
                )
            );
            (
                outline,
                hover_overlay,
                on_surface_variant,
            )
        } else {
            (outline, self.background_color.unwrap_or(Color32::TRANSPARENT), on_surface_variant)
        };

        // Draw hover background
        if fill_color != Color32::TRANSPARENT {
            ui.painter()
                .circle_filled(radio_rect.center(), radio_size / 2.0 + 8.0, fill_color);
        }

        // Draw radio border
        ui.painter().circle_stroke(
            radio_rect.center(),
            radio_size / 2.0,
            Stroke::new(2.0, border_color),
        );

        // Draw selected inner circle
        if is_selected {
            let inner_radius = self.inner_radius.unwrap_or(radio_size / 4.0);
            ui.painter()
                .circle_filled(radio_rect.center(), inner_radius, inner_color);
        }

        // Draw label text
        if !self.text.is_empty() {
            let text_pos = Pos2::new(radio_rect.max.x + 8.0, rect.center().y);

            let text_color = if self.enabled {
                on_surface
            } else {
                get_global_color("onSurfaceVariant").linear_multiply(0.38)
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                &self.text,
                egui::FontId::default(),
                text_color,
            );
        }

        // Add ripple effect on hover
        if response.hovered() && self.enabled {
            let ripple_color = self.overlay_color.unwrap_or_else(|| {
                if is_selected {
                    Color32::from_rgba_premultiplied(
                        primary_color.r(),
                        primary_color.g(),
                        primary_color.b(),
                        20,
                    )
                } else {
                    Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20)
                }
            });

            let ripple_radius = self.splash_radius.unwrap_or(radio_size / 2.0 + 12.0);
            ui.painter()
                .circle_filled(radio_rect.center(), ripple_radius, ripple_color);
        }

        response
    }
}

impl<'a, T: PartialEq + Clone> MaterialRadioGroup<'a, T> {
    /// Create a new radio button group.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected value
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = Some(1);
    /// let group = MaterialRadioGroup::new(&mut selection);
    /// # });
    /// ```
    pub fn new(selected: &'a mut Option<T>) -> Self {
        Self {
            selected,
            options: Vec::new(),
            enabled: true,
            toggleable: false,
        }
    }

    /// Add an option to the radio group.
    ///
    /// # Arguments
    /// * `value` - The value this option represents
    /// * `text` - The text label for this option
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// let group = MaterialRadioGroup::new(&mut selection)
    ///     .option(0, "First Choice")
    ///     .option(1, "Second Choice");
    /// # });
    /// ```
    pub fn option(mut self, value: T, text: impl Into<String>) -> Self {
        self.options.push(RadioOption {
            text: text.into(),
            value,
        });
        self
    }

    /// Set whether the entire radio group is enabled.
    ///
    /// # Arguments
    /// * `enabled` - Whether all radio buttons in the group should respond to interactions
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// let group = MaterialRadioGroup::new(&mut selection)
    ///     .option(0, "Option 1")
    ///     .enabled(false); // Disable all options
    /// # });
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set whether radios in the group can be toggled off.
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }
}

impl<'a, T: PartialEq + Clone> Widget for MaterialRadioGroup<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut group_response = None;

        ui.vertical(|ui| {
            for option in self.options {
                let radio = MaterialRadio::new(self.selected, option.value, option.text)
                    .enabled(self.enabled)
                    .toggleable(self.toggleable);

                let response = ui.add(radio);

                if group_response.is_none() {
                    group_response = Some(response);
                } else if let Some(ref mut group_resp) = group_response {
                    *group_resp = group_resp.union(response);
                }
            }
        });

        group_response.unwrap_or_else(|| {
            let (_rect, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::hover());
            response
        })
    }
}

/// Control affinity for RadioListTile - determines radio button position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListTileControlAffinity {
    /// Radio button appears before the title (leading edge)
    Leading,
    /// Radio button appears after the title (trailing edge)
    Trailing,
}

/// Material Design radio list tile component.
///
/// Combines a radio button with list tile functionality, including title, subtitle,
/// and secondary widgets. The entire tile is interactive.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selected = Some(0);
///
/// ui.add(RadioListTile::new(&mut selected, 0)
///     .title("First Option")
///     .subtitle("Description of first option"));
/// ui.add(RadioListTile::new(&mut selected, 1)
///     .title("Second Option")
///     .subtitle("Description of second option"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct RadioListTile<'a, T: PartialEq + Clone> {
    /// Reference to the selected value
    selected: &'a mut Option<T>,
    /// Value this radio button represents
    value: T,
    /// Primary title text
    title: Option<String>,
    /// Secondary subtitle text
    subtitle: Option<String>,
    /// Whether the radio list tile is enabled
    enabled: bool,
    /// Whether the radio can be toggled off
    toggleable: bool,
    /// Control affinity (radio position)
    control_affinity: ListTileControlAffinity,
    /// Whether to use dense/compact layout
    dense: bool,
    /// Custom fill color
    fill_color: Option<Color32>,
    /// Tile background color
    tile_color: Option<Color32>,
    /// Selected tile background color
    selected_tile_color: Option<Color32>,
}

impl<'a, T: PartialEq + Clone> RadioListTile<'a, T> {
    /// Create a new radio list tile.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected value
    /// * `value` - The value this radio list tile represents
    pub fn new(selected: &'a mut Option<T>, value: T) -> Self {
        Self {
            selected,
            value,
            title: None,
            subtitle: None,
            enabled: true,
            toggleable: false,
            control_affinity: ListTileControlAffinity::Leading,
            dense: false,
            fill_color: None,
            tile_color: None,
            selected_tile_color: None,
        }
    }

    /// Set the primary title text.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the secondary subtitle text.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set whether the radio list tile is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set whether the radio can be toggled off when clicked while selected.
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    /// Set the control affinity (radio button position).
    pub fn control_affinity(mut self, affinity: ListTileControlAffinity) -> Self {
        self.control_affinity = affinity;
        self
    }

    /// Set whether to use dense/compact layout.
    pub fn dense(mut self, dense: bool) -> Self {
        self.dense = dense;
        self
    }

    /// Set custom fill color for the radio button.
    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set tile background color.
    pub fn tile_color(mut self, color: Color32) -> Self {
        self.tile_color = Some(color);
        self
    }

    /// Set selected tile background color.
    pub fn selected_tile_color(mut self, color: Color32) -> Self {
        self.selected_tile_color = Some(color);
        self
    }
}

impl<'a, T: PartialEq + Clone> Widget for RadioListTile<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let is_selected = self.selected.as_ref().map_or(false, |s| s == &self.value);
        
        // Calculate dimensions
        let height = if self.dense {
            if self.subtitle.is_some() { 48.0 } else { 40.0 }
        } else {
            if self.subtitle.is_some() { 64.0 } else { 48.0 }
        };
        
        let available_width = ui.available_width();
        let desired_size = Vec2::new(available_width, height);
        
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
        
        // Handle click
        if response.clicked() && self.enabled {
            if self.toggleable && is_selected {
                *self.selected = None;
            } else {
                *self.selected = Some(self.value.clone());
            }
            response.mark_changed();
        }
        
        // Determine colors
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let surface_variant = get_global_color("surfaceVariant");
        
        // Background
        let bg_color = if is_selected {
            self.selected_tile_color.unwrap_or_else(|| 
                surface_variant.linear_multiply(0.5)
            )
        } else if response.hovered() && self.enabled {
            self.tile_color.unwrap_or_else(|| 
                Color32::from_rgba_premultiplied(
                    on_surface.r(),
                    on_surface.g(),
                    on_surface.b(),
                    10,
                )
            )
        } else {
            self.tile_color.unwrap_or(Color32::TRANSPARENT)
        };
        
        if bg_color != Color32::TRANSPARENT {
            ui.painter().rect_filled(rect, 4.0, bg_color);
        }
        
        // Radio button dimensions
        let radio_size = 20.0;
        let padding = 16.0;
        let gap = 16.0;
        
        // Calculate positions based on control affinity
        let (radio_x, text_x) = match self.control_affinity {
            ListTileControlAffinity::Leading => {
                let radio_x = rect.min.x + padding + radio_size / 2.0;
                let text_x = radio_x + radio_size / 2.0 + gap;
                (radio_x, text_x)
            }
            ListTileControlAffinity::Trailing => {
                let radio_x = rect.max.x - padding - radio_size / 2.0;
                let text_x = rect.min.x + padding;
                (radio_x, text_x)
            }
        };
        
        let radio_center = Pos2::new(radio_x, rect.center().y);
        
        // Draw radio button
        let primary_color = self.fill_color.unwrap_or_else(|| get_global_color("primary"));
        let outline = get_global_color("outline");
        
        let (border_color, inner_color) = if !self.enabled {
            let disabled_color = on_surface_variant.linear_multiply(0.38);
            (disabled_color, disabled_color)
        } else if is_selected {
            (primary_color, primary_color)
        } else {
            (outline, outline)
        };
        
        // Draw radio outer circle
        ui.painter().circle_stroke(
            radio_center,
            radio_size / 2.0,
            Stroke::new(2.0, border_color),
        );
        
        // Draw selected inner circle
        if is_selected {
            ui.painter().circle_filled(radio_center, radio_size / 4.0, inner_color);
        }
        
        // Draw text content
        let text_color = if self.enabled {
            on_surface
        } else {
            on_surface_variant.linear_multiply(0.38)
        };
        
        let text_rect_width = match self.control_affinity {
            ListTileControlAffinity::Leading => rect.max.x - text_x - padding,
            ListTileControlAffinity::Trailing => radio_x - radio_size / 2.0 - gap - text_x,
        };
        
        if let Some(title) = &self.title {
            let title_y = if self.subtitle.is_some() {
                rect.min.y + height * 0.35
            } else {
                rect.center().y
            };
            
            let title_font = if self.dense {
                FontId::proportional(14.0)
            } else {
                FontId::proportional(16.0)
            };
            
            ui.painter().text(
                Pos2::new(text_x, title_y),
                egui::Align2::LEFT_CENTER,
                title,
                title_font,
                text_color,
            );
        }
        
        if let Some(subtitle) = &self.subtitle {
            let subtitle_y = rect.min.y + height * 0.65;
            let subtitle_font = FontId::proportional(if self.dense { 12.0 } else { 14.0 });
            
            ui.painter().text(
                Pos2::new(text_x, subtitle_y),
                egui::Align2::LEFT_CENTER,
                subtitle,
                subtitle_font,
                on_surface_variant,
            );
        }
        
        response
    }
}

/// Convenience function to create a radio list tile.
///
/// Shorthand for `MaterialRadio::new()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected value
/// * `value` - The value this radio button represents
/// * `text` - The text label to display
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selection = Some(0);
/// ui.add(radio(&mut selection, 0, "First Option"));
/// ui.add(radio(&mut selection, 1, "Second Option"));
/// # });
/// ```
pub fn radio<'a, T: PartialEq + Clone>(
    selected: &'a mut Option<T>,
    value: T,
    text: impl Into<String>,
) -> MaterialRadio<'a, T> {
    MaterialRadio::new(selected, value, text)
}

/// Convenience function to create a radio button group.
///
/// Shorthand for `MaterialRadioGroup::new()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected value
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selection = None;
/// ui.add(radio_group(&mut selection)
///     .option(0, "Option A")
///     .option(1, "Option B"));
/// # });
/// ```
pub fn radio_group<'a, T: PartialEq + Clone>(selected: &'a mut Option<T>) -> MaterialRadioGroup<'a, T> {
    MaterialRadioGroup::new(selected)
}

/// Convenience function to create a radio list tile.
///
/// Shorthand for `RadioListTile::new()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected value
/// * `value` - The value this radio list tile represents
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selection = Some(0);
/// ui.add(radio_list_tile(&mut selection, 0)
///     .title("Option One")
///     .subtitle("First choice"));
/// # });
/// ```
pub fn radio_list_tile<'a, T: PartialEq + Clone>(
    selected: &'a mut Option<T>,
    value: T,
) -> RadioListTile<'a, T> {
    RadioListTile::new(selected, value)
}
