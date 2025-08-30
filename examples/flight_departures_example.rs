use eframe::egui;
use egui_material::{
    MaterialDataTable, MaterialButton,
    theme::{setup_google_fonts, setup_local_fonts, setup_local_theme, load_fonts, load_themes, MaterialThemeFile, MaterialThemeContext, ThemeMode, ContrastLevel, update_global_theme},
};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1400.0, 900.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Incheon Airport Flight Departures DataTable",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and themes
            setup_google_fonts(Some("Nanum Gothic"));
            setup_local_fonts(Some("resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf"));
            setup_local_theme(Some("resources/material-theme2.json"));

            load_fonts(&cc.egui_ctx);
            load_themes();
            
            Ok(Box::<FlightDeparturesApp>::default())
        }),
    )
}

#[derive(Debug, Clone)]
struct SimpleFlightData {
    flight_number: String,
    departure_time: String,
    destination: String,
    airline: String,
    aircraft: String,
    status: String,
}

#[derive(Debug)]
enum DataState {
    Loading,
    Loaded(Vec<SimpleFlightData>),
    Error(String),
}

// Helper function to parse flight data from the API response with flexible parsing
fn parse_flight_item(item: &serde_json::Value) -> SimpleFlightData {
    // Print item for debugging (limit output to avoid spam)
    if let Ok(pretty_item) = serde_json::to_string_pretty(item) {
        let preview = &pretty_item[..std::cmp::min(800, pretty_item.len())];
        println!("Parsing item: {}", preview);
        if pretty_item.len() > 800 {
            println!("... (truncated)");
        }
    }
    
    // Try flight number - first check if data is nested under "flight" object
    let flight_obj = item.get("flight");
    let mut flight_number = flight_obj
        .and_then(|f| f.get("identification"))
        .and_then(|i| i.get("number"))
        .and_then(|n| n.get("default"))
        .and_then(|d| d.as_str())
        .map(|s| s.to_string());
    
    // If not found, try to construct from airline code + flight number
    if flight_number.is_none() {
        if let (Some(airline_code), Some(flight_num)) = (
            flight_obj
                .and_then(|f| f.get("airline"))
                .and_then(|a| a.get("code"))
                .and_then(|c| c.get("iata"))
                .and_then(|i| i.as_str()),
            flight_obj
                .and_then(|f| f.get("number"))
                .and_then(|n| n.as_str())
                .or_else(|| item.get("flight_number").and_then(|n| n.as_str()))
        ) {
            flight_number = Some(format!("{}{}", airline_code, flight_num));
        }
    }
    
    // Final fallback
    let flight_number = flight_number
        .or_else(|| item.get("callsign").and_then(|c| c.as_str()).map(|s| s.to_string()))
        .unwrap_or_else(|| "N/A".to_string());
    
    // Try departure time - check nested under "flight" object first
    let departure_timestamp = flight_obj
        .and_then(|f| f.get("time"))
        .and_then(|t| t.get("scheduled"))
        .and_then(|s| s.get("departure"))
        .and_then(|d| d.as_i64())
        .or_else(|| {
            // Fallback: Try at root level
            item.get("time")
                .and_then(|t| t.get("scheduled"))
                .and_then(|s| s.get("departure"))
                .and_then(|d| d.as_i64())
        });
        
    
    let departure_time = match departure_timestamp {
        Some(ts) => {
            // Convert timestamp (handle both seconds and milliseconds)
            let ts_seconds = if ts > 10_000_000_000 { ts / 1000 } else { ts };
            
            // Use UTC time but show just HH:MM format
            use std::time::{SystemTime, UNIX_EPOCH, Duration};
            if let Some(datetime) = SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(ts_seconds as u64)) {
                // Simple calculation for hours and minutes
                let total_seconds = ts_seconds % 86400; // seconds in current day
                let hours = (total_seconds / 3600) as u8;
                let minutes = ((total_seconds % 3600) / 60) as u8;
                format!("{:02}:{:02}", hours, minutes)
            } else {
                "N/A".to_string()
            }
        }
        None => {
            // Try string format time fields
            item.get("scheduled_time")
                .and_then(|t| t.as_str())
                .or_else(|| item.get("departure_time").and_then(|t| t.as_str()))
                .or_else(|| item.get("time").and_then(|t| t.as_str()))
                .unwrap_or("N/A")
                .to_string()
        }
    };
    
    // Try destination - check nested under "flight" object first
    let destination = flight_obj
        .and_then(|f| f.get("airport"))
        .and_then(|a| a.get("destination"))
        .and_then(|d| d.get("name"))
        .and_then(|n| n.as_str())
        .or_else(|| {
            // Fallback: Try at root level
            item.get("airport")
                .and_then(|a| a.get("destination"))
                .and_then(|d| d.get("name"))
                .and_then(|n| n.as_str())
        })
        .unwrap_or("N/A")
        .to_string();
    
    // Try airline - check nested under "flight" object first  
    let airline = flight_obj
        .and_then(|f| f.get("airline"))
        .and_then(|a| a.get("name"))
        .and_then(|n| n.as_str())
        .or_else(|| {
            // Fallback: Try at root level
            item.get("airline")
                .and_then(|a| a.get("name"))
                .and_then(|n| n.as_str())
        })
        .unwrap_or("N/A")
        .to_string();
    
    // Try aircraft - check nested under "flight" object first
    let aircraft = flight_obj
        .and_then(|f| f.get("aircraft"))
        .and_then(|a| a.get("model"))
        .and_then(|m| m.get("text"))
        .and_then(|t| t.as_str())
        .or_else(|| {
            // Fallback: Try aircraft model code if text is empty
            flight_obj
                .and_then(|f| f.get("aircraft"))
                .and_then(|a| a.get("model"))
                .and_then(|m| m.get("code"))
                .and_then(|c| c.as_str())
        })
        .or_else(|| {
            // Fallback: Try at root level
            item.get("aircraft")
                .and_then(|a| a.get("model"))
                .and_then(|m| m.get("text"))
                .and_then(|t| t.as_str())
        })
        .unwrap_or("N/A")
        .to_string();
    
    // Try status - check nested under "flight" object first
    let status = flight_obj
        .and_then(|f| f.get("status"))
        .and_then(|s| s.get("text"))
        .and_then(|t| t.as_str())
        .or_else(|| {
            // Fallback: Try at root level
            item.get("status")
                .and_then(|s| s.get("text"))
                .and_then(|t| t.as_str())
        })
        .or_else(|| {
            // Fallback: Try status as direct string
            item.get("status").and_then(|s| s.as_str())
        })
        .unwrap_or("N/A")
        .to_string();
    
    let result = SimpleFlightData {
        flight_number: flight_number.clone(),
        departure_time: departure_time.clone(),
        destination: destination.clone(),
        airline: airline.clone(),
        aircraft: aircraft.clone(),
        status: status.clone(),
    };
    
    println!("Parsed: {} | {} | {} | {} | {} | {}", 
             result.flight_number, result.departure_time, result.destination, 
             result.airline, result.aircraft, result.status);
             
    // Additional debug info
    println!("Debug - Found flight object: {}", flight_obj.is_some());
    if let Some(f) = flight_obj {
        println!("Debug - Airline in flight: {}", f.get("airline").is_some());
        if let Some(airline_obj) = f.get("airline") {
            println!("Debug - Airline name: {:?}", airline_obj.get("name"));
        }
    }
    
    result
}

