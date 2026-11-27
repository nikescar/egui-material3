//! Painting primitives shared by the components: state layers, elevation
//! shadows and the focus ring. Custom widgets that use them match the
//! built-in ones.

use egui::{Color32, CornerRadius, Painter, Rect, Stroke, StrokeKind};

use crate::tokens::{ColorRole, Elevation, InteractionState};

/// Paints the state layer for `state` over `rect`, in `content_color` at the
/// state's opacity (see [`InteractionState::opacity`]). Call it after the
/// container fill and before the content. Nothing is drawn when idle.
///
/// Works on transparent containers too (text and outlined buttons): the
/// layer is a translucent fill of its own, not a blend into the container.
pub fn paint_state_layer(
    painter: &Painter,
    rect: Rect,
    corner_radius: impl Into<CornerRadius>,
    content_color: Color32,
    state: InteractionState,
) {
    let opacity = state.opacity();
    if opacity > 0.0 {
        painter.rect_filled(rect, corner_radius, content_color.gamma_multiply(opacity));
    }
}

/// `content_color` at `opacity`, composited over `container`: a state layer
/// on a container, as one colour. Correct for translucent and transparent
/// containers as well as opaque ones.
pub fn state_layer_over(container: Color32, content_color: Color32, opacity: f32) -> Color32 {
    let layer = content_color.gamma_multiply(opacity.clamp(0.0, 1.0));
    // Premultiplied "over": layer + container * (1 - layer alpha).
    let keep = 1.0 - f32::from(layer.a()) / 255.0;
    let mix = |l: u8, c: u8| {
        (f32::from(l) + f32::from(c) * keep)
            .round()
            .clamp(0.0, 255.0) as u8
    };
    Color32::from_rgba_premultiplied(
        mix(layer.r(), container.r()),
        mix(layer.g(), container.g()),
        mix(layer.b(), container.b()),
        mix(layer.a(), container.a()),
    )
}

/// Paints the soft key and ambient shadows of `level` under a surface at
/// `rect`. Call it before the surface fill.
pub fn paint_elevation(
    painter: &Painter,
    rect: Rect,
    corner_radius: impl Into<CornerRadius>,
    level: Elevation,
    shadow_color: Color32,
) {
    let corner_radius = corner_radius.into();
    for shadow in level.shadows(shadow_color) {
        if shadow.color != Color32::TRANSPARENT {
            painter.add(shadow.as_shape(rect, corner_radius));
        }
    }
}

/// Focus ring stroke width (3 dp), from the M3 focus-indicator spec. The
/// colour is each component's `FocusIndicatorColor` token, which Compose
/// sets to `ColorSchemeKeyTokens.Secondary` throughout.
pub const FOCUS_RING_WIDTH: f32 = 3.0;
/// Gap between a widget's edge and its focus ring (2 dp).
pub const FOCUS_RING_OFFSET: f32 = 2.0;

/// Paints the keyboard focus ring outside `rect`, following its corners.
pub fn paint_focus_ring(
    painter: &Painter,
    rect: Rect,
    corner_radius: impl Into<CornerRadius>,
    color: Color32,
) {
    let corner_radius = corner_radius.into();
    let grow = FOCUS_RING_OFFSET + FOCUS_RING_WIDTH / 2.0;
    let grow_u8 = grow.ceil().clamp(0.0, 255.0) as u8;
    let grown = |r: u8| if r == 0 { 0 } else { r.saturating_add(grow_u8) };
    let radius = CornerRadius {
        nw: grown(corner_radius.nw),
        ne: grown(corner_radius.ne),
        sw: grown(corner_radius.sw),
        se: grown(corner_radius.se),
    };
    painter.rect_stroke(
        rect.expand(grow),
        radius,
        Stroke::new(FOCUS_RING_WIDTH, color),
        StrokeKind::Middle,
    );
}

/// Paints the focus ring around `response` when it has keyboard focus, in
/// the theme's `secondary` colour.
pub fn paint_focus_ring_if_focused(
    painter: &Painter,
    response: &egui::Response,
    corner_radius: impl Into<CornerRadius>,
) {
    if response.has_focus() {
        let color = crate::MaterialTheme::of(&response.ctx).color(ColorRole::Secondary);
        paint_focus_ring(painter, response.rect, corner_radius, color);
    }
}
