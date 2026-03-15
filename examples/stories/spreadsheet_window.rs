#![doc(hidden)]

use eframe::egui::{self, Ui, Window};

#[cfg(feature = "spreadsheet")]
use crate::{MaterialButton, MaterialCheckbox};
#[cfg(feature = "spreadsheet")]
use egui_material3::spreadsheet::{
    integer_column, number_column, text_column, MaterialSpreadsheet,
};

#[doc(hidden)]
pub struct SpreadsheetWindow {
    pub open: bool,
    #[cfg(feature = "spreadsheet")]
    spreadsheet: Option<MaterialSpreadsheet>,
    #[cfg(feature = "spreadsheet")]
    allow_editing: bool,
    #[cfg(feature = "spreadsheet")]
    allow_selection: bool,
    #[cfg(feature = "spreadsheet")]
    striped: bool,
    #[cfg(feature = "spreadsheet")]
    show_file_dialog: bool,
    #[cfg(feature = "spreadsheet")]
    file_dialog: egui_file_dialog::FileDialog,
    #[cfg(feature = "spreadsheet")]
    pending_action: Option<PendingAction>,
    #[cfg(feature = "spreadsheet")]
    last_operation: String,
}

#[cfg(feature = "spreadsheet")]
#[derive(Clone, Debug)]
enum PendingAction {
    LoadCsv,
    SaveCsv,
    LoadParquet,
    SaveParquet,
}

impl Default for SpreadsheetWindow {
    fn default() -> Self {
        #[cfg(feature = "spreadsheet")]
        {
            // Create sample spreadsheet with demo data
            let columns = vec![
                text_column("Product", 150.0),
                text_column("Category", 120.0),
                number_column("Price", 100.0),
                integer_column("Stock", 80.0),
                text_column("Supplier", 150.0),
            ];

            let mut spreadsheet = MaterialSpreadsheet::new("demo_spreadsheet", columns)
                .expect("Failed to create spreadsheet");

            // Add sample data - initialize with pre-populated rows
            spreadsheet.init_with_data(vec![
                vec!["Laptop".to_string(), "Electronics".to_string(), "999.99".to_string(), "15".to_string(), "TechCorp".to_string()],
                vec!["Mouse".to_string(), "Electronics".to_string(), "29.99".to_string(), "150".to_string(), "TechCorp".to_string()],
                vec!["Keyboard".to_string(), "Electronics".to_string(), "79.99".to_string(), "85".to_string(), "KeyMaster".to_string()],
                vec!["Monitor".to_string(), "Electronics".to_string(), "299.99".to_string(), "42".to_string(), "ScreenPro".to_string()],
                vec!["Desk Chair".to_string(), "Furniture".to_string(), "199.99".to_string(), "30".to_string(), "ComfortSeats".to_string()],
                vec!["Desk".to_string(), "Furniture".to_string(), "349.99".to_string(), "20".to_string(), "OfficePro".to_string()],
                vec!["Notebook".to_string(), "Stationery".to_string(), "5.99".to_string(), "500".to_string(), "PaperGoods".to_string()],
                vec!["Pen Pack".to_string(), "Stationery".to_string(), "12.99".to_string(), "300".to_string(), "WriteWell".to_string()],
            ]);

            Self {
                open: false,
                spreadsheet: Some(spreadsheet),
                allow_editing: true,
                allow_selection: true,
                striped: true,
                show_file_dialog: false,
                file_dialog: egui_file_dialog::FileDialog::new(),
                pending_action: None,
                last_operation: String::new(),
            }
        }
        #[cfg(not(feature = "spreadsheet"))]
        {
            Self {
                open: false,
            }
        }
    }
}

