use eframe::egui::{self, Ui, Window};
use crate::{MaterialCard, MaterialButton};

pub struct CardWindow {
    pub open: bool,
}

impl Default for CardWindow {
    fn default() -> Self {
        Self {
            open: false,
        }
    }
}

impl CardWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Card Stories")
            .open(&mut open)
            .default_size([400.0, 300.0])
            .max_size([400.0, 350.0])
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_cards(ui);
                    ui.add_space(30.0);
                    self.render_cards_with_actions(ui);
                    ui.add_space(30.0);
                    self.render_additional_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Card Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://m3.material.io/components/cards/overview");
            }
        });
    }

    fn render_basic_cards(&mut self, ui: &mut Ui) {
        ui.heading("Cards");
        ui.label("Basic card variants:");
        
        ui.add_space(10.0);
        
        ui.horizontal_wrapped(|ui| {
            // Set fixed card width
            let card_width = 400.0;
            
            // Elevated card
            ui.add(MaterialCard::elevated()
                .min_size(egui::Vec2::new(card_width, 200.0))
                .content(|ui| {
                    ui.vertical(|ui| {
                        // Placeholder image area - adjusted to card width
                        let image_width = (card_width - 32.0).max(80.0);
                        let image_rect = egui::Rect::from_min_size(
                            ui.next_widget_position(),
                            egui::Vec2::new(image_width, 96.0),
                        );
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                            ui.painter().rect_filled(
                                image_rect,
                                8.0,
                                egui::Color32::from_rgb(218, 220, 224),
                            );
                            ui.centered_and_justified(|ui| {
                                ui.label("Image");
                            });
                        });
                        
                        ui.add_space(16.0);
                        ui.label("A static elevated card");
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
            
            ui.add_space(8.0);

            // Filled card
            ui.add(MaterialCard::filled()
                .min_size(egui::Vec2::new(card_width, 200.0))
                .content(|ui| {
                    ui.vertical(|ui| {
                        // Placeholder image area - adjusted to card width
                        let image_width = (card_width - 32.0).max(80.0);
                        let image_rect = egui::Rect::from_min_size(
                            ui.next_widget_position(),
                            egui::Vec2::new(image_width, 96.0),
                        );
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                            ui.painter().rect_filled(
                                image_rect,
                                8.0,
                                egui::Color32::from_rgb(218, 220, 224),
                            );
                            ui.centered_and_justified(|ui| {
                                ui.label("Image");
                            });
                        });
                        
                        ui.add_space(16.0);
                        ui.label("A static filled card");
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
            
            ui.add_space(8.0);

            // Outlined card
            ui.add(MaterialCard::outlined()
                .min_size(egui::Vec2::new(card_width, 200.0))
                .content(|ui| {
                    ui.vertical(|ui| {
                        // Placeholder image area - adjusted to card width
                        let image_width = (card_width - 32.0).max(80.0);
                        let image_rect = egui::Rect::from_min_size(
                            ui.next_widget_position(),
                            egui::Vec2::new(image_width, 96.0),
                        );
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                            ui.painter().rect_filled(
                                image_rect,
                                8.0,
                                egui::Color32::from_rgb(218, 220, 224),
                            );
                            ui.centered_and_justified(|ui| {
                                ui.label("Image");
                            });
                        });
                        
                        ui.add_space(16.0);
                        ui.label("A static outlined card");
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
        });
    }

    fn render_cards_with_actions(&mut self, ui: &mut Ui) {
        ui.heading("Cards with Actions");
        ui.label("Cards can contain interactive elements:");
        
        ui.add_space(10.0);
        
        ui.horizontal_wrapped(|ui| {
            // Set fixed card width
            let card_width = 400.0;
            
            // Elevated card with action
            ui.add(MaterialCard::elevated()
                .min_size(egui::Vec2::new(card_width, 240.0))
                .content(|ui| {
                    ui.vertical(|ui| {
                        // Placeholder image area - adjusted to card width
                        let image_width = (card_width - 32.0).max(80.0);
                        let image_rect = egui::Rect::from_min_size(
                            ui.next_widget_position(),
                            egui::Vec2::new(image_width, 96.0),
                        );
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                            ui.painter().rect_filled(
                                image_rect,
                                8.0,
                                egui::Color32::from_rgb(218, 220, 224),
                            );
                            ui.centered_and_justified(|ui| {
                                ui.label("Image");
                            });
                        });
                        
                        ui.add_space(16.0);
                        ui.label("An elevated card with actions");
                        ui.add_space(16.0);
                        
                        if ui.add(MaterialButton::filled("Card action")).clicked() {
                            println!("Elevated card action clicked!");
                        }
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
            
            ui.add_space(8.0);

            // Filled card with action
            ui.add(MaterialCard::filled()
                .min_size(egui::Vec2::new(card_width, 240.0))
                .content(|ui| {
                    ui.vertical(|ui| {
                        // Placeholder image area - adjusted to card width
                        let image_width = (card_width - 32.0).max(80.0);
                        let image_rect = egui::Rect::from_min_size(
                            ui.next_widget_position(),
                            egui::Vec2::new(image_width, 96.0),
                        );
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                            ui.painter().rect_filled(
                                image_rect,
                                8.0,
                                egui::Color32::from_rgb(218, 220, 224),
                            );
                            ui.centered_and_justified(|ui| {
                                ui.label("Image");
                            });
                        });
                        
                        ui.add_space(16.0);
                        ui.label("A filled card with actions");
                        ui.add_space(16.0);
                        
                        if ui.add(MaterialButton::filled("Card action")).clicked() {
                            println!("Filled card action clicked!");
                        }
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
            
            ui.add_space(8.0);

            // Outlined card with action
            ui.add(MaterialCard::outlined()
                .min_size(egui::Vec2::new(card_width, 240.0))
                .content(|ui| {
                    ui.vertical(|ui| {
                        // Placeholder image area - adjusted to card width
                        let image_width = (card_width - 32.0).max(80.0);
                        let image_rect = egui::Rect::from_min_size(
                            ui.next_widget_position(),
                            egui::Vec2::new(image_width, 96.0),
                        );
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                            ui.painter().rect_filled(
                                image_rect,
                                8.0,
                                egui::Color32::from_rgb(218, 220, 224),
                            );
                            ui.centered_and_justified(|ui| {
                                ui.label("Image");
                            });
                        });
                        
                        ui.add_space(16.0);
                        ui.label("An outlined card with actions");
                        ui.add_space(16.0);
                        
                        if ui.add(MaterialButton::filled("Card action")).clicked() {
                            println!("Outlined card action clicked!");
                        }
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
        });
    }

    fn render_additional_examples(&mut self, ui: &mut Ui) {
        ui.heading("Additional Card Examples");
        let card_width = 400.0;
        
        // Clickable card
        ui.label("Clickable card:");
        let clickable_card = ui.add(MaterialCard::elevated()
            .clickable(true)
            .min_size(egui::Vec2::new(card_width, 100.0))
            .content(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Click anywhere on this card");
                    ui.label("(This entire card is clickable)");
                });
                ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
            }));
        
        if clickable_card.clicked() {
            println!("Clickable card was clicked!");
        }
        
        ui.add_space(20.0);

        // Different sizes (all now 400px width)
        ui.label("Card examples (all 400px width):");
        ui.horizontal_wrapped(|ui| {
            ui.add(MaterialCard::outlined()
                .min_size(egui::Vec2::new(card_width, 120.0))
                .content(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Outlined Card");
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
            
            ui.add_space(8.0);
            
            ui.add(MaterialCard::filled()
                .min_size(egui::Vec2::new(card_width, 120.0))
                .content(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Filled Card");
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
            
            ui.add_space(8.0);
            
            ui.add(MaterialCard::elevated()
                .min_size(egui::Vec2::new(card_width, 120.0))
                .content(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Elevated Card");
                    });
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                }));
        });

        ui.add_space(20.0);

        // Card with complex content
        ui.label("Card with complex content:");
        ui.add(MaterialCard::outlined()
            .min_size(egui::Vec2::new(card_width, 200.0))
            .content(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("🎵");
                        ui.vertical(|ui| {
                            ui.label("Now Playing");
                            ui.label("Song Title - Artist Name");
                        });
                    });
                    
                    ui.add_space(10.0);
                    
                    // Progress bar simulation
                    let progress_rect = egui::Rect::from_min_size(
                        ui.next_widget_position(),
                        egui::Vec2::new(ui.available_width(), 4.0),
                    );
                    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(progress_rect), |ui| {
                        ui.painter().rect_filled(
                            progress_rect,
                            2.0,
                            egui::Color32::from_rgb(196, 199, 197),
                        );
                        let progress_fill = egui::Rect::from_min_size(
                            progress_rect.min,
                            egui::Vec2::new(progress_rect.width() * 0.3, progress_rect.height()),
                        );
                        ui.painter().rect_filled(
                            progress_fill,
                            2.0,
                            egui::Color32::from_rgb(103, 80, 164),
                        );
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(MaterialButton::text("⏮")).clicked() {
                            println!("Previous track");
                        }
                        if ui.add(MaterialButton::filled("⏸")).clicked() {
                            println!("Pause");
                        }
                        if ui.add(MaterialButton::text("⏭")).clicked() {
                            println!("Next track");
                        }
                    });
                });
                ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
            }));
    }
}