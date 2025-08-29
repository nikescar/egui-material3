use crate::theme::get_global_color;
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
    ///
    /// # Arguments
    /// * `message` - The message text to display in the snackbar
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("File saved successfully");
    /// # });
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            action_text: None,
            action_callback: None,
            visible: true,
            auto_dismiss: Some(Duration::from_secs(4)),
            show_time: None,
            position: SnackbarPosition::Bottom,
            corner_radius: CornerRadius::from(4.0), // Material Design small shape radius
            elevation: None,
        }
    }

    /// Add an action button to the snackbar.
    ///
    /// # Arguments
    /// * `text` - Text label for the action button
    /// * `callback` - Function to execute when the button is clicked
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("File deleted")
    ///     .action("Undo", || println!("Undo action performed"));
    /// # });
    /// ```
    pub fn action<F>(mut self, text: impl Into<String>, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.action_text = Some(text.into());
        self.action_callback = Some(Box::new(callback));
        self
    }

    /// Set auto-dismiss duration. Set to None to disable auto-dismiss.
    ///
    /// # Arguments
    /// * `duration` - How long to show the snackbar before auto-dismissing.
    ///                Use `None` to disable auto-dismiss.
    ///
    /// # Example
    /// ```rust
    /// use std::time::Duration;
    /// # egui::__run_test_ui(|ui| {
    /// // Auto-dismiss after 6 seconds
    /// let snackbar = MaterialSnackbar::new("Custom timeout")
    ///     .auto_dismiss(Some(Duration::from_secs(6)));
    ///
    /// // Never auto-dismiss
    /// let persistent = MaterialSnackbar::new("Persistent message")
    ///     .auto_dismiss(None);
    /// # });
    /// ```
    pub fn auto_dismiss(mut self, duration: Option<Duration>) -> Self {
        self.auto_dismiss = duration;
        self
    }

    /// Set the position of the snackbar.
    ///
    /// # Arguments
    /// * `position` - Where to position the snackbar (Bottom or Top)
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("Top notification")
    ///     .position(SnackbarPosition::Top);
    /// # });
    /// ```
    pub fn position(mut self, position: SnackbarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set corner radius for rounded corners.
    ///
    /// # Arguments
    /// * `corner_radius` - The corner radius value or CornerRadius struct
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("Rounded snackbar")
    ///     .corner_radius(8.0);
    /// # });
    /// ```
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set elevation shadow for the snackbar.
    ///
    /// # Arguments
    /// * `elevation` - Shadow configuration for elevation effect
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui::epaint::Shadow;
    /// let shadow = Shadow::small_dark();
    /// let snackbar = MaterialSnackbar::new("Elevated snackbar")
    ///     .elevation(shadow);
    /// # });
    /// ```
    pub fn elevation(mut self, elevation: impl Into<Shadow>) -> Self {
        self.elevation = Some(elevation.into());
        self
    }

    /// Show the snackbar only if the condition is true.
    /// 
    /// This method manages the visibility state properly and is useful for
    /// toggling snackbar visibility based on application state.
    ///
    /// # Arguments
    /// * `visible` - Mutable reference to a boolean controlling visibility
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut show_notification = true;
    /// let snackbar = MaterialSnackbar::new("Conditional message")
    ///     .show_if(&mut show_notification);
    /// # });
    /// ```
    pub fn show_if(mut self, visible: &mut bool) -> Self {
        self.visible = *visible;
        self
    }

    /// Show the snackbar with a vertical offset for stacking.
    /// This method is used by snackbar_window.rs to manage multiple snackbars.
    pub fn show_with_offset(mut self, visible: &mut bool, vertical_offset: f32) -> MaterialSnackbarWithOffset<'a> {
        self.visible = *visible;
        MaterialSnackbarWithOffset {
            snackbar: self,
            vertical_offset,
        }
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
        // Material Design spec: mix 80% on-surface with 20% surface
        let on_surface = get_global_color("onSurface");
        let surface = get_global_color("surface");
        
        // Mix colors: 80% on-surface + 20% surface
        let bg_color = Color32::from_rgba_unmultiplied(
            ((on_surface.r() as f32 * 0.8) + (surface.r() as f32 * 0.2)) as u8,
            ((on_surface.g() as f32 * 0.8) + (surface.g() as f32 * 0.2)) as u8,
            ((on_surface.b() as f32 * 0.8) + (surface.b() as f32 * 0.2)) as u8,
            255
        );
        
        (bg_color, None)
    }
}

impl Widget for MaterialSnackbar<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        if !self.visible {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        // Initialize show time when first rendered
        if self.show_time.is_none() {
            self.show_time = Some(Instant::now());
        }

