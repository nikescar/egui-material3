// ============================================================================
// Module:       slider
// Description:  M3 Expressive sliders: single, centred, range and vertical.
//
// Dependencies: egui; crate::{a11y, tokens, material_theme}
// ============================================================================

//! Material 3 Expressive sliders: [`MaterialSlider`] (standard, centred,
//! vertical) and [`MaterialRangeSlider`].
//!
//! # Tokens
//!
//! From Compose's `SliderTokens.kt` (androidx-main, the Expressive token
//! set) and `Slider.kt`:
//!
//! - track: active `Primary`, inactive `SecondaryContainer`, `CornerFull`
//!   outer corners, 2 dp inside corners next to the handle
//!   (`TrackInsideCornerSize`);
//! - handle: `Primary`, 4 dp wide (`HandleWidth`), narrowing to 2 dp when
//!   pressed or focused (`PressedHandleWidth`, `FocusHandleWidth`), with a
//!   6 dp gap to the track on each side (`ActiveHandleLeadingSpace`);
//! - stop indicators: 4 dp (`StopIndicatorSize`), `OnPrimary` on the active
//!   track and `OnSecondaryContainer` on the inactive one;
//! - value indicator: `InverseSurface` with `InverseOnSurface` LabelLarge
//!   text, 12 dp above the handle (`ValueIndicatorActiveBottomSpace`);
//! - disabled: active track and handle `OnSurface` at 0.38, inactive track
//!   `OnSurface` at 0.12.
//!
//! The track sizes S to XL ([`SliderSize`]) are not in the Compose token
//! files on androidx-main; they come from the M3 Expressive slider spec on
//! m3.material.io (a stated exception to the Compose-token rule).
//!
//! # Keyboard
//!
//! A focused slider moves by one step (or 1% of the range without steps)
//! with the arrow keys, by ten with Page Up / Page Down, and to the ends
//! with Home / End. In right-to-left layouts Left and Right swap.

use std::ops::RangeInclusive;

use egui::{
    self, Align2, Color32, CornerRadius, Id, Key, LayerId, Modifiers, Order, Pos2, Rect, Response,
    Sense, Ui, Vec2, Widget,
};

use crate::tokens::{ColorRole, LayoutDirection, TypeRole};
use crate::MaterialTheme;

/// `SliderTokens.HandleWidth`: 4 dp.
pub const HANDLE_WIDTH: f32 = 4.0;
/// `SliderTokens.PressedHandleWidth` and `FocusHandleWidth`: 2 dp.
pub const HANDLE_PRESSED_WIDTH: f32 = 2.0;
/// `SliderTokens.ActiveHandleLeadingSpace` / `TrailingSpace`
/// (`MeasurementTokens.Space75`): 6 dp between the handle and the track.
pub const HANDLE_GAP: f32 = 6.0;
/// `Slider.kt` `TrackInsideCornerSize`: 2 dp.
pub const TRACK_INSIDE_CORNER: f32 = 2.0;
/// `SliderTokens.StopIndicatorSize`: 4 dp.
pub const STOP_INDICATOR_SIZE: f32 = 4.0;
/// `SliderTokens.ValueIndicatorActiveBottomSpace` (`Space150`): 12 dp.
pub const VALUE_INDICATOR_BOTTOM_SPACE: f32 = 12.0;
/// The value indicator container: 48 x 44 dp (m3.material.io slider spec).
const VALUE_INDICATOR_SIZE: Vec2 = Vec2::new(48.0, 44.0);
/// Width reserved to the right of a horizontal slider for its label and
/// value text.
const LABEL_AREA: f32 = 100.0;
/// Width of the legacy round thumb.
const ROUND_THUMB_DIAMETER: f32 = 20.0;

/// The track size of an Expressive slider.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SliderSize {
    /// 16 dp track (`SliderTokens.InactiveTrackHeight`), 44 dp handle.
    #[default]
    ExtraSmall,
    /// 24 dp track, 44 dp handle.
    Small,
    /// 40 dp track, 52 dp handle, room for a 24 dp inset icon.
    Medium,
    /// 56 dp track, 68 dp handle, room for a 24 dp inset icon.
    Large,
    /// 96 dp track, 108 dp handle, room for a 32 dp inset icon.
    ExtraLarge,
}

/// The geometry of one [`SliderSize`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderMetrics {
    /// Track thickness.
    pub track_height: f32,
    /// Handle length (across the track).
    pub handle_height: f32,
    /// Radius of the track's outer corners.
    pub track_corner: f32,
    /// Size of an inset icon, for the sizes that have one.
    pub icon_size: Option<f32>,
}

impl SliderSize {
    /// Every size, smallest first.
    pub const ALL: [SliderSize; 5] = [
        SliderSize::ExtraSmall,
        SliderSize::Small,
        SliderSize::Medium,
        SliderSize::Large,
        SliderSize::ExtraLarge,
    ];

