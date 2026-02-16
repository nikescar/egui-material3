#![doc(hidden)]

use crate::{elevated_card2, filled_card2, outlined_card2, MaterialButton, MaterialCheckbox};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
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
    // New elevation and styling controls
    demo_elevation: f32,
    show_surface_tint: bool,
    show_shadow: bool,
    elevation_mode: ElevationMode,
    clip_content: bool,
    border_foreground: bool,
    margin: f32,
    // Custom cards management
    custom_cards: Vec<CustomCard>,
    next_card_id: usize,
    edit_dialog_open: bool,
    editing_card: Option<CustomCard>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ElevationMode {
    TintOnly,
    TintAndShadow,
    ShadowOnly,
}

#[derive(Debug, Clone)]
struct CustomCard {
    id: usize,
    title: String,
    subtitle: String,
    content: String,
    variant: CardVariant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CardVariant {
    Elevated,
    Filled,
    Outlined,
}

impl Default for CustomCard {
    fn default() -> Self {
        Self {
            id: 0,
            title: "New Card".to_string(),
            subtitle: "Card subtitle".to_string(),
            content: "Card content goes here...".to_string(),
            variant: CardVariant::Elevated,
        }
    }
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
            demo_elevation: 1.0,
            show_surface_tint: true,
            show_shadow: true,
            elevation_mode: ElevationMode::TintAndShadow,
            clip_content: false,
            border_foreground: true,
            margin: 4.0,
            custom_cards: Vec::new(),
            next_card_id: 1,
            edit_dialog_open: false,
            editing_card: None,
        }
    }
}

impl Card2Window {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Enhanced Card (Card2) Stories")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_elevation_demo(ui);
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

