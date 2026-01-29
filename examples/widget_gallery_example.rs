use eframe::egui;
use egui_material3::{
    theme::{
        load_fonts, load_themes, setup_google_fonts, setup_local_fonts, setup_local_theme,
        update_global_theme, update_window_background, ContrastLevel, MaterialThemeContext,
        MaterialThemeFile, ThemeMode,
    },
    MaterialButton, MaterialCard2, MaterialCheckbox, MaterialChip, MaterialDialog, MaterialDrawer,
    MaterialFab, MaterialIcon, MaterialIconButton, MaterialList, MaterialProgress, MaterialRadio,
    MaterialSelect, MaterialSlider, MaterialSwitch, MaterialTabs, MaterialTopAppBar,
};
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Material Widget Gallery Example 테스트",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and themes
            // setup_google_fonts(Some("Roboto"));
            // setup_local_fonts(Some("resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf"));
            setup_local_theme(Some("resources/material-theme6.json"));

            load_fonts(&cc.egui_ctx);
            load_themes();

            // Apply initial window background based on loaded theme
            update_window_background(&cc.egui_ctx);

            Ok(Box::<WidgetGalleryApp>::default())
        }),
    )
}

#[derive(Debug, PartialEq)]
enum Animal {
    Ferris,
    Dolphin,
    Horse,
}

#[derive(Debug, PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

struct WidgetGalleryApp {
    // Widget state
    text: String,
    multiline_text: String,
    number: f32,
    slider_value: f32,
    checkbox: bool,
    radio: Enum,
    radio_selected: Option<usize>,
    select_selected: Option<usize>,
    color: egui::Color32,
    animate_progress_bar: bool,

    // Gallery controls
    enabled: bool,
    visible: bool,

    // Theme
    theme_loaded: bool,
    theme_mode: ThemeMode,
    contrast_level: ContrastLevel,
}

impl Default for WidgetGalleryApp {
    fn default() -> Self {
        Self {
            text: "Edit me!".to_owned(),
            multiline_text: "This is a\nmultiline text\neditor.".to_owned(),
            number: 42.0,
            slider_value: 0.5,
            checkbox: true,
            radio: Enum::First,
            radio_selected: Some(0),
            select_selected: Some(0),
            color: egui::Color32::LIGHT_BLUE,
            animate_progress_bar: false,
            enabled: true,
            visible: true,
            theme_loaded: false,
            theme_mode: ThemeMode::Light,
            contrast_level: ContrastLevel::Normal,
        }
    }
}

impl WidgetGalleryApp {
    fn load_theme_from_file(
        &self,
        file_path: &str,
    ) -> Result<MaterialThemeFile, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let theme: MaterialThemeFile = serde_json::from_str(&content)?;
        Ok(theme)
    }

    fn update_theme_mode(&mut self, ctx: &egui::Context) {
        // Update the global theme context with new theme mode and contrast level
        if let Ok(mut global_theme) = egui_material3::theme::get_global_theme().lock() {
            global_theme.theme_mode = self.theme_mode;
            global_theme.contrast_level = self.contrast_level;
        }
        // Update window background to reflect the new theme settings
        update_window_background(ctx);
    }
}

