use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius, Shadow},
    Rect, Response, Sense, Ui, Vec2, Widget,
};
use std::time::{Duration, Instant};

/// Material Design snackbar component.
///
/// Snackbars provide brief messages about app processes at the bottom of the screen.
/// They inform users of a process that an app has performed or will perform.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let mut snackbar_visible = true;
/// let mut snackbar = MaterialSnackbar::new("File deleted successfully")
///     .action("Undo", || println!("Undo clicked!"))
///     .show_if(&mut snackbar_visible);
///
/// ui.add(snackbar);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialSnackbar<'a> {
    message: String,
    action_text: Option<String>,
    action_callback: Option<Box<dyn Fn() + Send + Sync + 'a>>,
    visible: bool,
    auto_dismiss: Option<Duration>,
    show_time: Option<Instant>,
    position: SnackbarPosition,
    corner_radius: CornerRadius,
    elevation: Option<Shadow>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnackbarPosition {
    Bottom,
    Top,
}

impl<'a> MaterialSnackbar<'a> {
    /// Create a new snackbar with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            action_text: None,
            action_callback: None,
            visible: true,
            auto_dismiss: Some(Duration::from_secs(4)),
            show_time: None,
            position: SnackbarPosition::Bottom,
            corner_radius: CornerRadius::from(4.0),
            elevation: None,
        }
    }

    /// Add an action button to the snackbar.
    pub fn action<F>(mut self, text: impl Into<String>, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.action_text = Some(text.into());
        self.action_callback = Some(Box::new(callback));
        self
    }

    /// Set auto-dismiss duration. Set to None to disable auto-dismiss.
    pub fn auto_dismiss(mut self, duration: Option<Duration>) -> Self {
        self.auto_dismiss = duration;
        self
    }

    /// Set the position of the snackbar.
    pub fn position(mut self, position: SnackbarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set corner radius.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set elevation shadow.
    pub fn elevation(mut self, elevation: impl Into<Shadow>) -> Self {
        self.elevation = Some(elevation.into());
        self
    }

    /// Show the snackbar only if the condition is true.
    pub fn show_if(mut self, visible: &mut bool) -> Self {
        self.visible = *visible;
        if !self.visible {
            *visible = false;
        }
        self
    }

    /// Show the snackbar and set up auto-dismiss.
    pub fn show(mut self) -> Self {
        self.visible = true;
        if self.show_time.is_none() {
            self.show_time = Some(Instant::now());
        }
        self
    }

    /// Hide the snackbar.
    pub fn hide(mut self) -> Self {
        self.visible = false;
        self
    }

    fn get_snackbar_style(&self) -> (Color32, Option<Stroke>) {
        let md_inverse_surface = Color32::from_rgb(49, 48, 51); // Dark surface for snackbar
        (md_inverse_surface, None)
    }
}

