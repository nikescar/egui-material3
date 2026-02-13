use crate::get_global_color;
use eframe::egui::{self, Color32, Context, Id, Modal, Response, Sense, Stroke, Ui, Vec2};

/// Material Design dialog types following Material Design 3 specifications
#[derive(Clone, Copy, PartialEq)]
pub enum DialogType {
    /// Standard dialog for general purpose use
    Standard,
    /// Alert dialog for important notifications requiring acknowledgment
    Alert,
    /// Confirmation dialog for confirming actions before proceeding
    Confirm,
    /// Form dialog containing input fields and form elements
    Form,
}

/// Material Design dialog component following Material Design 3 specifications
///
/// Dialogs interrupt users with overlaid content that requires a response.
/// They appear above all other content and disable all app functionality when shown.
///
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut dialog_open = false;
///
/// // Basic dialog
/// let dialog = MaterialDialog::new("my_dialog", "Confirm Action", &mut dialog_open)
///     .content(|ui| {
///         ui.label("Are you sure you want to proceed?");
///     })
///     .action("Cancel", ActionType::Text, || {
///         // Cancel action
///     })
///     .action("Confirm", ActionType::Filled, || {
///         // Confirm action  
///     });
///
/// dialog.show(ui.ctx());
/// # });
/// ```
///
/// ## Material Design Spec
/// - Max width: 560dp on large screens
/// - Corner radius: 28dp
/// - Elevation: 6dp (24dp shadow)
/// - Surface color background
/// - Minimum touch target: 48x48dp for actions
pub struct MaterialDialog<'a> {
    /// Unique identifier for the dialog
    id: Id,
    /// Dialog title text
    title: String,
    /// Mutable reference to dialog open state
    open: &'a mut bool,
    /// Type of dialog (affects styling and behavior)
    dialog_type: DialogType,
    /// Optional icon to display in dialog header
    icon: Option<String>,
    /// Content rendering function called once
    content: Box<dyn FnOnce(&mut Ui) + 'a>,
    /// List of action buttons at the bottom of the dialog
    actions: Vec<DialogAction<'a>>,
    /// Whether this is a quick/temporary dialog
    quick: bool,
    /// Whether to disable focus trapping within the dialog
    no_focus_trap: bool,
    /// Maximum width constraint for the dialog
    max_width: Option<f32>,
}

/// Represents an action button in a Material Design dialog
pub struct DialogAction<'a> {
    /// Button text label
    text: String,
    /// Visual style of the action button
    action_type: ActionType,
    /// Whether the action is currently enabled
    _enabled: bool,
    /// Callback function executed when action is triggered
    action: Box<dyn FnOnce() + 'a>,
}

/// Material Design action button styles for dialogs
#[derive(Clone, Copy, PartialEq)]
pub enum ActionType {
    /// Text button - lowest emphasis, used for secondary actions
    Text,
    /// Filled tonal button - medium emphasis, used for secondary actions
    FilledTonal,
    /// Filled button - highest emphasis, used for primary actions
    Filled,
}

