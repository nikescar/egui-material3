use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

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

        let icon_color = self.color.unwrap_or_else(|| {
            Color32::from_gray(if ui.visuals().dark_mode { 230 } else { 30 })
        });

        // Draw a simple icon representation (can be replaced with actual icon rendering)
        match self.name.as_str() {
            "home" => draw_home_icon(ui, rect, icon_color, self.filled),
            "search" => draw_search_icon(ui, rect, icon_color, self.filled),
            "settings" => draw_settings_icon(ui, rect, icon_color, self.filled),
            "menu" => draw_menu_icon(ui, rect, icon_color, self.filled),
            "close" => draw_close_icon(ui, rect, icon_color, self.filled),
            "check" => draw_check_icon(ui, rect, icon_color, self.filled),
            "add" => draw_add_icon(ui, rect, icon_color, self.filled),
            "remove" => draw_remove_icon(ui, rect, icon_color, self.filled),
            "arrow_back" => draw_arrow_back_icon(ui, rect, icon_color, self.filled),
            "arrow_forward" => draw_arrow_forward_icon(ui, rect, icon_color, self.filled),
            _ => draw_default_icon(ui, rect, icon_color, self.filled),
        }

        response
    }
}

fn draw_home_icon(ui: &mut Ui, rect: Rect, color: Color32, filled: bool) {
    let center = rect.center();
    let size = rect.width() * 0.8;
    
    // House shape
    let points = [
        Pos2::new(center.x, center.y - size * 0.3),           // top
        Pos2::new(center.x - size * 0.3, center.y),          // left
        Pos2::new(center.x - size * 0.2, center.y),          // left wall
        Pos2::new(center.x - size * 0.2, center.y + size * 0.3), // left bottom
        Pos2::new(center.x + size * 0.2, center.y + size * 0.3), // right bottom
        Pos2::new(center.x + size * 0.2, center.y),          // right wall
        Pos2::new(center.x + size * 0.3, center.y),          // right
    ];
    
    if filled {
        ui.painter().add(egui::Shape::convex_polygon(points.to_vec(), color, Stroke::NONE));
    } else {
        for i in 0..points.len() {
            let next = (i + 1) % points.len();
            ui.painter().line_segment([points[i], points[next]], Stroke::new(2.0, color));
        }
    }
}

fn draw_search_icon(ui: &mut Ui, rect: Rect, color: Color32, filled: bool) {
    let center = rect.center();
    let radius = rect.width() * 0.25;
    
    // Circle
    if filled {
        ui.painter().circle_filled(center - Vec2::new(radius * 0.3, radius * 0.3), radius, color);
    } else {
        ui.painter().circle_stroke(center - Vec2::new(radius * 0.3, radius * 0.3), radius, Stroke::new(2.0, color));
    }
    
    // Handle
    let handle_start = center + Vec2::new(radius * 0.3, radius * 0.3);
    let handle_end = center + Vec2::new(radius * 0.8, radius * 0.8);
    ui.painter().line_segment([handle_start, handle_end], Stroke::new(3.0, color));
}

fn draw_settings_icon(ui: &mut Ui, rect: Rect, color: Color32, filled: bool) {
    let center = rect.center();
    let radius = rect.width() * 0.3;
    
    // Draw gear shape (simplified)
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::TAU / 8.0;
        let outer_radius = if i % 2 == 0 { radius } else { radius * 0.7 };
        let point = center + Vec2::new(angle.cos() * outer_radius, angle.sin() * outer_radius);
        
        if i == 0 {
            continue;
        }
        
        let prev_angle = (i - 1) as f32 * std::f32::consts::TAU / 8.0;
        let prev_radius = if (i - 1) % 2 == 0 { radius } else { radius * 0.7 };
        let prev_point = center + Vec2::new(prev_angle.cos() * prev_radius, prev_angle.sin() * prev_radius);
        
        ui.painter().line_segment([prev_point, point], Stroke::new(2.0, color));
    }
    
    // Center circle
    if filled {
        ui.painter().circle_filled(center, radius * 0.3, color);
    } else {
        ui.painter().circle_stroke(center, radius * 0.3, Stroke::new(2.0, color));
    }
}