        // Check auto-dismiss
        let should_auto_dismiss = if let (Some(auto_dismiss), Some(show_time)) = (self.auto_dismiss, self.show_time) {
            show_time.elapsed() >= auto_dismiss
        } else {
            false
        };
        
        if should_auto_dismiss {
            // Return empty response if auto-dismissed
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
            elevation: _,
        } = self;

        // Material Design spec dimensions and typography
        let label_text_color = get_global_color("surface"); // White text on dark background
        let action_text_color = get_global_color("inversePrimary"); // Material action color
        
        // First calculate action button size to determine available space for text
        let action_galley = action_text.as_ref().map(|text| {
            ui.painter().layout_no_wrap(
                text.clone(),
                egui::FontId::proportional(14.0),
                action_text_color
            )
        });
        
        // Calculate available width for message (leave space for action)
        let action_area_width = if action_galley.is_some() {
            action_galley.as_ref().unwrap().size().x + 64.0 // action + generous padding
        } else {
            0.0
        };
        
        let max_message_width = 600.0 - action_area_width; // reasonable max width
        
        // Calculate message text with width constraint to prevent overlap
        let text_galley = ui.painter().layout(
            message.clone(),
            egui::FontId::proportional(14.0), // body2 typography scale
            label_text_color,
            max_message_width.max(200.0) // minimum readable width
        );

        // Material Design padding: 16px left, 8px right, 14px top/bottom (minimum 48px height)
        let label_padding = Vec2::new(16.0, 14.0);
        let action_padding = Vec2::new(8.0, 14.0);
        let action_spacing = if action_text.is_some() { 8.0 } else { 0.0 };
        let action_width = action_galley.as_ref().map_or(0.0, |g| g.size().x + 32.0); // More generous button padding
        
        // Calculate width following Material Design constraints
        let content_width = text_galley.size().x + action_width + action_spacing + label_padding.x + action_padding.x;
        let min_width = 344.0; // Material Design min-width
        let max_width = 672.0; // Material Design max-width  
        let available_width = ui.available_width().max(min_width + 48.0) - 48.0; // 24px margins on each side, ensure positive
        
        let snackbar_width = content_width.max(min_width).min(max_width).min(available_width).max(min_width);
        
        // Calculate dynamic height based on wrapped text and action button
        let min_height = 48.0; // Material Design minimum height
        let text_height = text_galley.size().y;
        let action_height = if action_text.is_some() { 36.0 } else { 0.0 }; // Action button height
        let content_height = text_height.max(action_height);
        let snackbar_height = (content_height + label_padding.y * 2.0).max(min_height);
        
        let snackbar_size = Vec2::new(snackbar_width, snackbar_height);
        
        // Allocate the snackbar size first to ensure proper space allocation
        let (_allocated_rect, mut response) = ui.allocate_exact_size(snackbar_size, Sense::click());
        
        // For positioning, use screen coordinates but respect the allocated space
        let screen_rect = ui.ctx().screen_rect();
        let snackbar_x = (screen_rect.width() - snackbar_size.x).max(0.0) / 2.0;
        let snackbar_y = match position {
            SnackbarPosition::Bottom => screen_rect.height() - snackbar_size.y - 32.0,
            SnackbarPosition::Top => 32.0,
        };
        
        // Use the calculated position for drawing, but keep allocated_rect for interaction
        let snackbar_pos = egui::pos2(snackbar_x, snackbar_y);
        let snackbar_rect = Rect::from_min_size(snackbar_pos, snackbar_size);

        // Draw Material Design elevation 6dp shadow
        let shadow_layers = [
            (Vec2::new(0.0, 6.0), 10.0, Color32::from_rgba_unmultiplied(0, 0, 0, 20)),
            (Vec2::new(0.0, 1.0), 18.0, Color32::from_rgba_unmultiplied(0, 0, 0, 14)),
            (Vec2::new(0.0, 3.0), 5.0, Color32::from_rgba_unmultiplied(0, 0, 0, 12)),
        ];
        
        for (offset, blur_radius, color) in shadow_layers {
            let shadow_rect = snackbar_rect.translate(offset).expand(blur_radius / 2.0);
            ui.painter().rect_filled(shadow_rect, corner_radius, color);
        }

        // Draw snackbar background
        ui.painter().rect_filled(snackbar_rect, corner_radius, background_color);
        
        // Draw border if present
        if let Some(stroke) = border_stroke {
            ui.painter().rect_stroke(snackbar_rect, corner_radius, stroke, egui::epaint::StrokeKind::Outside);
        }

        // Draw message text with proper Material Design positioning
        // For multi-line text, align to the top with proper padding
        let text_pos = egui::pos2(
            snackbar_rect.min.x + label_padding.x,
            snackbar_rect.min.y + label_padding.y
        );
        ui.painter().galley(text_pos, text_galley, label_text_color);