impl<'a> MaterialDialog<'a> {
    /// Create a new Material Design dialog
    ///
    /// ## Parameters
    /// - `id`: Unique identifier for the dialog (used for egui state)
    /// - `title`: Title text displayed at the top of the dialog
    /// - `open`: Mutable reference to boolean controlling dialog visibility
    ///
    /// ## Returns
    /// A new MaterialDialog instance ready for customization
    pub fn new(id: impl Into<Id>, title: impl Into<String>, open: &'a mut bool) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            open,
            dialog_type: DialogType::Standard,
            icon: None,
            content: Box::new(|_| {}),
            actions: Vec::new(),
            quick: false,
            no_focus_trap: false,
            max_width: None,
        }
    }

    /// Set the dialog type (affects styling and behavior)
    ///
    /// ## Parameters
    /// - `dialog_type`: The type of dialog to display
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }

    /// Set an optional icon to display in the dialog header
    ///
    /// ## Parameters
    /// - `icon`: The icon to display (as a string identifier)
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the content of the dialog
    ///
    /// ## Parameters
    /// - `content`: A closure that renders the content UI
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn content<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.content = Box::new(content);
        self
    }

    /// Set whether this is a quick/temporary dialog
    ///
    /// ## Parameters
    /// - `quick`: If true, the dialog is considered quick/temporary
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn quick(mut self, quick: bool) -> Self {
        self.quick = quick;
        self
    }

    /// Set whether to disable focus trapping within the dialog
    ///
    /// ## Parameters
    /// - `no_focus_trap`: If true, focus trapping is disabled
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn no_focus_trap(mut self, no_focus_trap: bool) -> Self {
        self.no_focus_trap = no_focus_trap;
        self
    }

    /// Set the maximum width constraint for the dialog
    ///
    /// ## Parameters
    /// - `width`: The maximum width in pixels
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Add a text action button to the dialog
    ///
    /// ## Parameters
    /// - `text`: The text label for the button
    /// - `action`: A closure that is called when the button is clicked
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn text_action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            action_type: ActionType::Text,
            _enabled: true,
            action: Box::new(action),
        });
        self
    }

    /// Add a filled tonal action button to the dialog
    ///
    /// ## Parameters
    /// - `text`: The text label for the button
    /// - `action`: A closure that is called when the button is clicked
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn filled_tonal_action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            action_type: ActionType::FilledTonal,
            _enabled: true,
            action: Box::new(action),
        });
        self
    }

    /// Add a filled action button to the dialog
    ///
    /// ## Parameters
    /// - `text`: The text label for the button
    /// - `action`: A closure that is called when the button is clicked
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn filled_action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            action_type: ActionType::Filled,
            _enabled: true,
            action: Box::new(action),
        });
        self
    }

    /// Backward compatibility methods
    ///
    /// These methods exist to support older code that used different naming conventions for actions.
    /// They are functionally equivalent to the more descriptively named methods introduced later.
    ///
    /// ## Parameters
    /// - `text`: The text label for the button
    /// - `action`: A closure that is called when the button is clicked
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.text_action(text, action)
    }

    /// Backward compatibility method for primary actions
    ///
    /// This method is provided for convenience and is functionally equivalent to `filled_action`.
    ///
    /// ## Parameters
    /// - `text`: The text label for the button
    /// - `action`: A closure that is called when the button is clicked
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn primary_action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.filled_action(text, action)
    }

    /// Show the dialog, rendering it in the given context
    ///
    /// ## Parameters
    /// - `ctx`: The egui context used for rendering the dialog
    ///
    /// ## Behavior
    /// - The dialog will be displayed as an overlay, blocking interaction with other windows
    /// - Clicking outside the dialog or pressing the escape key will close the dialog
    /// - Action buttons will execute their associated actions when clicked
    pub fn show(mut self, ctx: &Context) {
        if !*self.open {
            return;
        }

        let mut should_close = false;
        let mut pending_actions = Vec::new();

        // Extract values we need before moving into closure
        let dialog_width = self.max_width.unwrap_or(match self.dialog_type {
            DialogType::Alert => 280.0,
            DialogType::Confirm => 320.0,
            DialogType::Form => 800.0,
            DialogType::Standard => 400.0,
        });

        let title = self.title.clone();
        let icon = self.icon.clone();
        let actions = std::mem::take(&mut self.actions);
        let open_ref = self.open as *mut bool;

        let modal = Modal::new(self.id).show(ctx, |ui| {
            // ui.set_width(dialog_width);
            ui.set_min_width(dialog_width);
            ui.set_height(200.0);

            // Material Design colors
            let surface_container_high = get_global_color("surfaceContainerHigh");
            let on_surface = get_global_color("onSurface");
            let on_surface_variant = get_global_color("onSurfaceVariant");

            // Set dialog background
            ui.style_mut().visuals.window_fill = surface_container_high;
            ui.style_mut().visuals.panel_fill = surface_container_high;
            ui.style_mut().visuals.window_stroke = Stroke::NONE;

            ui.vertical(|ui| {
                ui.add_space(24.0);

                // Icon (if present) - positioned above headline
                if let Some(ref icon) = icon {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(0.0);
                        // Material icon placeholder
                        ui.label(
                            egui::RichText::new(icon)
                                .size(24.0)
                                .color(on_surface_variant),
                        );
                        ui.add_space(16.0);
                    });
                }

                // Headline
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add_space(24.0);
                    ui.label(
                        egui::RichText::new(&title)
                            .size(24.0)
                            .color(on_surface)
                            .family(egui::FontFamily::Proportional),
                    );
                    ui.add_space(24.0);
                });

                ui.add_space(16.0);

                // Content area (full width, no padding)
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    (self.content)(ui);
                });

                ui.add_space(24.0);

                // Actions area
                if !actions.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(24.0);

                        for (index, action) in actions.into_iter().enumerate().rev() {
                            let button_response = Self::draw_action_button_static(ui, &action);

                            if button_response.clicked() {
                                pending_actions.push((index, action.action));
                            }

                            ui.add_space(8.0);
                        }

                        ui.add_space(16.0); // Extra space from right edge
                    });

                    ui.add_space(24.0);
                }
            });
        });

        // Execute pending actions
        for (_index, action) in pending_actions {
            action();
            should_close = true;
        }

        // Handle modal close events (escape key, click outside, etc.)
        if modal.should_close() || should_close {
            unsafe {
                *open_ref = false;
            }
        }
    }

    fn draw_action_button_static(ui: &mut Ui, action: &DialogAction) -> Response {
        let primary = get_global_color("primary");
        let on_primary = get_global_color("onPrimary");
        let secondary_container = get_global_color("secondaryContainer");
        let on_secondary_container = get_global_color("onSecondaryContainer");
        let _on_surface_variant = get_global_color("onSurfaceVariant");

        let text_width = ui.fonts(|fonts| {
            fonts
                .layout_no_wrap(action.text.clone(), egui::FontId::default(), Color32::WHITE)
                .rect
                .width()
        });

        let button_width = (text_width + 24.0).max(64.0);
        let button_height = 40.0;
        let desired_size = Vec2::new(button_width, button_height);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        let (bg_color, text_color, _border_color) = match action.action_type {
            ActionType::Text => {
                if response.hovered() {
                    (
                        Color32::from_rgba_premultiplied(primary.r(), primary.g(), primary.b(), 20), // 8% opacity state layer
                        primary,
                        Color32::TRANSPARENT,
                    )
                } else {
                    (Color32::TRANSPARENT, primary, Color32::TRANSPARENT)
                }
            }
            ActionType::FilledTonal => {
                if response.hovered() {
                    (
                        secondary_container,
                        on_secondary_container,
                        Color32::TRANSPARENT,
                    )
                } else {
                    (
                        secondary_container,
                        on_secondary_container,
                        Color32::TRANSPARENT,
                    )
                }
            }
            ActionType::Filled => {
                if response.hovered() {
                    (primary, on_primary, Color32::TRANSPARENT)
                } else {
                    (primary, on_primary, Color32::TRANSPARENT)
                }
            }
        };

        // Draw button background
        ui.painter().rect_filled(
            rect, 20.0, // Full rounded corners
            bg_color,
        );

        // Draw state layer for pressed state
        if response.is_pointer_button_down_on() {
            let pressed_overlay = Color32::from_rgba_premultiplied(
                text_color.r(),
                text_color.g(),
                text_color.b(),
                31,
            ); // 12% opacity
            ui.painter().rect_filled(rect, 20.0, pressed_overlay);
        }

        // Draw button text
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &action.text,
            egui::FontId::proportional(14.0),
            text_color,
        );

        response
    }

    fn _draw_action_button(&self, ui: &mut Ui, action: &DialogAction) -> Response {
        Self::draw_action_button_static(ui, action)
    }
}

