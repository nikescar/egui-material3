//! Material Design 3 switch.
//!
//! Geometry, colours and opacities follow Compose's `SwitchTokens`:
//!
//! - a 52 × 32 dp track with full corners in a 48 dp tall touch target, with
//!   a 2 dp outline drawn *inside* it when unselected;
//! - the thumb (handle) is 16 dp unselected, 24 dp selected or when showing
//!   an icon, and 28 dp while pressed. Its centre sits 16 dp from the
//!   track's start when off and 36 dp when on, whatever its size;
//! - the thumb moves and resizes with the fast spatial spring of the theme's
//!   motion scheme;
//! - selected: `primary` track, `onPrimary` thumb (`primaryContainer` when
//!   hovered, focused or pressed), `onPrimaryContainer` icon (`primary` when
//!   hovered, focused or pressed);
//! - unselected: `surfaceContainerHighest` track, `outline` outline and
//!   thumb (`onSurfaceVariant` when hovered, focused or pressed),
//!   `surfaceContainerHighest` icon;
//! - disabled: track at 12 %; selected thumb `surface` at full opacity with
//!   an `onSurface` icon at 38 %; unselected thumb `onSurface` at 38 %;
//! - a 40 dp circular state layer centred on the thumb, and a focus ring
//!   around the track.

use crate::checkbox::{focus_ring, full_corner, label_color, label_galley, TOUCH_TARGET};
use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{animate_value_with_spring, ColorRole, InteractionState, SpringSpeed};
use crate::MaterialTheme;
use egui::{
    self, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget,
};

/// `SwitchTokens.TrackWidth`.
pub const TRACK_WIDTH: f32 = 52.0;
/// `SwitchTokens.TrackHeight`.
pub const TRACK_HEIGHT: f32 = 32.0;
/// `SwitchTokens.TrackOutlineWidth`: drawn inside the track.
pub const TRACK_OUTLINE_WIDTH: f32 = 2.0;
/// `SwitchTokens.UnselectedHandleWidth`.
pub const UNSELECTED_HANDLE_SIZE: f32 = 16.0;
/// `SwitchTokens.SelectedHandleWidth` (and `HandleWidth`, with an icon).
pub const SELECTED_HANDLE_SIZE: f32 = 24.0;
/// `SwitchTokens.PressedHandleWidth`.
pub const PRESSED_HANDLE_SIZE: f32 = 28.0;
/// `SwitchTokens.SelectedIconSize` / `UnselectedIconSize`.
pub const ICON_SIZE: f32 = 16.0;
/// `SwitchTokens.StateLayerSize`.
pub const STATE_LAYER_SIZE: f32 = 40.0;
/// `SwitchTokens.DisabledTrackOpacity`.
pub const DISABLED_TRACK_OPACITY: f32 = 0.12;
/// Thumb centre, from the track's start, when off: half the track height.
pub const THUMB_CENTER_OFF: f32 = TRACK_HEIGHT / 2.0;
/// Thumb centre, from the track's start, when on.
pub const THUMB_CENTER_ON: f32 = TRACK_WIDTH - TRACK_HEIGHT / 2.0;
/// Gap between the track and the label.
const LABEL_GAP: f32 = 8.0;

/// Material Design 3 switch.
///
/// ```rust
/// # use egui_material3::MaterialSwitch;
/// # egui::__run_test_ui(|ui| {
/// let mut wifi = false;
/// ui.add(MaterialSwitch::new(&mut wifi).text("Wi-Fi"));
///
/// let mut locked = true;
/// ui.add(MaterialSwitch::new(&mut locked).text("Locked").enabled(false));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialSwitch<'a> {
    selected: &'a mut bool,
    text: Option<String>,
    enabled: bool,
    selected_icon: Option<char>,
    unselected_icon: Option<char>,
    show_track_outline: bool,
}

impl<'a> MaterialSwitch<'a> {
    /// A switch bound to `selected`.
    pub fn new(selected: &'a mut bool) -> Self {
        Self {
            selected,
            text: None,
            enabled: true,
            selected_icon: None,
            unselected_icon: None,
            show_track_outline: true,
        }
    }

    /// A label after the switch; also its accessible name.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Enables or disables the switch. A disabled switch is dimmed, is not
    /// focusable and ignores clicks.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// An icon on the thumb while on. The thumb is then 24 dp.
    pub fn selected_icon(mut self, icon: char) -> Self {
        self.selected_icon = Some(icon);
        self
    }

    /// An icon on the thumb while off. The thumb is then 24 dp, not 16.
    pub fn unselected_icon(mut self, icon: char) -> Self {
        self.unselected_icon = Some(icon);
        self
    }

    /// Icons for both states.
    pub fn with_icons(mut self, selected: char, unselected: char) -> Self {
        self.selected_icon = Some(selected);
        self.unselected_icon = Some(unselected);
        self
    }

    /// Whether to draw the unselected track outline (default `true`).
    pub fn show_track_outline(mut self, show: bool) -> Self {
        self.show_track_outline = show;
        self
    }
}

/// The colours of one switch state.
struct Colors {
    track: Color32,
    outline: Color32,
    handle: Color32,
    icon: Color32,
}