impl SpreadsheetWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        let mut should_close = false;

        Window::new("Spreadsheet Stories")
            .open(&mut open)
            .default_size([1200.0, 700.0])
            .show(ctx, |ui| {
                // Check for ESC key press to close window
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    should_close = true;
                }

                #[cfg(feature = "spreadsheet")]
                {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.render_controls(ui);
                        ui.add_space(20.0);
                        self.render_spreadsheet(ui);
                    });
                }

                #[cfg(not(feature = "spreadsheet"))]
                {
                    ui.heading("Spreadsheet Feature Not Enabled");
                    ui.label("Please enable the 'spreadsheet' feature to use this component.");
                    ui.label("Add to Cargo.toml:");
                    ui.code("egui-material3 = { features = [\"spreadsheet\"] }");
                }
            });

        if should_close {
            open = false;
        }
        self.open = open;
    }

    #[cfg(feature = "spreadsheet")]
    fn render_controls(&mut self, ui: &mut Ui) {
        ui.push_id("spreadsheet_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Spreadsheet Controls");

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
                    let _ = webbrowser::open("https://m3.material.io/components/data-tables");
                }
            });

            ui.horizontal(|ui| {
                ui.push_id("allow_editing_control", |ui| {
                    if ui.add(MaterialCheckbox::new(&mut self.allow_editing, "Allow Editing")).changed() {
                        if let Some(ref mut spreadsheet) = self.spreadsheet {
                            spreadsheet.set_allow_editing(self.allow_editing);
                        }
                    }
                });

                ui.add_space(10.0);

                ui.push_id("allow_selection_control", |ui| {
                    if ui.add(MaterialCheckbox::new(&mut self.allow_selection, "Allow Selection")).changed() {
                        if let Some(ref mut spreadsheet) = self.spreadsheet {
                            spreadsheet.set_allow_selection(self.allow_selection);
                        }
                    }
                });

                ui.add_space(10.0);

                ui.push_id("striped_control", |ui| {
                    if ui.add(MaterialCheckbox::new(&mut self.striped, "Striped Rows")).changed() {
                        if let Some(ref mut spreadsheet) = self.spreadsheet {
                            spreadsheet.set_striped(self.striped);
                        }
                    }
                });
            });

            ui.add_space(10.0);

            // Action buttons - CSV
            ui.horizontal(|ui| {
                ui.label("CSV:");

                if ui.add(MaterialButton::outlined("Load CSV")).clicked() {
                    self.pending_action = Some(PendingAction::LoadCsv);
                    self.file_dialog.pick_file();
                }

                ui.add_space(10.0);

                if ui.add(MaterialButton::outlined("Save CSV")).clicked() {
                    self.pending_action = Some(PendingAction::SaveCsv);
                    self.file_dialog.save_file();
                }
            });

            ui.add_space(10.0);

            // Action buttons - Parquet
            ui.horizontal(|ui| {
                ui.label("Parquet:");

                if ui.add(MaterialButton::outlined("Load Parquet")).clicked() {
                    self.pending_action = Some(PendingAction::LoadParquet);
                    self.file_dialog.pick_file();
                }

                ui.add_space(10.0);

                if ui.add(MaterialButton::outlined("Save Parquet")).clicked() {
                    self.pending_action = Some(PendingAction::SaveParquet);
                    self.file_dialog.save_file();
                }
            });

            // Handle file dialog - using the correct API pattern
            if let Some(path) = self.file_dialog.update(ui.ctx()).picked() {
                match self.pending_action.take() {
                    Some(PendingAction::LoadCsv) => {
                        if let Some(ref mut spreadsheet) = self.spreadsheet {
                            self.last_operation = format!("Loading CSV from: {}", path.display());
                            eprintln!("Loading CSV from: {}", path.display());
                            spreadsheet.load_from_file(path.to_path_buf());
                            ui.ctx().request_repaint();
                        }
                    }
                    Some(PendingAction::LoadParquet) => {
                        if let Some(ref mut spreadsheet) = self.spreadsheet {
                            self.last_operation = format!("Loading Parquet from: {}", path.display());
                            eprintln!("Loading Parquet from: {}", path.display());
                            spreadsheet.load_from_file(path.to_path_buf());
                            ui.ctx().request_repaint();
                        }
                    }
                    Some(PendingAction::SaveCsv) => {
                        if let Some(ref mut spreadsheet) = self.spreadsheet {
                            self.last_operation = format!("Saving CSV to: {}", path.display());
                            eprintln!("Saving CSV to: {}", path.display());
                            spreadsheet.save_to_file(path.to_path_buf());
                            ui.ctx().request_repaint();
                        }
                    }
                    Some(PendingAction::SaveParquet) => {
                        if let Some(ref mut spreadsheet) = self.spreadsheet {
                            self.last_operation = format!("Saving Parquet to: {}", path.display());
                            eprintln!("Saving Parquet to: {}", path.display());
                            spreadsheet.save_to_file(path.to_path_buf());
                            ui.ctx().request_repaint();
                        }
                    }
                    None => {}
                }
            }

            // Show last operation status
            if !self.last_operation.is_empty() {
                ui.add_space(10.0);
                ui.label(&self.last_operation);
            }
        });
    }

    #[cfg(feature = "spreadsheet")]
    fn render_spreadsheet(&mut self, ui: &mut Ui) {
        ui.heading("Material Spreadsheet");
        ui.add_space(10.0);

        if let Some(spreadsheet) = &mut self.spreadsheet {
            let columns_count = spreadsheet.columns().len();
            let rows_count = spreadsheet.rows().len();

            ui.group(|ui| {
                ui.set_min_height(400.0);
                spreadsheet.show(ui);
            });

            ui.add_space(10.0);

            // Show statistics
            ui.horizontal(|ui| {
                ui.label(format!("Total rows: {}", rows_count));
                ui.add_space(20.0);
                ui.label(format!("Total columns: {}", columns_count));
            });
        }
    }
}
