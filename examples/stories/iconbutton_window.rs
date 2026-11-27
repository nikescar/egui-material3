// ============================================================================
// Module:       stories::iconbutton_window (example)
// Description:  Icon button stories: styles, sizes, widths, shapes and
//               toggles.
//
// Dependencies: eframe, egui_material3, webbrowser
// ============================================================================

//! Icon button stories: every style at every size, the three widths, round
//! and square shapes, toggle buttons of every style, and disabled buttons.

#![doc(hidden)]

use crate::{
    IconButtonShape, IconButtonSize, IconButtonVariant, IconButtonWidth, MaterialButton,
    MaterialCheckbox, MaterialIconButton,
};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub(crate) struct IconButtonWindow {
    pub(crate) open: bool,
    enabled: bool,
    square: bool,
    size: IconButtonSize,
    width: IconButtonWidth,
    icon: String,
    toggles: [bool; 4],
}

impl Default for IconButtonWindow {
    fn default() -> Self {
        Self {
            open: false,
            enabled: true,
            square: false,
            size: IconButtonSize::Small,
            width: IconButtonWidth::Default,
            icon: "favorite".to_owned(),
            toggles: [false, true, false, true],
        }
    }
}

fn variant_name(variant: IconButtonVariant) -> &'static str {
    match variant {
        IconButtonVariant::Standard => "Standard",
        IconButtonVariant::Filled => "Filled",
        IconButtonVariant::FilledTonal => "Filled tonal",
        IconButtonVariant::Outlined => "Outlined",
    }
}

fn size_name(size: IconButtonSize) -> String {
    match size {
        IconButtonSize::ExtraSmall => "XS".to_owned(),
        IconButtonSize::Small => "S".to_owned(),
        IconButtonSize::Medium => "M".to_owned(),
        IconButtonSize::Large => "L".to_owned(),
        IconButtonSize::ExtraLarge => "XL".to_owned(),
        IconButtonSize::Custom(side) => format!("{side}"),
    }
}

impl IconButtonWindow {
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Icon Button Stories")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(16.0);
                    self.render_styles(ui);
                    ui.add_space(16.0);
                    self.render_sizes(ui);
                    ui.add_space(16.0);
                    self.render_toggles(ui);
                });
            });
        self.open = open;
    }

    fn shape(&self) -> IconButtonShape {
        if self.square {
            IconButtonShape::Square
        } else {
            IconButtonShape::Round
        }
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Icon Button Controls");
            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                // Opening the spec in a browser is a convenience; a failure
                // leaves nothing to do.
                let _ = webbrowser::open("https://m3.material.io/components/icon-buttons/specs");
            }
        });
        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.enabled, "Enabled"));
            ui.add(MaterialCheckbox::new(&mut self.square, "Square"));
        });
        ui.horizontal(|ui| {
            ui.label("Size:");
            for size in IconButtonSize::TOKEN_SIZES {
                ui.selectable_value(&mut self.size, size, size_name(size));
            }
            ui.separator();
            ui.label("Width:");
            ui.selectable_value(&mut self.width, IconButtonWidth::Narrow, "Narrow");
            ui.selectable_value(&mut self.width, IconButtonWidth::Default, "Default");
            ui.selectable_value(&mut self.width, IconButtonWidth::Wide, "Wide");
        });
        ui.horizontal(|ui| {
            ui.label("Icon:");
            ui.text_edit_singleline(&mut self.icon);
            ui.label("(Material Symbols name)");
        });
    }

    fn button(&self, variant: IconButtonVariant) -> MaterialIconButton<'static> {
        MaterialIconButton::new(self.icon.as_str(), variant)
            .size(self.size)
            .width(self.width)
            .shape(self.shape())
            .enabled(self.enabled)
            .label(variant_name(variant))
    }

    fn render_styles(&mut self, ui: &mut Ui) {
        ui.heading("Styles");
        ui.horizontal(|ui| {
            for variant in IconButtonVariant::ALL {
                ui.vertical(|ui| {
                    ui.label(variant_name(variant));
                    ui.add(self.button(variant));
                    ui.add(self.button(variant).enabled(false));
                });
            }
        });
    }

    fn render_sizes(&mut self, ui: &mut Ui) {
        ui.heading("Sizes and widths");
        for variant in IconButtonVariant::ALL {
            ui.label(variant_name(variant));
            ui.horizontal(|ui| {
                for size in IconButtonSize::TOKEN_SIZES {
                    for width in IconButtonWidth::ALL {
                        ui.add(
                            MaterialIconButton::new(self.icon.as_str(), variant)
                                .size(size)
                                .width(width)
                                .shape(self.shape())
                                .label(format!("{} {}", variant_name(variant), size_name(size))),
                        );
                    }
                }
            });
        }
    }

    fn render_toggles(&mut self, ui: &mut Ui) {
        ui.heading("Toggle buttons");
        let (size, width, shape, enabled) = (self.size, self.width, self.shape(), self.enabled);
        let icon = self.icon.clone();
        ui.horizontal(|ui| {
            for (variant, on) in IconButtonVariant::ALL
                .into_iter()
                .zip(self.toggles.iter_mut())
            {
                ui.vertical(|ui| {
                    ui.label(format!(
                        "{} ({})",
                        variant_name(variant),
                        if *on { "on" } else { "off" }
                    ));
                    ui.add(
                        MaterialIconButton::new(icon.as_str(), variant)
                            .toggleable(on)
                            .size(size)
                            .width(width)
                            .shape(shape)
                            .enabled(enabled)
                            .label(variant_name(variant)),
                    );
                });
            }
        });
    }
}
