#![doc(hidden)]

use crate::{
    snackbar, snackbar_with_action, MaterialButton, MaterialCheckbox, MaterialSnackbar,
    SnackBarBehavior, SnackbarDuration, SnackbarHost, SnackbarPosition, SnackbarVisuals,
};
use eframe::egui::{self, Ui, Window};
use std::time::Duration;

#[doc(hidden)]
pub(crate) struct SnackbarWindow {
    pub open: bool,
    show_basic_snackbar: bool,
    show_action_snackbar: bool,
    show_top_snackbar: bool,
    message_text: String,
    action_text: String,
    auto_dismiss_seconds: f32,
    use_auto_dismiss: bool,
    last_host_result: Option<String>,
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
            last_host_result: None,
            behavior: SnackBarBehavior::Floating,
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
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Snackbar Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_snackbar_examples(ui);
                    ui.add_space(20.0);
                    self.render_host_controls(ui);
                });
            });
        self.open = open;

        // Render snackbars outside the window (they should overlay everything)
        self.render_active_snackbars(ctx);
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Snackbar Controls");
            if ui
                .add(MaterialButton::filled("Material Specs").small())
                .clicked()
            {
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
            ui.add(
                egui::Slider::new(&mut self.action_overflow_threshold, 0.0..=1.0).fixed_decimals(2),
            );
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
            if ui.add(MaterialButton::filled("Show Snackbar")).clicked() {
                if self.action_text.is_empty() {
                    self.show_basic_snackbar = true;
                } else {
                    self.show_action_snackbar = true;
                }
            }

            if ui.add(MaterialButton::outlined("Top Position")).clicked() {
                self.show_top_snackbar = true;
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Success Icon")).clicked() {
                self.message_text = "Action completed successfully!".to_string();
                self.leading_icon = "✓".to_string();
                self.show_leading_icon = true;
                self.show_basic_snackbar = true;
            }

            if ui.add(MaterialButton::outlined("Error Icon")).clicked() {
                self.message_text = "An error occurred. Please try again.".to_string();
                self.leading_icon = "⚠".to_string();
                self.show_leading_icon = true;
                self.show_action_snackbar = true;
                self.action_text = "Retry".to_string();
            }

            if ui.add(MaterialButton::outlined("Info Icon")).clicked() {
                self.message_text = "New feature available. Check it out!".to_string();
                self.leading_icon = "ℹ".to_string();
                self.show_leading_icon = true;
                self.show_action_snackbar = true;
                self.action_text = "Learn More".to_string();
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Long Message Test")).clicked() {
                self.message_text = "This is a very long message that should demonstrate text wrapping functionality in the snackbar. It should properly wrap to multiple lines without overlapping the action button area.".to_string();
                self.show_action_snackbar = true;
                self.action_text = "Dismiss".to_string();
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
            }

            if ui
                .add(MaterialButton::filled("Floating Behavior"))
                .clicked()
            {
                self.message_text = "Floating snackbar with margins".to_string();
                self.behavior = SnackBarBehavior::Floating;
                self.show_basic_snackbar = true;
            }

            if ui.add(MaterialButton::filled("Custom Width")).clicked() {
                self.message_text = "Custom width floating snackbar".to_string();
                self.behavior = SnackBarBehavior::Floating;
                self.use_custom_width = true;
                self.custom_width = 450.0;
                self.show_basic_snackbar = true;
            }

            if ui.add(MaterialButton::filled("With Close Icon")).clicked() {
                self.message_text = "Snackbar with closable icon".to_string();
                self.show_close_icon = true;
                self.show_basic_snackbar = true;
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Action Overflow")).clicked() {
                self.message_text = "This message will cause the action button to overflow to a new line because the threshold is met".to_string();
                self.action_text = "Very Long Action Text".to_string();
                self.action_overflow_threshold = 0.20;
                self.show_action_snackbar = true;
            }

            if ui.add(MaterialButton::outlined("All Features")).clicked() {
                self.message_text = "Complete feature showcase with icon, action, and close button".to_string();
                self.behavior = SnackBarBehavior::Floating;
                self.show_leading_icon = true;
                self.leading_icon = "🎉".to_string();
                self.show_close_icon = true;
                self.action_text = "View".to_string();
                self.show_action_snackbar = true;
            }
        });
    }

    fn render_host_controls(&mut self, ui: &mut Ui) {
        ui.heading("Snackbar Host (queue)");
        ui.label(
            "SnackbarHost keeps a queue in the context and shows one snackbar at a time.              Queue several and watch them follow each other.",
        );
        let host = SnackbarHost::new();
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled("Short (4 s)")).clicked() {
                host.enqueue(ui.ctx(), SnackbarVisuals::new("Photo saved"));
            }
            if ui.add(MaterialButton::filled("Long (10 s)")).clicked() {
                host.enqueue(
                    ui.ctx(),
                    SnackbarVisuals::new("Backup finished: 1,204 files copied")
                        .duration(SnackbarDuration::Long),
                );
            }
            if ui
                .add(MaterialButton::filled("Indefinite + action"))
                .clicked()
            {
                host.enqueue(
                    ui.ctx(),
                    SnackbarVisuals::new("Message archived")
                        .action("Undo")
                        .with_dismiss_action(true)
                        .duration(SnackbarDuration::Indefinite),
                );
            }
            if ui
                .add(MaterialButton::outlined("Dismiss current"))
                .clicked()
            {
                host.dismiss_current(ui.ctx());
            }
        });
        ui.label(format!(
            "Waiting: {}. Last result: {}",
            host.pending(ui.ctx()),
            self.last_host_result.as_deref().unwrap_or("none")
        ));
    }

    fn render_active_snackbars(&mut self, ctx: &egui::Context) {
        if let Some(outcome) = SnackbarHost::new().show(ctx) {
            self.last_host_result = Some(format!("{:?}", outcome.result));
        }

        let auto_dismiss = self
            .use_auto_dismiss
            .then(|| Duration::from_secs_f32(self.auto_dismiss_seconds));
        let behavior = self.behavior;
        let threshold = self.action_overflow_threshold;
        let width = self.use_custom_width.then_some(self.custom_width);
        let close_icon = self.show_close_icon;
        let leading_icon = (self.show_leading_icon && !self.leading_icon.is_empty())
            .then(|| self.leading_icon.clone());
        let configure = |snackbar: MaterialSnackbar<'static>| {
            let mut snackbar = snackbar
                .auto_dismiss(auto_dismiss)
                .behavior(behavior)
                .action_overflow_threshold(threshold)
                .show_close_icon(close_icon);
            if let Some(width) = width {
                snackbar = snackbar.width(width);
            }
            if let Some(icon) = &leading_icon {
                snackbar = snackbar.leading_icon(icon.clone());
            }
            snackbar
        };

        // The snackbar draws in its own foreground layer and writes `false`
        // back to the flag when it times out or is dismissed.
        egui::Area::new(egui::Id::unique("snackbar_story_overlay"))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let basic = configure(snackbar(self.message_text.clone()));
                ui.add(basic.show_with_offset(&mut self.show_basic_snackbar, 0.0));

                let action = configure(snackbar_with_action(
                    self.message_text.clone(),
                    self.action_text.clone(),
                    || println!("Snackbar action clicked!"),
                ));
                ui.add(action.show_with_offset(&mut self.show_action_snackbar, 0.0));

                let top =
                    configure(snackbar(self.message_text.clone())).position(SnackbarPosition::Top);
                ui.add(top.show_with_offset(&mut self.show_top_snackbar, 0.0));
            });
    }
}
