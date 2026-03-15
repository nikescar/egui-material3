//! Material Design Spreadsheet Component
//!
//! A spreadsheet widget with DataFusion backend for data storage and manipulation.
//! Supports importing/exporting CSV, Excel, Parquet, and Arrow formats.

#[cfg(feature = "spreadsheet")]
use crate::theme::get_global_color;
#[cfg(feature = "spreadsheet")]
use std::path::PathBuf;
#[cfg(feature = "spreadsheet")]
use std::sync::Arc;
#[cfg(feature = "spreadsheet")]
use tokio::sync::Mutex;

#[cfg(feature = "spreadsheet")]
use datafusion::prelude::*;
#[cfg(feature = "spreadsheet")]
use datafusion::arrow::array::{ArrayRef, RecordBatch, StringArray};
#[cfg(feature = "spreadsheet")]
use datafusion::arrow::datatypes::{DataType, Field, Schema};
#[cfg(feature = "spreadsheet")]
use egui::{Id, Response, Sense, TextEdit, Ui, Widget};
#[cfg(feature = "spreadsheet")]
use egui_async::{Bind, StateWithData};
#[cfg(feature = "spreadsheet")]
use std::sync::Arc as StdArc;

// Re-export for convenience
#[cfg(feature = "spreadsheet")]
pub use egui_extras::{Column, TableBuilder};

/// Column definition for spreadsheet
#[cfg(feature = "spreadsheet")]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub col_type: ColumnType,
    pub width: f32,
}

/// Supported column data types
#[cfg(feature = "spreadsheet")]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ColumnType {
    Text,
    Integer,
    Real,
    Boolean,
}

#[cfg(feature = "spreadsheet")]
impl ColumnType {
    fn to_arrow(&self) -> DataType {
        match self {
            ColumnType::Text => DataType::Utf8,
            ColumnType::Integer => DataType::Int64,
            ColumnType::Real => DataType::Float64,
            ColumnType::Boolean => DataType::Boolean,
        }
    }
}

/// A single row of data
#[cfg(feature = "spreadsheet")]
#[derive(Clone, Debug)]
pub struct RowData {
    pub id: usize,
    pub values: Vec<String>,
}

/// File format for import/export
#[cfg(feature = "spreadsheet")]
#[derive(Clone, Debug, PartialEq)]
pub enum FileFormat {
    Csv,
    Excel,
    Parquet,
    Arrow,
}

#[cfg(feature = "spreadsheet")]
impl FileFormat {
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        path.extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
            "csv" => Some(FileFormat::Csv),
            "xls" | "xlsx" => Some(FileFormat::Excel),
            "parquet" => Some(FileFormat::Parquet),
            "arrow" => Some(FileFormat::Arrow),
            _ => None,
        })
    }
}

/// DataFusion-backed data model for spreadsheet
#[cfg(feature = "spreadsheet")]
pub struct SpreadsheetDataModel {
    ctx: SessionContext,
    columns: Vec<ColumnDef>,
    data: Vec<Vec<String>>, // In-memory storage for modifications
    row_count: usize,
}

#[cfg(feature = "spreadsheet")]
impl SpreadsheetDataModel {
    /// Create a new in-memory spreadsheet data model
    pub fn new(columns: Vec<ColumnDef>) -> Result<Self, String> {
        let ctx = SessionContext::new();

        let model = Self {
            ctx,
            columns: columns.clone(),
            data: Vec::new(),
            row_count: 0,
        };

        Ok(model)
    }

    fn data_to_record_batch(&self) -> Result<RecordBatch, String> {
        // Build Arrow schema from columns (use column names instead of col0, col1, etc.)
        let mut fields = vec![];
        for col in self.columns.iter() {
            fields.push(Field::new(&col.name, col.col_type.to_arrow(), true));
        }
        let schema = StdArc::new(Schema::new(fields));

        if self.data.is_empty() {
            return RecordBatch::try_new(schema, vec![])
                .map_err(|e| format!("Failed to create empty batch: {}", e));
        }

        // Create data columns (all as strings for now)
        let mut columns: Vec<ArrayRef> = vec![];
        for col_idx in 0..self.columns.len() {
            let values: Vec<String> = self.data.iter()
                .map(|row| row.get(col_idx).cloned().unwrap_or_default())
                .collect();
            columns.push(StdArc::new(StringArray::from(values)));
        }

        RecordBatch::try_new(schema, columns)
            .map_err(|e| format!("Failed to create batch: {}", e))
    }

