use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    FontFamily, FontId,
    Id, Rect, Response, Sense, Ui, Vec2, Widget, WidgetText,
};
use std::collections::HashMap;

/// Persistent state for a data table
#[derive(Clone, Debug, Default)]
pub struct DataTableState {
    /// Selection state for each row
    pub selected_rows: Vec<bool>,
    /// Header checkbox state
    pub header_checkbox: bool,
    /// Column sort states
    pub column_sorts: HashMap<String, SortDirection>,
}

/// Response from a data table widget including selection state
#[derive(Debug)]
pub struct DataTableResponse {
    /// The UI response
    pub response: Response,
    /// Current selection state for each row
    pub selected_rows: Vec<bool>,
    /// Header checkbox state
    pub header_checkbox: bool,
    /// Column that was clicked for sorting (if any)
    pub column_clicked: Option<usize>,
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
}

#[derive(Clone, Debug)]
pub struct DataTableColumn {
    pub title: String,
    pub width: f32,
    pub numeric: bool,
    pub sortable: bool,
    pub sort_direction: Option<SortDirection>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

pub struct DataTableRow<'a> {
    cells: Vec<WidgetText>,
    selected: bool,
    readonly: bool,
    id: Option<String>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> DataTableRow<'a> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            selected: false,
            readonly: false,
            id: None,
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn cell(mut self, text: impl Into<WidgetText>) -> Self {
        self.cells.push(text.into());
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
        }
    }

    /// Add a column to the data table.
    pub fn column(mut self, title: impl Into<String>, width: f32, numeric: bool) -> Self {
        self.columns.push(DataTableColumn {
            title: title.into(),
            width,
            numeric,
            sortable: true, // Make all columns sortable by default
            sort_direction: None,
        });
        self
    }

    /// Add a sortable column to the data table.
    pub fn sortable_column(mut self, title: impl Into<String>, width: f32, numeric: bool) -> Self {
        self.columns.push(DataTableColumn {
            title: title.into(),
            width,
            numeric,
            sortable: true,
            sort_direction: None,
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

    fn get_table_style(&self) -> (Color32, Stroke) {
        let md_surface = get_global_color("surface");
        let md_outline = get_global_color("outline");
        (md_surface, Stroke::new(1.0, md_outline))
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
                    cell.text().hash(&mut hasher);
                }
            }
            self.rows.len().hash(&mut hasher);
            Id::new(format!("datatable_{}", hasher.finish()))
        });

        // Get or create persistent state
        let mut state: DataTableState = ui.data_mut(|d| d.get_persisted(table_id).unwrap_or_default());
        
        // Ensure state vectors match current row count
        if state.selected_rows.len() != self.rows.len() {
            state.selected_rows.resize(self.rows.len(), false);
        }
        
        // Initialize selection from rows if this is the first time or rows changed
        for (i, row) in self.rows.iter().enumerate() {
            if i < state.selected_rows.len() && row.selected {
                state.selected_rows[i] = row.selected;
            }
        }

        let MaterialDataTable {
            columns,
            rows,
            allow_selection,
            sticky_header: _,
            progress_visible,
            corner_radius,
            ..
        } = self;

        // Calculate table dimensions with dynamic row heights
        let checkbox_width = if allow_selection { 48.0 } else { 0.0 };
        let total_width = checkbox_width + columns.iter().map(|col| col.width).sum::<f32>();
        let min_row_height = 52.0;
        let header_height = 56.0;
        
        // Calculate individual row heights based on content
        let mut row_heights = Vec::new();
        for row in &rows {
            let mut max_height: f32 = min_row_height;
            for (cell_idx, cell_text) in row.cells.iter().enumerate() {
                if let Some(column) = columns.get(cell_idx) {
                    let available_width = column.width - 32.0;
                    let cell_font = FontId::new(14.0, FontFamily::Proportional);
                    
                    let galley = ui.fonts(|f| f.layout_job(egui::text::LayoutJob {
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
                        halign: if column.numeric { egui::Align::RIGHT } else { egui::Align::LEFT },
                        justify: false,
                        first_row_min_height: 0.0,
                        round_output_to_gui: true,
                    }));
                    
                    let content_height = galley.size().y + 16.0f32; // Add padding
                    max_height = max_height.max(content_height);
                }
            }
            row_heights.push(max_height);
        }
        
        let total_height = header_height + row_heights.iter().sum::<f32>();

        let desired_size = Vec2::new(total_width, total_height);
        let response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw table background
            ui.painter().rect_filled(rect, corner_radius, background_color);
            ui.painter().rect_stroke(rect, corner_radius, border_stroke, egui::epaint::StrokeKind::Outside);

            let mut current_y = rect.min.y;

            // Draw header
            let header_rect = Rect::from_min_size(rect.min, Vec2::new(total_width, header_height));
            let header_bg = get_global_color("surfaceVariant");
            ui.painter().rect_filled(header_rect, CornerRadius::ZERO, header_bg);
            
            let mut current_x = rect.min.x;
            
            // Header checkbox
            if allow_selection {
                let checkbox_rect = Rect::from_min_size(
                    egui::pos2(current_x, current_y),
                    Vec2::new(checkbox_width, header_height)
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
                    checkbox_color
                );
                ui.painter().rect_stroke(
                    checkbox_inner_rect,
                    CornerRadius::from(2.0),
                    Stroke::new(2.0, get_global_color("outline")),
                    egui::epaint::StrokeKind::Outside
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
                        Stroke::new(2.0, Color32::WHITE)
                    );
                    ui.painter().line_segment(
                        [check_points[1], check_points[2]],
                        Stroke::new(2.0, Color32::WHITE)
                    );
                }
                
                // Handle header checkbox click
                let header_checkbox_id = table_id.with("header_checkbox");
                let checkbox_response = ui.interact(checkbox_inner_rect, header_checkbox_id, Sense::click());
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
                    Vec2::new(column.width, header_height)
                );
                
                let text_pos = egui::pos2(
                    current_x + 16.0,
                    current_y + (header_height - ui.text_style_height(&egui::TextStyle::Body)) / 2.0
                );
                
                ui.painter().text(
                    text_pos,
                    egui::Align2::LEFT_TOP,
                    &column.title,
                    egui::TextStyle::Body.resolve(ui.style()),
                    get_global_color("onSurface")
                );
                
                // Handle column header clicks for sorting
                if column.sortable {
                    let header_click_id = table_id.with(format!("column_header_{}", col_idx));
                    let header_response = ui.interact(col_rect, header_click_id, Sense::click());
                    if header_response.clicked() {
                        ui.memory_mut(|mem| {
                            mem.data.insert_temp(table_id.with("column_clicked"), Some(col_idx));
                        });
                    }
                    
                    let icon_pos = egui::pos2(
                        current_x + column.width - 32.0,
                        current_y + (header_height - 24.0) / 2.0
                    );
                    let icon_rect = Rect::from_min_size(icon_pos, Vec2::splat(24.0));
                    
                    // Draw sort arrow (simplified)
                    let arrow_color = get_global_color("onSurfaceVariant");
                    let center = icon_rect.center();
                    let arrow_points = match column.sort_direction {
                        Some(SortDirection::Ascending) => [
                            center + Vec2::new(0.0, -4.0),
                            center + Vec2::new(-4.0, 4.0),
                            center + Vec2::new(4.0, 4.0),
                        ],
                        Some(SortDirection::Descending) => [
                            center + Vec2::new(0.0, 4.0),
                            center + Vec2::new(-4.0, -4.0),
                            center + Vec2::new(4.0, -4.0),
                        ],
                        None => [
                            center + Vec2::new(0.0, -4.0),
                            center + Vec2::new(-4.0, 4.0),
                            center + Vec2::new(4.0, 4.0),
                        ],
                    };
                    
                    ui.painter().line_segment([arrow_points[0], arrow_points[1]], Stroke::new(1.5, arrow_color));
                    ui.painter().line_segment([arrow_points[1], arrow_points[2]], Stroke::new(1.5, arrow_color));
                    ui.painter().line_segment([arrow_points[2], arrow_points[0]], Stroke::new(1.5, arrow_color));
                }
                
                current_x += column.width;
            }

            current_y += header_height;

            // Draw rows with dynamic heights
            for (row_idx, row) in rows.iter().enumerate() {
                let row_height = row_heights.get(row_idx).copied().unwrap_or(min_row_height);
                let row_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(total_width, row_height)
                );
                
                let row_selected = state.selected_rows.get(row_idx).copied().unwrap_or(false);
                let row_bg = if row_selected {
                    get_global_color("primaryContainer")
                } else if row.readonly {
                    // Subtle background for readonly rows
                    let surface_variant = get_global_color("surfaceVariant");
                    Color32::from_rgba_premultiplied(
                        surface_variant.r(),
                        surface_variant.g(), 
                        surface_variant.b(),
                        (surface_variant.a() as f32 * 0.3) as u8
                    )
                } else if row_idx % 2 == 1 {
                    get_global_color("surfaceVariant")
                } else {
                    background_color
                };
                
                ui.painter().rect_filled(row_rect, CornerRadius::ZERO, row_bg);
                
                current_x = rect.min.x;
                
                // Row checkbox
                if allow_selection {
                    let checkbox_rect = Rect::from_min_size(
                        egui::pos2(current_x, current_y),
                        Vec2::new(checkbox_width, row_height)
                    );
                    
                    let checkbox_center = checkbox_rect.center();
                    let checkbox_size = Vec2::splat(18.0);
                    let checkbox_inner_rect = Rect::from_center_size(checkbox_center, checkbox_size);
                    
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
                        checkbox_color
                    );
                    ui.painter().rect_stroke(
                        checkbox_inner_rect,
                        CornerRadius::from(2.0),
                        Stroke::new(2.0, border_color),
                        egui::epaint::StrokeKind::Outside
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
                            Stroke::new(2.0, Color32::WHITE)
                        );
                        ui.painter().line_segment(
                            [check_points[1], check_points[2]],
                            Stroke::new(2.0, Color32::WHITE)
                        );
                    }
                    
                    // Handle row checkbox click
                    let row_checkbox_id = table_id.with(format!("row_checkbox_{}", row_idx));
                    let checkbox_response = ui.interact(checkbox_inner_rect, row_checkbox_id, Sense::click());
                    if checkbox_response.clicked() && !row.readonly {
                        if let Some(selected) = state.selected_rows.get_mut(row_idx) {
                            *selected = !*selected;
                        }
                        
                        // Update header checkbox state based on row selections
                        // Only consider non-readonly rows for header checkbox state
                        let non_readonly_indices: Vec<usize> = rows.iter()
                            .enumerate()
                            .filter(|(_, row)| !row.readonly)
                            .map(|(idx, _)| idx)
                            .collect();
                        
                        if !non_readonly_indices.is_empty() {
                            let all_non_readonly_selected = non_readonly_indices.iter()
                                .all(|&idx| state.selected_rows.get(idx).copied().unwrap_or(false));
                            let none_non_readonly_selected = non_readonly_indices.iter()
                                .all(|&idx| !state.selected_rows.get(idx).copied().unwrap_or(false));
                            state.header_checkbox = all_non_readonly_selected && !none_non_readonly_selected;
                        }
                    }
                    
                    current_x += checkbox_width;
                }
                
                // Row cells
                for (cell_idx, cell_text) in row.cells.iter().enumerate() {
                    if let Some(column) = columns.get(cell_idx) {
                        let cell_rect = Rect::from_min_size(
                            egui::pos2(current_x, current_y),
                            Vec2::new(column.width, row_height)
                        );
                        
                        let text_align = if column.numeric {
                            egui::Align2::RIGHT_CENTER
                        } else {
                            egui::Align2::LEFT_CENTER
                        };
                        
                        // Handle text wrapping for cell content
                        let available_width = column.width - 32.0; // Account for padding
                        let cell_font = FontId::new(14.0, FontFamily::Proportional);
                        
                        let galley = ui.fonts(|f| f.layout_job(egui::text::LayoutJob {
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
                            halign: if column.numeric { egui::Align::RIGHT } else { egui::Align::LEFT },
                            justify: false,
                            first_row_min_height: 0.0,
                            round_output_to_gui: true,
                        }));
                        
                        let text_pos = if column.numeric {
                            egui::pos2(current_x + column.width - 16.0 - galley.size().x, current_y + (row_height - galley.size().y) / 2.0)
                        } else {
                            egui::pos2(current_x + 16.0, current_y + (row_height - galley.size().y) / 2.0)
                        };
                        
                        ui.painter().galley(text_pos, galley, get_global_color("onSurface"));
                        
                        current_x += column.width;
                    }
                }
                
                current_y += row_height;
            }

            // Draw progress indicator if visible
            if progress_visible {
                let scrim_color = Color32::from_rgba_unmultiplied(255, 255, 255, 128);
                ui.painter().rect_filled(rect, corner_radius, scrim_color);
                
                // Draw progress bar
                let progress_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, rect.min.y + header_height),
                    Vec2::new(total_width, 4.0)
                );
                
                let progress_color = get_global_color("primary");
                ui.painter().rect_filled(progress_rect, CornerRadius::ZERO, progress_color);
            }
        }

        // Persist the state
        ui.data_mut(|d| d.insert_persisted(table_id, state.clone()));

        // Check for column clicks using stored state
        let column_clicked = ui.memory(|mem| {
            mem.data.get_temp::<Option<usize>>(table_id.with("column_clicked"))
        }).flatten();
        
        // Clear the stored click state
        ui.memory_mut(|mem| {
            mem.data.remove::<Option<usize>>(table_id.with("column_clicked"));
        });
        
        DataTableResponse {
            response,
            selected_rows: state.selected_rows,
            header_checkbox: state.header_checkbox,
            column_clicked,
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