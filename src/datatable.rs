use crate::theme::get_global_color;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke},
    FontFamily, FontId, Id, Rect, Response, Sense, Ui, Vec2, Widget, WidgetText,
};
use std::collections::{HashMap, HashSet};

/// Theme/styling configuration for MaterialDataTable
#[derive(Clone, Debug)]
pub struct DataTableTheme {
    pub decoration: Option<Color32>,
    pub heading_row_color: Option<Color32>,
    pub heading_row_height: Option<f32>,
    pub heading_text_style: Option<(FontId, Color32)>,
    pub data_row_color: Option<Color32>,
    pub data_row_min_height: Option<f32>,
    pub data_row_max_height: Option<f32>,
    pub data_text_style: Option<(FontId, Color32)>,
    pub horizontal_margin: Option<f32>,
    pub column_spacing: Option<f32>,
    pub divider_thickness: Option<f32>,
    pub divider_color: Option<Color32>,
    pub checkbox_horizontal_margin: Option<f32>,
    pub border_stroke: Option<Stroke>,
    pub sort_active_color: Option<Color32>,
    pub sort_inactive_color: Option<Color32>,
    pub selected_row_color: Option<Color32>,
    pub show_bottom_border: bool,
    pub show_checkbox_column: bool,
}

impl Default for DataTableTheme {
    fn default() -> Self {
        Self {
            decoration: None,
            heading_row_color: None,
            heading_row_height: Some(56.0),
            heading_text_style: None,
            data_row_color: None,
            data_row_min_height: Some(52.0),
            data_row_max_height: None,
            data_text_style: None,
            horizontal_margin: Some(24.0),
            column_spacing: Some(56.0),
            divider_thickness: Some(1.0),
            divider_color: None,
            checkbox_horizontal_margin: Some(16.0),
            border_stroke: None,
            sort_active_color: None,
            sort_inactive_color: None,
            selected_row_color: None,
            show_bottom_border: true,
            show_checkbox_column: true,
        }
    }
}

/// Column width specification
#[derive(Clone, Debug, PartialEq)]
pub enum ColumnWidth {
    Fixed(f32),
    Flex(f32),
}

impl Default for ColumnWidth {
    fn default() -> Self {
        ColumnWidth::Fixed(100.0)
    }
}

/// Persistent state for a Material Design data table.
///
/// This structure maintains the state of the table including selections,
/// sorting, and editing state across frames.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DataTableState {
    /// Selection state for each row (true if selected)
    pub selected_rows: Vec<bool>,
    /// State of the header checkbox (for select-all functionality)
    pub header_checkbox: bool,
    /// Sort states for each column by column name
    pub column_sorts: HashMap<String, SortDirection>,
    /// Index of the currently sorted column (if any)
    pub sorted_column: Option<usize>,
    /// Current sort direction for the sorted column
    pub sort_direction: SortDirection,
    /// Set of row indices currently being edited
    pub editing_rows: std::collections::HashSet<usize>,
    /// Temporary edit data for rows being edited (row_index -> cell_values)
    pub edit_data: HashMap<usize, Vec<String>>,
}

/// Response returned by the data table widget.
///
/// Contains both the standard egui Response and additional table-specific
/// information about user interactions.
#[derive(Debug)]
pub struct DataTableResponse {
    /// The standard egui widget response
    pub response: Response,
    /// Current selection state for each row
    pub selected_rows: Vec<bool>,
    /// Current state of the header checkbox
    pub header_checkbox: bool,
    /// Index of column that was clicked for sorting (if any)
    pub column_clicked: Option<usize>,
    /// Current sort state (column index, direction)
    pub sort_state: (Option<usize>, SortDirection),
    /// List of row actions performed (edit, delete, save)
    pub row_actions: Vec<RowAction>,
}

/// Actions that can be performed on data table rows.
#[derive(Debug, Clone)]
pub enum RowAction {
    /// User clicked edit button for the specified row
    Edit(usize),
    /// User clicked delete button for the specified row
    Delete(usize),
    /// User clicked save button for the specified row
    Save(usize),
    /// User clicked cancel button for the specified row
    Cancel(usize),
}

/// Trait for providing data to a table lazily
pub trait DataTableSource {
    fn row_count(&self) -> usize;
    fn get_row(&self, index: usize) -> Option<DataTableRow<'_>>;
    fn is_row_count_approximate(&self) -> bool {
        false
    }
    fn selected_row_count(&self) -> usize {
        0
    }
}

/// Material Design data table component.
///
/// Data tables display sets of data across rows and columns.
/// They organize information in a way that's easy to scan.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Basic data table
/// let mut table = MaterialDataTable::new()
///     .column("Name", 120.0, false)
///     .column("Age", 80.0, true)
///     .column("City", 100.0, false);
///
/// table.row(|row| {
///     row.cell("John Doe");
///     row.cell("25");
///     row.cell("New York");
/// });
///
/// ui.add(table);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialDataTable<'a> {
    columns: Vec<DataTableColumn>,
    rows: Vec<DataTableRow<'a>>,
    id: Option<Id>,
    allow_selection: bool,
    sticky_header: bool,
    progress_visible: bool,
    corner_radius: CornerRadius,
    sorted_column: Option<usize>,
    sort_direction: SortDirection,
    default_row_height: f32,
    theme: DataTableTheme,
    row_hover_states: HashMap<usize, bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