    /// Insert multiple rows
    pub fn insert_rows(&mut self, rows: Vec<Vec<String>>) -> Result<(), String> {
        for row_values in rows {
            self.insert_row(row_values)?;
        }
        Ok(())
    }

    /// Insert a single row
    pub fn insert_row(&mut self, values: Vec<String>) -> Result<(), String> {
        self.data.push(values);
        self.row_count += 1;
        Ok(())
    }

    /// Query all rows (returns data directly from memory)
    pub fn query_rows(&self) -> Result<Vec<RowData>, String> {
        let mut result = Vec::new();
        for (id, row) in self.data.iter().enumerate() {
            result.push(RowData {
                id,
                values: row.clone(),
            });
        }
        Ok(result)
    }

    /// Update a single cell
    pub fn update_cell(&mut self, row_id: usize, col_idx: usize, value: String) -> Result<(), String> {
        // Validate value against column type
        if col_idx < self.columns.len() {
            let col_type = &self.columns[col_idx].col_type;

            // For numeric types, validate that the value can be parsed
            match col_type {
                ColumnType::Integer => {
                    if !value.is_empty() && value.parse::<i64>().is_err() {
                        return Err(format!("'{}' is not a valid integer", value));
                    }
                }
                ColumnType::Real => {
                    if !value.is_empty() && value.parse::<f64>().is_err() {
                        return Err(format!("'{}' is not a valid number", value));
                    }
                }
                ColumnType::Boolean => {
                    if !value.is_empty() && value.parse::<bool>().is_err() {
                        // Accept common boolean representations
                        let lower = value.to_lowercase();
                        if lower != "true" && lower != "false" && lower != "1" && lower != "0" {
                            return Err(format!("'{}' is not a valid boolean (use true/false or 1/0)", value));
                        }
                    }
                }
                ColumnType::Text => {} // Text accepts anything
            }
        }

        // Update in memory
        if row_id < self.data.len() && col_idx < self.columns.len() {
            self.data[row_id][col_idx] = value;
            Ok(())
        } else {
            Err("Invalid row or column index".to_string())
        }
    }

    /// Delete a row
    pub fn delete_row(&mut self, row_id: usize) -> Result<(), String> {
        if row_id < self.data.len() {
            self.data.remove(row_id);
            Ok(())
        } else {
            Err("Invalid row index".to_string())
        }
    }

