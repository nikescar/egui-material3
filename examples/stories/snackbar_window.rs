#![doc(hidden)]

use crate::{snackbar, snackbar_with_action, MaterialButton, MaterialCheckbox, SnackbarPosition};
use eframe::egui::{self, Ui, Window};
use std::time::Instant;

#[doc(hidden)]
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
            if ui.button("Material Specs").clicked() {
                let _ = webbrowser::open("https://m2.material.io/components/snackbars#specs");
            }
            if ui.button("MDC Reference").clicked() {
                let _ = webbrowser::open("https://material-components.github.io/material-components-web-catalog/#/component/snackbar");
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
            ui.add(MaterialCheckbox::new(
                &mut self.use_auto_dismiss,
                "Auto Dismiss",
            ));
            if self.use_auto_dismiss {
                ui.label("After:");
                ui.add(egui::Slider::new(&mut self.auto_dismiss_seconds, 1.0..=10.0).suffix("s"));
            }
        });

        ui.horizontal(|ui| {
            if ui
                .add(MaterialButton::filled("Show Basic Snackbar"))
                .clicked()
            {
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }
            if ui
                .add(MaterialButton::filled("Show Action Snackbar"))
                .clicked()
            {
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            if ui
                .add(MaterialButton::filled("Show Top Snackbar"))
                .clicked()
            {
                self.show_top_snackbar = true;
                self.top_snackbar_start = Some(Instant::now());
            }
        });

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Long Message Test")).clicked() {
                self.message_text = "This is a very long message that should demonstrate text wrapping functionality in the snackbar. It should properly wrap to multiple lines without overlapping the action button area.".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::outlined("Very Long Message Test")).clicked() {
                self.message_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            if ui.add(MaterialButton::outlined("Reset Message")).clicked() {
                self.message_text = "Operation completed successfully".to_string();
            }
        });
    }

    fn render_snackbar_examples(&mut self, ui: &mut Ui) {
        ui.heading("Material Design Snackbar Specifications");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("📐 Dimensions:");
                ui.label("• Min width: 344px");
                ui.label("• Max width: 672px");
                ui.label("• Height: 48px minimum (dynamic for multi-line text)");
                ui.label("• Corner radius: 4px");
                ui.add_space(10.0);

                ui.label("🎨 Colors:");
                ui.label("• Background: 80% on-surface + 20% surface");
                ui.label("• Text: surface (high contrast)");
                ui.label("• Action: inverse-primary");
            });

            ui.vertical(|ui| {
                ui.label("📏 Padding:");
                ui.label("• Label: 16px left, 8px right");
                ui.label("• Vertical: 14px top/bottom");
                ui.label("• Action button: 8px padding");
                ui.add_space(10.0);

                ui.label("🌟 Elevation:");
                ui.label("• Level: 6dp (shadow)");
                ui.label("• Typography: body2 (14px)");
                ui.label("• Position: bottom center");
            });
        });

        ui.add_space(20.0);

        ui.heading("Snackbar Rectangle Demonstration");

        // Show properly styled rectangle examples
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled("Basic Rectangle")).clicked() {
                self.message_text = "This is a basic snackbar with proper rectangle shape".to_string();
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }
            
            if ui.add(MaterialButton::filled("With Action")).clicked() {
                self.message_text = "Rectangle snackbar with action button".to_string();
                self.action_text = "Action".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
            
            if ui.add(MaterialButton::filled("Long Message")).clicked() {
                self.message_text = "This is a longer message to demonstrate how the rectangle snackbar handles text wrapping and maintains proper dimensions according to Material Design specifications".to_string();
                self.action_text = "Got it".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
        });

        ui.add_space(10.0);

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Top Position")).clicked() {
                self.message_text = "Snackbar positioned at top".to_string();
                self.show_top_snackbar = true;
                self.top_snackbar_start = Some(Instant::now());
            }

            if ui.add(MaterialButton::outlined("File Deleted")).clicked() {
                self.message_text = "File deleted successfully".to_string();
                self.action_text = "Undo".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }

            if ui.add(MaterialButton::outlined("No Connection")).clicked() {
                self.message_text = "No internet connection available".to_string();
                self.action_text = "Retry".to_string();
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }
        });
    }

    fn render_active_snackbars(&mut self, ctx: &egui::Context) {
        // Handle auto-dismiss timing at window level for better control
        if self.show_basic_snackbar && self.use_auto_dismiss {
            if let Some(start_time) = self.basic_snackbar_start {
                if start_time.elapsed().as_secs_f32() >= self.auto_dismiss_seconds {
                    self.show_basic_snackbar = false;
                    self.basic_snackbar_start = None;
                }
            }
        }

        if self.show_action_snackbar && self.use_auto_dismiss {
            if let Some(start_time) = self.action_snackbar_start {
                if start_time.elapsed().as_secs_f32() >= self.auto_dismiss_seconds {
                    self.show_action_snackbar = false;
                    self.action_snackbar_start = None;
                }
            }
        }

        if self.show_top_snackbar && self.use_auto_dismiss {
            if let Some(start_time) = self.top_snackbar_start {
                if start_time.elapsed().as_secs_f32() >= self.auto_dismiss_seconds {
                    self.show_top_snackbar = false;
                    self.top_snackbar_start = None;
                }
            }
        }

        // Calculate stacking offsets for each position
        let bottom_offset = 0.0;
        let top_offset = 0.0;
        let _snackbar_spacing = 56.0; // Material Design spacing (48px height + 8px margin)

        // Render bottom-positioned snackbars with stacking
        if self.show_basic_snackbar {
            egui::Area::new("basic_snackbar".into())
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    ui.set_clip_rect(ctx.screen_rect());

                    let snackbar = snackbar(&self.message_text).auto_dismiss(None); // Disable widget auto-dismiss, handled by window

                    let mut show_snackbar = self.show_basic_snackbar;
                    let response =
                        ui.add(snackbar.show_with_offset(&mut show_snackbar, bottom_offset));

                    // Update state based on snackbar widget's decision
                    if !show_snackbar && self.show_basic_snackbar {
                        self.show_basic_snackbar = false;
                        self.basic_snackbar_start = None;
                    }

                    // Force close if clicked on snackbar (but not action)
                    if response.clicked() {
                        self.show_basic_snackbar = false;
                        self.basic_snackbar_start = None;
                    }
                });
            // bottom_offset += snackbar_spacing; // Unused assignment removed
        }

        if self.show_action_snackbar {
            egui::Area::new("action_snackbar".into())
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    ui.set_clip_rect(ctx.screen_rect());

                    let message = self.message_text.clone();
                    let action_text = self.action_text.clone();

                    let snackbar = snackbar_with_action(message, action_text, || {
                        println!("Snackbar action clicked!");
                    })
                    .auto_dismiss(None); // Disable widget auto-dismiss, handled by window

                    let mut show_snackbar = self.show_action_snackbar;
                    let response =
                        ui.add(snackbar.show_with_offset(&mut show_snackbar, bottom_offset));

                    // Update state based on snackbar widget's decision
                    if !show_snackbar && self.show_action_snackbar {
                        self.show_action_snackbar = false;
                        self.action_snackbar_start = None;
                    }

                    // Force close if clicked on message area (not action button)
                    if response.clicked() && self.action_text.is_empty() {
                        self.show_action_snackbar = false;
                        self.action_snackbar_start = None;
                    }
                });
            // bottom_offset += snackbar_spacing; // Unused assignment removed
        }

        // Render top-positioned snackbars with stacking
        if self.show_top_snackbar {
            egui::Area::new("top_snackbar".into())
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    ui.set_clip_rect(ctx.screen_rect());

                    let snackbar = snackbar(&self.message_text)
                        .position(SnackbarPosition::Top)
                        .auto_dismiss(None); // Disable widget auto-dismiss, handled by window

                    let mut show_snackbar = self.show_top_snackbar;
                    let response =
                        ui.add(snackbar.show_with_offset(&mut show_snackbar, top_offset));

                    // Update state based on snackbar widget's decision
                    if !show_snackbar && self.show_top_snackbar {
                        self.show_top_snackbar = false;
                        self.top_snackbar_start = None;
                    }

                    // Force close if clicked
                    if response.clicked() {
                        self.show_top_snackbar = false;
                        self.top_snackbar_start = None;
                    }
                });
            // top_offset += snackbar_spacing; // Unused assignment removed
        }
    }
}
