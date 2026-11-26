#![doc(hidden)]

#[cfg(feature = "svg_emoji")]
use crate::svg_emoji::SOLAR_ICONS;
use crate::{
    assist_chip, filter_chip, image_utils, input_chip, suggestion_chip, MaterialButton,
    MaterialCheckbox,
};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct ChipsWindow {
    pub open: bool,
    label: String,
    elevated: bool,
    disabled: bool,
    // Chip states
    filter_selected: bool,
    filter_disabled_selected: bool,
    filter_icon_selected: bool,
    filter_removable_selected: bool,
    filter_elevated_selected: bool,
    // Font testing
    system_fonts: Vec<(String, String)>,
    system_fonts_loaded: bool,
    selected_font_name: String,
    font_load_requested: bool,
}

impl Default for ChipsWindow {
    fn default() -> Self {
        Self {
            open: false,
            label: String::new(),
            elevated: false,
            disabled: false,
            filter_selected: true,
            filter_disabled_selected: true,
            filter_icon_selected: false,
            filter_removable_selected: true,
            filter_elevated_selected: false,
            system_fonts: Vec::new(),
            system_fonts_loaded: false,
            selected_font_name: "Default".to_string(),
            font_load_requested: false,
        }
    }
}

impl ChipsWindow {
    fn enumerate_system_fonts() -> Vec<(String, String)> {
        let mut fonts: Vec<(String, String)> = Vec::new();

        let font_dirs: Vec<&str> = if cfg!(target_os = "linux") {
            vec![
                "/usr/share/fonts",
                "/usr/local/share/fonts",
                "/usr/X11R6/lib/X11/fonts", // OpenBSD/BSD
            ]
        } else if cfg!(target_os = "macos") {
            vec!["/System/Library/Fonts", "/Library/Fonts"]
        } else if cfg!(target_os = "windows") {
            vec!["C:\\Windows\\Fonts"]
        } else {
            // Generic Unix/BSD paths
            vec![
                "/usr/share/fonts",
                "/usr/local/share/fonts",
                "/usr/X11R6/lib/X11/fonts",
            ]
        };

        println!("Scanning font directories: {:?}", font_dirs);

        for dir in font_dirs {
            match std::fs::read_dir(dir) {
                Ok(entries) => {
                    println!("  Scanning: {}", dir);
                    let mut dir_font_count = 0;
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            let ext_lower = ext.to_lowercase();
                            if ext_lower == "ttf" || ext_lower == "otf" {
                                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                                    fonts.push((
                                        name.to_string(),
                                        path.to_string_lossy().to_string(),
                                    ));
                                    dir_font_count += 1;
                                }
                            }
                        }
                        // Scan subdirectories one level deep
                        if path.is_dir() {
                            if let Ok(sub_entries) = std::fs::read_dir(&path) {
                                for sub_entry in sub_entries.flatten() {
                                    let sub_path = sub_entry.path();
                                    if let Some(ext) = sub_path.extension().and_then(|e| e.to_str())
                                    {
                                        let ext_lower = ext.to_lowercase();
                                        if ext_lower == "ttf" || ext_lower == "otf" {
                                            if let Some(name) =
                                                sub_path.file_stem().and_then(|n| n.to_str())
                                            {
                                                fonts.push((
                                                    name.to_string(),
                                                    sub_path.to_string_lossy().to_string(),
                                                ));
                                                dir_font_count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    println!("    Found {} fonts", dir_font_count);
                }
                Err(e) => {
                    println!("  Cannot access {}: {}", dir, e);
                }
            }
        }

        fonts.sort_by_key(|a| a.0.to_lowercase());
        println!("Total fonts found: {}", fonts.len());
        fonts
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        // Load system fonts on first show
        if !self.system_fonts_loaded {
            self.system_fonts = Self::enumerate_system_fonts();
            self.system_fonts_loaded = true;
        }

        // Handle font loading if requested
        if self.font_load_requested {
            self.font_load_requested = false;
            if self.selected_font_name == "Default" {
                // Reset to default fonts
                ctx.global_style_mut(|style| {
                    style.override_font_id = None;
                });
            } else if let Some((_, path)) = self
                .system_fonts
                .iter()
                .find(|(name, _)| name == &self.selected_font_name)
            {
                // Load custom font
                if let Ok(font_data) = std::fs::read(path) {
                    let mut fonts = egui::FontDefinitions::default();
                    fonts.font_data.insert(
                        "custom".to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .insert(0, "custom".to_owned());
                    ctx.set_fonts(fonts);
                }
            }
        }

        let mut open = self.open;
        Window::new("Chips Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_chips(ui);
                    ui.add_space(20.0);
                    self.render_assist_chips(ui);
                    ui.add_space(20.0);
                    self.render_small_assist_chips(ui);
                    ui.add_space(20.0);
                    self.render_filter_chips(ui);
                    ui.add_space(20.0);
                    self.render_small_filter_chips(ui);
                    ui.add_space(20.0);
                    self.render_input_chips(ui);
                    ui.add_space(20.0);
                    self.render_suggestion_chips(ui);
                    ui.add_space(20.0);
                    self.render_font_variations(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Chips Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/chip/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);
        });

        ui.add(MaterialCheckbox::new(&mut self.elevated, "Elevated"));
        ui.add(MaterialCheckbox::new(&mut self.disabled, "Disabled"));

        ui.separator();
        ui.label("System Font (for testing vertical centering):");
        ui.horizontal(|ui| {
            ui.label("Font:");
            let mut selected = self.selected_font_name.clone();
            egui::ComboBox::from_id_salt("font_selector")
                .selected_text(&selected)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(&mut selected, "Default".to_string(), "Default")
                        .clicked()
                    {
                        self.selected_font_name = selected.clone();
                        self.font_load_requested = true;
                    }
                    for (name, _) in &self.system_fonts {
                        if ui
                            .selectable_value(&mut selected, name.clone(), name.as_str())
                            .clicked()
                        {
                            self.selected_font_name = selected.clone();
                            self.font_load_requested = true;
                        }
                    }
                });
            if self.selected_font_name != "Default" {
                ui.label(format!("({})", self.selected_font_name));
            }
        });
    }

    fn render_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Chips");

        let label = self.label.clone();
        let elevated = self.elevated;
        let disabled = self.disabled;

        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        // Row 1: Enabled chips (matches Flutter component_screen.dart)
        ui.horizontal_wrapped(|ui| {
            let mut chip =
                assist_chip(l("Assist")).leading_icon(image_utils::material_icons::EVENT);
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip.on_click(|| println!("Assist chip clicked!")));
            ui.add_space(8.0);

            let mut chip = filter_chip(l("Filter"), &mut self.filter_selected);
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let mut chip = input_chip(l("Input")).removable(true);
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip.on_click(|| println!("Input chip clicked!")));
            ui.add_space(8.0);

            let mut chip = suggestion_chip(l("Suggestion"));
            if elevated {
                chip = chip.elevated(true);
            }
            if disabled {
                chip = chip.enabled(false);
            }
            ui.add(chip.on_click(|| println!("Suggestion chip clicked!")));
        });

