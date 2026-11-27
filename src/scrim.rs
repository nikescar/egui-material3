// ============================================================================
// Module:       scrim
// Description:  The modal scrim, the modal layer and enter/exit transitions
//               shared by dialogs and sheets.
//
// Dependencies: egui; crate::material_theme, crate::tokens
// ============================================================================

//! The scrim behind modal surfaces, and the modal layer it belongs to.
//!
//! Colour and opacity from Compose's `ScrimTokens.kt`
//! (`ContainerColor = ColorSchemeKeyTokens.Scrim`, `ContainerOpacity =
//! 0.32f`).
//!
//! A modal dialog or sheet paints its scrim and its surface in **one**
//! layer: an egui [`Modal`] area in [`egui::Order::Foreground`]. That puts
//! the scrim above every window and panel of the app, and keeps the scrim
//! of a second modal above the surface of the first, so stacked modals
//! layer correctly. egui's modal layer also limits pointer input and
//! keyboard focus to the top modal, which is what traps Tab inside a
//! dialog, and only the top modal sees Escape.
//!
//! [`show_modal`] is the helper the components use; custom modal surfaces
//! can use it too.

use std::time::Duration;

use egui::{Area, Color32, Context, Frame, Id, Key, Modal, Modifiers, Response, Ui};

use crate::tokens::{ColorRole, CubicBezier};
use crate::MaterialTheme;

/// `ScrimTokens.ContainerOpacity`: the scrim is the `scrim` role at 32 %.
pub const SCRIM_OPACITY: f32 = 0.32;

/// The scrim colour for `theme`, faded by `visibility` (0 hidden, 1 fully
/// shown; used while a modal enters or leaves).
pub fn scrim_color(theme: &MaterialTheme, visibility: f32) -> Color32 {
    theme
        .color(ColorRole::Scrim)
        .gamma_multiply(SCRIM_OPACITY * visibility.clamp(0.0, 1.0))
}

/// What happened in a modal layer this frame.
pub struct ModalLayerResponse<T> {
    /// What the content closure returned.
    pub inner: T,
    /// The response of the layer's content.
    pub response: Response,
    /// The scrim (outside the surface) was clicked.
    pub scrim_clicked: bool,
    /// Escape was pressed while this was the top modal (and no popup was
    /// open). The key is consumed, so modals below do not see it.
    pub escape_pressed: bool,
}

/// An [`Area`] set up for a modal surface (foreground, centred). Change its
/// anchor or position before passing it to [`show_modal`].
pub fn modal_area(id: Id) -> Area {
    Modal::default_area(id)
}

/// Shows `content` in a modal layer over a scrim of colour `scrim`.
///
/// `content` paints its own surface: the layer adds no frame. When
/// `interactive` is false (a modal that is animating out) the layer
/// neither blocks the app nor takes input; the scrim is only painted.
pub fn show_modal<T>(
    ctx: &Context,
    area: Area,
    scrim: Color32,
    interactive: bool,
    content: impl FnOnce(&mut Ui) -> T,
) -> ModalLayerResponse<T> {
    if !interactive {
        let shown = area.interactable(false).show(ctx, |ui| {
            ui.painter()
                .rect_filled(ui.ctx().content_rect(), 0.0, scrim);
            content(ui)
        });
        return ModalLayerResponse {
            inner: shown.inner,
            response: shown.response,
            scrim_clicked: false,
            escape_pressed: false,
        };
    }
    let shown = Modal::new(area.layer().id)
        .area(area)
        .frame(Frame::NONE)
        .backdrop_color(scrim)
        .show(ctx, content);
    let escape_pressed = shown.is_top_modal
        && !shown.any_popup_open
        && ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape));
    ModalLayerResponse {
        scrim_clicked: shown.backdrop_response.clicked(),
        inner: shown.inner,
        response: shown.response,
        escape_pressed,
    }
}

/// A timed transition between hidden (0) and shown (1), with separate
/// enter and exit timing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transition {
    /// Duration of the enter (0 → 1) transition.
    pub enter_duration: Duration,
    /// Easing of the enter transition.
    pub enter_easing: CubicBezier,
    /// Duration of the exit (1 → 0) transition.
    pub exit_duration: Duration,
    /// Easing of the exit transition.
    pub exit_easing: CubicBezier,
}

impl Transition {
    /// Emphasized motion for a surface entering and leaving the screen:
    /// enter with `MotionTokens.EasingEmphasizedDecelerateCubicBezier`
    /// over `MotionTokens.DurationMedium4` (400 ms), exit with
    /// `EasingEmphasizedAccelerateCubicBezier` over `DurationShort4`
    /// (200 ms).
    pub const EMPHASIZED: Transition = Transition {
        enter_duration: crate::tokens::motion::DURATION_MEDIUM4,
        enter_easing: CubicBezier::EMPHASIZED_DECELERATE,
        exit_duration: crate::tokens::motion::DURATION_SHORT4,
        exit_easing: CubicBezier::EMPHASIZED_ACCELERATE,
    };

    /// No motion: jumps straight to the end state.
    pub const NONE: Transition = Transition {
        enter_duration: Duration::ZERO,
        enter_easing: CubicBezier::LINEAR,
        exit_duration: Duration::ZERO,
        exit_easing: CubicBezier::LINEAR,
    };