fn colors(theme: &MaterialTheme, selected: bool, enabled: bool, active: bool) -> Colors {
    let c = |role| theme.color(role);
    let dim = |color: Color32| color.gamma_multiply(DISABLED_CONTENT_OPACITY);
    match (enabled, selected) {
        (false, true) => Colors {
            track: c(ColorRole::OnSurface).gamma_multiply(DISABLED_TRACK_OPACITY),
            outline: Color32::TRANSPARENT,
            handle: c(ColorRole::Surface),
            icon: dim(c(ColorRole::OnSurface)),
        },
        (false, false) => Colors {
            track: c(ColorRole::SurfaceContainerHighest).gamma_multiply(DISABLED_TRACK_OPACITY),
            outline: c(ColorRole::OnSurface).gamma_multiply(DISABLED_TRACK_OPACITY),
            handle: dim(c(ColorRole::OnSurface)),
            icon: dim(c(ColorRole::SurfaceContainerHighest)),
        },
        (true, true) => Colors {
            track: c(ColorRole::Primary),
            outline: Color32::TRANSPARENT,
            handle: if active {
                c(ColorRole::PrimaryContainer)
            } else {
                c(ColorRole::OnPrimary)
            },
            icon: if active {
                c(ColorRole::Primary)
            } else {
                c(ColorRole::OnPrimaryContainer)
            },
        },
        (true, false) => Colors {
            track: c(ColorRole::SurfaceContainerHighest),
            outline: c(ColorRole::Outline),
            handle: if active {
                c(ColorRole::OnSurfaceVariant)
            } else {
                c(ColorRole::Outline)
            },
            icon: c(ColorRole::SurfaceContainerHighest),
        },
    }
}

impl Widget for MaterialSwitch<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let text_color = label_color(&theme, self.enabled);
        let galley = self
            .text
            .as_deref()
            .and_then(|text| label_galley(ui, text, text_color));
        let label_width = galley.as_ref().map_or(0.0, |g| LABEL_GAP + g.size().x);
        let desired_size = Vec2::new(TRACK_WIDTH + label_width, TOUCH_TARGET);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            crate::a11y::sense(self.enabled, Sense::click()),
        );

        if response.clicked() && self.enabled {
            *self.selected = !*self.selected;
            response.mark_changed();
        }
        let selected = *self.selected;
        crate::a11y::toggle(
            &response,
            egui::Role::Switch,
            self.enabled,
            self.text.as_deref().unwrap_or_default(),
            selected,
        );

        let interaction = InteractionState::from_response(&response);
        let active = interaction.hovered || interaction.focused || interaction.pressed;
        let icon = if selected {
            self.selected_icon
        } else {
            self.unselected_icon
        };

        // Position and size both follow the fast spatial spring.
        let spring = theme.motion.spatial(SpringSpeed::Fast);
        let ctx = ui.ctx().clone();
        let progress = animate_value_with_spring(
            &ctx,
            response.id.with("thumb_position"),
            if selected { 1.0 } else { 0.0 },
            spring,
        );
        let target_size = if interaction.pressed {
            PRESSED_HANDLE_SIZE
        } else if selected || icon.is_some() {
            SELECTED_HANDLE_SIZE
        } else {
            UNSELECTED_HANDLE_SIZE
        };
        let handle_size =
            animate_value_with_spring(&ctx, response.id.with("thumb_size"), target_size, spring);

        let track_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - TRACK_HEIGHT / 2.0),
            Vec2::new(TRACK_WIDTH, TRACK_HEIGHT),
        );
        let (start, end) = match theme.layout_direction {
            crate::tokens::LayoutDirection::Ltr => (
                track_rect.min.x + THUMB_CENTER_OFF,
                track_rect.min.x + THUMB_CENTER_ON,
            ),
            crate::tokens::LayoutDirection::Rtl => (
                track_rect.max.x - THUMB_CENTER_OFF,
                track_rect.max.x - THUMB_CENTER_ON,
            ),
        };
        let thumb_center = Pos2::new(start + (end - start) * progress, track_rect.center().y);

        let off = colors(&theme, false, self.enabled, active);
        let on = colors(&theme, true, self.enabled, active);
        let t = progress.clamp(0.0, 1.0);
        let mix = |a: Color32, b: Color32| a.lerp_to_gamma(b, t);
        let track = mix(off.track, on.track);
        let outline = mix(off.outline, on.outline);
        let handle = mix(off.handle, on.handle);
        let icon_color = if selected { on.icon } else { off.icon };

        let painter = ui.painter();
        let track_corner = full_corner(TRACK_HEIGHT);
        painter.rect_filled(track_rect, track_corner, track);
        if self.show_track_outline && outline.a() > 0 {
            painter.rect_stroke(
                track_rect,
                track_corner,
                Stroke::new(TRACK_OUTLINE_WIDTH, outline),
                StrokeKind::Inside,
            );
        }

        let layer_color = if selected {
            theme.color(ColorRole::Primary)
        } else {
            theme.color(ColorRole::OnSurface)
        };
        crate::paint::paint_state_layer(
            painter,
            Rect::from_center_size(thumb_center, Vec2::splat(STATE_LAYER_SIZE)),
            full_corner(STATE_LAYER_SIZE),
            layer_color,
            interaction,
        );

        painter.circle_filled(thumb_center, handle_size / 2.0, handle);
        if let Some(icon) = icon {
            painter.text(
                thumb_center,
                egui::Align2::CENTER_CENTER,
                icon.to_string(),
                FontId::proportional(ICON_SIZE),
                icon_color,
            );
        }

        focus_ring(painter, &theme, &response, track_rect, TRACK_HEIGHT);

        if let Some(galley) = galley {
            let pos = Pos2::new(
                track_rect.max.x + LABEL_GAP,
                rect.center().y - galley.size().y / 2.0,
            );
            painter.galley(pos, galley, text_color);
        }

        response
    }
}

/// Shorthand for [`MaterialSwitch::new`].
pub fn switch(selected: &mut bool) -> MaterialSwitch<'_> {
    MaterialSwitch::new(selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thumb_centres_match_compose() {
        assert_eq!(THUMB_CENTER_OFF, 16.0, "off thumb centre");
        assert_eq!(THUMB_CENTER_ON, 36.0, "on thumb centre");
    }
}
