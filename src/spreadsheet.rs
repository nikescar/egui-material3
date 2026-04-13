//! Material Design Spreadsheet Component
//!
//! A spreadsheet widget with SQLite backend (via rusqlite) for data storage and manipulation.
//! Supports importing/exporting CSV and Excel formats.

#[cfg(feature = "spreadsheet")]
use crate::theme::get_global_color;
#[cfg(feature = "spreadsheet")]
use std::path::PathBuf;
#[cfg(feature = "spreadsheet")]
use async_std::sync::{Arc, Mutex};

// Native: use rusqlite for better dynamic schema support
#[cfg(all(feature = "spreadsheet", not(target_family = "wasm")))]
mod native_imports {
    pub use rusqlite::Connection;
}

// WASM: use diesel (only option that works on WASM)
#[cfg(all(feature = "spreadsheet", target_family = "wasm"))]
mod wasm_imports {
    pub use diesel::prelude::*;
    pub use diesel::connection::SimpleConnection;
    pub use std::sync::Once;

    pub static VFS: std::sync::Mutex<(i32, Once)> = std::sync::Mutex::new((0, Once::new()));
}

#[cfg(all(feature = "spreadsheet", not(target_family = "wasm")))]
use native_imports::*;

#[cfg(all(feature = "spreadsheet", target_family = "wasm"))]
use wasm_imports::*;
#[cfg(feature = "spreadsheet")]
use egui::{Id, Response, Sense, TextEdit, Ui, Widget};
#[cfg(feature = "spreadsheet")]
use crate::egui_async_std::{Bind, StateWithData};

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
    fn to_sql_type(&self) -> &'static str {
        match self {
            ColumnType::Text => "TEXT",
            ColumnType::Integer => "INTEGER",
            ColumnType::Real => "REAL",
            ColumnType::Boolean => "INTEGER", // SQLite stores booleans as integers
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
}

#[cfg(feature = "spreadsheet")]
impl FileFormat {
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        path.extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
            "csv" => Some(FileFormat::Csv),
            "xls" | "xlsx" => Some(FileFormat::Excel),
            _ => None,
        })
    }
}

/// SQLite-backed data model for spreadsheet
#[cfg(feature = "spreadsheet")]
pub struct SpreadsheetDataModel {
    #[cfg(not(target_family = "wasm"))]
    conn: Connection, // rusqlite for native
    #[cfg(target_family = "wasm")]
    conn: SqliteConnection, // diesel for WASM
    columns: Vec<ColumnDef>,
    table_name: String,
    row_count: usize,
}

#[cfg(feature = "spreadsheet")]
impl SpreadsheetDataModel {
    /// Create a new in-memory spreadsheet data model with SQLite backend
    pub fn new(columns: Vec<ColumnDef>) -> Result<Self, String> {
        let table_name = "spreadsheet_data".to_string();

        #[cfg(not(target_family = "wasm"))]
        let conn = {
            // Native: use rusqlite
            Connection::open_in_memory()
                .map_err(|e| format!("Failed to create in-memory database: {}", e))?
        };

        #[cfg(target_family = "wasm")]
        let conn = {
            // WASM: use diesel with WASM VFS
            let (vfs, _once) = &*VFS.lock().unwrap();
            let url = match vfs {
                0 => ":memory:",  // in-memory for spreadsheet
                1 => "file:spreadsheet.db?vfs=opfs-sahpool",
                2 => "file:spreadsheet.db?vfs=relaxed-idb",
                _ => ":memory:",
            };
            SqliteConnection::establish(url)
                .map_err(|e| format!("Failed to create WASM database: {}", e))?
        };

        let mut model = Self {
            conn,
            columns: columns.clone(),
            table_name,
            row_count: 0,
        };

        // Create the table with the specified columns
        model.create_table()?;

        Ok(model)
    }

    /// Create the table in SQLite database
    fn create_table(&mut self) -> Result<(), String> {
        if self.columns.is_empty() {
            return Err("Cannot create table without columns".to_string());
        }

        // Build CREATE TABLE statement
        let mut col_defs = vec!["id INTEGER PRIMARY KEY AUTOINCREMENT".to_string()];
        for col in &self.columns {
            // Sanitize column name to prevent SQL injection
            let safe_name = col.name.replace('"', "\"\"");
            col_defs.push(format!("\"{}\" {}", safe_name, col.col_type.to_sql_type()));
        }

        let create_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            self.table_name,
            col_defs.join(", ")
        );