        ui.add_space(12.0);

        // Row 2: Disabled chips
        ui.horizontal_wrapped(|ui| {
            let mut chip = assist_chip(l("Assist"))
                .leading_icon(image_utils::material_icons::EVENT)
                .enabled(false);
            if elevated {
                chip = chip.elevated(true);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let mut chip =
                filter_chip(l("Filter"), &mut self.filter_disabled_selected).enabled(false);
            if elevated {
                chip = chip.elevated(true);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let mut chip = input_chip(l("Input")).removable(true).enabled(false);
            if elevated {
                chip = chip.elevated(true);
            }
            ui.add(chip);
            ui.add_space(8.0);

            let chip = suggestion_chip(l("Suggestion")).enabled(false);
            ui.add(chip);
        });
    }

    fn render_assist_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Assist Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Plain assist chip
            ui.add(assist_chip(l("Assist chip")));
            ui.add_space(8.0);

            // With material icon
            ui.add(
                assist_chip(l("With icon"))
                    .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
            );
            ui.add_space(8.0);

            // With SVG icon (Google logo)
            ui.add(assist_chip(l("Assist link")).leading_icon_svg(image_utils::GOOGLE_LOGO_SVG));
            ui.add_space(8.0);

            // Elevated
            ui.add(assist_chip(l("Elevated")).elevated(true));
            ui.add_space(8.0);

            // Soft-disabled
            ui.add(assist_chip(l("Soft-disabled")).soft_disabled(true));
        });
    }

    fn render_small_assist_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Small Assist Chips");

        let label = self.label.clone();
        let _l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        #[cfg(feature = "svg_emoji")]
        {
            ui.horizontal_wrapped(|ui| {
                // Plain small assist chip
                ui.add(assist_chip(l("Small")).small());
                ui.add_space(8.0);

                // Small with material icon
                ui.add(
                    assist_chip(l("Icon"))
                        .small()
                        .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
                );
                ui.add_space(8.0);

                // Small with SVG icon from Solar Icons
                if let Some(&star_svg) = SOLAR_ICONS.get("star") {
                    ui.add(assist_chip(l("Star")).small().leading_icon_svg(star_svg));
                    ui.add_space(8.0);
                }

                if let Some(&heart_svg) = SOLAR_ICONS.get("heart") {
                    ui.add(assist_chip(l("Like")).small().leading_icon_svg(heart_svg));
                    ui.add_space(8.0);
                }

                if let Some(&settings_svg) = SOLAR_ICONS.get("settings") {
                    ui.add(
                        assist_chip(l("Settings"))
                            .small()
                            .leading_icon_svg(settings_svg),
                    );
                    ui.add_space(8.0);
                }

                // Small elevated
                ui.add(assist_chip(l("Elevated")).small().elevated(true));
                ui.add_space(8.0);

                // Small soft-disabled
                ui.add(assist_chip(l("Disabled")).small().soft_disabled(true));
            });
        }
    }

    fn render_filter_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Filter Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Basic filter chip
            ui.add(filter_chip(
                l("Filter chip"),
                &mut self.filter_icon_selected,
            ));
            ui.add_space(8.0);

            // With leading icon
            ui.add(
                filter_chip(l("With icon"), &mut self.filter_elevated_selected)
                    .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
            );
            ui.add_space(8.0);

            // Removable filter chip
            ui.add(
                filter_chip(l("Removable"), &mut self.filter_removable_selected).removable(true),
            );
            ui.add_space(8.0);

            // Elevated filter chip
            let mut elevated_selected = false;
            ui.add(filter_chip(l("Elevated"), &mut elevated_selected).elevated(true));
            ui.add_space(8.0);

            // Soft-disabled filter chip
            let mut soft_selected = true;
            ui.add(filter_chip(l("Soft-disabled"), &mut soft_selected).soft_disabled(true));
        });
    }

    fn render_small_filter_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Small Filter Chips");

        let label = self.label.clone();
        let _l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        #[cfg(feature = "svg_emoji")]
        {
            ui.horizontal_wrapped(|ui| {
                // Basic small filter chip
                let mut small_filter_selected_1 = true;
                ui.add(filter_chip(l("Filter"), &mut small_filter_selected_1).small());
                ui.add_space(8.0);

                // Small filter with material icon
                let mut small_filter_selected_2 = false;
                ui.add(
                    filter_chip(l("Icon"), &mut small_filter_selected_2)
                        .small()
                        .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
                );
                ui.add_space(8.0);

                // Small filter with SVG icons from Solar Icons
                if let Some(&star_svg) = SOLAR_ICONS.get("star") {
                    let mut selected = false;
                    ui.add(
                        filter_chip(l("Star"), &mut selected)
                            .small()
                            .leading_icon_svg(star_svg),
                    );
                    ui.add_space(8.0);
                }

                if let Some(&bookmark_svg) = SOLAR_ICONS.get("bookmark") {
                    let mut small_filter_selected_3 = true;
                    ui.add(
                        filter_chip(l("Save"), &mut small_filter_selected_3)
                            .small()
                            .leading_icon_svg(bookmark_svg),
                    );
                    ui.add_space(8.0);
                }

                if let Some(&heart_svg) = SOLAR_ICONS.get("heart") {
                    let mut selected = true;
                    ui.add(
                        filter_chip(l("Like"), &mut selected)
                            .small()
                            .leading_icon_svg(heart_svg),
                    );
                    ui.add_space(8.0);
                }

                // Small removable filter chip
                let mut removable_selected = true;
                ui.add(
                    filter_chip(l("Remove"), &mut removable_selected)
                        .small()
                        .removable(true),
                );
                ui.add_space(8.0);

                // Small elevated filter chip
                let mut elevated_selected = false;
                ui.add(
                    filter_chip(l("Elevated"), &mut elevated_selected)
                        .small()
                        .elevated(true),
                );
                ui.add_space(8.0);

                // Small soft-disabled filter chip
                let mut soft_selected = true;
                ui.add(
                    filter_chip(l("Disabled"), &mut soft_selected)
                        .small()
                        .soft_disabled(true),
                );
            });
        }
    }

    fn render_input_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Plain input chip
            ui.add(input_chip(l("Input chip")));
            ui.add_space(8.0);

            // With material icon
            ui.add(
                input_chip(l("With icon"))
                    .leading_icon(image_utils::material_icons::ACCOUNT_CIRCLE),
            );
            ui.add_space(8.0);

            // With avatar (SVG) - uses avatar styling
            ui.add(
                input_chip(l("With avatar"))
                    .leading_icon_svg(image_utils::AVATAR_SVG)
                    .avatar(true),
            );
            ui.add_space(8.0);

            // With Google logo link
            ui.add(input_chip(l("Input link")).leading_icon_svg(image_utils::GOOGLE_LOGO_SVG));
            ui.add_space(8.0);

            // Removable
            ui.add(input_chip(l("Removable")).removable(true));
            ui.add_space(8.0);

            // Soft-disabled
            ui.add(input_chip(l("Soft-disabled")).soft_disabled(true));
        });
    }

    fn render_suggestion_chips(&mut self, ui: &mut egui::Ui) {
        ui.heading("Suggestion Chips");

        let label = self.label.clone();
        let l = |default: &str| -> String {
            if label.is_empty() {
                default.to_string()
            } else {
                label.clone()
            }
        };

        ui.horizontal_wrapped(|ui| {
            // Plain suggestion chip
            ui.add(suggestion_chip(l("Suggestion chip")));
            ui.add_space(8.0);

            // With icon
            ui.add(
                suggestion_chip(l("With icon"))
                    .leading_icon(image_utils::material_icons::LOCAL_LAUNDRY_SERVICE),
            );
            ui.add_space(8.0);

            // With SVG icon
            ui.add(
                suggestion_chip(l("Suggestion link"))
                    .leading_icon_svg(image_utils::GOOGLE_LOGO_SVG),
            );
            ui.add_space(8.0);

            // Elevated
            ui.add(suggestion_chip(l("Elevated")).elevated(true));
            ui.add_space(8.0);

            // Soft-disabled
            ui.add(suggestion_chip(l("Soft-disabled")).soft_disabled(true));
        });
    }

    fn render_font_variations(&mut self, ui: &mut egui::Ui) {
        ui.heading("Font Variations (Vertical Centering Test)");
        ui.label("Chips with different fonts/sizes should maintain vertical centering:");

        // Test with different text styles
        ui.add_space(12.0);
        ui.strong("Text Style Variations:");

        let text_styles = [
            (egui::TextStyle::Small, "Small"),
            (egui::TextStyle::Body, "Body"),
            (egui::TextStyle::Button, "Button"),
            (egui::TextStyle::Heading, "Heading"),
            (egui::TextStyle::Monospace, "Monospace"),
        ];

        for (text_style, label) in text_styles {
            ui.add_space(8.0);
            ui.label(format!("  {} style:", label));

            ui.horizontal_wrapped(|ui| {
                ui.scope(|ui| {
                    ui.style_mut().override_text_style = Some(text_style.clone());

                    let mut filter_selected = true;
                    ui.add(filter_chip("Filter", &mut filter_selected).elevated(true));
                    ui.add_space(8.0);

                    ui.add(
                        assist_chip("Assist")
                            .elevated(true)
                            .leading_icon(image_utils::material_icons::EVENT),
                    );
                    ui.add_space(8.0);

                    ui.add(input_chip("Input").elevated(true).removable(true));
                    ui.add_space(8.0);

                    ui.add(suggestion_chip("Suggestion").elevated(true));
                });
            });
        }

        // Test with custom font sizes (Proportional family)
        ui.add_space(16.0);
        ui.strong("Proportional Font - Custom Sizes:");

        let font_sizes = [10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 24.0];

        for size in font_sizes {
            ui.add_space(8.0);
            ui.label(format!("  {}px:", size));

            ui.horizontal_wrapped(|ui| {
                ui.scope(|ui| {
                    let font_id = egui::FontId::proportional(size);
                    ui.style_mut().override_text_style = None;
                    // Store font_id in temporary var to modify text rendering
                    let old_text_styles = ui.style().text_styles.clone();
                    ui.style_mut()
                        .text_styles
                        .insert(egui::TextStyle::Body, font_id.clone());

                    let mut filter_selected = true;
                    ui.add(filter_chip("Filter", &mut filter_selected).elevated(true));
                    ui.add_space(8.0);

                    ui.add(
                        assist_chip("Assist")
                            .elevated(true)
                            .leading_icon(image_utils::material_icons::EVENT),
                    );
                    ui.add_space(8.0);

                    ui.add(input_chip("Input").elevated(true).removable(true));
                    ui.add_space(8.0);

                    ui.add(suggestion_chip("Suggestion").elevated(true));

                    ui.style_mut().text_styles = old_text_styles;
                });
            });
        }

        // Test with custom font sizes (Monospace family)
        ui.add_space(16.0);
        ui.strong("Monospace Font - Custom Sizes:");

        for size in [10.0, 12.0, 14.0, 16.0, 18.0, 20.0] {
            ui.add_space(8.0);
            ui.label(format!("  {}px:", size));

            ui.horizontal_wrapped(|ui| {
                ui.scope(|ui| {
                    let font_id = egui::FontId::monospace(size);
                    ui.style_mut().override_text_style = None;
                    let old_text_styles = ui.style().text_styles.clone();
                    ui.style_mut()
                        .text_styles
                        .insert(egui::TextStyle::Body, font_id.clone());

                    let mut filter_selected = false;
                    ui.add(filter_chip("Filter", &mut filter_selected).elevated(true));
                    ui.add_space(8.0);

                    ui.add(
                        assist_chip("Assist")
                            .elevated(true)
                            .leading_icon(image_utils::material_icons::EVENT),
                    );
                    ui.add_space(8.0);

                    ui.add(input_chip("Code").elevated(true).removable(true));
                    ui.add_space(8.0);

                    ui.add(suggestion_chip("Terminal").elevated(true));

                    ui.style_mut().text_styles = old_text_styles;
                });
            });
        }
    }
}
