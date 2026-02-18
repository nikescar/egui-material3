use crate::get_global_color;
use egui::{self, Color32, FontId, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};
use std::ops::RangeInclusive;

/// Interaction modes for sliders
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderInteraction {
    /// Allow both tapping and sliding (default)
    TapAndSlide,
    /// Only allow tapping to set value
    TapOnly,
    /// Only allow sliding from current position
    SlideOnly,
    /// Only allow sliding the thumb itself
    SlideThumb,
}

impl Default for SliderInteraction {
    fn default() -> Self {
        Self::TapAndSlide
    }
}

/// Thumb shape variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThumbShape {
    /// Round thumb (classic Material Design)
    Round,
    /// Handle thumb (Material Design 3 2024)
    Handle,
}

impl Default for ThumbShape {
    fn default() -> Self {
        Self::Round
    }
}

/// Range values for RangeSlider
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeValues {
    pub start: f32,
    pub end: f32,
}

impl RangeValues {
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }
}

/// Material Design slider component following Material Design 3 specifications
///
/// Sliders allow users to make selections from a range of values. They're ideal for
/// adjusting settings such as volume, brightness, or applying image filters.
pub struct MaterialSlider<'a> {
    /// Mutable reference to the slider value
    value: &'a mut f32,
    /// Valid range of values for the slider
    range: RangeInclusive<f32>,
    /// Optional text label displayed above the slider
    text: Option<String>,
    /// Whether the slider is interactive (enabled/disabled)
    enabled: bool,
    /// Custom width for the slider (None uses available width)
    width: Option<f32>,
    /// Optional step increment for discrete values
    step: Option<f32>,
    /// Whether to show the current value next to the slider
    show_value: bool,
    /// Secondary track value (e.g., for buffering indicators)
    secondary_track_value: Option<f32>,
    /// Whether to show value indicator while dragging
    show_value_indicator: bool,
    /// Interaction mode
    interaction_mode: SliderInteraction,
    /// Thumb shape
    thumb_shape: ThumbShape,
    /// Custom overlay/ripple color
    overlay_color: Option<Color32>,
    /// Custom thumb color
    thumb_color: Option<Color32>,
    /// Secondary active track color
    secondary_active_color: Option<Color32>,
}

impl<'a> MaterialSlider<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            text: None,
            enabled: true,
            width: None,
            step: None,
            show_value: true,
            secondary_track_value: None,
            show_value_indicator: false,
            interaction_mode: SliderInteraction::default(),
            thumb_shape: ThumbShape::default(),
            overlay_color: None,
            thumb_color: None,
            secondary_active_color: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    pub fn secondary_track_value(mut self, value: f32) -> Self {
        self.secondary_track_value = Some(value);
        self
    }

    pub fn show_value_indicator(mut self, show: bool) -> Self {
        self.show_value_indicator = show;
        self
    }

    pub fn interaction_mode(mut self, mode: SliderInteraction) -> Self {
        self.interaction_mode = mode;
        self
    }

    pub fn thumb_shape(mut self, shape: ThumbShape) -> Self {
        self.thumb_shape = shape;
        self
    }

    pub fn overlay_color(mut self, color: Color32) -> Self {
        self.overlay_color = Some(color);
        self
    }

    pub fn thumb_color(mut self, color: Color32) -> Self {
        self.thumb_color = Some(color);
        self
    }

    pub fn secondary_active_color(mut self, color: Color32) -> Self {
        self.secondary_active_color = Some(color);
        self
    }
}