impl Default for VAlign {
    fn default() -> Self {
        VAlign::Center
    }
}

impl Default for HAlign {
    fn default() -> Self {
        HAlign::Left
    }
}

#[derive(Clone)]
pub struct DataTableColumn {
    /// Display title for the column header (can be text or widget closure)
    pub title: String,
    /// Optional widget builder for custom header content
    pub header_widget: Option<std::sync::Arc<dyn Fn(&mut Ui) + Send + Sync>>,
    /// Fixed width of the column in pixels
    pub width: f32,
    /// Whether the column contains numeric data (affects alignment and sorting)
    pub numeric: bool,
    /// Whether this column can be sorted by clicking the header
    pub sortable: bool,
    /// Current sort direction for this column (if sorted)
    pub sort_direction: Option<SortDirection>,
    /// Horizontal alignment for column cells
    pub h_align: HAlign,
    /// Vertical alignment for column cells
    pub v_align: VAlign,
    /// Tooltip text for column header
    pub tooltip: Option<String>,
    /// Heading text alignment (separate from cell alignment)
    pub heading_alignment: Option<HAlign>,
    /// Column width specification
    pub column_width: ColumnWidth,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Ascending
    }
}

pub enum CellContent {
    Text(WidgetText),
    Widget(std::sync::Arc<dyn Fn(&mut Ui) + Send + Sync>),
}

pub struct DataTableCell {
    pub content: CellContent,
    pub h_align: Option<HAlign>,
    pub v_align: Option<VAlign>,
    pub placeholder: bool,
    pub show_edit_icon: bool,
}

impl DataTableCell {
    pub fn text(text: impl Into<WidgetText>) -> Self {
        Self {
            content: CellContent::Text(text.into()),
            h_align: None,
            v_align: None,
            placeholder: false,
            show_edit_icon: false,
        }
    }

    pub fn widget<F>(f: F) -> Self
    where
        F: Fn(&mut Ui) + Send + Sync + 'static,
    {
        Self {
            content: CellContent::Widget(std::sync::Arc::new(f)),
            h_align: None,
            v_align: None,
            placeholder: false,
            show_edit_icon: false,
        }
    }

    pub fn h_align(mut self, align: HAlign) -> Self {
        self.h_align = Some(align);
        self
    }

    pub fn v_align(mut self, align: VAlign) -> Self {
        self.v_align = Some(align);
        self
    }

    pub fn placeholder(mut self, is_placeholder: bool) -> Self {
        self.placeholder = is_placeholder;
        self
    }

    pub fn show_edit_icon(mut self, show: bool) -> Self {
        self.show_edit_icon = show;
        self
    }
}

pub struct DataTableRow<'a> {
    cells: Vec<DataTableCell>,
    selected: bool,
    readonly: bool,
    id: Option<String>,
    color: Option<Color32>,
    on_hover: bool,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> DataTableRow<'a> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            selected: false,
            readonly: false,
            id: None,
            color: None,
            on_hover: true,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Add a text cell
    pub fn cell(mut self, text: impl Into<WidgetText>) -> Self {
        self.cells.push(DataTableCell::text(text));
        self
    }

    /// Add a custom cell with full control
    pub fn custom_cell(mut self, cell: DataTableCell) -> Self {
        self.cells.push(cell);
        self
    }

    /// Add a widget cell
    pub fn widget_cell<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Ui) + Send + Sync + 'static,
    {
        self.cells.push(DataTableCell::widget(f));
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn on_hover(mut self, hover: bool) -> Self {
        self.on_hover = hover;
        self
    }
}

impl<'a> MaterialDataTable<'a> {
    /// Create a new data table.
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            id: None,
            allow_selection: false,
            sticky_header: false,
            progress_visible: false,
            corner_radius: CornerRadius::from(4.0),
            sorted_column: None,
            sort_direction: SortDirection::Ascending,
            default_row_height: 52.0,
            theme: DataTableTheme::default(),
            row_hover_states: HashMap::new(),
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
    pub fn default_row_height(mut self, height: f32) -> Self {
        self.default_row_height = height;
        self
    }

    /// Set custom theme for this table.
    pub fn theme(mut self, theme: DataTableTheme) -> Self {
        self.theme = theme;
        self
    }

    fn get_table_style(&self) -> (Color32, Stroke) {
        let md_surface = self.theme.decoration.unwrap_or_else(|| get_global_color("surface"));
        let md_outline = get_global_color("outline");
        let border_stroke = self.theme.border_stroke.unwrap_or_else(|| Stroke::new(1.0, md_outline));
        (md_surface, border_stroke)
    }