    /// The size's geometry. XS is `SliderTokens` (16 dp track, 44 dp
    /// handle, `CornerFull`); S to XL are the M3 Expressive slider spec
    /// values from m3.material.io (track 24 / 40 / 56 / 96, handle
    /// 44 / 52 / 68 / 108, corners 8 / 12 / 16 / 28, icons 24 / 24 / 32).
    pub const fn metrics(self) -> SliderMetrics {
        match self {
            SliderSize::ExtraSmall => SliderMetrics {
                track_height: 16.0,
                handle_height: 44.0,
                track_corner: 8.0,
                icon_size: None,
            },
            SliderSize::Small => SliderMetrics {
                track_height: 24.0,
                handle_height: 44.0,
                track_corner: 8.0,
                icon_size: None,
            },
            SliderSize::Medium => SliderMetrics {
                track_height: 40.0,
                handle_height: 52.0,
                track_corner: 12.0,
                icon_size: Some(24.0),
            },
            SliderSize::Large => SliderMetrics {
                track_height: 56.0,
                handle_height: 68.0,
                track_corner: 16.0,
                icon_size: Some(24.0),
            },
            SliderSize::ExtraLarge => SliderMetrics {
                track_height: 96.0,
                handle_height: 108.0,
                track_corner: 28.0,
                icon_size: Some(32.0),
            },
        }
    }
}

/// Interaction modes for sliders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderInteraction {
    /// Allow both tapping and sliding (default).
    #[default]
    TapAndSlide,
    /// Only allow tapping to set value.
    TapOnly,
    /// Only allow sliding from current position.
    SlideOnly,
    /// Only allow sliding the thumb itself.
    SlideThumb,
}

/// Thumb shape variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThumbShape {
    /// The M3 Expressive handle: a 4 dp bar that narrows when pressed.
    #[default]
    Handle,
    /// The pre-Expressive 20 dp round thumb.
    Round,
}

/// Range values for [`MaterialRangeSlider`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeValues {
    /// Lower end of the selected range.
    pub start: f32,
    /// Upper end of the selected range.
    pub end: f32,
}

impl RangeValues {
    /// A range from `start` to `end`.
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }
}

// ---------------------------------------------------------------------------
// Values and steps

/// The value range and optional step, shared by both sliders.
#[derive(Clone, Copy, Debug)]
struct Scale {
    min: f32,
    max: f32,
    step: Option<f32>,
}

impl Scale {
    fn new(range: &RangeInclusive<f32>, step: Option<f32>) -> Self {
        let (a, b) = (*range.start(), *range.end());
        let (min, max) = if a <= b { (a, b) } else { (b, a) };
        let step = step.filter(|s| s.is_finite() && *s > 0.0 && *s <= max - min);
        Self { min, max, step }
    }

    fn span(&self) -> f32 {
        self.max - self.min
    }

