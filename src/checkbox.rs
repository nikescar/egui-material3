//! Material Design 3 checkbox.
//!
//! Geometry, colours and opacities follow Compose's `CheckboxTokens`:
//!
//! - an 18 dp container with 2 dp corners and a 2 dp outline drawn *inside*
//!   it when unselected, centred in a 48 dp touch target;
//! - a 40 dp circular state layer in `primary` (selected), `onSurface`
//!   (unselected) or `error`;
//! - selected: `primary` container, `onPrimary` mark; error: `error` /
//!   `onError`;
//! - unselected: `onSurfaceVariant` outline, `onSurface` when hovered,
//!   focused or pressed; `error` in the error state;
//! - disabled: `onSurface` at 38 % for the outline or the container, and the
//!   mark in `surface` (`SelectedDisabledIconColor`) so a disabled checked
//!   box still shows its mark.
//!
//! The mark draws itself in over 100 ms when checked and morphs between the
//! check and the indeterminate dash, as Compose's `TriStateCheckbox` does.
//! A three-state box is a [`ToggleableState`] (see
//! [`MaterialCheckbox::tri_state`]); AccessKit reports it as `CheckBox` with
//! `Toggled::True`, `False` or `Mixed`.

use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{ColorRole, InteractionState, TypeRole};
use crate::MaterialTheme;
use egui::{
    self, Color32, CornerRadius, Galley, Pos2, Rect, Response, Sense, Shape, Stroke, StrokeKind,
    Ui, Vec2, Widget,
};
use std::sync::Arc;

/// `CheckboxTokens.ContainerSize`: the visible box.
pub const CONTAINER_SIZE: f32 = 18.0;
/// `CheckboxTokens.ContainerShape`: 2 dp corners.
pub const CONTAINER_CORNER: u8 = 2;
/// `CheckboxTokens.UnselectedOutlineWidth`: drawn inside the container.
pub const OUTLINE_WIDTH: f32 = 2.0;
/// Width of the check mark and the indeterminate dash.
pub const MARK_STROKE_WIDTH: f32 = 2.0;
/// `CheckboxTokens.StateLayerSize`: the circular hover/focus/press layer.
pub const STATE_LAYER_SIZE: f32 = 40.0;
/// Minimum touch target of every selection control (48 dp).
pub const TOUCH_TARGET: f32 = 48.0;
/// Duration of the check draw-in and the box colour change (Compose's
/// `CheckAnimationDuration`, `MotionTokens.DurationShort2`).
const CHECK_ANIMATION_SECONDS: f32 = 0.1;

/// The state of a three-state checkbox (Compose's `ToggleableState`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ToggleableState {
    /// Checked.
    On,
    /// Unchecked.
    #[default]
    Off,
    /// Partly checked: some, not all, of the items it stands for.
    Indeterminate,
}

