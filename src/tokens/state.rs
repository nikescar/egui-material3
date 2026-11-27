//! State layers: the translucent overlay that shows hover, focus, press
//! and drag.
//!
//! Opacities from Compose's `StateLayerTokens.kt` (hover 0.08, focus 0.10,
//! pressed 0.10, dragged 0.16) and the disabled opacities that every
//! component token file repeats (`FilledButtonTokens.DisabledContainerOpacity`
//! 0.1, `DisabledLabelTextOpacity` 0.38). The layer is painted over the
//! container and under the content, in the content colour.

/// `StateLayerTokens.HoverOpacity`.
pub const HOVER_OPACITY: f32 = 0.08;
/// `StateLayerTokens.FocusOpacity`.
pub const FOCUS_OPACITY: f32 = 0.10;
/// `StateLayerTokens.PressedOpacity`.
pub const PRESSED_OPACITY: f32 = 0.10;
/// `StateLayerTokens.DraggedOpacity`.
pub const DRAGGED_OPACITY: f32 = 0.16;
/// `DisabledLabelTextOpacity` / `DisabledIconOpacity`: disabled content.
pub const DISABLED_CONTENT_OPACITY: f32 = 0.38;
/// `DisabledContainerOpacity`: a disabled container.
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.10;

/// The interaction state of a widget, for choosing a state-layer opacity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct InteractionState {
    /// The pointer is over the widget.
    pub hovered: bool,
    /// The widget has keyboard focus.
    pub focused: bool,
    /// The widget is being pressed.
    pub pressed: bool,
    /// The widget is being dragged.
    pub dragged: bool,
}

impl InteractionState {
    /// Reads the state from a response. A disabled response has none.
    pub fn from_response(response: &egui::Response) -> Self {
        if !response.enabled() {
            return Self::default();
        }
        Self {
            hovered: response.hovered(),
            focused: response.has_focus(),
            pressed: response.is_pointer_button_down_on(),
            dragged: response.dragged() && response.drag_delta() != egui::Vec2::ZERO,
        }
    }

    /// The state-layer opacity: the strongest active state wins (dragged,
    /// then pressed, then focused, then hovered), or 0 when idle.
    pub fn opacity(self) -> f32 {
        if self.dragged {
            DRAGGED_OPACITY
        } else if self.pressed {
            PRESSED_OPACITY
        } else if self.focused {
            FOCUS_OPACITY
        } else if self.hovered {
            HOVER_OPACITY
        } else {
            0.0
        }
    }
}
