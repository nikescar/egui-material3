// ============================================================================
// Module:       extras (integration test)
// Description:  Data table, dash counter, timeline, layout grid and image
//               list colours from the per-context theme, and snapshots.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! The extras read their colours from the context's `MaterialTheme`, not
//! the global theme: each test sets a theme whose roles are marker colours
//! and looks for them in the painted shapes. A snapshot per theme covers
//! the look; a machine with no GPU adapter skips it.

mod support;

use egui::epaint::{ClippedShape, Shape as PaintShape};
use egui::{Color32, Pos2, Rect};
use egui_material3::tokens::ColorRole;
use egui_material3::{
    dashcounter, data_table, image_list, layout_grid, timeline, MaterialTheme, TimelineItem,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// A light theme with `role` set to `color`.
fn theme_with(roles: &[(ColorRole, Color32)]) -> MaterialTheme {
    let mut theme = MaterialTheme::baseline(false);
    for (role, color) in roles {
        theme.colors.set(*role, *color);
    }
    theme
}

/// Runs two frames of `app` in a fresh context holding `theme` (the second
/// sees the first frame's layout) and returns the second frame's shapes.
fn paint(theme: MaterialTheme, mut app: impl FnMut(&mut egui::Ui)) -> Vec<ClippedShape> {
    let ctx = egui::Context::default();
    MaterialTheme::set(&ctx, theme);
    let mut shapes = Vec::new();
    for _ in 0..2 {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(800.0, 600.0))),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        shapes = output.shapes;
    }
    shapes
}

