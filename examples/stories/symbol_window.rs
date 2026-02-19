#![doc(hidden)]

use crate::{
    assist_chip, fab_primary, fab_secondary, filter_chip, icon, icon_button_filled,
    icon_button_filled_tonal, icon_button_outlined, icon_button_standard, input_chip, list,
    list_item, suggestion_chip, MaterialButton,
};
use egui_material3::material_symbol::{
    ICON_ADD, ICON_BOOKMARK, ICON_CHECK, ICON_DELETE, ICON_EDIT, ICON_FAVORITE, ICON_HOME,
    ICON_MUSIC_NOTE, ICON_NOTIFICATIONS, ICON_PERSON, ICON_SEARCH, ICON_SETTINGS, ICON_SHARE,
    ICON_SHOPPING_CART, ICON_STAR,
};
use egui_material3::noto_emoji;
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct SymbolWindow {
    pub open: bool,
    filter_selected_1: bool,
    filter_selected_2: bool,
    filter_selected_3: bool,
    filter_selected_4: bool,
}

impl Default for SymbolWindow {
    fn default() -> Self {
        Self {
            open: false,
            filter_selected_1: false,
            filter_selected_2: true,
            filter_selected_3: false,
            filter_selected_4: true,
        }
    }
}

impl SymbolWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Symbol Stories")
            .open(&mut open)
            .default_size([800.0, 900.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    Self::render_loaded_fonts(ui);
                    ui.add_space(20.0);
                    self.render_material_symbol_basic(ui);
                    ui.add_space(20.0);
                    self.render_material_symbol_with_components(ui);
                    ui.add_space(20.0);
                    self.render_noto_emoji_basic(ui);
                    ui.add_space(20.0);
                    self.render_noto_emoji_with_components(ui);
                });
            });
        self.open = open;
    }

    fn render_loaded_fonts(ui: &mut egui::Ui) {
        ui.heading("Loaded Fonts");
        ui.add_space(4.0);

        let families = ui.ctx().fonts(|fonts| {
            fonts.families()
        });
        
        // Note: In egui 0.33, glyph checking requires mutable access which isn't available in the fonts closure
        // For now, we'll assume fonts are available if they're in the family list
        let has_material_symbols = families.iter().any(|f| f.to_string().contains("MaterialSymbols"));
        let has_noto_emoji = families.iter().any(|f| f.to_string().contains("NotoEmoji"));

        // Display status
        ui.horizontal(|ui| {
            ui.label("Material Symbols:");
            if has_material_symbols {
                ui.colored_label(egui::Color32::from_rgb(0, 180, 0), "✓ Available");
            } else {
                ui.colored_label(egui::Color32::from_rgb(200, 0, 0), "✗ Not found");
            }
            ui.add_space(16.0);
            ui.label("Noto Emoji:");
            if has_noto_emoji {
                ui.colored_label(egui::Color32::from_rgb(0, 180, 0), "✓ Available");
            } else {
                ui.colored_label(egui::Color32::from_rgb(200, 150, 0), "⚠ Using fallback");
            }
        });

        ui.add_space(4.0);

        // Show font families
        egui::CollapsingHeader::new(format!("Font Families ({} families)", families.len()))
            .default_open(false)
            .show(ui, |ui| {
                for family in &families {
                    ui.label(format!("  {:?}", family));
                }
            });
    }

    fn render_material_symbol_basic(&self, ui: &mut egui::Ui) {
        

        ui.heading("Material Symbol Icons");
        ui.label("Using constants from material_symbol module (char type, rendered via MaterialSymbolsOutlined font)");
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label("home");
                ui.add(icon(ICON_HOME.to_string()).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("search");
                ui.add(icon(ICON_SEARCH.to_string()).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("settings");
                ui.add(icon(ICON_SETTINGS.to_string()).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("favorite");
                ui.add(icon(ICON_FAVORITE.to_string()).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("person");
                ui.add(icon(ICON_PERSON.to_string()).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("notifications");
                ui.add(icon(ICON_NOTIFICATIONS.to_string()).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("star");
                ui.add(icon(ICON_STAR.to_string()).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("delete");
                ui.add(icon(ICON_DELETE.to_string()).size(24.0));
            });
        });

        ui.add_space(12.0);
        ui.label("Different sizes:");
        ui.horizontal(|ui| {
            ui.add(icon(ICON_FAVORITE.to_string()).size(16.0));
            ui.label("16px");
            ui.add(icon(ICON_FAVORITE.to_string()).size(24.0));
            ui.label("24px");
            ui.add(icon(ICON_FAVORITE.to_string()).size(32.0));
            ui.label("32px");
            ui.add(icon(ICON_FAVORITE.to_string()).size(48.0));
            ui.label("48px");
        });
    }

    fn render_material_symbol_with_components(&mut self, ui: &mut egui::Ui) {
        ui.heading("Material Symbols with Components");
        ui.add_space(8.0);

        // Buttons with material symbol icons
        // Note: MaterialButton.leading_icon() expects icon name strings (e.g., "favorite"),
        // not Unicode char constants, because it uses material_symbol_text() internally.
        ui.label("Buttons:");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                MaterialButton::filled("Favorite")
                    .leading_icon("favorite"),
            );
            ui.add(
                MaterialButton::outlined("Search")
                    .leading_icon("search"),
            );
            ui.add(
                MaterialButton::elevated("Settings")
                    .leading_icon("settings"),
            );
            ui.add(
                MaterialButton::text("Share")
                    .leading_icon("share"),
            );
        });

        ui.add_space(12.0);

        // Chips with material symbol icons (render icon string directly, use char constants)
        ui.label("Chips:");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                assist_chip("Home")
                    .leading_icon(ICON_HOME.to_string())
                    .on_click(|| println!("Home chip clicked")),
            );
            ui.add_space(4.0);
            ui.add(filter_chip("Bookmarks", &mut self.filter_selected_1)
                .leading_icon(ICON_BOOKMARK.to_string()));
            ui.add_space(4.0);
            ui.add(
                input_chip("Shopping")
                    .leading_icon(ICON_SHOPPING_CART.to_string())
                    .removable(true)
                    .on_click(|| println!("Shopping chip clicked")),
            );
            ui.add_space(4.0);
            ui.add(
                suggestion_chip("Edit")
                    .leading_icon(ICON_EDIT.to_string())
                    .on_click(|| println!("Edit chip clicked")),
            );
        });

        ui.add_space(12.0);

        // FABs with material symbol icons
        ui.label("FABs:");
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.add(
                    fab_primary()
                        .icon("add")
                        .on_click(|| println!("Add FAB clicked")),
                );
            });
            ui.vertical(|ui| {
                ui.add(
                    fab_secondary()
                        .icon("edit")
                        .text("Edit")
                        .on_click(|| println!("Edit FAB clicked")),
                );
            });
        });

        ui.add_space(12.0);

        // Icon buttons with material symbol icons (render icon string directly, use char constants)
        ui.label("Icon Buttons:");
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(icon_button_standard(ICON_FAVORITE.to_string()))
                .clicked()
            {
                println!("Standard icon button clicked");
            }
            if ui
                .add(icon_button_filled(ICON_ADD.to_string()))
                .clicked()
            {
                println!("Filled icon button clicked");
            }
            if ui
                .add(icon_button_filled_tonal(ICON_BOOKMARK.to_string()))
                .clicked()
            {
                println!("Filled tonal icon button clicked");
            }
            if ui
                .add(icon_button_outlined(ICON_SHARE.to_string()))
                .clicked()
            {
                println!("Outlined icon button clicked");
            }
        });

        ui.add_space(12.0);

        // List with material symbol icons
        ui.label("List:");
        let symbol_list = list()
            .id("material_symbol_list")
            .item(
                list_item("Home")
                    .leading_icon("home")
                    .trailing_icon("notifications")
                    .on_click(|| println!("Home item clicked")),
            )
            .item(
                list_item("Favorites")
                    .secondary_text("Your saved items")
                    .leading_icon("favorite")
                    .trailing_icon("star")
                    .on_click(|| println!("Favorites item clicked")),
            )
            .item(
                list_item("Settings")
                    .secondary_text("App configuration\nAdvanced options")
                    .leading_icon("settings")
                    .trailing_icon("person")
                    .on_click(|| println!("Settings item clicked")),
            );
        ui.add(symbol_list);
    }

    fn render_noto_emoji_basic(&self, ui: &mut egui::Ui) {
        ui.heading("Noto Emoji Icons");
        ui.label("Using constants from noto_emoji module (&str type, rendered via Noto Emoji font)");
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label("heart");
                ui.add(icon(noto_emoji::HEAVY_BLACK_HEART).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("star");
                ui.add(icon(noto_emoji::WHITE_MEDIUM_STAR).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("sparkles");
                ui.add(icon(noto_emoji::SPARKLES).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("sun");
                ui.add(icon(noto_emoji::BLACK_SUN_WITH_RAYS).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("cloud");
                ui.add(icon(noto_emoji::CLOUD).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("umbrella");
                ui.add(icon(noto_emoji::UMBRELLA).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("snowman");
                ui.add(icon(noto_emoji::SNOWMAN).size(24.0));
            });
            ui.vertical(|ui| {
                ui.label("coffee");
                ui.add(icon(noto_emoji::HOT_BEVERAGE).size(24.0));
            });
        });

        ui.add_space(12.0);
        ui.label("Different sizes:");
        ui.horizontal(|ui| {
            ui.add(icon(noto_emoji::HEAVY_BLACK_HEART).size(16.0));
            ui.label("16px");
            ui.add(icon(noto_emoji::HEAVY_BLACK_HEART).size(24.0));
            ui.label("24px");
            ui.add(icon(noto_emoji::HEAVY_BLACK_HEART).size(32.0));
            ui.label("32px");
            ui.add(icon(noto_emoji::HEAVY_BLACK_HEART).size(48.0));
            ui.label("48px");
        });
    }

    fn render_noto_emoji_with_components(&mut self, ui: &mut egui::Ui) {
        ui.heading("Noto Emoji with Components");
        ui.add_space(8.0);

        // Note: MaterialButton.leading_icon(), fab.icon(), and list_item.leading_icon()
        // use material_symbol_text() internally (name-based lookup), so they don't support
        // noto emoji directly. Chips and IconButtons render the icon string directly,
        // so they work with noto emoji.

        // Chips with noto emoji (renders icon string directly)
        ui.label("Chips:");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                assist_chip("Weather")
                    .leading_icon(noto_emoji::BLACK_SUN_WITH_RAYS)
                    .on_click(|| println!("Weather chip clicked")),
            );
            ui.add_space(4.0);
            ui.add(filter_chip("Favorites", &mut self.filter_selected_3)
                .leading_icon(noto_emoji::WHITE_MEDIUM_STAR));
            ui.add_space(4.0);
            ui.add(
                input_chip("Hot")
                    .leading_icon(noto_emoji::HOT_BEVERAGE)
                    .removable(true)
                    .on_click(|| println!("Hot chip clicked")),
            );
            ui.add_space(4.0);
            ui.add(
                suggestion_chip("Magic")
                    .leading_icon(noto_emoji::SPARKLES)
                    .on_click(|| println!("Magic chip clicked")),
            );
        });

        ui.add_space(12.0);

        // Icon buttons with noto emoji (renders icon string directly)
        ui.label("Icon Buttons:");
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(icon_button_standard(noto_emoji::HEAVY_BLACK_HEART))
                .clicked()
            {
                println!("Heart icon button clicked");
            }
            if ui
                .add(icon_button_filled(noto_emoji::WHITE_MEDIUM_STAR))
                .clicked()
            {
                println!("Star icon button clicked");
            }
            if ui
                .add(icon_button_filled_tonal(noto_emoji::SPARKLES))
                .clicked()
            {
                println!("Sparkles icon button clicked");
            }
            if ui
                .add(icon_button_outlined(noto_emoji::BLACK_SUN_WITH_RAYS))
                .clicked()
            {
                println!("Sun icon button clicked");
            }
        });
    }
}
