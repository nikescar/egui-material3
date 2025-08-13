use eframe::egui::{self, Ui, Window, vec2};
use crate::{MaterialFocusRing, add_focus_ring_to_response, MaterialButton};

pub struct FocusWindow {
    pub open: bool,
    inward: bool,
    always_visible: bool,
}

impl Default for FocusWindow {
    fn default() -> Self {
        Self {
            open: false,
            inward: false,
            always_visible: false,
        }
    }
}

impl FocusWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Focus Stories")
            .open(&mut open)
            .default_size([700.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_focus_ring_examples(ui);
                    ui.add_space(30.0);
                    self.render_multi_action_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Focus Ring Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://github.com/material-components/material-web/blob/main/docs/components/focus-ring.md");
            }
        });

        ui.checkbox(&mut self.inward, "inward");
        ui.checkbox(&mut self.always_visible, "Always visible (for demonstration)");
    }

    fn render_focus_ring_examples(&mut self, ui: &mut Ui) {
        ui.heading("Focus Ring Examples");
        ui.label("Click buttons to see focus rings appear. Use Tab key to navigate with keyboard.");
        
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // First button with focus ring
            let button1 = ui.add(MaterialButton::filled("Button 1")
                .min_size(vec2(64.0, 64.0)));
            
            if self.always_visible || button1.has_focus() {
                ui.add(MaterialFocusRing::new()
                    .visible(true)
                    .inward(self.inward)
                    .target_rect(button1.rect)
                    .corner_radius(16.0));
            }
            
            ui.add_space(8.0);

            // Second button with focus ring
            let button2 = ui.add(MaterialButton::outlined("Button 2")
                .min_size(vec2(64.0, 64.0)));
            
            if self.always_visible || button2.has_focus() {
                ui.add(MaterialFocusRing::new()
                    .visible(true)
                    .inward(self.inward)
                    .target_rect(button2.rect)
                    .corner_radius(16.0));
            }
            
            ui.add_space(8.0);

            // Third button with focus ring
            let button3 = ui.add(MaterialButton::elevated("Button 3")
                .min_size(vec2(64.0, 64.0)));
            
            if self.always_visible || button3.has_focus() {
                ui.add(MaterialFocusRing::new()
                    .visible(true)
                    .inward(self.inward)
                    .target_rect(button3.rect)
                    .corner_radius(16.0));
            }
        });

        ui.add_space(20.0);

        // Example with manual focus ring positioning
        ui.heading("Manual Focus Ring Positioning");
        
        ui.horizontal(|ui| {
            // Create a custom interactive area
            let size = vec2(80.0, 80.0);
            let response = ui.allocate_response(size, egui::Sense::click());
            let rect = response.rect;
            
            // Draw a custom surface
            ui.painter().rect_filled(
                rect,
                16.0,
                egui::Color32::from_rgb(232, 222, 248), // md-sys-color-primary-container
            );
            ui.painter().rect_stroke(
                rect,
                16.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(121, 116, 126)),
                egui::epaint::StrokeKind::Outside,
            );
            
            // Add text
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Custom");
                });
            });
            
            // Add focus ring
            if self.always_visible || response.has_focus() {
                ui.add(MaterialFocusRing::new()
                    .visible(true)
                    .inward(self.inward)
                    .target_rect(rect)
                    .corner_radius(16.0));
            }
            
            if response.clicked() {
                println!("Custom area clicked!");
            }
        });
    }

    fn render_multi_action_examples(&mut self, ui: &mut Ui) {
        ui.heading("Multi-Action Components");
        ui.label("Components with multiple interactive areas can have individual focus rings.");
        
        ui.add_space(10.0);

        // Create a multi-action component similar to Material Web example
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(248, 248, 248)) // md-sys-color-surface
            .corner_radius(16.0)
            .inner_margin(egui::Margin::symmetric(16, 8))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(121, 116, 126)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Primary action area
                    let primary_response = ui.add(MaterialButton::text("Primary Action")
                        .frame(false));
                    
                    // Add focus ring for primary action
                    if self.always_visible || primary_response.has_focus() {
                        ui.add(MaterialFocusRing::new()
                            .visible(true)
                            .inward(self.inward)
                            .target_rect(primary_response.rect)
                            .corner_radius(8.0));
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Secondary action (circular button)
                        let secondary_response = ui.add(MaterialButton::text("×")
                            .min_size(vec2(32.0, 32.0))
                            .corner_radius(32.0));
                        
                        // Add focus ring for secondary action
                        if self.always_visible || secondary_response.has_focus() {
                            ui.add(MaterialFocusRing::new()
                                .visible(true)
                                .inward(self.inward)
                                .target_rect(secondary_response.rect)
                                .corner_radius(32.0));
                        }
                        
                        if secondary_response.clicked() {
                            println!("Secondary action clicked!");
                        }
                    });
                    
                    if primary_response.clicked() {
                        println!("Primary action clicked!");
                    }
                });
            });

        ui.add_space(20.0);

        // Additional examples
        ui.heading("Focus Ring Variations");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Outward Focus Ring:");
                let button = ui.button("Focus me");
                if self.always_visible || button.has_focus() {
                    ui.add(MaterialFocusRing::new()
                        .visible(true)
                        .inward(false)
                        .target_rect(button.rect));
                }
            });
            
            ui.add_space(20.0);
            
            ui.vertical(|ui| {
                ui.label("Inward Focus Ring:");
                let button = ui.button("Focus me");
                if self.always_visible || button.has_focus() {
                    ui.add(MaterialFocusRing::new()
                        .visible(true)
                        .inward(true)
                        .target_rect(button.rect));
                }
            });
        });

        ui.add_space(10.0);

        // Example with helper function
        ui.label("Using helper function:");
        let button_with_helper = ui.button("Button with helper");
        add_focus_ring_to_response(&button_with_helper, ui, self.inward);
        
        if button_with_helper.clicked() {
            println!("Button with helper clicked!");
        }
    }
}