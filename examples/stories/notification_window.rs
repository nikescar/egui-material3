#![doc(hidden)]

use crate::{notification, MaterialButton, NotificationAlign};
use eframe::egui::{self, Ui, Window};
use std::time::{Duration, Instant};

#[doc(hidden)]
pub struct NotificationWindow {
    pub open: bool,
    // State for different notification examples
    show_basic: bool,
    show_with_icon: bool,
    show_with_subtitle: bool,
    show_full: bool,
    show_closeable: bool,
    show_custom: bool,
    show_stack_1: bool,
    show_stack_2: bool,
    show_stack_3: bool,
    // Timing state for auto-dismiss
    basic_show_time: Option<Instant>,
    with_icon_show_time: Option<Instant>,
    with_subtitle_show_time: Option<Instant>,
    full_show_time: Option<Instant>,
    closeable_show_time: Option<Instant>,
    custom_show_time: Option<Instant>,
    stack_1_show_time: Option<Instant>,
    stack_2_show_time: Option<Instant>,
    stack_3_show_time: Option<Instant>,
    // Custom controls
    custom_title: String,
    custom_subtitle: String,
    custom_text: String,
    custom_icon: String,
    custom_right_text: String,
    custom_closeable: bool,
    // Alignment setting
    notification_align: NotificationAlign,
}

impl Default for NotificationWindow {
    fn default() -> Self {
        Self {
            open: false,
            show_basic: false,
            show_with_icon: false,
            show_with_subtitle: false,
            show_full: false,
            show_closeable: false,
            show_custom: false,
            show_stack_1: false,
            show_stack_2: false,
            show_stack_3: false,
            basic_show_time: None,
            with_icon_show_time: None,
            with_subtitle_show_time: None,
            full_show_time: None,
            closeable_show_time: None,
            custom_show_time: None,
            stack_1_show_time: None,
            stack_2_show_time: None,
            stack_3_show_time: None,
            custom_title: "Custom Title".to_string(),
            custom_subtitle: "Custom subtitle".to_string(),
            custom_text: "Custom notification message text".to_string(),
            custom_icon: "info".to_string(),
            custom_right_text: "now".to_string(),
            custom_closeable: true,
            notification_align: NotificationAlign::Center,
        }
    }
}