impl From<bool> for ToggleableState {
    fn from(on: bool) -> Self {
        if on {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl ToggleableState {
    /// The state after a click: `Off` becomes `On`, `On` becomes `Off`, and
    /// `Indeterminate` becomes `On`.
    pub fn toggled(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off | Self::Indeterminate => Self::On,
        }
    }
}

enum Value<'a> {
    Bool(&'a mut bool),
    Tri(&'a mut ToggleableState),
}

/// Material Design 3 checkbox.
///
/// ```rust
/// # use egui_material3::MaterialCheckbox;
/// # use egui_material3::checkbox::ToggleableState;
/// # egui::__run_test_ui(|ui| {
/// let mut checked = false;
/// ui.add(MaterialCheckbox::new(&mut checked, "Accept terms"));
///
/// // Shown as indeterminate; a click checks it.
/// let mut partial = false;
/// ui.add(MaterialCheckbox::new(&mut partial, "Select all").indeterminate(true));
///
/// // Three states held in the value itself.
/// let mut all = ToggleableState::Indeterminate;
/// ui.add(MaterialCheckbox::tri_state(&mut all, "All files"));
///
/// // Disabled and error states.
/// let mut locked = true;
/// ui.add(MaterialCheckbox::new(&mut locked, "Locked").enabled(false));
/// ui.add(MaterialCheckbox::new(&mut checked, "Required").is_error(true));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialCheckbox<'a> {
    value: Value<'a>,
    text: String,
    indeterminate: bool,
    enabled: bool,
    is_error: bool,
    check_color: Option<Color32>,
    fill_color: Option<Color32>,
    border_width: f32,
}

impl<'a> MaterialCheckbox<'a> {
    /// A two-state checkbox bound to `checked`, labelled `text`.
    pub fn new(checked: &'a mut bool, text: impl Into<String>) -> Self {
        Self::with_value(Value::Bool(checked), text.into())
    }

    /// A three-state checkbox bound to `state`, labelled `text`. A click
    /// moves it as [`ToggleableState::toggled`] does.
    pub fn tri_state(state: &'a mut ToggleableState, text: impl Into<String>) -> Self {
        Self::with_value(Value::Tri(state), text.into())
    }

    fn with_value(value: Value<'a>, text: String) -> Self {
        Self {
            value,
            text,
            indeterminate: false,
            enabled: true,
            is_error: false,
            check_color: None,
            fill_color: None,
            border_width: OUTLINE_WIDTH,
        }
    }

    /// Shows a two-state checkbox as indeterminate (a dash). A click checks
    /// it. Has no effect on a [`MaterialCheckbox::tri_state`] box, whose
    /// state carries this itself.
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Enables or disables the checkbox. A disabled checkbox is dimmed, is
    /// not focusable and ignores clicks.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Draws the checkbox in the error colours (`error` / `onError`).
    pub fn is_error(mut self, is_error: bool) -> Self {
        self.is_error = is_error;
        self
    }

    /// Overrides the mark colour (default `onPrimary`, or `onError`).
    pub fn check_color(mut self, color: Color32) -> Self {
        self.check_color = Some(color);
        self
    }

    /// Overrides the selected container colour (default `primary`, or
    /// `error`).
    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Overrides the unselected outline width (default 2 dp, drawn inside
    /// the container).
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    fn state(&self) -> ToggleableState {
        match &self.value {
            Value::Tri(state) => **state,
            Value::Bool(_) if self.indeterminate => ToggleableState::Indeterminate,
            Value::Bool(checked) => ToggleableState::from(**checked),
        }
    }
}

/// The label of a selection control, laid out in `BodyLarge`.
pub(crate) fn label_galley(ui: &Ui, text: &str, color: Color32) -> Option<Arc<Galley>> {
    if text.is_empty() {
        return None;
    }
    let job = TypeRole::BodyLarge
        .style()
        .layout_job(ui.ctx(), text, color);
    Some(ui.painter().layout_job(job))
}

/// Label colour of a selection control: `onSurface`, at 38 % when disabled.
pub(crate) fn label_color(theme: &MaterialTheme, enabled: bool) -> Color32 {
    let on_surface = theme.color(ColorRole::OnSurface);
    if enabled {
        on_surface
    } else {
        on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY)
    }
}

/// Paints the focus ring (`FocusIndicatorColor` = `secondary`) around
/// `rect`, a shape with full corners of height `height`, when `response`
/// has keyboard focus.
pub(crate) fn focus_ring(
    painter: &egui::Painter,
    theme: &MaterialTheme,
    response: &Response,
    rect: Rect,
    height: f32,
) {
    if response.has_focus() {
        crate::paint::paint_focus_ring(
            painter,
            rect,
            full_corner(height),
            theme.color(ColorRole::Secondary),
        );
    }
}

/// A full (circular) corner radius for a square of `size`.
pub(crate) fn full_corner(size: f32) -> CornerRadius {
    CornerRadius::same((size / 2.0).round().clamp(0.0, 255.0) as u8)
}

/// The three points of the check mark in a box of `size` at `min`, pulled
/// towards a centred dash by `gravitation` (0 = check, 1 = dash), from
/// Compose's `CheckDrawingCache`.
fn mark_points(min: Pos2, size: f32, gravitation: f32) -> [Pos2; 3] {
    let lerp = |a: f32, b: f32| a + (b - a) * gravitation;
    let at = |x: f32, y: f32| Pos2::new(min.x + size * x, min.y + size * y);
    [
        at(0.2, lerp(0.5, 0.5)),
        at(lerp(0.4, 0.5), lerp(0.7, 0.5)),
        at(0.8, lerp(0.3, 0.5)),
    ]
}

/// The first `fraction` of the polyline `points`, by length.
fn partial_polyline(points: [Pos2; 3], fraction: f32) -> Vec<Pos2> {
    let first = points[0].distance(points[1]);
    let second = points[1].distance(points[2]);
    let length = (first + second) * fraction.clamp(0.0, 1.0);
    if length <= first {
        let t = if first > 0.0 { length / first } else { 0.0 };
        vec![points[0], points[0].lerp(points[1], t)]
    } else {
        let t = if second > 0.0 {
            (length - first) / second
        } else {
            0.0
        };
        vec![points[0], points[1], points[1].lerp(points[2], t)]
    }
}

impl Widget for MaterialCheckbox<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let state = self.state();
        let text_color = label_color(&theme, self.enabled);
        let galley = label_galley(ui, &self.text, text_color);
        let label_width = galley.as_ref().map_or(0.0, |g| g.size().x);
        let desired_size = Vec2::new(TOUCH_TARGET + label_width, TOUCH_TARGET);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            crate::a11y::sense(self.enabled, Sense::click()),
        );

