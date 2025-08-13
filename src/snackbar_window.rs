use eframe::egui::{self, Ui, Window};
use crate::{MaterialSnackbar, SnackbarPosition, MaterialButton, MaterialCheckbox, snackbar, snackbar_with_action};
use std::time::{Duration, Instant};

pub struct SnackbarWindow {
    pub open: bool,
    show_basic_snackbar: bool,
    show_action_snackbar: bool,
    show_top_snackbar: bool,
    message_text: String,
    action_text: String,
    auto_dismiss_seconds: f32,
    use_auto_dismiss: bool,
    // Add timing for auto-dismiss
    basic_snackbar_start: Option<Instant>,
    action_snackbar_start: Option<Instant>,
    top_snackbar_start: Option<Instant>,
}

impl Default for SnackbarWindow {
    fn default() -> Self {
        Self {
            open: false,
            show_basic_snackbar: false,
            show_action_snackbar: false,
            show_top_snackbar: false,
            message_text: "Operation completed successfully".to_string(),
            action_text: "Undo".to_string(),
            auto_dismiss_seconds: 4.0,
            use_auto_dismiss: true,
            basic_snackbar_start: None,
            action_snackbar_start: None,
            top_snackbar_start: None,
        }
    }
}

impl SnackbarWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Snackbar Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_snackbar_examples(ui);
                });
            });
        self.open = open;
        
        // Render snackbars outside the window (they should overlay everything)
        self.render_active_snackbars(ctx);
        
        // Request repaint to ensure auto-dismiss works properly
        if self.show_basic_snackbar || self.show_action_snackbar || self.show_top_snackbar {
            ctx.request_repaint();
        }
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Snackbar Controls");
            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/snackbar/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Message:");
            ui.text_edit_singleline(&mut self.message_text);
        });

        ui.horizontal(|ui| {
            ui.label("Action Text:");
            ui.text_edit_singleline(&mut self.action_text);
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.use_auto_dismiss, "Auto Dismiss"));
            if self.use_auto_dismiss {
                ui.label("After:");
                ui.add(egui::Slider::new(&mut self.auto_dismiss_seconds, 1.0..=10.0).suffix("s"));
            }
        });

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Show Basic Snackbar")).clicked() {
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::filled("Show Action Snackbar")).clicked() {
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::filled("Show Top Snackbar")).clicked() {
                self.show_top_snackbar = true;
                self.top_snackbar_start = Some(Instant::now());
            }
        });
    }

    fn render_snackbar_examples(&mut self, ui: &mut Ui) {
        ui.heading("Snackbar Types");
        
        ui.vertical(|ui| {
            ui.label("Basic Snackbar:");
            ui.label("• Simple message notification");
            ui.label("• Auto-dismisses after a timeout");
            ui.label("• Can be dismissed by user interaction");
            ui.add_space(10.0);
            
            ui.label("Action Snackbar:");
            ui.label("• Includes an action button");
            ui.label("• Allows user to respond to the notification");
            ui.label("• Common for undo operations");
            ui.add_space(10.0);
            
            ui.label("Positioning:");
            ui.label("• Bottom position (default)");
            ui.label("• Top position (alternative)");
            ui.label("• Always centered horizontally");
        });

        ui.add_space(20.0);
        
        ui.heading("Usage Guidelines");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Best Practices:");
                ui.label("• Keep messages brief");
                ui.label("• Use for confirmations");
                ui.label("• Provide undo when possible");
                ui.label("• Don't stack multiple snackbars");
            });
            
            ui.vertical(|ui| {
                ui.label("Common Use Cases:");
                ui.label("• File operations");
                ui.label("• Form submissions");
                ui.label("• Network status");
                ui.label("• Settings changes");
            });
        });

        ui.add_space(20.0);
        
        ui.heading("Interactive Demo");
        
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Save Document")).clicked() {
                self.message_text = "Document saved successfully".to_string();
                self.action_text = "Open".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::filled("Delete Item")).clicked() {
                self.message_text = "Item deleted".to_string();
                self.action_text = "Undo".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::filled("Send Message")).clicked() {
                self.message_text = "Message sent".to_string();
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }
        });
        
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Connection Lost")).clicked() {
                self.message_text = "No internet connection".to_string();
                self.action_text = "Retry".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::outlined("Settings Updated")).clicked() {
                self.message_text = "Settings saved".to_string();
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::outlined("Copy Complete")).clicked() {
                self.message_text = "Copied to clipboard".to_string();
                self.show_top_snackbar = true;
                self.top_snackbar_start = Some(Instant::now());
            }
        });
    }
    
    fn render_active_snackbars(&mut self, ctx: &egui::Context) {
        // Check auto-dismiss for basic snackbar
        if self.show_basic_snackbar && self.use_auto_dismiss {
            if let Some(start_time) = self.basic_snackbar_start {
                if start_time.elapsed().as_secs_f32() >= self.auto_dismiss_seconds {
                    self.show_basic_snackbar = false;
                    self.basic_snackbar_start = None;
                }
            }
        }
        
        // Check auto-dismiss for action snackbar
        if self.show_action_snackbar && self.use_auto_dismiss {
            if let Some(start_time) = self.action_snackbar_start {
                if start_time.elapsed().as_secs_f32() >= self.auto_dismiss_seconds {
                    self.show_action_snackbar = false;
                    self.action_snackbar_start = None;
                }
            }
        }
        
        // Check auto-dismiss for top snackbar
        if self.show_top_snackbar && self.use_auto_dismiss {
            if let Some(start_time) = self.top_snackbar_start {
                if start_time.elapsed().as_secs_f32() >= self.auto_dismiss_seconds {
                    self.show_top_snackbar = false;
                    self.top_snackbar_start = None;
                }
            }
        }
        
        // Render snackbars as overlays
        if self.show_basic_snackbar {
            egui::Area::new("basic_snackbar".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    let auto_dismiss = if self.use_auto_dismiss {
                        Some(Duration::from_secs_f32(self.auto_dismiss_seconds))
                    } else {
                        None
                    };
                    
                    let mut snackbar = snackbar(&self.message_text);
                    
                    if let Some(auto_dismiss) = auto_dismiss {
                        snackbar = snackbar.auto_dismiss(Some(auto_dismiss));
                    }
                    
                    let response = ui.add(snackbar.show_if(&mut self.show_basic_snackbar));
                    
                    // Force close if clicked
                    if response.clicked() {
                        self.show_basic_snackbar = false;
                        self.basic_snackbar_start = None;
                    }
                });
        }
        
        if self.show_action_snackbar {
            egui::Area::new("action_snackbar".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    let auto_dismiss = if self.use_auto_dismiss {
                        Some(Duration::from_secs_f32(self.auto_dismiss_seconds))
                    } else {
                        None
                    };
                    
                    let message = self.message_text.clone();
                    let action_text = self.action_text.clone();
                    
                    let mut snackbar = snackbar_with_action(
                        message,
                        action_text,
                        || println!("Snackbar action clicked!")
                    );
                    
                    if let Some(auto_dismiss) = auto_dismiss {
                        snackbar = snackbar.auto_dismiss(Some(auto_dismiss));
                    }
                    
                    let response = ui.add(snackbar.show_if(&mut self.show_action_snackbar));
                    
                    // Force close if clicked
                    if response.clicked() {
                        self.show_action_snackbar = false;
                        self.action_snackbar_start = None;
                    }
                });
        }
        
        if self.show_top_snackbar {
            egui::Area::new("top_snackbar".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    let auto_dismiss = if self.use_auto_dismiss {
                        Some(Duration::from_secs_f32(self.auto_dismiss_seconds))
                    } else {
                        None
                    };
                    
                    let mut snackbar = snackbar(&self.message_text)
                        .position(SnackbarPosition::Top);
                    
                    if let Some(auto_dismiss) = auto_dismiss {
                        snackbar = snackbar.auto_dismiss(Some(auto_dismiss));
                    }
                    
                    let response = ui.add(snackbar.show_if(&mut self.show_top_snackbar));
                    
                    // Force close if clicked
                    if response.clicked() {
                        self.show_top_snackbar = false;
                        self.top_snackbar_start = None;
                    }
                });
        }
    }
}