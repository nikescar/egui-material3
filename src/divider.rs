// ============================================================================
// Module:       divider
// Description:  Material 3 horizontal and vertical dividers, full or inset.
//
// Dependencies: egui; crate::material_theme, crate::tokens
// ============================================================================

//! Material 3 dividers.
//!
//! A divider is a thin line that groups content in lists and containers.
//! Values from Compose's `DividerTokens.kt`: `Thickness` 1 dp,
//! `Color` = `OutlineVariant`. The inset variants use the list divider
//! spacing from `ListTokens.kt` (`DividerLeadingSpace` and
//! `DividerTrailingSpace`, both `MeasurementTokens.Space200` = 16 dp).
//!
//! ```rust
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::{divider, MaterialDivider};
//!
//! ui.label("Above");
//! ui.add(divider());
//! ui.add(MaterialDivider::inset());
//! ui.label("Below");
//! # });
//! ```

use egui::{Color32, Rect, Response, Sense, Ui, Vec2, Widget};

use crate::material_theme::MaterialTheme;
use crate::tokens::{ColorRole, LayoutDirection};

/// `DividerTokens.Thickness`: 1 dp.
pub const DIVIDER_THICKNESS: f32 = 1.0;
/// `ListTokens.DividerLeadingSpace` / `DividerTrailingSpace`
/// (`MeasurementTokens.Space200`): 16 dp, the inset of an inset divider.
pub const DIVIDER_INSET: f32 = 16.0;

/// Which way the divider runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DividerOrientation {
    /// A horizontal line across the available width.
    Horizontal,
    /// A vertical line down the available height.
    Vertical,
}

/// A Material 3 divider.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Clone, Debug)]
pub struct MaterialDivider {
    orientation: DividerOrientation,
    inset_start: f32,
    inset_end: f32,
    thickness: f32,
    color: Option<Color32>,
    length: Option<f32>,
}

impl Default for MaterialDivider {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialDivider {
    /// A full-width horizontal divider.
    pub fn new() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            inset_start: 0.0,
            inset_end: 0.0,
            thickness: DIVIDER_THICKNESS,
            color: None,
            length: None,
        }
    }

    /// A horizontal divider inset 16 dp at the start (left in LTR), as
    /// between list items that have a leading element.
    pub fn inset() -> Self {
        Self::new().inset_start(DIVIDER_INSET)
    }

    /// A horizontal divider inset 16 dp at both ends.
    pub fn middle_inset() -> Self {
        Self::new()
            .inset_start(DIVIDER_INSET)
            .inset_end(DIVIDER_INSET)
    }

    /// A full-height vertical divider.
    pub fn vertical() -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            ..Self::new()
        }
    }

    /// Space before the line: at the start for a horizontal divider (left
    /// in LTR, right in RTL), at the top for a vertical one.
    pub fn inset_start(mut self, inset: f32) -> Self {
        self.inset_start = inset.max(0.0);
        self
    }

    /// Space after the line: at the end for a horizontal divider, at the
    /// bottom for a vertical one.
    pub fn inset_end(mut self, inset: f32) -> Self {
        self.inset_end = inset.max(0.0);
        self
    }

    /// Line thickness. Default 1 dp.
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness.max(0.0);
        self
    }

    /// Line colour. Default: the theme's `outlineVariant`.
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Length along the line, instead of all the available width (or
    /// height for a vertical divider).
    pub fn length(mut self, length: f32) -> Self {
        self.length = Some(length.max(0.0));
        self
    }

    /// Which way this divider runs.
    pub fn orientation(&self) -> DividerOrientation {
        self.orientation
    }
}

impl Widget for MaterialDivider {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let color = self
            .color
            .unwrap_or_else(|| theme.color(ColorRole::OutlineVariant));
        let size = match self.orientation {
            DividerOrientation::Horizontal => Vec2::new(
                self.length.unwrap_or_else(|| ui.available_width()),
                self.thickness,
            ),
            DividerOrientation::Vertical => Vec2::new(
                self.thickness,
                self.length.unwrap_or_else(|| ui.available_height()),
            ),
        };
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        let line = line_rect(
            rect,
            self.orientation,
            self.inset_start,
            self.inset_end,
            theme.layout_direction,
        );
        if ui.is_rect_visible(line) && line.width() > 0.0 && line.height() > 0.0 {
            ui.painter().rect_filled(line, 0.0, color);
        }
        response
    }
}

/// The painted line inside the divider's allocated `rect`.
fn line_rect(
    rect: Rect,
    orientation: DividerOrientation,
    start: f32,
    end: f32,
    direction: LayoutDirection,
) -> Rect {
    let mut line = rect;
    match orientation {
        DividerOrientation::Horizontal => {
            let (left, right) = match direction {
                LayoutDirection::Ltr => (start, end),
                LayoutDirection::Rtl => (end, start),
            };
            line.min.x += left;
            line.max.x -= right;
        }
        DividerOrientation::Vertical => {
            line.min.y += start;
            line.max.y -= end;
        }
    }
    if line.min.x > line.max.x {
        line.max.x = line.min.x;
    }
    if line.min.y > line.max.y {
        line.max.y = line.min.y;
    }
    line
}

/// A full-width horizontal divider.
pub fn divider() -> MaterialDivider {
    MaterialDivider::new()
}

/// A full-height vertical divider.
pub fn vertical_divider() -> MaterialDivider {
    MaterialDivider::vertical()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inset_mirrors_in_rtl() {
        let rect = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(100.0, 1.0));
        let ltr = line_rect(
            rect,
            DividerOrientation::Horizontal,
            16.0,
            0.0,
            LayoutDirection::Ltr,
        );
        let rtl = line_rect(
            rect,
            DividerOrientation::Horizontal,
            16.0,
            0.0,
            LayoutDirection::Rtl,
        );
        assert_eq!(ltr.min.x, 16.0, "LTR start inset is on the left");
        assert_eq!(rtl.max.x, 84.0, "RTL start inset is on the right");
    }

    #[test]
    fn oversized_insets_collapse_to_nothing() {
        let rect = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(10.0, 1.0));
        let line = line_rect(
            rect,
            DividerOrientation::Horizontal,
            16.0,
            16.0,
            LayoutDirection::Ltr,
        );
        assert_eq!(
            line.width(),
            0.0,
            "insets wider than the divider leave no line"
        );
    }
}