    fn fraction(&self, value: f32) -> f32 {
        if self.span() > 0.0 {
            ((value - self.min) / self.span()).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Snaps to the nearest step counted from the range start, and clamps.
    fn snap(&self, value: f32) -> f32 {
        let value = if value.is_nan() { self.min } else { value };
        let snapped = match self.step {
            Some(step) => self.min + ((value - self.min) / step).round() * step,
            None => value,
        };
        snapped.clamp(self.min, self.max)
    }

    fn value_at(&self, fraction: f32) -> f32 {
        self.snap(self.min + fraction.clamp(0.0, 1.0) * self.span())
    }

    /// One keyboard step: the step, or 1% of the range.
    fn key_step(&self) -> f32 {
        self.step.unwrap_or(self.span() / 100.0)
    }

    /// Tick fractions for a stepped slider, or none.
    fn ticks(&self) -> Vec<f32> {
        match self.step {
            Some(step) if self.span() > 0.0 => {
                let count = (self.span() / step).floor() as usize;
                (0..=count.min(1000))
                    .map(|i| (i as f32 * step / self.span()).clamp(0.0, 1.0))
                    .collect()
            }
            Some(_) | None => Vec::new(),
        }
    }

    fn format(&self, value: f32) -> String {
        match self.step {
            Some(step) if step >= 1.0 => format!("{value:.0}"),
            Some(_) | None => format!("{value:.2}"),
        }
    }
}

/// Applies the arrow / page / home / end keys to `value` when `response`
/// has focus. Returns whether a key was used.
fn apply_keys(ui: &Ui, response: &Response, scale: &Scale, rtl: bool, value: &mut f32) -> bool {
    if !response.has_focus() {
        return false;
    }
    let step = scale.key_step();
    let (left, right) = if rtl { (step, -step) } else { (-step, step) };
    let moves = [
        (Key::ArrowLeft, left),
        (Key::ArrowRight, right),
        (Key::ArrowDown, -step),
        (Key::ArrowUp, step),
        (Key::PageDown, -10.0 * step),
        (Key::PageUp, 10.0 * step),
    ];
    let mut used = false;
    ui.input_mut(|i| {
        for (key, delta) in moves {
            let presses = i.count_and_consume_key(Modifiers::NONE, key);
            if presses > 0 {
                *value = scale.snap(*value + delta * presses as f32);
                used = true;
            }
        }
        if i.consume_key(Modifiers::NONE, Key::Home) {
            *value = scale.min;
            used = true;
        }
        if i.consume_key(Modifiers::NONE, Key::End) {
            *value = scale.max;
            used = true;
        }
    });
    used
}

// ---------------------------------------------------------------------------
// Geometry

/// The track laid out on screen: positions along it run from 0 (the value
/// minimum) to `length` (the maximum), whatever the orientation and
/// direction.
#[derive(Clone, Copy, Debug)]
struct Track {
    /// The rect the track (not the handle) occupies.
    rect: Rect,
    vertical: bool,
    rtl: bool,
    metrics: SliderMetrics,
}

impl Track {
    fn new(area: Rect, vertical: bool, rtl: bool, metrics: SliderMetrics, thumb: f32) -> Self {
        // The track is inset by half a handle at each end (Compose
        // `SliderLayout`: `trackOffsetX = thumbCoreWidth / 2`).
        let rect = if vertical {
            Rect::from_center_size(
                area.center(),
                Vec2::new(metrics.track_height, (area.height() - thumb).max(0.0)),
            )
        } else {
            Rect::from_center_size(
                area.center(),
                Vec2::new((area.width() - thumb).max(0.0), metrics.track_height),
            )
        };
        Self {
            rect,
            vertical,
            rtl: rtl && !vertical,
            metrics,
        }
    }

    fn length(&self) -> f32 {
        if self.vertical {
            self.rect.height()
        } else {
            self.rect.width()
        }
    }

    /// Screen point at `along`, on the track's centre line.
    fn pos(&self, along: f32) -> Pos2 {
        let c = self.rect.center();
        if self.vertical {
            Pos2::new(c.x, self.rect.max.y - along)
        } else if self.rtl {
            Pos2::new(self.rect.max.x - along, c.y)
        } else {
            Pos2::new(self.rect.min.x + along, c.y)
        }
    }

    /// Position along the track of a screen point.
    fn along_of(&self, p: Pos2) -> f32 {
        let a = if self.vertical {
            self.rect.max.y - p.y
        } else if self.rtl {
            self.rect.max.x - p.x
        } else {
            p.x - self.rect.min.x
        };
        a.clamp(0.0, self.length())
    }

    /// Position along the track of a value fraction. Stepped sliders keep
    /// interior steps clear of the rounded ends (`SliderLayout`).
    fn along(&self, fraction: f32, stepped: bool) -> f32 {
        let l = self.length();
        let c = self.metrics.track_corner.min(l / 2.0);
        let at_end = fraction <= 0.0 || fraction >= 1.0;
        if stepped && !at_end {
            (l - 2.0 * c) * fraction + c
        } else {
            l * fraction
        }
    }

    /// The rect of the stretch `a0..a1` of the track.
    fn segment(&self, a0: f32, a1: f32) -> Rect {
        Rect::from_two_pos(self.pos(a0), self.pos(a1)).expand2(if self.vertical {
            Vec2::new(self.metrics.track_height / 2.0, 0.0)
        } else {
            Vec2::new(0.0, self.metrics.track_height / 2.0)
        })
    }

    /// Corner radii for a segment: `start` at its low end, `end` at its high
    /// end.
    fn corners(&self, rect: Rect, start: f32, end: f32) -> CornerRadius {
        let limit = rect.width().min(rect.height()) / 2.0;
        let r = |v: f32| v.min(limit).clamp(0.0, 255.0).round() as u8;
        let (s, e) = (r(start), r(end));
        if self.vertical {
            CornerRadius {
                nw: e,
                ne: e,
                sw: s,
                se: s,
            }
        } else if self.rtl {
            CornerRadius {
                nw: e,
                ne: s,
                sw: e,
                se: s,
            }
        } else {
            CornerRadius {
                nw: s,
                ne: e,
                sw: s,
                se: e,
            }
        }
    }

    /// Paints `a0..a1` if it has any length.
    fn paint_segment(&self, ui: &Ui, a0: f32, a1: f32, start: f32, end: f32, color: Color32) {
        if a1 - a0 <= 0.0 {
            return;
        }
        let rect = self.segment(a0, a1);
        ui.painter()
            .rect_filled(rect, self.corners(rect, start, end), color);
    }

    /// The handle's rect at `along`, `width` thick.
    fn handle_rect(&self, along: f32, width: f32) -> Rect {
        let size = if self.vertical {
            Vec2::new(self.metrics.handle_height, width)
        } else {
            Vec2::new(width, self.metrics.handle_height)
        };
        Rect::from_center_size(self.pos(along), size)
    }
}

/// Colours of one slider frame.
#[derive(Clone, Copy, Debug)]
struct Palette {
    active: Color32,
    inactive: Color32,
    handle: Color32,
    stop_on_active: Color32,
    stop_on_inactive: Color32,
}

impl Palette {
    fn new(theme: &MaterialTheme, enabled: bool, handle: Option<Color32>) -> Self {
        if enabled {
            Self {
                active: theme.color(ColorRole::Primary),
                inactive: theme.color(ColorRole::SecondaryContainer),
                handle: handle.unwrap_or_else(|| theme.color(ColorRole::Primary)),
                stop_on_active: theme.color(ColorRole::OnPrimary),
                stop_on_inactive: theme.color(ColorRole::OnSecondaryContainer),
            }
        } else {
            let on_surface = theme.color(ColorRole::OnSurface);
            Self {
                active: on_surface.gamma_multiply(0.38),
                inactive: on_surface.gamma_multiply(0.12),
                handle: on_surface.gamma_multiply(0.38),
                stop_on_active: theme.color(ColorRole::InverseOnSurface),
                stop_on_inactive: on_surface.gamma_multiply(0.38),
            }
        }
    }
}

/// One handle on the track.
#[derive(Clone, Copy, Debug)]
struct Handle {
    along: f32,
    width: f32,
}

impl Handle {
    /// Distance from the handle's centre to where the track resumes.
    fn clearance(&self) -> f32 {
        self.width / 2.0 + HANDLE_GAP
    }
}

/// How the active track is laid out.
#[derive(Clone, Copy, Debug)]
enum Layout {
    /// Active from the start to the handle.
    Standard(Handle),
    /// Active from the centre to the handle.
    Centered(Handle),
    /// Active between two handles.
    Range(Handle, Handle),
}

/// Paints the tracks and stop indicators for `layout`.
fn paint_track(ui: &Ui, track: &Track, layout: Layout, ticks: &[f32], palette: &Palette) {
    let l = track.length();
    let c = track.metrics.track_corner;
    let i = TRACK_INSIDE_CORNER;
    let mut gaps: Vec<(f32, f32)> = Vec::new();
    let mut active: Vec<(f32, f32)> = Vec::new();
    let mut end_stops: Vec<f32> = Vec::new();
    match layout {
        Layout::Standard(h) => {
            let (g0, g1) = (h.along - h.clearance(), h.along + h.clearance());
            track.paint_segment(ui, 0.0, g0, c, i, palette.active);
            track.paint_segment(ui, g1, l, i, c, palette.inactive);
            if g1 < l - c {
                end_stops.push(l - c);
            }
            gaps.push((g0, g1));
            active.push((0.0, h.along));
        }
        Layout::Centered(h) => {
            let mid = l / 2.0;
            let (g0, g1) = (h.along - h.clearance(), h.along + h.clearance());
            if h.along >= mid {
                track.paint_segment(ui, 0.0, mid, c, i, palette.inactive);
                track.paint_segment(ui, mid, g0, i, i, palette.active);
                track.paint_segment(ui, g1, l, i, c, palette.inactive);
                active.push((mid, h.along));
            } else {
                track.paint_segment(ui, 0.0, g0, c, i, palette.inactive);
                track.paint_segment(ui, g1, mid, i, i, palette.active);
                track.paint_segment(ui, mid, l, i, c, palette.inactive);
                active.push((h.along, mid));
            }
            if g0 > c {
                end_stops.push(c);
            }
            if g1 < l - c {
                end_stops.push(l - c);
            }
            gaps.push((g0, g1));
        }
        Layout::Range(s, e) => {
            let (s0, s1) = (s.along - s.clearance(), s.along + s.clearance());
            let (e0, e1) = (e.along - e.clearance(), e.along + e.clearance());
            track.paint_segment(ui, 0.0, s0, c, i, palette.inactive);
            track.paint_segment(ui, s1, e0, i, i, palette.active);
            track.paint_segment(ui, e1, l, i, c, palette.inactive);
            if s0 > c {
                end_stops.push(c);
            }
            if e1 < l - c {
                end_stops.push(l - c);
            }
            gaps.push((s0, s1));
            gaps.push((e0, e1));
            active.push((s.along, e.along));
        }
    }
    let in_gap = |a: f32| gaps.iter().any(|(g0, g1)| a > *g0 && a < *g1);
    let is_active = |a: f32| active.iter().any(|(a0, a1)| a >= *a0 && a <= *a1);
    let paint_stop = |a: f32| {
        let color = if is_active(a) {
            palette.stop_on_active
        } else {
            palette.stop_on_inactive
        };
        ui.painter()
            .circle_filled(track.pos(a), STOP_INDICATOR_SIZE / 2.0, color);
    };
    if ticks.is_empty() {
        for a in end_stops {
            paint_stop(a);
        }
    } else if l / ticks.len().max(1) as f32 >= 2.0 * STOP_INDICATOR_SIZE {
        for fraction in ticks {
            let a = track.along(*fraction, true);
            // Ticks at the very ends sit inside the rounded corners.
            let a = a.clamp(c.min(l / 2.0), (l - c).max(l / 2.0));
            if !in_gap(a) {
                paint_stop(a);
            }
        }
    }
}

/// Paints a handle (Expressive bar or legacy round thumb).
fn paint_handle(ui: &Ui, track: &Track, handle: Handle, shape: ThumbShape, color: Color32) -> Rect {
    match shape {
        ThumbShape::Handle => {
            let rect = track.handle_rect(handle.along, handle.width);
            // `HandleShape = CornerFull`.
            ui.painter().rect_filled(rect, handle.width / 2.0, color);
            rect
        }
        ThumbShape::Round => {
            let center = track.pos(handle.along);
            ui.painter()
                .circle_filled(center, ROUND_THUMB_DIAMETER / 2.0, color);
            Rect::from_center_size(center, Vec2::splat(ROUND_THUMB_DIAMETER))
        }
    }
}

/// Paints the value indicator bubble above (or beside, when vertical) the
/// handle, on the tooltip layer so it is not clipped.
fn paint_value_indicator(ui: &Ui, id: Id, track: &Track, handle_rect: Rect, text: &str) {
    let theme = MaterialTheme::of(ui.ctx());
    let painter = ui
        .ctx()
        .layer_painter(LayerId::new(Order::Tooltip, id.with("value_indicator")));
    let galley = painter.layout_job(TypeRole::LabelLarge.style().layout_job(
        ui.ctx(),
        text,
        theme.color(ColorRole::InverseOnSurface),
    ));
    let size = Vec2::new(
        VALUE_INDICATOR_SIZE.x.max(galley.size().x + 16.0),
        VALUE_INDICATOR_SIZE.y,
    );
    let rect = if track.vertical {
        Rect::from_min_size(
            Pos2::new(
                handle_rect.max.x + VALUE_INDICATOR_BOTTOM_SPACE,
                handle_rect.center().y - size.y / 2.0,
            ),
            size,
        )
    } else {
        Rect::from_min_size(
            Pos2::new(
                handle_rect.center().x - size.x / 2.0,
                handle_rect.min.y - VALUE_INDICATOR_BOTTOM_SPACE - size.y,
            ),
            size,
        )
    };
    painter.rect_filled(rect, size.y / 2.0, theme.color(ColorRole::InverseSurface));
    let pos = rect.center() - galley.size() / 2.0;
    painter.galley(pos, galley, theme.color(ColorRole::InverseOnSurface));
}

/// Paints the label and value text to the right of a horizontal slider.
fn paint_side_text(
    ui: &Ui,
    rect: Rect,
    track: &Track,
    text: Option<&str>,
    value: Option<&str>,
    enabled: bool,
) {
    let theme = MaterialTheme::of(ui.ctx());
    let disabled = theme.color(ColorRole::OnSurface).gamma_multiply(0.38);
    let x = track.rect.max.x + HANDLE_WIDTH / 2.0 + 16.0;
    if let Some(text) = text {
        ui.painter().text(
            Pos2::new(x, rect.center().y - if value.is_some() { 8.0 } else { 0.0 }),
            Align2::LEFT_CENTER,
            text,
            TypeRole::BodyMedium.style().font_id(ui.ctx()),
            if enabled {
                theme.color(ColorRole::OnSurface)
            } else {
                disabled
            },
        );
    }
    if let Some(value) = value {
        ui.painter().text(
            Pos2::new(x, rect.center().y + if text.is_some() { 8.0 } else { 0.0 }),
            Align2::LEFT_CENTER,
            value,
            TypeRole::LabelMedium.style().font_id(ui.ctx()),
            if enabled {
                theme.color(ColorRole::OnSurfaceVariant)
            } else {
                disabled
            },
        );
    }
}

/// Lays out the widget: the whole allocated size and the track area's.
fn allocation(
    ui: &Ui,
    metrics: SliderMetrics,
    vertical: bool,
    length: Option<f32>,
    side_text: bool,
) -> (Vec2, Vec2) {
    let cross = metrics.handle_height.max(metrics.track_height);
    if vertical {
        let length = length.unwrap_or(200.0).max(cross);
        let size = Vec2::new(cross, length);
        (size, size)
    } else {
        let extra = if side_text { LABEL_AREA } else { 0.0 };
        let length = length
            .unwrap_or_else(|| ui.available_width() - extra)
            .max(cross);
        (Vec2::new(length + extra, cross), Vec2::new(length, cross))
    }
}

/// The handle width for the current interaction state.
fn handle_width(pressed: bool, focused: bool, shape: ThumbShape) -> f32 {
    match shape {
        ThumbShape::Handle if pressed || focused => HANDLE_PRESSED_WIDTH,
        ThumbShape::Handle => HANDLE_WIDTH,
        ThumbShape::Round => ROUND_THUMB_DIAMETER,
    }
}

fn is_rtl(ui: &Ui) -> bool {
    match MaterialTheme::of(ui.ctx()).layout_direction {
        LayoutDirection::Rtl => true,
        LayoutDirection::Ltr => false,
    }
}

// ---------------------------------------------------------------------------
// MaterialSlider

/// A Material 3 Expressive slider.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialSlider, SliderSize};
/// let mut volume = 40.0;
/// ui.add(MaterialSlider::new(&mut volume, 0.0..=100.0).text("Volume"));
/// ui.add(
///     MaterialSlider::new(&mut volume, 0.0..=100.0)
///         .size(SliderSize::Medium)
///         .step(10.0)
///         .inset_icon("\u{e050}"),
/// );
/// # });
/// ```
#[must_use = "add it with `ui.add(...)`"]
pub struct MaterialSlider<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    text: Option<String>,
    enabled: bool,
    width: Option<f32>,
    step: Option<f32>,
    show_value: bool,
    secondary_track_value: Option<f32>,
    show_value_indicator: bool,
    interaction_mode: SliderInteraction,
    thumb_shape: ThumbShape,
    overlay_color: Option<Color32>,
    thumb_color: Option<Color32>,
    secondary_active_color: Option<Color32>,
    size: SliderSize,
    centered: bool,
    vertical: bool,
    inset_icon: Option<String>,
}

impl<'a> MaterialSlider<'a> {
    /// A slider editing `value` within `range`.
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            text: None,
            enabled: true,
            width: None,
            step: None,
            show_value: true,
            secondary_track_value: None,
            show_value_indicator: false,
            interaction_mode: SliderInteraction::default(),
            thumb_shape: ThumbShape::default(),
            overlay_color: None,
            thumb_color: None,
            secondary_active_color: None,
            size: SliderSize::default(),
            centered: false,
            vertical: false,
            inset_icon: None,
        }
    }

    /// The label: painted beside a horizontal slider, and its accessible name.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Enables or disables the slider.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// The track length (height when vertical). Defaults to the available
    /// width (200 when vertical).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Snaps to multiples of `step` counted from the range start, and shows
    /// a stop indicator at each step.
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Shows the value beside a horizontal slider (default on).
    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    /// A secondary (buffered) value, painted on the inactive track.
    pub fn secondary_track_value(mut self, value: f32) -> Self {
        self.secondary_track_value = Some(value);
        self
    }

    /// Shows the value indicator bubble while dragging or focused.
    pub fn show_value_indicator(mut self, show: bool) -> Self {
        self.show_value_indicator = show;
        self
    }

    /// How pointer input moves the value.
    pub fn interaction_mode(mut self, mode: SliderInteraction) -> Self {
        self.interaction_mode = mode;
        self
    }

    /// The Expressive handle (default) or the legacy round thumb.
    pub fn thumb_shape(mut self, shape: ThumbShape) -> Self {
        self.thumb_shape = shape;
        self
    }

    /// Paints a state layer of `color` around the handle while hovered.
    pub fn overlay_color(mut self, color: Color32) -> Self {
        self.overlay_color = Some(color);
        self
    }

    /// Overrides the handle colour.
    pub fn thumb_color(mut self, color: Color32) -> Self {
        self.thumb_color = Some(color);
        self
    }

    /// The colour of the secondary track.
    pub fn secondary_active_color(mut self, color: Color32) -> Self {
        self.secondary_active_color = Some(color);
        self
    }

    /// The track size (default [`SliderSize::ExtraSmall`]).
    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    /// A centred slider: the active track runs from the middle of the range
    /// to the value.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// A vertical slider, minimum at the bottom.
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// An icon inside the track, at its start (Medium and larger sizes; a
    /// Material Symbols glyph).
    pub fn inset_icon(mut self, icon: impl Into<String>) -> Self {
        self.inset_icon = Some(icon.into());
        self
    }
}

