// ============================================================================
// Module:       stories::badge_window (example)
// Description:  Story window demonstrating the badge component.
//
// Dependencies: eframe, egui
// ============================================================================

//! Story window demonstrating the badge component.

#![doc(hidden)]

use crate::{
    badge, badge_dot, noto_emoji, BadgeColor, BadgePosition, FabSize, MaterialBadge,
    MaterialButton, MaterialFab, MaterialIconButton,
};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub(crate) struct BadgeWindow {
    pub open: bool,
    selected_color: BadgeColor,
    badge_content: String,
    show_as_dot: bool,
    count: u32,
}

impl Default for BadgeWindow {
    fn default() -> Self {
        Self {
            open: false,
            selected_color: BadgeColor::Error,
            badge_content: "5".to_string(),
            show_as_dot: false,
            count: 5,
        }
    }
}

impl BadgeWindow {
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Badge Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Badge Controls");
            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                // Opening the reference page is best-effort.
                let _ = webbrowser::open("https://m3.material.io/components/badges/specs");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Badge text:");
            ui.text_edit_singleline(&mut self.badge_content);
        });
        ui.checkbox(&mut self.show_as_dot, "Show as dot");
        ui.horizontal(|ui| {
            ui.label("Colour:");
            ui.selectable_value(&mut self.selected_color, BadgeColor::Error, "Error (M3)");
            ui.selectable_value(&mut self.selected_color, BadgeColor::Primary, "Primary");
        });
        ui.horizontal(|ui| {
            ui.label("Count:");
            ui.add(egui::Slider::new(&mut self.count, 0..=1500));
        });
    }

    fn render_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Standalone");
        ui.horizontal(|ui| {
            ui.add(badge_dot());
            ui.add(badge("5"));
            ui.add(badge("NEW"));
            ui.add(MaterialBadge::count(self.count));
            ui.add(MaterialBadge::count(1000));
            let custom = badge(&self.badge_content)
                .as_dot(self.show_as_dot)
                .color(self.selected_color);
            ui.add(custom);
        });

        ui.add_space(20.0);
        ui.heading("On icon buttons");
        ui.label(
            "Large badges start 12 in from the anchor's end and rise 2 above it; dots start 6 in.",
        );
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 24.0;
            let inbox = ui.add(MaterialIconButton::standard(noto_emoji::ENVELOPE).size(48.0));
            MaterialBadge::count(self.count)
                .color(self.selected_color)
                .description(format!("{} new messages", self.count))
                .show_on(ui, &inbox, BadgePosition::TopRight);

            let calendar = ui.add(MaterialIconButton::filled(noto_emoji::CALENDAR).size(48.0));
            badge("7").show_on(ui, &calendar, BadgePosition::TopRight);

            let bell = ui.add(MaterialIconButton::filled_tonal(noto_emoji::BELL).size(48.0));
            badge_dot().show_on(ui, &bell, BadgePosition::TopRight);
        });

        ui.add_space(20.0);
        ui.heading("Positions");
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 24.0;
            for (salt, position) in [
                ("tr", BadgePosition::TopRight),
                ("tl", BadgePosition::TopLeft),
                ("br", BadgePosition::BottomRight),
                ("bl", BadgePosition::BottomLeft),
            ] {
                let anchor = ui.add(MaterialIconButton::outlined(noto_emoji::BELL).size(48.0));
                badge("3").id_salt(salt).show_on(ui, &anchor, position);
            }
        });

        ui.add_space(20.0);
        ui.heading("On FABs");
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 24.0;
            let fab = ui.add(MaterialFab::primary().icon("add").size(FabSize::Regular));
            badge("3").show_on(ui, &fab, BadgePosition::TopRight);
            let fab = ui.add(MaterialFab::secondary().icon("edit").size(FabSize::Small));
            badge_dot().show_on(ui, &fab, BadgePosition::TopRight);
        });
    }
}
