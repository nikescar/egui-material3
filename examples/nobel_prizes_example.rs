use eframe::egui;
use egui_material3::{
    theme::{
        load_fonts, load_themes, setup_google_fonts, setup_local_fonts, setup_local_theme,
        update_global_theme, ContrastLevel, MaterialThemeContext, MaterialThemeFile, ThemeMode,
    },
    MaterialButton, MaterialDataTable,
};
use serde::Deserialize;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1400.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Nobel Prizes DataTable 테스트",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and themes
            setup_google_fonts(Some("Nanum Gothic"));
            // setup_local_fonts(Some("resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf"));
            setup_local_theme(Some("resources/material-theme2.json"));

            load_fonts(&cc.egui_ctx);
            load_themes();

            Ok(Box::<NobelPrizesApp>::default())
        }),
    )
}

#[derive(Debug, Clone, Deserialize)]
struct NobelPrizeData {
    title: String,
    subtitle: String,
    updated_at: String,
    data: Vec<YearlyPrizes>,
}

#[derive(Debug, Clone, Deserialize)]
struct YearlyPrizes {
    year: u32,
    categories: Vec<Category>,
}

#[derive(Debug, Clone, Deserialize)]
struct Category {
    name: String,
    full_title: String,
    awards: Vec<Award>,
}

#[derive(Debug, Clone, Deserialize)]
struct Award {
    laureates: Vec<Laureate>,
    motivation: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Laureate {
    name: String,
    profile_url: Option<String>,
}

#[derive(Debug, Clone)]
struct NobelPrizeEntry {
    year: String,
    category: String,
    laureate: String,
    motivation: String,
}

#[derive(Debug)]
enum DataState {
    Loading,
    Loaded(Vec<NobelPrizeEntry>),
    Error(String),
}

// Helper function to parse Nobel Prize data and flatten it into table entries
fn parse_nobel_data(data: &NobelPrizeData) -> Vec<NobelPrizeEntry> {
    let mut entries = Vec::new();

    for yearly_prizes in &data.data {
        for category in &yearly_prizes.categories {
            for award in &category.awards {
                for laureate in &award.laureates {
                    entries.push(NobelPrizeEntry {
                        year: yearly_prizes.year.to_string(),
                        category: category.name.clone(),
                        laureate: laureate.name.clone(),
                        motivation: award.motivation.clone(),
                    });
                }
            }
        }
    }

    entries
}

struct NobelPrizesApp {
    data_state: DataState,
    theme_loaded: bool,
    receiver: Option<std::sync::mpsc::Receiver<Result<Vec<NobelPrizeEntry>, String>>>,
}

impl Default for NobelPrizesApp {
    fn default() -> Self {
        let mut app = Self {
            data_state: DataState::Loading,
            theme_loaded: false,
            receiver: None,
        };

        // Fetch Nobel Prize data on startup
        app.fetch_nobel_data();

        app
    }
}

impl NobelPrizesApp {
    fn load_custom_theme(&mut self, file_path: &str) {
        match self.load_theme_from_file(file_path) {
            Ok(theme_file) => {
                let theme_context = MaterialThemeContext {
                    theme_mode: ThemeMode::Light,
                    contrast_level: ContrastLevel::Normal,
                    material_theme: Some(theme_file),
                    selected_colors: std::collections::HashMap::new(),
                };
                update_global_theme(theme_context);
                self.theme_loaded = true;
                println!("Custom theme loaded successfully!");
            }
            Err(e) => {
                eprintln!("Failed to load custom theme: {}", e);
                self.theme_loaded = false;
            }
        }
    }

    fn load_theme_from_file(
        &self,
        file_path: &str,
    ) -> Result<MaterialThemeFile, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let theme: MaterialThemeFile = serde_json::from_str(&content)?;
        Ok(theme)
    }