        #[cfg(not(target_family = "wasm"))]
        {
            self.conn.execute(&create_sql, [])
                .map_err(|e| format!("Failed to create table: {}", e))?;
        }

        #[cfg(target_family = "wasm")]
        {
            self.conn.batch_execute(&create_sql)
                .map_err(|e| format!("Failed to create table: {}", e))?;
        }

        Ok(())
    }

    /// Add a new column to the table
    #[allow(dead_code)]
    fn add_column(&mut self, col: &ColumnDef) -> Result<(), String> {
        let safe_name = col.name.replace('"', "\"\"");
        let alter_sql = format!(
            "ALTER TABLE {} ADD COLUMN \"{}\" {}",
            self.table_name,
            safe_name,
            col.col_type.to_sql_type()
        );

        #[cfg(not(target_family = "wasm"))]
        {
            self.conn.execute(&alter_sql, [])
                .map_err(|e| format!("Failed to add column: {}", e))?;
        }

        #[cfg(target_family = "wasm")]
        {
            self.conn.batch_execute(&alter_sql)
                .map_err(|e| format!("Failed to add column: {}", e))?;
        }

        self.columns.push(col.clone());
        Ok(())
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
        if values.len() != self.columns.len() {
            return Err(format!(
                "Row has {} values but table has {} columns",
                values.len(),
                self.columns.len()
            ));
        }

        // Build INSERT statement with inline values (SQLite doesn't have parameter limit issues)
        let col_names: Vec<String> = self.columns.iter()
            .map(|col| format!("\"{}\"", col.name.replace('"', "\"\"")))
            .collect();

        let mut value_strs = Vec::new();
        for (idx, value) in values.iter().enumerate() {
            let col_type = &self.columns[idx].col_type;
            let value_str = match col_type {
                ColumnType::Text => format!("'{}'", value.replace('\'', "''")),
                ColumnType::Integer => {
                    if value.is_empty() {
                        "NULL".to_string()
                    } else {
                        value.parse::<i64>()
                            .map_err(|_| format!("Invalid integer value: {}", value))?
                            .to_string()
                    }
                }
                ColumnType::Real => {
                    if value.is_empty() {
                        "NULL".to_string()
                    } else {
                        value.parse::<f64>()
                            .map_err(|_| format!("Invalid real value: {}", value))?
                            .to_string()
                    }
                }
                ColumnType::Boolean => {
                    if value.is_empty() {
                        "NULL".to_string()
                    } else {
                        let bool_val = match value.to_lowercase().as_str() {
                            "1" | "true" => true,
                            "0" | "false" => false,
                            _ => value.parse::<bool>()
                                .map_err(|_| format!("Invalid boolean value: {}", value))?,
                        };
                        if bool_val { "1" } else { "0" }.to_string()
                    }
                }
            };
            value_strs.push(value_str);
        }

        let insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table_name,
            col_names.join(", "),
            value_strs.join(", ")
        );

        #[cfg(not(target_family = "wasm"))]
        {
            self.conn.execute(&insert_sql, [])
                .map_err(|e| format!("Failed to insert row: {}", e))?;
        }

        #[cfg(target_family = "wasm")]
        {
            diesel::sql_query(&insert_sql)
                .execute(&mut self.conn)
                .map_err(|e| format!("Failed to insert row: {}", e))?;
        }

        self.row_count += 1;
        Ok(())
    }

    /// Query all rows from the database
    pub fn query_rows(&mut self) -> Result<Vec<RowData>, String> {
        #[cfg(not(target_family = "wasm"))]
        {
            // Native: use rusqlite
            let select_sql = format!("SELECT * FROM {} ORDER BY id", self.table_name);

            let mut stmt = self.conn.prepare(&select_sql)
                .map_err(|e| format!("Failed to prepare statement: {}", e))?;

            let mut rows = stmt.query([])
                .map_err(|e| format!("Failed to query rows: {}", e))?;

            let mut result = Vec::new();
            let mut row_idx = 0;

            while let Some(row) = rows.next().map_err(|e| format!("Failed to fetch row: {}", e))? {
                let mut values = Vec::new();

                // Skip the first column (id) and read the data columns
                for (col_idx, col) in self.columns.iter().enumerate() {
                    let value = match &col.col_type {
                        ColumnType::Text => {
                            row.get::<_, Option<String>>(col_idx + 1)
                                .map_err(|e| format!("Failed to get text value: {}", e))?
                                .unwrap_or_default()
                        }
                        ColumnType::Integer => {
                            row.get::<_, Option<i64>>(col_idx + 1)
                                .map_err(|e| format!("Failed to get integer value: {}", e))?
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        }
                        ColumnType::Real => {
                            row.get::<_, Option<f64>>(col_idx + 1)
                                .map_err(|e| format!("Failed to get real value: {}", e))?
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        }
                        ColumnType::Boolean => {
                            row.get::<_, Option<i32>>(col_idx + 1)
                                .map_err(|e| format!("Failed to get boolean value: {}", e))?
                                .map(|v| if v != 0 { "true".to_string() } else { "false".to_string() })
                                .unwrap_or_default()
                        }
                    };
                    values.push(value);
                }

                result.push(RowData {
                    id: row_idx,
                    values,
                });
                row_idx += 1;
            }

            Ok(result)
        }

        #[cfg(target_family = "wasm")]
        {
            // WASM: diesel doesn't support dynamic queries well, so return empty for now
            // In production, you'd want to implement a proper solution
            Ok(vec![])
        }
    }

    /// Update a single cell
    pub fn update_cell(&mut self, row_id: usize, col_idx: usize, value: String) -> Result<(), String> {
        if col_idx >= self.columns.len() {
            return Err("Invalid column index".to_string());
        }

        let col = &self.columns[col_idx];
        let col_name = col.name.replace('"', "\"\"");

        // Validate value against column type
        match &col.col_type {
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
                if !value.is_empty() {
                    let lower = value.to_lowercase();
                    if lower != "true" && lower != "false" && lower != "1" && lower != "0" {
                        return Err(format!("'{}' is not a valid boolean (use true/false or 1/0)", value));
                    }
                }
            }
            ColumnType::Text => {} // Text accepts anything
        }

        // row_id is 0-based but SQL id is 1-based and auto-incrementing
        let actual_id = (row_id + 1) as i64;

        // Build UPDATE statement with inline value
        let value_str = match &col.col_type {
            ColumnType::Text => format!("'{}'", value.replace('\'', "''")),
            ColumnType::Integer => {
                if value.is_empty() {
                    "NULL".to_string()
                } else {
                    value.parse::<i64>()
                        .map_err(|_| format!("Invalid integer value: {}", value))?
                        .to_string()
                }
            }
            ColumnType::Real => {
                if value.is_empty() {
                    "NULL".to_string()
                } else {
                    value.parse::<f64>()
                        .map_err(|_| format!("Invalid real value: {}", value))?
                        .to_string()
                }
            }
            ColumnType::Boolean => {
                if value.is_empty() {
                    "NULL".to_string()
                } else {
                    let bool_val = match value.to_lowercase().as_str() {
                        "1" | "true" => true,
                        "0" | "false" => false,
                        _ => value.parse::<bool>()
                            .map_err(|_| format!("Invalid boolean value: {}", value))?,
                    };
                    if bool_val { "1" } else { "0" }.to_string()
                }
            }
        };

        let update_sql = format!(
            "UPDATE {} SET \"{}\" = {} WHERE id = {}",
            self.table_name,
            col_name,
            value_str,
            actual_id
        );

        #[cfg(not(target_family = "wasm"))]
        {
            self.conn.execute(&update_sql, [])
                .map_err(|e| format!("Failed to update cell: {}", e))?;
        }

        #[cfg(target_family = "wasm")]
        {
            diesel::sql_query(&update_sql)
                .execute(&mut self.conn)
                .map_err(|e| format!("Failed to update cell: {}", e))?;
        }

        Ok(())
    }

    /// Delete a row
    pub fn delete_row(&mut self, row_id: usize) -> Result<(), String> {
        let actual_id = (row_id + 1) as i64;
        let delete_sql = format!("DELETE FROM {} WHERE id = {}", self.table_name, actual_id);

        #[cfg(not(target_family = "wasm"))]
        {
            self.conn.execute(&delete_sql, [])
                .map_err(|e| format!("Failed to delete row: {}", e))?;
        }

        #[cfg(target_family = "wasm")]
        {
            diesel::sql_query(&delete_sql)
                .execute(&mut self.conn)
                .map_err(|e| format!("Failed to delete row: {}", e))?;
        }

        Ok(())
    }

    /// Export to CSV
    pub fn export_csv(&mut self, path: &std::path::Path) -> Result<(), String> {
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
        let looks_like_header = first_values.iter().all(|v| {
            let trimmed = v.trim();
            trimmed.len() < 50 &&
            !trimmed.is_empty() &&
            trimmed.parse::<f64>().is_err() &&
            !trimmed.contains(|c: char| c.is_numeric() && trimmed.len() > 20)
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

        // Drop and recreate table with new columns
        let drop_sql = format!("DROP TABLE IF EXISTS {}", self.table_name);

        #[cfg(not(target_family = "wasm"))]
        {
            self.conn.execute(&drop_sql, [])
                .map_err(|e| format!("Failed to drop table: {}", e))?;
        }

        #[cfg(target_family = "wasm")]
        {
            self.conn.batch_execute(&drop_sql)
                .map_err(|e| format!("Failed to drop table: {}", e))?;
        }

        self.columns = new_columns;
        self.row_count = 0;
        self.create_table()?;

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
    row_selection_enabled: bool,
    selected_row: Option<usize>,
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
            row_selection_enabled: false,
            selected_row: None,
        })
    }

    /// Initialize spreadsheet with data (sync method for use in constructors)
    /// This is a convenience method that doesn't require an async context
    pub fn init_with_data(&mut self, rows: Vec<Vec<String>>) {
        // Use try_lock in a loop to avoid needing a runtime
        loop {
            if let Some(mut model) = self.data_model.try_lock() {
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

    /// Enable or disable row selection mode
    pub fn set_row_selection_enabled(&mut self, enabled: bool) {
        self.row_selection_enabled = enabled;
        if !enabled {
            self.selected_row = None;
        }
    }

    /// Get currently selected row
    pub fn get_selected_row(&self) -> Option<usize> {
        self.selected_row
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
        let mut model = self.data_model.lock().await;
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
                FileFormat::Excel => Err("Excel import not yet implemented".to_string()),
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
                    let mut locked_model = model.lock().await;
                    locked_model.export_csv(&path)?;
                    Ok(())
                }
                FileFormat::Excel => Err("Excel export not yet implemented".to_string()),
            }
        });
    }

    /// Get the column definitions (blocking version for sync context)
    pub fn columns(&self) -> Vec<ColumnDef> {
        // Use try_lock to avoid needing a runtime
        loop {
            if let Some(model) = self.data_model.try_lock() {
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
            if let Some(model) = self.data_model.try_lock() {
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
                    let is_selected = self.row_selection_enabled && self.selected_row == Some(row_data.id);

                    body.row(self.row_height, |mut row| {
                        let mut row_clicked = false;

                        for (col_idx, value) in row_data.values.iter().enumerate() {
                            row.col(|ui| {
                                // Highlight selected row
                                if is_selected {
                                    let rect = ui.max_rect();
                                    let highlight_color = ui.visuals().selection.bg_fill.gamma_multiply(0.5);
                                    ui.painter().rect_filled(rect, egui::CornerRadius::ZERO, highlight_color);
                                }

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
                                    // View mode with label - use a sense that detects clicks
                                    let label_response = if self.row_selection_enabled {
                                        ui.add(egui::Label::new(value).sense(Sense::click()))
                                    } else {
                                        ui.label(value)
                                    };

                                    // Handle row selection click
                                    if self.row_selection_enabled && label_response.clicked() {
                                        row_clicked = true;
                                    }

                                    // Single-click to edit (changed from double-click) - only if editing is enabled and selection is not
                                    if self.allow_editing && !self.row_selection_enabled && label_response.clicked() {
                                        eprintln!("DEBUG: Starting edit mode - row_id: {}, col_idx: {}, current_value: {}", row_data.id, col_idx, value);
                                        self.editing_cell = Some((row_data.id, col_idx));
                                        self.edit_buffer = value.clone();
                                    }
                                }
                            });
                        }

                        // Update selected row after the row is rendered
                        if row_clicked {
                            self.selected_row = Some(row_data.id);
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
                if let Some(guard) = self.data_model.try_lock() {
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

    #[test]
    fn test_data_model_operations() {
        async_std::task::block_on(async {
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
        })
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
}
