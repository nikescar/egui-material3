#![doc(hidden)]

use crate::{
    noto_emoji, show_tooltip_on_hover, tooltip, MaterialButton, MaterialCard2, MaterialIconButton,
    TooltipPosition,
};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub struct TooltipWindow {
    pub open: bool,
}

impl Default for TooltipWindow {
    fn default() -> Self {
        Self { open: false }
    }
}

impl TooltipWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Tooltip Stories")
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

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Tooltip Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://framework7.io/vue/tooltip");
            }
        });

        ui.label("Hover over elements to see tooltips appear:");
    }

    fn render_examples(&mut self, ui: &mut Ui) {
        // Basic Tooltips on Buttons
        ui.heading("Basic Tooltips on Buttons");
        ui.label("Hover over buttons to see tooltips:");

        ui.horizontal_wrapped(|ui| {
            let button1 = ui.add(MaterialButton::filled("Top Tooltip"));
            show_tooltip_on_hover(ui, &button1, "This tooltip appears on top", TooltipPosition::Top);

            let button2 = ui.add(MaterialButton::filled("Bottom Tooltip"));
            show_tooltip_on_hover(ui, &button2, "This tooltip appears at the bottom", TooltipPosition::Bottom);

            let button3 = ui.add(MaterialButton::filled("Left Tooltip"));
            show_tooltip_on_hover(ui, &button3, "This tooltip appears on the left", TooltipPosition::Left);

            let button4 = ui.add(MaterialButton::filled("Right Tooltip"));
            show_tooltip_on_hover(ui, &button4, "This tooltip appears on the right", TooltipPosition::Right);

            let button5 = ui.add(MaterialButton::filled("Auto Tooltip"));
            show_tooltip_on_hover(ui, &button5, "This tooltip automatically finds the best position", TooltipPosition::Auto);
        });

        ui.add_space(20.0);

        // Different Button Styles
        ui.heading("Tooltips on Different Button Styles");
        ui.label("Tooltips work with all button variants:");

        ui.horizontal_wrapped(|ui| {
            let btn1 = ui.add(MaterialButton::text("Text Button"));
            show_tooltip_on_hover(ui, &btn1, "Text button with tooltip", TooltipPosition::Top);

            let btn2 = ui.add(MaterialButton::outlined("Outlined Button"));
            show_tooltip_on_hover(ui, &btn2, "Outlined button with tooltip", TooltipPosition::Top);

            let btn3 = ui.add(MaterialButton::filled_tonal("Tonal Button"));
            show_tooltip_on_hover(ui, &btn3, "Filled tonal button with tooltip", TooltipPosition::Top);

            let btn4 = ui.add(MaterialButton::elevated("Elevated Button"));
            show_tooltip_on_hover(ui, &btn4, "Elevated button with tooltip", TooltipPosition::Top);
        });

        ui.add_space(20.0);

        // Icon Buttons with Tooltips
        ui.heading("Tooltips on Icon Buttons");
        ui.label("Icon buttons benefit from tooltips to explain their function:");

        ui.horizontal_wrapped(|ui| {
            let icon1 = ui.add(MaterialIconButton::standard(noto_emoji::HOUSE_BUILDING).size(40.0));
            show_tooltip_on_hover(ui, &icon1, "Home", TooltipPosition::Bottom);

            let icon2 = ui.add(MaterialIconButton::standard(noto_emoji::RIGHT_POINTING_MAGNIFYING_GLASS).size(40.0));
            show_tooltip_on_hover(ui, &icon2, "Search", TooltipPosition::Bottom);

            let icon3 = ui.add(MaterialIconButton::standard(noto_emoji::GEAR).size(40.0));
            show_tooltip_on_hover(ui, &icon3, "Settings", TooltipPosition::Bottom);

            let icon4 = ui.add(MaterialIconButton::standard(noto_emoji::SPARKLING_HEART).size(40.0));
            show_tooltip_on_hover(ui, &icon4, "Favorite", TooltipPosition::Bottom);

            let icon5 = ui.add(MaterialIconButton::standard(noto_emoji::ARROW_POINTING_RIGHTWARDS_THEN_CURVING_UPWARDS).size(40.0));
            show_tooltip_on_hover(ui, &icon5, "Share", TooltipPosition::Bottom);

            let icon6 = ui.add(MaterialIconButton::standard(noto_emoji::WASTEBASKET).size(40.0));
            show_tooltip_on_hover(ui, &icon6, "Delete", TooltipPosition::Bottom);
        });

        ui.add_space(20.0);

        // Different Icon Button Styles
        ui.heading("Tooltips on Icon Button Variants");
        ui.label("All icon button variants support tooltips:");

        ui.horizontal_wrapped(|ui| {
            let icon1 = ui.add(MaterialIconButton::standard(noto_emoji::INFORMATION_SOURCE).size(40.0));
            show_tooltip_on_hover(ui, &icon1, "Standard icon button", TooltipPosition::Top);

            let icon2 = ui.add(MaterialIconButton::filled(noto_emoji::WHITE_HEAVY_CHECK_MARK).size(40.0));
            show_tooltip_on_hover(ui, &icon2, "Filled icon button", TooltipPosition::Top);

            let icon3 = ui.add(MaterialIconButton::filled_tonal(noto_emoji::BELL).size(40.0));
            show_tooltip_on_hover(ui, &icon3, "Filled tonal icon button", TooltipPosition::Top);

            let icon4 = ui.add(MaterialIconButton::outlined(noto_emoji::ENVELOPE).size(40.0));
            show_tooltip_on_hover(ui, &icon4, "Outlined icon button", TooltipPosition::Top);
        });

        ui.add_space(20.0);

        // Long Tooltip Text
        ui.heading("Long Tooltip Text");
        ui.label("Tooltips automatically wrap long text:");

        ui.horizontal_wrapped(|ui| {
            let btn = ui.add(MaterialButton::filled("Long Tooltip"));
            show_tooltip_on_hover(
                ui,
                &btn,
                "This is a much longer tooltip text that demonstrates how tooltips handle multiple lines of text. The tooltip will wrap the text to fit within the maximum width.",
                TooltipPosition::Top,
            );
        });

        ui.add_space(20.0);

        // Custom Styled Tooltips
        ui.heading("Custom Styled Tooltips");
        ui.label("Tooltips with custom styling:");

        ui.horizontal_wrapped(|ui| {
            let btn1 = ui.add(MaterialButton::filled("Large Font"));
            if btn1.hovered() {
                tooltip("Large font tooltip")
                    .font_size(16.0)
                    .position(TooltipPosition::Top)
                    .show(ui, btn1.rect);
            }

            let btn2 = ui.add(MaterialButton::filled("Wide Tooltip"));
            if btn2.hovered() {
                tooltip("This tooltip has a wider maximum width to accommodate longer text without wrapping")
                    .max_width(300.0)
                    .position(TooltipPosition::Top)
                    .show(ui, btn2.rect);
            }

            let btn3 = ui.add(MaterialButton::filled("Custom Padding"));
            if btn3.hovered() {
                tooltip("Custom padding")
                    .padding(egui::Vec2::new(16.0, 12.0))
                    .position(TooltipPosition::Top)
                    .show(ui, btn3.rect);
            }
        });

        ui.add_space(20.0);

        // Tooltips in a Grid Layout
        ui.heading("Tooltips in Grid Layout");
        ui.label("Tooltips work well in grid layouts:");

        egui::Grid::new("tooltip_grid")
            .num_columns(4)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                for row in 0..3 {
                    for col in 0..4 {
                        let button = ui.add(
                            MaterialButton::outlined(format!("R{}C{}", row + 1, col + 1))
                                .min_size(egui::Vec2::new(60.0, 40.0)),
                        );
                        show_tooltip_on_hover(
                            ui,
                            &button,
                            format!("Row {}, Column {}", row + 1, col + 1),
                            TooltipPosition::Top,
                        );
                    }
                    ui.end_row();
                }
            });

        ui.add_space(20.0);

        // Tooltips on Cards
        ui.heading("Tooltips on Cards");
        ui.label("Hover over cards to see tooltips:");

        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                let card_response = ui.add(
                    MaterialCard2::elevated()
                        .header("Card 1", None::<String>)
                        .content(|ui| {
                            ui.label("This card has a tooltip");
                        })
                        .min_size(egui::Vec2::new(150.0, 100.0)),
                );
                show_tooltip_on_hover(
                    ui,
                    &card_response,
                    "This is an elevated card",
                    TooltipPosition::Top,
                );
            });

            ui.vertical(|ui| {
                let card_response = ui.add(
                    MaterialCard2::filled()
                        .header("Card 2", None::<String>)
                        .content(|ui| {
                            ui.label("This card also has a tooltip");
                        })
                        .min_size(egui::Vec2::new(150.0, 100.0)),
                );
                show_tooltip_on_hover(
                    ui,
                    &card_response,
                    "This is a filled card",
                    TooltipPosition::Top,
                );
            });

            ui.vertical(|ui| {
                let card_response = ui.add(
                    MaterialCard2::outlined()
                        .header("Card 3", None::<String>)
                        .content(|ui| {
                            ui.label("And this one too");
                        })
                        .min_size(egui::Vec2::new(150.0, 100.0)),
                );
                show_tooltip_on_hover(
                    ui,
                    &card_response,
                    "This is an outlined card",
                    TooltipPosition::Top,
                );
            });
        });

        ui.add_space(20.0);

        // Action Buttons with Tooltips
        ui.heading("Action Buttons with Descriptive Tooltips");
        ui.label("Tooltips provide helpful context for actions:");

        ui.horizontal_wrapped(|ui| {
            let save_btn = ui.add(MaterialButton::filled("Save").min_size(egui::Vec2::new(80.0, 36.0)));
            show_tooltip_on_hover(
                ui,
                &save_btn,
                "Save your current changes",
                TooltipPosition::Top,
            );

            let cancel_btn = ui.add(MaterialButton::outlined("Cancel").min_size(egui::Vec2::new(80.0, 36.0)));
            show_tooltip_on_hover(
                ui,
                &cancel_btn,
                "Cancel and discard changes",
                TooltipPosition::Top,
            );

            let delete_btn = ui.add(MaterialButton::text("Delete").min_size(egui::Vec2::new(80.0, 36.0)));
            show_tooltip_on_hover(
                ui,
                &delete_btn,
                "Permanently delete this item",
                TooltipPosition::Top,
            );
        });

        ui.add_space(20.0);

        // Position Demonstration
        ui.heading("Position Options");
        ui.label("Tooltips can be positioned in different directions:");

        ui.vertical_centered(|ui| {
            ui.add_space(50.0);

            // Top
            let top_btn = ui.add(MaterialButton::outlined("Top"));
            show_tooltip_on_hover(ui, &top_btn, "Tooltip on top", TooltipPosition::Top);

            ui.add_space(20.0);

            // Left, Center, Right
            ui.horizontal(|ui| {
                ui.add_space(100.0);

                let left_btn = ui.add(MaterialButton::outlined("Left"));
                show_tooltip_on_hover(ui, &left_btn, "Tooltip on left", TooltipPosition::Left);

                ui.add_space(50.0);

                let auto_btn = ui.add(MaterialButton::outlined("Auto"));
                show_tooltip_on_hover(ui, &auto_btn, "Auto-positioned tooltip", TooltipPosition::Auto);

                ui.add_space(50.0);

                let right_btn = ui.add(MaterialButton::outlined("Right"));
                show_tooltip_on_hover(ui, &right_btn, "Tooltip on right", TooltipPosition::Right);
            });

            ui.add_space(20.0);

            // Bottom
            let bottom_btn = ui.add(MaterialButton::outlined("Bottom"));
            show_tooltip_on_hover(ui, &bottom_btn, "Tooltip on bottom", TooltipPosition::Bottom);

            ui.add_space(50.0);
        });
    }
}
