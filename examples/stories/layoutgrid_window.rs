#![doc(hidden)]

use crate::{layout_grid, MaterialButton, MaterialCard2, MaterialCheckbox};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub struct LayoutGridWindow {
    pub open: bool,
    columns: usize,
    gutter: f32,
    margin: f32,
    debug_mode: bool,
    max_width: Option<f32>,
    use_max_width: bool,
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
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Layout Grid Controls");
            if ui.button("Target").clicked() {
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
            if ui.add(MaterialButton::filled("Add Column")).clicked() {
                println!("Add column functionality would be implemented here");
            }
            if ui.add(MaterialButton::outlined("Remove Column")).clicked() {
                println!("Remove column functionality would be implemented here");
            }
            if ui.add(MaterialButton::text("Reset Layout")).clicked() {
                println!("Reset layout functionality would be implemented here");
            }
        });

        ui.add_space(10.0);

        let interactive_grid = layout_grid()
            .columns(self.columns)
            .gutter(self.gutter)
            .margin(self.margin)
            .debug_mode(self.debug_mode)
            .cell(2, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Card 1");
                }));
            })
            .cell(2, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Card 2");
                }));
            })
            .cell(2, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Card 3");
                }));
            })
            .cell(2, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Card 4");
                }));
            })
            .cell(2, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Card 5");
                }));
            })
            .cell(2, |ui| {
                ui.add(MaterialCard2::filled().content(|ui| {
                    ui.label("Card 6");
                }));
            });

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
}
