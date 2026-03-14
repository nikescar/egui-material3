use crate::theme::get_global_color;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke},
    pos2, Area, Id, Order, Rect, Response, Sense, Ui, Vec2,
};

/// Material Design action sheet component.
///
/// Action sheets present a set of actions to the user in a slide-up panel.
/// They appear from the bottom of the screen and can contain multiple groups of actions.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut sheet_open = false;
///
/// let sheet = MaterialActionSheet::new("my_sheet", &mut sheet_open)
///     .label("Do something")
///     .button("Button 1", || {
///         println!("Button 1 clicked");
///     })
///     .button("Button 2", || {
///         println!("Button 2 clicked");
///     })
///     .button("Cancel", || {
///         println!("Cancel clicked");
///     });
///
/// sheet.show(ui.ctx());
/// # });
/// ```
#[must_use = "You should call .show() to display the action sheet"]
pub struct MaterialActionSheet<'a> {
    /// Unique identifier for this action sheet
    id: Id,
    /// Reference to open/closed state
    open: &'a mut bool,
    /// Whether to show backdrop/scrim
    backdrop: bool,
    /// Whether tapping backdrop closes the sheet
    backdrop_dismissible: bool,
    /// Groups of action buttons
    groups: Vec<ActionGroup<'a>>,
    /// Current group being built
    current_group: ActionGroup<'a>,
    /// Maximum width of the sheet
    max_width: f32,
}

/// A group of action buttons with an optional label
pub struct ActionGroup<'a> {
    /// Optional label for this group
    pub label: Option<String>,
    /// Buttons in this group
    pub buttons: Vec<ActionButton<'a>>,
}

/// An individual action button in the sheet
pub struct ActionButton<'a> {
    /// Button text
    pub text: String,
    /// Whether text should be bold
    pub bold: bool,
    /// Optional callback
    pub on_click: Option<Box<dyn FnOnce() + 'a>>,
    /// Whether button is enabled
    pub enabled: bool,
}

impl<'a> MaterialActionSheet<'a> {
    /// Create a new action sheet
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this action sheet
    /// * `open` - Mutable reference to the open/closed state
    pub fn new(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self {
            id: id.into(),
            open,
            backdrop: true,
            backdrop_dismissible: true,
            groups: Vec::new(),
            current_group: ActionGroup {
                label: None,
                buttons: Vec::new(),
            },
            max_width: 448.0, // Material Design 3 max width for mobile
        }
    }

    /// Set whether to show backdrop (default: true)
    pub fn backdrop(mut self, show: bool) -> Self {
        self.backdrop = show;
        self
    }

    /// Set whether tapping backdrop dismisses the sheet (default: true)
    pub fn backdrop_dismissible(mut self, dismissible: bool) -> Self {
        self.backdrop_dismissible = dismissible;
        self
    }

