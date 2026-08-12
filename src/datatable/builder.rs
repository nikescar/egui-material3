//! Builder methods for MaterialDataTable

use super::{types::*, MaterialDataTable};
use egui::{epaint::CornerRadius, Id};

impl<'a> MaterialDataTable<'a> {
    /// Create a new data table.
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            id: None,
            allow_selection: false,
            allow_drawer: false,
            drawer_row_height: None,
            sticky_header: false,
            progress_visible: false,
            corner_radius: CornerRadius::from(4.0),
            sorted_column: None,
            sort_direction: SortDirection::Ascending,
            default_row_height: 52.0,
            theme: DataTableTheme::default(),
            auto_height: false,
            refresh_interval: 0.05, // Default 50ms throttle for smooth scrolling
        }
    }

    /// Set the initial sort column and direction
    pub fn sort_by(mut self, column_index: usize, direction: SortDirection) -> Self {
        self.sorted_column = Some(column_index);
        self.sort_direction = direction;
        self
    }

    /// Get current sorting state
    pub fn get_sort_state(&self) -> (Option<usize>, SortDirection) {
        (self.sorted_column, self.sort_direction.clone())
    }

    /// Set refresh throttle interval in seconds (0.0 = no throttling)
    /// Recommended: 0.05-0.1 for smooth scrolling with large tables
    pub fn refresh_interval(mut self, interval: f32) -> Self {
        self.refresh_interval = interval;
        self
    }

    /// Add a column to the data table.
    pub fn column(mut self, title: impl Into<String>, width: f32, numeric: bool) -> Self {
        self.columns.push(DataTableColumn {
            title: title.into(),
            header_widget: None,
            width,
            numeric,
            sortable: true, // Make all columns sortable by default
            sort_direction: None,
            h_align: if numeric { HAlign::Right } else { HAlign::Left },
            v_align: VAlign::Center,
            tooltip: None,
            heading_alignment: None,
            column_width: ColumnWidth::Fixed(width),
        });
        self
    }

    /// Add a sortable column to the data table.
    pub fn sortable_column(mut self, title: impl Into<String>, width: f32, numeric: bool) -> Self {
        self.columns.push(DataTableColumn {
            title: title.into(),
            header_widget: None,
            width,
            numeric,
            sortable: true,
            sort_direction: None,
            h_align: if numeric { HAlign::Right } else { HAlign::Left },
            v_align: VAlign::Center,
            tooltip: None,
            heading_alignment: None,
            column_width: ColumnWidth::Fixed(width),
        });
        self
    }

    pub fn sortable_column_with_align(
        mut self,
        title: impl Into<String>,
        width: f32,
        numeric: bool,
        h_align: HAlign,
        v_align: VAlign,
    ) -> Self {
        self.columns.push(DataTableColumn {
            title: title.into(),
            header_widget: None,
            width,
            numeric,
            sortable: true,
            sort_direction: None,
            h_align,
            v_align,
            tooltip: None,
            heading_alignment: None,
            column_width: ColumnWidth::Fixed(width),
        });
        self
    }

    /// Add a column with custom alignment
    pub fn column_with_align(
        mut self,
        title: impl Into<String>,
        width: f32,
        numeric: bool,
        h_align: HAlign,
        v_align: VAlign,
    ) -> Self {
        self.columns.push(DataTableColumn {
            title: title.into(),
            header_widget: None,
            width,
            numeric,
            sortable: true,
            sort_direction: None,
            h_align,
            v_align,
            tooltip: None,
            heading_alignment: None,
            column_width: ColumnWidth::Fixed(width),
        });
        self
    }

    /// Set a tooltip for the most recently added column
    pub fn column_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        if let Some(column) = self.columns.last_mut() {
            column.tooltip = Some(tooltip.into());
        }
        self
    }

    /// Add a row using a builder pattern.
    pub fn row<F>(mut self, f: F) -> Self
    where
        F: FnOnce(DataTableRow<'a>) -> DataTableRow<'a>,
    {
        let row = f(DataTableRow::new());
        self.rows.push(row);
        self
    }

    /// Set the ID for state persistence.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Enable row selection.
    pub fn allow_selection(mut self, allow: bool) -> Self {
        self.allow_selection = allow;
        self
    }

    /// Enable row drawers. Rows with a `.drawer()` closure will show a clickable
    /// arrow (> closed, v open) that expands a panel below the row.
    pub fn allow_drawer(mut self, allow: bool) -> Self {
        self.allow_drawer = allow;
        self
    }

    /// Set the fixed height of expanded drawer panels (default: automatic sizing).
    /// If not set, drawer height will automatically adjust to fit its contents.
    pub fn drawer_row_height(mut self, height: f32) -> Self {
        self.drawer_row_height = Some(height);
        self
    }

    /// Make the header sticky.
    pub fn sticky_header(mut self, sticky: bool) -> Self {
        self.sticky_header = sticky;
        self
    }

    /// Show progress indicator.
    pub fn show_progress(mut self, show: bool) -> Self {
        self.progress_visible = show;
        self
    }

    /// Set corner radius.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set default row height in pixels.
    /// This sets a fixed minimum height for all rows.
    pub fn default_row_height(mut self, height: f32) -> Self {
        self.default_row_height = height;
        self.theme.data_row_min_height = Some(height);
        self.auto_height = false;
        self
    }

    /// Enable automatic row height calculation based on content.
    /// Each row will size independently to fit its content.
    /// You can still set a minimum height that will be respected.
    pub fn auto_row_height(mut self, enabled: bool) -> Self {
        self.auto_height = enabled;
        if enabled {
            // Set a minimal default height to allow content-based sizing
            self.theme.data_row_min_height = Some(20.0);
        }
        self
    }

    /// Set minimum row height for auto-sizing mode.
    /// Only effective when auto_row_height is enabled.
    pub fn min_row_height(mut self, height: f32) -> Self {
        self.theme.data_row_min_height = Some(height);
        self
    }

    /// Set custom theme for this table.
    pub fn theme(mut self, theme: DataTableTheme) -> Self {
        self.theme = theme;
        self
    }
}

impl<'a> Default for MaterialDataTable<'a> {
    fn default() -> Self {
        Self::new()
    }
}
