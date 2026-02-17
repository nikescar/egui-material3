#![doc(hidden)]

use crate::{layout_grid, GridTile, GridTileBar, MaterialButton, MaterialCard2, MaterialCheckbox};
use eframe::egui::{self, Color32, Ui, Window};

#[doc(hidden)]
pub struct LayoutGridWindow {
    pub open: bool,
    columns: usize,
    gutter: f32,
    margin: f32,
    debug_mode: bool,
    max_width: Option<f32>,
    use_max_width: bool,
    show_grid_tiles: bool,
    show_elevation_demo: bool,
    interactive_card_count: usize,
}

impl Default for LayoutGridWindow {
    fn default() -> Self {
        Self {
            open: false,
            columns: 12,
            gutter: 16.0,
            margin: 24.0,
            debug_mode: true,
            max_width: Some(1200.0),
            use_max_width: false,
            show_grid_tiles: true,
            show_elevation_demo: true,
            interactive_card_count: 6,
        }
    }
}

impl LayoutGridWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Layout Grid Stories")
            .open(&mut open)
            .default_size([1200.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_layout_grid_examples(ui);
                    
                    if self.show_grid_tiles {
                        ui.add_space(40.0);
                        ui.separator();
                        ui.add_space(20.0);
                        self.render_grid_tile_gallery(ui);
                    }
                    
                    if self.show_elevation_demo {
                        ui.add_space(40.0);
                        ui.separator();
                        ui.add_space(20.0);
                        self.render_elevation_demo(ui);
                    }
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Layout Grid Controls");
            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ =
                    webbrowser::open("https://material-web.dev/components/layout-grid/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Columns:");
            ui.add(egui::Slider::new(&mut self.columns, 1..=24));
        });

        ui.horizontal(|ui| {
            ui.label("Gutter:");
            ui.add(egui::Slider::new(&mut self.gutter, 0.0..=48.0).suffix("px"));
        });

        ui.horizontal(|ui| {
            ui.label("Margin:");
            ui.add(egui::Slider::new(&mut self.margin, 0.0..=48.0).suffix("px"));
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.debug_mode, "Debug Mode"));
            ui.add(MaterialCheckbox::new(
                &mut self.use_max_width,
                "Use Max Width",
            ));
        });

        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.show_grid_tiles, "Show Grid Tiles"));
            ui.add(MaterialCheckbox::new(&mut self.show_elevation_demo, "Show Elevation Demo"));
        });

        if self.use_max_width {
            ui.horizontal(|ui| {
                ui.label("Max Width:");
                let mut max_width = self.max_width.unwrap_or(1200.0);
                ui.add(egui::Slider::new(&mut max_width, 400.0..=2000.0).suffix("px"));
                self.max_width = Some(max_width);
            });
        }
    }

    fn render_layout_grid_examples(&mut self, ui: &mut Ui) {
        ui.heading("Basic 12-Column Grid");

        let mut basic_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode);

        if self.use_max_width {
            basic_grid = basic_grid.max_width(self.max_width.unwrap_or(1200.0));
        }

        basic_grid = basic_grid
            .cell(4, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Column 1-4");
                    ui.label("First third of the row");
                }));
            })
            .cell(4, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Column 5-8");
                    ui.label("Middle third of the row");
                }));
            })
            .cell(4, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Column 9-12");
                    ui.label("Last third of the row");
                }));
            });

        ui.add(basic_grid);

        ui.add_space(30.0);

        ui.heading("Responsive Layout Example");

        let responsive_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell(6, |ui| {
                ui.add(MaterialCard2::elevated().content(|ui| {
                    ui.heading("Main Content");
                    ui.label("This takes up half the width on larger screens.");
                    ui.add_space(10.0);
                    ui.add(MaterialButton::filled("Primary Action"));
                }));
            })
            .cell(3, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("Sidebar 1");
                    ui.label("Quarter width content");
                    ui.add_space(5.0);
                    ui.add(MaterialButton::outlined("Action"));
                }));
            })
            .cell(3, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("Sidebar 2");
                    ui.label("Quarter width content");
                    ui.add_space(5.0);
                    ui.add(MaterialButton::text("Link"));
                }));
            });

        ui.add(responsive_grid);

        ui.add_space(30.0);

        ui.heading("Grid with Offsets");

        let offset_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell_with_offset(6, 3, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Centered Content");
                    ui.label("6 columns wide, offset by 3 columns");
                }));
            })
            .cell_with_offset(4, 2, |ui| {
                ui.add(MaterialCard2::elevated().content(|ui| {
                    ui.label("Offset Content");
                    ui.label("4 columns wide, offset by 2");
                }));
            })
            .cell_with_offset(4, 2, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("Another Offset");
                    ui.label("Same offset pattern");
                }));
            });

        ui.add(offset_grid);

        ui.add_space(30.0);

        ui.heading("Mixed Column Spans");

        let mixed_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell(12, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.heading("Full Width Header");
                    ui.label("This spans all 12 columns");
                }));
            })
            .cell(8, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("Main Article");
                    ui.label("8 columns for the main content area.");
                    ui.add_space(10.0);
                    ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
                }));
            })
            .cell(4, |ui| {
                ui.add(MaterialCard2::elevated().content(|ui| {
                    ui.label("Widget Area");
                    ui.label("4 columns for widgets");
                    ui.add_space(5.0);
                    if ui.add(MaterialButton::filled("Widget Action")).clicked() {
                        println!("Widget action!");
                    }
                    ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover());
                }));
            })
            .cell(3, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("25% width");
                }));
            })
            .cell(3, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("25% width");
                }));
            })
            .cell(3, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("25% width");
                }));
            })
            .cell(3, |ui| {
                ui.add(MaterialCard2::outlined().content(|ui| {
                    ui.label("25% width");
                }));
            });

        ui.add(mixed_grid);

        ui.add_space(30.0);

        ui.heading("Interactive Grid Demo");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Add Card")).clicked() {
                self.interactive_card_count += 1;
            }
            if ui.add(MaterialButton::outlined("Remove Card")).clicked() {
                if self.interactive_card_count > 0 {
                    self.interactive_card_count -= 1;
                }
            }
            if ui.add(MaterialButton::text("Reset Layout")).clicked() {
                self.interactive_card_count = 6;
            }
            ui.label(format!("Cards: {}", self.interactive_card_count));
        });

        ui.add_space(10.0);

        let mut interactive_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode);

        for i in 1..=self.interactive_card_count {
            interactive_grid = interactive_grid.cell(2, move |ui| {
                ui.add(MaterialCard2::filled().content(move |ui| {
                    ui.label(format!("Card {}", i));
                }));
            });
        }

        ui.add(interactive_grid);

        ui.add_space(20.0);

        ui.heading("Layout Grid Usage");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Common Patterns:");
                ui.label("• 12-column standard grid");
                ui.label("• Responsive breakpoints");
                ui.label("• Content + sidebar layouts");
                ui.label("• Card-based designs");
            });

            ui.vertical(|ui| {
                ui.label("Grid Benefits:");
                ui.label("• Consistent spacing");
                ui.label("• Responsive design");
                ui.label("• Visual alignment");
                ui.label("• Flexible layouts");
            });
        });
    }

    fn render_grid_tile_gallery(&mut self, ui: &mut Ui) {
        ui.heading("Grid Tile Gallery");
        ui.label("GridTile components with headers and footers for image galleries and card grids.");
        ui.add_space(20.0);

        ui.heading("Basic Grid Tiles with Footers");
        
        let tile_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell(4, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::filled().min_size(egui::Vec2::new(0.0, 200.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            ui.heading("🖼️");
                            ui.label("Image 1");
                        });
                    }));
                }).footer(
                    GridTileBar::new()
                        .title("Mountain View")
                        .subtitle("Location: Alps")
                        .background_color(Color32::from_black_alpha(180))
                ));
            })
            .cell(4, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::filled().min_size(egui::Vec2::new(0.0, 200.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            ui.heading("🌅");
                            ui.label("Image 2");
                        });
                    }));
                }).footer(
                    GridTileBar::new()
                        .title("Sunset Beach")
                        .subtitle("Location: California")
                        .background_color(Color32::from_black_alpha(180))
                ));
            })
            .cell(4, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::filled().min_size(egui::Vec2::new(0.0, 200.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            ui.heading("🏙️");
                            ui.label("Image 3");
                        });
                    }));
                }).footer(
                    GridTileBar::new()
                        .title("City Lights")
                        .subtitle("Location: Tokyo")
                        .background_color(Color32::from_black_alpha(180))
                ));
            });

        ui.add(tile_grid);
        ui.add_space(30.0);

        ui.heading("Grid Tiles with Headers and Actions");

        let action_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell(6, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::elevated().min_size(egui::Vec2::new(0.0, 250.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(100.0);
                            ui.heading("📱");
                            ui.label("Product Image");
                        });
                    }));
                })
                .header(
                    GridTileBar::new()
                        .title("Featured Product")
                        .background_color(Color32::from_black_alpha(150))
                        .trailing(|ui| {
                            ui.label("⭐");
                        })
                )
                .footer(
                    GridTileBar::new()
                        .title("Smartphone XYZ")
                        .subtitle("$999.99")
                        .background_color(Color32::from_black_alpha(180))
                        .leading(|ui| {
                            ui.label("🛒");
                        })
                ));
            })
            .cell(6, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::elevated().min_size(egui::Vec2::new(0.0, 250.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(100.0);
                            ui.heading("💻");
                            ui.label("Product Image");
                        });
                    }));
                })
                .header(
                    GridTileBar::new()
                        .title("New Arrival")
                        .background_color(Color32::from_black_alpha(150))
                        .trailing(|ui| {
                            ui.label("🆕");
                        })
                )
                .footer(
                    GridTileBar::new()
                        .title("Laptop Pro")
                        .subtitle("$1,499.99")
                        .background_color(Color32::from_black_alpha(180))
                        .leading(|ui| {
                            ui.label("🛒");
                        })
                ));
            });

        ui.add(action_grid);
        ui.add_space(30.0);

        ui.heading("Photo Gallery Grid");

        let photo_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell(3, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::outlined().min_size(egui::Vec2::new(0.0, 180.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(70.0);
                            ui.heading("🎨");
                        });
                    }));
                }).footer(
                    GridTileBar::new()
                        .title("Art 1")
                        .background_color(Color32::from_black_alpha(160))
                ));
            })
            .cell(3, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::outlined().min_size(egui::Vec2::new(0.0, 180.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(70.0);
                            ui.heading("🎭");
                        });
                    }));
                }).footer(
                    GridTileBar::new()
                        .title("Art 2")
                        .background_color(Color32::from_black_alpha(160))
                ));
            })
            .cell(3, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::outlined().min_size(egui::Vec2::new(0.0, 180.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(70.0);
                            ui.heading("🎪");
                        });
                    }));
                }).footer(
                    GridTileBar::new()
                        .title("Art 3")
                        .background_color(Color32::from_black_alpha(160))
                ));
            })
            .cell(3, |ui| {
                ui.add(GridTile::new(|ui| {
                    ui.add(MaterialCard2::outlined().min_size(egui::Vec2::new(0.0, 180.0)).content(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(70.0);
                            ui.heading("🎬");
                        });
                    }));
                }).footer(
                    GridTileBar::new()
                        .title("Art 4")
                        .background_color(Color32::from_black_alpha(160))
                ));
            });

        ui.add(photo_grid);
    }

    fn render_elevation_demo(&mut self, ui: &mut Ui) {
        ui.heading("Elevation Grid Demo");
        ui.label("Demonstrating Material Design elevation levels with surface tint and shadow effects.");
        ui.add_space(20.0);

        // Elevation data
        let elevations = [
            (0, 0.0, 0),
            (1, 1.0, 5),
            (2, 3.0, 8),
            (3, 6.0, 11),
            (4, 8.0, 12),
            (5, 12.0, 14),
        ];

        ui.heading("Surface Tint Color Only");
        ui.add_space(10.0);

        let tint_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(false);

        let tint_grid = elevations.iter().fold(tint_grid, |grid, (level, elevation, percent)| {
            let level = *level;
            let elevation = *elevation;
            let percent = *percent;
            grid.cell(2, move |ui| {
                ui.add(MaterialCard2::elevated()
                    .elevation(elevation as f32)
                    .min_size(egui::Vec2::new(0.0, 120.0))
                    .content(move |ui| {
                        ui.vertical(|ui| {
                            ui.label(format!("Level {}", level));
                            ui.label(format!("{} dp", elevation));
                            ui.add_space(10.0);
                            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                                ui.label(egui::RichText::new(format!("{}%", percent))
                                    .small()
                                    .color(ui.style().visuals.weak_text_color()));
                            });
                        });
                    }));
            })
        });

        ui.add(tint_grid);
        ui.add_space(30.0);

        ui.heading("Surface Tint Color and Shadow Color");
        ui.add_space(10.0);

        let shadow_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(false);

        let shadow_grid = elevations.iter().fold(shadow_grid, |grid, (level, elevation, percent)| {
            let level = *level;
            let elevation = *elevation;
            let percent = *percent;
            grid.cell(2, move |ui| {
                ui.add(MaterialCard2::elevated()
                    .elevation(elevation as f32)
                    .min_size(egui::Vec2::new(0.0, 120.0))
                    .content(move |ui| {
                        ui.vertical(|ui| {
                            ui.label(format!("Level {}", level));
                            ui.label(format!("{} dp", elevation));
                            ui.add_space(10.0);
                            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                                ui.label(egui::RichText::new(format!("{}%", percent))
                                    .small()
                                    .color(ui.style().visuals.weak_text_color()));
                            });
                        });
                    }));
            })
        });

        ui.add(shadow_grid);
        ui.add_space(30.0);

        ui.heading("Elevation Comparison");
        ui.label("Side-by-side comparison of different elevation levels:");

        let comparison_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell(4, |ui| {
                ui.vertical(|ui| {
                    ui.label("Low Elevation (Level 1)");
                    ui.add(MaterialCard2::elevated()
                        .elevation(1.0)
                        .min_size(egui::Vec2::new(0.0, 100.0))
                        .content(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(30.0);
                                ui.label("Subtle depth");
                            });
                        }));
                });
            })
            .cell(4, |ui| {
                ui.vertical(|ui| {
                    ui.label("Medium Elevation (Level 3)");
                    ui.add(MaterialCard2::elevated()
                        .elevation(6.0)
                        .min_size(egui::Vec2::new(0.0, 100.0))
                        .content(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(30.0);
                                ui.label("Moderate depth");
                            });
                        }));
                });
            })
            .cell(4, |ui| {
                ui.vertical(|ui| {
                    ui.label("High Elevation (Level 5)");
                    ui.add(MaterialCard2::elevated()
                        .elevation(12.0)
                        .min_size(egui::Vec2::new(0.0, 100.0))
                        .content(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(30.0);
                                ui.label("Strong depth");
                            });
                        }));
                });
            });

        ui.add(comparison_grid);
    }
}
