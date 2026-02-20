/// Example demonstrating SVG icon usage with Material Design components
///
/// This example shows how to use SVG icons from the svg_emoji module
/// with various Material Design 3 components.

use eframe::egui;
use egui_material3::{
    // Chips
    assist_chip, filter_chip, input_chip, suggestion_chip,
    // Icon Buttons
    icon_button_filled, icon_button_filled_tonal, icon_button_outlined, icon_button_standard,
    // Buttons
    MaterialButton,
    // SVG emoji collections
    svg_emoji::{SOLAR_ICONS, NOTO_EMOJIS, TWEMOJI},
    // Theme functions
    theme::{load_fonts, load_themes, setup_google_fonts, setup_local_fonts, setup_local_theme, update_window_background},
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("SVG Icon Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "SVG Icon Demo",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and theme
            setup_google_fonts(Some("Roboto"));
            setup_local_fonts(Some("resources/MaterialSymbolsOutlined.ttf"));
            setup_local_theme(None);
            load_fonts(&cc.egui_ctx);
            load_themes();
            update_window_background(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    filter_selected: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SVG Icon Usage Examples");
            ui.add_space(16.0);

            // ══════════════════════════════════════════════════════════════════
            // Example 1: Using Solar Icons (UI/UX icons)
            // ══════════════════════════════════════════════════════════════════
            ui.heading("1. Solar Icons (UI/UX Icons)");
            ui.label("Solar Icons are perfect for UI controls - ~1200 icons available");
            ui.add_space(8.0);

            if let Some(&home_svg) = SOLAR_ICONS.get("home") {
                ui.label("Chips with 'home' icon:");
                ui.horizontal(|ui| {
                    ui.add(assist_chip("Home").leading_icon_svg(home_svg).on_click(|| {
                        println!("Home clicked!");
                    }));

                    ui.add(filter_chip("Home", &mut self.filter_selected).leading_icon_svg(home_svg));

                    ui.add(input_chip("Home")
                        .leading_icon_svg(home_svg)
                        .removable(true)
                        .on_click(|| {
                            println!("Input chip clicked!");
                        }));

                    ui.add(suggestion_chip("Home").leading_icon_svg(home_svg).on_click(|| {
                        println!("Suggestion clicked!");
                    }));
                });
            }

            ui.add_space(8.0);

            if let Some(&settings_svg) = SOLAR_ICONS.get("settings") {
                ui.label("Icon Buttons with 'settings' icon:");
                ui.horizontal(|ui| {
                    if ui.add(icon_button_standard("").svg_data(settings_svg)).clicked() {
                        println!("Standard icon button clicked!");
                    }

                    if ui.add(icon_button_filled("").svg_data(settings_svg)).clicked() {
                        println!("Filled icon button clicked!");
                    }

                    if ui.add(icon_button_filled_tonal("").svg_data(settings_svg)).clicked() {
                        println!("Filled tonal icon button clicked!");
                    }

                    if ui.add(icon_button_outlined("").svg_data(settings_svg)).clicked() {
                        println!("Outlined icon button clicked!");
                    }
                });
            }

            ui.add_space(8.0);

            if let Some(&star_svg) = SOLAR_ICONS.get("star") {
                ui.label("Buttons with 'star' icon:");
                ui.horizontal(|ui| {
                    if ui.add(MaterialButton::filled("Favorite").leading_svg(star_svg)).clicked() {
                        println!("Favorite button clicked!");
                    }

                    if ui.add(MaterialButton::outlined("Star").leading_svg(star_svg)).clicked() {
                        println!("Star button clicked!");
                    }

                    if ui.add(MaterialButton::elevated("Rate")
                        .leading_svg(star_svg)
                        .trailing_svg(star_svg)).clicked() {
                        println!("Rate button clicked!");
                    }
                });
            }

            ui.separator();

            // ══════════════════════════════════════════════════════════════════
            // Example 2: Using Noto Emoji
            // ══════════════════════════════════════════════════════════════════
            ui.heading("2. Noto Emoji (Google's Emoji Collection)");
            ui.label("~3600 emoji available - great for expressive UI");
            ui.add_space(8.0);

            // Noto emoji use Unicode filenames like "emoji_u1f600" for 😀
            if let Some(&smile_svg) = NOTO_EMOJIS.get("emoji_u1f600") {
                ui.label("Smile emoji (😀) on controls:");
                ui.horizontal(|ui| {
                    ui.add(assist_chip("Happy").leading_icon_svg(smile_svg).on_click(|| {}));
                    ui.add(icon_button_filled("").svg_data(smile_svg));
                    ui.add(MaterialButton::filled("Smile").leading_svg(smile_svg));
                });
            }

            ui.add_space(8.0);

            if let Some(&heart_svg) = NOTO_EMOJIS.get("emoji_u2764") {
                ui.label("Heart emoji (❤️) on controls:");
                ui.horizontal(|ui| {
                    ui.add(suggestion_chip("Love").leading_icon_svg(heart_svg).on_click(|| {}));
                    ui.add(icon_button_filled_tonal("").svg_data(heart_svg));
                    ui.add(MaterialButton::outlined("Like").leading_svg(heart_svg));
                });
            }

            ui.separator();

            // ══════════════════════════════════════════════════════════════════
            // Example 3: Using Twemoji (Twitter's Emoji)
            // ══════════════════════════════════════════════════════════════════
            ui.heading("3. Twemoji (Twitter's Emoji Collection)");
            ui.label("~3700 emoji with Twitter's distinctive style");
            ui.add_space(8.0);

            // Twemoji use Unicode codepoints like "1f44d" for 👍
            if let Some(&thumbs_up_svg) = TWEMOJI.get("1f44d") {
                ui.label("Thumbs up emoji (👍) on controls:");
                ui.horizontal(|ui| {
                    ui.add(assist_chip("Like").leading_icon_svg(thumbs_up_svg).on_click(|| {}));
                    ui.add(icon_button_standard("").svg_data(thumbs_up_svg));
                    ui.add(MaterialButton::elevated("Approve").leading_svg(thumbs_up_svg));
                });
            }

            ui.add_space(8.0);

            if let Some(&rocket_svg) = TWEMOJI.get("1f680") {
                ui.label("Rocket emoji (🚀) on controls:");
                ui.horizontal(|ui| {
                    ui.add(input_chip("Launch")
                        .leading_icon_svg(rocket_svg)
                        .removable(true)
                        .on_click(|| {}));
                    ui.add(icon_button_outlined("").svg_data(rocket_svg));
                    ui.add(MaterialButton::text("Deploy").trailing_svg(rocket_svg));
                });
            }

            ui.separator();

            // ══════════════════════════════════════════════════════════════════
            // Example 4: Browse Available Icons
            // ══════════════════════════════════════════════════════════════════
            ui.heading("4. Finding Available Icons");
            ui.label("Here are some common icon names you can use:");
            ui.add_space(8.0);

            ui.group(|ui| {
                ui.label("Solar Icons (common UI icons):");
                ui.code("home, settings, star, heart, bookmark, search, menu, close,");
                ui.code("add, edit, delete, check, arrow-left, arrow-right, user, bell");

                ui.add_space(8.0);
                ui.label("Finding emoji codes:");
                ui.code("Noto: Use 'emoji_u' + Unicode hex (e.g., emoji_u1f600)");
                ui.code("Twemoji: Use Unicode hex directly (e.g., 1f600)");
                ui.code("Unicode lookup: https://unicode.org/emoji/charts/full-emoji-list.html");
            });

            ui.separator();

            // ══════════════════════════════════════════════════════════════════
            // Example 5: Complete Code Pattern
            // ══════════════════════════════════════════════════════════════════
            ui.heading("5. Code Pattern");
            ui.add_space(8.0);

            ui.group(|ui| {
                ui.code("use egui_material3::svg_emoji::SOLAR_ICONS;");
                ui.code("");
                ui.code("// Get SVG data from the collection (note the &)");
                ui.code("if let Some(&svg) = SOLAR_ICONS.get(\"icon_name\") {");
                ui.code("    // Use with chips");
                ui.code("    ui.add(assist_chip(\"Label\").leading_icon_svg(svg));");
                ui.code("");
                ui.code("    // Use with icon buttons");
                ui.code("    ui.add(icon_button_filled(\"\").svg_data(svg));");
                ui.code("");
                ui.code("    // Use with buttons (leading and trailing)");
                ui.code("    ui.add(MaterialButton::filled(\"Action\")");
                ui.code("        .leading_svg(svg)");
                ui.code("        .trailing_svg(svg));");
                ui.code("}");
            });

            ui.add_space(16.0);

            // Note about FABs
            ui.horizontal(|ui| {
                ui.label("ℹ️");
                ui.label("Note: FABs use a different API (Material Symbols font icons). For SVG support in FABs, you would need to pre-render to textures.");
            });
        });
    }
}