// Convenience constructors
/// Create a standard Material Design dialog
///
/// ## Parameters
/// - `id`: Unique identifier for the dialog (used for egui state)
/// - `title`: Title text displayed at the top of the dialog
/// - `open`: Mutable reference to boolean controlling dialog visibility
///
/// ## Returns
/// A new MaterialDialog instance configured as a standard dialog
pub fn dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open)
}

/// Create an alert dialog
///
/// ## Parameters
/// - `id`: Unique identifier for the dialog (used for egui state)
/// - `title`: Title text displayed at the top of the dialog
/// - `open`: Mutable reference to boolean controlling dialog visibility
///
/// ## Returns
/// A new MaterialDialog instance configured as an alert dialog
pub fn alert_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Alert)
}

/// Create a confirmation dialog
///
/// ## Parameters
/// - `id`: Unique identifier for the dialog (used for egui state)
/// - `title`: Title text displayed at the top of the dialog
/// - `open`: Mutable reference to boolean controlling dialog visibility
///
/// ## Returns
/// A new MaterialDialog instance configured as a confirmation dialog
pub fn confirm_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Confirm)
}

/// Create a form dialog
///
/// ## Parameters
/// - `id`: Unique identifier for the dialog (used for egui state)
/// - `title`: Title text displayed at the top of the dialog
/// - `open`: Mutable reference to boolean controlling dialog visibility
///
/// ## Returns
/// A new MaterialDialog instance configured as a form dialog
pub fn form_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Form)
}
