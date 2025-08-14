use eframe::egui::{self, Color32, Stroke, Ui, Context, Modal, Id};

pub struct MaterialDialog<'a> {
    id: Id,
    title: String,
    open: &'a mut bool,
    icon: Option<String>,
    content: Box<dyn FnOnce(&mut Ui) + 'a>,
    actions: Vec<DialogAction<'a>>,
}

pub struct DialogAction<'a> {
    text: String,
    primary: bool,
    enabled: bool,
    action: Box<dyn FnOnce() + 'a>,
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
            icon: None,
            content: Box::new(|_| {}),
            actions: Vec::new(),
        }
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

    pub fn action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            primary: false,
            enabled: true,
            action: Box::new(action),
        });
        self
    }

    pub fn primary_action<F>(mut self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            primary: true,
            enabled: true,
            action: Box::new(action),
        });
        self
    }

    pub fn show(self, ctx: &Context) {
        if !*self.open {
            return;
        }

        let mut should_close = false;
        let mut pending_actions = Vec::new();

        let modal = Modal::new(self.id).show(ctx, |ui| {
            // Set the modal width
            ui.set_width(400.0);
            
            // Set Material Design styling
            ui.style_mut().visuals.widgets.noninteractive.bg_fill = 
                Color32::from_gray(if ui.visuals().dark_mode { 28 } else { 251 });
            ui.style_mut().visuals.widgets.noninteractive.fg_stroke = 
                Stroke::new(1.0, Color32::from_gray(if ui.visuals().dark_mode { 202 } else { 73 }));

            // Title
            ui.heading(&self.title);
            ui.add_space(8.0);

            // Show icon if present
            if let Some(ref _icon) = self.icon {
                // Placeholder for icon rendering
                ui.label("📋"); // placeholder icon
                ui.add_space(4.0);
            }

            // Content area
            (self.content)(ui);
            ui.add_space(16.0);

            // Actions area
            if !self.actions.is_empty() {
                ui.separator();
                ui.add_space(8.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    for (index, action) in self.actions.into_iter().enumerate().rev() {
                        // Create transparent buttons with purple hover
                        let button_text = &action.text;
                        let desired_size = egui::Vec2::new(button_text.len() as f32 * 8.0 + 16.0, 32.0);
                        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
                        
                        // Draw transparent background with purple hover
                        if response.hovered() {
                            ui.painter().rect_filled(
                                rect,
                                4.0,
                                Color32::from_rgba_unmultiplied(147, 51, 234, 50), // Purple with transparency
                            );
                        }
                        
                        // Draw button text
                        let text_color = if response.hovered() {
                            Color32::from_rgb(147, 51, 234) // Purple color on hover
                        } else {
                            if action.primary {
                                Color32::from_rgb(147, 51, 234) // Purple for primary
                            } else {
                                Color32::from_gray(100) // Gray for secondary
                            }
                        };
                        
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            button_text,
                            egui::FontId::proportional(14.0),
                            text_color,
                        );

                        if response.clicked() {
                            pending_actions.push((index, action.action));
                        }
                    }
                });

                ui.add_space(8.0);
            }
        });

        // Execute pending actions
        for (_index, action) in pending_actions {
            action();
            should_close = true;
        }

        // Handle modal close events (escape key, click outside, etc.)
        if modal.should_close() || should_close {
            *self.open = false;
        }
    }
}

pub fn dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog {
    MaterialDialog::new(id, title, open)
}