    /// Show the data table and return both UI response and selection state
    pub fn show(self, ui: &mut Ui) -> DataTableResponse {
        let (background_color, border_stroke) = self.get_table_style();

        // Generate table ID for state persistence
        let table_id = self.id.unwrap_or_else(|| {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();

            // Hash based on columns and first few rows for uniqueness
            for col in &self.columns {
                col.title.hash(&mut hasher);
                col.width.to_bits().hash(&mut hasher);
            }
            for (i, row) in self.rows.iter().take(3).enumerate() {
                i.hash(&mut hasher);
                for cell in &row.cells {
                    match &cell.content {
                        CellContent::Text(t) => t.text().hash(&mut hasher),
                        CellContent::Widget(_) => "widget".hash(&mut hasher),
                    }
                }
            }
            self.rows.len().hash(&mut hasher);
            Id::new(format!("datatable_{}", hasher.finish()))
        });

        // Get or create persistent state
        let mut state: DataTableState =
            ui.data_mut(|d| d.get_persisted(table_id).unwrap_or_default());

        // Get external editing state from UI memory if available
        if let Some(external_editing_state) = ui.memory(|mem| {
            mem.data
                .get_temp::<(HashSet<usize>, HashMap<usize, Vec<String>>)>(
                    table_id.with("external_edit_state"),
                )
        }) {
            state.editing_rows = external_editing_state.0;
            state.edit_data = external_editing_state.1;
        }

        // Initialize sorting state from widget if not set
        if state.sorted_column.is_none() && self.sorted_column.is_some() {
            state.sorted_column = self.sorted_column;
            state.sort_direction = self.sort_direction.clone();
        }

        // Ensure state vectors match current row count
        if state.selected_rows.len() != self.rows.len() {
            state.selected_rows.resize(self.rows.len(), false);
        }

        // Sync selection state from rows - always update to match external state
        for (i, row) in self.rows.iter().enumerate() {
            if i < state.selected_rows.len() {
                state.selected_rows[i] = row.selected;
            }
        }

        let MaterialDataTable {
            columns,
            mut rows,
            allow_selection,
            sticky_header: _,
            progress_visible,
            corner_radius,
            default_row_height,
            theme,
            ..
        } = self;

        // Sort rows if a column is selected for sorting
        if let Some(sort_col_idx) = state.sorted_column {
            if let Some(sort_column) = columns.get(sort_col_idx) {
                rows.sort_by(|a, b| {
                    let cell_a_text = a
                        .cells
                        .get(sort_col_idx)
                        .and_then(|c| match &c.content {
                            CellContent::Text(t) => Some(t.text()),
                            CellContent::Widget(_) => None,
                        })
                        .unwrap_or("");
                    let cell_b_text = b
                        .cells
                        .get(sort_col_idx)
                        .and_then(|c| match &c.content {
                            CellContent::Text(t) => Some(t.text()),
                            CellContent::Widget(_) => None,
                        })
                        .unwrap_or("");

                    let comparison = if sort_column.numeric {
                        // Try to parse as numbers for numeric columns
                        let a_num: f64 = cell_a_text.trim_start_matches('$').parse().unwrap_or(0.0);
                        let b_num: f64 = cell_b_text.trim_start_matches('$').parse().unwrap_or(0.0);
                        a_num
                            .partial_cmp(&b_num)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    } else {
                        // Alphabetical comparison for text columns
                        cell_a_text.cmp(cell_b_text)
                    };

                    match state.sort_direction {
                        SortDirection::Ascending => comparison,
                        SortDirection::Descending => comparison.reverse(),
                    }
                });
            }
        }

        // Calculate table dimensions with dynamic row heights
        let checkbox_width = if allow_selection && theme.show_checkbox_column { 48.0 } else { 0.0 };
        let total_width = checkbox_width + columns.iter().map(|col| col.width).sum::<f32>();
        let min_row_height = theme.data_row_min_height.unwrap_or(default_row_height);
        let min_header_height = theme.heading_row_height.unwrap_or(56.0);

        // Calculate header height with text wrapping
        let mut header_height: f32 = min_header_height;
        for column in &columns {
            let available_width = column.width - 48.0; // Account for padding and sort icon
            let header_font = FontId::new(16.0, FontFamily::Proportional);

            let galley = ui.fonts(|f| {
                f.layout_job(egui::text::LayoutJob {
                    text: column.title.clone(),
                    sections: vec![egui::text::LayoutSection {
                        leading_space: 0.0,
                        byte_range: 0..column.title.len(),
                        format: egui::TextFormat {
                            font_id: header_font,
                            color: get_global_color("onSurface"),
                            ..Default::default()
                        },
                    }],
                    wrap: egui::text::TextWrapping {
                        max_width: available_width,
                        ..Default::default()
                    },
                    break_on_newline: true,
                    halign: egui::Align::LEFT,
                    justify: false,
                    first_row_min_height: 0.0,
                    round_output_to_gui: true,
                })
            });

            let content_height: f32 = galley.size().y + 16.0; // Add padding
            header_height = header_height.max(content_height);
        }

        // Calculate individual row heights based on content
        let mut row_heights = Vec::new();
        for row in &rows {
            let mut max_height: f32 = min_row_height;
            for (cell_idx, cell) in row.cells.iter().enumerate() {
                if let Some(column) = columns.get(cell_idx) {
                    match &cell.content {
                        CellContent::Text(cell_text) => {
                            let available_width = column.width - 32.0;
                            let cell_font = FontId::new(14.0, FontFamily::Proportional);

                            let galley = ui.fonts(|f| {
                                f.layout_job(egui::text::LayoutJob {
                                    text: cell_text.text().to_string(),
                                    sections: vec![egui::text::LayoutSection {
                                        leading_space: 0.0,
                                        byte_range: 0..cell_text.text().len(),
                                        format: egui::TextFormat {
                                            font_id: cell_font,
                                            color: get_global_color("onSurface"),
                                            ..Default::default()
                                        },
                                    }],
                                    wrap: egui::text::TextWrapping {
                                        max_width: available_width,
                                        ..Default::default()
                                    },
                                    break_on_newline: true,
                                    halign: egui::Align::LEFT, // Always left-align within galley; positioning handles cell alignment
                                    justify: false,
                                    first_row_min_height: 0.0,
                                    round_output_to_gui: true,
                                })
                            });

                            let content_height: f32 = galley.size().y + 16.0; // Add padding
                            max_height = max_height.max(content_height);
                        }
                        CellContent::Widget(_) => {
                            // For widgets, use minimum height - they will size themselves
                            // We could make this configurable in the future
                        }
                    }
                }
            }
            row_heights.push(max_height);
        }

        let total_height = header_height + row_heights.iter().sum::<f32>();

        // Collect all row actions from this frame
        let mut all_row_actions: Vec<RowAction> = Vec::new();

        // Apply Material theme styling
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let primary = get_global_color("primary");

        let mut style = (*ui.ctx().style()).clone();
        style.visuals.widgets.noninteractive.bg_fill = surface;
        style.visuals.widgets.inactive.bg_fill = surface;
        style.visuals.widgets.hovered.bg_fill =
            egui::Color32::from_rgba_premultiplied(primary.r(), primary.g(), primary.b(), 20);
        style.visuals.widgets.active.bg_fill =
            egui::Color32::from_rgba_premultiplied(primary.r(), primary.g(), primary.b(), 40);
        style.visuals.selection.bg_fill = primary;
        style.visuals.widgets.noninteractive.fg_stroke.color = on_surface;
        style.visuals.widgets.inactive.fg_stroke.color = on_surface;
        style.visuals.widgets.hovered.fg_stroke.color = on_surface;
        style.visuals.widgets.active.fg_stroke.color = on_surface;
        style.visuals.striped = true;
        style.visuals.faint_bg_color = egui::Color32::from_rgba_premultiplied(
            on_surface.r(),
            on_surface.g(),
            on_surface.b(),
            10,
        );
        ui.ctx().set_style(style);

        let desired_size = Vec2::new(total_width, total_height);
        let response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw table background
            ui.painter()
                .rect_filled(rect, corner_radius, background_color);
            ui.painter().rect_stroke(
                rect,
                corner_radius,
                border_stroke,
                egui::epaint::StrokeKind::Outside,
            );

            let mut current_y = rect.min.y;

            // Draw header
            let header_rect = Rect::from_min_size(rect.min, Vec2::new(total_width, header_height));
            let header_bg = theme.heading_row_color.unwrap_or_else(|| get_global_color("surfaceVariant"));
            ui.painter()
                .rect_filled(header_rect, CornerRadius::ZERO, header_bg);

            let mut current_x = rect.min.x;

            // Header checkbox
            if allow_selection && theme.show_checkbox_column {
                let checkbox_rect = Rect::from_min_size(
                    egui::pos2(current_x, current_y),
                    Vec2::new(checkbox_width, header_height),
                );

                let checkbox_center = checkbox_rect.center();
                let checkbox_size = Vec2::splat(18.0);
                let checkbox_inner_rect = Rect::from_center_size(checkbox_center, checkbox_size);

                let checkbox_color = if state.header_checkbox {
                    get_global_color("primary")
                } else {
                    Color32::TRANSPARENT
                };

                ui.painter().rect_filled(
                    checkbox_inner_rect,
                    CornerRadius::from(2.0),
                    checkbox_color,
                );
                ui.painter().rect_stroke(
                    checkbox_inner_rect,
                    CornerRadius::from(2.0),
                    Stroke::new(2.0, get_global_color("outline")),
                    egui::epaint::StrokeKind::Outside,
                );

                if state.header_checkbox {
                    // Draw checkmark
                    let check_points = [
                        checkbox_inner_rect.min + Vec2::new(4.0, 9.0),
                        checkbox_inner_rect.min + Vec2::new(8.0, 13.0),
                        checkbox_inner_rect.min + Vec2::new(14.0, 5.0),
                    ];
                    ui.painter().line_segment(
                        [check_points[0], check_points[1]],
                        Stroke::new(2.0, Color32::WHITE),
                    );
                    ui.painter().line_segment(
                        [check_points[1], check_points[2]],
                        Stroke::new(2.0, Color32::WHITE),
                    );
                }

                // Handle header checkbox click
                let header_checkbox_id = table_id.with("header_checkbox");
                let checkbox_response =
                    ui.interact(checkbox_inner_rect, header_checkbox_id, Sense::click());
                if checkbox_response.clicked() {
                    state.header_checkbox = !state.header_checkbox;
                    // Only update non-readonly rows
                    for (idx, selected) in state.selected_rows.iter_mut().enumerate() {
                        if let Some(row) = rows.get(idx) {
                            if !row.readonly {
                                *selected = state.header_checkbox;
                            }
                        }
                    }
                }

                current_x += checkbox_width;
            }

            // Header columns
            for (col_idx, column) in columns.iter().enumerate() {
                let col_rect = Rect::from_min_size(
                    egui::pos2(current_x, current_y),
                    Vec2::new(column.width, header_height),
                );

                // Render header text with wrapping support
                let available_width = column.width - 48.0; // Account for padding and sort icon
                let header_font = FontId::new(16.0, FontFamily::Proportional);

                let galley = ui.fonts(|f| {
                    f.layout_job(egui::text::LayoutJob {
                        text: column.title.clone(),
                        sections: vec![egui::text::LayoutSection {
                            leading_space: 0.0,
                            byte_range: 0..column.title.len(),
                            format: egui::TextFormat {
                                font_id: header_font,
                                color: get_global_color("onSurface"),
                                ..Default::default()
                            },
                        }],
                        wrap: egui::text::TextWrapping {
                            max_width: available_width,
                            ..Default::default()
                        },
                        break_on_newline: true,
                        halign: egui::Align::LEFT,
                        justify: false,
                        first_row_min_height: 0.0,
                        round_output_to_gui: true,
                    })
                });

                let text_pos = egui::pos2(
                    current_x + 16.0,
                    current_y + (header_height - galley.size().y) / 2.0,
                );

                ui.painter()
                    .galley(text_pos, galley, get_global_color("onSurface"));

                // Handle column header clicks for sorting
                if column.sortable {
                    let header_click_id = table_id.with(format!("column_header_{}", col_idx));
                    let mut header_response = ui.interact(col_rect, header_click_id, Sense::click());
                    
                    // Show tooltip if available
                    if let Some(ref tooltip) = column.tooltip {
                        header_response = header_response.on_hover_text(tooltip);
                    }
                    
                    if header_response.clicked() {
                        // Handle sorting logic
                        if state.sorted_column == Some(col_idx) {
                            // Same column clicked, toggle direction
                            state.sort_direction = match state.sort_direction {
                                SortDirection::Ascending => SortDirection::Descending,
                                SortDirection::Descending => SortDirection::Ascending,
                            };
                        } else {
                            // New column clicked
                            state.sorted_column = Some(col_idx);
                            state.sort_direction = SortDirection::Ascending;
                        }
                        ui.memory_mut(|mem| {
                            mem.data
                                .insert_temp(table_id.with("column_clicked"), Some(col_idx));
                        });
                    }

                    let icon_pos = egui::pos2(
                        current_x + column.width - 32.0,
                        current_y + (header_height - 24.0) / 2.0,
                    );
                    let icon_rect = Rect::from_min_size(icon_pos, Vec2::splat(24.0));

                    // Determine if this column is currently sorted
                    let is_sorted = state.sorted_column == Some(col_idx);
                    let sort_direction = if is_sorted {
                        Some(&state.sort_direction)
                    } else {
                        None
                    };

                    // Draw sort arrow with enhanced visual feedback
                    let arrow_color = if is_sorted {
                        theme.sort_active_color.unwrap_or_else(|| get_global_color("primary")) // Highlight active sort column
                    } else {
                        theme.sort_inactive_color.unwrap_or_else(|| get_global_color("onSurfaceVariant"))
                    };

                    let center = icon_rect.center();

                    // Draw triangle arrows
                    match sort_direction {
                        Some(SortDirection::Ascending) => {
                            // Up triangle (▲)
                            let points = [
                                center + Vec2::new(0.0, -6.0), // Top point
                                center + Vec2::new(-5.0, 4.0), // Bottom left
                                center + Vec2::new(5.0, 4.0),  // Bottom right
                            ];
                            ui.painter().line_segment(
                                [points[0], points[1]],
                                Stroke::new(2.0, arrow_color),
                            );
                            ui.painter().line_segment(
                                [points[1], points[2]],
                                Stroke::new(2.0, arrow_color),
                            );
                            ui.painter().line_segment(
                                [points[2], points[0]],
                                Stroke::new(2.0, arrow_color),
                            );
                        }
                        Some(SortDirection::Descending) => {
                            // Down triangle (▼)
                            let points = [
                                center + Vec2::new(0.0, 6.0),   // Bottom point
                                center + Vec2::new(-5.0, -4.0), // Top left
                                center + Vec2::new(5.0, -4.0),  // Top right
                            ];
                            ui.painter().line_segment(
                                [points[0], points[1]],
                                Stroke::new(2.0, arrow_color),
                            );
                            ui.painter().line_segment(
                                [points[1], points[2]],
                                Stroke::new(2.0, arrow_color),
                            );
                            ui.painter().line_segment(
                                [points[2], points[0]],
                                Stroke::new(2.0, arrow_color),
                            );
                        }
                        None => {
                            // Neutral state - show both arrows faintly
                            let light_color = arrow_color.gamma_multiply(0.5);
                            // Up triangle
                            let up_points = [
                                center + Vec2::new(0.0, -8.0),
                                center + Vec2::new(-3.0, -2.0),
                                center + Vec2::new(3.0, -2.0),
                            ];
                            ui.painter().line_segment(
                                [up_points[0], up_points[1]],
                                Stroke::new(1.0, light_color),
                            );
                            ui.painter().line_segment(
                                [up_points[1], up_points[2]],
                                Stroke::new(1.0, light_color),
                            );
                            ui.painter().line_segment(
                                [up_points[2], up_points[0]],
                                Stroke::new(1.0, light_color),
                            );

                            // Down triangle
                            let down_points = [
                                center + Vec2::new(0.0, 8.0),
                                center + Vec2::new(-3.0, 2.0),
                                center + Vec2::new(3.0, 2.0),
                            ];
                            ui.painter().line_segment(
                                [down_points[0], down_points[1]],
                                Stroke::new(1.0, light_color),
                            );
                            ui.painter().line_segment(
                                [down_points[1], down_points[2]],
                                Stroke::new(1.0, light_color),
                            );
                            ui.painter().line_segment(
                                [down_points[2], down_points[0]],
                                Stroke::new(1.0, light_color),
                            );
                        }
                    }
                }

                current_x += column.width;
            }

            current_y += header_height;

            // Draw rows with dynamic heights
            for (row_idx, row) in rows.iter().enumerate() {
                let row_height = row_heights.get(row_idx).copied().unwrap_or(min_row_height);
                let row_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(total_width, row_height),
                );

                let row_selected = state.selected_rows.get(row_idx).copied().unwrap_or(false);
                
                // Determine row background color with priority: custom color > selected > readonly > alternating
                let row_bg = if let Some(custom_color) = row.color {
                    custom_color
                } else if row_selected {
                    theme.selected_row_color.unwrap_or_else(|| get_global_color("primaryContainer"))
                } else if row.readonly {
                    // Subtle background for readonly rows
                    let surface_variant = get_global_color("surfaceVariant");
                    Color32::from_rgba_premultiplied(
                        surface_variant.r(),
                        surface_variant.g(),
                        surface_variant.b(),
                        (surface_variant.a() as f32 * 0.3) as u8,
                    )
                } else if row_idx % 2 == 1 {
                    theme.data_row_color.unwrap_or_else(|| get_global_color("surfaceVariant"))
                } else {
                    background_color
                };

                ui.painter()
                    .rect_filled(row_rect, CornerRadius::ZERO, row_bg);
                    
                // Draw divider below row
                if row_idx < rows.len() - 1 || theme.show_bottom_border {
                    let divider_y = current_y + row_height;
                    let divider_thickness = theme.divider_thickness.unwrap_or(1.0);
                    let divider_color = theme.divider_color.unwrap_or_else(|| get_global_color("outlineVariant"));
                    ui.painter().line_segment(
                        [
                            egui::pos2(rect.min.x, divider_y),
                            egui::pos2(rect.min.x + total_width, divider_y),
                        ],
                        Stroke::new(divider_thickness, divider_color),
                    );
                }

                current_x = rect.min.x;

                // Row checkbox
                if allow_selection && theme.show_checkbox_column {
                    let checkbox_rect = Rect::from_min_size(
                        egui::pos2(current_x, current_y),
                        Vec2::new(checkbox_width, row_height),
                    );

                    let checkbox_center = checkbox_rect.center();
                    let checkbox_size = Vec2::splat(18.0);
                    let checkbox_inner_rect =
                        Rect::from_center_size(checkbox_center, checkbox_size);

                    let checkbox_color = if row_selected {
                        get_global_color("primary")
                    } else {
                        Color32::TRANSPARENT
                    };

                    let border_color = if row.readonly {
                        get_global_color("outline").linear_multiply(0.5) // Dimmed for readonly
                    } else {
                        get_global_color("outline")
                    };

                    ui.painter().rect_filled(
                        checkbox_inner_rect,
                        CornerRadius::from(2.0),
                        checkbox_color,
                    );
                    ui.painter().rect_stroke(
                        checkbox_inner_rect,
                        CornerRadius::from(2.0),
                        Stroke::new(2.0, border_color),
                        egui::epaint::StrokeKind::Outside,
                    );

                    if row_selected {
                        // Draw checkmark
                        let check_points = [
                            checkbox_inner_rect.min + Vec2::new(4.0, 9.0),
                            checkbox_inner_rect.min + Vec2::new(8.0, 13.0),
                            checkbox_inner_rect.min + Vec2::new(14.0, 5.0),
                        ];
                        ui.painter().line_segment(
                            [check_points[0], check_points[1]],
                            Stroke::new(2.0, Color32::WHITE),
                        );
                        ui.painter().line_segment(
                            [check_points[1], check_points[2]],
                            Stroke::new(2.0, Color32::WHITE),
                        );
                    }

                    // Handle row checkbox click
                    let row_checkbox_id = table_id.with(format!("row_checkbox_{}", row_idx));
                    let checkbox_response =
                        ui.interact(checkbox_inner_rect, row_checkbox_id, Sense::click());
                    if checkbox_response.clicked() && !row.readonly {
                        if let Some(selected) = state.selected_rows.get_mut(row_idx) {
                            *selected = !*selected;
                        }

                        // Update header checkbox state based on row selections
                        // Only consider non-readonly rows for header checkbox state
                        let non_readonly_indices: Vec<usize> = rows
                            .iter()
                            .enumerate()
                            .filter(|(_, row)| !row.readonly)
                            .map(|(idx, _)| idx)
                            .collect();

                        if !non_readonly_indices.is_empty() {
                            let all_non_readonly_selected = non_readonly_indices
                                .iter()
                                .all(|&idx| state.selected_rows.get(idx).copied().unwrap_or(false));
                            let none_non_readonly_selected =
                                non_readonly_indices.iter().all(|&idx| {
                                    !state.selected_rows.get(idx).copied().unwrap_or(false)
                                });
                            state.header_checkbox =
                                all_non_readonly_selected && !none_non_readonly_selected;
                        }
                    }

                    current_x += checkbox_width;
                }

                // Track row actions for this specific row
                let mut row_actions: Vec<RowAction> = Vec::new();

                // Row cells
                for (cell_idx, cell) in row.cells.iter().enumerate() {
                    if let Some(column) = columns.get(cell_idx) {
                        let _cell_rect = Rect::from_min_size(
                            egui::pos2(current_x, current_y),
                            Vec2::new(column.width, row_height),
                        );

                        let is_row_editing = state.editing_rows.contains(&row_idx);
                        let is_actions_column = column.title == "Actions";

                        if is_actions_column {
                            // Render action buttons
                            let button_rect = Rect::from_min_size(
                                egui::pos2(current_x + 8.0, current_y + (row_height - 32.0) / 2.0),
                                Vec2::new(column.width - 16.0, 32.0),
                            );

                            ui.scope_builder(egui::UiBuilder::new().max_rect(button_rect), |ui| {
                                ui.horizontal(|ui| {
                                    if is_row_editing {
                                        if ui.small_button("Save").clicked() {
                                            row_actions.push(RowAction::Save(row_idx));
                                        }
                                        if ui.small_button("Cancel").clicked() {
                                            row_actions.push(RowAction::Cancel(row_idx));
                                        }
                                    } else {
                                        if ui.small_button("Edit").clicked() {
                                            row_actions.push(RowAction::Edit(row_idx));
                                        }
                                        if ui.small_button("Delete").clicked() {
                                            row_actions.push(RowAction::Delete(row_idx));
                                        }
                                    }
                                });
                            });
                        } else if is_row_editing {
                            // Render editable text field
                            let edit_rect = Rect::from_min_size(
                                egui::pos2(current_x + 8.0, current_y + (row_height - 24.0) / 2.0),
                                Vec2::new(column.width - 16.0, 24.0),
                            );

                            // Get or initialize edit data
                            let edit_data = state.edit_data.entry(row_idx).or_insert_with(|| {
                                row.cells
                                    .iter()
                                    .map(|c| match &c.content {
                                        CellContent::Text(t) => t.text().to_string(),
                                        CellContent::Widget(_) => String::new(),
                                    })
                                    .collect()
                            });

                            // Ensure we have enough entries for this cell
                            if edit_data.len() <= cell_idx {
                                edit_data.resize(cell_idx + 1, String::new());
                            }

                            let edit_text = &mut edit_data[cell_idx];

                            ui.scope_builder(egui::UiBuilder::new().max_rect(edit_rect), |ui| {
                                ui.add(
                                    egui::TextEdit::singleline(edit_text)
                                        .desired_width(column.width - 16.0),
                                );
                            });
                        } else {
                            // Determine alignment from cell or column
                            let h_align = cell.h_align.as_ref().unwrap_or(&column.h_align);
                            let v_align = cell.v_align.as_ref().unwrap_or(&column.v_align);

                            match &cell.content {
                                CellContent::Text(cell_text) => {
                                    // Render normal text with alignment
                                    let available_width = column.width - 32.0; // Account for padding
                                    let cell_font = if let Some((ref font_id, _)) = theme.data_text_style {
                                        font_id.clone()
                                    } else {
                                        FontId::new(14.0, FontFamily::Proportional)
                                    };
                                    
                                    let text_color = if cell.placeholder {
                                        let base_color = get_global_color("onSurface");
                                        Color32::from_rgba_premultiplied(
                                            base_color.r(),
                                            base_color.g(),
                                            base_color.b(),
                                            (base_color.a() as f32 * 0.6) as u8,
                                        )
                                    } else if let Some((_, ref color)) = theme.data_text_style {
                                        *color
                                    } else {
                                        get_global_color("onSurface")
                                    };

                                    let galley = ui.fonts(|f| {
                                        f.layout_job(egui::text::LayoutJob {
                                            text: cell_text.text().to_string(),
                                            sections: vec![egui::text::LayoutSection {
                                                leading_space: 0.0,
                                                byte_range: 0..cell_text.text().len(),
                                                format: egui::TextFormat {
                                                    font_id: cell_font,
                                                    color: text_color,
                                                    ..Default::default()
                                                },
                                            }],
                                            wrap: egui::text::TextWrapping {
                                                max_width: available_width,
                                                ..Default::default()
                                            },
                                            break_on_newline: true,
                                            halign: egui::Align::LEFT, // Always left-align within galley; positioning handles cell alignment
                                            justify: false,
                                            first_row_min_height: 0.0,
                                            round_output_to_gui: true,
                                        })
                                    });

                                    // Calculate horizontal position based on alignment
                                    let text_x = match h_align {
                                        HAlign::Left => current_x + 16.0,
                                        HAlign::Center => {
                                            current_x + (column.width - galley.size().x) / 2.0
                                        }
                                        HAlign::Right => {
                                            current_x + column.width - 16.0 - galley.size().x
                                        }
                                    };

                                    // Calculate vertical position based on alignment
                                    let text_y = match v_align {
                                        VAlign::Top => current_y + 8.0,
                                        VAlign::Center => {
                                            current_y + (row_height - galley.size().y) / 2.0
                                        }
                                        VAlign::Bottom => {
                                            current_y + row_height - galley.size().y - 8.0
                                        }
                                    };

                                    let text_pos = egui::pos2(text_x, text_y);
                                    ui.painter().galley(
                                        text_pos,
                                        galley,
                                        text_color,
                                    );
                                    
                                    // Draw edit icon if requested
                                    if cell.show_edit_icon {
                                        let icon_size = 16.0;
                                        let icon_x = current_x + column.width - icon_size - 8.0;
                                        let icon_y = current_y + (row_height - icon_size) / 2.0;
                                        let icon_rect = Rect::from_min_size(
                                            egui::pos2(icon_x, icon_y),
                                            Vec2::splat(icon_size),
                                        );
                                        // Draw simple pencil icon
                                        let icon_color = get_global_color("onSurfaceVariant");
                                        ui.painter().line_segment(
                                            [
                                                icon_rect.left_top() + Vec2::new(4.0, 10.0),
                                                icon_rect.left_top() + Vec2::new(10.0, 4.0),
                                            ],
                                            Stroke::new(1.5, icon_color),
                                        );
                                        ui.painter().line_segment(
                                            [
                                                icon_rect.left_top() + Vec2::new(2.0, 12.0),
                                                icon_rect.left_top() + Vec2::new(4.0, 10.0),
                                            ],
                                            Stroke::new(1.5, icon_color),
                                        );
                                    }
                                }
                                CellContent::Widget(widget_fn) => {
                                    // Render custom widget
                                    // Calculate widget rect based on alignment
                                    let padding = 8.0;
                                    let available_width = column.width - 2.0 * padding;
                                    let available_height = row_height - 2.0 * padding;

                                    // For now, center the widget area. Alignment can be refined based on widget's actual size
                                    let widget_rect = match (h_align, v_align) {
                                        (HAlign::Left, VAlign::Top) => Rect::from_min_size(
                                            egui::pos2(current_x + padding, current_y + padding),
                                            Vec2::new(available_width, available_height),
                                        ),
                                        (HAlign::Center, VAlign::Center) => Rect::from_min_size(
                                            egui::pos2(current_x + padding, current_y + padding),
                                            Vec2::new(available_width, available_height),
                                        ),
                                        (HAlign::Right, VAlign::Center) => Rect::from_min_size(
                                            egui::pos2(current_x + padding, current_y + padding),
                                            Vec2::new(available_width, available_height),
                                        ),
                                        _ => Rect::from_min_size(
                                            egui::pos2(current_x + padding, current_y + padding),
                                            Vec2::new(available_width, available_height),
                                        ),
                                    };

                                    ui.scope_builder(
                                        egui::UiBuilder::new().max_rect(widget_rect),
                                        |ui| {
                                            // Apply alignment to the UI
                                            match h_align {
                                                HAlign::Left => ui.with_layout(
                                                    egui::Layout::left_to_right(egui::Align::Min),
                                                    |ui| {
                                                        widget_fn(ui);
                                                    },
                                                ),
                                                HAlign::Center => ui.with_layout(
                                                    egui::Layout::left_to_right(
                                                        egui::Align::Center,
                                                    ),
                                                    |ui| {
                                                        widget_fn(ui);
                                                    },
                                                ),
                                                HAlign::Right => ui.with_layout(
                                                    egui::Layout::right_to_left(egui::Align::Min),
                                                    |ui| {
                                                        widget_fn(ui);
                                                    },
                                                ),
                                            };
                                        },
                                    );
                                }
                            }
                        }

                        current_x += column.width;
                    }
                }

                // Add this row's actions to the global collection
                all_row_actions.extend(row_actions);

                current_y += row_height;
            }