    /// Set the maximum width of the action sheet
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Add a label for the current group
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.current_group.label = Some(text.into());
        self
    }

    /// Add an action button to the current group
    pub fn button<F>(mut self, text: impl Into<String>, callback: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.current_group.buttons.push(ActionButton {
            text: text.into(),
            bold: false,
            on_click: Some(Box::new(callback)),
            enabled: true,
        });
        self
    }

    /// Add a bold action button to the current group
    pub fn bold_button<F>(mut self, text: impl Into<String>, callback: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.current_group.buttons.push(ActionButton {
            text: text.into(),
            bold: true,
            on_click: Some(Box::new(callback)),
            enabled: true,
        });
        self
    }

    /// Add a button without callback
    pub fn simple_button(mut self, text: impl Into<String>) -> Self {
        self.current_group.buttons.push(ActionButton {
            text: text.into(),
            bold: false,
            on_click: None,
            enabled: true,
        });
        self
    }

    /// Start a new group (finalizes the current group)
    pub fn new_group(mut self) -> Self {
        if !self.current_group.buttons.is_empty() || self.current_group.label.is_some() {
            let group = std::mem::replace(
                &mut self.current_group,
                ActionGroup {
                    label: None,
                    buttons: Vec::new(),
                },
            );
            self.groups.push(group);
        }
        self
    }

    /// Show the action sheet
    pub fn show(mut self, ctx: &egui::Context) -> Response {
        // Finalize any remaining group
        if !self.current_group.buttons.is_empty() || self.current_group.label.is_some() {
            let group = std::mem::replace(
                &mut self.current_group,
                ActionGroup {
                    label: None,
                    buttons: Vec::new(),
                },
            );
            self.groups.push(group);
        }

        if !*self.open {
            // Return empty response when closed
            return Area::new(self.id.with("dummy"))
                .fixed_pos(pos2(-1000.0, -1000.0))
                .show(ctx, |ui| ui.allocate_response(Vec2::ZERO, Sense::hover()))
                .response;
        }

        // Handle ESC key
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *self.open = false;
        }

        let screen_rect = ctx.viewport_rect();
        let backdrop_dismissible = self.backdrop_dismissible;

        // Draw backdrop/scrim
        if self.backdrop {
            let scrim_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);

            Area::new(self.id.with("backdrop"))
                .order(Order::Middle)
                .fixed_pos(pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    let scrim_response = ui.allocate_response(screen_rect.size(), Sense::click());
                    ui.painter()
                        .rect_filled(screen_rect, CornerRadius::ZERO, scrim_color);

                    if scrim_response.clicked() && backdrop_dismissible {
                        *self.open = false;
                    }
                });
        }

        // Draw action sheet
        let open_ptr = self.open as *mut bool;
        Area::new(self.id.with("sheet"))
            .order(Order::Foreground)
            .anchor(egui::Align2::CENTER_BOTTOM, Vec2::new(0.0, 0.0))
            .show(ctx, |ui| {
                let width = self.max_width.min(screen_rect.width());
                ui.set_width(width);

                // SAFETY: We're only using this pointer within this closure
                // and it doesn't outlive the mutable borrow
                unsafe { self.render_content(ui, &mut *open_ptr) }
            })
            .response
    }

    fn render_content(self, ui: &mut Ui, open: &'a mut bool) -> Response {
        let background_color = get_global_color("surfaceContainer");
        let corner_radius = CornerRadius {
            nw: 16,
            ne: 16,
            sw: 0,
            se: 0,
        };

        let mut total_height = 0.0;
        let button_height = 48.0; // Material Design 3 touch target

        // Calculate total height
        for group in &self.groups {
            if group.label.is_some() {
                total_height += button_height;
            }
            total_height += group.buttons.len() as f32 * button_height;
        }

        // Add safe area padding at bottom
        total_height += 8.0;

        let available_rect = ui.available_rect_before_wrap();
        let sheet_size = Vec2::new(available_rect.width(), total_height);
        let sheet_rect = Rect::from_min_size(
            pos2(available_rect.min.x, available_rect.max.y - total_height),
            sheet_size,
        );

        // Draw background
        ui.painter()
            .rect_filled(sheet_rect, corner_radius, background_color);

        let mut current_y = sheet_rect.min.y;
        let mut response = ui.allocate_response(sheet_size, Sense::hover());

        // Collect callbacks to execute after rendering
        let mut callbacks_to_execute: Vec<Box<dyn FnOnce()>> = Vec::new();

        // Store groups length before moving
        let groups_len = self.groups.len();

        // Render groups
        for (group_idx, group) in self.groups.into_iter().enumerate() {
            // Draw label if present
            if let Some(label) = &group.label {
                let label_rect = Rect::from_min_size(
                    pos2(sheet_rect.min.x, current_y),
                    Vec2::new(sheet_rect.width(), button_height),
                );

                let text_color = get_global_color("primary");
                let text_pos = pos2(label_rect.min.x + 16.0, current_y + button_height / 2.0);

                ui.painter().text(
                    text_pos,
                    egui::Align2::LEFT_CENTER,
                    label,
                    egui::FontId::proportional(14.0),
                    text_color,
                );

                current_y += button_height;
            }

            // Draw buttons
            for (button_idx, mut button) in group.buttons.into_iter().enumerate() {
                let button_id = self.id.with("group").with(group_idx).with(button_idx);
                let button_rect = Rect::from_min_size(
                    pos2(sheet_rect.min.x, current_y),
                    Vec2::new(sheet_rect.width(), button_height),
                );

                let button_response = ui.interact(button_rect, button_id, Sense::click());

                // Hover effect
                if button_response.hovered() && button.enabled {
                    let hover_color = get_global_color("onSurface").linear_multiply(0.08);
                    ui.painter()
                        .rect_filled(button_rect, CornerRadius::ZERO, hover_color);
                }

                // Draw button text
                let text_color = if !button.enabled {
                    get_global_color("onSurface").linear_multiply(0.38)
                } else {
                    get_global_color("onSurface")
                };

                let font_id = if button.bold {
                    egui::FontId::proportional(16.0)
                } else {
                    egui::FontId::proportional(16.0)
                };

                let text_pos = pos2(button_rect.min.x + 16.0, current_y + button_height / 2.0);

                ui.painter().text(
                    text_pos,
                    egui::Align2::LEFT_CENTER,
                    &button.text,
                    font_id,
                    text_color,
                );

                // Handle click
                if button_response.clicked() && button.enabled {
                    *open = false;
                    if let Some(callback) = button.on_click.take() {
                        callbacks_to_execute.push(callback);
                    }
                }

                response = response.union(button_response);
                current_y += button_height;
            }

            // Draw divider between groups
            if group_idx < groups_len - 1 {
                let divider_y = current_y;
                ui.painter().line_segment(
                    [
                        pos2(sheet_rect.min.x, divider_y),
                        pos2(sheet_rect.max.x, divider_y),
                    ],
                    Stroke::new(1.0, get_global_color("outlineVariant")),
                );
            }
        }

        // Execute callbacks after rendering
        for callback in callbacks_to_execute {
            callback();
        }

        response
    }
}

/// Convenience function to create an action sheet
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut open = false;
/// action_sheet("my_sheet", &mut open)
///     .button("Action 1", || {})
///     .button("Action 2", || {})
///     .show(ui.ctx());
/// # });
/// ```
pub fn action_sheet<'a>(id: impl Into<Id>, open: &'a mut bool) -> MaterialActionSheet<'a> {
    MaterialActionSheet::new(id, open)
}
