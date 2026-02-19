//! Material Design Spreadsheet Component
//!
//! A spreadsheet widget with DuckDB backend for data storage and manipulation.
//! Supports importing/exporting CSV, Excel, Parquet, and Arrow formats.

#[cfg(feature = "spreadsheet")]
use crate::button::MaterialButton;
#[cfg(feature = "spreadsheet")]
use crate::theme::get_global_color;
#[cfg(feature = "spreadsheet")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "spreadsheet")]
use std::path::PathBuf;
#[cfg(feature = "spreadsheet")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "spreadsheet")]
use duckdb::{params, Connection, Result as DuckResult};
#[cfg(feature = "spreadsheet")]
use egui::{
    Color32, FontFamily, FontId, Id, Rect, Response, Sense, TextEdit, Ui, Vec2, Widget,
};
#[cfg(feature = "spreadsheet")]
use egui_async::{Bind, StateWithData};

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
    fn to_sql(&self) -> &'static str {
        match self {
            ColumnType::Text => "TEXT",
            ColumnType::Integer => "INTEGER",
            ColumnType::Real => "REAL",
            ColumnType::Boolean => "BOOLEAN",
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

/// DuckDB-backed data model for spreadsheet
#[cfg(feature = "spreadsheet")]
pub struct SpreadsheetDataModel {
    conn: Connection,
    table_name: String,
    columns: Vec<ColumnDef>,
    row_count: usize,
}

#[cfg(feature = "spreadsheet")]
impl SpreadsheetDataModel {
    /// Create a new in-memory spreadsheet data model
    pub fn new(columns: Vec<ColumnDef>) -> DuckResult<Self> {
        let conn = Connection::open_in_memory()?;
        let table_name = "spreadsheet_data".to_string();

        let mut model = Self {
            conn,
            table_name: table_name.clone(),
            columns: columns.clone(),
            row_count: 0,
        };

        model.create_table()?;
        Ok(model)
    }

    fn create_table(&mut self) -> DuckResult<()> {
        let mut col_defs = vec!["id INTEGER PRIMARY KEY".to_string()];
        for (idx, col) in self.columns.iter().enumerate() {
            col_defs.push(format!("col{} {}", idx, col.col_type.to_sql()));
        }

        let create_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            self.table_name,
            col_defs.join(", ")
        );

        self.conn.execute_batch(&create_sql)?;
        Ok(())
    }

    /// Insert multiple rows
    pub fn insert_rows(&mut self, rows: Vec<Vec<String>>) -> DuckResult<()> {
        for row_values in rows {
            self.insert_row(row_values)?;
        }
        Ok(())
    }

    /// Insert a single row
    pub fn insert_row(&mut self, values: Vec<String>) -> DuckResult<()> {
        let placeholders: Vec<_> = (0..=self.columns.len()).map(|_| "?").collect();
        let insert_sql = format!(
            "INSERT INTO {} VALUES ({})",
            self.table_name,
            placeholders.join(", ")
        );

        let mut stmt = self.conn.prepare(&insert_sql)?;
        let row_id = self.row_count;
        self.row_count += 1;

        // Build params dynamically
        let mut params_vec: Vec<Box<dyn duckdb::ToSql>> = vec![Box::new(row_id)];
        for value in values {
            params_vec.push(Box::new(value));
        }

        // Convert to params slice
        let params_refs: Vec<&dyn duckdb::ToSql> = params_vec.iter().map(|b| &**b as &dyn duckdb::ToSql).collect();
        stmt.execute(params_refs.as_slice())?;

        Ok(())
    }

    /// Query all rows
    pub fn query_rows(&self) -> DuckResult<Vec<RowData>> {
        let query_sql = format!("SELECT * FROM {} ORDER BY id", self.table_name);
        let mut stmt = self.conn.prepare(&query_sql)?;

        let col_count = self.columns.len();
        let rows = stmt.query_map([], |row| {
            let id: usize = row.get(0)?;
            let mut values = Vec::new();
            for i in 0..col_count {
                // Convert all column types to strings for display
                let val: String = match row.get_ref(i + 1) {
                    Ok(val_ref) => {
                        use duckdb::types::ValueRef;
                        match val_ref {
                            ValueRef::Null => String::new(),
                            ValueRef::Boolean(b) => b.to_string(),
                            ValueRef::TinyInt(i) => i.to_string(),
                            ValueRef::SmallInt(i) => i.to_string(),
                            ValueRef::Int(i) => i.to_string(),
                            ValueRef::BigInt(i) => i.to_string(),
                            ValueRef::HugeInt(i) => i.to_string(),
                            ValueRef::UTinyInt(i) => i.to_string(),
                            ValueRef::USmallInt(i) => i.to_string(),
                            ValueRef::UInt(i) => i.to_string(),
                            ValueRef::UBigInt(i) => i.to_string(),
                            ValueRef::Float(f) => f.to_string(),
                            ValueRef::Double(f) => f.to_string(),
                            ValueRef::Text(s) => String::from_utf8_lossy(s).to_string(),
                            ValueRef::Blob(b) => format!("<blob {} bytes>", b.len()),
                            _ => row.get::<_, String>(i + 1).unwrap_or_default(),
                        }
                    }
                    Err(_) => String::new(),
                };
                values.push(val);
            }
            Ok(RowData { id, values })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
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
        
        let update_sql = format!(
            "UPDATE {} SET col{} = ? WHERE id = ?",
            self.table_name, col_idx
        );
        self.conn.execute(&update_sql, params![value, row_id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Delete a row
    pub fn delete_row(&mut self, row_id: usize) -> DuckResult<()> {
        let delete_sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
        self.conn.execute(&delete_sql, params![row_id])?;
        Ok(())
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
        
        // Determine if first line is a header (heuristic: check if types differ)
        let first_line_is_header = first_values.iter().zip(second_values.iter()).any(|(v1, v2)| {
            let v1_is_num = v1.trim().parse::<f64>().is_ok();
            let v2_is_num = v2.trim().parse::<f64>().is_ok();
            v1_is_num != v2_is_num
        });
        
        // Create new column definitions
        let new_columns: Vec<ColumnDef> = if first_line_is_header {
            first_values.iter().enumerate().map(|(i, name)| {
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
        self.conn.execute_batch(&format!("DROP TABLE IF EXISTS {}", self.table_name))
            .map_err(|e| format!("Failed to drop table: {}", e))?;
        self.row_count = 0;
        self.create_table()
            .map_err(|e| format!("Failed to create table: {}", e))?;
        
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

    /// Export to Parquet using DuckDB's built-in support
    pub fn export_parquet(&self, path: &std::path::Path) -> Result<(), String> {
        let export_sql = format!(
            "COPY {} TO '{}' (FORMAT PARQUET)",
            self.table_name,
            path.to_string_lossy()
        );
        self.conn
            .execute_batch(&export_sql)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Import from Parquet using DuckDB's built-in support
    pub fn import_parquet(&mut self, path: &std::path::Path) -> Result<(), String> {
        // Clear existing data before importing
        self.conn.execute(&format!("DELETE FROM {}", self.table_name), [])
            .map_err(|e| format!("Failed to clear table: {}", e))?;
        self.row_count = 0;

        // Read parquet data and insert row by row to handle type conversions
        let query_sql = format!("SELECT * FROM read_parquet('{}')", path.to_string_lossy());
        let mut stmt = self.conn.prepare(&query_sql)
            .map_err(|e| format!("Failed to read parquet file: {}", e))?;
        
        let col_count = self.columns.len();
        let rows = stmt.query_map([], |row| {
            let mut values = Vec::new();
            for i in 0..col_count {
                // Convert all values to strings for compatibility
                let val: String = match row.get_ref(i) {
                    Ok(val_ref) => {
                        use duckdb::types::ValueRef;
                        match val_ref {
                            ValueRef::Null => String::new(),
                            ValueRef::Boolean(b) => b.to_string(),
                            ValueRef::TinyInt(i) => i.to_string(),
                            ValueRef::SmallInt(i) => i.to_string(),
                            ValueRef::Int(i) => i.to_string(),
                            ValueRef::BigInt(i) => i.to_string(),
                            ValueRef::HugeInt(i) => i.to_string(),
                            ValueRef::UTinyInt(i) => i.to_string(),
                            ValueRef::USmallInt(i) => i.to_string(),
                            ValueRef::UInt(i) => i.to_string(),
                            ValueRef::UBigInt(i) => i.to_string(),
                            ValueRef::Float(f) => f.to_string(),
                            ValueRef::Double(f) => f.to_string(),
                            ValueRef::Text(s) => String::from_utf8_lossy(s).to_string(),
                            _ => String::new(),
                        }
                    }
                    Err(_) => String::new(),
                };
                values.push(val);
            }
            Ok(values)
        })
        .map_err(|e| format!("Failed to query parquet data: {}", e))?;

        // Collect all rows first to avoid borrow conflicts
        let all_rows: Vec<Vec<String>> = rows
            .map(|row_result| row_result.map_err(|e| format!("Failed to process row: {}", e)))
            .collect::<Result<Vec<_>, String>>()?;
        
        // Drop stmt to release the borrow on self.conn
        drop(stmt);

        // Insert all rows
        for values in all_rows {
            self.insert_row(values).map_err(|e| format!("Failed to insert row: {}", e))?;
        }

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
    selected_rows: HashSet<usize>,
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
            selected_rows: HashSet::new(),
            allow_editing: true,
            allow_selection: true,
            striped: true,
            row_height: 36.0,
            load_bind: Bind::new(false),
            save_bind: Bind::new(false),
            load_processed: false,
        })
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
    pub fn add_row(&mut self) -> Result<(), String> {
        let mut model = self.data_model.lock().unwrap();
        let col_count = model.columns.len();
        let empty_values = vec![String::new(); col_count];
        model.insert_row(empty_values).map_err(|e| e.to_string())?;
        // Refresh cached rows to show the new row
        self.cached_rows = model.query_rows().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Delete a row by ID
    pub fn delete_row(&mut self, row_id: usize) -> Result<(), String> {
        let mut model = self.data_model.lock().unwrap();
        model.delete_row(row_id).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Refresh cached data from database
    pub fn refresh_data(&mut self) -> Result<(), String> {
        let model = self.data_model.lock().unwrap();
        self.cached_rows = model.query_rows().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Load data from file (async)
    pub fn load_from_file(&mut self, path: PathBuf) {
        self.load_processed = false; // Reset flag for new load
        let model = Arc::clone(&self.data_model);
        self.load_bind.refresh(async move {
            let format = FileFormat::from_path(&path).ok_or_else(|| "Unknown file format".to_string())?;
            
            let mut locked_model = model.lock().unwrap();
            match format {
                FileFormat::Csv => locked_model.import_csv(&path)?,
                FileFormat::Parquet => locked_model.import_parquet(&path)?,
                FileFormat::Excel => return Err("Excel import not yet implemented".to_string()),
                FileFormat::Arrow => return Err("Arrow import not yet implemented".to_string()),
            }
            
            // Return all rows after loading
            locked_model.query_rows().map_err(|e| e.to_string())
        });
    }

    /// Save data to file (async)
    pub fn save_to_file(&mut self, path: PathBuf) {
        let model = Arc::clone(&self.data_model);
        self.save_bind.refresh(async move {
            let format = FileFormat::from_path(&path).ok_or_else(|| "Unknown file format".to_string())?;
            
            let locked_model = model.lock().unwrap();
            match format {
                FileFormat::Csv => locked_model.export_csv(&path)?,
                FileFormat::Parquet => locked_model.export_parquet(&path)?,
                FileFormat::Excel => return Err("Excel export not yet implemented".to_string()),
                FileFormat::Arrow => return Err("Arrow export not yet implemented".to_string()),
            }
            
            Ok(())
        });
    }

    /// Get the column definitions
    pub fn columns(&self) -> Vec<ColumnDef> {
        self.data_model.lock().unwrap().columns.clone()
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
            }
            StateWithData::Failed(err) => {
                ui.label(format!("Save error: {}", err));
            }
            _ => {}
        }

        // Get column definitions
        let columns = self.data_model.lock().unwrap().columns.clone();

        // Get theme colors
        let surface = get_global_color("surface");
        let on_surface = get_global_color("on-surface");
        let primary = get_global_color("primary");
        let surface_variant = get_global_color("surface-variant");

        // Build table
        let text_height = ui.text_style_height(&egui::TextStyle::Body);
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
                        ui.painter().rect_filled(rect, egui::Rounding::ZERO, surface_variant);
                        
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
            
            let mut model = self.data_model.lock().unwrap();
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
