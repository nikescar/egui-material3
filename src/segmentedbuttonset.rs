use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, TextStyle, Ui, Vec2, Widget,
};
use crate::get_global_color;

/// Material Design segmented button set component.
///
/// Segmented buttons help people select options, switch views, or sort elements.
/// They typically contain 2-5 segments that can be used for single or multi-selection.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let mut selected = vec![false, true, false];
/// ui.add(MaterialSegmentedButtonSet::new(&mut selected)
///     .button("Option 1", Some("⭐"))
///     .button("Option 2", Some("❤️"))
///     .button("Option 3", Some("🔥")));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialSegmentedButtonSet<'a> {
    selected_states: &'a mut Vec<bool>,
    buttons: Vec<SegmentedButton>,
    multiselect: bool,
    width: Option<f32>,
}

/// A single button within a segmented button set.
#[derive(Clone)]
pub struct SegmentedButton {
    label: Option<String>,
    icon: Option<String>,
    disabled: bool,
    no_checkmark: bool,
}

impl SegmentedButton {
    /// Create a new segmented button.
    pub fn new() -> Self {
        Self {
            label: None,
            icon: None,
            disabled: false,
            no_checkmark: false,
        }
    }

    /// Set the button label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the button icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set whether the button is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Hide the checkmark when selected (for icon-only buttons).
    pub fn no_checkmark(mut self, no_checkmark: bool) -> Self {
        self.no_checkmark = no_checkmark;
        self
    }
}

impl Default for SegmentedButton {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> MaterialSegmentedButtonSet<'a> {
    /// Create a new material segmented button set.
    pub fn new(selected_states: &'a mut Vec<bool>) -> Self {
        Self {
            selected_states,
            buttons: Vec::new(),
            multiselect: false,
            width: None,
        }
    }

    /// Add a button with label and optional icon.
    pub fn button(mut self, label: impl Into<String>, icon: Option<&str>) -> Self {
        let mut button = SegmentedButton::new().label(label);
        if let Some(icon_str) = icon {
            button = button.icon(icon_str);
        }
        self.buttons.push(button);
        self
    }

    /// Add a button with only an icon.
    pub fn icon_button(mut self, icon: impl Into<String>, no_checkmark: bool) -> Self {
        self.buttons.push(SegmentedButton::new()
            .icon(icon)
            .no_checkmark(no_checkmark));
        self
    }

    /// Add a custom segmented button.
    pub fn add_button(mut self, button: SegmentedButton) -> Self {
        self.buttons.push(button);
        self
    }

    /// Enable multi-selection mode.
    pub fn multiselect(mut self, multiselect: bool) -> Self {
        self.multiselect = multiselect;
        self
    }

    /// Set the width of the button set.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    fn ensure_selected_states_size(&mut self) {
        while self.selected_states.len() < self.buttons.len() {
            self.selected_states.push(false);
        }
    }
}

impl Widget for MaterialSegmentedButtonSet<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        // Ensure we have enough selected states first
        self.ensure_selected_states_size();
        
        let MaterialSegmentedButtonSet {
            selected_states,
            buttons,
            multiselect,
            width,
        } = self;

        if buttons.is_empty() {
            return ui.label("No buttons").into();
        }

        // Material Design colors
        let md_outline = get_global_color("outline"); // md-sys-color-outline
        let md_outline_variant = get_global_color("outlineVariant"); // md-sys-color-outline-variant
        let md_on_surface = get_global_color("onSurface"); // md-sys-color-on-surface
        let md_on_surface_variant = get_global_color("onSurfaceVariant"); // md-sys-color-on-surface-variant
        let md_primary = get_global_color("primary"); // md-sys-color-primary
        let md_on_primary = get_global_color("onPrimary"); // md-sys-color-on-primary
        let md_secondary_container = get_global_color("secondaryContainer"); // md-sys-color-secondary-container
        let md_on_secondary_container = get_global_color("onSecondaryContainer"); // md-sys-color-on-secondary-container

        // Button dimensions
        let button_height = 40.0;
        let total_width = width.unwrap_or(ui.available_width());
        let button_width = total_width / buttons.len() as f32;

        let desired_size = Vec2::new(total_width, button_height);
        let mut response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw buttons
            for (i, button) in buttons.iter().enumerate() {
                let is_selected = selected_states.get(i).copied().unwrap_or(false);
                let is_first = i == 0;
                let is_last = i == buttons.len() - 1;

                let button_rect = Rect::from_min_size(
                    rect.min + Vec2::new(i as f32 * button_width, 0.0),
                    Vec2::new(button_width, button_height),
                );

                // Handle button interaction
                if !button.disabled {
                    let button_response = ui.allocate_rect(button_rect, Sense::click());
                    if button_response.clicked() {
                        if multiselect {
                            // Multi-select: toggle this button
                            if let Some(state) = selected_states.get_mut(i) {
                                *state = !*state;
                            }
                        } else {
                            // Single-select: select this button, deselect others
                            for (j, state) in selected_states.iter_mut().enumerate() {
                                *state = j == i;
                            }
                        }
                        response = response.union(button_response);
                    }
                }

                // Calculate corner radius for segmented appearance
                let corner_radius = if is_first && is_last {
                    CornerRadius::from(20) // Single button
                } else if is_first {
                    CornerRadius {
                        nw: 20,
                        ne: 0,
                        sw: 20,
                        se: 0,
                    }
                } else if is_last {
                    CornerRadius {
                        nw: 0,
                        ne: 20,
                        sw: 0,
                        se: 20,
                    }
                } else {
                    CornerRadius::ZERO
                };

                // Determine button colors and stroke
                let (bg_color, text_color, stroke) = if button.disabled {
                    (
                        Color32::TRANSPARENT,
                        Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 97),
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(md_on_surface.r(), md_on_surface.g(), md_on_surface.b(), 31)),
                    )
                } else if is_selected {
                    (
                        md_secondary_container,
                        md_on_secondary_container,
                        Stroke::new(1.0, md_outline),
                    )
                } else {
                    (
                        Color32::TRANSPARENT,
                        md_on_surface,
                        Stroke::new(1.0, md_outline),
                    )
                };

                // Draw button background
                ui.painter().rect_filled(button_rect, corner_radius, bg_color);

                // Draw button outline
                ui.painter().rect_stroke(button_rect, corner_radius, stroke, egui::epaint::StrokeKind::Outside);

                // Draw button content
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(button_rect.shrink(8.0)), |ui| {
                    ui.horizontal_centered(|ui| {
                        // Icon
                        if let Some(icon) = &button.icon {
                            ui.style_mut().override_text_style = Some(TextStyle::Body);
                            ui.colored_label(text_color, icon);
                        }

                        // Checkmark for selected state (unless no_checkmark is true)
                        if is_selected && !button.no_checkmark && !button.disabled {
                            if button.icon.is_some() && button.label.is_some() {
                                ui.add_space(4.0);
                            }
                            ui.style_mut().override_text_style = Some(TextStyle::Small);
                            ui.colored_label(text_color, "✓");
                        }

                        // Label
                        if let Some(label) = &button.label {
                            if button.icon.is_some() {
                                ui.add_space(4.0);
                            }
                            ui.style_mut().override_text_style = Some(TextStyle::Body);
                            ui.colored_label(text_color, label);
                        }
                    });
                });
            }
        }

        response
    }
}

/// Convenience function to create a segmented button.
pub fn segmented_button() -> SegmentedButton {
    SegmentedButton::new()
}