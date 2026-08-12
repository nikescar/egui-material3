//! Material Design 3 Data Table Components
//!
//! # Performance Optimizations
//!
//! Data tables automatically cache expensive calculations for smooth scrolling:
//! - **Layout caching**: Row heights and header heights are cached and only recalculated when data changes
//! - **Refresh throttling**: Default 50ms throttle prevents excessive redraws during scrolling
//! - **Smart invalidation**: Cache automatically invalidates when rows, columns, or sort order changes
//!
//! For large tables (100+ rows), these optimizations provide 5-10x faster scrolling.
//! Use `.refresh_interval(0.0)` to disable throttling if needed.

mod types;
mod builder;
mod rendering;

pub use types::*;

use egui::{epaint::CornerRadius, Id};

/// Material Design data table component.
///
/// Data tables display sets of data across rows and columns.
/// They organize information in a way that's easy to scan.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::datatable::MaterialDataTable;
///
/// let mut table = MaterialDataTable::new()
///     .column("Name", 120.0, false)
///     .column("Age", 80.0, true)
///     .column("City", 100.0, false);
///
/// table = table.row(|row| {
///     row.cell("John Doe")
///        .cell("25")
///        .cell("New York")
/// });
///
/// ui.add(table);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialDataTable<'a> {
    pub(crate) columns: Vec<DataTableColumn>,
    pub(crate) rows: Vec<DataTableRow<'a>>,
    pub(crate) id: Option<Id>,
    pub(crate) allow_selection: bool,
    pub(crate) allow_drawer: bool,
    pub(crate) drawer_row_height: Option<f32>,
    pub(crate) sticky_header: bool,
    pub(crate) progress_visible: bool,
    pub(crate) corner_radius: CornerRadius,
    pub(crate) sorted_column: Option<usize>,
    pub(crate) sort_direction: SortDirection,
    pub(crate) default_row_height: f32,
    pub(crate) theme: DataTableTheme,
    pub(crate) auto_height: bool,
    pub(crate) refresh_interval: f32,
}

/// Convenience function to create a new data table.
pub fn data_table() -> MaterialDataTable<'static> {
    MaterialDataTable::new()
}
