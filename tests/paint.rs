//! Painting primitives: state-layer compositing, and elevation levels
//! rendering soft shadows (snapshot).

mod support;

use egui::{Color32, Pos2};
use egui_material3::paint::{paint_elevation, paint_state_layer, state_layer_over};
use egui_material3::tokens::{ColorRole, Elevation, InteractionState};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn state_layer_over_composites_unmultiplied() {
    let black = Color32::BLACK;
    let white = Color32::WHITE;
    let over_opaque = state_layer_over(white, black, 0.10);
    assert_eq!(over_opaque.a(), 255, "opaque container stays opaque");
    assert!(
        (228..=230).contains(&over_opaque.r()),
        "10% black over white: {over_opaque:?}"
    );
    let over_clear = state_layer_over(Color32::TRANSPARENT, white, 0.08);
    assert!(
        (19..=21).contains(&over_clear.a()),
        "8% layer on transparent: {over_clear:?}"
    );
    assert_eq!(
        over_clear.r(),
        over_clear.a(),
        "premultiplied white keeps r == a"
    );
}

#[test]
fn elevation_levels_snapshot() -> TestResult {
    for (name, theme) in support::themes() {
        let shadow = theme.color(ColorRole::Shadow);
        let surface = theme.color(ColorRole::SurfaceContainerLow);
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(560.0, 120.0), move |ui| {
            let painter = ui.painter();
            painter.rect_filled(ui.max_rect(), 0.0, background);
            for (i, level) in Elevation::ALL.into_iter().enumerate() {
                let rect = egui::Rect::from_min_size(
                    Pos2::new(20.0 + 90.0 * i as f32, 30.0),
                    egui::vec2(64.0, 56.0),
                );
                paint_elevation(painter, rect, 12, level, shadow);
                painter.rect_filled(rect, 12, surface);
                let hovered = InteractionState {
                    hovered: i % 2 == 1,
                    ..Default::default()
                };
                paint_state_layer(painter, rect, 12, Color32::BLACK, hovered);
            }
        });
        // Software and hardware rasterisers differ in anti-aliasing at the
        // shadow edges; allow that, not a missing or moved shadow.
        let options = egui_kittest::SnapshotOptions::new()
            .threshold(2.0)
            .max_failed_pixels(400);
        match harness.try_snapshot_options(format!("elevation_levels_{name}"), &options) {
            Ok(()) => {}
            // No GPU adapter (headless CI without a software renderer): the
            // geometry is still covered by the token tests.
            Err(egui_kittest::SnapshotError::RenderError { err }) => {
                eprintln!("skipping snapshot, no renderer: {err}");
            }
            Err(err) => return Err(err.to_string().into()),
        }
    }
    Ok(())
}