    /// Export to CSV
    pub fn export_csv(&self, path: &std::path::Path) -> Result<(), String> {
        use std::fs::File;
        use std::io::Write;

        let rows = self.query_rows().map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;

        // Write header
        let header: Vec<String> = self.columns.iter().map(|c| c.name.clone()).collect();
        writeln!(file, "{}", header.join(",")).map_err(|e| e.to_string())?;

        // Write rows
        for row in rows {
            writeln!(file, "{}", row.values.join(",")).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Import from CSV
    pub fn import_csv(&mut self, path: &std::path::Path) -> Result<(), String> {
        use std::fs::File;
        use std::io::BufRead;

        let file = File::open(path).map_err(|e| format!("Cannot open file: {}", e))?;
        let reader = std::io::BufReader::new(file);
        let all_lines: Vec<String> = reader.lines()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        if all_lines.is_empty() {
            return Err("CSV file is empty".to_string());
        }
        
        if all_lines.len() < 2 {
            return Err("CSV file has only one line".to_string());
        }

        let first_line = &all_lines[0];
        let second_line = &all_lines[1];
        let last_line = all_lines.last().unwrap();
        
        // Detect delimiter by comparing counts in first, second, and last lines
        let delimiters = [',', ';', '\t'];
        let mut best_delimiter = ',';
        let mut best_score = 0;
        
        for &delim in &delimiters {
            let count1 = first_line.matches(delim).count();
            let count2 = second_line.matches(delim).count();
            let count_last = last_line.matches(delim).count();
            
            // Score based on consistency across lines and total count
            if count1 > 0 && count1 == count2 && count2 == count_last {
                // Perfect consistency
                let score = count1 * 100;
                if score > best_score {
                    best_score = score;
                    best_delimiter = delim;
                }
            } else if count1 > 0 && count2 > 0 {
                // Partial consistency - prefer if counts are similar
                let min_count = count1.min(count2).min(count_last);
                let max_count = count1.max(count2).max(count_last);
                if max_count > 0 && min_count > 0 {
                    let score = (min_count * 50) / max_count;
                    if score > best_score {
                        best_score = score;
                        best_delimiter = delim;
                    }
                }
            }
        }
        
        let delimiter = best_delimiter;
        let delimiter_name = match delimiter {
            ',' => "comma",
            ';' => "semicolon",
            '\t' => "tab",
            _ => "unknown",
        };
        
        let first_values: Vec<&str> = first_line.split(delimiter).collect();
        let second_values: Vec<&str> = second_line.split(delimiter).collect();
        
        let col_count = first_values.len();

        // Determine if first line is a header (improved heuristic)
        // Check multiple conditions:
        // 1. First line values look like column names (short, non-numeric, no special chars)
        // 2. Type difference between first and second line
        // 3. First line has unique values (columns should have unique names)
        let looks_like_header = first_values.iter().all(|v| {
            let trimmed = v.trim();
            // Column names are typically short and don't start with numbers
            trimmed.len() < 50 &&
            !trimmed.is_empty() &&
            trimmed.parse::<f64>().is_err() && // Not purely numeric
            !trimmed.contains(|c: char| c.is_numeric() && trimmed.len() > 20) // Not long with numbers
        });

        let has_type_difference = first_values.iter().zip(second_values.iter()).any(|(v1, v2)| {
            let v1_is_num = v1.trim().parse::<f64>().is_ok();
            let v2_is_num = v2.trim().parse::<f64>().is_ok();
            v1_is_num != v2_is_num
        });

        let has_unique_values = {
            let mut seen = std::collections::HashSet::new();
            first_values.iter().all(|v| seen.insert(v.trim()))
        };

        // First line is header if it looks like header OR has type difference
        let first_line_is_header = looks_like_header || has_type_difference || (has_unique_values && looks_like_header);
        
        // Create new column definitions
        let new_columns: Vec<ColumnDef> = if first_line_is_header {
            first_values.iter().enumerate().map(|(_i, name)| {
                ColumnDef {
                    name: name.trim().to_string(),
                    col_type: ColumnType::Text,
                    width: 100.0,
                }
            }).collect()
        } else {
            (0..col_count).map(|i| {
                ColumnDef {
                    name: format!("column{}", i + 1),
                    col_type: ColumnType::Text,
                    width: 100.0,
                }
            }).collect()
        };
        
        eprintln!("Detected {} columns with {} delimiter", col_count, delimiter_name);
        eprintln!("First line is header: {}", first_line_is_header);
        
        // Recreate table with new columns
        self.columns = new_columns;
        self.data.clear();
        self.row_count = 0;
        
        // Prepare data rows
        let start_idx = if first_line_is_header { 1 } else { 0 };
        let data_lines: Vec<&String> = all_lines.iter()
            .skip(start_idx)
            .filter(|line| !line.trim().is_empty())
            .collect();
        
        // Insert data rows
        for (idx, line) in data_lines.iter().enumerate() {
            let values: Vec<String> = line.split(delimiter).map(|s| s.trim().to_string()).collect();
            
            // Validate column count
            if values.len() != col_count {
                return Err(format!(
                    "CSV row {} has {} columns, but expected {} columns",
                    idx + if first_line_is_header { 2 } else { 1 },
                    values.len(),
                    col_count
                ));
            }
            
            self.insert_row(values)
                .map_err(|e| format!("Failed to insert row {}: {}", idx + 1, e))?;
        }

        eprintln!("Successfully imported {} rows from CSV", data_lines.len());
        Ok(())
    }

    /// Export to Parquet using Arrow
    pub async fn export_parquet(&self, path: &std::path::Path) -> Result<(), String> {
        use datafusion::parquet::arrow::ArrowWriter;
        use std::fs::File;

        let batch = self.data_to_record_batch()?;
        let file = File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;

        let mut writer = ArrowWriter::try_new(file, batch.schema(), None)
            .map_err(|e| format!("Failed to create parquet writer: {}", e))?;

        writer.write(&batch)
            .map_err(|e| format!("Failed to write batch: {}", e))?;

        writer.close()
            .map_err(|e| format!("Failed to close writer: {}", e))?;

        Ok(())
    }

    /// Import from Parquet using DataFusion SQL
    pub async fn import_parquet(&mut self, path: &std::path::Path) -> Result<(), String> {
        // Clear existing data
        self.data.clear();
        self.row_count = 0;

        // Register parquet file with DataFusion
        let table_name = "imported_data";
        self.ctx.register_parquet(
            table_name,
            path.to_str().unwrap(),
            ParquetReadOptions::default(),
        )
        .await
        .map_err(|e| format!("Failed to register parquet: {}", e))?;

        // Query all data using SQL
        let df = self.ctx
            .sql(&format!("SELECT * FROM {}", table_name))
            .await
            .map_err(|e| format!("Failed to query parquet: {}", e))?;

        // Get schema and update columns
        let schema = df.schema();
        let mut new_columns = vec![];
        for field in schema.fields() {
            let col_type = match field.data_type() {
                DataType::Int64 | DataType::Int32 | DataType::Int16 | DataType::Int8 => ColumnType::Integer,
                DataType::Float64 | DataType::Float32 => ColumnType::Real,
                DataType::Boolean => ColumnType::Boolean,
                _ => ColumnType::Text,
            };
            new_columns.push(ColumnDef {
                name: field.name().clone(),
                col_type,
                width: 100.0,
            });
        }
        self.columns = new_columns;

        // Collect batches
        let batches = df.collect()
            .await
            .map_err(|e| format!("Failed to collect batches: {}", e))?;

        if batches.is_empty() {
            return Ok(());
        }

        // Process each batch
        for batch in batches {
            let num_rows = batch.num_rows();

            for row_idx in 0..num_rows {
                let mut row_values = Vec::new();

                // Get all data columns
                for col_idx in 0..batch.num_columns() {
                    let column = batch.column(col_idx);
                    let value = datafusion::arrow::util::display::array_value_to_string(column, row_idx)
                        .map_err(|e| format!("Failed to convert value: {}", e))?;
                    row_values.push(value);
                }

                self.insert_row(row_values)?;
            }
        }

        // Deregister table to clean up
        let _ = self.ctx.deregister_table(table_name);

        Ok(())
    }
}

/// Actions that can be performed on spreadsheet
#[cfg(feature = "spreadsheet")]
#[derive(Debug, Clone)]
pub enum SpreadsheetAction {
    CellEdited { row_id: usize, col_idx: usize, value: String },
    RowAdded,
    RowDeleted(usize),
    DataLoaded(PathBuf),
    DataSaved(PathBuf),
}

/// Material Design Spreadsheet widget
#[cfg(feature = "spreadsheet")]
pub struct MaterialSpreadsheet {
    id: Id,
    pub data_model: Arc<Mutex<SpreadsheetDataModel>>,
    cached_rows: Vec<RowData>,
    editing_cell: Option<(usize, usize)>,
    edit_buffer: String,
    allow_editing: bool,
    allow_selection: bool,
    striped: bool,
    row_height: f32,
    load_bind: Bind<Vec<RowData>, String>,
    save_bind: Bind<(), String>,
    load_processed: bool, // Track if we've processed the load result
}

#[cfg(feature = "spreadsheet")]
impl MaterialSpreadsheet {
    /// Create a new spreadsheet with the given columns
    pub fn new(id: &str, columns: Vec<ColumnDef>) -> Result<Self, String> {
        let data_model = SpreadsheetDataModel::new(columns).map_err(|e| e.to_string())?;

        Ok(Self {
            id: Id::new(id),
            data_model: Arc::new(Mutex::new(data_model)),
            cached_rows: Vec::new(),
            editing_cell: None,
            edit_buffer: String::new(),
            allow_editing: true,
            allow_selection: true,
            striped: true,
            row_height: 36.0,
            load_bind: Bind::new(false),
            save_bind: Bind::new(false),
            load_processed: false,
        })
    }

    /// Initialize spreadsheet with data (sync method for use in constructors)
    /// This is a convenience method that doesn't require an async context
    pub fn init_with_data(&mut self, rows: Vec<Vec<String>>) {
        // Use try_lock in a loop to avoid needing a runtime
        loop {
            if let Ok(mut model) = self.data_model.try_lock() {
                for row in rows {
                    let _ = model.insert_row(row);
                }
                self.cached_rows = model.query_rows().unwrap_or_default();
                break;
            }
            // Brief sleep to avoid busy waiting
            std::thread::sleep(std::time::Duration::from_micros(10));
        }
    }

    /// Set whether cells can be edited
    pub fn allow_editing(mut self, allow: bool) -> Self {
        self.allow_editing = allow;
        self
    }

    /// Set whether rows can be selected
    pub fn allow_selection(mut self, allow: bool) -> Self {
        self.allow_selection = allow;
        self
    }

    /// Set striped rows
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Set whether cells can be edited (mutable setter)
    pub fn set_allow_editing(&mut self, allow: bool) {
        self.allow_editing = allow;
    }

    /// Set whether rows can be selected (mutable setter)
    pub fn set_allow_selection(&mut self, allow: bool) {
        self.allow_selection = allow;
    }

    /// Set striped rows (mutable setter)
    pub fn set_striped(&mut self, striped: bool) {
        self.striped = striped;
    }

    /// Add a new empty row
    pub async fn add_row(&mut self) -> Result<(), String> {
        let mut model = self.data_model.lock().await;
        let col_count = model.columns.len();
        let empty_values = vec![String::new(); col_count];
        model.insert_row(empty_values).map_err(|e| e.to_string())?;
        // Refresh cached rows to show the new row
        self.cached_rows = model.query_rows().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Delete a row by ID
    pub async fn delete_row(&mut self, row_id: usize) -> Result<(), String> {
        let mut model = self.data_model.lock().await;
        model.delete_row(row_id).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Refresh cached data from database
    pub async fn refresh_data(&mut self) -> Result<(), String> {
        let model = self.data_model.lock().await;
        self.cached_rows = model.query_rows().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Load data from file (async)
    pub fn load_from_file(&mut self, path: PathBuf) {
        self.load_processed = false; // Reset flag for new load
        let model = Arc::clone(&self.data_model);
        self.load_bind.refresh(async move {
            let format = FileFormat::from_path(&path).ok_or_else(|| "Unknown file format".to_string())?;

            match format {
                FileFormat::Csv => {
                    let mut locked_model = model.lock().await;
                    locked_model.import_csv(&path)?;
                    locked_model.query_rows()
                }
                FileFormat::Parquet => {
                    // Import parquet asynchronously
                    let mut locked_model = model.lock().await;
                    locked_model.import_parquet(&path).await?;
                    locked_model.query_rows()
                }
                FileFormat::Excel => Err("Excel import not yet implemented".to_string()),
                FileFormat::Arrow => Err("Arrow import not yet implemented".to_string()),
            }
        });
    }

    /// Save data to file (async)
    pub fn save_to_file(&mut self, path: PathBuf) {
        let model = Arc::clone(&self.data_model);
        self.save_bind.refresh(async move {
            let format = FileFormat::from_path(&path).ok_or_else(|| "Unknown file format".to_string())?;

            match format {
                FileFormat::Csv => {
                    let locked_model = model.lock().await;
                    locked_model.export_csv(&path)?;
                    Ok(())
                }
                FileFormat::Parquet => {
                    let locked_model = model.lock().await;
                    locked_model.export_parquet(&path).await?;
                    Ok(())
                }
                FileFormat::Excel => Err("Excel export not yet implemented".to_string()),
                FileFormat::Arrow => Err("Arrow export not yet implemented".to_string()),
            }
        });
    }

    /// Get the column definitions (blocking version for sync context)
    pub fn columns(&self) -> Vec<ColumnDef> {
        // Use try_lock to avoid needing a runtime
        loop {
            if let Ok(model) = self.data_model.try_lock() {
                return model.columns.clone();
            }
            std::thread::sleep(std::time::Duration::from_micros(10));
        }
    }

    /// Get the current rows
    pub fn rows(&self) -> Vec<RowData> {
        self.cached_rows.clone()
    }

    /// Show the spreadsheet UI (alternative to Widget trait)
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // Register egui-async plugin (requires egui 0.33+)
        ui.ctx().plugin_or_default::<egui_async::EguiAsyncPlugin>();

        // Handle async load state
        match self.load_bind.state() {
            StateWithData::Pending => {
                ui.ctx().request_repaint();
                return ui.label("Loading...").interact(Sense::hover());
            }
            StateWithData::Finished(rows) => {
                // Only update cached_rows once when load completes
                if !self.load_processed {
                    eprintln!("DEBUG: Load finished, updating cached_rows with {} rows", rows.len());
                    self.cached_rows = rows.clone();
                    self.load_processed = true;
                }
            }
            StateWithData::Failed(err) => {
                return ui.label(format!("Load error: {}", err)).interact(Sense::hover());
            }
            StateWithData::Idle => {}
        }

        // Handle async save state
        match self.save_bind.state() {
            StateWithData::Pending => {
                ui.ctx().request_repaint();
                ui.label("Saving...");
            }
            StateWithData::Finished(_) => {
                ui.label("✓ Save completed successfully");
            }
            StateWithData::Failed(err) => {
                ui.colored_label(egui::Color32::RED, format!("Save error: {}", err));
            }
            StateWithData::Idle => {}
        }

        // Get column definitions (using try_lock for sync UI context)
        let columns = loop {
            if let Ok(model) = self.data_model.try_lock() {
                break model.columns.clone();
            }
            std::thread::sleep(std::time::Duration::from_micros(10));
        };

        // Get theme colors
        let on_surface = get_global_color("on-surface");
        let surface_variant = get_global_color("surface-variant");

        // Build table
        let available_height = ui.available_height();

        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        // Add columns
        for col in columns.iter() {
            table = table.column(Column::initial(col.width).at_least(50.0).resizable(true));
        }

        // Clone cached rows for rendering to avoid borrow issues
        let display_rows = self.cached_rows.clone();
        eprintln!("DEBUG: Rendering table with {} cached rows", display_rows.len());
        if !display_rows.is_empty() && display_rows.len() > 4 {
            eprintln!("DEBUG: Row 4 data: {:?}", display_rows[4].values);
        }

        // Use UI memory to store pending cell updates
        let pending_update_id = self.id.with("pending_cell_update");
        
        table
            .header(30.0, |mut header| {
                for col in columns.iter() {
                    header.col(|ui| {
                        // Paint header background color like datatable
                        let rect = ui.max_rect();
                        ui.painter().rect_filled(rect, egui::CornerRadius::ZERO, surface_variant);
                        
                        ui.style_mut().visuals.override_text_color = Some(on_surface);
                        ui.strong(&col.name);
                    });
                }
            })
            .body(|mut body| {
                for row_data in &display_rows {
                    body.row(self.row_height, |mut row| {
                        for (col_idx, value) in row_data.values.iter().enumerate() {
                            row.col(|ui| {
                                let is_editing = self.editing_cell == Some((row_data.id, col_idx));

                                if is_editing {
                                    // Edit mode with TextEdit
                                    eprintln!("DEBUG: Rendering TextEdit - row_id: {}, col_idx: {}, buffer: '{}'", row_data.id, col_idx, self.edit_buffer);
                                    let edit_response = ui.add(
                                        TextEdit::singleline(&mut self.edit_buffer)
                                            .desired_width(f32::INFINITY)
                                    );
                                    eprintln!("DEBUG: TextEdit state - has_focus: {}, lost_focus: {}, gained_focus: {}", 
                                        edit_response.has_focus(), edit_response.lost_focus(), edit_response.gained_focus());

                                    // Handle Enter to save, Escape to cancel, or save on blur
                                    if edit_response.lost_focus() {
                                        let escape_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape));
                                        eprintln!("DEBUG: TextEdit lost focus - escape_pressed: {}", escape_pressed);
                                        
                                        if !escape_pressed {
                                            eprintln!("DEBUG: Storing cell update - row_id: {}, col_idx: {}, value: '{}'", row_data.id, col_idx, self.edit_buffer);
                                            // Store the update in UI memory for processing after rendering
                                            ui.memory_mut(|mem| {
                                                mem.data.insert_temp(pending_update_id, (row_data.id, col_idx, self.edit_buffer.clone()));
                                            });
                                        } else {
                                            eprintln!("DEBUG: Edit cancelled with Escape");
                                        }
                                        // Always exit edit mode when losing focus
                                        self.editing_cell = None;
                                    }

                                    if edit_response.gained_focus() {
                                        edit_response.request_focus();
                                        eprintln!("DEBUG: Requested focus for TextEdit");
                                    }
                                } else {
                                    // View mode with label
                                    let label_response = ui.label(value);

                                    // Single-click to edit (changed from double-click)
                                    if self.allow_editing && label_response.clicked() {
                                        eprintln!("DEBUG: Starting edit mode - row_id: {}, col_idx: {}, current_value: {}", row_data.id, col_idx, value);
                                        self.editing_cell = Some((row_data.id, col_idx));
                                        self.edit_buffer = value.clone();
                                    }
                                }
                            });
                        }
                    });
                }
            });

        // Apply any pending cell update after rendering
        if let Some((row_id, col_idx, new_value)) = ui.memory(|mem| {
            mem.data.get_temp::<(usize, usize, String)>(pending_update_id)
        }) {
            eprintln!("DEBUG: Retrieved cell update - row_id: {}, col_idx: {}, value: {}", row_id, col_idx, new_value);
            
            // Clear the pending update
            ui.memory_mut(|mem| {
                mem.data.remove::<(usize, usize, String)>(pending_update_id);
            });


            // Use try_lock to avoid needing runtime in UI context
            let mut model = loop {
                if let Ok(guard) = self.data_model.try_lock() {
                    break guard;
                }
                std::thread::sleep(std::time::Duration::from_micros(10));
            };
            match model.update_cell(row_id, col_idx, new_value.clone()) {
                Ok(_) => {
                    eprintln!("DEBUG: Cell updated in database successfully");
                    // Refresh cached rows to show the update
                    match model.query_rows() {
                        Ok(rows) => {
                            eprintln!("DEBUG: Refreshed {} rows from database", rows.len());
                            if rows.len() > 4 {
                                eprintln!("DEBUG: After query, row 4 data: {:?}", rows[4].values);
                            }
                            self.cached_rows = rows;
                            // Request repaint so the updated data appears immediately
                            ui.ctx().request_repaint();
                            eprintln!("DEBUG: Requested repaint");
                        }
                        Err(e) => {
                            eprintln!("DEBUG: Failed to query rows: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("DEBUG: Failed to update cell: {}", e);
                    
                    // Show error to user - store in temp memory for display
                    ui.memory_mut(|mem| {
                        mem.data.insert_temp(
                            self.id.with("cell_error"),
                            e.clone()
                        );
                    });
                    ui.ctx().request_repaint();
                }
            }
        } else {
            eprintln!("DEBUG: No pending cell update found");
        }

        // Display error message if there's a cell error
        if let Some(error_msg) = ui.memory(|mem| {
            mem.data.get_temp::<String>(self.id.with("cell_error"))
        }) {
            // Clear the error
            ui.memory_mut(|mem| {
                mem.data.remove::<String>(self.id.with("cell_error"));
            });
            
            // Show error message at the top
            ui.ctx().debug_painter().text(
                ui.max_rect().center_top() + egui::vec2(0.0, 10.0),
                egui::Align2::CENTER_TOP,
                &error_msg,
                egui::FontId::proportional(14.0),
                get_global_color("error"),
            );
            
            // Keep showing the error for a bit
            ui.ctx().request_repaint_after(std::time::Duration::from_secs(3));
        }

        ui.interact(ui.max_rect(), self.id, Sense::hover())
    }
}

#[cfg(feature = "spreadsheet")]
impl Widget for MaterialSpreadsheet {
    fn ui(mut self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

/// Helper function to create a column definition
#[cfg(feature = "spreadsheet")]
pub fn column(name: impl Into<String>, col_type: ColumnType, width: f32) -> ColumnDef {
    ColumnDef {
        name: name.into(),
        col_type,
        width,
    }
}

/// Helper function to create a text column
#[cfg(feature = "spreadsheet")]
pub fn text_column(name: impl Into<String>, width: f32) -> ColumnDef {
    column(name, ColumnType::Text, width)
}

/// Helper function to create a number column
#[cfg(feature = "spreadsheet")]
pub fn number_column(name: impl Into<String>, width: f32) -> ColumnDef {
    column(name, ColumnType::Real, width)
}

/// Helper function to create an integer column
#[cfg(feature = "spreadsheet")]
pub fn integer_column(name: impl Into<String>, width: f32) -> ColumnDef {
    column(name, ColumnType::Integer, width)
}

#[cfg(test)]
#[cfg(feature = "spreadsheet")]
mod tests {
    use super::*;

    #[test]
    fn test_spreadsheet_init() {
        let columns = vec![
            text_column("Name", 100.0),
            text_column("Value", 100.0),
        ];

        let mut spreadsheet = MaterialSpreadsheet::new("test", columns)
            .expect("Failed to create spreadsheet");

        // Initialize with data
        spreadsheet.init_with_data(vec![
            vec!["Item1".to_string(), "Value1".to_string()],
            vec!["Item2".to_string(), "Value2".to_string()],
        ]);

        // Verify rows
        let rows = spreadsheet.rows();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].values[0], "Item1");
        assert_eq!(rows[1].values[1], "Value2");
    }

    #[tokio::test]
    async fn test_data_model_operations() {
        let columns = vec![
            ColumnDef { name: "Name".to_string(), col_type: ColumnType::Text, width: 100.0 },
            ColumnDef { name: "Count".to_string(), col_type: ColumnType::Integer, width: 80.0 },
        ];

        let mut model = SpreadsheetDataModel::new(columns).expect("Failed to create model");

        // Insert data
        model.insert_row(vec!["Test".to_string(), "42".to_string()]).expect("Failed to insert");

        // Query data
        let rows = model.query_rows().expect("Failed to query");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].values[0], "Test");
        assert_eq!(rows[0].values[1], "42");

        // Update cell
        model.update_cell(0, 0, "Updated".to_string()).expect("Failed to update");
        let rows = model.query_rows().expect("Failed to query");
        assert_eq!(rows[0].values[0], "Updated");
    }

    #[test]
    fn test_csv_import_export() {
        use std::path::Path;

        let columns = vec![
            text_column("Name", 100.0),
            text_column("Value", 100.0),
        ];

        let mut model = SpreadsheetDataModel::new(columns).expect("Failed to create model");

        // Add some data
        model.insert_row(vec!["Item1".to_string(), "Value1".to_string()]).expect("Failed to insert");
        model.insert_row(vec!["Item2".to_string(), "Value2".to_string()]).expect("Failed to insert");

        // Export to CSV
        let export_path = Path::new("/tmp/test_export.csv");
        model.export_csv(export_path).expect("Failed to export CSV");

        // Create new model and import
        let columns2 = vec![text_column("Col1", 100.0), text_column("Col2", 100.0)];
        let mut model2 = SpreadsheetDataModel::new(columns2).expect("Failed to create model");
        model2.import_csv(export_path).expect("Failed to import CSV");

        // Verify data
        let rows = model2.query_rows().expect("Failed to query");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].values[0], "Item1");
        assert_eq!(rows[1].values[1], "Value2");
    }

    #[tokio::test]
    async fn test_parquet_import_export() {
        use std::path::Path;

        let columns = vec![
            text_column("Product", 100.0),
            text_column("Price", 100.0),
        ];

        let mut model = SpreadsheetDataModel::new(columns).expect("Failed to create model");

        // Add some data
        model.insert_row(vec!["Laptop".to_string(), "999.99".to_string()]).expect("Failed to insert");
        model.insert_row(vec!["Mouse".to_string(), "29.99".to_string()]).expect("Failed to insert");

        // Export to Parquet
        let export_path = Path::new("/tmp/test_export.parquet");
        model.export_parquet(export_path).await.expect("Failed to export Parquet");

        // Create new model and import using DataFusion SQL
        let columns2 = vec![text_column("Col1", 100.0)];
        let mut model2 = SpreadsheetDataModel::new(columns2).expect("Failed to create model");
        model2.import_parquet(export_path).await.expect("Failed to import Parquet");

        // Verify data
        let rows = model2.query_rows().expect("Failed to query");
        assert_eq!(rows.len(), 2, "Expected 2 rows");
        assert_eq!(rows[0].values[0], "Laptop");
        assert_eq!(rows[1].values[1], "29.99");

        // Verify columns were updated from parquet schema
        assert_eq!(model2.columns.len(), 2, "Should have 2 columns from parquet file");
        assert_eq!(model2.columns[0].name, "Product");
        assert_eq!(model2.columns[1].name, "Price");
    }
}