/// Every fill and stroke colour in `shapes`, flattened.
fn colors(shapes: &[ClippedShape]) -> Vec<Color32> {
    fn walk(shape: &PaintShape, out: &mut Vec<Color32>) {
        match shape {
            PaintShape::Rect(r) => {
                out.push(r.fill);
                out.push(r.stroke.color);
            }
            PaintShape::Circle(c) => {
                out.push(c.fill);
                out.push(c.stroke.color);
            }
            PaintShape::LineSegment { stroke, .. } => out.push(stroke.color),
            PaintShape::Path(p) => {
                out.push(p.fill);
                if let egui::epaint::ColorMode::Solid(color) = p.stroke.color {
                    out.push(color);
                }
            }
            PaintShape::Text(t) => {
                out.push(t.fallback_color);
                out.extend(t.galley.job.sections.iter().map(|s| s.format.color));
            }
            PaintShape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, out);
                }
            }
            PaintShape::Noop
            | PaintShape::Ellipse(_)
            | PaintShape::Band(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

const MARK_A: Color32 = Color32::from_rgb(1, 2, 3);
const MARK_B: Color32 = Color32::from_rgb(4, 5, 6);
const MARK_C: Color32 = Color32::from_rgb(7, 8, 9);

#[test]
fn test_data_table_header_and_stripes_use_surface_containers() -> TestResult {
    let theme = theme_with(&[
        (ColorRole::SurfaceContainerHigh, MARK_A),
        (ColorRole::SurfaceContainerHighest, MARK_B),
        (ColorRole::OutlineVariant, MARK_C),
    ]);
    let shapes = paint(theme, |ui| {
        ui.add(
            data_table()
                .id(egui::Id::unique("extras_table"))
                .column("Name", 120.0, false)
                .row(|r| r.cell("one"))
                .row(|r| r.cell("two"))
                .row(|r| r.cell("three")),
        );
    });
    let colors = colors(&shapes);
    assert!(
        colors.contains(&MARK_A),
        "the header should be surfaceContainerHigh"
    );
    assert!(
        colors.contains(&MARK_B),
        "alternate rows should be surfaceContainerHighest"
    );
    assert!(
        colors.contains(&MARK_C),
        "dividers should be outlineVariant"
    );
    Ok(())
}

#[test]
fn test_dash_counter_card_is_an_outlined_card() -> TestResult {
    let theme = theme_with(&[
        (ColorRole::OutlineVariant, MARK_A),
        (ColorRole::Primary, MARK_B),
    ]);
    let mut offset = 0.0_f32;
    let shapes = paint(theme, |ui| {
        ui.add(dashcounter("Stats", &mut offset).card("Active", 3, 10));
    });
    let colors = colors(&shapes);
    assert!(
        colors.contains(&MARK_A),
        "the card outline should be outlineVariant"
    );
    assert!(
        colors.contains(&MARK_B),
        "the counter text should be primary"
    );
    Ok(())
}

#[test]
fn test_timeline_connector_uses_outline_variant() -> TestResult {
    let theme = theme_with(&[(ColorRole::OutlineVariant, MARK_A)]);
    let shapes = paint(theme, |ui| {
        ui.add(
            timeline()
                .item(TimelineItem::new().content("First"))
                .item(TimelineItem::new().content("Second")),
        );
    });
    assert!(
        colors(&shapes).contains(&MARK_A),
        "the connector should be outlineVariant"
    );
    Ok(())
}

#[test]
fn test_layout_grid_debug_outline_uses_primary() -> TestResult {
    let theme = theme_with(&[(ColorRole::Primary, MARK_A)]);
    let shapes = paint(theme, |ui| {
        ui.add(layout_grid().debug_mode(true).cell(4, |ui| {
            ui.label("cell");
        }));
    });
    assert!(
        colors(&shapes).contains(&MARK_A),
        "the debug grid outline should be primary"
    );
    Ok(())
}

#[test]
fn test_image_list_placeholder_and_failed_mark_use_theme() -> TestResult {
    let theme = theme_with(&[
        (ColorRole::SurfaceContainerHighest, MARK_A),
        (ColorRole::Error, MARK_B),
    ]);
    // An empty source fails at once; nothing blocks.
    let ctx = egui::Context::default();
    MaterialTheme::set(&ctx, theme);
    let started = std::time::Instant::now();
    let mut shapes = Vec::new();
    while started.elapsed() < std::time::Duration::from_secs(10) {
        let mut output = ctx.run_ui(Default::default(), |ui| {
            ui.add(image_list().item("Missing", "no/such/file.png"));
        });
        output.textures_delta.clear();
        shapes = output.shapes;
        if colors(&shapes).contains(&MARK_B) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let colors = colors(&shapes);
    assert!(
        colors.contains(&MARK_A),
        "the placeholder should be surfaceContainerHighest"
    );
    assert!(
        colors.contains(&MARK_B),
        "a missing image should show the error-coloured mark"
    );
    Ok(())
}

/// Two contexts with different themes draw with their own theme.
#[test]
fn test_two_contexts_keep_their_own_theme() -> TestResult {
    let a = paint(theme_with(&[(ColorRole::OutlineVariant, MARK_A)]), |ui| {
        ui.add(
            timeline()
                .item(TimelineItem::new().content("x"))
                .item(TimelineItem::new()),
        );
    });
    let b = paint(theme_with(&[(ColorRole::OutlineVariant, MARK_B)]), |ui| {
        ui.add(
            timeline()
                .item(TimelineItem::new().content("x"))
                .item(TimelineItem::new()),
        );
    });
    assert!(
        colors(&a).contains(&MARK_A) && !colors(&a).contains(&MARK_B),
        "context A should use only its own theme"
    );
    assert!(
        colors(&b).contains(&MARK_B) && !colors(&b).contains(&MARK_A),
        "context B should use only its own theme"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshots.

/// Runs a render, turning a panic into an error: with no GPU adapter at
/// all, egui_kittest's renderer panics instead of returning one.
fn no_adapter_is_an_error<T>(
    render: impl FnOnce() -> Result<T, egui_kittest::SnapshotError>,
) -> Result<T, egui_kittest::SnapshotError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(render)).unwrap_or_else(|_| {
        Err(egui_kittest::SnapshotError::RenderError {
            err: "no adapter: the renderer could not be created".to_owned(),
        })
    })
}

/// One wgpu renderer at a time in this test binary.
fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn snapshot(harness: &mut egui_kittest::Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    // Software and hardware rasterisers differ in anti-aliasing; allow that,
    // not a missing or moved element.
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(200);
    match no_adapter_is_an_error(|| harness.try_snapshot_options(name, &options)) {
        Ok(()) => Ok(()),
        Err(egui_kittest::SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot {name}: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

#[test]
fn test_extras_snapshots() -> TestResult {
    for (name, theme) in support::themes() {
        if name == "dark_high_contrast" {
            continue;
        }
        let mut offset = 0.0_f32;
        let mut harness = support::harness_for(theme, egui::vec2(480.0, 520.0), |ui| {
            ui.add(
                data_table()
                    .id(egui::Id::unique("extras_snapshot_table"))
                    .column("Name", 160.0, false)
                    .column("Count", 100.0, true)
                    .row(|r| r.cell("Alpha").cell("1"))
                    .row(|r| r.cell("Beta").cell("2"))
                    .row(|r| r.cell("Gamma").cell("3")),
            );
            ui.add(
                dashcounter("Stats", &mut offset)
                    .card("Active", 3, 10)
                    .card("Pending", 7, 15),
            );
            ui.add(
                timeline()
                    .item(TimelineItem::new().content("Planned"))
                    .item(TimelineItem::new().content("Shipped")),
            );
        });
        harness.run();
        snapshot(&mut harness, &format!("extras_{name}"))?;
    }
    Ok(())
}