impl Widget for MaterialSlider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let metrics = self.size.metrics();
        let side_text = !self.vertical && (self.text.is_some() || self.show_value);
        let (outer, track_area) = allocation(ui, metrics, self.vertical, self.width, side_text);
        let (rect, mut response) = ui.allocate_exact_size(
            outer,
            crate::a11y::sense(self.enabled, Sense::click_and_drag()),
        );
        let scale = Scale::new(&self.range, self.step);
        let rtl = is_rtl(ui);
        let area = Rect::from_min_size(rect.min, track_area);
        let track = Track::new(area, self.vertical, rtl, metrics, HANDLE_WIDTH);
        let stepped = scale.step.is_some();

        let old_value = *self.value;
        if self.enabled {
            let handle_at = track.pos(track.along(scale.fraction(scale.snap(old_value)), stepped));
            let near_thumb = |p: Pos2| (p - handle_at).length() < metrics.handle_height / 2.0;
            let can_interact = match self.interaction_mode {
                SliderInteraction::TapAndSlide => response.clicked() || response.dragged(),
                SliderInteraction::TapOnly => response.clicked(),
                SliderInteraction::SlideOnly => response.dragged(),
                SliderInteraction::SlideThumb => {
                    let origin = ui.input(|i| i.pointer.press_origin());
                    response.dragged() && origin.is_some_and(near_thumb)
                }
            };
            if let (true, Some(p)) = (can_interact, response.interact_pointer_pos()) {
                *self.value = scale.value_at(track.along_of(p) / track.length().max(1.0));
            }
            let mut keyed = *self.value;
            if apply_keys(ui, &response, &scale, rtl, &mut keyed) {
                *self.value = keyed;
            }
            if (*self.value - old_value).abs() > f32::EPSILON {
                response.mark_changed();
            }
        }

        crate::a11y::slider(
            &response,
            self.enabled,
            self.text.as_deref().unwrap_or_default(),
            f64::from(*self.value),
        );

        if !ui.is_rect_visible(rect) {
            return response;
        }
        let theme = MaterialTheme::of(ui.ctx());
        let palette = Palette::new(&theme, self.enabled, self.thumb_color);
        let value = scale.snap(*self.value);
        let pressed = response.is_pointer_button_down_on() || response.dragged();
        let handle = Handle {
            along: track.along(scale.fraction(value), stepped),
            width: handle_width(pressed, response.has_focus(), self.thumb_shape),
        };
        let layout = if self.centered {
            Layout::Centered(handle)
        } else {
            Layout::Standard(handle)
        };
        paint_track(ui, &track, layout, &scale.ticks(), &palette);

        if let Some(secondary) = self.secondary_track_value {
            let to = track.along(scale.fraction(secondary), stepped);
            let from = handle.along + handle.clearance();
            if to > from {
                let color = self
                    .secondary_active_color
                    .unwrap_or_else(|| palette.active.gamma_multiply(0.5));
                track.paint_segment(
                    ui,
                    from,
                    to,
                    TRACK_INSIDE_CORNER,
                    TRACK_INSIDE_CORNER,
                    color,
                );
            }
        }

        if let (Some(icon), Some(icon_size)) = (&self.inset_icon, metrics.icon_size) {
            let icon_colors = IconColors::new(&theme, &palette, self.enabled);
            paint_inset_icon(ui, &track, handle, icon, icon_size, icon_colors);
        }

        if let (Some(color), true) = (self.overlay_color, response.hovered() && self.enabled) {
            ui.painter()
                .circle_filled(track.pos(handle.along), metrics.handle_height / 2.0, color);
        }
        let handle_rect = paint_handle(ui, &track, handle, self.thumb_shape, palette.handle);

        if self.show_value_indicator && self.enabled && (response.dragged() || response.has_focus())
        {
            paint_value_indicator(ui, response.id, &track, handle_rect, &scale.format(value));
        }
        if side_text {
            let value_text = self.show_value.then(|| scale.format(value));
            paint_side_text(
                ui,
                rect,
                &track,
                self.text.as_deref(),
                value_text.as_deref(),
                self.enabled,
            );
        }
        response
    }
}