impl Widget for MaterialSnackbar<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        // Initialize show time if not set
        if self.show_time.is_none() && self.visible {
            self.show_time = Some(Instant::now());
        }

        // Check auto-dismiss
        if self.visible {
            if let (Some(auto_dismiss), Some(show_time)) = (self.auto_dismiss, self.show_time) {
                if show_time.elapsed() >= auto_dismiss {
                    self.visible = false;
                }
            }
        }

        if !self.visible {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        let (background_color, border_stroke) = self.get_snackbar_style();
        
        let MaterialSnackbar {
            message,
            action_text,
            action_callback,
            visible: _,
            auto_dismiss: _,
            show_time: _,
            position,
            corner_radius,
            elevation,
        } = self;

        // Calculate snackbar dimensions
        let text_galley = ui.painter().layout_no_wrap(
            message.clone(),
            egui::FontId::proportional(14.0),
            Color32::WHITE
        );
        
        let action_galley = action_text.as_ref().map(|text| {
            ui.painter().layout_no_wrap(
                text.clone(),
                egui::FontId::proportional(14.0),
                Color32::from_rgb(187, 134, 252) // Primary color for action
            )
        });

        let padding = Vec2::new(16.0, 12.0);
        let action_spacing = if action_text.is_some() { 24.0 } else { 0.0 };
        let action_width = action_galley.as_ref().map_or(0.0, |g| g.size().x + 16.0); // Add padding for button
        
        let snackbar_width = text_galley.size().x + action_width + action_spacing + padding.x * 2.0;
        let snackbar_height = text_galley.size().y.max(32.0) + padding.y * 2.0; // Min height for touch target
        
        let snackbar_size = Vec2::new(snackbar_width.min(ui.available_width() - 32.0), snackbar_height);
        
        // Position the snackbar
        let screen_rect = ui.ctx().screen_rect();
        let snackbar_x = (screen_rect.width() - snackbar_size.x) / 2.0;
        let snackbar_y = match position {
            SnackbarPosition::Bottom => screen_rect.height() - snackbar_size.y - 32.0,
            SnackbarPosition::Top => 32.0,
        };
        
        let snackbar_pos = egui::pos2(snackbar_x, snackbar_y);
        let snackbar_rect = Rect::from_min_size(snackbar_pos, snackbar_size);

        // Draw elevation shadow if present
        if let Some(_shadow) = elevation {
            let shadow_rect = snackbar_rect.translate(Vec2::new(0.0, 2.0));
            ui.painter().rect_filled(
                shadow_rect,
                corner_radius,
                Color32::from_rgba_unmultiplied(0, 0, 0, 60),
            );
        }

        // Draw snackbar background
        ui.painter().rect_filled(snackbar_rect, corner_radius, background_color);
        
        // Draw border if present
        if let Some(stroke) = border_stroke {
            ui.painter().rect_stroke(snackbar_rect, corner_radius, stroke, egui::epaint::StrokeKind::Outside);
        }

        // Draw message text
        let text_pos = egui::pos2(
            snackbar_rect.min.x + padding.x,
            snackbar_rect.center().y - text_galley.size().y / 2.0
        );
        ui.painter().galley(text_pos, text_galley, Color32::WHITE);

        // Draw action button if present
        let mut response = ui.interact(snackbar_rect, ui.next_auto_id(), Sense::hover());
        
        if let (Some(action_text), Some(action_galley)) = (action_text.as_ref(), action_galley.as_ref()) {
            let action_rect = Rect::from_min_size(
                egui::pos2(
                    snackbar_rect.max.x - action_width - padding.x,
                    snackbar_rect.center().y - 16.0
                ),
                Vec2::new(action_width, 32.0)
            );
            
            let action_response = ui.interact(action_rect, ui.next_auto_id(), Sense::click());
            
            // Action button background on hover
            if action_response.hovered() {
                let hover_color = Color32::from_rgba_unmultiplied(187, 134, 252, 20);
                ui.painter().rect_filled(action_rect, CornerRadius::from(4.0), hover_color);
            }
            
            // Action text
            let action_text_pos = egui::pos2(
                action_rect.center().x - action_galley.size().x / 2.0,
                action_rect.center().y - action_galley.size().y / 2.0
            );
            ui.painter().galley(
                action_text_pos, 
                action_galley.clone(), 
                Color32::from_rgb(187, 134, 252)
            );
            
            if action_response.clicked() {
                if let Some(callback) = action_callback {
                    callback();
                }
            }
            
            response = response.union(action_response);
        }

        // Allow dismissing by clicking anywhere on the snackbar
        if response.clicked() && action_text.is_none() {
            // Dismiss snackbar (in a real implementation, you'd set visible to false)
        }

        response
    }
}

/// Convenience function to create a simple snackbar.
pub fn snackbar(message: impl Into<String>) -> MaterialSnackbar<'static> {
    MaterialSnackbar::new(message)
}

/// Convenience function to create a snackbar with an action.
pub fn snackbar_with_action<F>(
    message: impl Into<String>, 
    action_text: impl Into<String>, 
    callback: F
) -> MaterialSnackbar<'static>
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialSnackbar::new(message).action(action_text, callback)
}