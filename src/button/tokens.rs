//! Button tokens: per-size metrics, per-style colours and elevations.
//!
//! Sizes from Compose's `ButtonXSmallTokens` .. `ButtonXLargeTokens` and
//! `BaselineButtonTokens`; colours and elevations from `FilledButtonTokens`,
//! `FilledTonalButtonTokens`, `ElevatedButtonTokens`, `OutlinedButtonTokens`
//! and `TextButtonTokens` (the `Selected*` / `Unselected*` sets are the
//! toggle-button colours).

use egui::Color32;

use super::{ButtonShape, ButtonSize, MaterialButtonVariant};
use crate::tokens::state::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY};
use crate::tokens::{ColorRole, CornerSize, Elevation, InteractionState, TypeRole};
use crate::MaterialTheme;

/// The geometry and type of one button size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonMetrics {
    /// `ContainerHeight`.
    pub height: f32,
    /// `LeadingSpace`: padding before the content.
    pub leading_space: f32,
    /// `TrailingSpace`: padding after the content.
    pub trailing_space: f32,
    /// `IconSize`.
    pub icon_size: f32,
    /// `IconLabelSpace`: gap between an icon and the label.
    pub icon_label_space: f32,
    /// `OutlinedOutlineWidth`.
    pub outline_width: f32,
    /// `LabelText`.
    pub label: TypeRole,
    /// `ContainerShapeSquare`.
    pub square_shape: CornerSize,
    /// `PressedContainerShape`.
    pub pressed_shape: CornerSize,
    /// `SelectedContainerShapeRound`: a selected round toggle button.
    pub selected_round_shape: CornerSize,
    /// `SelectedContainerShapeSquare`: a selected square toggle button.
    pub selected_square_shape: CornerSize,
    /// Whether the side padding shrinks next to an icon (the baseline
    /// button's `ButtonWithIconContentPadding`); the sized tokens keep it.
    pub icon_padding_shrinks: bool,
}

impl ButtonMetrics {
    /// `BaselineButtonTokens`: the button with no size set.
    pub const BASELINE: ButtonMetrics = ButtonMetrics {
        height: 40.0,
        leading_space: 24.0,
        trailing_space: 24.0,
        icon_size: 20.0,
        icon_label_space: 8.0,
        outline_width: 1.0,
        label: TypeRole::LabelLarge,
        square_shape: CornerSize::Medium,
        pressed_shape: CornerSize::Small,
        selected_round_shape: CornerSize::Medium,
        selected_square_shape: CornerSize::Full,
        icon_padding_shrinks: true,
    };

    /// The metrics for `size`, or the baseline for `None`.
    pub const fn of(size: Option<ButtonSize>) -> ButtonMetrics {
        match size {
            None => Self::BASELINE,
            Some(size) => size.metrics(),
        }
    }

    /// The radius of a resting button of this size and `shape`.
    pub const fn shape(&self, shape: ButtonShape) -> CornerSize {
        match shape {
            ButtonShape::Round => CornerSize::Full,
            ButtonShape::Square => self.square_shape,
        }
    }

    /// The radius of a selected toggle button of this size and `shape`.
    pub const fn selected_shape(&self, shape: ButtonShape) -> CornerSize {
        match shape {
            ButtonShape::Round => self.selected_round_shape,
            ButtonShape::Square => self.selected_square_shape,
        }
    }
}

/// Sized metrics; `shapes` is square, pressed, selected round, selected square.
const fn m(
    height: f32,
    space: f32,
    icon_size: f32,
    icon_label_space: f32,
    outline_width: f32,
    label: TypeRole,
    shapes: [CornerSize; 4],
) -> ButtonMetrics {
    let [square_shape, pressed_shape, selected_round_shape, selected_square_shape] = shapes;
    ButtonMetrics {
        height,
        leading_space: space,
        trailing_space: space,
        icon_size,
        icon_label_space,
        outline_width,
        label,
        square_shape,
        pressed_shape,
        selected_round_shape,
        selected_square_shape,
        icon_padding_shrinks: false,
    }
}

impl ButtonSize {
    /// The token metrics of this size.
    pub const fn metrics(self) -> ButtonMetrics {
        match self {
            ButtonSize::ExtraSmall => m(
                32.0,
                12.0,
                20.0,
                4.0,
                1.0,
                TypeRole::LabelLarge,
                [
                    CornerSize::Medium,
                    CornerSize::Small,
                    CornerSize::Medium,
                    CornerSize::Full,
                ],
            ),
            ButtonSize::Small => m(
                40.0,
                16.0,
                20.0,
                8.0,
                1.0,
                TypeRole::LabelLarge,
                [
                    CornerSize::Medium,
                    CornerSize::Small,
                    CornerSize::Medium,
                    // `ButtonSmallTokens.SelectedContainerShapeSquare` is
                    // CornerMedium, unlike the other sizes.
                    CornerSize::Medium,
                ],
            ),
            ButtonSize::Medium => m(
                56.0,
                24.0,
                24.0,
                8.0,
                1.0,
                TypeRole::TitleMedium,
                [
                    CornerSize::Large,
                    CornerSize::Medium,
                    CornerSize::Large,
                    CornerSize::Full,
                ],
            ),
            ButtonSize::Large => m(
                96.0,
                48.0,
                32.0,
                12.0,
                2.0,
                TypeRole::HeadlineSmall,
                [
                    CornerSize::ExtraLarge,
                    CornerSize::Large,
                    CornerSize::ExtraLarge,
                    CornerSize::Full,
                ],
            ),
            ButtonSize::ExtraLarge => m(
                136.0,
                64.0,
                40.0,
                16.0,
                3.0,
                TypeRole::HeadlineLarge,
                [
                    CornerSize::ExtraLarge,
                    CornerSize::Large,
                    CornerSize::ExtraLarge,
                    CornerSize::Full,
                ],
            ),
        }
    }
}