    fn fetch_nobel_data(&mut self) {
        // Spawn async task to fetch Nobel Prize data
        let (tx, rx) = std::sync::mpsc::channel();
        self.receiver = Some(rx);

        std::thread::spawn(move || {
            let url = "https://raw.githubusercontent.com/sharmadhiraj/free-json-datasets/refs/heads/master/docs/awards-achievements/nobel_prizes.json";

            match ureq::get(url).call() {
                Ok(response) => {
                    println!("HTTP request successful, parsing JSON...");
                    match response.into_json::<NobelPrizeData>() {
                        Ok(nobel_data) => {
                            println!("JSON parsed successfully");
                            let entries = parse_nobel_data(&nobel_data);
                            println!("Successfully parsed {} Nobel Prize entries", entries.len());
                            tx.send(Ok(entries)).ok();
                        }
                        Err(e) => {
                            println!("JSON parsing failed: {}", e);
                            tx.send(Err(format!("JSON parsing error: {}", e))).ok();
                        }
                    }
                }
                Err(e) => {
                    println!("HTTP request failed: {}", e);
                    tx.send(Err(format!("HTTP request error: {}", e))).ok();
                }
            }
        });
    }

    fn create_nobel_datatable(&self, prizes: &[NobelPrizeEntry]) -> MaterialDataTable<'_> {
        let mut datatable = MaterialDataTable::new()
            .id("nobel_prizes_table")
            .column("Year", 80.0, false)
            .column("Category", 120.0, false)
            .column("Laureate", 200.0, false)
            .column("Motivation", 400.0, false)
            .allow_selection(true)
            .sticky_header(true)
            .corner_radius(8.0);

        for prize in prizes {
            datatable = datatable.row(|row| {
                row.cell(&prize.year)
                    .cell(&prize.category)
                    .cell(&prize.laureate)
                    .cell(&prize.motivation)
            });
        }

        datatable
    }
}

impl eframe::App for NobelPrizesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for HTTP response
        if let Some(receiver) = &self.receiver {
            if let Ok(result) = receiver.try_recv() {
                self.data_state = match result {
                    Ok(prizes) => DataState::Loaded(prizes),
                    Err(e) => DataState::Error(e),
                };
                self.receiver = None; // Clear receiver after getting response
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nobel Prizes Database (1901-2024) 테스트");

            ui.add_space(10.0);

            // Theme status
            ui.horizontal(|ui| {
                ui.label("Theme Status:");
                if self.theme_loaded {
                    ui.colored_label(
                        egui::Color32::GREEN,
                        "✓ Custom theme loaded (material-theme2.json)",
                    );
                } else {
                    ui.colored_label(egui::Color32::RED, "✗ Using default theme");
                }
            });

            ui.add_space(10.0);

            // Refresh button
            if ui.add(MaterialButton::new("Refresh Data")).clicked() {
                self.data_state = DataState::Loading;
                self.receiver = None; // Clear any existing receiver
                self.fetch_nobel_data();
            }

            ui.add_space(20.0);

            // Display data table based on state
            match &self.data_state {
                DataState::Loading => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Loading Nobel Prize data...");
                    });
                }
                DataState::Loaded(prizes) => {
                    if prizes.is_empty() {
                        ui.label("No Nobel Prize data found.");
                        ui.add_space(10.0);
                        if ui.add(MaterialButton::new("Try Again")).clicked() {
                            self.data_state = DataState::Loading;
                            self.receiver = None;
                            self.fetch_nobel_data();
                        }
                    } else {
                        ui.label(format!("Showing {} Nobel Prize entries:", prizes.len()));
                        ui.add_space(10.0);

                        let datatable = self.create_nobel_datatable(prizes);
                        let response = datatable.show(ui);

                        // Handle any table interactions if needed
                        if !response.row_actions.is_empty() {
                            println!("Table actions: {:?}", response.row_actions);
                        }

                        ui.add_space(10.0);
                        ui.label("Data source: GitHub - Nobel Prizes Dataset (1901-2024)");
                    }
                }
                DataState::Error(error) => {
                    ui.colored_label(egui::Color32::RED, format!("Error loading data: {}", error));

                    ui.add_space(10.0);

                    if ui.add(MaterialButton::new("Retry")).clicked() {
                        self.data_state = DataState::Loading;
                        self.receiver = None; // Clear any existing receiver
                        self.fetch_nobel_data();
                    }
                }
            }
        });

        // Request repaint for loading animation
        if matches!(self.data_state, DataState::Loading) {
            ctx.request_repaint();
        }
    }
}
