use eframe::egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use std::f32::consts::PI;

#[derive(Clone, Copy, PartialEq)]
pub enum ProgressVariant {
    Linear,
    Circular,
}

pub struct MaterialProgress {
    variant: ProgressVariant,
    value: f32,
    max: f32,
    buffer: Option<f32>,
    indeterminate: bool,
    four_color_enabled: bool,
    size: Vec2,
}

impl MaterialProgress {
    pub fn new(variant: ProgressVariant) -> Self {
        Self {
            variant,
            value: 0.0,
            max: 1.0,
            buffer: None,
            indeterminate: false,
            four_color_enabled: false,
            size: match variant {
                ProgressVariant::Linear => Vec2::new(200.0, 4.0),
                ProgressVariant::Circular => Vec2::splat(48.0),
            },
        }
    }

    pub fn linear() -> Self {
        Self::new(ProgressVariant::Linear)
    }

    pub fn circular() -> Self {
        Self::new(ProgressVariant::Circular)
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, self.max);
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max.max(0.001); // Prevent division by zero
        self.value = self.value.clamp(0.0, self.max);
        self
    }

    pub fn buffer(mut self, buffer: f32) -> Self {
        self.buffer = Some(buffer.clamp(0.0, self.max));
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn four_color_enabled(mut self, enabled: bool) -> Self {
        self.four_color_enabled = enabled;
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.size.x = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }

    pub fn four_color(mut self, enabled: bool) -> Self {
        self.four_color_enabled = enabled;
        self
    }
}

impl Widget for MaterialProgress {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());

        match self.variant {
            ProgressVariant::Linear => self.render_linear(ui, rect),
            ProgressVariant::Circular => self.render_circular(ui, rect),
        }

        response
    }
}

impl MaterialProgress {
    fn render_linear(&self, ui: &mut Ui, rect: Rect) {
        // Material Design colors
        let primary_color = Color32::from_rgb(103, 80, 164);
        let surface_variant = Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 231 });
        let primary_container = Color32::from_rgb(234, 221, 255);

        // Draw track background
        ui.painter().rect_filled(
            rect,
            rect.height() / 2.0,
            surface_variant,
        );

        if self.indeterminate {
            // Indeterminate animation - simplified for egui
            let time = ui.input(|i| i.time) as f32;
            let progress = ((time * 2.0).sin() + 1.0) / 2.0; // Oscillate between 0 and 1
            
            let bar_width = rect.width() * 0.3; // 30% of total width
            let start_x = rect.min.x + (rect.width() - bar_width) * progress;
            
            let bar_rect = Rect::from_min_size(
                Pos2::new(start_x, rect.min.y),
                Vec2::new(bar_width, rect.height()),
            );
            
            ui.painter().rect_filled(
                bar_rect,
                rect.height() / 2.0,
                primary_color,
            );
            
            // Request repaint for animation
            ui.ctx().request_repaint();
        } else {
            // Draw buffer if present
            if let Some(buffer) = self.buffer {
                let buffer_progress = (buffer / self.max).clamp(0.0, 1.0);
                let buffer_width = rect.width() * buffer_progress;
                
                if buffer_width > 0.0 {
                    let buffer_rect = Rect::from_min_size(
                        rect.min,
                        Vec2::new(buffer_width, rect.height()),
                    );
                    
                    ui.painter().rect_filled(
                        buffer_rect,
                        rect.height() / 2.0,
                        primary_container,
                    );
                }
            }

            // Draw progress bar
            let progress = (self.value / self.max).clamp(0.0, 1.0);
            let progress_width = rect.width() * progress;
            
            if progress_width > 0.0 {
                let progress_rect = Rect::from_min_size(
                    rect.min,
                    Vec2::new(progress_width, rect.height()),
                );
                
                ui.painter().rect_filled(
                    progress_rect,
                    rect.height() / 2.0,
                    primary_color,
                );
            }
        }
    }

    fn render_circular(&self, ui: &mut Ui, rect: Rect) {
        let center = rect.center();
        let radius = (rect.width().min(rect.height()) / 2.0) - 4.0;
        let stroke_width = 4.0;

        // Material Design colors
        let primary_color = Color32::from_rgb(103, 80, 164);
        let surface_variant = Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 231 });

        // Draw track circle
        ui.painter().circle_stroke(
            center,
            radius,
            Stroke::new(stroke_width, surface_variant),
        );

        if self.indeterminate {
            // Indeterminate animation - spinning arc
            let time = ui.input(|i| i.time) as f32;
            let rotation = time * 2.0; // Rotation speed
            let arc_length = PI; // Half circle arc

            self.draw_arc(
                ui,
                center,
                radius,
                rotation,
                rotation + arc_length,
                stroke_width,
                primary_color,
            );

            // Request repaint for animation
            ui.ctx().request_repaint();
        } else {
            // Draw progress arc
            let progress = (self.value / self.max).clamp(0.0, 1.0);
            let arc_length = 2.0 * PI * progress;
            
            if progress > 0.0 {
                self.draw_arc(
                    ui,
                    center,
                    radius,
                    -PI / 2.0, // Start at top
                    -PI / 2.0 + arc_length,
                    stroke_width,
                    primary_color,
                );
            }
        }
    }

    fn draw_arc(
        &self,
        ui: &mut Ui,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        stroke_width: f32,
        color: Color32,
    ) {
        let segments = 32;
        let angle_step = (end_angle - start_angle) / segments as f32;
        
        for i in 0..segments {
            let angle1 = start_angle + i as f32 * angle_step;
            let angle2 = start_angle + (i + 1) as f32 * angle_step;
            
            let point1 = Pos2::new(
                center.x + radius * angle1.cos(),
                center.y + radius * angle1.sin(),
            );
            let point2 = Pos2::new(
                center.x + radius * angle2.cos(),
                center.y + radius * angle2.sin(),
            );
            
            ui.painter().line_segment(
                [point1, point2],
                Stroke::new(stroke_width, color),
            );
        }
    }
}

pub fn linear_progress() -> MaterialProgress {
    MaterialProgress::linear()
}

pub fn circular_progress() -> MaterialProgress {
    MaterialProgress::circular()
}