//! MVVM Counter Example
//!
//! Demonstrates using egui-material3 with MVVM pattern:
//! - Model: Counter data (i32)
//! - ViewModel: CounterViewModel with reactive state
//! - View: Material Design components

use eframe::egui;
use egui_material3::mvvm::{ValState, ViewModel};
use egui_material3::{
    theme::{load_fonts, load_themes, setup_google_fonts, setup_local_fonts, setup_local_theme, update_window_background},
    MaterialButton, MaterialCheckbox,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MVVM Counter Example - egui-material3",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and themes
            setup_google_fonts(Some("Roboto"));
            setup_local_fonts(Some("resources/MaterialSymbolsOutlined.ttf"));
            setup_local_theme(None);
            load_fonts(&cc.egui_ctx);
            load_themes();
            update_window_background(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

// ==================== ViewModel ====================

/// Counter ViewModel - holds reactive state and business logic
struct CounterViewModel {
    count: ValState<i32>,
    step: ValState<i32>,
    message: ValState<String>,
    auto_update: ValState<bool>,
}

impl CounterViewModel {
    fn new() -> Self {
        let mut vm = Self {
            count: ValState::new(0),
            step: ValState::new(1),
            message: ValState::new(String::new()),
            auto_update: ValState::new(false),
        };
        vm.init();
        vm
    }

    // Commands (business logic)
    fn increment(&mut self) {
        let step = self.step.get();
        self.count.update(|v| *v += step);
        self.update_message();
    }

    fn decrement(&mut self) {
        let step = self.step.get();
        self.count.update(|v| *v -= step);
        self.update_message();
    }

    fn reset(&mut self) {
        self.count.set(0);
        self.update_message();
    }

    fn set_step(&mut self, new_step: i32) {
        self.step.set(new_step);
    }

    fn toggle_auto_update(&mut self) {
        self.auto_update.update(|v| *v = !*v);
    }

    fn update_message(&mut self) {
        let count = self.count.get();
        let message = match count {
            0 => "Starting point!".to_string(),
            n if n > 0 => format!("Counting up: {}", n),
            n => format!("Counting down: {}", n),
        };
        self.message.set(message);
    }
}

impl ViewModel for CounterViewModel {
    fn init(&mut self) {
        println!("CounterViewModel initialized");
        self.update_message();
    }

    fn dispose(&mut self) {
        println!("CounterViewModel disposed");
    }
}

// ==================== View ====================

struct MyApp {
    view_model: CounterViewModel,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            view_model: CounterViewModel::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("MVVM Counter Example");
                ui.label("Using egui-material3 with reactive ViewModels");
                ui.add_space(10.0);
            });

            ui.separator();
            ui.add_space(20.0);

            // Display current count
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(
                    format!("Count: {}", self.view_model.count.get())
                ).size(48.0).strong());

                ui.add_space(10.0);
                ui.label(egui::RichText::new(
                    &self.view_model.message.get()
                ).size(20.0));
            });

            ui.add_space(30.0);

            // Counter controls
            ui.horizontal(|ui| {
                ui.add_space(50.0);

                if ui.add(MaterialButton::filled("Increment")).clicked() {
                    self.view_model.increment();
                    ctx.request_repaint();
                }

                ui.add_space(10.0);

                if ui.add(MaterialButton::outlined("Decrement")).clicked() {
                    self.view_model.decrement();
                    ctx.request_repaint();
                }

                ui.add_space(10.0);

                if ui.add(MaterialButton::text("Reset")).clicked() {
                    self.view_model.reset();
                    ctx.request_repaint();
                }
            });

            ui.add_space(30.0);
            ui.separator();
            ui.add_space(20.0);

            // Step size control
            ui.horizontal(|ui| {
                ui.label("Step size:");
                ui.add_space(10.0);

                let current_step = self.view_model.step.get();

                for step in [1, 5, 10, 100] {
                    let is_selected = current_step == step;
                    let button = if is_selected {
                        MaterialButton::filled(format!("{}", step))
                    } else {
                        MaterialButton::outlined(format!("{}", step))
                    };

                    if ui.add(button).clicked() {
                        self.view_model.set_step(step);
                        ctx.request_repaint();
                    }
                    ui.add_space(5.0);
                }
            });

            ui.add_space(20.0);

            // Auto-update toggle
            ui.horizontal(|ui| {
                let mut auto_update = self.view_model.auto_update.get();
                if ui.add(MaterialCheckbox::new(&mut auto_update, "Auto increment")).changed() {
                    self.view_model.toggle_auto_update();
                    ctx.request_repaint();
                }
            });

            // Auto-increment logic
            if self.view_model.auto_update.get() {
                self.view_model.increment();
                ctx.request_repaint();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // Info panel
            ui.group(|ui| {
                ui.label(egui::RichText::new("MVVM Architecture").strong());
                ui.add_space(5.0);
                ui.label("Model: Counter data (i32)");
                ui.label("ViewModel: CounterViewModel with ValState");
                ui.label("View: Material Design buttons & checkbox");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("No tokio dependency - using smol runtime").italics());
            });
        });
    }
}
