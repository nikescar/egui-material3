//! Type definitions for Material Design 3 Button Components

use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Shadow, Stroke},
    Image, Role, Sense, TextWrapMode, Vec2, WidgetText,
};

/// Material Design button variants following Material Design 3 specifications
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialButtonVariant {
    /// Filled button - High emphasis, filled background with primary color
    Filled,
    /// Outlined button - Medium emphasis, transparent background with outline
    Outlined,
    /// Text button - Low emphasis, transparent background, no outline
    Text,
    /// Elevated button - Medium emphasis, filled background with shadow elevation
    Elevated,
    /// Filled tonal button - Medium emphasis, filled background with secondary container color
    FilledTonal,
}

impl MaterialButtonVariant {
    /// Every variant, highest emphasis first.
    pub const ALL: [MaterialButtonVariant; 5] = [
        MaterialButtonVariant::Elevated,
        MaterialButtonVariant::Filled,
        MaterialButtonVariant::FilledTonal,
        MaterialButtonVariant::Outlined,
        MaterialButtonVariant::Text,
    ];
}

/// The M3 Expressive button sizes (`Button{XSmall,Small,Medium,Large,XLarge}Tokens`).
///
/// A button with no size set uses the baseline metrics
/// (`BaselineButtonTokens`: 40 dp, 24 dp side padding), which is what
/// Compose's plain `Button` uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonSize {
    /// 32 dp high, LabelLarge, 20 dp icons.
    ExtraSmall,
    /// 40 dp high, LabelLarge, 20 dp icons.
    Small,
    /// 56 dp high, TitleMedium, 24 dp icons.
    Medium,
    /// 96 dp high, HeadlineSmall, 32 dp icons.
    Large,
    /// 136 dp high, HeadlineLarge, 40 dp icons.
    ExtraLarge,
}

impl ButtonSize {
    /// Every size, smallest first.
    pub const ALL: [ButtonSize; 5] = [
        ButtonSize::ExtraSmall,
        ButtonSize::Small,
        ButtonSize::Medium,
        ButtonSize::Large,
        ButtonSize::ExtraLarge,
    ];
}

/// The resting shape of a button: a pill or a rounded rectangle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ButtonShape {
    /// `ContainerShapeRound`: fully rounded ends.
    #[default]
    Round,
    /// `ContainerShapeSquare`: a rounded rectangle (the radius depends on the size).
    Square,
}

/// Toggle state of a button.
pub(crate) enum ToggleState<'a> {
    /// A plain push button.
    None,
    /// Shows the selected/unselected colours; clicks do not change it.
    Display(bool),
    /// A toggle button bound to a flag that clicks flip.
    Bound(&'a mut bool),
}

impl ToggleState<'_> {
    pub(crate) fn selected(&self) -> Option<bool> {
        match self {
            ToggleState::None => None,
            ToggleState::Display(selected) => Some(*selected),
            ToggleState::Bound(selected) => Some(**selected),
        }
    }
}

/// How one side (start or end) of a segment in a connected row is rounded.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SideCorners {
    /// An outer edge of the row: fully rounded.
    Outer,
    /// An edge facing a neighbour.
    Inner {
        /// Radius at rest.
        idle: f32,
        /// Radius while hovered.
        hovered: f32,
        /// Radius while pressed.
        pressed: f32,
        /// Fully round while selected.
        selected_full: bool,
    },
}

impl SideCorners {
    /// The target radius for this side of a `height`-high segment.
    pub(crate) fn radius(self, height: f32, hovered: bool, pressed: bool, selected: bool) -> f32 {
        let full = height / 2.0;
        match self {
            SideCorners::Outer => full,
            SideCorners::Inner {
                idle,
                hovered: hover_radius,
                pressed: press_radius,
                selected_full,
            } => {
                if pressed {
                    press_radius
                } else if selected && selected_full {
                    full
                } else if hovered {
                    hover_radius
                } else {
                    idle
                }
            }
        }
    }
}

/// Corner rules for a segment of a button group or split button.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SegmentCorners {
    pub(crate) start: SideCorners,
    pub(crate) end: SideCorners,
    /// Use the selected shape whatever the toggle state (an open split
    /// button's trailing half).
    pub(crate) selected: bool,
}

/// Material Design button widget implementing Material Design 3 button specifications
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialButton<'a> {
    pub(crate) image: Option<Image<'a>>,
    pub(crate) text: Option<WidgetText>,
    pub(crate) shortcut_text: WidgetText,
    pub(crate) wrap_mode: Option<TextWrapMode>,
    pub(crate) variant: MaterialButtonVariant,
    pub(crate) size: Option<ButtonSize>,
    pub(crate) shape: ButtonShape,
    pub(crate) fill: Option<Color32>,
    pub(crate) stroke: Option<Stroke>,
    pub(crate) sense: Sense,
    pub(crate) frame: Option<bool>,
    pub(crate) min_size: Vec2,
    pub(crate) width: Option<f32>,
    pub(crate) padding: Option<(f32, f32)>,
    pub(crate) corner_radius: Option<CornerRadius>,
    pub(crate) segment: Option<SegmentCorners>,
    pub(crate) toggle: ToggleState<'a>,
    pub(crate) role: Role,
    pub(crate) image_tint_follows_text_color: bool,
    pub(crate) elevation: Option<Shadow>,
    pub(crate) disabled: bool,
    pub(crate) leading_icon: Option<String>,
    pub(crate) trailing_icon: Option<String>,
    pub(crate) leading_svg: Option<String>,
    pub(crate) trailing_svg: Option<String>,
    pub(crate) text_color: Option<Color32>,
    pub(crate) label: Option<String>,
}
