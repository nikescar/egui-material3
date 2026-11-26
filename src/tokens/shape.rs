//! The Material 3 shape (corner) scale.
//!
//! Values from Compose's `ShapeTokens.kt` (`CornerValue*`, and the
//! `*Top`, `*End`, `*Start` variants of `CornerExtraSmall`, `CornerLarge`
//! and `CornerExtraLarge`).

use egui::{CornerRadius, Rect};

/// Which way text and layout run, for shapes and positions that name a
/// start or end edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LayoutDirection {
    /// Left to right: start is left.
    #[default]
    Ltr,
    /// Right to left: start is right.
    Rtl,
}

/// A step on the corner scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CornerSize {
    /// `CornerValueNone`: 0.
    None,
    /// `CornerValueExtraSmall`: 4.
    ExtraSmall,
    /// `CornerValueSmall`: 8.
    Small,
    /// `CornerValueMedium`: 12.
    Medium,
    /// `CornerValueLarge`: 16.
    Large,
    /// `CornerValueLargeIncreased`: 20.
    LargeIncreased,
    /// `CornerValueExtraLarge`: 28.
    ExtraLarge,
    /// `CornerValueExtraLargeIncreased`: 32.
    ExtraLargeIncreased,
    /// `CornerValueExtraExtraLarge`: 48.
    ExtraExtraLarge,
    /// `CornerFull`: half the shorter side (a pill or circle).
    Full,
}

impl CornerSize {
    /// The radius in points, or `None` for [`CornerSize::Full`], which
    /// depends on the size of the shape.
    pub const fn value(self) -> Option<f32> {
        match self {
            CornerSize::None => Some(0.0),
            CornerSize::ExtraSmall => Some(4.0),
            CornerSize::Small => Some(8.0),
            CornerSize::Medium => Some(12.0),
            CornerSize::Large => Some(16.0),
            CornerSize::LargeIncreased => Some(20.0),
            CornerSize::ExtraLarge => Some(28.0),
            CornerSize::ExtraLargeIncreased => Some(32.0),
            CornerSize::ExtraExtraLarge => Some(48.0),
            CornerSize::Full => None,
        }
    }

    /// The radius for a shape occupying `rect`.
    pub fn radius(self, rect: Rect) -> f32 {
        let half_min = rect.width().min(rect.height()) / 2.0;
        self.value().unwrap_or(half_min).min(half_min).max(0.0)
    }
}

/// Which corners a shape rounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Corners {
    /// All four corners.
    #[default]
    All,
    /// The two top corners (`*Top` tokens: sheets, menus from a bar).
    Top,
    /// The two end corners (`*End` tokens: a start-aligned drawer).
    End,
    /// The two start corners (`*Start` tokens).
    Start,
}

/// A shape: a corner size applied to a set of corners.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Shape {
    /// The radius.
    pub size: CornerSize,
    /// Which corners carry it.
    pub corners: Corners,
}

impl Shape {
    /// All corners rounded by `size`.
    pub const fn all(size: CornerSize) -> Self {
        Self {
            size,
            corners: Corners::All,
        }
    }

    /// `ShapeTokens.CornerNone`.
    pub const NONE: Shape = Shape::all(CornerSize::None);
    /// `ShapeTokens.CornerExtraSmall`.
    pub const EXTRA_SMALL: Shape = Shape::all(CornerSize::ExtraSmall);
    /// `ShapeTokens.CornerExtraSmallTop`.
    pub const EXTRA_SMALL_TOP: Shape = Shape {
        size: CornerSize::ExtraSmall,
        corners: Corners::Top,
    };
    /// `ShapeTokens.CornerSmall`.
    pub const SMALL: Shape = Shape::all(CornerSize::Small);
    /// `ShapeTokens.CornerMedium`.
    pub const MEDIUM: Shape = Shape::all(CornerSize::Medium);
    /// `ShapeTokens.CornerLarge`.
    pub const LARGE: Shape = Shape::all(CornerSize::Large);
    /// `ShapeTokens.CornerLargeTop`.
    pub const LARGE_TOP: Shape = Shape {
        size: CornerSize::Large,
        corners: Corners::Top,
    };
    /// `ShapeTokens.CornerLargeEnd`.
    pub const LARGE_END: Shape = Shape {
        size: CornerSize::Large,
        corners: Corners::End,
    };
    /// `ShapeTokens.CornerLargeStart`.
    pub const LARGE_START: Shape = Shape {
        size: CornerSize::Large,
        corners: Corners::Start,
    };
    /// `ShapeTokens.CornerLargeIncreased`.
    pub const LARGE_INCREASED: Shape = Shape::all(CornerSize::LargeIncreased);
    /// `ShapeTokens.CornerExtraLarge`.
    pub const EXTRA_LARGE: Shape = Shape::all(CornerSize::ExtraLarge);
    /// `ShapeTokens.CornerExtraLargeTop`.
    pub const EXTRA_LARGE_TOP: Shape = Shape {
        size: CornerSize::ExtraLarge,
        corners: Corners::Top,
    };
    /// `ShapeTokens.CornerExtraLargeIncreased`.
    pub const EXTRA_LARGE_INCREASED: Shape = Shape::all(CornerSize::ExtraLargeIncreased);
    /// `ShapeTokens.CornerExtraExtraLarge`.
    pub const EXTRA_EXTRA_LARGE: Shape = Shape::all(CornerSize::ExtraExtraLarge);
    /// `ShapeTokens.CornerFull`.
    pub const FULL: Shape = Shape::all(CornerSize::Full);

    /// The egui corner radii for this shape on `rect`, mirroring start and
    /// end in right-to-left layouts.
    pub fn corner_radius(self, rect: Rect, direction: LayoutDirection) -> CornerRadius {
        // CornerRadius stores whole points; the token radii are whole numbers.
        let r = self.size.radius(rect).round().clamp(0.0, 255.0) as u8;
        let (left, right) = (
            CornerRadius {
                nw: r,
                sw: r,
                ne: 0,
                se: 0,
            },
            CornerRadius {
                ne: r,
                se: r,
                nw: 0,
                sw: 0,
            },
        );
        match (self.corners, direction) {
            (Corners::All, _) => CornerRadius::same(r),
            (Corners::Top, _) => CornerRadius {
                nw: r,
                ne: r,
                sw: 0,
                se: 0,
            },
            (Corners::Start, LayoutDirection::Ltr) | (Corners::End, LayoutDirection::Rtl) => left,
            (Corners::End, LayoutDirection::Ltr) | (Corners::Start, LayoutDirection::Rtl) => right,
        }
    }
}
