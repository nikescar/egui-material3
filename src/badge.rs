//! Material Design 3 Badge Components
//!
//! # M3 Color Role Usage
//!
//! - **error**: Error badge background
//! - **onError**: Error badge text
//! - **primary**: Primary badge background (for counts/notifications)
//! - **onPrimary**: Primary badge text
//! - **Custom colors**: For success, warning, neutral variants
//!
//! ## Dimensions
//! - **Small badge**: 6x6dp (no label)
//! - **Large badge**: 16dp height (with label/count)
//! - **Corner radius**: Fully rounded (50%)

use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, pos2, FontId, Rect, Response,
    Sense, Ui, Vec2, Widget,
};

/// Badge color variants following Material Design 3 specifications
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeColor {
    /// Primary color badge (uses primary theme color)
    Primary,
    /// Error/danger color badge (red, for alerts and errors)
    Error,
    /// Success color badge (green, for positive states)
    Success,
    /// Warning color badge (yellow/orange, for caution)
    Warning,
    /// Neutral/gray color badge (for general information)
    Neutral,
    /// Custom color badge
    Custom(Color32, Color32), // (background, text)
}

/// Badge size variants
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeSize {
    /// Small badge - 16px height, suitable for compact layouts
    Small,
    /// Regular badge - 20px height, standard size
    Regular,
    /// Large badge - 24px height, for more prominent display
    Large,
}

/// Badge positioning relative to parent element
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgePosition {
    /// Top-right corner
    TopRight,
    /// Top-left corner
    TopLeft,
    /// Bottom-right corner
    BottomRight,
    /// Bottom-left corner
    BottomLeft,
    /// Custom position with offset
    Custom(Vec2),
}

/// Material Design badge component.
///
/// Badges are small status descriptors for UI elements. They typically appear
/// as small circles or rounded rectangles with text or numbers, positioned
/// on or near other elements to provide context or notifications.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// // Simple numeric badge
/// ui.add(MaterialBadge::new("5")
///     .color(BadgeColor::Error));
///
/// // Text badge
/// ui.add(MaterialBadge::new("NEW")
///     .color(BadgeColor::Success)
///     .size(BadgeSize::Regular));
///
/// // Badge on an icon (standalone positioning)
/// ui.add(MaterialBadge::new("3")
///     .color(BadgeColor::Primary)
///     .size(BadgeSize::Small));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialBadge {
    /// Text content of the badge
    content: String,
    /// Color variant
    color: BadgeColor,
    /// Size variant
    size: BadgeSize,
    /// Whether to show as a dot (no text, just indicator)
    dot: bool,
    /// Custom position offset when used as overlay
    position_offset: Vec2,
}