    /// Advances the transition keyed by `id` towards `shown` and returns
    /// its eased value in `0..=1`. The first call while `shown` starts the
    /// enter transition from 0; the first call while hidden returns 0.
    /// Repaints are requested until it settles.
    pub fn animate(&self, ctx: &Context, id: Id, shown: bool) -> f32 {
        let now = ctx.input(|i| i.time);
        let target = if shown { 1.0 } else { 0.0 };
        let previous = ctx.data(|d| d.get_temp::<TransitionState>(id));
        let state = match previous {
            Some(s) if s.shown == shown => s,
            Some(s) => TransitionState {
                shown,
                start_time: now,
                from: s.value(self, now),
            },
            None => TransitionState {
                shown,
                start_time: now,
                from: 0.0,
            },
        };
        ctx.data_mut(|d| d.insert_temp(id, state));
        let value = state.value(self, now);
        if (value - target).abs() > f32::EPSILON {
            ctx.request_repaint();
        }
        value
    }
}

#[derive(Clone, Copy, Debug)]
struct TransitionState {
    shown: bool,
    start_time: f64,
    from: f32,
}

impl TransitionState {
    fn value(&self, transition: &Transition, now: f64) -> f32 {
        let (to, duration, easing) = if self.shown {
            (1.0, transition.enter_duration, transition.enter_easing)
        } else {
            (0.0, transition.exit_duration, transition.exit_easing)
        };
        // Scale the duration by the distance left, so an interrupted
        // transition reverses at the same speed.
        let distance = (to - self.from).abs();
        let seconds = duration.as_secs_f32() * distance;
        if seconds <= 0.0 {
            return to;
        }
        let fraction = (((now - self.start_time) as f32) / seconds).clamp(0.0, 1.0);
        self.from + (to - self.from) * easing.transform(fraction)
    }
}

/// Remembers which widget had focus when a modal opened, so it can be
/// given back when the modal closes.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct FocusReturn {
    was_open: bool,
    previous: Option<Id>,
    /// Focus to give back on the frame after closing: on the closing frame
    /// the app is still behind the modal layer, and egui drops focus from
    /// widgets there.
    pending: Option<Id>,
}

impl FocusReturn {
    /// Call once per frame with whether the modal is open. On opening, the
    /// focused widget outside the modal is remembered and loses focus, so
    /// Tab starts inside the modal; on closing (and again on the next
    /// frame, once the app is out from under the modal layer), focus goes
    /// back to it.
    pub(crate) fn update(ctx: &Context, id: Id, open: bool) {
        let state_id = id.with("focus_return");
        let state = ctx
            .data(|d| d.get_temp::<FocusReturn>(state_id))
            .unwrap_or_default();
        let next = match (state.was_open, open) {
            (false, true) => {
                let previous = ctx.memory(|m| m.focused());
                if let Some(previous) = previous {
                    ctx.memory_mut(|m| m.surrender_focus(previous));
                }
                FocusReturn {
                    was_open: true,
                    previous,
                    pending: None,
                }
            }
            (true, false) => {
                if let Some(previous) = state.previous {
                    ctx.memory_mut(|m| m.request_focus(previous));
                }
                FocusReturn {
                    pending: state.previous,
                    ..FocusReturn::default()
                }
            }
            (false, false) => {
                if let Some(pending) = state.pending {
                    ctx.memory_mut(|m| m.request_focus(pending));
                }
                FocusReturn::default()
            }
            (true, true) => state,
        };
        ctx.data_mut(|d| d.insert_temp(state_id, next));
    }
}

/// Paints a surface (fill, corners and the shadows of `level`) around
/// `content`, which is laid out inside `margin`. The shadows go under the
/// fill, sized to the finished surface.
pub(crate) fn elevated_surface<R>(
    ui: &mut Ui,
    fill: Color32,
    corner_radius: egui::CornerRadius,
    level: crate::tokens::Elevation,
    margin: egui::Margin,
    content: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    let shadow_slot = ui.painter().add(egui::Shape::Noop);
    let shown = Frame::new()
        .fill(fill)
        .corner_radius(corner_radius)
        .inner_margin(margin)
        .show(ui, content);
    let shadow_color = MaterialTheme::of(ui.ctx()).color(ColorRole::Shadow);
    let shadows: Vec<egui::Shape> = level
        .shadows(shadow_color)
        .iter()
        .filter(|s| s.color != Color32::TRANSPARENT)
        .map(|s| egui::Shape::from(s.as_shape(shown.response.rect, corner_radius)))
        .collect();
    ui.painter().set(shadow_slot, egui::Shape::Vec(shadows));
    shown
}

/// A [`egui::Margin`] from `[left, right, top, bottom]` points.
pub(crate) fn margin(padding: [f32; 4]) -> egui::Margin {
    let to_i8 = |v: f32| v.round().clamp(0.0, f32::from(i8::MAX)) as i8;
    let [left, right, top, bottom] = padding;
    egui::Margin {
        left: to_i8(left),
        right: to_i8(right),
        top: to_i8(top),
        bottom: to_i8(bottom),
    }
}