/// Inset icon colours: on the active track and on the inactive track.
#[derive(Clone, Copy, Debug)]
struct IconColors {
    on_active: Color32,
    on_inactive: Color32,
}

impl IconColors {
    fn new(theme: &MaterialTheme, palette: &Palette, enabled: bool) -> Self {
        if enabled {
            Self {
                on_active: theme.color(ColorRole::OnPrimary),
                on_inactive: theme.color(ColorRole::OnSecondaryContainer),
            }
        } else {
            Self {
                on_active: palette.stop_on_active,
                on_inactive: palette.stop_on_inactive,
            }
        }
    }
}

/// Paints the inset icon at the start of the active track, or just past the
/// handle on the inactive track when the active part is too short.
fn paint_inset_icon(
    ui: &Ui,
    track: &Track,
    handle: Handle,
    icon: &str,
    icon_size: f32,
    colors: IconColors,
) {
    let inset = (track.metrics.track_height - icon_size) / 2.0;
    let room = handle.along - handle.clearance();
    let (along, color) = if room >= icon_size + 2.0 * inset {
        (inset + icon_size / 2.0, colors.on_active)
    } else {
        (
            handle.along + handle.clearance() + inset + icon_size / 2.0,
            colors.on_inactive,
        )
    };
    if along + icon_size / 2.0 > track.length() {
        return;
    }
    ui.painter().text(
        track.pos(along),
        Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(icon_size),
        color,
    );
}

