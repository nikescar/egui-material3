/// Test: DataTable sorting state persists across redraws and scrolling
///
/// Verifies that when a user clicks a column header to sort, the sort state
/// is preserved even after scrolling or redrawing the table.
use egui::{Context, Id};
use egui_material3::datatable::{data_table, SortDirection};

#[test]
fn test_sort_state_persists_across_redraws() {
    let ctx = Context::default();

    // First frame: render table and click to sort
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table = data_table()
                .id(Id::new("persist_test_table"))
                .sortable_column("Name", 120.0, false)
                .sortable_column("Value", 80.0, true)
                .row(|row| row.cell("Zebra").cell("100"))
                .row(|row| row.cell("Alpha").cell("200"))
                .row(|row| row.cell("Beta").cell("50"));

            let response = table.show(ui);

            // Initially, no column should be sorted
            assert_eq!(
                response.sort_state.0, None,
                "No column should be sorted initially"
            );
        });
    });

    // Second frame: simulate clicking the first column header
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Simulate header click by directly modifying state
            let table_id = Id::new("persist_test_table");
            ui.data_mut(|d| {
                let mut state = d
                    .get_persisted_mut_or_default::<egui_material3::datatable::DataTableState>(
                        table_id,
                    );
                state.sorted_column = Some(0);
                state.sort_direction = SortDirection::Ascending;
            });

            let table = data_table()
                .id(Id::new("persist_test_table"))
                .sortable_column("Name", 120.0, false)
                .sortable_column("Value", 80.0, true)
                .row(|row| row.cell("Zebra").cell("100"))
                .row(|row| row.cell("Alpha").cell("200"))
                .row(|row| row.cell("Beta").cell("50"));

            let response = table.show(ui);

            // Verify sorting is applied
            assert_eq!(response.sort_state.0, Some(0), "Column 0 should be sorted");
            assert_eq!(
                response.sort_state.1,
                SortDirection::Ascending,
                "Should be ascending"
            );
        });
    });

    // Third frame: redraw without any interaction (simulate scroll)
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table = data_table()
                .id(Id::new("persist_test_table"))
                .sortable_column("Name", 120.0, false)
                .sortable_column("Value", 80.0, true)
                .row(|row| row.cell("Zebra").cell("100"))
                .row(|row| row.cell("Alpha").cell("200"))
                .row(|row| row.cell("Beta").cell("50"));

            let response = table.show(ui);

            // CRITICAL: Sorting state should persist
            assert_eq!(
                response.sort_state.0,
                Some(0),
                "Sorting should persist after redraw"
            );
            assert_eq!(
                response.sort_state.1,
                SortDirection::Ascending,
                "Direction should persist"
            );
        });
    });
}

#[test]
fn test_sort_direction_toggles_correctly() {
    let ctx = Context::default();

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table_id = Id::new("toggle_test_table");

            // Set initial sort: column 0, ascending
            ui.data_mut(|d| {
                let mut state = d
                    .get_persisted_mut_or_default::<egui_material3::datatable::DataTableState>(
                        table_id,
                    );
                state.sorted_column = Some(0);
                state.sort_direction = SortDirection::Ascending;
            });

            let table = data_table()
                .id(table_id)
                .sortable_column("Name", 120.0, false)
                .sortable_column("Value", 80.0, true)
                .row(|row| row.cell("Zebra").cell("100"))
                .row(|row| row.cell("Alpha").cell("200"));

            let response = table.show(ui);
            assert_eq!(response.sort_state.1, SortDirection::Ascending);

            // Simulate clicking the same column again (toggle direction)
            ui.data_mut(|d| {
                let mut state = d
                    .get_persisted_mut_or_default::<egui_material3::datatable::DataTableState>(
                        table_id,
                    );
                state.sort_direction = SortDirection::Descending;
            });
        });
    });

    // Next frame: verify direction persisted
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table = data_table()
                .id(Id::new("toggle_test_table"))
                .sortable_column("Name", 120.0, false)
                .sortable_column("Value", 80.0, true)
                .row(|row| row.cell("Zebra").cell("100"))
                .row(|row| row.cell("Alpha").cell("200"));

            let response = table.show(ui);
            assert_eq!(
                response.sort_state.1,
                SortDirection::Descending,
                "Direction toggle should persist"
            );
        });
    });
}

#[test]
fn test_different_columns_have_independent_sort_state() {
    let ctx = Context::default();

    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table_id = Id::new("multi_col_test");

            // Sort by column 0
            ui.data_mut(|d| {
                let mut state = d
                    .get_persisted_mut_or_default::<egui_material3::datatable::DataTableState>(
                        table_id,
                    );
                state.sorted_column = Some(0);
                state.sort_direction = SortDirection::Ascending;
            });

            let table = data_table()
                .id(table_id)
                .sortable_column("Name", 120.0, false)
                .sortable_column("Value", 80.0, true)
                .row(|row| row.cell("Zebra").cell("100"))
                .row(|row| row.cell("Alpha").cell("200"));

            let response = table.show(ui);
            assert_eq!(response.sort_state.0, Some(0));

            // Switch to column 1
            ui.data_mut(|d| {
                let mut state = d
                    .get_persisted_mut_or_default::<egui_material3::datatable::DataTableState>(
                        table_id,
                    );
                state.sorted_column = Some(1);
                state.sort_direction = SortDirection::Ascending; // Reset to ascending for new column
            });
        });
    });

    // Verify column 1 is now sorted
    ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table = data_table()
                .id(Id::new("multi_col_test"))
                .sortable_column("Name", 120.0, false)
                .sortable_column("Value", 80.0, true)
                .row(|row| row.cell("Zebra").cell("100"))
                .row(|row| row.cell("Alpha").cell("200"));

            let response = table.show(ui);
            assert_eq!(response.sort_state.0, Some(1), "Should switch to column 1");
            assert_eq!(response.sort_state.1, SortDirection::Ascending);
        });
    });
}