        if response.clicked() && self.enabled {
            match &mut self.value {
                Value::Tri(value) => **value = state.toggled(),
                Value::Bool(checked) => **checked = state.toggled() == ToggleableState::On,
            }
            response.mark_changed();
        }
        let state = self.state();
        crate::a11y::describe(
            &response,
            egui::Role::CheckBox,
            self.enabled,
            &self.text,
            match state {
                ToggleableState::On => Some(true),
                ToggleableState::Off => Some(false),
                ToggleableState::Indeterminate => None,
            },
        );

        let selected = state != ToggleableState::Off;
        let ctx = ui.ctx().clone();
        let t = ctx.animate_bool_with_time(
            response.id.with("selected"),
            selected,
            CHECK_ANIMATION_SECONDS,
        );
        let gravitation = ctx.animate_bool_with_time(
            response.id.with("indeterminate"),
            state == ToggleableState::Indeterminate,
            CHECK_ANIMATION_SECONDS,
        );

        let interaction = InteractionState::from_response(&response);
        let active = interaction.hovered || interaction.focused || interaction.pressed;
        let c = |role| theme.color(role);
        let disabled = |color: Color32| color.gamma_multiply(DISABLED_CONTENT_OPACITY);

        let (container, mark, unselected_outline, layer_color) = if !self.enabled {
            (
                disabled(c(ColorRole::OnSurface)),
                c(ColorRole::Surface),
                disabled(c(ColorRole::OnSurface)),
                c(ColorRole::OnSurface),
            )
        } else if self.is_error {
            (
                self.fill_color.unwrap_or(c(ColorRole::Error)),
                self.check_color.unwrap_or(c(ColorRole::OnError)),
                c(ColorRole::Error),
                c(ColorRole::Error),
            )
        } else {
            let outline = if active {
                c(ColorRole::OnSurface)
            } else {
                c(ColorRole::OnSurfaceVariant)
            };
            let layer = if selected {
                c(ColorRole::Primary)
            } else {
                c(ColorRole::OnSurface)
            };
            (
                self.fill_color.unwrap_or(c(ColorRole::Primary)),
                self.check_color.unwrap_or(c(ColorRole::OnPrimary)),
                outline,
                layer,
            )
        };

