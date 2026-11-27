//! State layers show on transparent containers: a hovered text button
//! paints a translucent layer where it used to paint nothing (#18).

use egui::{epaint::Shape as PaintShape, Color32, Pos2};
use egui_material3::{MaterialButton, MaterialTheme};

/// Collects every filled rect the button paints in one frame.
fn filled_rects(ctx: &egui::Context, pointer: Option<Pos2>) -> Vec<Color32> {
    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            Pos2::ZERO,
            egui::vec2(300.0, 100.0),
        )),
        ..Default::default()
    };
    if let Some(pos) = pointer {
        input.events.push(egui::Event::PointerMoved(pos));
    }
    let mut output = ctx.run_ui(input, |ui| {
        ui.add(MaterialButton::text("Text button"));
    });
    output.textures_delta.clear();
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            PaintShape::Rect(rect) if rect.fill != Color32::TRANSPARENT => Some(rect.fill),
            _ => None,
        })
        .collect()
}

#[test]
fn hover_paints_a_state_layer_on_a_text_button() {
    let ctx = egui::Context::default();
    MaterialTheme::set(&ctx, MaterialTheme::baseline(false));
    let _ = filled_rects(&ctx, None);
    let idle = filled_rects(&ctx, None);
    let _ = filled_rects(&ctx, Some(Pos2::new(20.0, 20.0)));
    let hovered = filled_rects(&ctx, Some(Pos2::new(20.0, 20.0)));
    assert!(idle.is_empty(), "idle text button has no fill: {idle:?}");
    assert!(
        hovered.iter().any(|c| c.a() > 0 && c.a() < 40),
        "hovered text button paints a translucent state layer: {hovered:?}"
    );
}
