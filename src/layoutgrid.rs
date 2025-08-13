use egui::{
    ecolor::Color32, 
    epaint::CornerRadius,
    Rect, Response, Sense, Ui, Vec2, Widget,
};

/// Material Design layout grid component.
///
/// Layout grids provide structure and organize content across multiple screen sizes.
/// They help create consistent layouts following Material Design principles.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let grid = MaterialLayoutGrid::new()
///     .columns(12)
///     .gutter(16.0)
///     .margin(24.0)
///     .cell(4, |ui| { ui.label("Column 1-4"); })
///     .cell(4, |ui| { ui.label("Column 5-8"); })
///     .cell(4, |ui| { ui.label("Column 9-12"); });
///
/// ui.add(grid);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialLayoutGrid<'a> {
    cells: Vec<GridCell<'a>>,
    columns: usize,
    gutter: f32,
    margin: f32,
    max_width: Option<f32>,
    debug_mode: bool,
}

struct GridCell<'a> {
    span: usize,
    content: Box<dyn FnOnce(&mut Ui) -> Response + 'a>,
    offset: Option<usize>,
}

impl<'a> MaterialLayoutGrid<'a> {
    /// Create a new layout grid.
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            columns: 12, // Standard 12-column grid
            gutter: 16.0, // Standard gutter size
            margin: 24.0, // Standard margin
            max_width: None,
            debug_mode: false,
        }
    }

    /// Set the number of columns.
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    /// Set the gutter size (space between columns).
    pub fn gutter(mut self, gutter: f32) -> Self {
        self.gutter = gutter;
        self
    }

    /// Set the margin (space around the grid).
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    /// Set maximum width of the grid.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Enable debug mode to visualize grid structure.
    pub fn debug_mode(mut self, debug: bool) -> Self {
        self.debug_mode = debug;
        self
    }

    /// Add a cell that spans the specified number of columns.
    pub fn cell<F>(mut self, span: usize, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.cells.push(GridCell {
            span: span.clamp(1, self.columns),
            content: Box::new(move |ui| {
                content(ui);
                ui.allocate_response(Vec2::ZERO, Sense::hover())
            }),
            offset: None,
        });
        self
    }

    /// Add a cell with an offset (empty columns before this cell).
    pub fn cell_with_offset<F>(mut self, span: usize, offset: usize, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.cells.push(GridCell {
            span: span.clamp(1, self.columns),
            content: Box::new(move |ui| {
                content(ui);
                ui.allocate_response(Vec2::ZERO, Sense::hover())
            }),
            offset: Some(offset),
        });
        self
    }

    /// Add an empty cell (spacer).
    pub fn spacer(mut self, span: usize) -> Self {
        self.cells.push(GridCell {
            span: span.clamp(1, self.columns),
            content: Box::new(|ui| ui.allocate_response(Vec2::ZERO, Sense::hover())),
            offset: None,
        });
        self
    }

    fn calculate_column_width(&self, available_width: f32) -> f32 {
        let effective_width = if let Some(max_width) = self.max_width {
            available_width.min(max_width)
        } else {
            available_width
        };
        
        let total_gutter_width = (self.columns - 1) as f32 * self.gutter;
        let content_width = effective_width - 2.0 * self.margin - total_gutter_width;
        content_width / self.columns as f32
    }
}

impl<'a> Default for MaterialLayoutGrid<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialLayoutGrid<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_width = ui.available_width();
        let column_width = self.calculate_column_width(available_width);
        
        let MaterialLayoutGrid {
            cells,
            columns,
            gutter,
            margin,
            max_width,
            debug_mode,
        } = self;

        if cells.is_empty() {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        let effective_width = if let Some(max_width) = max_width {
            available_width.min(max_width)
        } else {
            available_width
        };
        
        // Start layout
        let start_pos = ui.next_widget_position();
        let mut current_row_y = start_pos.y + margin;
        let mut current_column = 0;
        let mut row_height: f32 = 0.0;
        let mut max_y = current_row_y;
        
        let mut responses = Vec::new();

        // Process each cell
        for cell in cells {
            // Handle offset
            if let Some(offset) = cell.offset {
                current_column += offset;
            }
            
            // Check if we need to wrap to next row
            if current_column + cell.span > columns {
                current_row_y = max_y + gutter;
                current_column = 0;
                row_height = 0.0;
            }
            
            // Calculate cell position and size
            let cell_x = start_pos.x + margin + current_column as f32 * (column_width + gutter);
            let cell_width = cell.span as f32 * column_width + (cell.span - 1) as f32 * gutter;
            
            // Create a constrained UI for this cell
            let cell_rect = Rect::from_min_size(
                egui::pos2(cell_x, current_row_y),
                Vec2::new(cell_width, ui.available_height())
            );
            
            let cell_response = ui.scope_builder(
                egui::UiBuilder::new().max_rect(cell_rect),
                |ui| {
                    // Debug visualization
                    if debug_mode {
                        let debug_color = Color32::from_rgba_unmultiplied(103, 80, 164, 30);
                        ui.painter().rect_filled(
                            cell_rect,
                            CornerRadius::from(2.0),
                            debug_color
                        );
                    }
                    
                    (cell.content)(ui)
                }
            );
            
            let cell_height = cell_response.response.rect.height();
            row_height = row_height.max(cell_height);
            max_y = max_y.max(current_row_y + row_height);
            
            responses.push(cell_response.response);
            current_column += cell.span;
        }
        
        // Calculate total grid size
        let total_height = max_y - start_pos.y + margin;
        let grid_rect = Rect::from_min_size(start_pos, Vec2::new(effective_width, total_height));
        
        // Debug: Draw grid outline
        if debug_mode {
            let outline_color = Color32::from_rgb(103, 80, 164);
            ui.painter().rect_stroke(
                grid_rect,
                CornerRadius::from(4.0),
                egui::epaint::Stroke::new(1.0, outline_color),
                egui::epaint::StrokeKind::Outside
            );
            
            // Draw column guides
            for i in 0..=columns {
                let x = start_pos.x + margin + i as f32 * (column_width + gutter) - gutter / 2.0;
                if i > 0 && i < columns {
                    ui.painter().line_segment(
                        [
                            egui::pos2(x, start_pos.y + margin),
                            egui::pos2(x, max_y)
                        ],
                        egui::epaint::Stroke::new(0.5, Color32::from_rgb(200, 200, 200))
                    );
                }
            }
        }
        
        // Allocate the full grid space
        let (grid_response_rect, mut grid_response) = ui.allocate_at_least(
            Vec2::new(effective_width, total_height),
            Sense::hover()
        );
        
        // Union all cell responses
        for response in responses {
            grid_response = grid_response.union(response);
        }
        
        grid_response
    }
}

/// Convenience function to create a new layout grid.
pub fn layout_grid() -> MaterialLayoutGrid<'static> {
    MaterialLayoutGrid::new()
}

/// Convenience function to create a layout grid with debug mode enabled.
pub fn debug_layout_grid() -> MaterialLayoutGrid<'static> {
    MaterialLayoutGrid::new().debug_mode(true)
}