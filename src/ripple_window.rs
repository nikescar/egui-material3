use eframe::egui::{self, Window, Color32, Sense, Vec2};
use crate::{MaterialRipple, add_ripple_to_response_with_bounds, MaterialButton};

pub struct RippleWindow {
    pub open: bool,
    pressed_color: String,
    pressed_opacity: f32,
    hover_color: String,
    hover_opacity: f32,
    // Ripple states
    bounded_ripple: MaterialRipple,
    unbounded_ripple: MaterialRipple,
    button_ripple: MaterialRipple,
    custom_ripple: MaterialRipple,
}

impl Default for RippleWindow {
    fn default() -> Self {
        Self {
            open: false,
            pressed_color: "#6750A4".to_string(),
            pressed_opacity: 0.12,
            hover_color: "#6750A4".to_string(),
            hover_opacity: 0.08,
            bounded_ripple: MaterialRipple::default(),
            unbounded_ripple: MaterialRipple::default(),
            button_ripple: MaterialRipple::default(),
            custom_ripple: MaterialRipple::default(),
        }
    }
}

impl RippleWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Ripple Stories")
            .open(&mut open)
            .default_size([700.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_ripple_demos(ui);
                    ui.add_space(20.0);
                    self.render_component_ripples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Ripple Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/ripple/stories/");
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Pressed Color:");
            ui.text_edit_singleline(&mut self.pressed_color);
        });
        
        ui.horizontal(|ui| {
            ui.label("Pressed Opacity:");
            ui.add(egui::Slider::new(&mut self.pressed_opacity, 0.0..=1.0).step_by(0.01));
        });
        
        ui.horizontal(|ui| {
            ui.label("Hover Color:");
            ui.text_edit_singleline(&mut self.hover_color);
        });
        
        ui.horizontal(|ui| {
            ui.label("Hover Opacity:");
            ui.add(egui::Slider::new(&mut self.hover_opacity, 0.0..=1.0).step_by(0.01));
        });
    }

    fn render_ripple_demos(&mut self, ui: &mut egui::Ui) {
        ui.heading("Ripple Demonstrations");
        
        ui.horizontal(|ui| {
            // Bounded ripple container
            ui.vertical(|ui| {
                ui.label("Bounded Ripple");
                
                let desired_size = Vec2::new(128.0, 64.0);
                let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
                
                // Draw container with border
                ui.painter().rect_stroke(
                    rect,
                    12.0, // border radius
                    egui::Stroke::new(1.0, Color32::from_gray(120)),
                    egui::epaint::StrokeKind::Outside,
                );
                
                // Add bounded ripple effect
                if let Ok(color) = self.parse_color(&self.pressed_color) {
                    let mut ripple_color = color;
                    ripple_color = ripple_color.linear_multiply(self.pressed_opacity);
                    add_ripple_to_response_with_bounds(&response, ui, &mut self.bounded_ripple, Some(ripple_color), true);
                } else {
                    add_ripple_to_response_with_bounds(&response, ui, &mut self.bounded_ripple, None, true);
                }
                
                if response.clicked() {
                    println!("Bounded ripple clicked!");
                }
            });
            
            ui.add_space(40.0);
            
            // Unbounded ripple container
            ui.vertical(|ui| {
                ui.label("Unbounded Ripple");
                
                let desired_size = Vec2::new(64.0, 64.0);
                let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
                
                // Draw circular container
                let center = rect.center();
                let radius = rect.width() / 2.0;
                
                // Draw outer dashed circle
                ui.painter().circle_stroke(
                    center,
                    radius,
                    egui::Stroke::new(1.0, Color32::from_gray(120))
                );
                
                // Draw inner anchor circle
                let anchor_radius = 12.0;
                ui.painter().circle_filled(
                    center,
                    anchor_radius,
                    Color32::from_rgb(103, 80, 164) // Material primary container
                );
                
                ui.painter().circle_stroke(
                    center,
                    anchor_radius,
                    egui::Stroke::new(1.0, Color32::from_gray(120))
                );
                
                // Add unbounded ripple effect
                if let Ok(color) = self.parse_color(&self.pressed_color) {
                    let mut ripple_color = color;
                    ripple_color = ripple_color.linear_multiply(self.pressed_opacity);
                    add_ripple_to_response_with_bounds(&response, ui, &mut self.unbounded_ripple, Some(ripple_color), false);
                } else {
                    add_ripple_to_response_with_bounds(&response, ui, &mut self.unbounded_ripple, None, false);
                }
                
                if response.clicked() {
                    println!("Unbounded ripple clicked!");
                }
            });
        });
        
        // Update and render all ripples with proper clipping
        // For bounded ripples, we need to clip to a rounded rectangle matching the container
        let bounded_rect = egui::Rect::from_min_size(
            ui.min_rect().min + egui::vec2(0.0, 0.0),
            Vec2::new(128.0, 64.0)
        );
        self.bounded_ripple.update_and_render_clipped(ui, Some(bounded_rect));
        
        // For unbounded ripples, no clipping needed - they can extend beyond the anchor
        self.unbounded_ripple.update_and_render(ui);
    }

    fn render_component_ripples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Component Ripples");
        
        ui.horizontal_wrapped(|ui| {
            // Button with custom ripple
            let button = ui.add(MaterialButton::filled("Custom Ripple"));
            if let Ok(color) = self.parse_color(&self.pressed_color) {
                let mut ripple_color = color;
                ripple_color = ripple_color.linear_multiply(self.pressed_opacity);
                add_ripple_to_response_with_bounds(&button, ui, &mut self.button_ripple, Some(ripple_color), true);
            } else {
                add_ripple_to_response_with_bounds(&button, ui, &mut self.button_ripple, None, true);
            }
            if button.clicked() {
                println!("Custom ripple button clicked!");
            }
            
            // Card-like element with ripple
            let card_button = ui.add(MaterialButton::outlined("Card with Ripple"));
            let card_ripple_color = Color32::from_rgb(25, 118, 210).linear_multiply(self.pressed_opacity);
            add_ripple_to_response_with_bounds(&card_button, ui, &mut self.custom_ripple, Some(card_ripple_color), true);
            if card_button.clicked() {
                println!("Card ripple clicked!");
            }
        });
        
        ui.add_space(10.0);
        
        // Custom ripple container
        ui.label("Custom Styled Ripple:");
        let custom_size = Vec2::new(200.0, 80.0);
        let (custom_rect, custom_response) = ui.allocate_exact_size(custom_size, Sense::click());
        
        // Draw custom background
        ui.painter().rect_filled(
            custom_rect,
            8.0,
            Color32::from_rgb(232, 222, 248) // Material surface variant
        );
        
        ui.painter().rect_stroke(
            custom_rect,
            8.0,
            egui::Stroke::new(1.0, Color32::from_rgb(121, 116, 126)), // Material outline
            egui::epaint::StrokeKind::Outside,
        );
        
        // Add text
        ui.painter().text(
            custom_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Click me for custom ripple",
            egui::FontId::default(),
            Color32::from_rgb(28, 27, 31) // Material on-surface
        );
        
        // Add custom ripple (bounded to the custom container)
        if let Ok(color) = self.parse_color(&self.hover_color) {
            let mut ripple_color = color;
            ripple_color = ripple_color.linear_multiply(self.hover_opacity);
            add_ripple_to_response_with_bounds(&custom_response, ui, &mut self.custom_ripple, Some(ripple_color), true);
        }
        
        if custom_response.clicked() {
            println!("Custom styled ripple clicked!");
        }
        
        // Update and render component ripples with clipping to their respective containers
        self.button_ripple.update_and_render(ui);
        
        // Custom ripple should be clipped to the custom container
        self.custom_ripple.update_and_render_clipped(ui, Some(custom_rect));
    }

    fn parse_color(&self, color_str: &str) -> Result<Color32, ()> {
        if color_str.starts_with('#') && color_str.len() == 7 {
            if let Ok(rgb) = u32::from_str_radix(&color_str[1..], 16) {
                let r = ((rgb >> 16) & 0xFF) as u8;
                let g = ((rgb >> 8) & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;
                return Ok(Color32::from_rgb(r, g, b));
            }
        }
        Err(())
    }
}