use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use crate::get_global_color;

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
pub struct MaterialRadio<'a> {
    /// Reference to the selected value
    selected: &'a mut Option<usize>,
    /// Value this radio button represents
    value: usize,
    /// Text label for the radio button
    text: String,
    /// Whether the radio button is enabled
    enabled: bool,
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
pub struct MaterialRadioGroup<'a> {
    /// Reference to the selected value
    selected: &'a mut Option<usize>,
    /// List of available options
    options: Vec<RadioOption>,
    /// Whether the entire group is enabled
    enabled: bool,
}

/// Individual radio option data.
pub struct RadioOption {
    /// Display text for the option
    text: String,
    /// Unique value identifying this option
    value: usize,
}

impl<'a> MaterialRadio<'a> {
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
    pub fn new(selected: &'a mut Option<usize>, value: usize, text: impl Into<String>) -> Self {
        Self {
            selected,
            value,
            text: text.into(),
            enabled: true,
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
}

impl<'a> Widget for MaterialRadio<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(
            ui.available_width().min(300.0),
            24.0,
        );

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        let is_selected = self.selected.map_or(false, |s| s == self.value);

        if response.clicked() && self.enabled {
            *self.selected = Some(self.value);
            response.mark_changed();
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
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
            (primary_color, Color32::TRANSPARENT, primary_color)
        } else if response.hovered() {
            (outline, Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20), on_surface_variant)
        } else {
            (outline, Color32::TRANSPARENT, on_surface_variant)
        };

        // Draw hover background
        if fill_color != Color32::TRANSPARENT {
            ui.painter().circle_filled(radio_rect.center(), radio_size / 2.0 + 8.0, fill_color);
        }

        // Draw radio border
        ui.painter().circle_stroke(
            radio_rect.center(),
            radio_size / 2.0,
            Stroke::new(2.0, border_color),
        );

        // Draw selected inner circle
        if is_selected {
            ui.painter().circle_filled(
                radio_rect.center(),
                radio_size / 4.0,
                inner_color,
            );
        }

        // Draw label text
        if !self.text.is_empty() {
            let text_pos = Pos2::new(
                radio_rect.max.x + 8.0,
                rect.center().y,
            );
            
            let text_color = if self.enabled { on_surface } else {
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
            let ripple_color = if is_selected {
                Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 20)
            } else {
                Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20)
            };
            
            ui.painter().circle_filled(
                radio_rect.center(),
                radio_size / 2.0 + 12.0,
                ripple_color,
            );
        }

        response
    }
}

impl<'a> MaterialRadioGroup<'a> {
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
    pub fn new(selected: &'a mut Option<usize>) -> Self {
        Self {
            selected,
            options: Vec::new(),
            enabled: true,
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
    pub fn option(mut self, value: usize, text: impl Into<String>) -> Self {
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
}

impl<'a> Widget for MaterialRadioGroup<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut group_response = None;
        
        ui.vertical(|ui| {
            for option in self.options {
                let radio = MaterialRadio::new(self.selected, option.value, option.text)
                    .enabled(self.enabled);
                
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

/// Convenience function to create a radio button.
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
pub fn radio<'a>(selected: &'a mut Option<usize>, value: usize, text: impl Into<String>) -> MaterialRadio<'a> {
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
pub fn radio_group<'a>(selected: &'a mut Option<usize>) -> MaterialRadioGroup<'a> {
    MaterialRadioGroup::new(selected)
}