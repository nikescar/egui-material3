use crate::get_global_color;
use egui::{self, Color32, Context, Id, Modal, Response, Sense, Stroke, Ui, Vec2};

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
    /// Minimum width constraint for the dialog (default: 280dp)
    min_width: Option<f32>,
    /// Maximum height constraint for the dialog
    max_height: Option<f32>,
    /// Padding around the title (default: 24dp horizontal, varies vertical)
    title_padding: Option<[f32; 4]>,
    /// Padding around the content (default: 24dp horizontal, 16dp top, 24dp bottom)
    content_padding: Option<[f32; 4]>,
    /// Padding around the actions area (default: 24dp all sides)
    actions_padding: Option<[f32; 4]>,
    /// Padding around individual action buttons
    button_padding: Option<[f32; 2]>,
    /// Whether content should be scrollable
    scrollable: bool,
    /// Spacing between action buttons (default: 8dp)
    actions_spacing: f32,
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
            min_width: Some(280.0),
            max_height: None,
            title_padding: None,
            content_padding: None,
            actions_padding: None,
            button_padding: None,
            scrollable: false,
            actions_spacing: 8.0,
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

    /// Set the minimum width constraint for the dialog
    ///
    /// ## Parameters
    /// - `width`: The minimum width in pixels (default: 280.0)
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set the maximum height constraint for the dialog
    ///
    /// ## Parameters
    /// - `height`: The maximum height in pixels
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set custom padding for the title area
    ///
    /// ## Parameters
    /// - `padding`: [left, right, top, bottom] padding in pixels
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn title_padding(mut self, padding: [f32; 4]) -> Self {
        self.title_padding = Some(padding);
        self
    }

    /// Set custom padding for the content area
    ///
    /// ## Parameters
    /// - `padding`: [left, right, top, bottom] padding in pixels
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn content_padding(mut self, padding: [f32; 4]) -> Self {
        self.content_padding = Some(padding);
        self
    }

    /// Set custom padding for the actions area
    ///
    /// ## Parameters
    /// - `padding`: [left, right, top, bottom] padding in pixels
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn actions_padding(mut self, padding: [f32; 4]) -> Self {
        self.actions_padding = Some(padding);
        self
    }

    /// Set custom padding for individual action buttons
    ///
    /// ## Parameters
    /// - `padding`: [horizontal, vertical] padding in pixels
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn button_padding(mut self, padding: [f32; 2]) -> Self {
        self.button_padding = Some(padding);
        self
    }

    /// Set whether the content should be scrollable
    ///
    /// ## Parameters
    /// - `scrollable`: If true, content will be placed in a ScrollArea
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    /// Set the spacing between action buttons
    ///
    /// ## Parameters
    /// - `spacing`: Spacing in pixels (default: 8.0)
    ///
    /// ## Returns
    /// Self for method chaining
    pub fn actions_spacing(mut self, spacing: f32) -> Self {
        self.actions_spacing = spacing;
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
        let default_width: f32 = match self.dialog_type {
            DialogType::Alert => 280.0,
            DialogType::Confirm => 320.0,
            DialogType::Form => 560.0,
            DialogType::Standard => 400.0,
        };
        
        let dialog_min_width = self.min_width.unwrap_or(280.0);
        let dialog_max_width = self.max_width.unwrap_or(default_width.max(560.0));
        let dialog_max_height = self.max_height;
        
        // Calculate reasonable max height based on screen size if not specified
        let screen_height = ctx.screen_rect().height();
        let effective_max_height = dialog_max_height.unwrap_or((screen_height * 0.9).min(800.0));

        let title = self.title.clone();
        let icon = self.icon.clone();
        let actions = std::mem::take(&mut self.actions);
        let open_ref = self.open as *mut bool;
        
        let title_padding = self.title_padding;
        let content_padding = self.content_padding;
        let actions_padding = self.actions_padding;
        let button_padding = self.button_padding;
        let scrollable = self.scrollable;
        let actions_spacing = self.actions_spacing;

        // Configure Modal frame with top/bottom margin for proper padding
        let modal_frame = egui::Frame::default()
            .inner_margin(egui::vec2(0.0, 24.0))
            .fill(get_global_color("surfaceContainerHigh"))
            .rounding(egui::Rounding::same(28))
            .stroke(Stroke::NONE);
        
        let modal = Modal::new(self.id)
            .frame(modal_frame)
            .show(ctx, |ui| {
            ui.set_min_width(dialog_min_width);
            ui.set_max_width(dialog_max_width);
            // Only set max_height for scrollable dialogs to avoid empty space at bottom
            if scrollable {
                ui.set_max_height(effective_max_height);
            }

            // Material Design colors
            let surface_container_high = get_global_color("surfaceContainerHigh");
            let on_surface = get_global_color("onSurface");
            let on_surface_variant = get_global_color("onSurfaceVariant");

            // Set dialog background
            ui.style_mut().visuals.window_fill = surface_container_high;
            ui.style_mut().visuals.panel_fill = surface_container_high;
            ui.style_mut().visuals.window_stroke = Stroke::NONE;
            
            // Remove all automatic spacing and margins  
            // ui.spacing_mut().item_spacing.y = 0.0;
            // ui.spacing_mut().window_margin = egui::Margin::ZERO;
            
            ui.vertical(|ui| {
                // ui.spacing_mut().item_spacing.y = 0.0;
                // Top padding now handled by Modal frame margin

                // Icon (if present) - positioned above headline, centered
                if let Some(ref icon) = icon {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(0.0);
                        // Material icon - centered above title
                        // Use MaterialIcon for proper icon rendering
                        let icon_widget = crate::icon::MaterialIcon::new(crate::material_symbol::material_symbol_text(icon))
                            .size(24.0)
                            .color(on_surface_variant);
                        ui.add(icon_widget);
                        ui.add_space(16.0);
                    });
                }

                // Headline with custom padding support
                let [title_left, title_right, title_top, title_bottom] = 
                    title_padding.unwrap_or([24.0, 24.0, 0.0, 0.0]);
                
                ui.horizontal(|ui| {
                    ui.add_space(title_left);
                    // Center title if there's an icon
                    let layout = if icon.is_some() {
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight)
                    } else {
                        egui::Layout::left_to_right(egui::Align::TOP)
                    };
                    ui.with_layout(layout, |ui| {
                        ui.label(
                            egui::RichText::new(&title)
                                .size(24.0)
                                .color(on_surface)
                                .family(egui::FontFamily::Proportional),
                        );
                    });
                    ui.add_space(title_right);
                });

                ui.add_space(if title_bottom > 0.0 { title_bottom } else { 16.0 });

                // Content area with optional scrolling and custom padding
                let [content_left, content_right, content_top, content_bottom] = 
                    content_padding.unwrap_or([24.0, 24.0, 0.0, 24.0]);
                
                if scrollable {
                    // Scrollable content - use fixed width area
                    let scroll_width = ui.available_width() - content_left - content_right;
                    let scroll_height = ui.available_height() - content_bottom;
                    
                    ui.horizontal(|ui| {
                        ui.add_space(content_left);
                        
                        // Allocate fixed space for scroll area
                        ui.allocate_ui_with_layout(
                            egui::vec2(scroll_width, scroll_height),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                egui::ScrollArea::vertical()
                                    .id_salt("dialog_content_scroll")
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        ui.set_width(scroll_width - 20.0); // Account for scrollbar
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                                        if content_top > 0.0 {
                                            ui.add_space(content_top);
                                        }
                                        (self.content)(ui);
                                    });
                            },
                        );
                        
                        ui.add_space(content_right);
                    });
                } else {
                    // Non-scrollable content - render directly with width constraint
                    let content_width = ui.available_width() - content_left - content_right;
                    ui.horizontal(|ui| {
                        ui.add_space(content_left);
                        ui.vertical(|ui| {
                            ui.set_max_width(content_width);
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                            if content_top > 0.0 {
                                ui.add_space(content_top);
                            }
                            (self.content)(ui);
                            // Don't consume remaining vertical space - let content size naturally
                        });
                        ui.add_space(content_right);
                    });
                }

                // Actions area with custom padding and spacing
                if !actions.is_empty() {
                    let [actions_left, actions_right, actions_top, actions_bottom] = 
                        actions_padding.unwrap_or([24.0, 24.0, 0.0, 0.0]);
                    
                    // Add spacing between content and actions
                    // Use actions_top if specified, otherwise use smaller default spacing
                    let spacing_before_actions = if actions_top > 0.0 { 
                        actions_top 
                    } else if content_bottom > 0.0 { 
                        content_bottom.min(16.0) 
                    } else { 
                        16.0 
                    };
                    ui.add_space(spacing_before_actions);
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(actions_right);

                        for (index, action) in actions.into_iter().enumerate().rev() {
                            let button_response = Self::draw_action_button_static(ui, &action, button_padding);

                            if button_response.clicked() {
                                pending_actions.push((index, action.action));
                            }

                            if index > 0 {
                                ui.add_space(actions_spacing);
                            }
                        }

                        ui.add_space(actions_left);
                    });
                    // Bottom padding now handled by Modal frame margin
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

    fn draw_action_button_static(ui: &mut Ui, action: &DialogAction, button_padding: Option<[f32; 2]>) -> Response {
        let primary = get_global_color("primary");
        let on_primary = get_global_color("onPrimary");
        let secondary_container = get_global_color("secondaryContainer");
        let on_secondary_container = get_global_color("onSecondaryContainer");
        let _on_surface_variant = get_global_color("onSurfaceVariant");

        let [btn_h_padding, btn_v_padding] = button_padding.unwrap_or([12.0, 8.0]);
        
        let text_width = ui.fonts(|fonts| {
            fonts
                .layout_no_wrap(action.text.clone(), egui::FontId::default(), Color32::WHITE)
                .rect
                .width()
        });

        let button_width = (text_width + btn_h_padding * 2.0).max(64.0);
        let button_height = (20.0 + btn_v_padding * 2.0).max(40.0);
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
        Self::draw_action_button_static(ui, action, self.button_padding)
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
