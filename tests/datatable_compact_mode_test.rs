/// Test: DataTable CompactMode override functionality
///
/// Verifies that tables can be forced into specific display modes regardless
/// of their actual width using the .compact_mode() builder method.
use egui::{Context, Id};
use egui_material3::datatable::{data_table, CompactMode};

#[test]
fn test_auto_mode_uses_automatic_detection() {
    let ctx = Context::default();

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Wide table with Auto mode should use wide styling (default behavior)
            let table = data_table()
                .id(Id::new("auto_wide_test"))
                .compact_mode(CompactMode::Auto)
                .column("Col1", 180.0, false)
                .column("Col2", 150.0, false)
                .column("Col3", 120.0, false)
                .column("Col4", 100.0, false) // Total: 550px (wide)
                .row(|row| row.cell("A").cell("B").cell("C").cell("D"));

            let response = table.show(ui);

            // Should use wide mode automatically
            assert!(
                response.response.rect.width() >= 500.0,
                "Wide table with Auto should stay wide"
            );
        });
    });
}

#[test]
fn test_force_narrow_on_wide_table() {
    let ctx = Context::default();

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Wide table forced to use narrow styling
            let table = data_table()
                .id(Id::new("force_narrow_test"))
                .compact_mode(CompactMode::ForceNarrow)
                .column("Product", 180.0, false)
                .column("Category", 150.0, false)
                .column("Price", 100.0, true)
                .column("Stock", 80.0, true)
                .column("Status", 100.0, false) // Total: 610px
                .row(|row| {
                    row.cell("Laptop")
                        .cell("Electronics")
                        .cell("$999")
                        .cell("15")
                        .cell("Available")
                });

            let response = table.show(ui);

            // Table is wide but should render (we can't easily verify font size in tests)
            assert!(
                response.response.rect.width() > 0.0,
                "ForceNarrow table should render"
            );
            // Note: Visual verification needed for font/padding - tests verify no crashes
        });
    });
}

#[test]
fn test_force_compact_on_wide_table() {
    let ctx = Context::default();

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table = data_table()
                .id(Id::new("force_compact_test"))
                .compact_mode(CompactMode::ForceCompact)
                .column("Item", 150.0, false)
                .column("Description", 200.0, false)
                .column("Value", 100.0, true)
                .column("Status", 100.0, false) // Total: 550px
                .row(|row| {
                    row.cell("Alpha")
                        .cell("First item")
                        .cell("100")
                        .cell("Active")
                });

            let response = table.show(ui);

            assert!(
                response.response.rect.width() > 0.0,
                "ForceCompact table should render"
            );
        });
    });
}

#[test]
fn test_force_wide_on_narrow_table() {
    let ctx = Context::default();

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Narrow table forced to use wide styling
            let table = data_table()
                .id(Id::new("force_wide_test"))
                .compact_mode(CompactMode::ForceWide)
                .column("ID", 50.0, true)
                .column("Name", 80.0, false)
                .column("Val", 60.0, true) // Total: 190px (narrow)
                .row(|row| row.cell("1").cell("Alpha").cell("99"));

            let response = table.show(ui);

            assert!(
                response.response.rect.width() > 0.0,
                "ForceWide table should render"
            );
        });
    });
}

#[test]
fn test_compact_mode_persists_across_redraws() {
    let ctx = Context::default();

    // First frame: set ForceNarrow
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table = data_table()
                .id(Id::new("persist_compact_mode"))
                .compact_mode(CompactMode::ForceNarrow)
                .column("A", 100.0, false)
                .column("B", 100.0, false)
                .column("C", 100.0, false)
                .row(|row| row.cell("X").cell("Y").cell("Z"));

            let _response = table.show(ui);
        });
    });

    // Second frame: redraw same table
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Even without specifying compact_mode again, table should render
            // (compact_mode is not persisted in state, it's part of widget definition)
            let table = data_table()
                .id(Id::new("persist_compact_mode"))
                .compact_mode(CompactMode::ForceNarrow)
                .column("A", 100.0, false)
                .column("B", 100.0, false)
                .column("C", 100.0, false)
                .row(|row| row.cell("X").cell("Y").cell("Z"));

            let response = table.show(ui);
            assert!(response.response.rect.width() > 0.0);
        });
    });
}

#[test]
fn test_all_compact_modes_render_without_panic() {
    let ctx = Context::default();

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Test all 4 modes on the same table structure
            let modes = vec![
                CompactMode::Auto,
                CompactMode::ForceNarrow,
                CompactMode::ForceCompact,
                CompactMode::ForceWide,
            ];

            for (i, mode) in modes.iter().enumerate() {
                let table = data_table()
                    .id(Id::new(format!("mode_test_{}", i)))
                    .compact_mode(mode.clone())
                    .column("Col1", 100.0, false)
                    .column("Col2", 100.0, false)
                    .row(|row| row.cell("A").cell("B"));

                let response = table.show(ui);
                assert!(
                    response.response.rect.width() > 0.0,
                    "Mode {:?} should render successfully",
                    mode
                );
            }
        });
    });
}