/// The colours of a button in one state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonColors {
    /// Container fill (transparent for text and outlined buttons).
    pub container: Color32,
    /// Label and icon colour; also the state-layer colour.
    pub content: Color32,
    /// Outline colour, for outlined buttons.
    pub outline: Option<Color32>,
}

impl MaterialButtonVariant {
    /// The token colours for this style. `selected` is `None` for a push
    /// button, or the state of a toggle button.
    pub fn colors(
        self,
        theme: &MaterialTheme,
        selected: Option<bool>,
        enabled: bool,
    ) -> ButtonColors {
        use ColorRole as R;
        use MaterialButtonVariant as V;
        let c = |role| theme.color(role);
        let (container, content, outline) = match (self, selected) {
            (V::Filled, None | Some(true)) => (c(R::Primary), c(R::OnPrimary), None),
            (V::Filled, Some(false)) => (c(R::SurfaceContainer), c(R::OnSurfaceVariant), None),
            (V::FilledTonal, None | Some(false)) => {
                (c(R::SecondaryContainer), c(R::OnSecondaryContainer), None)
            }
            (V::FilledTonal, Some(true)) => (c(R::Secondary), c(R::OnSecondary), None),
            (V::Elevated, None | Some(false)) => (c(R::SurfaceContainerLow), c(R::Primary), None),
            (V::Elevated, Some(true)) => (c(R::Primary), c(R::OnPrimary), None),
            (V::Outlined, None | Some(false)) => (
                Color32::TRANSPARENT,
                c(R::OnSurfaceVariant),
                Some(c(R::OutlineVariant)),
            ),
            (V::Outlined, Some(true)) => (c(R::InverseSurface), c(R::InverseOnSurface), None),
            (V::Text, None | Some(false)) => (Color32::TRANSPARENT, c(R::Primary), None),
            // Compose has no text toggle button; a selected one takes the
            // tonal container so the state stays visible.
            (V::Text, Some(true)) => (c(R::SecondaryContainer), c(R::OnSecondaryContainer), None),
        };
        if enabled {
            return ButtonColors {
                container,
                content,
                outline,
            };
        }
        // Disabled: `DisabledContainerColor` OnSurface @ 0.10 where there is a
        // container, `DisabledLabelTextColor` OnSurface @ 0.38, and
        // `DisabledOutlineColor` OutlineVariant.
        let on_surface = c(R::OnSurface);
        let disabled_container = on_surface.gamma_multiply(DISABLED_CONTAINER_OPACITY);
        let container = if container == Color32::TRANSPARENT {
            Color32::TRANSPARENT
        } else {
            disabled_container
        };
        ButtonColors {
            container,
            content: on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY),
            outline: outline.map(|_| c(R::OutlineVariant)),
        }
    }

    /// The container elevation for `state` (`*ContainerElevation` tokens).
    pub fn elevation(self, state: InteractionState, enabled: bool) -> Elevation {
        if !enabled {
            return Elevation::Level0;
        }
        match self {
            MaterialButtonVariant::Elevated => {
                if state.pressed || state.focused || !state.hovered {
                    Elevation::Level1
                } else {
                    Elevation::Level2
                }
            }
            MaterialButtonVariant::Filled | MaterialButtonVariant::FilledTonal => {
                if state.hovered && !state.pressed && !state.focused {
                    Elevation::Level1
                } else {
                    Elevation::Level0
                }
            }
            MaterialButtonVariant::Outlined | MaterialButtonVariant::Text => Elevation::Level0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_tokens_match_compose() {
        let expected = [
            (ButtonSize::ExtraSmall, 32.0, 12.0, 20.0, 1.0),
            (ButtonSize::Small, 40.0, 16.0, 20.0, 1.0),
            (ButtonSize::Medium, 56.0, 24.0, 24.0, 1.0),
            (ButtonSize::Large, 96.0, 48.0, 32.0, 2.0),
            (ButtonSize::ExtraLarge, 136.0, 64.0, 40.0, 3.0),
        ];
        for (size, height, space, icon, outline) in expected {
            let m = size.metrics();
            assert_eq!(m.height, height, "{size:?} height");
            assert_eq!(m.leading_space, space, "{size:?} leading");
            assert_eq!(m.trailing_space, space, "{size:?} trailing");
            assert_eq!(m.icon_size, icon, "{size:?} icon");
            assert_eq!(m.outline_width, outline, "{size:?} outline");
        }
    }

    #[test]
    fn hovered_elevated_button_rises() {
        let hovered = InteractionState {
            hovered: true,
            ..Default::default()
        };
        let v = MaterialButtonVariant::Elevated;
        assert_eq!(
            v.elevation(InteractionState::default(), true),
            Elevation::Level1
        );
        assert_eq!(v.elevation(hovered, true), Elevation::Level2);
        assert_eq!(v.elevation(hovered, false), Elevation::Level0);
    }
}
