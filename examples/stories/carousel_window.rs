#![doc(hidden)]

use crate::MaterialButton;
use eframe::egui::{self, Window};
use egui_material3::carousel;

#[doc(hidden)]
pub struct CarouselWindow {
    pub open: bool,
    scroll_offset_1: f32,
    scroll_offset_2: f32,
}

impl Default for CarouselWindow {
    fn default() -> Self {
        Self {
            open: false,
            scroll_offset_1: 0.0,
            scroll_offset_2: 0.0,
        }
    }
}

impl CarouselWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Carousel Stories")
            .open(&mut open)
            .default_size([800.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_intro(ui);
                    ui.add_space(20.0);
                    self.render_uncontained_carousel(ui);
                    ui.add_space(20.0);
                    self.render_snapping_carousel(ui);
                });
            });
        self.open = open;
    }

    fn render_intro(&mut self, ui: &mut egui::Ui) {
        ui.push_id("carousel_intro", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Carousel Stories");

                if ui
                    .add(MaterialButton::filled("Material 3 Docs").small())
                    .clicked()
                {
                    let _ = webbrowser::open("https://m3.material.io/components/carousel/overview");
                }

                if ui
                    .add(MaterialButton::filled("Flutter Carousel").small())
                    .clicked()
                {
                    let _ = webbrowser::open(
                        "https://api.flutter.dev/flutter/material/CarouselView-class.html",
                    );
                }
            });

            ui.add_space(8.0);
            ui.label("Material Design 3 Carousel component - horizontally scrollable lists with smooth transitions.");
            ui.label("Items at the edges shrink while center items display at full size.");
        });
    }

    fn render_uncontained_carousel(&mut self, ui: &mut egui::Ui) {
        ui.push_id("uncontained_carousel", |ui| {
            ui.heading("Uncontained Carousel");
            ui.label("Basic carousel with 20 items, no snapping");
            ui.add_space(10.0);

            // Create carousel with 20 text items
            let mut carousel_widget = carousel(&mut self.scroll_offset_1)
                .id_salt("uncontained")
                .item_extent(200.0)
                .shrink_extent(120.0)
                .height(180.0);

            // Add 20 items
            for i in 0..20 {
                carousel_widget = carousel_widget.item_text(format!("Item {}", i));
            }

            ui.add(carousel_widget);

            ui.add_space(10.0);
            ui.label(format!(
                "Scroll offset: {:.0}px",
                self.scroll_offset_1
            ));
        });
    }

    fn render_snapping_carousel(&mut self, ui: &mut egui::Ui) {
        ui.push_id("snapping_carousel", |ui| {
            ui.heading("Snapping Carousel");
            ui.label("Carousel with item snapping enabled - items snap to alignment");
            ui.add_space(10.0);

            // Create carousel with item snapping
            let mut carousel_widget = carousel(&mut self.scroll_offset_2)
                .id_salt("snapping")
                .item_extent(200.0)
                .shrink_extent(120.0)
                .height(180.0)
                .item_snapping(true);

            // Add 20 items with more elaborate content
            for i in 0..20 {
                carousel_widget = carousel_widget.item(Box::new(move |ui: &mut egui::Ui, _rect| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.heading(format!("#{}", i + 1));
                        ui.add_space(10.0);
                        ui.label(format!("Item {}", i));
                        ui.add_space(10.0);
                        ui.label("📦");
                        ui.add_space(10.0);
                        if ui
                            .add(MaterialButton::filled("Action").small())
                            .clicked()
                        {
                            println!("Item {} action clicked", i);
                        }
                    });
                }));
            }

            ui.add(carousel_widget);

            ui.add_space(10.0);
            ui.label(format!(
                "Scroll offset: {:.0}px",
                self.scroll_offset_2
            ));
            ui.label("Scroll and release - items will snap to alignment");
        });
    }
}