// ---------------------------------------------------------------------------
// MaterialRangeSlider

/// Which thumb of a range slider.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Thumb {
    Start,
    End,
}

/// A Material 3 Expressive range slider: two handles, each focusable, with
/// its own accessible value.
#[must_use = "add it with `ui.add(...)`"]
pub struct MaterialRangeSlider<'a> {
    values: &'a mut RangeValues,
    range: RangeInclusive<f32>,
    text: Option<String>,
    enabled: bool,
    width: Option<f32>,
    step: Option<f32>,
    show_values: bool,
    show_value_indicator: bool,
    thumb_shape: ThumbShape,
    min_separation: f32,
    size: SliderSize,
    vertical: bool,
}

impl<'a> MaterialRangeSlider<'a> {
    /// A range slider editing `values` within `range`.
    pub fn new(values: &'a mut RangeValues, range: RangeInclusive<f32>) -> Self {
        Self {
            values,
            range,
            text: None,
            enabled: true,
            width: None,
            step: None,
            show_values: true,
            show_value_indicator: false,
            thumb_shape: ThumbShape::default(),
            min_separation: 0.0,
            size: SliderSize::default(),
            vertical: false,
        }
    }

    /// The label: painted beside a horizontal slider, and the accessible
    /// name ("<label> start" / "<label> end" for the thumbs).
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Enables or disables the slider.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// The track length (height when vertical).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Snaps to multiples of `step` counted from the range start.
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Shows "start - end" beside a horizontal slider (default on).
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Shows the value indicator of the thumb being dragged or focused.
    pub fn show_value_indicator(mut self, show: bool) -> Self {
        self.show_value_indicator = show;
        self
    }

