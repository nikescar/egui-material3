// ============================================================================
// Module:       stories::fab_window (example)
// Description:  FAB stories: colour schemes, sizes, extended FABs that
//               collapse, the FAB menu, and a brand-coloured FAB.
//
// Dependencies: eframe, egui_material3, webbrowser
// ============================================================================

//! FAB stories: every colour scheme at every size, lowered FABs, extended
//! FABs of three sizes that expand and collapse, the FAB menu in its three
//! colour schemes, and a brand-coloured FAB built from the public API
//! (custom colours and a multi-colour SVG icon) instead of a library
//! variant.

#![doc(hidden)]

use crate::{
    FabMenu, FabMenuColors, FabSize, FabVariant, Icon, MaterialButton, MaterialCheckbox,
    MaterialFab,
};
use eframe::egui::{self, Color32, Ui, Window};

/// A four-colour mark, drawn by the SVG itself so it keeps its colours.
const BRAND_MARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36">
<path d="M16 16v14h4V20z" fill="#34A853"/>
<path d="M30 16H20l-4 4h14z" fill="#4285F4"/>
<path d="M6 16v4h10l4-4z" fill="#FBBC05"/>
<path d="M20 16V6h-4v14z" fill="#EA4335"/>
</svg>"##;

#[doc(hidden)]
pub(crate) struct FabWindow {
    pub(crate) open: bool,
    icon: String,
    label: String,
    lowered: bool,
    size: FabSize,
    expanded: bool,
    menus_open: [bool; 3],
    last_menu_choice: String,
}

impl Default for FabWindow {
    fn default() -> Self {
        Self {
            open: false,
            icon: "edit".to_owned(),
            label: "Compose".to_owned(),
            lowered: false,
            size: FabSize::Regular,
            expanded: true,
            menus_open: [false; 3],
            last_menu_choice: String::new(),
        }
    }
}

#[expect(deprecated, reason = "names the deprecated Branded variant")]
fn variant_name(variant: FabVariant) -> &'static str {
    match variant {
        FabVariant::Surface => "Surface",
        FabVariant::Primary => "Primary",
        FabVariant::Secondary => "Secondary",
        FabVariant::Tertiary => "Tertiary",
        FabVariant::PrimaryContainer => "Primary container",
        FabVariant::SecondaryContainer => "Secondary container",
        FabVariant::TertiaryContainer => "Tertiary container",
        FabVariant::Branded => "Branded (deprecated)",
    }
}

fn size_name(size: FabSize) -> &'static str {
    match size {
        FabSize::Small => "Small",
        FabSize::Regular | FabSize::Extended => "Regular",
        FabSize::Medium => "Medium",
        FabSize::Large => "Large",
    }
}

impl FabWindow {
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("FAB Stories")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(16.0);
                    self.render_variants(ui);
                    ui.add_space(16.0);
                    self.render_extended(ui);
                    ui.add_space(16.0);
                    self.render_menus(ui);
                    ui.add_space(16.0);
                    self.render_brand(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("FAB Controls");
            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                // Opening the spec in a browser is a convenience; a failure
                // leaves nothing to do.
                let _ = webbrowser::open(
                    "https://m3.material.io/components/floating-action-button/specs",
                );
            }
        });
        ui.horizontal(|ui| {
            ui.label("Icon:");
            ui.text_edit_singleline(&mut self.icon);
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);
        });
        ui.horizontal(|ui| {
            ui.add(MaterialCheckbox::new(&mut self.lowered, "Lowered"));
            ui.add(MaterialCheckbox::new(
                &mut self.expanded,
                "Extended FABs expanded",
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Size:");
            for size in FabSize::ALL {
                ui.selectable_value(&mut self.size, size, size_name(size));
            }
        });
    }

    fn render_variants(&mut self, ui: &mut Ui) {
        ui.heading("Colour schemes");
        ui.horizontal_wrapped(|ui| {
            for variant in FabVariant::ALL {
                ui.vertical(|ui| {
                    ui.label(variant_name(variant));
                    ui.add(
                        MaterialFab::new(variant)
                            .icon(self.icon.as_str())
                            .size(self.size)
                            .lowered(self.lowered)
                            .label(variant_name(variant)),
                    );
                });
                ui.add_space(8.0);
            }
        });
        ui.heading("Sizes");
        ui.horizontal(|ui| {
            for size in FabSize::ALL {
                ui.add(
                    MaterialFab::primary_container()
                        .icon(self.icon.as_str())
                        .size(size)
                        .label(size_name(size)),
                );
                ui.add_space(8.0);
            }
        });
    }

    fn render_extended(&mut self, ui: &mut Ui) {
        ui.heading("Extended FABs");
        for size in [FabSize::Regular, FabSize::Medium, FabSize::Large] {
            ui.add(
                MaterialFab::primary_container()
                    .icon(self.icon.as_str())
                    .text(self.label.as_str())
                    .size(size)
                    .lowered(self.lowered)
                    .expanded(self.expanded),
            );
            ui.add_space(8.0);
        }
    }

    fn render_menus(&mut self, ui: &mut Ui) {
        ui.heading("FAB menu");
        ui.label("Click a button to open its menu; Escape closes it.");
        // Leave room above the buttons for the open items.
        ui.add_space(220.0);
        ui.horizontal(|ui| {
            for (colors, open) in FabMenuColors::ALL
                .into_iter()
                .zip(self.menus_open.iter_mut())
            {
                let name = match colors {
                    FabMenuColors::Primary => "primary",
                    FabMenuColors::Secondary => "secondary",
                    FabMenuColors::Tertiary => "tertiary",
                };
                let response = FabMenu::new(open)
                    .id_salt(name)
                    .colors(colors)
                    .label(format!("{name} menu"))
                    .item("reply", "Reply")
                    .item("reply_all", "Reply all")
                    .item("forward", "Forward")
                    .show(ui);
                if let Some(index) = response.clicked {
                    self.last_menu_choice = format!("{name} menu: item {index}");
                }
                ui.add_space(220.0);
            }
        });
        if !self.last_menu_choice.is_empty() {
            ui.label(format!("Last choice: {}", self.last_menu_choice));
        }
    }

    fn render_brand(&mut self, ui: &mut Ui) {
        ui.heading("Brand colours");
        ui.label(
            "A FAB in colours outside the theme: a white container and a \
             multi-colour SVG mark, through container_color and an SVG icon.",
        );
        ui.horizontal(|ui| {
            ui.add(
                MaterialFab::surface()
                    .container_color(Color32::WHITE)
                    .content_color(Color32::from_rgb(0x1F, 0x1F, 0x1F))
                    .icon(Icon::svg(BRAND_MARK_SVG.as_bytes().to_vec()))
                    .size(self.size)
                    .label("Brand"),
            );
            ui.add_space(8.0);
            ui.add(
                MaterialFab::surface()
                    .container_color(Color32::WHITE)
                    .content_color(Color32::from_rgb(0x1F, 0x1F, 0x1F))
                    .icon(Icon::svg(BRAND_MARK_SVG.as_bytes().to_vec()))
                    .text("Continue")
                    .label("Continue"),
            );
        });
    }
}
