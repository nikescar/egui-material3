use crate::theme::get_global_color;
use egui::{epaint::CornerRadius, Color32, Rect, Response, Sense, Ui, Vec2, Widget};

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
            columns: 12,  // Standard 12-column grid
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
                Vec2::new(cell_width, ui.available_height()),
            );

            let cell_response =
                ui.scope_builder(egui::UiBuilder::new().max_rect(cell_rect), |ui| {
                    // Debug visualization
                    if debug_mode {
                        let debug_color = get_global_color("primary").linear_multiply(0.12);
                        ui.painter()
                            .rect_filled(cell_rect, CornerRadius::from(2.0), debug_color);
                    }

                    (cell.content)(ui)
                });

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
            let outline_color = get_global_color("primary");
            ui.painter().rect_stroke(
                grid_rect,
                CornerRadius::from(4.0),
                egui::epaint::Stroke::new(1.0, outline_color),
                egui::epaint::StrokeKind::Outside,
            );

            // Draw column guides
            for i in 0..=columns {
                let x = start_pos.x + margin + i as f32 * (column_width + gutter) - gutter / 2.0;
                if i > 0 && i < columns {
                    ui.painter().line_segment(
                        [egui::pos2(x, start_pos.y + margin), egui::pos2(x, max_y)],
                        egui::epaint::Stroke::new(0.5, get_global_color("outlineVariant")),
                    );
                }
            }
        }

        // Allocate the full grid space
        let (_grid_response_rect, mut grid_response) =
            ui.allocate_at_least(Vec2::new(effective_width, total_height), Sense::hover());

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

/// Material Design grid tile bar.
///
/// A header or footer bar typically used with [GridTile].
/// Contains optional leading/trailing icons and title/subtitle text.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let tile_bar = GridTileBar::new()
///     .title("Image Title")
///     .subtitle("Subtitle text")
///     .background_color(egui::Color32::from_black_alpha(128));
///
/// ui.add(tile_bar);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct GridTileBar<'a> {
    background_color: Option<Color32>,
    leading: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    title: Option<String>,
    subtitle: Option<String>,
    trailing: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

impl<'a> GridTileBar<'a> {
    /// Create a new grid tile bar.
    pub fn new() -> Self {
        Self {
            background_color: None,
            leading: None,
            title: None,
            subtitle: None,
            trailing: None,
        }
    }

    /// Set the background color.
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set a leading widget (left side icon/widget).
    pub fn leading<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.leading = Some(Box::new(content));
        self
    }

    /// Set the title text.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the subtitle text.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set a trailing widget (right side icon/widget).
    pub fn trailing<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.trailing = Some(Box::new(content));
        self
    }
}

impl<'a> Default for GridTileBar<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for GridTileBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let GridTileBar {
            background_color,
            leading,
            title,
            subtitle,
            trailing,
        } = self;

        // Calculate height based on content
        let height = if title.is_some() && subtitle.is_some() {
            68.0
        } else {
            48.0
        };

        // Calculate padding
        let padding_start = if leading.is_some() { 8.0 } else { 16.0 };
        let padding_end = if trailing.is_some() { 8.0 } else { 16.0 };

        let available_width = ui.available_width();
        let start_pos = ui.next_widget_position();

        // Draw background if specified
        if let Some(bg_color) = background_color {
            let bg_rect = Rect::from_min_size(start_pos, Vec2::new(available_width, height));
            ui.painter().rect_filled(bg_rect, CornerRadius::ZERO, bg_color);
        }

        let _response = ui.horizontal(|ui| {
            ui.add_space(padding_start);

            // Leading widget
            if let Some(leading_fn) = leading {
                leading_fn(ui);
                ui.add_space(8.0);
            }

            // Title and subtitle
            if title.is_some() || subtitle.is_some() {
                ui.vertical(|ui| {
                    ui.set_min_height(height);
                    ui.add_space((height - if subtitle.is_some() { 40.0 } else { 20.0 }) / 2.0);

                    if let Some(title_text) = &title {
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                        ui.label(egui::RichText::new(title_text).color(Color32::WHITE));
                    }

                    if let Some(subtitle_text) = &subtitle {
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Small);
                        ui.label(egui::RichText::new(subtitle_text).color(Color32::WHITE));
                    }
                });
            }

            // Trailing widget
            if let Some(trailing_fn) = trailing {
                ui.add_space(8.0);
                trailing_fn(ui);
            }

            ui.add_space(padding_end);
        });

        ui.allocate_rect(
            Rect::from_min_size(start_pos, Vec2::new(available_width, height)),
            Sense::hover(),
        )
    }
}

/// Material Design grid tile.
///
/// A tile in a grid list with optional header and footer overlays.
/// Based on Flutter's GridTile component.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let tile = GridTile::new(|ui| {
///     ui.label("Tile content");
/// })
/// .footer(
///     GridTileBar::new()
///         .title("Image Title")
///         .background_color(egui::Color32::from_black_alpha(128))
/// );
///
/// ui.add(tile);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct GridTile<'a> {
    header: Option<GridTileBar<'a>>,
    footer: Option<GridTileBar<'a>>,
    child: Box<dyn FnOnce(&mut Ui) + 'a>,
    min_height: f32,
}

impl<'a> GridTile<'a> {
    /// Create a new grid tile with the given content.
    pub fn new<F>(content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        Self {
            header: None,
            footer: None,
            child: Box::new(content),
            min_height: 100.0,
        }
    }

    /// Set the header bar (shown at the top).
    pub fn header(mut self, header: GridTileBar<'a>) -> Self {
        self.header = Some(header);
        self
    }

    /// Set the footer bar (shown at the bottom).
    pub fn footer(mut self, footer: GridTileBar<'a>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the minimum height of the tile.
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = height;
        self
    }
}

impl Widget for GridTile<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let GridTile {
            header,
            footer,
            child,
            min_height,
        } = self;

        if header.is_none() && footer.is_none() {
            // Simple case: no overlays
            let response = ui.vertical(|ui| {
                ui.set_min_height(min_height);
                child(ui);
            });
            return response.response;
        }

        // Complex case: with header/footer overlays
        let available_width = ui.available_width();
        let start_pos = ui.next_widget_position();

        // First, render the child to get the content height
        let child_response = ui.vertical(|ui| {
            ui.set_min_height(min_height);
            child(ui);
        });

        let content_height = child_response.response.rect.height().max(min_height);
        let tile_rect = Rect::from_min_size(start_pos, Vec2::new(available_width, content_height));

        // Draw header overlay if present
        if let Some(header_bar) = header {
            let header_ui = &mut ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(Rect::from_min_size(start_pos, Vec2::new(available_width, 68.0)))
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            header_ui.add(header_bar);
        }

        // Draw footer overlay if present
        if let Some(footer_bar) = footer {
            let footer_height = 68.0; // Will be adjusted by the GridTileBar itself
            let footer_pos = egui::pos2(start_pos.x, start_pos.y + content_height - footer_height);
            let footer_ui = &mut ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(Rect::from_min_size(footer_pos, Vec2::new(available_width, footer_height)))
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            footer_ui.add(footer_bar);
        }

        ui.allocate_rect(tile_rect, Sense::hover())
    }
}