        // Handle action button if present
        let mut action_clicked = false;
        
        if let (Some(_action_text), Some(action_galley)) = (action_text.as_ref(), action_galley.as_ref()) {
            // Material Design action button positioning (right-aligned with proper spacing)
            // Position action button at top-right, aligned with text baseline
            let action_rect = Rect::from_min_size(
                egui::pos2(
                    snackbar_rect.max.x - action_width - 8.0, // 8px right margin
                    snackbar_rect.min.y + label_padding.y - 6.0 // Align with text, slight adjustment
                ),
                Vec2::new(action_width, 36.0)
            );
            
            let action_response = ui.interact(action_rect, ui.next_auto_id(), Sense::click());
            
            // Material Design state layers for action button
            if action_response.hovered() {
                let hover_color = action_text_color.linear_multiply(0.04); // Material hover opacity
                ui.painter().rect_filled(action_rect, CornerRadius::from(4.0), hover_color);
            }
            if action_response.is_pointer_button_down_on() {
                let pressed_color = action_text_color.linear_multiply(0.10); // Material pressed opacity
                ui.painter().rect_filled(action_rect, CornerRadius::from(4.0), pressed_color);
            }
            
            // Action text centered in button
            let action_text_pos = egui::pos2(
                action_rect.center().x - action_galley.size().x / 2.0,
                action_rect.center().y - action_galley.size().y / 2.0
            );
            ui.painter().galley(
                action_text_pos, 
                action_galley.clone(), 
                action_text_color
            );
            
            if action_response.clicked() {
                if let Some(callback) = action_callback {
                    callback();
                }
                action_clicked = true;
            }
            
            response = response.union(action_response);
        }

        // Update response state
        if action_clicked {
            response = response.on_hover_text("Action clicked");
        }
        
        // Allow clicking outside action to dismiss (only for basic snackbars)
        if response.clicked() && action_text.is_none() {
            response = response.on_hover_text("Dismissed");
        }

        response
    }
}

/// A wrapper for MaterialSnackbar that includes vertical offset for stacking
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialSnackbarWithOffset<'a> {
    snackbar: MaterialSnackbar<'a>,
    vertical_offset: f32,
}

impl Widget for MaterialSnackbarWithOffset<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        if !self.snackbar.visible {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        // Initialize show time when first rendered
        if self.snackbar.show_time.is_none() {
            self.snackbar.show_time = Some(Instant::now());
        }

        // Check auto-dismiss
        let should_auto_dismiss = if let (Some(auto_dismiss), Some(show_time)) = (self.snackbar.auto_dismiss, self.snackbar.show_time) {
            show_time.elapsed() >= auto_dismiss
        } else {
            false
        };
        
        if should_auto_dismiss {
            // Return empty response if auto-dismissed
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }

        let (background_color, border_stroke) = self.snackbar.get_snackbar_style();
        
        let MaterialSnackbar {
            message,
            action_text,
            action_callback,
            visible: _,
            auto_dismiss: _,
            show_time: _,
            position,
            corner_radius,
            elevation: _,
        } = self.snackbar;

        // Material Design spec dimensions and typography
        let label_text_color = get_global_color("surface"); // White text on dark background
        let action_text_color = get_global_color("inversePrimary"); // Material action color
        
        // First calculate action button size to determine available space for text
        let action_galley = action_text.as_ref().map(|text| {
            ui.painter().layout_no_wrap(
                text.clone(),
                egui::FontId::proportional(14.0),
                action_text_color
            )
        });
        
        // Calculate available width for message (leave space for action)
        let action_area_width = if action_galley.is_some() {
            action_galley.as_ref().unwrap().size().x + 64.0 // action + generous padding
        } else {
            0.0
        };
        
        let max_message_width = 600.0 - action_area_width; // reasonable max width
        
        // Calculate message text with width constraint to prevent overlap
        let text_galley = ui.painter().layout(
            message.clone(),
            egui::FontId::proportional(14.0), // body2 typography scale
            label_text_color,
            max_message_width.max(200.0) // minimum readable width
        );

        // Material Design padding: 16px left, 8px right, 14px top/bottom for 48px height
        let label_padding = Vec2::new(16.0, 14.0);
        let action_padding = Vec2::new(8.0, 14.0);
        let action_spacing = if action_text.is_some() { 8.0 } else { 0.0 };
        let action_width = action_galley.as_ref().map_or(0.0, |g| g.size().x + 32.0); // More generous button padding
        
        // Calculate width following Material Design constraints
        let content_width = text_galley.size().x + action_width + action_spacing + label_padding.x + action_padding.x;
        let min_width = 344.0; // Material Design min-width
        let max_width = 672.0; // Material Design max-width  
        let available_width = ui.available_width().max(min_width + 48.0) - 48.0; // 24px margins on each side, ensure positive
        
