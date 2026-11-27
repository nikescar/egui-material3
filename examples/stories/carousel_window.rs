// ============================================================================
// Module:       stories::carousel_window (example)
// Description:  Story window demonstrating the multi-browse, uncontained,
//               hero and full-screen carousels.
//
// Dependencies: eframe, egui; crate::{MaterialCarousel, CarouselStrategy}
// ============================================================================

//! Story window demonstrating the carousel component.

#![doc(hidden)]

use crate::{CarouselStrategy, MaterialButton, MaterialCarousel};
use eframe::egui::{self, Color32, Window};

#[doc(hidden)]
#[derive(Default)]
pub(crate) struct CarouselWindow {
    pub open: bool,
    offsets: [f32; 4],
    clicked: Option<usize>,
}

/// Colours for the story's placeholder items.
const SWATCHES: [Color32; 6] = [
    Color32::from_rgb(0x6f, 0x8f, 0xaf),
    Color32::from_rgb(0xaf, 0x7f, 0x6f),
    Color32::from_rgb(0x7f, 0xaf, 0x7f),
    Color32::from_rgb(0xa0, 0x80, 0xb0),
    Color32::from_rgb(0xc0, 0xa0, 0x60),
    Color32::from_rgb(0x60, 0xa0, 0xa0),
];

impl CarouselWindow {
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Carousel Stories")
            .open(&mut open)
            .default_size([800.0, 760.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_intro(ui);
                    self.render_carousels(ui);
                });
            });
        self.open = open;
    }

    fn render_intro(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Carousel Stories");
            if ui
                .add(MaterialButton::filled("Material 3 Docs").small())
                .clicked()
            {
                // Opening a browser is best effort; the story works without it.
                let _ = webbrowser::open("https://m3.material.io/components/carousel/overview");
            }
        });
        ui.label(
            "Drag, or shift + scroll, to browse. Items snap one at a time (except \
             uncontained). Tab into a carousel and use the arrow keys, Home and End.",
        );
    }

    fn render_carousels(&mut self, ui: &mut egui::Ui) {
        let strategies = [
            (
                "Multi-browse",
                CarouselStrategy::MultiBrowse {
                    preferred_item_width: 186.0,
                },
            ),
            (
                "Uncontained",
                CarouselStrategy::Uncontained { item_width: 186.0 },
            ),
            (
                "Hero (centred)",
                CarouselStrategy::Hero {
                    preferred_item_width: None,
                    centered: true,
                },
            ),
            ("Full-screen", CarouselStrategy::FullScreen),
        ];
        for ((name, strategy), offset) in strategies.into_iter().zip(self.offsets.iter_mut()) {
            ui.add_space(16.0);
            ui.heading(name);
            let mut carousel = MaterialCarousel::new(offset)
                .strategy(strategy)
                .item_spacing(8.0)
                .height(200.0)
                .label(name)
                .id_salt(name);
            for (n, color) in SWATCHES.iter().cycle().take(12).enumerate() {
                let color = *color;
                carousel = carousel.labeled_item(format!("Item {n}"), move |ui, rect| {
                    ui.painter().rect_filled(ui.clip_rect(), 28.0, color);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{n}"),
                        egui::FontId::proportional(24.0),
                        Color32::WHITE,
                    );
                });
            }
            if let Some(clicked) = carousel.show(ui).clicked {
                self.clicked = Some(clicked);
            }
        }
        if let Some(clicked) = self.clicked {
            ui.add_space(8.0);
            ui.label(format!("Last clicked item: {clicked}"));
        }
    }
}
