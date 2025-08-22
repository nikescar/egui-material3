use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget,
};
use std::time::{Duration, Instant};

/// Material Design focus ring component.
///
/// A focus ring provides a visual indicator for keyboard navigation and accessibility.
/// It appears around interactive elements when they receive keyboard focus.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Add a focus ring to a button
/// let button_response = ui.button("Focusable Button");
/// ui.add(MaterialFocusRing::new()
///     .visible(button_response.has_focus())
///     .target_rect(button_response.rect));
///
/// // Focus ring with inward animation
/// ui.add(MaterialFocusRing::new()
///     .visible(true)
///     .inward(true)
///     .target_rect(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(64.0, 64.0))));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialFocusRing {
    visible: bool,
    inward: bool,
    target_rect: Option<Rect>,
    corner_radius: CornerRadius,
    color: Option<Color32>,
    width: f32,
    offset: f32,
    animated: bool,
    focus_start_time: Option<Instant>,
}

impl MaterialFocusRing {
    /// Create a new material focus ring.
    pub fn new() -> Self {
        Self {
            visible: false,
            inward: false,
            target_rect: None,
            corner_radius: CornerRadius::from(16),
            color: None,
            width: 2.0,
            offset: 2.0,
            animated: true, // Enable animation by default to match Material Web
            focus_start_time: None,
        }
    }

    /// Set the visibility of the focus ring.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Make the focus ring animate inwards instead of outwards.
    pub fn inward(mut self, inward: bool) -> Self {
        self.inward = inward;
        self
    }

    /// Set the target rectangle that the focus ring should surround.
    pub fn target_rect(mut self, rect: Rect) -> Self {
        self.target_rect = Some(rect);
        self
    }

    /// Set the corner radius of the focus ring.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set the color of the focus ring.
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the width of the focus ring stroke.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the offset from the target element.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Enable animated focus ring that fades from thick to lighter over time.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        if animated && self.visible && self.focus_start_time.is_none() {
            self.focus_start_time = Some(Instant::now());
        }
        self
    }

    /// Attach the focus ring to a response, automatically setting visibility based on focus state.
    pub fn attach_to_response(mut self, response: &Response) -> Self {
        self.visible = response.has_focus();
        self.target_rect = Some(response.rect);
        self
    }
}

impl Default for MaterialFocusRing {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialFocusRing {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialFocusRing {
            visible,
            inward,
            target_rect,
            corner_radius,
            color,
            width,
            offset,
            animated,
            mut focus_start_time,
        } = self;

        // Material Design focus ring color - use the global color system
        let base_color = color.unwrap_or_else(|| crate::get_global_color("primary"));
        
        // Animation logic matching Material Web behavior
        let (focus_color, current_width, current_offset) = if animated && visible {
            if focus_start_time.is_none() {
                focus_start_time = Some(Instant::now());
            }
            
            if let Some(start_time) = focus_start_time {
                let elapsed = start_time.elapsed();
                let total_animation_duration = Duration::from_millis(600); // 0.6 seconds total
                
                if elapsed < total_animation_duration {
                    // Calculate animation progress (0.0 to 1.0)
                    let progress = elapsed.as_millis() as f32 / total_animation_duration.as_millis() as f32;
                    
                    // Material Design animation: appears thick and fades to thinner with reduced opacity
                    let thickness_factor = 1.0 + (1.0 - progress) * 2.0; // Start thick, become normal
                    let alpha_factor = 0.4 + (1.0 - progress) * 0.6; // Start at full, fade to 40%
                    
                    // For inward rings, animate the offset as well
                    let animated_offset = if inward {
                        offset * (1.0 - progress * 0.3) // Slightly shrink inward during animation
                    } else {
                        offset + (progress * 2.0) // Expand outward during animation
                    };
                    
                    let animated_color = Color32::from_rgba_unmultiplied(
                        base_color.r(),
                        base_color.g(),
                        base_color.b(),
                        (255.0 * alpha_factor) as u8,
                    );
                    
                    // Request repaint for next frame
                    ui.ctx().request_repaint();
                    
                    (animated_color, width * thickness_factor, animated_offset)
                } else {
                    // Animation finished, show final stable state
                    let final_color = Color32::from_rgba_unmultiplied(
                        base_color.r(),
                        base_color.g(),
                        base_color.b(),
                        102, // ~40% opacity
                    );
                    (final_color, width, offset)
                }
            } else {
                (base_color, width, offset)
            }
        } else {
            (base_color, width, offset)
        };

        let rect = if let Some(target) = target_rect {
            if inward {
                // Inward focus ring shrinks the ring inside the target
                target.shrink(current_offset)
            } else {
                // Outward focus ring expands outside the target
                target.expand(current_offset)
            }
        } else {
            // If no target rect is provided, use available space
            let size = Vec2::splat(64.0); // Default size matching Material Web stories
            Rect::from_min_size(ui.next_widget_position(), size)
        };

        let response = ui.allocate_response(rect.size(), Sense::hover());
        let response_rect = response.rect;

        if visible && ui.is_rect_visible(rect) {
            // Draw the focus ring
            ui.painter().rect_stroke(
                rect,
                corner_radius,
                Stroke::new(current_width, focus_color),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Set aria-hidden attribute equivalent behavior by not including in accessibility
        // In egui, this is done by not adding any widget info for screen readers
        response
    }
}

/// Helper function to add a focus ring to any response.
/// This is a convenience function for quickly adding focus rings to existing widgets.
pub fn add_focus_ring_to_response(response: &Response, ui: &mut Ui, inward: bool) {
    if response.has_focus() {
        ui.add(MaterialFocusRing::new()
            .visible(true)
            .inward(inward)
            .target_rect(response.rect));
    }
}

/// Helper function to add a focus ring that's always visible for demonstration purposes.
pub fn demo_focus_ring(ui: &mut Ui, rect: Rect, inward: bool) -> Response {
    ui.add(MaterialFocusRing::new()
        .visible(true)
        .inward(inward)
        .target_rect(rect))
}