        let snackbar_width = content_width.max(min_width).min(max_width).min(available_width).max(min_width);
        
        // Calculate dynamic height based on wrapped text and action button  
        let min_height = 48.0; // Material Design minimum height
        let text_height = text_galley.size().y;
        let action_height = if action_text.is_some() { 36.0 } else { 0.0 }; // Action button height
        let content_height = text_height.max(action_height);
        let snackbar_height = (content_height + label_padding.y * 2.0).max(min_height);
        
        let snackbar_size = Vec2::new(snackbar_width, snackbar_height);
        
        // Allocate the snackbar size first to ensure proper space allocation
        let (_allocated_rect, mut response) = ui.allocate_exact_size(snackbar_size, Sense::click());
        
        // For positioning, use screen coordinates with vertical offset for stacking
        let screen_rect = ui.ctx().screen_rect();
        let snackbar_x = (screen_rect.width() - snackbar_size.x).max(0.0) / 2.0;
        let snackbar_y = match position {
            SnackbarPosition::Bottom => screen_rect.height() - snackbar_size.y - 32.0 - self.vertical_offset,
            SnackbarPosition::Top => 32.0 + self.vertical_offset,
        };
        
        // Use the calculated position for drawing, but keep allocated_rect for interaction
        let snackbar_pos = egui::pos2(snackbar_x, snackbar_y);
        let snackbar_rect = Rect::from_min_size(snackbar_pos, snackbar_size);

        // Draw Material Design elevation 6dp shadow
        let shadow_layers = [
            (Vec2::new(0.0, 6.0), 10.0, Color32::from_rgba_unmultiplied(0, 0, 0, 20)),
            (Vec2::new(0.0, 1.0), 18.0, Color32::from_rgba_unmultiplied(0, 0, 0, 14)),
            (Vec2::new(0.0, 3.0), 5.0, Color32::from_rgba_unmultiplied(0, 0, 0, 12)),
        ];
        
        for (offset, blur_radius, color) in shadow_layers {
            let shadow_rect = snackbar_rect.translate(offset).expand(blur_radius / 2.0);
            ui.painter().rect_filled(shadow_rect, corner_radius, color);
        }

        // Draw snackbar background
        ui.painter().rect_filled(snackbar_rect, corner_radius, background_color);
        
        // Draw border if present
        if let Some(stroke) = border_stroke {
            ui.painter().rect_stroke(snackbar_rect, corner_radius, stroke, egui::epaint::StrokeKind::Outside);
        }

        // Draw message text with proper Material Design positioning
        // For multi-line text, align to the top with proper padding
        let text_pos = egui::pos2(
            snackbar_rect.min.x + label_padding.x,
            snackbar_rect.min.y + label_padding.y
        );
        ui.painter().galley(text_pos, text_galley, label_text_color);

        // Handle action button if present
        let mut action_clicked = false;
        
        if let (Some(_action_text), Some(action_galley)) = (action_text.as_ref(), action_galley.as_ref()) {
            // Material Design action button positioning (right-aligned with proper spacing)
            // Position action button at top-right, aligned with text baseline
            let action_rect = Rect::from_min_size(
                egui::pos2(
                    snackbar_rect.max.x - action_width - 8.0, // 8px right margin
                    snackbar_rect.min.y + label_padding.y - 6.0 // Align with text, slight adjustment
                ),
                Vec2::new(action_width, 36.0)
            );
            
            let action_response = ui.interact(action_rect, ui.next_auto_id(), Sense::click());
            
            // Material Design state layers for action button
            if action_response.hovered() {
                let hover_color = action_text_color.linear_multiply(0.04); // Material hover opacity
                ui.painter().rect_filled(action_rect, CornerRadius::from(4.0), hover_color);
            }
            if action_response.is_pointer_button_down_on() {
                let pressed_color = action_text_color.linear_multiply(0.10); // Material pressed opacity
                ui.painter().rect_filled(action_rect, CornerRadius::from(4.0), pressed_color);
            }
            
            // Action text centered in button
            let action_text_pos = egui::pos2(
                action_rect.center().x - action_galley.size().x / 2.0,
                action_rect.center().y - action_galley.size().y / 2.0
            );
            ui.painter().galley(
                action_text_pos, 
                action_galley.clone(), 
                action_text_color
            );
            
            if action_response.clicked() {
                if let Some(callback) = action_callback {
                    callback();
                }
                action_clicked = true;
            }
            
            response = response.union(action_response);
        }

        // Update response state
        if action_clicked {
            response = response.on_hover_text("Action clicked");
        }
        
        // Allow clicking outside action to dismiss (only for basic snackbars)
        if response.clicked() && action_text.is_none() {
            response = response.on_hover_text("Dismissed");
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