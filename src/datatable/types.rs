//! Type definitions for Material Design 3 Data Table

use egui::{ecolor::Color32, epaint::Stroke, FontId, WidgetText};
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

/// Sort direction for columns
#[derive(Clone, Debug, PartialEq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

/// Vertical alignment for cells
#[derive(Clone, Debug, PartialEq, Default)]
pub enum VAlign {
    Top,
    #[default]
    Center,
    Bottom,
}

/// Horizontal alignment for cells
#[derive(Clone, Debug, PartialEq, Default)]
pub enum HAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Actions that can be performed on data table rows
#[derive(Debug, Clone)]
pub enum RowAction {
    Edit(usize),
    Delete(usize),
    Save(usize),
    Cancel(usize),
}

/// Cell content - either text or a custom widget
pub enum CellContent {
    Text(WidgetText),
    Widget(std::sync::Arc<dyn Fn(&mut egui::Ui) + Send + Sync>),
}

/// A single cell in the data table
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
        F: Fn(&mut egui::Ui) + Send + Sync + 'static,
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

/// A column definition in the data table
#[derive(Clone)]
pub struct DataTableColumn {
    pub title: String,
    #[allow(clippy::type_complexity)]
    pub header_widget: Option<std::sync::Arc<dyn Fn(&mut egui::Ui) + Send + Sync>>,
    pub width: f32,
    pub numeric: bool,
    pub sortable: bool,
    pub sort_direction: Option<SortDirection>,
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub tooltip: Option<String>,
    pub heading_alignment: Option<HAlign>,
    pub column_width: ColumnWidth,
}

/// A single row in the data table
pub struct DataTableRow<'a> {
    pub cells: Vec<DataTableCell>,
    pub selected: bool,
    pub selection_externally_set: bool,
    pub readonly: bool,
    pub id: Option<String>,
    pub color: Option<Color32>,
    pub on_hover: bool,
    #[allow(clippy::type_complexity)]
    pub drawer: Option<std::sync::Arc<dyn Fn(&mut egui::Ui) + Send + Sync>>,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Default for DataTableRow<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> DataTableRow<'a> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            selected: false,
            selection_externally_set: false,
            readonly: false,
            id: None,
            color: None,
            on_hover: true,
            drawer: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn cell(mut self, text: impl Into<WidgetText>) -> Self {
        self.cells.push(DataTableCell::text(text));
        self
    }

    pub fn cell_widget<F>(mut self, widget: F) -> Self
    where
        F: Fn(&mut egui::Ui) + Send + Sync + 'static,
    {
        self.cells.push(DataTableCell::widget(widget));
        self
    }

    pub fn cell_custom(mut self, cell: DataTableCell) -> Self {
        self.cells.push(cell);
        self
    }

    /// Alias for cell_custom (backwards compatibility)
    pub fn custom_cell(mut self, cell: DataTableCell) -> Self {
        self.cell_custom(cell)
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self.selection_externally_set = true;
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

    pub fn drawer<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut egui::Ui) + Send + Sync + 'static,
    {
        self.drawer = Some(std::sync::Arc::new(f));
        self
    }
}

/// Persistent state for a Material Design data table
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DataTableState {
    pub selected_rows: Vec<bool>,
    pub header_checkbox: bool,
    pub column_sorts: HashMap<String, SortDirection>,
    pub sorted_column: Option<usize>,
    pub sort_direction: SortDirection,
    pub editing_rows: HashSet<usize>,
    pub edit_data: HashMap<usize, Vec<String>>,
    pub drawer_open_rows: HashSet<usize>,

    #[serde(skip)]
    pub cached_row_heights: Vec<f32>,
    #[serde(skip)]
    pub cached_header_height: f32,
    #[serde(skip)]
    pub cached_sorted_indices: Vec<usize>,
    #[serde(skip)]
    pub layout_cache_hash: u64,
    #[serde(skip)]
    pub last_refresh_time: f64,
}

/// Response returned by the data table widget
#[derive(Debug)]
pub struct DataTableResponse {
    pub response: egui::Response,
    pub selected_rows: Vec<bool>,
    pub header_checkbox: bool,
    pub column_clicked: Option<usize>,
    pub sort_state: (Option<usize>, SortDirection),
    pub row_actions: Vec<RowAction>,
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