            // Draw progress indicator if visible
            if progress_visible {
                let scrim_color = Color32::from_rgba_unmultiplied(255, 255, 255, 128);
                ui.painter().rect_filled(rect, corner_radius, scrim_color);

                // Draw progress bar
                let progress_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, rect.min.y + header_height),
                    Vec2::new(total_width, 4.0),
                );

                let progress_color = get_global_color("primary");
                ui.painter()
                    .rect_filled(progress_rect, CornerRadius::ZERO, progress_color);
            }
        }

        // Persist the state
        ui.data_mut(|d| d.insert_persisted(table_id, state.clone()));

        // Store editing state back to memory for external access
        ui.memory_mut(|mem| {
            mem.data.insert_temp(
                table_id.with("external_edit_state"),
                (state.editing_rows.clone(), state.edit_data.clone()),
            );
        });

        // Check for column clicks using stored state
        let column_clicked = ui
            .memory(|mem| {
                mem.data
                    .get_temp::<Option<usize>>(table_id.with("column_clicked"))
            })
            .flatten();

        // Clear the stored click state
        ui.memory_mut(|mem| {
            mem.data
                .remove::<Option<usize>>(table_id.with("column_clicked"));
        });

        DataTableResponse {
            response,
            selected_rows: state.selected_rows,
            header_checkbox: state.header_checkbox,
            column_clicked,
            sort_state: (state.sorted_column, state.sort_direction.clone()),
            row_actions: all_row_actions,
        }
    }
}

impl<'a> Default for MaterialDataTable<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialDataTable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

/// Convenience function to create a new data table.
pub fn data_table() -> MaterialDataTable<'static> {
    MaterialDataTable::new()
}