impl<'a> Widget for MaterialSlider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let slider_width = self.width.unwrap_or(200.0);
        let height = 48.0;

        let desired_size = if self.text.is_some() || self.show_value {
            Vec2::new(slider_width + 100.0, height)
        } else {
            Vec2::new(slider_width, height)
        };

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface_variant = get_global_color("surfaceVariant");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");

        // Calculate slider track area
        let track_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - 2.0),
            Vec2::new(slider_width, 4.0),
        );

        let old_value = *self.value;

        // Handle interaction based on mode
        let can_interact = match self.interaction_mode {
            SliderInteraction::TapAndSlide => response.clicked() || response.dragged(),
            SliderInteraction::TapOnly => response.clicked(),
            SliderInteraction::SlideOnly => response.dragged(),
            SliderInteraction::SlideThumb => {
                // Check if mouse is over thumb
                let normalized_value =
                    (*self.value - self.range.start()) / (self.range.end() - self.range.start());
                let normalized_value = normalized_value.clamp(0.0, 1.0);
                let thumb_x = track_rect.min.x + normalized_value * track_rect.width();
                let thumb_center = Pos2::new(thumb_x, track_rect.center().y);
                
                if let Some(mouse_pos) = response.interact_pointer_pos() {
                    let dist = (mouse_pos - thumb_center).length();
                    response.dragged() && dist < 20.0
                } else {
                    false
                }
            }
        };

        if can_interact && self.enabled {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let normalized =
                    ((mouse_pos.x - track_rect.min.x) / track_rect.width()).clamp(0.0, 1.0);
                let mut new_value =
                    *self.range.start() + normalized * (self.range.end() - self.range.start());

                // Apply step if specified
                if let Some(step) = self.step {
                    new_value = (new_value / step).round() * step;
                }

                *self.value = new_value.clamp(*self.range.start(), *self.range.end());
                if (*self.value - old_value).abs() > f32::EPSILON {
                    response.mark_changed();
                }
            }
        }

        if !self.enabled {
            response = response.on_disabled_hover_text("Slider is disabled");
        }

        // Calculate positions
        let normalized_value =
            (*self.value - self.range.start()) / (self.range.end() - self.range.start());
        let normalized_value = normalized_value.clamp(0.0, 1.0);
        let thumb_x = track_rect.min.x + normalized_value * track_rect.width();
        let thumb_center = Pos2::new(thumb_x, track_rect.center().y);

        // Determine colors based on state
        let effective_thumb_color = self.thumb_color.unwrap_or(primary_color);
        let (track_active_color, track_inactive_color, thumb_color) = if !self.enabled {
            let disabled_color = get_global_color("onSurface").linear_multiply(0.38);
            (disabled_color, disabled_color, disabled_color)
        } else if response.hovered() || response.dragged() {
            (
                Color32::from_rgba_premultiplied(
                    primary_color.r(),
                    primary_color.g(),
                    primary_color.b(),
                    200,
                ),
                surface_variant,
                Color32::from_rgba_premultiplied(
                    effective_thumb_color.r().saturating_add(20),
                    effective_thumb_color.g().saturating_add(20),
                    effective_thumb_color.b().saturating_add(20),
                    255,
                ),
            )
        } else {
            (primary_color, surface_variant, effective_thumb_color)
        };

        // Draw inactive track
        ui.painter()
            .rect_filled(track_rect, 2.0, track_inactive_color);

        // Draw secondary track if specified
        if let Some(secondary_value) = self.secondary_track_value {
            let secondary_normalized =
                (secondary_value - self.range.start()) / (self.range.end() - self.range.start());
            let secondary_normalized = secondary_normalized.clamp(0.0, 1.0);
            let secondary_x = track_rect.min.x + secondary_normalized * track_rect.width();
            
            if secondary_x > thumb_x {
                let secondary_rect = Rect::from_min_size(
                    Pos2::new(thumb_x, track_rect.min.y),
                    Vec2::new(secondary_x - thumb_x, track_rect.height()),
                );
                let secondary_color = self.secondary_active_color.unwrap_or_else(|| {
                    Color32::from_rgba_premultiplied(
                        primary_color.r(),
                        primary_color.g(),
                        primary_color.b(),
                        128,
                    )
                });
                ui.painter().rect_filled(secondary_rect, 2.0, secondary_color);
            }
        }

        // Draw active track (from start to thumb)
        let active_track_rect = Rect::from_min_size(
            track_rect.min,
            Vec2::new(thumb_x - track_rect.min.x, track_rect.height()),
        );

        if active_track_rect.width() > 0.0 {
            ui.painter()
                .rect_filled(active_track_rect, 2.0, track_active_color);
        }

        // Draw thumb based on shape
        match self.thumb_shape {
            ThumbShape::Round => {
                let thumb_radius = if response.hovered() || response.dragged() {
                    12.0
                } else {
                    10.0
                };
                ui.painter()
                    .circle_filled(thumb_center, thumb_radius, thumb_color);
            }
            ThumbShape::Handle => {
                // Handle shape: rounded rectangle
                let handle_width = if response.hovered() || response.dragged() {
                    8.0
                } else {
                    4.0
                };
                let handle_height = 20.0;
                let handle_rect = Rect::from_center_size(
                    thumb_center,
                    Vec2::new(handle_width, handle_height),
                );
                ui.painter().rect_filled(handle_rect, 2.0, thumb_color);
            }
        }

        // Add ripple effect when interacting
        if response.hovered() && self.enabled {
            let ripple_color = self.overlay_color.unwrap_or_else(|| {
                Color32::from_rgba_premultiplied(
                    primary_color.r(),
                    primary_color.g(),
                    primary_color.b(),
                    30,
                )
            });
            let ripple_radius = match self.thumb_shape {
                ThumbShape::Round => 28.0,
                ThumbShape::Handle => 24.0,
            };
            ui.painter()
                .circle_filled(thumb_center, ripple_radius, ripple_color);
        }

        // Draw value indicator if enabled and dragging
        if self.show_value_indicator && response.dragged() && self.enabled {
            let value_text = if let Some(step) = self.step {
                if step >= 1.0 {
                    format!("{:.0}", *self.value)
                } else {
                    format!("{:.2}", *self.value)
                }
            } else {
                format!("{:.2}", *self.value)
            };

            // Simple rectangle indicator
            let indicator_font = FontId::proportional(12.0);
            let galley = ui.fonts(|f| f.layout_no_wrap(value_text, indicator_font, on_surface));
            let indicator_size = Vec2::new(galley.size().x + 16.0, galley.size().y + 8.0);
            let indicator_pos = Pos2::new(
                thumb_center.x - indicator_size.x / 2.0,
                thumb_center.y - indicator_size.y - 16.0,
            );
            let indicator_rect = Rect::from_min_size(indicator_pos, indicator_size);

            // Draw indicator background
            ui.painter().rect_filled(
                indicator_rect,
                4.0,
                primary_color,
            );

            // Draw indicator text
            ui.painter().galley(
                Pos2::new(
                    indicator_rect.center().x - galley.size().x / 2.0,
                    indicator_rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                Color32::WHITE,
            );
        }

        // Draw label text
        if let Some(ref text) = self.text {
            let text_pos = Pos2::new(track_rect.max.x + 16.0, rect.center().y - 16.0);
            let text_color = if self.enabled {
                on_surface
            } else {
                get_global_color("onSurface").linear_multiply(0.38)
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::default(),
                text_color,
            );
        }

        // Draw value
        if self.show_value {
            let value_text = if let Some(step) = self.step {
                if step >= 1.0 {
                    format!("{:.0}", *self.value)
                } else {
                    format!("{:.2}", *self.value)
                }
            } else {
                format!("{:.2}", *self.value)
            };

            let value_pos = Pos2::new(
                track_rect.max.x + 16.0,
                rect.center().y + if self.text.is_some() { 8.0 } else { 0.0 },
            );

            let value_color = if self.enabled {
                on_surface_variant
            } else {
                get_global_color("onSurface").linear_multiply(0.38)
            };

            ui.painter().text(
                value_pos,
                egui::Align2::LEFT_CENTER,
                &value_text,
                egui::FontId::proportional(12.0),
                value_color,
            );
        }

        response
    }
}