    /// The Expressive handle (default) or the legacy round thumb.
    pub fn thumb_shape(mut self, shape: ThumbShape) -> Self {
        self.thumb_shape = shape;
        self
    }

    /// The smallest allowed distance between start and end.
    pub fn min_separation(mut self, separation: f32) -> Self {
        self.min_separation = separation;
        self
    }

    /// The track size (default [`SliderSize::ExtraSmall`]).
    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    /// A vertical slider, minimum at the bottom.
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    fn thumb_label(&self, thumb: Thumb) -> String {
        let base = self.text.as_deref().unwrap_or("Range");
        match thumb {
            Thumb::Start => format!("{base} start"),
            Thumb::End => format!("{base} end"),
        }
    }

    fn value(&self, thumb: Thumb) -> f32 {
        match thumb {
            Thumb::Start => self.values.start,
            Thumb::End => self.values.end,
        }
    }

    /// Moves `thumb` to `value`, keeping the thumbs ordered and apart.
    fn set(&mut self, thumb: Thumb, value: f32, scale: &Scale) {
        match thumb {
            Thumb::Start => {
                let hi = (self.values.end - self.min_separation).min(scale.max);
                self.values.start = value.clamp(scale.min, hi.max(scale.min));
            }
            Thumb::End => {
                let lo = (self.values.start + self.min_separation).max(scale.min);
                self.values.end = value.clamp(lo.min(scale.max), scale.max);
            }
        }
    }
}