impl NotificationWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Notification Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Notification Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://konstaui.com/react/notification");
            }
        });

        ui.label("Click buttons below to toggle different notification examples:");

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled_tonal("Basic")).clicked() {
                self.show_basic = !self.show_basic;
            }
            if ui.add(MaterialButton::filled_tonal("With Icon")).clicked() {
                self.show_with_icon = !self.show_with_icon;
            }
            if ui.add(MaterialButton::filled_tonal("With Subtitle")).clicked() {
                self.show_with_subtitle = !self.show_with_subtitle;
            }
            if ui.add(MaterialButton::filled_tonal("Full")).clicked() {
                self.show_full = !self.show_full;
            }
            if ui.add(MaterialButton::filled_tonal("Closeable")).clicked() {
                self.show_closeable = !self.show_closeable;
            }
            if ui.add(MaterialButton::filled_tonal("Custom")).clicked() {
                self.show_custom = !self.show_custom;
            }
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Notification Alignment:");
            ui.radio_value(&mut self.notification_align, NotificationAlign::Left, "Left");
            ui.radio_value(&mut self.notification_align, NotificationAlign::Center, "Center");
            ui.radio_value(&mut self.notification_align, NotificationAlign::Right, "Right");
        });
    }

    fn render_examples(&mut self, ui: &mut Ui) {
        // Basic Notification
        ui.heading("Basic Notification");
        ui.label("Simple notification with title and text (auto-dismisses after 5 seconds):");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Show Basic")).clicked() {
                self.show_basic = true;
                self.basic_show_time = Some(Instant::now());
            }
            if self.show_basic {
                if ui.add(MaterialButton::text("Hide")).clicked() {
                    self.show_basic = false;
                    self.basic_show_time = None;
                }
            }
        });

        // Check for auto-dismiss (5 seconds)
        if self.show_basic {
            if let Some(show_time) = self.basic_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_basic = false;
                    self.basic_show_time = None;
                }
            }
        }

        if self.show_basic {
            let response = ui.add(
                notification()
                    .title("New Message")
                    .text("You have received a new message")
                    .align(self.notification_align)
                    .opened(true),
            );

            // Click to dismiss
            if response.clicked() {
                self.show_basic = false;
                self.basic_show_time = None;
            }
        }

        ui.add_space(20.0);

        // Notification with Icon
        ui.heading("Notification with Icon");
        ui.label("Notification with a leading icon (auto-dismisses after 5 seconds):");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Show With Icon")).clicked() {
                self.show_with_icon = true;
                self.with_icon_show_time = Some(Instant::now());
            }
            if self.show_with_icon {
                if ui.add(MaterialButton::text("Hide")).clicked() {
                    self.show_with_icon = false;
                    self.with_icon_show_time = None;
                }
            }
        });

        // Check for auto-dismiss
        if self.show_with_icon {
            if let Some(show_time) = self.with_icon_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_with_icon = false;
                    self.with_icon_show_time = None;
                }
            }
        }

        if self.show_with_icon {
            let response = ui.add(
                notification()
                    .title("Download Complete")
                    .text("Your file has been downloaded successfully")
                    .icon("download")
                    .align(self.notification_align)
                    .opened(true),
            );

            if response.clicked() {
                self.show_with_icon = false;
                self.with_icon_show_time = None;
            }
        }

        ui.add_space(20.0);

        // Notification with Subtitle
        ui.heading("Notification with Subtitle");
        ui.label("Notification with title, subtitle, and text (auto-dismisses after 5 seconds):");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Show With Subtitle")).clicked() {
                self.show_with_subtitle = true;
                self.with_subtitle_show_time = Some(Instant::now());
            }
            if self.show_with_subtitle {
                if ui.add(MaterialButton::text("Hide")).clicked() {
                    self.show_with_subtitle = false;
                    self.with_subtitle_show_time = None;
                }
            }
        });

        // Check for auto-dismiss
        if self.show_with_subtitle {
            if let Some(show_time) = self.with_subtitle_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_with_subtitle = false;
                    self.with_subtitle_show_time = None;
                }
            }
        }

        if self.show_with_subtitle {
            let response = ui.add(
                notification()
                    .title("System Update")
                    .subtitle("Version 2.0.1")
                    .text("A new system update is available for installation")
                    .align(self.notification_align)
                    .opened(true),
            );

            if response.clicked() {
                self.show_with_subtitle = false;
                self.with_subtitle_show_time = None;
            }
        }

        ui.add_space(20.0);

        // Full Notification
        ui.heading("Full Notification");
        ui.label("Notification with all features: icon, title, subtitle, text, and timestamp (auto-dismisses after 5 seconds):");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Show Full")).clicked() {
                self.show_full = true;
                self.full_show_time = Some(Instant::now());
            }
            if self.show_full {
                if ui.add(MaterialButton::text("Hide")).clicked() {
                    self.show_full = false;
                    self.full_show_time = None;
                }
            }
        });

        // Check for auto-dismiss
        if self.show_full {
            if let Some(show_time) = self.full_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_full = false;
                    self.full_show_time = None;
                }
            }
        }

        if self.show_full {
            let response = ui.add(
                notification()
                    .icon("notifications")
                    .title("Meeting Reminder")
                    .subtitle("Team Standup")
                    .text("Your daily standup meeting starts in 5 minutes")
                    .title_right_text("now")
                    .align(self.notification_align)
                    .opened(true),
            );

            if response.clicked() {
                self.show_full = false;
                self.full_show_time = None;
            }
        }

        ui.add_space(20.0);

        // Closeable Notification
        ui.heading("Closeable Notification");
        ui.label("Notification with a close button (auto-dismisses after 5 seconds or click close/notification):");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Show Closeable")).clicked() {
                self.show_closeable = true;
                self.closeable_show_time = Some(Instant::now());
            }
        });

        // Check for auto-dismiss
        if self.show_closeable {
            if let Some(show_time) = self.closeable_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_closeable = false;
                    self.closeable_show_time = None;
                }
            }
        }

        if self.show_closeable {
            let response = ui.add(
                notification()
                    .title("Dismissible Alert")
                    .text("Click the close button or anywhere on notification to dismiss")
                    .icon("info")
                    .closeable(true)
                    .align(self.notification_align)
                    .opened(true),
            );

            // Close button was clicked or notification was clicked
            if response.changed() || response.clicked() {
                self.show_closeable = false;
                self.closeable_show_time = None;
            }
        }

        ui.add_space(20.0);

        // Custom Notification Builder
        ui.heading("Custom Notification Builder");
        ui.label("Build your own notification (auto-dismisses after 5 seconds):");

        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.custom_title);
        });

        ui.horizontal(|ui| {
            ui.label("Subtitle:");
            ui.text_edit_singleline(&mut self.custom_subtitle);
        });

        ui.horizontal(|ui| {
            ui.label("Text:");
            ui.text_edit_singleline(&mut self.custom_text);
        });

        ui.horizontal(|ui| {
            ui.label("Icon:");
            ui.text_edit_singleline(&mut self.custom_icon);
            ui.label("(Material Symbol name)");
        });

        ui.horizontal(|ui| {
            ui.label("Right Text:");
            ui.text_edit_singleline(&mut self.custom_right_text);
        });

        ui.checkbox(&mut self.custom_closeable, "Closeable");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Show Custom")).clicked() {
                self.show_custom = true;
                self.custom_show_time = Some(Instant::now());
            }
        });

        // Check for auto-dismiss
        if self.show_custom {
            if let Some(show_time) = self.custom_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_custom = false;
                    self.custom_show_time = None;
                }
            }
        }

        if self.show_custom {
            let response = ui.add(
                notification()
                    .title(&self.custom_title)
                    .subtitle(&self.custom_subtitle)
                    .text(&self.custom_text)
                    .icon(&self.custom_icon)
                    .title_right_text(&self.custom_right_text)
                    .closeable(self.custom_closeable)
                    .align(self.notification_align)
                    .opened(true),
            );

            if (response.changed() && self.custom_closeable) || response.clicked() {
                self.show_custom = false;
                self.custom_show_time = None;
            }
        }

        ui.add_space(20.0);

        // Notification Stack Example
        ui.heading("Notification Stack (Overlay at Top)");
        ui.label("Multiple notifications stacked at the top of the window:");
        ui.label("These notifications appear as overlays and don't push content down");
        ui.label("Click buttons to show notifications (auto-dismiss after 5 seconds):");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::outlined("Show Email")).clicked() {
                self.show_stack_1 = true;
                self.stack_1_show_time = Some(Instant::now());
            }
            if ui.add(MaterialButton::outlined("Show Calendar")).clicked() {
                self.show_stack_2 = true;
                self.stack_2_show_time = Some(Instant::now());
            }
            if ui.add(MaterialButton::outlined("Show Update")).clicked() {
                self.show_stack_3 = true;
                self.stack_3_show_time = Some(Instant::now());
            }
            if ui.add(MaterialButton::filled_tonal("Show All")).clicked() {
                self.show_stack_1 = true;
                self.show_stack_2 = true;
                self.show_stack_3 = true;
                self.stack_1_show_time = Some(Instant::now());
                self.stack_2_show_time = Some(Instant::now());
                self.stack_3_show_time = Some(Instant::now());
            }
        });

        // Check for auto-dismiss
        if self.show_stack_1 {
            if let Some(show_time) = self.stack_1_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_stack_1 = false;
                    self.stack_1_show_time = None;
                }
            }
        }
        if self.show_stack_2 {
            if let Some(show_time) = self.stack_2_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_stack_2 = false;
                    self.stack_2_show_time = None;
                }
            }
        }
        if self.show_stack_3 {
            if let Some(show_time) = self.stack_3_show_time {
                if show_time.elapsed() >= Duration::from_secs(5) {
                    self.show_stack_3 = false;
                    self.stack_3_show_time = None;
                }
            }
        }

        // First notification at the top
        if self.show_stack_1 {
            let response = ui.add(
                notification()
                    .icon("mail")
                    .title("New Email")
                    .subtitle("john@example.com")
                    .text("You have received a new email from John Doe")
                    .title_right_text("2m ago")
                    .closeable(true)
                    .align(self.notification_align)
                    .opened(true),
            );

            if response.changed() || response.clicked() {
                self.show_stack_1 = false;
                self.stack_1_show_time = None;
            }
        }

        // Second notification stacked below the first
        if self.show_stack_2 {
            let response = ui.add(
                notification()
                    .icon("event")
                    .title("Calendar Event")
                    .subtitle("Team Meeting")
                    .text("Meeting starts at 2:00 PM")
                    .title_right_text("10m ago")
                    .closeable(true)
                    .align(self.notification_align)
                    .opened(true)
                    .with_offset(100.0), // Offset by 100px from top
            );

            if response.changed() || response.clicked() {
                self.show_stack_2 = false;
                self.stack_2_show_time = None;
            }
        }

        // Third notification stacked below the second
        if self.show_stack_3 {
            let response = ui.add(
                notification()
                    .icon("update")
                    .title("App Update Available")
                    .text("Version 3.0 is now available for download")
                    .title_right_text("1h ago")
                    .closeable(true)
                    .align(self.notification_align)
                    .opened(true)
                    .with_offset(200.0), // Offset by 200px from top
            );

            if response.changed() || response.clicked() {
                self.show_stack_3 = false;
                self.stack_3_show_time = None;
            }
        }

        ui.add_space(400.0); // Add space so content below is visible
        ui.label("Content below notifications (scroll to see)");
    }
}
