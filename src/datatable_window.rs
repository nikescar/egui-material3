#![doc(hidden)]

use eframe::egui::{self, Ui, Window, Id};
use crate::{MaterialButton, MaterialCheckbox, data_table};
use crate::datatable::{SortDirection as DataTableSortDirection, RowAction};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct TableRow {
    product: String,
    category: String,
    price: String,
    stock: String,
    _selected: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum SortColumn {
    Product,
    Category,
    Price,
    Stock,
}

#[derive(Clone, Debug, PartialEq)]
enum SortDirection {
    Ascending,
    Descending,
}


#[doc(hidden)]
pub struct DataTableWindow {
    pub open: bool,
    allow_selection: bool,
    sticky_header: bool,
    show_progress: bool,
    _selected_rows: Vec<bool>,
    // Interactive table data
    interactive_rows: Vec<TableRow>,
    // Track selection state for interactive table
    interactive_selection: Vec<bool>,
    // Track editing state - which rows are being edited
    editing_rows: HashSet<usize>,
    // Temporary storage for editing data
    edit_data: HashMap<usize, Vec<String>>,
    // Sorting state
    sort_column: Option<SortColumn>,
    sort_direction: SortDirection,
}

impl Default for DataTableWindow {
    fn default() -> Self {
        let interactive_rows = vec![
            TableRow {
                product: "Laptop".to_string(),
                category: "Electronics".to_string(),
                price: "$999".to_string(),
                stock: "15".to_string(),
                _selected: false,
            },
            TableRow {
                product: "Book".to_string(),
                category: "Education".to_string(),
                price: "$29".to_string(),
                stock: "50".to_string(),
                _selected: false,
            },
            TableRow {
                product: "Shirt".to_string(),
                category: "Clothing".to_string(),
                price: "$49".to_string(),
                stock: "100".to_string(),
                _selected: false,
            },
            TableRow {
                product: "Phone".to_string(),
                category: "Electronics".to_string(),
                price: "$799".to_string(),
                stock: "25".to_string(),
                _selected: false,
            },
            TableRow {
                product: "Headphones".to_string(),
                category: "Electronics".to_string(),
                price: "$199".to_string(),
                stock: "40".to_string(),
                _selected: false,
            },
        ];
        let interactive_selection = vec![false; interactive_rows.len()];
        
        Self {
            open: false,
            allow_selection: true,
            sticky_header: false,
            show_progress: false,
            _selected_rows: vec![false, true, true, false, false],
            interactive_rows,
            interactive_selection,
            editing_rows: HashSet::new(),
            edit_data: HashMap::new(),
            sort_column: None,
            sort_direction: SortDirection::Ascending,
        }
    }
}

impl DataTableWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        let mut should_close = false;
        
        Window::new("Data Table Stories")
            .open(&mut open)
            .default_size([1000.0, 700.0])
            .show(ctx, |ui| {
                // Check for ESC key press to close window
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    should_close = true;
                }
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_data_table_examples(ui);
                });
            });
        
        if should_close {
            open = false;
        }
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.push_id("datatable_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Data Table Controls");

                if ui.button("Target").clicked() {
                    let _ = webbrowser::open("https://m2.material.io/components/data-tables");
                }
            });

            ui.horizontal(|ui| {
                ui.push_id("allow_selection_control", |ui| {
                    ui.add(MaterialCheckbox::new(&mut self.allow_selection, "Allow Selection"));
                });
                ui.push_id("sticky_header_control", |ui| {
                    ui.add(MaterialCheckbox::new(&mut self.sticky_header, "Sticky Header"));
                });
                ui.push_id("show_progress_control", |ui| {
                    ui.add(MaterialCheckbox::new(&mut self.show_progress, "Show Progress"));
                });
            });
        });
    }

    fn render_data_table_examples(&mut self, ui: &mut Ui) {
        ui.push_id("datatable_examples", |ui| {
            ui.heading("Basic Data Table");
            
            let mut basic_table = data_table()
                .id(Id::new("basic_data_table"))
                .column("Dessert", 180.0, false)
                .column("Carbs (g)", 100.0, true)
                .column("Protein (g)", 100.0, true)
                .column("Comments", 200.0, false)
                .row(|row| {
                    row.cell("Frozen yogurt")
                       .cell("24")
                       .cell("4.0")
                       .cell("Super tasty")
                       .id("basic_row_0")
                })
                .row(|row| {
                    row.cell("Ice cream sandwich")
                       .cell("37")
                       .cell("4.33")
                       .cell("I like ice cream more")
                       .id("basic_row_1")
                })
                .row(|row| {
                    row.cell("Eclair")
                       .cell("24")
                       .cell("6.0")
                       .cell("New filing flavor")
                       .id("basic_row_2")
                })
                .row(|row| {
                    row.cell("Cupcake")
                       .cell("67")
                       .cell("4.3")
                       .cell("Very sweet")
                       .id("basic_row_3")
                })
                .row(|row| {
                    row.cell("Jelly bean")
                       .cell("0")
                       .cell("0.0")
                       .cell("Colorful candy")
                       .id("basic_row_4")
                });

        if self.allow_selection {
            basic_table = basic_table.allow_selection(true);
        }
        if self.sticky_header {
            basic_table = basic_table.sticky_header(true);
        }
        if self.show_progress {
            basic_table = basic_table.show_progress(true);
        }

            ui.add(basic_table);

            ui.add_space(20.0);
            
            ui.heading("Data Table with Sortable Columns");
            
            let sortable_table = data_table()
                .id(Id::new("sortable_data_table"))
                .sortable_column("Signal name", 200.0, false)
                .sortable_column("Status", 100.0, false)
                .sortable_column("Severity", 100.0, false)
                .sortable_column("Stage", 120.0, false)
                .sortable_column("Time", 80.0, true)
                .column("Roles", 150.0, false)
                .allow_selection(true)
                .row(|row| {
                    row.cell("Arcus watch slowdown")
                       .cell("Online")
                       .cell("Medium")
                       .cell("Triaged")
                       .cell("0:33")
                       .cell("Allison Brie")
                       .id("sortable_row_0")
                })
                .row(|row| {
                    row.cell("monarch: prod shared ares-managed-features-provider-heavy")
                       .cell("Offline")
                       .cell("Huge")
                       .cell("Triaged")
                       .cell("0:33")
                       .cell("Brie Larson")
                       .selected(true)
                       .readonly(true)
                       .id("sortable_row_1")
                })
                .row(|row| {
                    row.cell("monarch: prod shared ares-managed-features-provider-heavy")
                       .cell("Online")
                       .cell("Minor")
                       .cell("Not triaged")
                       .cell("0:33")
                       .cell("Jeremy Lake")
                       .selected(true)
                       .readonly(true)
                       .id("sortable_row_2")
                })
                .row(|row| {
                    row.cell("Arcus watch slowdown")
                       .cell("Online")
                       .cell("Negligible")
                       .cell("Triaged")
                       .cell("0:33")
                       .cell("Angelina Cheng")
                       .id("sortable_row_3")
                });
                
            ui.add(sortable_table);

            ui.add_space(20.0);
            
            ui.heading("Compact Data Table");
            
            let compact_table = data_table()
                .id(Id::new("compact_data_table"))
                .column("ID", 60.0, true)
                .column("Name", 120.0, false)
                .column("Status", 80.0, false)
                .column("Progress", 100.0, true)
                .row(|row| {
                    row.cell("001")
                       .cell("Task Alpha")
                       .cell("Active")
                       .cell("75%")
                       .id("compact_row_0")
                })
                .row(|row| {
                    row.cell("002")
                       .cell("Task Beta")
                       .cell("Pending")
                       .cell("25%")
                       .id("compact_row_1")
                })
                .row(|row| {
                    row.cell("003")
                       .cell("Task Gamma")
                       .cell("Complete")
                       .cell("100%")
                       .id("compact_row_2")
                });

            ui.add(compact_table);

            ui.add_space(20.0);
        
        ui.heading("Interactive Data Table Demo");
        
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Add Row")).clicked() {
                // Add a new row with sample data
                let new_row = TableRow {
                    product: format!("Product {}", self.interactive_rows.len() + 1),
                    category: "New Category".to_string(),
                    price: "$99".to_string(),
                    stock: "10".to_string(),
                    _selected: false,
                };
                self.interactive_rows.push(new_row);
                self.interactive_selection.push(false);
                println!("Added new row");
            }
            
            if ui.add(MaterialButton::outlined("Delete Selected")).clicked() {
                // Remove selected rows based on our tracked selection
                let selected_count = self.interactive_selection.iter().filter(|&&sel| sel).count();
                
                if selected_count > 0 {
                    // Create new vectors without selected items
                    let mut new_rows = Vec::new();
                    let mut new_selection = Vec::new();
                    
                    for (_idx, (row, &selected)) in self.interactive_rows.iter().zip(self.interactive_selection.iter()).enumerate() {
                        if !selected {
                            new_rows.push(row.clone());
                            new_selection.push(false);
                        }
                    }
                    
                    self.interactive_rows = new_rows;
                    self.interactive_selection = new_selection;
                    self.editing_rows.clear(); // Cancel any ongoing edits
                    println!("Deleted {} selected rows", selected_count);
                } else {
                    println!("No rows selected for deletion");
                }
            }
            
            if ui.add(MaterialButton::text("Export")).clicked() {
                // Export data (in real implementation, this could save to CSV, etc.)
                let export_data: Vec<String> = self.interactive_rows
                    .iter()
                    .map(|row| format!("{},{},{},{}", row.product, row.category, row.price, row.stock))
                    .collect();
                
                println!("Exported data:");
                println!("Product,Category,Price,Stock");
                for line in export_data {
                    println!("{}", line);
                }
            }
        });

        ui.add_space(10.0);
        
        // The data table will now handle sorting internally, but we keep our local sorting for reference
        let sorted_rows = &self.interactive_rows; // Use reference to original data

        // Use the proper data table widget for Interactive Data Table Demo
        let mut interactive_table = data_table()
            .id(Id::new("interactive_data_table"))
            .column("Product", 180.0, false)  // All columns are sortable by default now
            .column("Category", 120.0, false)
            .column("Price", 100.0, true)
            .column("Stock", 80.0, true)
            .column("Actions", 140.0, false)  // Add Actions column
            .allow_selection(true);

        // Add rows dynamically from our data
        for (idx, row) in sorted_rows.iter().enumerate() {
            let original_idx = idx; // Use direct index since sorting is handled by the data table
            
            let is_selected = self.interactive_selection.get(original_idx).copied().unwrap_or(false);
            
            // Check if this row is being edited
            let is_editing = self.editing_rows.contains(&original_idx);
            
            // Create cell content - use actual values, let the data table handle edit mode rendering
            let (product_text, category_text, price_text, stock_text, actions_text) = (
                row.product.clone(), 
                row.category.clone(), 
                row.price.clone(), 
                row.stock.clone(),
                if is_editing { "Submit | Cancel".to_string() } else { "Edit | Delete".to_string() }
            );
            
            interactive_table = interactive_table.row(|table_row| {
                let mut row_builder = table_row
                    .cell(&product_text)
                    .cell(&category_text)
                    .cell(&price_text)
                    .cell(&stock_text)
                    .cell(&actions_text)  // Add actions cell
                    .id(format!("interactive_table_row_{}", original_idx));
                
                if is_selected {
                    row_builder = row_builder.selected(true);
                }
                
                row_builder
            });
        }

        // Set external editing state for the data table to use
        ui.memory_mut(|mem| {
            mem.data.insert_temp(Id::new("interactive_data_table").with("external_edit_state"), 
                (self.editing_rows.clone(), self.edit_data.clone()));
        });
        
        // Show the table and get the selection state back
        let table_response = interactive_table.show(ui);
        
        // Retrieve updated editing state from the data table
        if let Some((updated_editing_rows, updated_edit_data)) = ui.memory(|mem| {
            mem.data.get_temp::<(HashSet<usize>, HashMap<usize, Vec<String>>)>(Id::new("interactive_data_table").with("external_edit_state"))
        }) {
            self.editing_rows = updated_editing_rows;
            self.edit_data = updated_edit_data;
        }
        
        // Process row actions from the data table
        for action in &table_response.row_actions {
            match action {
                RowAction::Edit(row_idx) => {
                    if let Some(row) = self.interactive_rows.get(*row_idx) {
                        // Initialize edit data for this row
                        let row_data = vec![
                            row.product.clone(),
                            row.category.clone(),
                            row.price.clone(),
                            row.stock.clone(),
                        ];
                        self.edit_data.insert(*row_idx, row_data);
                        self.editing_rows.insert(*row_idx);
                        println!("Started editing row {}", row_idx);
                    }
                },
                RowAction::Save(row_idx) => {
                    if let Some(edit_data) = self.edit_data.get(row_idx) {
                        if let Some(row) = self.interactive_rows.get_mut(*row_idx) {
                            // Update the row with edited data
                            if edit_data.len() >= 4 {
                                row.product = edit_data[0].clone();
                                row.category = edit_data[1].clone();
                                row.price = edit_data[2].clone();
                                row.stock = edit_data[3].clone();
                            }
                        }
                    }
                    // Stop editing this row
                    self.editing_rows.remove(row_idx);
                    self.edit_data.remove(row_idx);
                    println!("Saved changes to row {}", row_idx);
                },
                RowAction::Cancel(row_idx) => {
                    // Stop editing this row without saving
                    self.editing_rows.remove(row_idx);
                    self.edit_data.remove(row_idx);
                    println!("Cancelled editing row {}", row_idx);
                },
                RowAction::Delete(row_idx) => {
                    // Remove the row and update indices
                    if *row_idx < self.interactive_rows.len() {
                        self.interactive_rows.remove(*row_idx);
                        if self.interactive_selection.len() > *row_idx {
                            self.interactive_selection.remove(*row_idx);
                        }
                        
                        // Update editing state - remove any references to this row
                        // and adjust indices for rows that come after
                        let mut new_editing_rows = HashSet::new();
                        let mut new_edit_data = HashMap::new();
                        
                        for (&editing_idx, data) in &self.edit_data {
                            if editing_idx < *row_idx {
                                // Keep rows before the deleted one
                                new_editing_rows.insert(editing_idx);
                                new_edit_data.insert(editing_idx, data.clone());
                            } else if editing_idx > *row_idx {
                                // Shift rows after the deleted one down by 1
                                new_editing_rows.insert(editing_idx - 1);
                                new_edit_data.insert(editing_idx - 1, data.clone());
                            }
                            // Skip the deleted row (editing_idx == *row_idx)
                        }
                        
                        self.editing_rows = new_editing_rows;
                        self.edit_data = new_edit_data;
                        println!("Deleted row {}", row_idx);
                    }
                },
            }
        }
        
        // Get current sort state from the data table response
        let (current_sort_col, current_sort_dir) = table_response.sort_state;
        
        // Update our local sort state to match the data table's internal state
        if let Some(sort_col_idx) = current_sort_col {
            let new_sort_column = match sort_col_idx {
                0 => Some(SortColumn::Product),
                1 => Some(SortColumn::Category),
                2 => Some(SortColumn::Price),
                3 => Some(SortColumn::Stock),
                _ => None, // Actions column or invalid
            };
            self.sort_column = new_sort_column;
            self.sort_direction = match current_sort_dir {
                DataTableSortDirection::Ascending => SortDirection::Ascending,
                DataTableSortDirection::Descending => SortDirection::Descending,
            };
        } else {
            self.sort_column = None;
        }
        
        // Sync the selection state back to our window state
        if table_response.selected_rows.len() == self.interactive_selection.len() {
            self.interactive_selection = table_response.selected_rows;
        }

        // Show editing status
        if !self.editing_rows.is_empty() {
            ui.add_space(10.0);
            ui.separator();
            ui.heading(format!("Currently editing {} row(s) - Edit values directly in the table cells above", self.editing_rows.len()));
        }

        // Additional features demonstration
        ui.add_space(20.0);
        ui.separator();
        ui.heading("Text Wrapping Example");
        
        let long_text_table = data_table()
            .id(Id::new("long_text_table"))
            .column("Short", 80.0, false)
            .column("Very Long Text Content That Should Wrap", 150.0, false)
            .column("Number", 80.0, true)
            .allow_selection(true)
            .row(|row| {
                row.cell("Item 1")
                   .cell("This is a very long text that should wrap to multiple lines when the content exceeds the available column width")
                   .cell("100")
            })
            .row(|row| {
                row.cell("Item 2")
                   .cell("Another extremely long piece of text content that demonstrates the text wrapping functionality in data table cells")
                   .cell("250")
            })
            .row(|row| {
                row.cell("Item 3")
                   .cell("Short text")
                   .cell("75")
            });
            
        ui.add(long_text_table);
        
        // Display current sorting state
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Current Sort:");
            if let Some(col) = &self.sort_column {
                let col_name = match col {
                    SortColumn::Product => "Product",
                    SortColumn::Category => "Category", 
                    SortColumn::Price => "Price",
                    SortColumn::Stock => "Stock",
                };
                let direction = match self.sort_direction {
                    SortDirection::Ascending => "↑",
                    SortDirection::Descending => "↓",
                };
                ui.label(format!("{} {}", col_name, direction));
            } else {
                ui.label("None");
            }
        });
        });
    }
}