        let target = Rect::from_min_size(rect.min, Vec2::splat(TOUCH_TARGET));
        let center = target.center();
        let layer_rect = Rect::from_center_size(center, Vec2::splat(STATE_LAYER_SIZE));
        let box_rect = Rect::from_center_size(center, Vec2::splat(CONTAINER_SIZE));
        let painter = ui.painter();

        crate::paint::paint_state_layer(
            painter,
            layer_rect,
            full_corner(STATE_LAYER_SIZE),
            layer_color,
            interaction,
        );

        // The container fades in over the outline; the outline takes the
        // container colour as it does, so the two meet without a seam.
        if t > 0.0 {
            painter.rect_filled(box_rect, CONTAINER_CORNER, container.gamma_multiply(t));
        }
        if t < 1.0 && self.border_width > 0.0 {
            let outline = unselected_outline.lerp_to_gamma(container, t);
            painter.rect_stroke(
                box_rect,
                CONTAINER_CORNER,
                Stroke::new(self.border_width, outline),
                StrokeKind::Inside,
            );
        }

        // Checking draws the mark in; unchecking fades it with the box.
        let (draw_fraction, mark_alpha) = if selected { (t, 1.0) } else { (1.0, t) };
        if t > 0.0 {
            let points = mark_points(box_rect.min, CONTAINER_SIZE, gravitation);
            let line = partial_polyline(points, draw_fraction);
            painter.add(Shape::line(
                line,
                Stroke::new(MARK_STROKE_WIDTH, mark.gamma_multiply(mark_alpha)),
            ));
        }

        focus_ring(painter, &theme, &response, layer_rect, STATE_LAYER_SIZE);

        if let Some(galley) = galley {
            let pos = Pos2::new(target.max.x, rect.center().y - galley.size().y / 2.0);
            painter.galley(pos, galley, text_color);
        }

        response
    }
}

/// Shorthand for [`MaterialCheckbox::new`].
pub fn checkbox(checked: &mut bool, text: impl Into<String>) -> MaterialCheckbox<'_> {
    MaterialCheckbox::new(checked, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggled_cycles_like_compose() {
        assert_eq!(ToggleableState::Off.toggled(), ToggleableState::On);
        assert_eq!(ToggleableState::On.toggled(), ToggleableState::Off);
        assert_eq!(
            ToggleableState::Indeterminate.toggled(),
            ToggleableState::On
        );
    }

    #[test]
    fn mark_morphs_into_a_centred_dash() {
        let dash = mark_points(Pos2::ZERO, 10.0, 1.0);
        assert!(
            dash.iter().all(|p| (p.y - 5.0).abs() < 1e-4),
            "indeterminate mark is a horizontal dash at mid-height: {dash:?}"
        );
        let check = mark_points(Pos2::ZERO, 10.0, 0.0);
        assert!(
            (check[1].y - 7.0).abs() < 1e-4,
            "check mark vertex at 0.7 of the box: {check:?}"
        );
    }

    #[test]
    fn partial_polyline_draws_by_length() {
        let points = [
            Pos2::new(0.0, 0.0),
            Pos2::new(1.0, 0.0),
            Pos2::new(1.0, 1.0),
        ];
        let half = partial_polyline(points, 0.5);
        assert_eq!(half, vec![points[0], points[1]], "half of two equal legs");
        let quarter = partial_polyline(points, 0.25);
        assert_eq!(quarter.len(), 2, "a quarter stays on the first leg");
        assert!(
            (quarter[1].x - 0.5).abs() < 1e-5,
            "quarter ends mid first leg"
        );
        assert_eq!(partial_polyline(points, 1.0)[2], points[2], "full mark");
    }
}
