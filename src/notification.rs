use crate::theme::get_global_color;
use crate::material_symbol::material_symbol_text;
use egui::{
    ecolor::Color32, pos2, Area, FontId, Id, Order, Rect, Response, Sense, Stroke, Ui, Vec2, Widget,
};
use std::time::Duration;

/// Notification alignment position
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NotificationAlign {
    /// Left-aligned notifications
    Left,
    /// Center-aligned notifications
    Center,
    /// Right-aligned notifications
    Right,
}

/// Material Design notification component.
///
/// Notifications display system-style messages to users with support for titles,
/// subtitles, icons, and close buttons. They follow Material Design 3 specifications.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{notification, MaterialNotification};
///
/// // Simple notification
/// ui.add(notification()
///     .title("New Message")
///     .text("You have a new message from John Doe"));
///
/// // Notification with icon and close button
/// ui.add(notification()
///     .title("Download Complete")
///     .subtitle("Your file is ready")
///     .icon("download")
///     .closeable(true));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialNotification {
    /// Main title text
    title: Option<String>,
    /// Subtitle text
    subtitle: Option<String>,
    /// Main message text
    text: Option<String>,
    /// Icon name (Material Symbol)
    icon: Option<String>,
    /// Right-aligned text (e.g., timestamp)
    title_right_text: Option<String>,
    /// Whether the notification can be closed
    closeable: bool,
    /// Whether the notification is currently opened
    opened: bool,
    /// Auto-dismiss duration (None means no auto-dismiss)
    auto_dismiss: Option<Duration>,
    /// Custom background color
    bg_color: Option<Color32>,
    /// Custom width
    width: Option<f32>,
    /// Notification alignment
    align: NotificationAlign,
}