fn draw_menu_icon(ui: &mut Ui, rect: Rect, color: Color32, _filled: bool) {
    let center = rect.center();
    let width = rect.width() * 0.6;
    let line_height = 2.0;
    let spacing = rect.height() * 0.15;
    
    // Three horizontal lines
    for i in 0..3 {
        let y = center.y + (i as f32 - 1.0) * spacing;
        let start = Pos2::new(center.x - width / 2.0, y);
        let end = Pos2::new(center.x + width / 2.0, y);
        ui.painter().line_segment([start, end], Stroke::new(line_height, color));
    }
}

fn draw_close_icon(ui: &mut Ui, rect: Rect, color: Color32, _filled: bool) {
    let center = rect.center();
    let size = rect.width() * 0.4;
    
    // X shape
    ui.painter().line_segment([
        center - Vec2::splat(size / 2.0),
        center + Vec2::splat(size / 2.0),
    ], Stroke::new(2.0, color));
    
    ui.painter().line_segment([
        center + Vec2::new(-size / 2.0, size / 2.0),
        center + Vec2::new(size / 2.0, -size / 2.0),
    ], Stroke::new(2.0, color));
}

fn draw_check_icon(ui: &mut Ui, rect: Rect, color: Color32, _filled: bool) {
    let center = rect.center();
    let size = rect.width() * 0.4;
    
    // Checkmark
    let start = Pos2::new(center.x - size * 0.3, center.y);
    let middle = Pos2::new(center.x - size * 0.1, center.y + size * 0.2);
    let end = Pos2::new(center.x + size * 0.3, center.y - size * 0.2);
    
    ui.painter().line_segment([start, middle], Stroke::new(2.0, color));
    ui.painter().line_segment([middle, end], Stroke::new(2.0, color));
}

fn draw_add_icon(ui: &mut Ui, rect: Rect, color: Color32, _filled: bool) {
    let center = rect.center();
    let size = rect.width() * 0.4;
    
    // Plus shape
    ui.painter().line_segment([
        Pos2::new(center.x - size / 2.0, center.y),
        Pos2::new(center.x + size / 2.0, center.y),
    ], Stroke::new(2.0, color));
    
    ui.painter().line_segment([
        Pos2::new(center.x, center.y - size / 2.0),
        Pos2::new(center.x, center.y + size / 2.0),
    ], Stroke::new(2.0, color));
}

fn draw_remove_icon(ui: &mut Ui, rect: Rect, color: Color32, _filled: bool) {
    let center = rect.center();
    let size = rect.width() * 0.4;
    
    // Minus shape
    ui.painter().line_segment([
        Pos2::new(center.x - size / 2.0, center.y),
        Pos2::new(center.x + size / 2.0, center.y),
    ], Stroke::new(2.0, color));
}

fn draw_arrow_back_icon(ui: &mut Ui, rect: Rect, color: Color32, _filled: bool) {
    let center = rect.center();
    let size = rect.width() * 0.4;
    
    // Arrow pointing left
    let tip = Pos2::new(center.x - size / 2.0, center.y);
    let top = Pos2::new(center.x, center.y - size / 2.0);
    let bottom = Pos2::new(center.x, center.y + size / 2.0);
    let tail_end = Pos2::new(center.x + size / 2.0, center.y);
    
    ui.painter().line_segment([tip, top], Stroke::new(2.0, color));
    ui.painter().line_segment([tip, bottom], Stroke::new(2.0, color));
    ui.painter().line_segment([tip, tail_end], Stroke::new(2.0, color));
}

fn draw_arrow_forward_icon(ui: &mut Ui, rect: Rect, color: Color32, _filled: bool) {
    let center = rect.center();
    let size = rect.width() * 0.4;
    
    // Arrow pointing right
    let tip = Pos2::new(center.x + size / 2.0, center.y);
    let top = Pos2::new(center.x, center.y - size / 2.0);
    let bottom = Pos2::new(center.x, center.y + size / 2.0);
    let tail_end = Pos2::new(center.x - size / 2.0, center.y);
    
    ui.painter().line_segment([tip, top], Stroke::new(2.0, color));
    ui.painter().line_segment([tip, bottom], Stroke::new(2.0, color));
    ui.painter().line_segment([tail_end, tip], Stroke::new(2.0, color));
}

fn draw_default_icon(ui: &mut Ui, rect: Rect, color: Color32, filled: bool) {
    let center = rect.center();
    let radius = rect.width() * 0.3;
    
    if filled {
        ui.painter().circle_filled(center, radius, color);
    } else {
        ui.painter().circle_stroke(center, radius, Stroke::new(2.0, color));
    }
}

pub fn icon(name: impl Into<String>) -> MaterialIcon {
    MaterialIcon::new(name)
}