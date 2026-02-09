use eframe::egui::{self, Color32, Response, Sense, Ui, Vec2, Widget};

pub struct MaterialIcon {
    name: String,
    size: f32,
    color: Option<Color32>,
    filled: bool,
}

impl MaterialIcon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: 24.0,
            color: None,
            filled: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }
}

impl Widget for MaterialIcon {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        let icon_color = self
            .color
            .unwrap_or_else(|| Color32::from_gray(if ui.visuals().dark_mode { 230 } else { 30 }));

        // Render icon character from MaterialSymbolsOutlined font
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.name,
            egui::FontId::proportional(self.size),
            icon_color,
        );

        response
    }
}

pub fn icon(name: impl Into<String>) -> MaterialIcon {
    MaterialIcon::new(name)
}