struct FlightDeparturesApp {
    data_state: DataState,
    theme_loaded: bool,
    receiver: Option<std::sync::mpsc::Receiver<Result<Vec<SimpleFlightData>, String>>>,
}

impl Default for FlightDeparturesApp {
    fn default() -> Self {
        let mut app = Self {
            data_state: DataState::Loading,
            theme_loaded: false,
            receiver: None,
        };
        
        // Fetch flight data on startup
        app.fetch_flight_data();
        
        app
    }
}

impl FlightDeparturesApp {
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
    
    fn load_theme_from_file(&self, file_path: &str) -> Result<MaterialThemeFile, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let theme: MaterialThemeFile = serde_json::from_str(&content)?;
        Ok(theme)
    }
    
    fn fetch_flight_data(&mut self) {
        // Spawn async task to fetch flight data from FlightRadar24 API
        let (tx, rx) = std::sync::mpsc::channel();
        self.receiver = Some(rx);
        
        std::thread::spawn(move || {
            let url = "https://api.flightradar24.com/common/v1/airport.json?code=icn&plugin[]=&plugin-setting[schedule][mode]=&plugin-setting[schedule][timestamp]=1756564415&page=1&limit=100&fleet=&token=";
            
            // Create request with Firefox user agent
            let request = ureq::get(url)
                .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0")
                .set("Accept", "application/json, text/plain, */*")
                .set("Accept-Language", "en-US,en;q=0.5")
                .set("Accept-Encoding", "gzip, deflate, br, zstd")
                .set("Referer", "https://www.flightradar24.com/")
                .set("Origin", "https://www.flightradar24.com")
                .set("Connection", "keep-alive")
                .set("Sec-Fetch-Dest", "empty")
                .set("Sec-Fetch-Mode", "cors")
                .set("Sec-Fetch-Site", "same-site")
                .set("TE", "trailers");
            
            match request.call() {
                Ok(response) => {
                    println!("HTTP request successful, parsing JSON...");
                    match response.into_json::<serde_json::Value>() {
                        Ok(json) => {
                            println!("JSON parsed successfully");
                            
                            // Debug: Print the JSON structure to understand the format
                            println!("API Response JSON structure:");
                            if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                                let preview = &pretty_json[..std::cmp::min(2000, pretty_json.len())];
                                println!("{}", preview);
                                if pretty_json.len() > 2000 {
                                    println!("... (truncated, total length: {} characters)", pretty_json.len());
                                }
                            }
                            
                            // Try to parse the response structure
                            let mut flights = Vec::new();
                            
                            // Look for the schedule data in the nested structure
                            println!("Attempting to navigate JSON structure...");
                            if let Some(result) = json.get("result") {
                                println!("Found 'result' field");
                                if let Some(response) = result.get("response") {
                                    println!("Found 'response' field");
                                    if let Some(airport) = response.get("airport") {
                                        println!("Found 'airport' field");
                                        if let Some(plugin_data) = airport.get("pluginData") {
                                            println!("Found 'pluginData' field");
                                            if let Some(schedule) = plugin_data.get("schedule") {
                                                println!("Found 'schedule' field");
                                                if let Some(departures) = schedule.get("departures") {
                                                    println!("Found 'departures' field");
                                                    if let Some(data) = departures.get("data") {
                                                        if let Some(data_array) = data.as_array() {
                                                            println!("Found flight data array with {} items", data_array.len());
                                                            for (i, item) in data_array.iter().enumerate() {
                                                                println!("Processing flight item {}", i);
                                                                flights.push(parse_flight_item(item));
                                                            }
                                                        } else {
                                                            println!("'data' field is not an array");
                                                        }
                                                    } else {
                                                        println!("No 'data' field in departures");
                                                    }
                                                } else {
                                                    println!("No 'departures' field in schedule");
                                                }
                                            } else {
                                                println!("No 'schedule' field in pluginData");
                                            }
                                        } else {
                                            println!("No 'pluginData' field in airport");
                                        }
                                    } else {
                                        println!("No 'airport' field in response");
                                    }
                                } else {
                                    println!("No 'response' field in result");
                                }
                            } else {
                                println!("No 'result' field in JSON");
                            }
                            
                            println!("Successfully parsed {} real flights from API", flights.len());
                            tx.send(Ok(flights)).ok();
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
    
    fn create_flights_datatable(&self, flights: &[SimpleFlightData]) -> MaterialDataTable {
        let mut datatable = MaterialDataTable::new()
            .id("flights_departures_table")
            .column("Time", 80.0, false)
            .column("Flight", 100.0, false)
            .column("To", 200.0, false)
            .column("Airline", 180.0, false)
            .column("Aircraft", 180.0, false)
            .column("Status", 100.0, false)
            .allow_selection(true)
            .sticky_header(true)
            .corner_radius(8.0);
        
        for flight in flights {
            datatable = datatable.row(|row| {
                row.cell(&flight.departure_time)
                   .cell(&flight.flight_number)
                   .cell(&flight.destination)
                   .cell(&flight.airline)
                   .cell(&flight.aircraft)
                   .cell(&flight.status)
            });
        }
        
        datatable
    }
}

impl eframe::App for FlightDeparturesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for HTTP response
        if let Some(receiver) = &self.receiver {
            if let Ok(result) = receiver.try_recv() {
                self.data_state = match result {
                    Ok(flights) => DataState::Loaded(flights),
                    Err(e) => DataState::Error(e),
                };
                self.receiver = None; // Clear receiver after getting response
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Incheon Airport (ICN) - Flight Departures");
            
            ui.add_space(10.0);
            
            // Theme status
            ui.horizontal(|ui| {
                ui.label("Theme Status:");
                if self.theme_loaded {
                    ui.colored_label(egui::Color32::GREEN, "✓ Custom theme loaded (material-theme2.json)");
                } else {
                    ui.colored_label(egui::Color32::RED, "✗ Using default theme");
                }
            });
            
            ui.add_space(10.0);
            
            // Refresh button
            if ui.add(MaterialButton::new("Refresh Data")).clicked() {
                self.data_state = DataState::Loading;
                self.receiver = None; // Clear any existing receiver
                self.fetch_flight_data();
            }
            
            ui.add_space(20.0);
            
            // Display data table based on state
            match &self.data_state {
                DataState::Loading => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Loading flight departures data...");
                    });
                }
                DataState::Loaded(flights) => {
                    if flights.is_empty() {
                        ui.label("No departure flights found from Incheon Airport at this time.");
                        ui.add_space(10.0);
                        ui.label("This could be due to:");
                        ui.label("• No scheduled departures in the current time window");
                        ui.label("• API access restrictions");
                        ui.label("• Network connectivity issues");
                        ui.add_space(10.0);
                        if ui.add(MaterialButton::new("Try Again")).clicked() {
                            self.data_state = DataState::Loading;
                            self.receiver = None;
                            self.fetch_flight_data();
                        }
                    } else {
                        ui.label(format!("Showing {} departure flights from Incheon Airport:", flights.len()));
                        ui.add_space(10.0);
                        
                        let datatable = self.create_flights_datatable(flights);
                        let response = datatable.show(ui);
                        
                        // Handle any table interactions if needed
                        if !response.row_actions.is_empty() {
                            println!("Table actions: {:?}", response.row_actions);
                        }
                        
                        ui.add_space(10.0);
                        ui.label("Data source: FlightRadar24 API (Real-time data)");
                    }
                }
                DataState::Error(error) => {
                    ui.colored_label(egui::Color32::RED, format!("Error loading data: {}", error));
                    
                    ui.add_space(10.0);
                    
                    if ui.add(MaterialButton::new("Retry")).clicked() {
                        self.data_state = DataState::Loading;
                        self.receiver = None; // Clear any existing receiver
                        self.fetch_flight_data();
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
