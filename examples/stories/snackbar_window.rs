#![doc(hidden)]

use crate::{snackbar, snackbar_with_action, MaterialButton, MaterialCheckbox, SnackbarPosition, SnackBarBehavior};
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
    // New fields for enhanced features
    behavior: SnackBarBehavior,
    use_custom_width: bool,
    custom_width: f32,
    show_close_icon: bool,
    show_leading_icon: bool,
    leading_icon: String,
    action_overflow_threshold: f32,
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
            behavior: SnackBarBehavior::Fixed,
            use_custom_width: false,
            custom_width: 500.0,
            show_close_icon: false,
            show_leading_icon: false,
            leading_icon: "✓".to_string(),
            action_overflow_threshold: 0.25,
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
            if ui.add(MaterialButton::filled("Material Specs").small()).clicked() {
                let _ = webbrowser::open("https://m3.material.io/components/snackbar/specs");
            }
        });

        ui.separator();
        ui.label("📝 Content");

        ui.horizontal(|ui| {
            ui.label("Message:");
            ui.text_edit_singleline(&mut self.message_text);
        });

        ui.horizontal(|ui| {
            ui.label("Action Text:");
            ui.text_edit_singleline(&mut self.action_text);
        });

        ui.separator();
        ui.label("⚙️ Behavior & Appearance");

        ui.horizontal(|ui| {
            ui.label("Behavior:");
            ui.radio_value(&mut self.behavior, SnackBarBehavior::Fixed, "Fixed");
            ui.radio_value(&mut self.behavior, SnackBarBehavior::Floating, "Floating");
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(
                &mut self.use_custom_width,
                "Custom Width",
            ));
            if self.use_custom_width {
                ui.add(egui::Slider::new(&mut self.custom_width, 344.0..=672.0).suffix("px"));
            }
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(
                &mut self.show_close_icon,
                "Show Close Icon",
            ));
            ui.add(MaterialCheckbox::new(
                &mut self.show_leading_icon,
                "Show Leading Icon",
            ));
            if self.show_leading_icon {
                ui.label("Icon:");
                ui.text_edit_singleline(&mut self.leading_icon);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Action Overflow Threshold:");
            ui.add(egui::Slider::new(&mut self.action_overflow_threshold, 0.0..=1.0)
                .fixed_decimals(2));
        });

        ui.separator();
        ui.label("⏱️ Auto Dismiss");

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

        ui.separator();
        ui.label("🎬 Quick Actions");

        ui.horizontal_wrapped(|ui| {
            if ui
                .add(MaterialButton::filled("Show Snackbar"))
                .clicked()
            {
                if self.action_text.is_empty() {
                    self.show_basic_snackbar = true;
                    self.basic_snackbar_start = Some(Instant::now());
                } else {
                    self.show_action_snackbar = true;
                    self.action_snackbar_start = Some(Instant::now());
                }
            }

            if ui
                .add(MaterialButton::outlined("Top Position"))
                .clicked()
            {
                self.show_top_snackbar = true;
                self.top_snackbar_start = Some(Instant::now());
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Success Icon")).clicked() {
                self.message_text = "Action completed successfully!".to_string();
                self.leading_icon = "✓".to_string();
                self.show_leading_icon = true;
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }

            if ui.add(MaterialButton::outlined("Error Icon")).clicked() {
                self.message_text = "An error occurred. Please try again.".to_string();
                self.leading_icon = "⚠".to_string();
                self.show_leading_icon = true;
                self.show_action_snackbar = true;
                self.action_text = "Retry".to_string();
                self.action_snackbar_start = Some(Instant::now());
            }

            if ui.add(MaterialButton::outlined("Info Icon")).clicked() {
                self.message_text = "New feature available. Check it out!".to_string();
                self.leading_icon = "ℹ".to_string();
                self.show_leading_icon = true;
                self.show_action_snackbar = true;
                self.action_text = "Learn More".to_string();
                self.action_snackbar_start = Some(Instant::now());
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Long Message Test")).clicked() {
                self.message_text = "This is a very long message that should demonstrate text wrapping functionality in the snackbar. It should properly wrap to multiple lines without overlapping the action button area.".to_string();
                self.show_action_snackbar = true;
                self.action_text = "Dismiss".to_string();
                self.action_snackbar_start = Some(Instant::now());
            }
            
            if ui.add(MaterialButton::outlined("Reset Settings")).clicked() {
                *self = Self::default();
                self.open = true;
            }
        });
    }

    fn render_snackbar_examples(&mut self, ui: &mut Ui) {
        ui.heading("Feature Demonstrations");

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled("Fixed Behavior")).clicked() {
                self.message_text = "Fixed snackbar at bottom of screen".to_string();
                self.behavior = SnackBarBehavior::Fixed;
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }
            
            if ui.add(MaterialButton::filled("Floating Behavior")).clicked() {
                self.message_text = "Floating snackbar with margins".to_string();
                self.behavior = SnackBarBehavior::Floating;
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }

            if ui.add(MaterialButton::filled("Custom Width")).clicked() {
                self.message_text = "Custom width floating snackbar".to_string();
                self.behavior = SnackBarBehavior::Floating;
                self.use_custom_width = true;
                self.custom_width = 450.0;
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }

            if ui.add(MaterialButton::filled("With Close Icon")).clicked() {
                self.message_text = "Snackbar with closable icon".to_string();
                self.show_close_icon = true;
                self.show_basic_snackbar = true;
                self.basic_snackbar_start = Some(Instant::now());
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Action Overflow")).clicked() {
                self.message_text = "This message will cause the action button to overflow to a new line because the threshold is met".to_string();
                self.action_text = "Very Long Action Text".to_string();
                self.action_overflow_threshold = 0.20;
                self.show_action_snackbar = true;
                self.action_snackbar_start = Some(Instant::now());
            }

            if ui.add(MaterialButton::outlined("All Features")).clicked() {
                self.message_text = "Complete feature showcase with icon, action, and close button".to_string();
                self.behavior = SnackBarBehavior::Floating;
                self.show_leading_icon = true;
                self.leading_icon = "🎉".to_string();
                self.show_close_icon = true;
                self.action_text = "View".to_string();
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

                    let mut snackbar = snackbar(&self.message_text)
                        .auto_dismiss(None)
                        .behavior(self.behavior)
                        .action_overflow_threshold(self.action_overflow_threshold);

                    if self.use_custom_width {
                        snackbar = snackbar.width(self.custom_width);
                    }

                    if self.show_close_icon {
                        snackbar = snackbar.show_close_icon(true);
                    }

                    if self.show_leading_icon && !self.leading_icon.is_empty() {
                        snackbar = snackbar.leading_icon(&self.leading_icon);
                    }

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
        }

        if self.show_action_snackbar {
            egui::Area::new("action_snackbar".into())
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    ui.set_clip_rect(ctx.screen_rect());

                    let message = self.message_text.clone();
                    let action_text = self.action_text.clone();

                    let mut snackbar = snackbar_with_action(message, action_text, || {
                        println!("Snackbar action clicked!");
                    })
                    .auto_dismiss(None)
                    .behavior(self.behavior)
                    .action_overflow_threshold(self.action_overflow_threshold);

                    if self.use_custom_width {
                        snackbar = snackbar.width(self.custom_width);
                    }

                    if self.show_close_icon {
                        snackbar = snackbar.show_close_icon(true);
                    }

                    if self.show_leading_icon && !self.leading_icon.is_empty() {
                        snackbar = snackbar.leading_icon(&self.leading_icon);
                    }

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
        }

        // Render top-positioned snackbars with stacking
        if self.show_top_snackbar {
            egui::Area::new("top_snackbar".into())
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    ui.set_clip_rect(ctx.screen_rect());

                    let mut snackbar = snackbar(&self.message_text)
                        .position(SnackbarPosition::Top)
                        .auto_dismiss(None)
                        .behavior(self.behavior)
                        .action_overflow_threshold(self.action_overflow_threshold);

                    if self.use_custom_width {
                        snackbar = snackbar.width(self.custom_width);
                    }

                    if self.show_close_icon {
                        snackbar = snackbar.show_close_icon(true);
                    }

                    if self.show_leading_icon && !self.leading_icon.is_empty() {
                        snackbar = snackbar.leading_icon(&self.leading_icon);
                    }

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
        }
    }
}
