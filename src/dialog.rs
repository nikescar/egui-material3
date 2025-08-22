use eframe::egui::{self, Color32, Stroke, Ui, Context, Modal, Id, Vec2, Sense, Response};
use crate::get_global_color;

#[derive(Clone, Copy, PartialEq)]
pub enum DialogType {
    Standard,
    Alert,
    Confirm,
    Form,
}

pub struct MaterialDialog<'a> {
    id: Id,
    title: String,
    open: &'a mut bool,
    dialog_type: DialogType,
    icon: Option<String>,
    content: Box<dyn FnOnce(&mut Ui) + 'a>,
    actions: Vec<DialogAction<'a>>,
    quick: bool,
    no_focus_trap: bool,
    max_width: Option<f32>,
}

pub struct DialogAction<'a> {
    text: String,
    action_type: ActionType,
    enabled: bool,
    action: Box<dyn FnOnce() + 'a>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ActionType {
    Text,
    FilledTonal,
    Filled,
}

impl<'a> MaterialDialog<'a> {
    pub fn new(
        id: impl Into<Id>,
        title: impl Into<String>,
        open: &'a mut bool,
    ) -> Self {
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

    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn content<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.content = Box::new(content);
        self
    }

    pub fn quick(mut self, quick: bool) -> Self {
        self.quick = quick;
        self
    }

    pub fn no_focus_trap(mut self, no_focus_trap: bool) -> Self {
        self.no_focus_trap = no_focus_trap;
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn text_action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            action_type: ActionType::Text,
            enabled: true,
            action: Box::new(action),
        });
        self
    }

    pub fn filled_tonal_action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            action_type: ActionType::FilledTonal,
            enabled: true,
            action: Box::new(action),
        });
        self
    }

    pub fn filled_action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            action_type: ActionType::Filled,
            enabled: true,
            action: Box::new(action),
        });
        self
    }

    // Backward compatibility methods
    pub fn action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.text_action(text, action)
    }

    pub fn primary_action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.filled_action(text, action)
    }

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
                        ui.label(egui::RichText::new(icon).size(24.0).color(on_surface_variant));
                        ui.add_space(16.0);
                    });
                }
                
                // Headline
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add_space(24.0);
                    ui.label(egui::RichText::new(&title)
                        .size(24.0)
                        .color(on_surface)
                        .family(egui::FontFamily::Proportional));
                    ui.add_space(24.0);
                });
                
                ui.add_space(16.0);
                
                // Content area with proper padding
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                        (self.content)(ui);
                    });
                    ui.add_space(24.0);
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
            unsafe { *open_ref = false; }
        }
    }
    
    fn draw_action_button_static(ui: &mut Ui, action: &DialogAction) -> Response {
        let primary = get_global_color("primary");
        let on_primary = get_global_color("onPrimary");
        let secondary_container = get_global_color("secondaryContainer");
        let on_secondary_container = get_global_color("onSecondaryContainer");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        
        let text_width = ui.fonts(|fonts| {
            fonts.layout_no_wrap(
                action.text.clone(),
                egui::FontId::default(),
                Color32::WHITE
            ).rect.width()
        });
        
        let button_width = (text_width + 24.0).max(64.0);
        let button_height = 40.0;
        let desired_size = Vec2::new(button_width, button_height);
        
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        
        let (bg_color, text_color, border_color) = match action.action_type {
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
                    (secondary_container, on_secondary_container, Color32::TRANSPARENT)
                }
            }
            ActionType::Filled => {
                if response.hovered() {
                    (
                        primary,
                        on_primary,
                        Color32::TRANSPARENT,
                    )
                } else {
                    (primary, on_primary, Color32::TRANSPARENT)
                }
            }
        };
        
        // Draw button background
        ui.painter().rect_filled(
            rect,
            20.0, // Full rounded corners
            bg_color,
        );
        
        // Draw state layer for pressed state
        if response.is_pointer_button_down_on() {
            let pressed_overlay = Color32::from_rgba_premultiplied(text_color.r(), text_color.g(), text_color.b(), 31); // 12% opacity
            ui.painter().rect_filled(
                rect,
                20.0,
                pressed_overlay,
            );
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

    fn draw_action_button(&self, ui: &mut Ui, action: &DialogAction) -> Response {
        Self::draw_action_button_static(ui, action)
    }
}

// Convenience constructors
pub fn dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog {
    MaterialDialog::new(id, title, open)
}

pub fn alert_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Alert)
}

pub fn confirm_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Confirm)
}

pub fn form_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Form)
}