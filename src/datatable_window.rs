use eframe::egui::{self, Ui, Window, Id};
use crate::{MaterialButton, MaterialCheckbox, data_table};
use egui::TextEdit;

#[derive(Clone, Debug)]
struct TableRow {
    product: String,
    category: String,
    price: String,
    stock: String,
    selected: bool,
}

#[derive(Clone, Debug)]
enum EditingState {
    None,
    Editing(usize), // Index of the row being edited
}

pub struct DataTableWindow {
    pub open: bool,
    allow_selection: bool,
    sticky_header: bool,
    show_progress: bool,
    selected_rows: Vec<bool>,
    // Interactive table data
    interactive_rows: Vec<TableRow>,
    // Track selection state for interactive table
    interactive_selection: Vec<bool>,
    // Track editing state
    editing_state: EditingState,
    // Temporary storage for editing
    edit_product: String,
    edit_category: String,
    edit_price: String,
    edit_stock: String,
}

impl Default for DataTableWindow {
    fn default() -> Self {
        let interactive_rows = vec![
            TableRow {
                product: "Laptop".to_string(),
                category: "Electronics".to_string(),
                price: "$999".to_string(),
                stock: "15".to_string(),
                selected: false,
            },
            TableRow {
                product: "Book".to_string(),
                category: "Education".to_string(),
                price: "$29".to_string(),
                stock: "50".to_string(),
                selected: false,
            },
            TableRow {
                product: "Shirt".to_string(),
                category: "Clothing".to_string(),
                price: "$49".to_string(),
                stock: "100".to_string(),
                selected: false,
            },
            TableRow {
                product: "Phone".to_string(),
                category: "Electronics".to_string(),
                price: "$799".to_string(),
                stock: "25".to_string(),
                selected: false,
            },
            TableRow {
                product: "Headphones".to_string(),
                category: "Electronics".to_string(),
                price: "$199".to_string(),
                stock: "40".to_string(),
                selected: false,
            },
        ];
        let interactive_selection = vec![false; interactive_rows.len()];
        
        Self {
            open: false,
            allow_selection: true,
            sticky_header: false,
            show_progress: false,
            selected_rows: vec![false, true, true, false, false],
            interactive_rows,
            interactive_selection,
            editing_state: EditingState::None,
            edit_product: String::new(),
            edit_category: String::new(),
            edit_price: String::new(),
            edit_stock: String::new(),
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
                    selected: false,
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
                    self.editing_state = EditingState::None; // Cancel any ongoing edit
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
        
        // Use the proper data table widget for Interactive Data Table Demo
        let mut interactive_table = data_table()
            .id(Id::new("interactive_data_table"))
            .column("Product", 150.0, false)
            .column("Category", 120.0, false)
            .sortable_column("Price", 100.0, true)
            .sortable_column("Stock", 80.0, true)
            .column("Actions", 120.0, false)
            .allow_selection(true);

        // Add rows dynamically from our state
        for (idx, row) in self.interactive_rows.iter().enumerate() {
            let is_selected = self.interactive_selection.get(idx).copied().unwrap_or(false);
            
            // Check if this row is being edited
            let is_editing = matches!(self.editing_state, EditingState::Editing(edit_idx) if edit_idx == idx);
            
            let actions_text = if is_editing {
                "Save | Cancel"
            } else {
                "Edit | Delete"
            };
            
            interactive_table = interactive_table.row(|table_row| {
                let mut row_builder = table_row
                    .cell(&row.product)
                    .cell(&row.category)
                    .cell(&row.price)
                    .cell(&row.stock)
                    .cell(actions_text)
                    .id(format!("interactive_table_row_{}", idx));
                
                if is_selected {
                    row_builder = row_builder.selected(true);
                }
                
                row_builder
            });
        }

        // Show the table and get the selection state back
        let table_response = interactive_table.show(ui);
        
        // Sync the selection state back to our window state
        if table_response.selected_rows.len() == self.interactive_selection.len() {
            self.interactive_selection = table_response.selected_rows;
        }

        // Handle clicking on Actions column for individual row actions
        // This is a workaround since we can't easily detect column-specific clicks
        // We'll place action buttons below the table for now, but make them row-specific
        if !matches!(self.editing_state, EditingState::Editing(_)) {
            ui.add_space(10.0);
            ui.label("Row Actions (Click to Edit/Delete specific rows):");
            
            ui.horizontal_wrapped(|ui| {
                let row_count = self.interactive_rows.len();
                let mut row_to_edit = None;
                let mut row_to_delete = None;
                
                for idx in 0..row_count {
                    ui.group(|ui| {
                        ui.label(&format!("Row {}: {}", idx + 1, self.interactive_rows[idx].product));
                        ui.horizontal(|ui| {
                            if ui.small_button("Edit").clicked() {
                                row_to_edit = Some(idx);
                            }
                            if ui.small_button("Delete").clicked() {
                                row_to_delete = Some(idx);
                            }
                        });
                    });
                }
                
                // Handle edit action
                if let Some(idx) = row_to_edit {
                    if let Some(row) = self.interactive_rows.get(idx) {
                        self.edit_product = row.product.clone();
                        self.edit_category = row.category.clone();
                        self.edit_price = row.price.clone();
                        self.edit_stock = row.stock.clone();
                        self.editing_state = EditingState::Editing(idx);
                        println!("Started editing row {}", idx);
                    }
                }
                
                // Handle delete action
                if let Some(idx) = row_to_delete {
                    self.interactive_rows.remove(idx);
                    if self.interactive_selection.len() > idx {
                        self.interactive_selection.remove(idx);
                    }
                    if matches!(self.editing_state, EditingState::Editing(edit_idx) if edit_idx == idx) {
                        self.editing_state = EditingState::None;
                    }
                    println!("Deleted row {}", idx);
                }
            });
        }

        // Show edit form below the table if editing
        if let EditingState::Editing(edit_idx) = self.editing_state {
            if let Some(_row) = self.interactive_rows.get(edit_idx) {
                ui.add_space(10.0);
                ui.separator();
                ui.heading("Edit Row");
                
                ui.horizontal(|ui| {
                    ui.label("Product:");
                    ui.add(TextEdit::singleline(&mut self.edit_product).desired_width(120.0));
                    
                    ui.label("Category:");
                    ui.add(TextEdit::singleline(&mut self.edit_category).desired_width(100.0));
                    
                    ui.label("Price:");
                    ui.add(TextEdit::singleline(&mut self.edit_price).desired_width(80.0));
                    
                    ui.label("Stock:");
                    ui.add(TextEdit::singleline(&mut self.edit_stock).desired_width(60.0));
                });
                
                ui.horizontal(|ui| {
                    if ui.add(MaterialButton::filled("Save")).clicked() {
                        // Save changes
                        if let Some(row) = self.interactive_rows.get_mut(edit_idx) {
                            row.product = self.edit_product.clone();
                            row.category = self.edit_category.clone();
                            row.price = self.edit_price.clone();
                            row.stock = self.edit_stock.clone();
                        }
                        self.editing_state = EditingState::None;
                        println!("Saved changes to row {}", edit_idx);
                    }
                    
                    if ui.add(MaterialButton::outlined("Cancel")).clicked() {
                        self.editing_state = EditingState::None;
                        println!("Cancelled editing");
                    }
                });
            }
        }
        });
    }
}