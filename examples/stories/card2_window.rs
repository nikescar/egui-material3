// ============================================================================
// Module:       stories::card2_window (example)
// Description:  Story window demonstrating elevated, filled and outlined
//               cards: clickable, selectable, disabled, with media.
//
// Dependencies: eframe, egui; crate::{MaterialCard2, Card2Variant}
// ============================================================================

//! Story window demonstrating the card component.

#![doc(hidden)]

use crate::{Card2Variant, MaterialButton, MaterialCard2};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub(crate) struct Card2Window {
    pub open: bool,
    show_subtitle: bool,
    show_media: bool,
    show_content: bool,
    show_actions: bool,
    clickable: bool,
    disabled: bool,
    selected: [bool; 3],
    clicks: usize,
}

impl Default for Card2Window {
    fn default() -> Self {
        Self {
            open: false,
            show_subtitle: true,
            show_media: true,
            show_content: true,
            show_actions: true,
            clickable: true,
            disabled: false,
            selected: [false, true, false],
            clicks: 0,
        }
    }
}

impl Card2Window {
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Card Stories")
            .open(&mut open)
            .default_size([960.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(16.0);
                    self.render_variants(ui);
                    ui.add_space(16.0);
                    self.render_selectable(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Cards");
            if ui
                .add(MaterialButton::filled("Material 3 Docs").small())
                .clicked()
            {
                // Opening a browser is best effort; the story works without it.
                let _ = webbrowser::open("https://m3.material.io/components/cards/overview");
            }
        });
        ui.label(
            "Hover a clickable card: elevated rises from Level1 to Level2, filled and \
             outlined from Level0 to Level1. Tab focuses cards; Space or Enter activates them.",
        );
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.show_subtitle, "Subtitle");
            ui.checkbox(&mut self.show_media, "Media");
            ui.checkbox(&mut self.show_content, "Content");
            ui.checkbox(&mut self.show_actions, "Actions");
            ui.checkbox(&mut self.clickable, "Clickable");
            ui.checkbox(&mut self.disabled, "Disabled");
        });
        ui.label(format!("Card clicks: {}", self.clicks));
    }

    fn card(&self, variant: Card2Variant) -> MaterialCard2<'static> {
        let subtitle = self.show_subtitle.then_some("Supporting text");
        let mut card = MaterialCard2::new_with_variant(variant)
            .header(format!("{variant:?} card"), subtitle)
            .width(280.0)
            .media_height(120.0)
            .clickable(self.clickable)
            .enabled(!self.disabled);
        if self.show_media {
            card = card.media_area(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Media");
                });
            });
        }
        if self.show_content {
            card = card.content(|ui| {
                ui.label("Cards hold content and actions about a single subject.");
            });
        }
        if self.show_actions {
            card = card.actions(|ui| {
                ui.add(MaterialButton::filled("Open"));
                ui.add(MaterialButton::text("Share"));
            });
        }
        card
    }

    fn render_variants(&mut self, ui: &mut Ui) {
        ui.heading("Variants");
        ui.horizontal_top(|ui| {
            for variant in Card2Variant::ALL {
                if ui.add(self.card(variant)).clicked() {
                    self.clicks += 1;
                }
            }
        });
    }

    fn render_selectable(&mut self, ui: &mut Ui) {
        ui.heading("Selectable cards");
        ui.label("Click, or focus and press Space, to toggle. Selected cards show a check.");
        let disabled = self.disabled;
        ui.horizontal_top(|ui| {
            for (variant, selected) in Card2Variant::ALL.into_iter().zip(self.selected.iter_mut()) {
                ui.add(
                    MaterialCard2::new_with_variant(variant)
                        .header(format!("{variant:?}"), Some("Selectable"))
                        .content(|ui| {
                            ui.label("Pick as many as you like.");
                        })
                        .width(280.0)
                        .enabled(!disabled)
                        .checked(selected),
                );
            }
        });
    }
}
