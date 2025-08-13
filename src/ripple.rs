use eframe::egui::{Color32, Pos2, Response, Ui};

#[derive(Default)]
pub struct RippleEffect {
    pub center: Pos2,
    pub radius: f32,
    pub color: Color32,
    pub max_radius: f32,
    pub start_time: f64,
    pub duration: f64,
    pub active: bool,
}

impl RippleEffect {
    pub fn new(center: Pos2, color: Color32, max_radius: f32) -> Self {
        Self {
            center,
            radius: 0.0,
            color,
            max_radius,
            start_time: 0.0,
            duration: 0.6, // 600ms ripple duration
            active: false,
        }
    }

    pub fn start(&mut self, ui: &Ui) {
        self.active = true;
        self.start_time = ui.input(|i| i.time);
        self.radius = 0.0;
    }

    pub fn update(&mut self, ui: &Ui) -> bool {
        if !self.active {
            return false;
        }

        let current_time = ui.input(|i| i.time);
        let elapsed = current_time - self.start_time;
        
        if elapsed >= self.duration {
            self.active = false;
            return false;
        }

        // Easing function for natural animation
        let progress = (elapsed / self.duration) as f32;
        let eased_progress = 1.0 - (1.0 - progress).powi(3); // Cubic ease-out
        
        self.radius = self.max_radius * eased_progress;
        
        // Fade out towards the end
        let alpha = if progress > 0.7 {
            ((1.0 - progress) / 0.3).clamp(0.0, 1.0)
        } else {
            1.0
        };
        
        let base_alpha = (self.color.a() as f32 / 255.0) * alpha;
        self.color = Color32::from_rgba_premultiplied(
            self.color.r(),
            self.color.g(),
            self.color.b(),
            (base_alpha * 255.0) as u8,
        );

        true
    }

    pub fn render(&self, ui: &Ui) {
        if self.active && self.radius > 0.0 {
            ui.painter().circle_filled(
                self.center,
                self.radius,
                self.color,
            );
        }
    }
}

pub struct MaterialRipple {
    effects: Vec<RippleEffect>,
}

impl Default for MaterialRipple {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialRipple {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn add_ripple(&mut self, center: Pos2, color: Color32, max_radius: f32, ui: &Ui) {
        // Remove finished effects to prevent accumulation
        self.effects.retain(|effect| effect.active);
        
        // Limit the number of simultaneous ripples
        if self.effects.len() >= 3 {
            self.effects.remove(0);
        }

        let mut effect = RippleEffect::new(center, color, max_radius);
        effect.start(ui);
        self.effects.push(effect);
    }

    pub fn update_and_render(&mut self, ui: &Ui) -> bool {
        let mut any_active = false;
        
        for effect in &mut self.effects {
            if effect.update(ui) {
                effect.render(ui);
                any_active = true;
            }
        }

        // Clean up finished effects
        self.effects.retain(|effect| effect.active);

        if any_active {
            ui.ctx().request_repaint();
        }

        any_active
    }

    pub fn clear(&mut self) {
        self.effects.clear();
    }
}

pub fn add_ripple_to_response(
    response: &Response,
    ui: &Ui,
    ripple: &mut MaterialRipple,
    color: Option<Color32>,
) {
    if response.clicked() {
        let rect = response.rect;
        let center = response.interact_pointer_pos().unwrap_or(rect.center());
        let max_radius = (rect.width().max(rect.height()) / 2.0) + 20.0;
        
        let ripple_color = color.unwrap_or_else(|| {
            Color32::from_rgba_premultiplied(128, 128, 128, 40)
        });
        
        ripple.add_ripple(center, ripple_color, max_radius, ui);
    }
}