impl MaterialBadge {
    /// Create a new badge with the specified content
    ///
    /// # Arguments
    /// * `content` - Text or number to display in the badge
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            color: BadgeColor::Error,
            size: BadgeSize::Regular,
            dot: false,
            position_offset: Vec2::new(0.0, 0.0),
        }
    }

    /// Create a badge showing just a dot (no text)
    pub fn dot() -> Self {
        Self {
            content: String::new(),
            color: BadgeColor::Error,
            size: BadgeSize::Small,
            dot: true,
            position_offset: Vec2::new(0.0, 0.0),
        }
    }

    /// Set the color variant of the badge
    pub fn color(mut self, color: BadgeColor) -> Self {
        self.color = color;
        self
    }

    /// Set the size of the badge
    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether to show as a dot indicator
    pub fn as_dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    /// Set a custom position offset for overlay positioning
    pub fn position_offset(mut self, offset: Vec2) -> Self {
        self.position_offset = offset;
        self
    }

    /// Draw the badge as an overlay on a specific rectangle
    ///
    /// This is useful for adding badges to other UI elements like buttons or icons.
    ///
    /// # Arguments
    /// * `ui` - The UI context
    /// * `target_rect` - The rectangle of the element to badge
    /// * `position` - Where to position the badge relative to the target
    pub fn draw_on(
        &self,
        ui: &mut Ui,
        target_rect: Rect,
        position: BadgePosition,
    ) -> Response {
        let (bg_color, text_color) = self.get_colors();
        let (min_width, min_height, font_size) = self.get_dimensions();

        let painter = ui.painter();

        // Calculate badge size
        let text_galley = if !self.dot && !self.content.is_empty() {
            Some(painter.layout_no_wrap(
                self.content.clone(),
                FontId::proportional(font_size),
                text_color,
            ))
        } else {
            None
        };

        let badge_width = if let Some(ref galley) = text_galley {
            (galley.size().x + min_width).max(min_height)
        } else {
            min_height
        };
        let badge_height = min_height;

        // Calculate badge position - positioned to overlap the icon edges
        // Using 95% overlap to make badges appear extremely close and tightly over the icon
        let overlap_factor = 0.95;
        let badge_pos = match position {
            BadgePosition::TopRight => {
                pos2(
                    target_rect.max.x - badge_width * (1.0 - overlap_factor),
                    target_rect.min.y - badge_height * (1.0 - overlap_factor),
                )
            }
            BadgePosition::TopLeft => {
                pos2(
                    target_rect.min.x - badge_width * overlap_factor,
                    target_rect.min.y - badge_height * (1.0 - overlap_factor),
                )
            }
            BadgePosition::BottomRight => {
                pos2(
                    target_rect.max.x - badge_width * (1.0 - overlap_factor),
                    target_rect.max.y - badge_height * (1.0 - overlap_factor),
                )
            }
            BadgePosition::BottomLeft => {
                pos2(
                    target_rect.min.x - badge_width * overlap_factor,
                    target_rect.max.y - badge_height * (1.0 - overlap_factor),
                )
            }
            BadgePosition::Custom(offset) => {
                pos2(target_rect.center().x + offset.x, target_rect.center().y + offset.y)
            }
        };

        let badge_pos = pos2(
            badge_pos.x + self.position_offset.x,
            badge_pos.y + self.position_offset.y,
        );

        let badge_rect = Rect::from_center_size(badge_pos, Vec2::new(badge_width, badge_height));

        // Draw badge background
        painter.rect_filled(badge_rect, badge_height / 2.0, bg_color);

        // Draw text if not a dot
        if let Some(galley) = text_galley {
            painter.galley(
                pos2(
                    badge_rect.center().x - galley.size().x / 2.0,
                    badge_rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                text_color,
            );
        }

        ui.interact(badge_rect, ui.id().with("badge"), Sense::hover())
    }

    fn get_colors(&self) -> (Color32, Color32) {
        match self.color {
            BadgeColor::Primary => {
                let bg = get_global_color("primary");
                let text = get_global_color("onPrimary");
                (bg, text)
            }
            BadgeColor::Error => (
                Color32::from_rgb(239, 68, 68), // red-500
                Color32::WHITE,
            ),
            BadgeColor::Success => (
                Color32::from_rgb(34, 197, 94), // green-500
                Color32::WHITE,
            ),
            BadgeColor::Warning => (
                Color32::from_rgb(234, 179, 8), // yellow-500
                Color32::WHITE,
            ),
            BadgeColor::Neutral => (
                Color32::from_rgb(107, 114, 128), // gray-500
                Color32::WHITE,
            ),
            BadgeColor::Custom(bg, text) => (bg, text),
        }
    }

    fn get_dimensions(&self) -> (f32, f32, f32) {
        // Returns (padding_width, min_height, font_size)
        match self.size {
            BadgeSize::Small => {
                if self.dot {
                    (0.0, 8.0, 0.0) // 8px dot
                } else {
                    (4.0, 16.0, 10.0) // text-2xs equivalent
                }
            }
            BadgeSize::Regular => {
                if self.dot {
                    (0.0, 10.0, 0.0) // 10px dot
                } else {
                    (6.0, 20.0, 12.0) // text-xs equivalent
                }
            }
            BadgeSize::Large => {
                if self.dot {
                    (0.0, 12.0, 0.0) // 12px dot
                } else {
                    (8.0, 24.0, 14.0) // text-sm equivalent
                }
            }
        }
    }
}

impl Widget for MaterialBadge {
    fn ui(self, ui: &mut Ui) -> Response {
        let (bg_color, text_color) = self.get_colors();
        let (min_width, min_height, font_size) = self.get_dimensions();

        // Calculate badge size
        let text_galley = if !self.dot && !self.content.is_empty() {
            Some(ui.painter().layout_no_wrap(
                self.content.clone(),
                FontId::proportional(font_size),
                text_color,
            ))
        } else {
            None
        };

        let badge_width = if let Some(ref galley) = text_galley {
            (galley.size().x + min_width).max(min_height)
        } else {
            min_height
        };
        let badge_height = min_height;

        let desired_size = Vec2::new(badge_width, badge_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Draw badge background
        ui.painter()
            .rect_filled(rect, badge_height / 2.0, bg_color);

        // Draw text if not a dot
        if let Some(galley) = text_galley {
            ui.painter().galley(
                pos2(
                    rect.center().x - galley.size().x / 2.0,
                    rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                text_color,
            );
        }

        response
    }
}

/// Convenience function to create a badge
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(badge("5").color(BadgeColor::Error));
/// # });
/// ```
pub fn badge(content: impl Into<String>) -> MaterialBadge {
    MaterialBadge::new(content)
}

/// Convenience function to create a dot badge
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// ui.add(badge_dot().color(BadgeColor::Success));
/// # });
/// ```
pub fn badge_dot() -> MaterialBadge {
    MaterialBadge::dot()
}