impl Widget for MaterialRangeSlider<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let metrics = self.size.metrics();
        let side_text = !self.vertical && (self.text.is_some() || self.show_values);
        let (outer, track_area) = allocation(ui, metrics, self.vertical, self.width, side_text);
        let sense = crate::a11y::sense(self.enabled, Sense::click_and_drag());
        let (rect, mut response) = ui.allocate_exact_size(outer, sense);
        crate::a11y::describe(
            &response,
            egui::Role::Slider,
            self.enabled,
            self.text.as_deref().unwrap_or_default(),
            None,
        );
        let scale = Scale::new(&self.range, self.step);
        let rtl = is_rtl(ui);
        let stepped = scale.step.is_some();
        let area = Rect::from_min_size(rect.min, track_area);
        let track = Track::new(area, self.vertical, rtl, metrics, HANDLE_WIDTH);
        let along_of = |v: f32| track.along(scale.fraction(scale.snap(v)), stepped);
        let to_value = |p: Pos2| scale.value_at(track.along_of(p) / track.length().max(1.0));
        let before = *self.values;

        // Each thumb is its own focusable widget on top of the track.
        let hit = Vec2::splat(metrics.handle_height);
        let thumb_id = |thumb: Thumb| {
            response.id.with(match thumb {
                Thumb::Start => "start_thumb",
                Thumb::End => "end_thumb",
            })
        };
        let start_response = ui.interact(
            Rect::from_center_size(track.pos(along_of(self.values.start)), hit),
            thumb_id(Thumb::Start),
            sense,
        );
        let end_response = ui.interact(
            Rect::from_center_size(track.pos(along_of(self.values.end)), hit),
            thumb_id(Thumb::End),
            sense,
        );

        // The thumb a track drag moves is chosen once, when the drag starts,
        // by which thumb is closer to where the press began, so the thumbs
        // cannot swap mid-drag.
        let active_id = response.id.with("active_thumb");
        let closer_to_start = |p: Pos2, values: RangeValues| {
            let a = track.along_of(p);
            (a - along_of(values.start)).abs() < (a - along_of(values.end)).abs()
        };
        if self.enabled {
            if response.drag_started() {
                let origin = ui
                    .input(|i| i.pointer.press_origin())
                    .or_else(|| response.interact_pointer_pos());
                if let Some(origin) = origin {
                    let moves_start = closer_to_start(origin, *self.values);
                    ui.data_mut(|d| d.insert_temp(active_id, moves_start));
                }
            }
            if response.dragged() || response.clicked() {
                if let Some(p) = response.interact_pointer_pos() {
                    let moves_start = ui
                        .data(|d| d.get_temp::<bool>(active_id))
                        .unwrap_or_else(|| closer_to_start(p, *self.values));
                    let thumb = if moves_start {
                        Thumb::Start
                    } else {
                        Thumb::End
                    };
                    self.set(thumb, to_value(p), &scale);
                }
            }
            if response.drag_stopped() {
                ui.data_mut(|d| d.remove::<bool>(active_id));
            }
            for (thumb, r) in [(Thumb::Start, &start_response), (Thumb::End, &end_response)] {
                if r.dragged() {
                    if let Some(p) = r.interact_pointer_pos() {
                        self.set(thumb, to_value(p), &scale);
                    }
                }
                let mut value = self.value(thumb);
                if apply_keys(ui, r, &scale, rtl, &mut value) {
                    self.set(thumb, value, &scale);
                }
            }
            if *self.values != before {
                response.mark_changed();
            }
        }

        for (thumb, r) in [(Thumb::Start, &start_response), (Thumb::End, &end_response)] {
            crate::a11y::slider(
                r,
                self.enabled,
                &self.thumb_label(thumb),
                f64::from(self.value(thumb)),
            );
        }

        if !ui.is_rect_visible(rect) {
            return response;
        }
        let theme = MaterialTheme::of(ui.ctx());
        let palette = Palette::new(&theme, self.enabled, None);
        let handle_for = |r: &Response, value: f32| Handle {
            along: along_of(value),
            width: handle_width(
                r.is_pointer_button_down_on() || r.dragged(),
                r.has_focus(),
                self.thumb_shape,
            ),
        };
        let start = handle_for(&start_response, self.values.start);
        let end = handle_for(&end_response, self.values.end);
        paint_track(
            ui,
            &track,
            Layout::Range(start, end),
            &scale.ticks(),
            &palette,
        );
        let start_rect = paint_handle(ui, &track, start, self.thumb_shape, palette.handle);
        let end_rect = paint_handle(ui, &track, end, self.thumb_shape, palette.handle);

        if self.show_value_indicator && self.enabled {
            let by_track = if response.dragged() {
                ui.data(|d| d.get_temp::<bool>(active_id))
            } else {
                None
            };
            for (thumb, r, handle_rect) in [
                (Thumb::Start, &start_response, start_rect),
                (Thumb::End, &end_response, end_rect),
            ] {
                let dragged_by_track = by_track == Some(thumb == Thumb::Start);
                if r.dragged() || r.has_focus() || dragged_by_track {
                    let text = scale.format(scale.snap(self.value(thumb)));
                    paint_value_indicator(ui, r.id, &track, handle_rect, &text);
                }
            }
        }
        if side_text {
            let values = self.show_values.then(|| {
                format!(
                    "{} - {}",
                    scale.format(self.values.start),
                    scale.format(self.values.end)
                )
            });
            paint_side_text(
                ui,
                rect,
                &track,
                self.text.as_deref(),
                values.as_deref(),
                self.enabled,
            );
        }
        response
    }
}

/// Shorthand for [`MaterialSlider::new`].
pub fn slider(value: &mut f32, range: RangeInclusive<f32>) -> MaterialSlider<'_> {
    MaterialSlider::new(value, range)
}

/// Shorthand for [`MaterialRangeSlider::new`].
pub fn range_slider(
    values: &mut RangeValues,
    range: RangeInclusive<f32>,
) -> MaterialRangeSlider<'_> {
    MaterialRangeSlider::new(values, range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_count_from_the_range_start() {
        let scale = Scale::new(&(3.0..=23.0), Some(5.0));
        assert_eq!(scale.snap(7.0), 8.0, "7 snaps to 3 + 5");
        assert_eq!(scale.snap(10.4), 8.0, "10.4 snaps to 8");
        assert_eq!(scale.snap(11.0), 13.0, "11 snaps to 13");
        assert_eq!(scale.snap(100.0), 23.0, "clamped to the end");
    }

    #[test]
    fn degenerate_ranges_do_not_divide_by_zero() {
        let scale = Scale::new(&(5.0..=5.0), Some(1.0));
        assert_eq!(scale.fraction(5.0), 0.0, "empty range");
        assert!(scale.ticks().is_empty(), "no ticks for an empty range");
        let reversed = Scale::new(&(10.0..=0.0), None);
        assert_eq!(reversed.snap(-3.0), 0.0, "reversed range is normalised");
    }

    #[test]
    fn sizes_are_ordered() {
        let heights: Vec<f32> = SliderSize::ALL
            .iter()
            .map(|s| s.metrics().track_height)
            .collect();
        assert_eq!(heights, vec![16.0, 24.0, 40.0, 56.0, 96.0], "track heights");
    }
}