impl eframe::App for WidgetGalleryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update window background based on theme
        update_window_background(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Material Design Widget Gallery 테스트");
            ui.add_space(10.0);

            // Theme controls
            ui.horizontal(|ui| {
                ui.label("Theme Status:");
                if self.theme_loaded {
                    ui.colored_label(
                        egui::Color32::GREEN,
                        "✓ Custom theme loaded (material-theme6.json)",
                    );
                } else {
                    ui.colored_label(egui::Color32::RED, "✗ Using default theme");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Theme Mode:");
                let mut changed = false;
                changed |= ui
                    .selectable_value(&mut self.theme_mode, ThemeMode::Light, "Light")
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.theme_mode, ThemeMode::Dark, "Dark")
                    .changed();

                ui.separator();

                ui.label("Contrast:");
                changed |= ui
                    .selectable_value(&mut self.contrast_level, ContrastLevel::Normal, "Normal")
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.contrast_level, ContrastLevel::Medium, "Medium")
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.contrast_level, ContrastLevel::High, "High")
                    .changed();

                if changed {
                    self.update_theme_mode(ctx);
                    ctx.request_repaint();
                }
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            // Gallery controls
            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.enabled, "Enabled"));
                ui.add(MaterialCheckbox::new(&mut self.visible, "Visible"));
            });

            ui.add_space(10.0);

            if !self.visible {
                ui.label("Widgets are hidden");
                return;
            }

            ui.add_enabled_ui(self.enabled, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("widget_grid")
                        .num_columns(2)
                        .spacing([40.0, 10.0])
                        .show(ui, |ui| {
                            ui.label("MaterialIcon");
                            ui.add(MaterialIcon::new("star").size(24.0));
                            ui.end_row();

                            ui.label("MaterialButton");
                            ui.add(MaterialButton::filled("Filled Button"));
                            ui.end_row();

                            ui.label("MaterialButton Outlined");
                            ui.add(MaterialButton::outlined("Outlined Button"));
                            ui.end_row();

                            ui.label("MaterialButton Text");
                            if ui.add(MaterialButton::text("Text Button")).clicked() {
                                self.checkbox = !self.checkbox;
                            }
                            ui.end_row();

                            ui.label("MaterialIconButton");
                            if ui.add(MaterialIconButton::filled("favorite")).clicked() {
                                self.checkbox = !self.checkbox;
                            }
                            ui.end_row();

                            ui.label("MaterialCheckbox");
                            ui.add(MaterialCheckbox::new(
                                &mut self.checkbox,
                                "Material Checkbox",
                            ));
                            ui.end_row();

                            ui.label("MaterialRadio");
                            ui.horizontal(|ui| {
                                ui.add(MaterialRadio::new(&mut self.radio_selected, 0, "First"));
                                ui.add(MaterialRadio::new(&mut self.radio_selected, 1, "Second"));
                                ui.add(MaterialRadio::new(&mut self.radio_selected, 2, "Third"));
                            });
                            ui.end_row();

                            ui.label("MaterialSwitch");
                            ui.add(MaterialSwitch::new(&mut self.checkbox).text("Material Switch"));
                            ui.end_row();

                            ui.label("MaterialSelect");
                            ui.add(
                                MaterialSelect::new(&mut self.select_selected)
                                    .option(0, "First")
                                    .option(1, "Second")
                                    .option(2, "Third")
                                    .placeholder("Select option"),
                            );
                            ui.end_row();

                            ui.label("MaterialSlider");
                            ui.add(
                                MaterialSlider::new(&mut self.slider_value, 0.0..=360.0)
                                    .text("Material Slider"),
                            );
                            ui.end_row();

                            ui.label("MaterialFab");
                            if ui.add(MaterialFab::primary().icon("add")).clicked() {
                                self.number += 1.0;
                            }
                            ui.end_row();

                            ui.label("MaterialProgress");
                            let progress = self.slider_value / 360.0;
                            ui.add(MaterialProgress::linear().value(progress));
                            ui.end_row();

                            ui.label("MaterialChip");
                            ui.add(MaterialChip::assist("Material Chip"));
                            ui.end_row();

                            ui.label("MaterialCard2");
                            ui.add(MaterialCard2::elevated().content(|ui| {
                                ui.label("Material Card Content");
                            }));
                            ui.end_row();

                            ui.label("MaterialIconButton Toggle");
                            ui.add(MaterialIconButton::outlined("toggle_on"));
                            ui.end_row();

                            // Date picker feature disabled for this example

                            ui.label("MaterialProgress Circular");
                            ui.add(MaterialProgress::circular().value(0.7));
                            ui.end_row();

                            ui.label("MaterialButton Elevated");
                            ui.add(MaterialButton::elevated("Elevated Button"));
                            ui.end_row();

                            ui.label("MaterialButton Tonal");
                            ui.add(MaterialButton::filled_tonal("Tonal Button"))
                                .on_hover_text("Material Design components!");
                            ui.end_row();

                            ui.label("Label");
                            ui.label("Welcome to the widget gallery!");
                            ui.end_row();

                            ui.label("Hyperlink");
                            use egui::special_emojis::GITHUB;
                            ui.hyperlink_to(
                                format!("{GITHUB} egui on GitHub"),
                                "https://github.com/emilk/egui",
                            );
                            ui.end_row();

                            ui.label("TextEdit");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.text)
                                    .hint_text("Write something here"),
                            );
                            ui.end_row();

                            ui.label("Button");
                            if ui.button("Click me!").clicked() {
                                self.checkbox = !self.checkbox;
                            }
                            ui.end_row();

                            ui.label("Link");
                            if ui.link("Click me!").clicked() {
                                self.checkbox = !self.checkbox;
                            }
                            ui.end_row();

                            ui.label("Checkbox");
                            ui.checkbox(&mut self.checkbox, "Checkbox");
                            ui.end_row();

                            ui.label("RadioButton");
                            ui.horizontal(|ui| {
                                ui.radio_value(&mut self.radio, Enum::First, "First");
                                ui.radio_value(&mut self.radio, Enum::Second, "Second");
                                ui.radio_value(&mut self.radio, Enum::Third, "Third");
                            });
                            ui.end_row();

                            ui.label("SelectableLabel");
                            ui.horizontal(|ui| {
                                ui.selectable_value(&mut self.radio, Enum::First, "First");
                                ui.selectable_value(&mut self.radio, Enum::Second, "Second");
                                ui.selectable_value(&mut self.radio, Enum::Third, "Third");
                            });
                            ui.end_row();

                            ui.label("ComboBox");

                            egui::ComboBox::from_label("Take your pick")
                                .selected_text(format!("{:?}", self.radio))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.radio, Enum::First, "First");
                                    ui.selectable_value(&mut self.radio, Enum::Second, "Second");
                                    ui.selectable_value(&mut self.radio, Enum::Third, "Third");
                                });
                            ui.end_row();

                            ui.label("Slider");
                            ui.add(egui::Slider::new(&mut self.number, 0.0..=360.0).suffix("°"));
                            ui.end_row();

                            ui.label("DragValue");
                            ui.add(egui::DragValue::new(&mut self.number).speed(1.0));
                            ui.end_row();

                            ui.label("ProgressBar");
                            let progress = self.number / 360.0;
                            let progress_bar = egui::ProgressBar::new(progress)
                                .show_percentage()
                                .animate(self.animate_progress_bar);
                            self.animate_progress_bar = ui
                                .add(progress_bar)
                                .on_hover_text("The progress bar can be animated!")
                                .hovered();
                            ui.end_row();

                            ui.label("Color picker");
                            ui.color_edit_button_srgba(&mut self.color);
                            ui.end_row();

                            ui.label("Image");
                            let egui_icon = egui::include_image!("../resources/screenshot.png");
                            ui.add(
                                egui::Image::new(egui_icon.clone())
                                    .max_size(egui::Vec2::new(64.0, 64.0)),
                            );
                            ui.end_row();

                            ui.label("Button with image");
                            if ui
                                .add(egui::Button::image_and_text(egui_icon, "Click me!"))
                                .clicked()
                            {
                                self.checkbox = !self.checkbox;
                            }
                            ui.end_row();

                            #[cfg(feature = "chrono")]
                            if *with_date_button {
                                let date = date
                                    .get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                                ui.add(doc_link_label_with_crate(
                                    "egui_extras",
                                    "DatePickerButton",
                                    "DatePickerButton",
                                ));
                                ui.add(egui_extras::DatePickerButton::new(date));
                                ui.end_row();
                            }

                            ui.label("Separator");
                            ui.separator();
                            ui.end_row();

                            ui.label("CollapsingHeader");
                            ui.collapsing("Click to see what is hidden!", |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    ui.label("It's a ");
                                    ui.label("spinner");
                                    ui.add_space(4.0);
                                    ui.add(egui::Spinner::new());
                                });
                            });
                            ui.end_row();
                        });
                });
            });
        });

        // Request repaint for animated elements
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}
