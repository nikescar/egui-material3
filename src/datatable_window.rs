use eframe::egui::{self, Ui, Window};
use crate::{MaterialButton, MaterialCheckbox, data_table};

pub struct DataTableWindow {
    pub open: bool,
    allow_selection: bool,
    sticky_header: bool,
    show_progress: bool,
    selected_rows: Vec<bool>,
}

impl Default for DataTableWindow {
    fn default() -> Self {
        Self {
            open: false,
            allow_selection: true,
            sticky_header: false,
            show_progress: false,
            selected_rows: vec![false, true, true, false, false],
        }
    }
}

impl DataTableWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Data Table Stories")
            .open(&mut open)
            .default_size([1000.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_data_table_examples(ui);
                });
            });
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
                ui.add(MaterialCheckbox::new(&mut self.allow_selection, "Allow Selection"));
                ui.add(MaterialCheckbox::new(&mut self.sticky_header, "Sticky Header"));
                ui.add(MaterialCheckbox::new(&mut self.show_progress, "Show Progress"));
            });
        });
    }

    fn render_data_table_examples(&mut self, ui: &mut Ui) {
        ui.heading("Basic Data Table");
        
        let mut basic_table = data_table()
            .column("Dessert", 180.0, false)
            .column("Carbs (g)", 100.0, true)
            .column("Protein (g)", 100.0, true)
            .column("Comments", 200.0, false)
            .row(|row| {
                row.cell("Frozen yogurt")
                   .cell("24")
                   .cell("4.0")
                   .cell("Super tasty")
            })
            .row(|row| {
                row.cell("Ice cream sandwich")
                   .cell("37")
                   .cell("4.33")
                   .cell("I like ice cream more")
            })
            .row(|row| {
                row.cell("Eclair")
                   .cell("24")
                   .cell("6.0")
                   .cell("New filing flavor")
            })
            .row(|row| {
                row.cell("Cupcake")
                   .cell("67")
                   .cell("4.3")
                   .cell("Very sweet")
            })
            .row(|row| {
                row.cell("Jelly bean")
                   .cell("0")
                   .cell("0.0")
                   .cell("Colorful candy")
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
            })
            .row(|row| {
                row.cell("monarch: prod shared ares-managed-features-provider-heavy")
                   .cell("Offline")
                   .cell("Huge")
                   .cell("Triaged")
                   .cell("0:33")
                   .cell("Brie Larson")
                   .selected(true)
            })
            .row(|row| {
                row.cell("monarch: prod shared ares-managed-features-provider-heavy")
                   .cell("Online")
                   .cell("Minor")
                   .cell("Not triaged")
                   .cell("0:33")
                   .cell("Jeremy Lake")
                   .selected(true)
            })
            .row(|row| {
                row.cell("Arcus watch slowdown")
                   .cell("Online")
                   .cell("Negligible")
                   .cell("Triaged")
                   .cell("0:33")
                   .cell("Angelina Cheng")
            });

        ui.add(sortable_table);

        ui.add_space(20.0);
        
        ui.heading("Compact Data Table");
        
        let compact_table = data_table()
            .column("ID", 60.0, true)
            .column("Name", 120.0, false)
            .column("Status", 80.0, false)
            .column("Progress", 100.0, true)
            .row(|row| {
                row.cell("001")
                   .cell("Task Alpha")
                   .cell("Active")
                   .cell("75%")
            })
            .row(|row| {
                row.cell("002")
                   .cell("Task Beta")
                   .cell("Pending")
                   .cell("25%")
            })
            .row(|row| {
                row.cell("003")
                   .cell("Task Gamma")
                   .cell("Complete")
                   .cell("100%")
            });

        ui.add(compact_table);

        ui.add_space(20.0);
        
        ui.heading("Interactive Data Table Demo");
        
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Add Row")).clicked() {
                println!("Add row clicked!");
            }
            if ui.add(MaterialButton::outlined("Delete Selected")).clicked() {
                println!("Delete selected clicked!");
            }
            if ui.add(MaterialButton::text("Export")).clicked() {
                println!("Export clicked!");
            }
        });

        ui.add_space(10.0);
        
        let interactive_table = data_table()
            .column("Product", 150.0, false)
            .column("Category", 120.0, false)
            .sortable_column("Price", 100.0, true)
            .sortable_column("Stock", 80.0, true)
            .column("Actions", 120.0, false)
            .allow_selection(true)
            .row(|row| {
                row.cell("Laptop")
                   .cell("Electronics")
                   .cell("$999")
                   .cell("15")
                   .cell("Edit | Delete")
            })
            .row(|row| {
                row.cell("Book")
                   .cell("Education")
                   .cell("$29")
                   .cell("50")
                   .cell("Edit | Delete")
            })
            .row(|row| {
                row.cell("Shirt")
                   .cell("Clothing")
                   .cell("$49")
                   .cell("100")
                   .cell("Edit | Delete")
            });

        ui.add(interactive_table);
    }
}