impl MaterialNotification {
    /// Create a new notification
    pub fn new() -> Self {
        Self {
            title: None,
            subtitle: None,
            text: None,
            icon: None,
            title_right_text: None,
            closeable: false,
            opened: true,
            auto_dismiss: None,
            bg_color: None,
            width: None,
            align: NotificationAlign::Center,
        }
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the subtitle
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set the text content
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the icon (Material Symbol name)
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the right-aligned title text (e.g., timestamp)
    pub fn title_right_text(mut self, text: impl Into<String>) -> Self {
        self.title_right_text = Some(text.into());
        self
    }

    /// Set whether the notification can be closed
    pub fn closeable(mut self, closeable: bool) -> Self {
        self.closeable = closeable;
        self
    }

    /// Set whether the notification is opened
    pub fn opened(mut self, opened: bool) -> Self {
        self.opened = opened;
        self
    }

    /// Set auto-dismiss duration
    pub fn auto_dismiss(mut self, duration: Duration) -> Self {
        self.auto_dismiss = Some(duration);
        self
    }

    /// Set custom background color
    pub fn bg_color(mut self, color: Color32) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set custom width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set notification alignment
    pub fn align(mut self, align: NotificationAlign) -> Self {
        self.align = align;
        self
    }

    /// Show the notification with a vertical offset for stacking.
    /// This is useful for displaying multiple notifications.
    pub fn with_offset(self, offset: f32) -> MaterialNotificationWithOffset {
        MaterialNotificationWithOffset {
            notification: self,
            vertical_offset: offset,
        }
    }
}

/// Notification with vertical offset for stacking
pub struct MaterialNotificationWithOffset {
    notification: MaterialNotification,
    vertical_offset: f32,
}

impl Default for MaterialNotification {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialNotification {
    fn ui(self, ui: &mut Ui) -> Response {
        self.ui_with_offset(ui, 0.0)
    }
}

impl MaterialNotification {
    fn ui_with_offset(self, ui: &mut Ui, vertical_offset: f32) -> Response {
        if !self.opened {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        // Get Material Design colors
        let surface_container_highest = get_global_color("surfaceContainerHighest");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let bg_color = self.bg_color.unwrap_or(surface_container_highest);

        // Calculate notification width
        let screen_rect = ui.ctx().content_rect();
        let max_width: f32 = 400.0;
        let width = self.width.unwrap_or(max_width.min(screen_rect.width() - 48.0));

        let padding = 12.0;
        let content_width = width - padding * 2.0;

        // Reserve space for icon if present
        let icon_size = 24.0;
        let icon_margin = 8.0;
        let has_icon = self.icon.is_some();
        let text_width = if has_icon {
            content_width - icon_size - icon_margin
        } else {
            content_width
        };

        // Calculate space for close button
        let close_button_space = if self.closeable { 40.0 } else { 0.0 };
        let available_text_width = text_width - close_button_space;

        // Pre-calculate all text layouts to determine height
        let title_galley = self.title.as_ref().map(|title_text| {
            ui.painter().layout(
                title_text.clone(),
                FontId::proportional(16.0),
                on_surface,
                available_text_width - if self.title_right_text.is_some() { 60.0 } else { 0.0 },
            )
        });

        let subtitle_galley = self.subtitle.as_ref().map(|subtitle_text| {
            ui.painter().layout(
                subtitle_text.clone(),
                FontId::proportional(14.0),
                on_surface_variant,
                available_text_width,
            )
        });

        let text_galley = self.text.as_ref().map(|content_text| {
            ui.painter().layout(
                content_text.clone(),
                FontId::proportional(14.0),
                on_surface_variant,
                available_text_width,
            )
        });

        let right_text_galley = self.title_right_text.as_ref().map(|right_text| {
            ui.painter().layout_no_wrap(
                right_text.clone(),
                FontId::proportional(12.0),
                on_surface_variant,
            )
        });

        // Calculate total height
        let mut total_height = padding * 2.0;
        if let Some(ref galley) = title_galley {
            total_height += galley.size().y + 4.0;
        }
        if let Some(ref galley) = subtitle_galley {
            total_height += galley.size().y + 4.0;
        }
        if let Some(ref galley) = text_galley {
            total_height += galley.size().y;
        }

        // Position notification based on alignment
        // Use screen_rect to keep notifications fixed in viewport (not affected by scrolling)
        // Add 50px to avoid being cropped by window header
        let screen_rect = ui.ctx().content_rect();
        let notification_x = match self.align {
            NotificationAlign::Left => screen_rect.min.x + 16.0,
            NotificationAlign::Center => screen_rect.min.x + (screen_rect.width() - width) / 2.0,
            NotificationAlign::Right => screen_rect.max.x - width - 16.0,
        };
        let notification_y = screen_rect.min.y + 16.0 + 50.0 + vertical_offset; // Top margin + header offset + stacking offset
        let notification_pos = pos2(notification_x, notification_y);

        // Create a unique ID for this notification based on its content
        let notification_id = Id::new("notification").with(self.title.as_deref().unwrap_or(""))
            .with(self.text.as_deref().unwrap_or(""))
            .with(vertical_offset as i32); // Convert f32 to i32 for Hash

        // Use Area to create a floating overlay on top of all content
        let area_response = Area::new(notification_id)
            .fixed_pos(notification_pos)
            .order(Order::Foreground) // Always on top
            .interactable(true)
            .show(ui.ctx(), |ui| {
                // Allocate space for the notification
                let (rect, mut response) = ui.allocate_exact_size(Vec2::new(width, total_height), Sense::click());
                let notification_rect = rect;

                // Draw background with rounded corners
                ui.painter().rect_filled(notification_rect, 12.0, bg_color);

                // Draw border
                ui.painter().rect_stroke(
                    notification_rect,
                    12.0,
                    Stroke::new(1.0, outline),
                    egui::epaint::StrokeKind::Outside,
                );

                // Now draw all content
                let mut current_y = notification_rect.min.y + padding;
                let left_margin = notification_rect.min.x + padding;
                let text_start_x = if has_icon {
                    left_margin + icon_size + icon_margin
                } else {
                    left_margin
                };

                // Draw icon if present
                if let Some(icon_name) = &self.icon {
                    let icon_text = material_symbol_text(icon_name);
                    let icon_galley = ui.painter().layout_no_wrap(
                        icon_text.to_string(),
                        FontId::proportional(icon_size),
                        on_surface,
                    );
                    let icon_pos = pos2(left_margin, current_y);
                    ui.painter().galley(icon_pos, icon_galley, on_surface);
                }

                // Draw title and right text
                if let Some(galley) = title_galley {
                    let title_pos = pos2(text_start_x, current_y);
                    ui.painter().galley(title_pos, galley.clone(), on_surface);

                    // Draw right text if present
                    if let Some(right_galley) = right_text_galley {
                        let right_pos = pos2(
                            notification_rect.max.x - padding - close_button_space - right_galley.size().x,
                            current_y,
                        );
                        ui.painter().galley(right_pos, right_galley, on_surface_variant);
                    }

                    current_y += galley.size().y + 4.0;
                }

                // Draw subtitle
                if let Some(galley) = subtitle_galley {
                    let subtitle_pos = pos2(text_start_x, current_y);
                    ui.painter().galley(subtitle_pos, galley.clone(), on_surface_variant);
                    current_y += galley.size().y + 4.0;
                }

                // Draw text content
                if let Some(galley) = text_galley {
                    let text_pos = pos2(text_start_x, current_y);
                    ui.painter().galley(text_pos, galley, on_surface_variant);
                }

                // Draw close button if closeable
                let mut close_clicked = false;
                if self.closeable {
                    let close_button_pos = pos2(
                        notification_rect.max.x - padding - 24.0,
                        notification_rect.min.y + padding,
                    );
                    let close_icon = material_symbol_text("close");
                    let close_galley = ui.painter().layout_no_wrap(
                        close_icon.to_string(),
                        FontId::proportional(20.0),
                        on_surface_variant,
                    );

                    let close_rect = Rect::from_center_size(
                        pos2(close_button_pos.x + 12.0, close_button_pos.y + 12.0),
                        Vec2::new(24.0, 24.0),
                    );

                    let close_response = ui.interact(close_rect, response.id.with("close"), Sense::click());

                    if close_response.hovered() {
                        ui.painter().circle_filled(close_rect.center(), 12.0, on_surface_variant.linear_multiply(0.1));
                    }

                    ui.painter().galley(close_button_pos, close_galley, on_surface_variant);

                    if close_response.clicked() {
                        close_clicked = true;
                        response.mark_changed();
                    }
                }

                // If notification was clicked (but not the close button), mark as clicked
                if response.clicked() && !close_clicked {
                    // Already marked by the response
                } else if close_clicked {
                    // Close button was clicked, already marked as changed
                }

                response
            });

        area_response.inner
    }
}

impl Widget for MaterialNotificationWithOffset {
    fn ui(self, ui: &mut Ui) -> Response {
        self.notification.ui_with_offset(ui, self.vertical_offset)
    }
}

/// Convenience function to create a notification
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::notification;
///
/// ui.add(notification()
///     .title("Notification Title")
///     .text("Notification message"));
/// # });
/// ```
pub fn notification() -> MaterialNotification {
    MaterialNotification::new()
}