        ui.separator();
        ui.label("Card Content:");
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
            ui.add(MaterialCheckbox::new(
                &mut self.show_subtitle,
                "Show Subtitle",
            ));
            ui.add(MaterialCheckbox::new(
                &mut self.show_media,
                "Show Media Area",
            ));
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(
                &mut self.show_actions,
                "Show Actions",
            ));
            ui.add(MaterialCheckbox::new(
                &mut self.clickable_cards,
                "Clickable Cards",
            ));
        });

        if self.show_media {
            ui.horizontal(|ui| {
                ui.label("Media Height:");
                ui.add(egui::Slider::new(&mut self.media_height, 80.0..=300.0).suffix("px"));
            });
        }

        ui.separator();
        ui.label("Elevation & Styling:");
        
        ui.horizontal(|ui| {
            ui.label("Elevation:");
            ui.add(egui::Slider::new(&mut self.demo_elevation, 0.0..=12.0).suffix(" dp"));
            ui.label(format!("Level: {}", self.get_elevation_level(self.demo_elevation)));
        });

        ui.horizontal(|ui| {
            ui.label("Margin:");
            ui.add(egui::Slider::new(&mut self.margin, 0.0..=20.0).suffix("px"));
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.show_surface_tint, "Surface Tint"));
            ui.add(MaterialCheckbox::new(&mut self.show_shadow, "Shadow"));
            ui.add(MaterialCheckbox::new(&mut self.clip_content, "Clip Content"));
            ui.add(MaterialCheckbox::new(&mut self.border_foreground, "Border Foreground"));
        });

        ui.horizontal(|ui| {
            ui.label("Elevation Mode:");
            ui.radio_value(&mut self.elevation_mode, ElevationMode::TintOnly, "Tint Only");
            ui.radio_value(&mut self.elevation_mode, ElevationMode::TintAndShadow, "Tint + Shadow");
            ui.radio_value(&mut self.elevation_mode, ElevationMode::ShadowOnly, "Shadow Only");
        });
    }
    fn get_elevation_level(&self, elevation: f32) -> i32 {
        match elevation as i32 {
            0 => 0,
            1 => 1,
            2..=3 => 2,
            4..=6 => 3,
            7..=8 => 4,
            _ => 5,
        }
    }

    fn get_overlay_percent(&self, elevation: f32) -> i32 {
        match elevation as i32 {
            0 => 0,
            1 => 5,
            2..=3 => 8,
            4..=6 => 11,
            7..=8 => 12,
            _ => 14,
        }
    }

    fn render_elevation_demo(&mut self, ui: &mut Ui) {
        ui.heading("Elevation Demonstration");
        
        ui.label("Material 3 defines 6 elevation levels (0-5) with corresponding overlay tints:");
        ui.add_space(10.0);

        // Render three modes
        match self.elevation_mode {
            ElevationMode::TintOnly => {
                ui.label("Surface Tint Color Only");
                self.render_elevation_grid(ui, true, false);
            }
            ElevationMode::TintAndShadow => {
                ui.label("Surface Tint Color and Shadow Color");
                self.render_elevation_grid(ui, true, true);
            }
            ElevationMode::ShadowOnly => {
                ui.label("Shadow Color Only");
                self.render_elevation_grid(ui, false, true);
            }
        }
    }

    fn render_elevation_grid(&mut self, ui: &mut Ui, show_tint: bool, show_shadow: bool) {
        // Standard Material 3 elevation levels
        let elevations = [(0, 0.0), (1, 1.0), (2, 3.0), (3, 6.0), (4, 8.0), (5, 12.0)];

        egui::Grid::new("elevation_grid")
            .num_columns(6)
            .spacing([8.0, 8.0])
            .show(ui, |ui| {
                for (level, elevation) in elevations.iter() {
                    ui.vertical(|ui| {
                        let mut card = elevated_card2()
                            .min_size(egui::Vec2::new(100.0, 100.0))
                            .elevation(*elevation)
                            .margin(0.0);

                        // Apply tint/shadow based on mode
                        if !show_tint {
                            card = card.surface_tint_color(egui::Color32::TRANSPARENT);
                        }
                        if !show_shadow {
                            card = card.shadow_color(egui::Color32::TRANSPARENT);
                        }

                        card = card.content(|ui| {
                            ui.vertical(|ui| {
                                ui.label(format!("Level {}", level));
                                ui.label(format!("{} dp", elevation.round() as i32));
                                if show_tint {
                                    ui.add_space(5.0);
                                    ui.label(format!("{}%", self.get_overlay_percent(*elevation)));
                                }
                            });
                        });

                        ui.add(card);
                    });
                }
            });
    }
    fn render_card2_examples(&mut self, ui: &mut Ui) {
        ui.heading("Enhanced Card Variants");

        ui.horizontal_wrapped(|ui| {
            // Elevated Card
            let mut elevated_card = elevated_card2()
                .clickable(self.clickable_cards)
                .media_height(self.media_height)
                .elevation(self.demo_elevation)
                .margin(self.margin)
                .clip_behavior(self.clip_content)
                .border_on_foreground(self.border_foreground);

            // Apply tint/shadow based on controls
            if !self.show_surface_tint {
                elevated_card = elevated_card.surface_tint_color(egui::Color32::TRANSPARENT);
            }
            if !self.show_shadow {
                elevated_card = elevated_card.shadow_color(egui::Color32::TRANSPARENT);
            }

            if self.show_header {
                elevated_card = elevated_card.header(
                    &self.card_title,
                    if self.show_subtitle {
                        Some(&self.card_subtitle)
                    } else {
                        None
                    },
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
                .media_height(self.media_height)
                .elevation(self.demo_elevation)
                .margin(self.margin)
                .clip_behavior(self.clip_content)
                .border_on_foreground(self.border_foreground);

            // Apply tint/shadow based on controls
            if !self.show_surface_tint {
                filled_card = filled_card.surface_tint_color(egui::Color32::TRANSPARENT);
            }
            if !self.show_shadow {
                filled_card = filled_card.shadow_color(egui::Color32::TRANSPARENT);
            }

            if self.show_header {
                filled_card = filled_card.header(
                    "Filled Card",
                    if self.show_subtitle {
                        Some("Filled variant")
                    } else {
                        None
                    },
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
                .media_height(self.media_height)
                .elevation(self.demo_elevation)
                .margin(self.margin)
                .clip_behavior(self.clip_content)
                .border_on_foreground(self.border_foreground);

            // Apply tint/shadow based on controls
            if !self.show_surface_tint {
                outlined_card = outlined_card.surface_tint_color(egui::Color32::TRANSPARENT);
            }
            if !self.show_shadow {
                outlined_card = outlined_card.shadow_color(egui::Color32::TRANSPARENT);
            }

            if self.show_header {
                outlined_card = outlined_card.header(
                    "Outlined Card",
                    if self.show_subtitle {
                        Some("Outlined variant")
                    } else {
                        None
                    },
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
            ui.add(
                elevated_card2()
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
                    }),
            );

            // Article Card
            ui.add(
                outlined_card2()
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
                    }),
            );

            // Profile Card
            ui.add(
                filled_card2()
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
                    }),
            );
        });

        ui.add_space(30.0);

        ui.heading("Interactive Demo - Custom Cards");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Create New Card")).clicked() {
                self.create_new_card();
            }
            if ui.add(MaterialButton::outlined("Edit Card")).clicked() {
                if let Some(card) = self.custom_cards.first().cloned() {
                    self.editing_card = Some(card);
                    self.edit_dialog_open = true;
                }
            }
            if ui.add(MaterialButton::text("Delete Card")).clicked() {
                if !self.custom_cards.is_empty() {
                    self.custom_cards.remove(0);
                }
            }
            ui.label(format!("({} custom cards)", self.custom_cards.len()));
        });

        ui.add_space(10.0);

        // Render custom cards
        if self.custom_cards.is_empty() {
            ui.label("No custom cards yet. Click 'Create New Card' to add one.");
        } else {
            let mut card_to_edit: Option<usize> = None;
            let mut card_to_delete: Option<usize> = None;
            
            ui.vertical(|ui| {
                let cards_to_render = self.custom_cards.clone();
                for (idx, card) in cards_to_render.iter().enumerate() {
                    let (edit_clicked, delete_clicked) = self.render_custom_card(ui, card);
                    if edit_clicked {
                        card_to_edit = Some(idx);
                    }
                    if delete_clicked {
                        card_to_delete = Some(idx);
                    }
                    ui.add_space(10.0);
                }
            });
            
            // Handle actions after rendering
            if let Some(idx) = card_to_edit {
                if let Some(card) = self.custom_cards.get(idx).cloned() {
                    self.editing_card = Some(card);
                    self.edit_dialog_open = true;
                }
            }
            if let Some(idx) = card_to_delete {
                self.custom_cards.remove(idx);
            }
        }

        // Edit dialog
        self.render_edit_dialog(ui.ctx());
    }

    fn create_new_card(&mut self) {
        let new_card = CustomCard {
            id: self.next_card_id,
            title: format!("Card {}", self.next_card_id),
            subtitle: "New card subtitle".to_string(),
            content: "This is a newly created card. Click Edit to customize it.".to_string(),
            variant: CardVariant::Elevated,
        };
        self.custom_cards.push(new_card);
        self.next_card_id += 1;
    }

    fn render_custom_card(&self, ui: &mut Ui, card: &CustomCard) -> (bool, bool) {
        let mut edit_clicked = false;
        let mut delete_clicked = false;
        
        ui.vertical(|ui| {
            let card_builder = match card.variant {
                CardVariant::Elevated => elevated_card2(),
                CardVariant::Filled => filled_card2(),
                CardVariant::Outlined => outlined_card2(),
            };

            let card_builder = card_builder
                .header(&card.title, Some(&card.subtitle))
                .content(|ui| {
                    ui.label(&card.content);
                });

            ui.add(card_builder);
            
            // Render action buttons below the card
            ui.horizontal(|ui| {
                if ui.add(MaterialButton::text("Edit")).clicked() {
                    edit_clicked = true;
                }
                if ui.add(MaterialButton::text("Delete")).clicked() {
                    delete_clicked = true;
                }
            });
        });
        
        (edit_clicked, delete_clicked)
    }

    fn render_edit_dialog(&mut self, ctx: &egui::Context) {
        if !self.edit_dialog_open {
            return;
        }

        let mut open = self.edit_dialog_open;
        let mut save_clicked = false;
        let mut cancel_clicked = false;
        
        egui::Window::new("Edit Card")
            .open(&mut open)
            .default_size([400.0, 300.0])
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(ref mut card) = self.editing_card {
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut card.title);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Subtitle:");
                        ui.text_edit_singleline(&mut card.subtitle);
                    });

                    ui.label("Content:");
                    ui.text_edit_multiline(&mut card.content);

                    ui.horizontal(|ui| {
                        ui.label("Variant:");
                        ui.radio_value(&mut card.variant, CardVariant::Elevated, "Elevated");
                        ui.radio_value(&mut card.variant, CardVariant::Filled, "Filled");
                        ui.radio_value(&mut card.variant, CardVariant::Outlined, "Outlined");
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.add(MaterialButton::filled("Save")).clicked() {
                            save_clicked = true;
                        }
                        if ui.add(MaterialButton::text("Cancel")).clicked() {
                            cancel_clicked = true;
                        }
                    });
                }
            });
        
        // Handle save/cancel after the window closes to avoid borrow issues
        if save_clicked {
            if let Some(card) = &self.editing_card {
                if let Some(existing) = self.custom_cards.iter_mut().find(|c| c.id == card.id) {
                    *existing = card.clone();
                }
            }
            self.edit_dialog_open = false;
            self.editing_card = None;
        }
        
        if cancel_clicked {
            self.edit_dialog_open = false;
            self.editing_card = None;
        }
        
        self.edit_dialog_open = open;
    }
}
