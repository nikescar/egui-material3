//! MVVM Todo List Example
//!
//! Demonstrates more complex MVVM patterns with shared state:
//! - Model: Todo items with persistence
//! - ViewModel: TodoListViewModel with RefState
//! - View: Material Design list and input components

use eframe::egui;
use egui_material3::mvvm::{RefState, ValState, ViewModel};
use egui_material3::{
    theme::{
        load_fonts, load_themes, setup_google_fonts, setup_local_fonts, setup_local_theme,
        update_window_background,
    },
    MaterialButton, MaterialCheckbox, TextEdit,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MVVM Todo List - egui-material3",
        options,
        Box::new(|cc| {
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

// ==================== Model ====================

#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

// ==================== ViewModel ====================

struct TodoListViewModel {
    todos: RefState<Vec<TodoItem>>,
    new_todo_text: ValState<String>,
    filter: ValState<TodoFilter>,
    next_id: ValState<usize>,
}

#[derive(Clone, Copy, PartialEq)]
enum TodoFilter {
    All,
    Active,
    Completed,
}

impl TodoListViewModel {
    fn new() -> Self {
        let mut vm = Self {
            todos: RefState::new(Vec::new()),
            new_todo_text: ValState::new(String::new()),
            filter: ValState::new(TodoFilter::All),
            next_id: ValState::new(1),
        };
        vm.init();
        vm
    }

    // Commands
    fn add_todo(&mut self) {
        let text = self.new_todo_text.get().trim().to_string();
        if text.is_empty() {
            return;
        }

        let id = self.next_id.get();
        self.next_id.update(|v| *v += 1);

        let todo = TodoItem {
            id,
            text,
            completed: false,
        };

        self.todos.update(|todos| todos.push(todo));
        self.new_todo_text.set(String::new());
    }

    fn toggle_todo(&mut self, id: usize) {
        self.todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    }

    fn delete_todo(&mut self, id: usize) {
        self.todos.update(|todos| {
            todos.retain(|t| t.id != id);
        });
    }

    fn clear_completed(&mut self) {
        self.todos.update(|todos| {
            todos.retain(|t| !t.completed);
        });
    }

    fn set_filter(&mut self, filter: TodoFilter) {
        self.filter.set(filter);
    }

    // Queries
    fn filtered_todos(&self) -> Vec<TodoItem> {
        let all_todos = self.todos.get();
        let filter = self.filter.get();

        all_todos
            .into_iter()
            .filter(|todo| match filter {
                TodoFilter::All => true,
                TodoFilter::Active => !todo.completed,
                TodoFilter::Completed => todo.completed,
            })
            .collect()
    }

    fn stats(&self) -> (usize, usize, usize) {
        let todos = self.todos.get();
        let total = todos.len();
        let completed = todos.iter().filter(|t| t.completed).count();
        let active = total - completed;
        (total, active, completed)
    }
}

impl ViewModel for TodoListViewModel {
    fn init(&mut self) {
        println!("TodoListViewModel initialized");
        // Could load from persistent storage here
        self.todos.update(|todos| {
            todos.push(TodoItem {
                id: 0,
                text: "Welcome to MVVM Todo!".to_string(),
                completed: false,
            });
        });
    }

    fn dispose(&mut self) {
        println!("TodoListViewModel disposed");
        // Could save to persistent storage here
    }
}

// ==================== View ====================

struct MyApp {
    view_model: TodoListViewModel,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            view_model: TodoListViewModel::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("MVVM Todo List Example");
                ui.label("Using RefState for shared mutable state");
                ui.add_space(10.0);
            });

            ui.separator();
            ui.add_space(20.0);

            // Add new todo
            ui.horizontal(|ui| {
                ui.label("New todo:");
                let mut text = self.view_model.new_todo_text.get();
                let response = ui.add(
                    TextEdit::singleline(&mut text)
                        .desired_width(400.0)
                        .hint_text("What needs to be done?"),
                );

                if response.changed() {
                    self.view_model.new_todo_text.set(text);
                }

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.view_model.add_todo();
                    ctx.request_repaint();
                }

                if ui.add(MaterialButton::filled("Add")).clicked() {
                    self.view_model.add_todo();
                    ctx.request_repaint();
                }
            });

            ui.add_space(20.0);

            // Filter buttons
            ui.horizontal(|ui| {
                ui.label("Show:");
                ui.add_space(10.0);

                let current_filter = self.view_model.filter.get();

                for (filter, label) in [
                    (TodoFilter::All, "All"),
                    (TodoFilter::Active, "Active"),
                    (TodoFilter::Completed, "Completed"),
                ] {
                    let button = if current_filter == filter {
                        MaterialButton::filled(label)
                    } else {
                        MaterialButton::outlined(label)
                    };

                    if ui.add(button).clicked() {
                        self.view_model.set_filter(filter);
                        ctx.request_repaint();
                    }
                    ui.add_space(5.0);
                }
            });

            ui.add_space(10.0);

            // Todo list
            egui::ScrollArea::vertical().show(ui, |ui| {
                let filtered_todos = self.view_model.filtered_todos();

                if filtered_todos.is_empty() {
                    ui.add_space(40.0);
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("No todos to show").italics().size(16.0));
                    });
                } else {
                    for todo in filtered_todos {
                        ui.horizontal(|ui| {
                            let mut completed = todo.completed;
                            if ui.add(MaterialCheckbox::new(&mut completed, "")).changed() {
                                self.view_model.toggle_todo(todo.id);
                                ctx.request_repaint();
                            }

                            let text_style = if todo.completed {
                                egui::RichText::new(&todo.text).strikethrough()
                            } else {
                                egui::RichText::new(&todo.text)
                            };
                            ui.label(text_style);

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.add(MaterialButton::text("Delete")).clicked() {
                                        self.view_model.delete_todo(todo.id);
                                        ctx.request_repaint();
                                    }
                                },
                            );
                        });
                        ui.separator();
                    }
                }
            });

            ui.add_space(20.0);

            // Stats and actions
            ui.horizontal(|ui| {
                let (total, active, completed) = self.view_model.stats();
                ui.label(format!(
                    "Total: {} | Active: {} | Completed: {}",
                    total, active, completed
                ));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if completed > 0 {
                        if ui.add(MaterialButton::text("Clear Completed")).clicked() {
                            self.view_model.clear_completed();
                            ctx.request_repaint();
                        }
                    }
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Info
            ui.group(|ui| {
                ui.label(egui::RichText::new("Architecture").strong());
                ui.add_space(5.0);
                ui.label("Model: TodoItem struct");
                ui.label("ViewModel: RefState<Vec<TodoItem>> for shared state");
                ui.label("View: Material Design components");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Thread-safe with Arc<Mutex<T>>").italics());
            });
        });
    }
}
