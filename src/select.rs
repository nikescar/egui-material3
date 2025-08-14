use crate::theme::get_global_color;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct MaterialSelect<'a> {
    selected: &'a mut Option<usize>,
    options: Vec<SelectOption>,
    placeholder: String,
    open: bool,
    enabled: bool,
    width: Option<f32>,
    error_text: Option<String>,
    helper_text: Option<String>,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    keep_open_on_select: bool,
}

pub struct SelectOption {
    value: usize,
    text: String,
}

impl<'a> MaterialSelect<'a> {
    pub fn new(selected: &'a mut Option<usize>) -> Self {
        Self {
            selected,
            options: Vec::new(),
            placeholder: "Select an option".to_string(),
            open: false,
            enabled: true,
            width: None,
            error_text: None,
            helper_text: None,
            leading_icon: None,
            trailing_icon: None,
            keep_open_on_select: false,
        }
    }

    pub fn option(mut self, value: usize, text: impl Into<String>) -> Self {
        self.options.push(SelectOption {
            value,
            text: text.into(),
        });
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub fn keep_open_on_select(mut self, keep_open: bool) -> Self {
        self.keep_open_on_select = keep_open;
        self
    }
}

impl<'a> Widget for MaterialSelect<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let width = self.width.unwrap_or(200.0);
        let height = 56.0;
        let desired_size = Vec2::new(width, height);

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        // Use persistent state for dropdown open/close
        let select_id = egui::Id::new(format!("select_{}", rect.min.x as i32));
        let mut open = ui.memory(|mem| mem.data.get_temp::<bool>(select_id).unwrap_or(false));

        if response.clicked() && self.enabled {
            open = !open;
            ui.memory_mut(|mem| mem.data.insert_temp(select_id, open));
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface = get_global_color("surface");
        let surface_variant = get_global_color("surfaceVariant");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let (bg_color, border_color, text_color) = if !self.enabled {
            (
                get_global_color("surfaceVariant").linear_multiply(0.38),
                get_global_color("outline").linear_multiply(0.38),
                get_global_color("onSurface").linear_multiply(0.38),
            )
        } else if response.hovered() || open {
            (surface, primary_color, on_surface)
        } else {
            (surface, outline, on_surface_variant)
        };

        // Draw select field background
        ui.painter().rect_filled(
            rect,
            4.0,
            bg_color,
        );

        // Draw border
        ui.painter().rect_stroke(
            rect,
            4.0,
            Stroke::new(1.0, border_color),
            egui::epaint::StrokeKind::Outside,
        );

        // Draw selected text or placeholder
        let display_text = if let Some(selected_value) = *self.selected {
            self.options.iter()
                .find(|option| option.value == selected_value)
                .map(|option| option.text.as_str())
                .unwrap_or(&self.placeholder)
        } else {
            &self.placeholder
        };

        let text_pos = Pos2::new(rect.min.x + 16.0, rect.center().y);
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            display_text,
            egui::FontId::default(),
            text_color,
        );

        // Draw dropdown arrow
        let arrow_center = Pos2::new(rect.max.x - 24.0, rect.center().y);
        let arrow_size = 8.0;
        
        if open {
            // Up arrow
            ui.painter().line_segment([
                Pos2::new(arrow_center.x - arrow_size / 2.0, arrow_center.y + arrow_size / 4.0),
                Pos2::new(arrow_center.x, arrow_center.y - arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
            ui.painter().line_segment([
                Pos2::new(arrow_center.x, arrow_center.y - arrow_size / 4.0),
                Pos2::new(arrow_center.x + arrow_size / 2.0, arrow_center.y + arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
        } else {
            // Down arrow
            ui.painter().line_segment([
                Pos2::new(arrow_center.x - arrow_size / 2.0, arrow_center.y - arrow_size / 4.0),
                Pos2::new(arrow_center.x, arrow_center.y + arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
            ui.painter().line_segment([
                Pos2::new(arrow_center.x, arrow_center.y + arrow_size / 4.0),
                Pos2::new(arrow_center.x + arrow_size / 2.0, arrow_center.y - arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
        }

        // Show dropdown if open
        if open {
            let dropdown_height = self.options.len() as f32 * 48.0 + 16.0;
            let dropdown_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.max.y + 4.0),
                Vec2::new(width, dropdown_height),
            );

            // Draw dropdown background
            ui.painter().rect_filled(
                dropdown_rect,
                8.0,
                surface,
            );

            // Draw dropdown border
            ui.painter().rect_stroke(
                dropdown_rect,
                8.0,
                Stroke::new(1.0, outline),
                egui::epaint::StrokeKind::Outside,
            );

            // Draw options
            let mut current_y = dropdown_rect.min.y + 8.0;
            for option in &self.options {
                let option_rect = Rect::from_min_size(
                    Pos2::new(dropdown_rect.min.x + 8.0, current_y),
                    Vec2::new(width - 16.0, 48.0),
                );

                let option_response = ui.interact(
                    option_rect,
                    egui::Id::new(("select_option", option.value)),
                    Sense::click(),
                );

                if option_response.hovered() {
                    ui.painter().rect_filled(
                        option_rect,
                        4.0,
                        Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20),
                    );
                }

                if option_response.clicked() {
                    *self.selected = Some(option.value);
                    if !self.keep_open_on_select {
                        open = false;
                        ui.memory_mut(|mem| mem.data.insert_temp(select_id, open));
                    }
                    response.mark_changed();
                }

                let text_pos = Pos2::new(option_rect.min.x + 12.0, option_rect.center().y);
                ui.painter().text(
                    text_pos,
                    egui::Align2::LEFT_CENTER,
                    &option.text,
                    egui::FontId::default(),
                    on_surface,
                );

                current_y += 48.0;
            }
        }

        response
    }
}

pub fn select<'a>(selected: &'a mut Option<usize>) -> MaterialSelect<'a> {
    MaterialSelect::new(selected)
}