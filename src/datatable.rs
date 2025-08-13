use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget, WidgetText,
};

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
    selected_rows: Vec<bool>,
    header_checkbox: bool,
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
    id: Option<String>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> DataTableRow<'a> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            selected: false,
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
            selected_rows: Vec::new(),
            header_checkbox: false,
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
            sortable: false,
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

    /// Enable row selection.
    pub fn allow_selection(mut self, allow: bool) -> Self {
        self.allow_selection = allow;
        if allow && self.selected_rows.len() != self.rows.len() {
            self.selected_rows = vec![false; self.rows.len()];
        }
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
        let md_surface = Color32::from_rgb(254, 247, 255);
        let md_outline = Color32::from_rgb(196, 199, 197);
        (md_surface, Stroke::new(1.0, md_outline))
    }
}

impl<'a> Default for MaterialDataTable<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialDataTable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (background_color, border_stroke) = self.get_table_style();
        
        let MaterialDataTable {
            columns,
            rows,
            mut selected_rows,
            mut header_checkbox,
            allow_selection,
            sticky_header: _,
            progress_visible,
            corner_radius,
        } = self;

        // Calculate table dimensions
        let checkbox_width = if allow_selection { 48.0 } else { 0.0 };
        let total_width = checkbox_width + columns.iter().map(|col| col.width).sum::<f32>();
        let row_height = 52.0;
        let header_height = 56.0;
        let total_height = header_height + (rows.len() as f32 * row_height);

        let desired_size = Vec2::new(total_width, total_height);
        let mut response = ui.allocate_response(desired_size, Sense::click());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw table background
            ui.painter().rect_filled(rect, corner_radius, background_color);
            ui.painter().rect_stroke(rect, corner_radius, border_stroke, egui::epaint::StrokeKind::Outside);

            let mut current_y = rect.min.y;

            // Draw header
            let header_rect = Rect::from_min_size(rect.min, Vec2::new(total_width, header_height));
            let header_bg = Color32::from_rgb(245, 245, 245);
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
                
                let checkbox_color = if header_checkbox {
                    Color32::from_rgb(103, 80, 164)
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
                    Stroke::new(2.0, Color32::from_rgb(121, 116, 126)),
                    egui::epaint::StrokeKind::Outside
                );
                
                if header_checkbox {
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
                let checkbox_response = ui.interact(checkbox_inner_rect, ui.next_auto_id(), Sense::click());
                if checkbox_response.clicked() {
                    header_checkbox = !header_checkbox;
                    for selected in &mut selected_rows {
                        *selected = header_checkbox;
                    }
                }
                
                current_x += checkbox_width;
            }
            
            // Header columns
            for column in &columns {
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
                    Color32::from_rgb(28, 27, 31)
                );
                
                // Draw sort icon if sortable
                if column.sortable {
                    let icon_pos = egui::pos2(
                        current_x + column.width - 32.0,
                        current_y + (header_height - 24.0) / 2.0
                    );
                    let icon_rect = Rect::from_min_size(icon_pos, Vec2::splat(24.0));
                    
                    // Draw sort arrow (simplified)
                    let arrow_color = Color32::from_rgb(121, 116, 126);
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

            // Draw rows
            for (row_idx, row) in rows.iter().enumerate() {
                let row_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(total_width, row_height)
                );
                
                let row_selected = selected_rows.get(row_idx).copied().unwrap_or(false);
                let row_bg = if row_selected {
                    Color32::from_rgb(230, 224, 233)
                } else if row_idx % 2 == 1 {
                    Color32::from_rgb(251, 251, 251)
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
                        Color32::from_rgb(103, 80, 164)
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
                        Stroke::new(2.0, Color32::from_rgb(121, 116, 126)),
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
                    let checkbox_response = ui.interact(checkbox_inner_rect, ui.next_auto_id(), Sense::click());
                    if checkbox_response.clicked() {
                        if let Some(selected) = selected_rows.get_mut(row_idx) {
                            *selected = !*selected;
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
                        
                        let text_pos = if column.numeric {
                            egui::pos2(current_x + column.width - 16.0, current_y + row_height / 2.0)
                        } else {
                            egui::pos2(current_x + 16.0, current_y + row_height / 2.0)
                        };
                        
                        ui.painter().text(
                            text_pos,
                            text_align,
                            cell_text.text(),
                            egui::TextStyle::Body.resolve(ui.style()),
                            Color32::from_rgb(28, 27, 31)
                        );
                        
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
                
                let progress_color = Color32::from_rgb(103, 80, 164);
                ui.painter().rect_filled(progress_rect, CornerRadius::ZERO, progress_color);
            }
        }

        response
    }
}

/// Convenience function to create a new data table.
pub fn data_table() -> MaterialDataTable<'static> {
    MaterialDataTable::new()
}