use eframe::egui::{self, Ui, Window};
use crate::{MaterialButton, MaterialCheckbox, elevated_card2, filled_card2, outlined_card2};

pub struct Card2Window {
    pub open: bool,
    show_header: bool,
    show_subtitle: bool,
    show_media: bool,
    show_actions: bool,
    clickable_cards: bool,
    card_title: String,
    card_subtitle: String,
    media_height: f32,
}

impl Default for Card2Window {
    fn default() -> Self {
        Self {
            open: false,
            show_header: true,
            show_subtitle: true,
            show_media: true,
            show_actions: true,
            clickable_cards: false,
            card_title: "Enhanced Card Title".to_string(),
            card_subtitle: "This is a subtitle".to_string(),
            media_height: 160.0,
        }
    }
}

impl Card2Window {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Enhanced Card (Card2) Stories")
            .open(&mut open)
            .default_size([400.0, 300.0])
            .max_size([400.0, 350.0])
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_card2_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Enhanced Card Controls");
            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-components.github.io/material-components-web-catalog/#/component/card");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.card_title);
        });

        ui.horizontal(|ui| {
            ui.label("Subtitle:");
            ui.text_edit_singleline(&mut self.card_subtitle);
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.show_header, "Show Header"));
            ui.add(MaterialCheckbox::new(&mut self.show_subtitle, "Show Subtitle"));
            ui.add(MaterialCheckbox::new(&mut self.show_media, "Show Media Area"));
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.show_actions, "Show Actions"));
            ui.add(MaterialCheckbox::new(&mut self.clickable_cards, "Clickable Cards"));
        });

        if self.show_media {
            ui.horizontal(|ui| {
                ui.label("Media Height:");
                ui.add(egui::Slider::new(&mut self.media_height, 80.0..=300.0).suffix("px"));
            });
        }
    }

    fn render_card2_examples(&mut self, ui: &mut Ui) {
        ui.heading("Enhanced Card Variants");
        
        ui.horizontal_wrapped(|ui| {
            // Elevated Card
            let mut elevated_card = elevated_card2()
                .clickable(self.clickable_cards)
                .media_height(self.media_height);
                
            if self.show_header {
                elevated_card = elevated_card.header(
                    &self.card_title, 
                    if self.show_subtitle { Some(&self.card_subtitle) } else { None }
                );
            }
            
            if self.show_media {
                elevated_card = elevated_card.media_area(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("📷 Media Content");
                        ui.label("Image or video would go here");
                    });
                });
            }
            
            elevated_card = elevated_card.content(|ui| {
                ui.label("This is an elevated card with enhanced features.");
                ui.add_space(5.0);
                ui.label("It supports headers, media areas, content, and actions.");
            });
            
            if self.show_actions {
                elevated_card = elevated_card.actions(|ui| {
                    if ui.add(MaterialButton::text("Share")).clicked() {
                        println!("Elevated card: Share clicked!");
                    }
                    if ui.add(MaterialButton::text("Learn More")).clicked() {
                        println!("Elevated card: Learn More clicked!");
                    }
                });
            }
            
            ui.add(elevated_card);
            
            // Filled Card
            let mut filled_card = filled_card2()
                .clickable(self.clickable_cards)
                .media_height(self.media_height);
                
            if self.show_header {
                filled_card = filled_card.header(
                    "Filled Card", 
                    if self.show_subtitle { Some("Filled variant") } else { None }
                );
            }
            
            if self.show_media {
                filled_card = filled_card.media_area(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("🎨 Filled Media");
                        ui.label("Filled card media area");
                    });
                });
            }
            
            filled_card = filled_card.content(|ui| {
                ui.label("This is a filled card variant.");
                ui.add_space(5.0);
                ui.label("It has a filled background color.");
            });
            
            if self.show_actions {
                filled_card = filled_card.actions(|ui| {
                    if ui.add(MaterialButton::text("Action")).clicked() {
                        println!("Filled card: Action clicked!");
                    }
                });
            }
            
            ui.add(filled_card);
            
            // Outlined Card
            let mut outlined_card = outlined_card2()
                .clickable(self.clickable_cards)
                .media_height(self.media_height);
                
            if self.show_header {
                outlined_card = outlined_card.header(
                    "Outlined Card", 
                    if self.show_subtitle { Some("Outlined variant") } else { None }
                );
            }
            
            if self.show_media {
                outlined_card = outlined_card.media_area(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("🖼️ Outlined Media");
                        ui.label("Outlined card media");
                    });
                });
            }
            
            outlined_card = outlined_card.content(|ui| {
                ui.label("This is an outlined card variant.");
                ui.add_space(5.0);
                ui.label("It has a visible border outline.");
            });
            
            if self.show_actions {
                outlined_card = outlined_card.actions(|ui| {
                    if ui.add(MaterialButton::text("View")).clicked() {
                        println!("Outlined card: View clicked!");
                    }
                    if ui.add(MaterialButton::text("Edit")).clicked() {
                        println!("Outlined card: Edit clicked!");
                    }
                });
            }
            
            ui.add(outlined_card);
        });

        ui.add_space(30.0);
        
        ui.heading("Specialized Card Examples");
        
        ui.horizontal_wrapped(|ui| {
            // Product Card
            ui.add(elevated_card2()
                .header("Premium Product", Some("Limited Edition"))
                .media_area(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("🛍️ Product Image");
                        ui.label("High quality product photo");
                    });
                })
                .content(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Price:");
                        ui.label("$299.99");
                    });
                    ui.label("Premium quality product with excellent features.");
                    ui.label("★★★★★ 4.8/5 stars");
                })
                .actions(|ui| {
                    if ui.add(MaterialButton::text("Buy Now")).clicked() {
                        println!("Product card: Buy Now clicked!");
                    }
                    if ui.add(MaterialButton::text("Add to Cart")).clicked() {
                        println!("Product card: Add to Cart clicked!");
                    }
                }));
                
            // Article Card
            ui.add(outlined_card2()
                .header("Latest News", Some("Tech Update"))
                .media_area(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("📰 Article Image");
                        ui.label("Featured article image");
                    });
                })
                .content(|ui| {
                    ui.label("Breaking: New technology breakthrough announced...");
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("By Author Name");
                        ui.label("•");
                        ui.label("2 hours ago");
                    });
                })
                .actions(|ui| {
                    if ui.add(MaterialButton::text("Read More")).clicked() {
                        println!("Article card: Read More clicked!");
                    }
                    if ui.add(MaterialButton::text("Share")).clicked() {
                        println!("Article card: Share clicked!");
                    }
                }));
                
            // Profile Card
            ui.add(filled_card2()
                .header("User Profile", Some("Premium Member"))
                .media_area(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("👤 Profile Picture");
                        ui.label("User avatar or photo");
                    });
                })
                .content(|ui| {
                    ui.label("John Doe");
                    ui.label("Software Developer");
                    ui.add_space(5.0);
                    ui.label("📍 San Francisco, CA");
                    ui.label("🎂 Joined March 2020");
                })
                .actions(|ui| {
                    if ui.add(MaterialButton::text("Follow")).clicked() {
                        println!("Profile card: Follow clicked!");
                    }
                    if ui.add(MaterialButton::text("Message")).clicked() {
                        println!("Profile card: Message clicked!");
                    }
                }));
        });

        ui.add_space(30.0);
        
        ui.heading("Interactive Demo");
        
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Create New Card")).clicked() {
                println!("Create new card functionality would be implemented here");
            }
            if ui.add(MaterialButton::outlined("Edit Card")).clicked() {
                println!("Edit card functionality would be implemented here");
            }
            if ui.add(MaterialButton::text("Delete Card")).clicked() {
                println!("Delete card functionality would be implemented here");
            }
        });
        
        ui.add_space(10.0);
        
        // Interactive card with dynamic content
        ui.add(elevated_card2()
            .header("Interactive Card", Some("Click actions below"))
            .content(|ui| {
                ui.label("This card demonstrates interactive functionality.");
                ui.add_space(5.0);
                ui.label("Use the controls above to modify card properties.");
                ui.label("Try toggling different sections on and off.");
            })
            .actions(|ui| {
                if ui.add(MaterialButton::text("Refresh")).clicked() {
                    println!("Interactive card: Refresh clicked!");
                }
                if ui.add(MaterialButton::text("Settings")).clicked() {
                    println!("Interactive card: Settings clicked!");
                }
                if ui.add(MaterialButton::text("Info")).clicked() {
                    println!("Interactive card: Info clicked!");
                }
            }));

        ui.add_space(20.0);
        
        ui.heading("Enhanced Card Features");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Header Section:");
                ui.label("• Primary title");
                ui.label("• Optional subtitle");
                ui.label("• Consistent typography");
                ui.label("• Proper spacing");
            });
            
            ui.vertical(|ui| {
                ui.label("Media Area:");
                ui.label("• Configurable height");
                ui.label("• Image/video support");
                ui.label("• Responsive design");
                ui.label("• Clipped to card bounds");
            });
            
            ui.vertical(|ui| {
                ui.label("Actions Area:");
                ui.label("• Right-aligned buttons");
                ui.label("• Consistent spacing");
                ui.label("• Material button styling");
                ui.label("• Multiple action support");
            });
        });
    }
}