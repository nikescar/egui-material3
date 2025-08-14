use eframe::egui;
use egui_material::{
    MaterialCard, MaterialButton, MaterialCheckbox, layout_grid,
    theme::{setup_google_fonts, setup_local_fonts, load_fonts}
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "egui Material Design Example",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts - prepare Nanum Gothic and Material Symbols
            setup_google_fonts(Some("Nanum Gothic"));
            setup_local_fonts(Some("resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf"));
            // Load all prepared fonts
            load_fonts(&cc.egui_ctx);
            
            Ok(Box::<MaterialApp>::default())
        }),
    )
}

struct MaterialApp {
    columns: usize,
    gutter: f32,
    margin: f32,
    debug_mode: bool,
    max_width: Option<f32>,
    use_max_width: bool,
}

impl Default for MaterialApp {
    fn default() -> Self {
        Self {
            columns: 12,
            gutter: 16.0,
            margin: 24.0,
            debug_mode: true,
            max_width: Some(1200.0),
            use_max_width: false,
        }
    }
}

impl eframe::App for MaterialApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Material Design Layout Grid Example");
            ui.label("Using Nanum Gothic font with Material Design components");
            
            ui.add_space(20.0);
            
            // Controls
            ui.horizontal(|ui| {
                ui.heading("Grid Controls");
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
                ui.add(MaterialCheckbox::new(&mut self.use_max_width, "Use Max Width"));
            });

            if self.use_max_width {
                ui.horizontal(|ui| {
                    ui.label("Max Width:");
                    let mut max_width = self.max_width.unwrap_or(1200.0);
                    ui.add(egui::Slider::new(&mut max_width, 400.0..=2000.0).suffix("px"));
                    self.max_width = Some(max_width);
                });
            }
            
            ui.add_space(20.0);
            
            // Basic 12-Column Grid
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
                    ui.add(MaterialCard::filled()
                        .content(|ui| {
                            ui.label("Column 1-4");
                            ui.label("First third of the row");
                            ui.add_space(5.0);
                            ui.add(MaterialButton::filled("Action 1"));
                            ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                        }));
                })
                .cell(4, |ui| {
                    ui.add(MaterialCard::elevated()
                        .content(|ui| {
                            ui.label("Column 5-8");
                            ui.label("Middle third of the row");
                            ui.add_space(5.0);
                            ui.add(MaterialButton::outlined("Action 2"));
                            ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                        }));
                })
                .cell(4, |ui| {
                    ui.add(MaterialCard::outlined()
                        .content(|ui| {
                            ui.label("Column 9-12");
                            ui.label("Last third of the row");
                            ui.add_space(5.0);
                            ui.add(MaterialButton::text("Action 3"));
                            ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                        }));
                });
            
            ui.add(basic_grid);

            ui.add_space(30.0);
            
            // Responsive Layout Example
            ui.heading("Responsive Layout Example");
            
            let responsive_grid = layout_grid()
                .columns(self.columns)
                .gutter(self.gutter)
                .margin(self.margin)
                .debug_mode(self.debug_mode)
                .cell(8, |ui| {
                    ui.add(MaterialCard::elevated()
                        .content(|ui| {
                            ui.heading("Main Content Area");
                            ui.label("This is the main content area taking up 8 columns.");
                            ui.label("Perfect for articles, posts, or primary content.");
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                ui.add(MaterialButton::filled("Primary Action"));
                                ui.add(MaterialButton::outlined("Secondary"));
                            });
                            ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                        }));
                })
                .cell(4, |ui| {
                    ui.add(MaterialCard::filled()
                        .content(|ui| {
                            ui.label("Sidebar");
                            ui.label("This sidebar takes 4 columns.");
                            ui.add_space(10.0);
                            ui.label("Widget 1");
                            ui.add(MaterialButton::text("Link"));
                            ui.add_space(5.0);
                            ui.label("Widget 2");
                            ui.add(MaterialButton::text("Another Link"));
                            ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                        }));
                });
            
            ui.add(responsive_grid);

            ui.add_space(30.0);
            
            // Mixed Column Layout
            ui.heading("Mixed Column Layout");
            
            let mixed_grid = layout_grid()
                .columns(self.columns)
                .gutter(self.gutter)
                .margin(self.margin)
                .debug_mode(self.debug_mode)
                .cell(12, |ui| {
                    ui.add(MaterialCard::filled()
                        .content(|ui| {
                            ui.heading("Full Width Header");
                            ui.label("This header spans all 12 columns - perfect for titles and navigation");
                            ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                        }));
                })
                .cell(3, |ui| {
                    ui.add(MaterialCard::outlined().content(|ui| {
                        ui.label("Card 1");
                        ui.label("25% width");
                        ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                    }));
                })
                .cell(3, |ui| {
                    ui.add(MaterialCard::outlined().content(|ui| {
                        ui.label("Card 2");
                        ui.label("25% width");
                        ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                    }));
                })
                .cell(3, |ui| {
                    ui.add(MaterialCard::outlined().content(|ui| {
                        ui.label("Card 3");
                        ui.label("25% width");
                        ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                    }));
                })
                .cell(3, |ui| {
                    ui.add(MaterialCard::outlined().content(|ui| {
                        ui.label("Card 4");
                        ui.label("25% width");
                        ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
                    }));
                });
            
            ui.add(mixed_grid);

            ui.add_space(20.0);
            
            // Info section
            ui.heading("About This Example");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Features Demonstrated:");
                    ui.label("• Nanum Gothic custom font loading");
                    ui.label("• Material Design layout grid");
                    ui.label("• Material Design cards");
                    ui.label("• Material Design buttons");
                    ui.label("• Interactive grid controls");
                });
                
                ui.vertical(|ui| {
                    ui.label("Grid System Benefits:");
                    ui.label("• Consistent spacing and alignment");
                    ui.label("• Responsive design capabilities");
                    ui.label("• Flexible column-based layouts");
                    ui.label("• Material Design guidelines compliance");
                });
            });
        });
    }
}