/// Range Slider component for selecting a range of values
pub struct MaterialRangeSlider<'a> {
    values: &'a mut RangeValues,
    range: RangeInclusive<f32>,
    text: Option<String>,
    enabled: bool,
    width: Option<f32>,
    step: Option<f32>,
    show_values: bool,
    show_value_indicator: bool,
    thumb_shape: ThumbShape,
    min_separation: f32,
}

impl<'a> MaterialRangeSlider<'a> {
    pub fn new(values: &'a mut RangeValues, range: RangeInclusive<f32>) -> Self {
        Self {
            values,
            range,
            text: None,
            enabled: true,
            width: None,
            step: None,
            show_values: true,
            show_value_indicator: false,
            thumb_shape: ThumbShape::default(),
            min_separation: 0.0,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn show_value_indicator(mut self, show: bool) -> Self {
        self.show_value_indicator = show;
        self
    }

    pub fn thumb_shape(mut self, shape: ThumbShape) -> Self {
        self.thumb_shape = shape;
        self
    }

    pub fn min_separation(mut self, separation: f32) -> Self {
        self.min_separation = separation;
        self
    }
}

impl<'a> Widget for MaterialRangeSlider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let slider_width = self.width.unwrap_or(200.0);
        let height = 48.0;

        let desired_size = if self.text.is_some() || self.show_values {
            Vec2::new(slider_width + 120.0, height)
        } else {
            Vec2::new(slider_width, height)
        };

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface_variant = get_global_color("surfaceVariant");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");

        // Calculate slider track area
        let track_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - 2.0),
            Vec2::new(slider_width, 4.0),
        );

        // Handle interaction
        if response.dragged() && self.enabled {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let normalized =
                    ((mouse_pos.x - track_rect.min.x) / track_rect.width()).clamp(0.0, 1.0);
                let mut new_value =
                    *self.range.start() + normalized * (self.range.end() - self.range.start());

                // Apply step if specified
                if let Some(step) = self.step {
                    new_value = (new_value / step).round() * step;
                }

                // Determine which thumb is closer
                let dist_to_start = (new_value - self.values.start).abs();
                let dist_to_end = (new_value - self.values.end).abs();

                if dist_to_start < dist_to_end {
                    // Move start thumb
                    self.values.start = new_value.clamp(
                        *self.range.start(),
                        (self.values.end - self.min_separation).min(*self.range.end()),
                    );
                } else {
                    // Move end thumb
                    self.values.end = new_value.clamp(
                        (self.values.start + self.min_separation).max(*self.range.start()),
                        *self.range.end(),
                    );
                }
            }
        }

        // Calculate thumb positions
        let start_normalized =
            (self.values.start - self.range.start()) / (self.range.end() - self.range.start());
        let start_normalized = start_normalized.clamp(0.0, 1.0);
        let start_x = track_rect.min.x + start_normalized * track_rect.width();
        let start_center = Pos2::new(start_x, track_rect.center().y);

        let end_normalized =
            (self.values.end - self.range.start()) / (self.range.end() - self.range.start());
        let end_normalized = end_normalized.clamp(0.0, 1.0);
        let end_x = track_rect.min.x + end_normalized * track_rect.width();
        let end_center = Pos2::new(end_x, track_rect.center().y);

        // Determine colors
        let (track_active_color, track_inactive_color, thumb_color) = if !self.enabled {
            let disabled_color = get_global_color("onSurface").linear_multiply(0.38);
            (disabled_color, disabled_color, disabled_color)
        } else if response.hovered() || response.dragged() {
            (
                Color32::from_rgba_premultiplied(
                    primary_color.r(),
                    primary_color.g(),
                    primary_color.b(),
                    200,
                ),
                surface_variant,
                primary_color,
            )
        } else {
            (primary_color, surface_variant, primary_color)
        };

        // Draw inactive track (full width)
        ui.painter()
            .rect_filled(track_rect, 2.0, track_inactive_color);

        // Draw active track (between thumbs)
        let active_track_rect = Rect::from_min_size(
            Pos2::new(start_x, track_rect.min.y),
            Vec2::new(end_x - start_x, track_rect.height()),
        );

        if active_track_rect.width() > 0.0 {
            ui.painter()
                .rect_filled(active_track_rect, 2.0, track_active_color);
        }

        // Draw thumbs
        let thumb_radius = if response.hovered() || response.dragged() {
            12.0
        } else {
            10.0
        };

        match self.thumb_shape {
            ThumbShape::Round => {
                ui.painter()
                    .circle_filled(start_center, thumb_radius, thumb_color);
                ui.painter()
                    .circle_filled(end_center, thumb_radius, thumb_color);
            }
            ThumbShape::Handle => {
                let handle_width = if response.hovered() || response.dragged() {
                    8.0
                } else {
                    4.0
                };
                let handle_height = 20.0;

                let start_handle_rect = Rect::from_center_size(
                    start_center,
                    Vec2::new(handle_width, handle_height),
                );
                ui.painter()
                    .rect_filled(start_handle_rect, 2.0, thumb_color);

                let end_handle_rect = Rect::from_center_size(
                    end_center,
                    Vec2::new(handle_width, handle_height),
                );
                ui.painter()
                    .rect_filled(end_handle_rect, 2.0, thumb_color);
            }
        }

        // Add ripple effects
        if response.hovered() && self.enabled {
            let ripple_color = Color32::from_rgba_premultiplied(
                primary_color.r(),
                primary_color.g(),
                primary_color.b(),
                30,
            );
            ui.painter()
                .circle_filled(start_center, 28.0, ripple_color);
            ui.painter()
                .circle_filled(end_center, 28.0, ripple_color);
        }

        // Draw label
        if let Some(ref text) = self.text {
            let text_pos = Pos2::new(track_rect.max.x + 16.0, rect.center().y - 16.0);
            let text_color = if self.enabled {
                on_surface
            } else {
                get_global_color("onSurface").linear_multiply(0.38)
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::default(),
                text_color,
            );
        }

        // Draw values
        if self.show_values {
            let format_value = |value: f32| {
                if let Some(step) = self.step {
                    if step >= 1.0 {
                        format!("{:.0}", value)
                    } else {
                        format!("{:.2}", value)
                    }
                } else {
                    format!("{:.2}", value)
                }
            };

            let value_text = format!(
                "{} - {}",
                format_value(self.values.start),
                format_value(self.values.end)
            );

            let value_pos = Pos2::new(
                track_rect.max.x + 16.0,
                rect.center().y + if self.text.is_some() { 8.0 } else { 0.0 },
            );

            let value_color = if self.enabled {
                on_surface_variant
            } else {
                get_global_color("onSurface").linear_multiply(0.38)
            };

            ui.painter().text(
                value_pos,
                egui::Align2::LEFT_CENTER,
                &value_text,
                egui::FontId::proportional(12.0),
                value_color,
            );
        }

        response
    }
}

pub fn slider<'a>(value: &'a mut f32, range: RangeInclusive<f32>) -> MaterialSlider<'a> {
    MaterialSlider::new(value, range)
}

pub fn range_slider<'a>(
    values: &'a mut RangeValues,
    range: RangeInclusive<f32>,
) -> MaterialRangeSlider<'a> {
    MaterialRangeSlider::new(values, range)
}
