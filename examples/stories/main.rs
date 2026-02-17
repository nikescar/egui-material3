#![doc(hidden)]

use eframe::egui::{self, Color32};
use egui_file_dialog::FileDialog;
use egui_material3::theme::{
    load_fonts, load_themes, setup_google_fonts,
    setup_local_fonts_from_bytes, setup_local_theme,
};
use egui_material3::*;
use std::collections::HashMap;
use std::path::PathBuf;

// Import window modules - reorganized from src/ to examples/stories/ directory structure
mod button_window;
mod card2_window;
mod checkbox_window;
mod chips_window;
mod datatable_window;
mod dialog_window;
mod drawer_window;
mod fab_window;
mod iconbutton_window;
mod imagelist_window;
mod layoutgrid_window;
mod list_window;
mod menu_window;
mod progress_window;
mod radio_window;
mod select_window;
mod slider_window;
mod snackbar_window;
mod switch_window;
mod tabs_window;
mod material_symbol_icons;
mod topappbar_window;

use button_window::ButtonWindow;
use card2_window::Card2Window;
use checkbox_window::CheckboxWindow;
use chips_window::ChipsWindow;
use datatable_window::DataTableWindow;
use dialog_window::DialogWindow;
use drawer_window::DrawerWindow;
use fab_window::FabWindow;
use iconbutton_window::IconButtonWindow;
use imagelist_window::ImageListWindow;
use layoutgrid_window::LayoutGridWindow;
use list_window::ListWindow;
use menu_window::MenuWindow;
use progress_window::ProgressWindow;
use radio_window::RadioWindow;
use select_window::SelectWindow;
use slider_window::SliderWindow;
use snackbar_window::SnackbarWindow;
use switch_window::SwitchWindow;
use tabs_window::TabsWindow;
use topappbar_window::TopAppBarWindow;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 1200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Material Design Components Demo 고",
        options,
        Box::new(|cc| {
            // Prepare local fonts including Material Symbols (using include_bytes!)
            setup_local_fonts_from_bytes(
                "MaterialSymbolsOutlined",
                include_bytes!("../../resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf"),
            );
            setup_local_fonts_from_bytes(
                "Nanum Gothic",
                include_bytes!("../../resources/nanum-gothic.ttf"),
            );
            // Prepare Google Sans Code font for Material Design (default)
            setup_google_fonts(Some("Google Sans Code"));
            setup_google_fonts(Some("Nanum Gothic"));
            // Prepare themes from build-time constants
            setup_local_theme(None);
            // Install image loaders
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // Load all prepared fonts and themes
            load_fonts(&cc.egui_ctx);
            load_themes();
            Ok(Box::<MaterialApp>::default())
        }),
    )
}

struct MaterialApp {
    text_content: String,
    checkbox_checked: bool,
    checkbox_indeterminate: bool,
    filter_chip_selected: bool,
    dialog_open: bool,
    menu_open: bool,
    icon_button_selected: bool,
    // New control states
    progress_value: f32,
    radio_selected: Option<usize>,
    select_selected: Option<usize>,
    slider_value: f32,
    switch_enabled: bool,
    tab_selected: usize,
    // Theme changer controls
    file_dialog: FileDialog,
    selected_file_path: Option<PathBuf>,
    color_pickers_open: HashMap<String, bool>,
    // Demo windows
    button_window: ButtonWindow,
    checkbox_window: CheckboxWindow,
    chips_window: ChipsWindow,
    dialog_window: DialogWindow,
    fab_window: FabWindow,
    iconbutton_window: IconButtonWindow,
    list_window: ListWindow,
    menu_window: MenuWindow,
    progress_window: ProgressWindow,
    radio_window: RadioWindow,
    select_window: SelectWindow,
    slider_window: SliderWindow,
    switch_window: SwitchWindow,
    tabs_window: TabsWindow,
    datatable_window: DataTableWindow,
    drawer_window: DrawerWindow,
    imagelist_window: ImageListWindow,
    layoutgrid_window: LayoutGridWindow,
    snackbar_window: SnackbarWindow,
    topappbar_window: TopAppBarWindow,
    card2_window: Card2Window,
}

impl Default for MaterialApp {
    fn default() -> Self {
        Self {
            text_content: String::new(),
            checkbox_checked: false,
            checkbox_indeterminate: false,
            filter_chip_selected: false,
            dialog_open: false,
            menu_open: false,
            icon_button_selected: false,
            progress_value: 0.0,
            radio_selected: None,
            select_selected: None,
            slider_value: 0.0,
            switch_enabled: false,
            tab_selected: 0,
            file_dialog: FileDialog::new(),
            selected_file_path: None,
            color_pickers_open: HashMap::new(),
            button_window: ButtonWindow::default(),
            checkbox_window: CheckboxWindow::default(),
            chips_window: ChipsWindow::default(),
            dialog_window: DialogWindow::default(),
            fab_window: FabWindow::default(),
            iconbutton_window: IconButtonWindow::default(),
            list_window: ListWindow::default(),
            menu_window: MenuWindow::default(),
            progress_window: ProgressWindow::default(),
            radio_window: RadioWindow::default(),
            select_window: SelectWindow::default(),
            slider_window: SliderWindow::default(),
            switch_window: SwitchWindow::default(),
            tabs_window: TabsWindow::default(),
            datatable_window: DataTableWindow::default(),
            drawer_window: DrawerWindow::default(),
            imagelist_window: ImageListWindow::default(),
            layoutgrid_window: LayoutGridWindow::default(),
            snackbar_window: SnackbarWindow::default(),
            topappbar_window: TopAppBarWindow::default(),
            card2_window: Card2Window::default(),
        }
    }
}

impl MaterialApp {
    fn get_theme(&self) -> MaterialThemeContext {
        if let Ok(theme) = get_global_theme().lock() {
            theme.clone()
        } else {
            MaterialThemeContext::default()
        }
    }

    fn update_theme<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut MaterialThemeContext),
    {
        if let Ok(mut theme) = get_global_theme().lock() {
            update_fn(&mut *theme);
        }
    }

    fn load_theme_from_file(
        &self,
        file_path: &PathBuf,
    ) -> Result<MaterialThemeFile, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let theme: MaterialThemeFile = serde_json::from_str(&content)?;
        Ok(theme)
    }

    fn load_theme_file(&mut self) {
        // Open the file dialog to pick a file.
        self.file_dialog.pick_file();
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let theme = self.get_theme();

        let mut visuals = match theme.theme_mode {
            ThemeMode::Light => egui::Visuals::light(),
            ThemeMode::Dark => egui::Visuals::dark(),
            ThemeMode::Auto => {
                // Use system preference or default to light
                if ctx.style().visuals.dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                }
            }
        };

        // Apply Material Design 3 colors if theme is loaded
        let primary_color = theme.get_primary_color();
        let on_primary = theme.get_on_primary_color();
        let surface = theme.get_surface_color(visuals.dark_mode);
        let on_surface = theme.get_color_by_name("onSurface");
        let background = theme.get_color_by_name("background");
        let on_background = theme.get_color_by_name("onBackground");

        // Apply colors to visuals
        visuals.selection.bg_fill = primary_color;
        visuals.selection.stroke.color = primary_color;
        visuals.hyperlink_color = primary_color;

        // Button and widget colors
        visuals.widgets.noninteractive.bg_fill = surface;

        visuals.widgets.inactive.bg_fill = Color32::from_rgba_unmultiplied(
            primary_color.r(),
            primary_color.g(),
            primary_color.b(),
            20,
        );

        visuals.widgets.hovered.bg_fill = Color32::from_rgba_unmultiplied(
            primary_color.r(),
            primary_color.g(),
            primary_color.b(),
            40,
        );

        visuals.widgets.active.bg_fill = primary_color;
        visuals.widgets.active.fg_stroke.color = on_primary;

        // Window background
        visuals.window_fill = surface;
        visuals.panel_fill = theme.get_color_by_name("surfaceContainer");

        // Text colors
        // visuals.override_text_color = Some(background);

        // Apply surface colors
        visuals.extreme_bg_color = theme.get_color_by_name("surfaceContainerLowest");

        ctx.set_visuals(visuals);
    }

    fn lighten_color(&self, color: Color32, factor: f32) -> Color32 {
        let r = (color.r() as f32 + (255.0 - color.r() as f32) * factor).min(255.0) as u8;
        let g = (color.g() as f32 + (255.0 - color.g() as f32) * factor).min(255.0) as u8;
        let b = (color.b() as f32 + (255.0 - color.b() as f32) * factor).min(255.0) as u8;
        Color32::from_rgb(r, g, b)
    }

    fn darken_color(&self, color: Color32, factor: f32) -> Color32 {
        let r = (color.r() as f32 * (1.0 - factor)).max(0.0) as u8;
        let g = (color.g() as f32 * (1.0 - factor)).max(0.0) as u8;
        let b = (color.b() as f32 * (1.0 - factor)).max(0.0) as u8;
        Color32::from_rgb(r, g, b)
    }

    fn is_dark_color(&self, color: Color32) -> bool {
        let luminance =
            0.299 * color.r() as f32 + 0.587 * color.g() as f32 + 0.114 * color.b() as f32;
        luminance < 128.0
    }

    fn adjust_color_hue(&self, color: Color32, hue_shift: f32) -> Color32 {
        // Convert to HSV, adjust hue, convert back
        let r = color.r() as f32 / 255.0;
        let g = color.g() as f32 / 255.0;
        let b = color.b() as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let mut hue = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        if hue < 0.0 {
            hue += 360.0;
        }

        // Adjust hue
        hue = (hue + hue_shift) % 360.0;
        if hue < 0.0 {
            hue += 360.0;
        }

        let saturation = if max == 0.0 { 0.0 } else { delta / max };
        let value = max;

        // Convert back to RGB
        let h = hue / 60.0;
        let c = value * saturation;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = value - c;

        let (r_prime, g_prime, b_prime) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r = ((r_prime + m) * 255.0).clamp(0.0, 255.0) as u8;
        let g = ((g_prime + m) * 255.0).clamp(0.0, 255.0) as u8;
        let b = ((b_prime + m) * 255.0).clamp(0.0, 255.0) as u8;

        Color32::from_rgb(r, g, b)
    }

    /// Close all open demo windows
    fn close_all_windows(&mut self) {
        self.button_window.open = false;
        self.checkbox_window.open = false;
        self.chips_window.open = false;
        self.dialog_window.open = false;
        self.fab_window.open = false;
        self.iconbutton_window.open = false;
        self.list_window.open = false;
        self.menu_window.open = false;
        self.progress_window.open = false;
        self.radio_window.open = false;
        self.select_window.open = false;
        self.slider_window.open = false;
        self.switch_window.open = false;
        self.tabs_window.open = false;
        self.datatable_window.open = false;
        self.drawer_window.open = false;
        self.imagelist_window.open = false;
        self.layoutgrid_window.open = false;
        self.snackbar_window.open = false;
        self.topappbar_window.open = false;
        self.card2_window.open = false;
        self.color_pickers_open.clear(); // Also close all color pickers
    }
}

impl eframe::App for MaterialApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme based on settings
        self.apply_theme(ctx);

        // Global ESC key handler to close all sub windows
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.close_all_windows();
        }

        // Update the file dialog
        self.file_dialog.update(ctx);

        // Check if the user picked a file.
        if let Some(path) = self.file_dialog.take_picked() {
            match self.load_theme_from_file(&path) {
                Ok(theme) => {
                    self.selected_file_path = Some(path.clone());
                    self.update_theme(|global_theme| {
                        global_theme.material_theme = Some(theme);
                        global_theme.selected_colors.clear();
                    });
                }
                Err(e) => {
                    eprintln!("Failed to load theme file: {}", e);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = self.get_theme();

            ui.heading("Material Design Components Demo 테스트");
            ui.add_space(10.0);

            // Material Design 3 Theme Controls
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Material Theme Controls");

                    if ui.add(MaterialButton::filled("Theme Builder").small()).clicked() {
                        let _ = webbrowser::open(
                            "https://material-foundation.github.io/material-theme-builder/",
                        );
                    }

                    if ui.add(MaterialButton::filled("Google Fonts").small()).clicked() {
                        let _ = webbrowser::open(
                            "https://fonts.google.com/specimen/Google+Sans+Code?query=google+sans",
                        );
                    }
                });

                ui.add_space(8.0);

                // File Upload Section
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Theme File:");

                        if ui.add(MaterialButton::filled("📁 Load JSON Theme").small()).clicked() {
                            self.load_theme_file();
                        }

                        if let Some(ref path) = self.selected_file_path {
                            ui.label(format!(
                                "Loaded: {}",
                                path.file_name().unwrap_or_default().to_string_lossy()
                            ));
                        } else {
                            ui.label("No theme loaded");
                        }
                    });
                });

                ui.add_space(8.0);

                // Theme Mode Selector
                ui.horizontal(|ui| {
                    ui.label("Color Mode:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            // Light mode button
                            let light_selected = theme.theme_mode == ThemeMode::Light;
                            let light_button = ui.selectable_label(light_selected, "☀️ Light");
                            if light_button.clicked() {
                                self.update_theme(|theme| {
                                    theme.theme_mode = ThemeMode::Light;
                                });
                            }

                            // Auto mode button
                            let auto_selected = theme.theme_mode == ThemeMode::Auto;
                            let auto_button = ui.selectable_label(auto_selected, "🌗 Auto");
                            if auto_button.clicked() {
                                self.update_theme(|theme| {
                                    theme.theme_mode = ThemeMode::Auto;
                                });
                            }

                            // Dark mode button
                            let dark_selected = theme.theme_mode == ThemeMode::Dark;
                            let dark_button = ui.selectable_label(dark_selected, "🌙 Dark");
                            if dark_button.clicked() {
                                self.update_theme(|theme| {
                                    theme.theme_mode = ThemeMode::Dark;
                                });
                            }
                        });
                    });
                });

                ui.add_space(8.0);

                // Contrast Level Selection
                ui.horizontal(|ui| {
                    ui.label("Contrast:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            let normal_selected = theme.contrast_level == ContrastLevel::Normal;
                            let normal_button = ui.selectable_label(normal_selected, "Normal");
                            if normal_button.clicked() {
                                self.update_theme(|theme| {
                                    theme.contrast_level = ContrastLevel::Normal;
                                });
                            }

                            let medium_selected = theme.contrast_level == ContrastLevel::Medium;
                            let medium_button = ui.selectable_label(medium_selected, "Medium");
                            if medium_button.clicked() {
                                self.update_theme(|theme| {
                                    theme.contrast_level = ContrastLevel::Medium;
                                });
                            }

                            let high_selected = theme.contrast_level == ContrastLevel::High;
                            let high_button = ui.selectable_label(high_selected, "High");
                            if high_button.clicked() {
                                self.update_theme(|theme| {
                                    theme.contrast_level = ContrastLevel::High;
                                });
                            }
                        });
                    });
                });

                ui.add_space(8.0);

                // 49 Color Selectors - always show since we have default theme
                {
                    ui.group(|ui| {
                        ui.label("Material Color Tokens:");
                        ui.add_space(4.0);

                        let color_names = [
                            "primary",
                            "surfaceTint",
                            "onPrimary",
                            "primaryContainer",
                            "onPrimaryContainer",
                            "secondary",
                            "onSecondary",
                            "secondaryContainer",
                            "onSecondaryContainer",
                            "tertiary",
                            "onTertiary",
                            "tertiaryContainer",
                            "onTertiaryContainer",
                            "error",
                            "onError",
                            "errorContainer",
                            "onErrorContainer",
                            "background",
                            "onBackground",
                            "surface",
                            "onSurface",
                            "surfaceVariant",
                            "onSurfaceVariant",
                            "outline",
                            "outlineVariant",
                            "shadow",
                            "scrim",
                            "inverseSurface",
                            "inverseOnSurface",
                            "inversePrimary",
                            "primaryFixed",
                            "onPrimaryFixed",
                            "primaryFixedDim",
                            "onPrimaryFixedVariant",
                            "secondaryFixed",
                            "onSecondaryFixed",
                            "secondaryFixedDim",
                            "onSecondaryFixedVariant",
                            "tertiaryFixed",
                            "onTertiaryFixed",
                            "tertiaryFixedDim",
                            "onTertiaryFixedVariant",
                            "surfaceDim",
                            "surfaceBright",
                            "surfaceContainerLowest",
                            "surfaceContainerLow",
                            "surfaceContainer",
                            "surfaceContainerHigh",
                            "surfaceContainerHighest",
                        ];

                        ui.horizontal_wrapped(|ui| {
                            for color_name in &color_names {
                                let current_color = theme.get_color_by_name(color_name);

                                // Color name label
                                ui.label(*color_name);

                                let mut temp_color = current_color;
                                let color_changed =
                                    ui.color_edit_button_srgba(&mut temp_color).changed();
                                if color_changed {
                                    self.update_theme(|theme| {
                                        theme
                                            .selected_colors
                                            .insert(color_name.to_string(), temp_color);
                                    });
                                }
                                ui.separator();
                            }
                        });
                    });
                }
            });

            ui.add_space(15.0);
            ui.label("Demo Windows:");
            ui.horizontal_wrapped(|ui| {
                if ui.add(MaterialButton::filled("Button Stories")).clicked() {
                    self.button_window.open = true;
                }

                if ui.add(MaterialButton::filled("Checkbox Stories")).clicked() {
                    self.checkbox_window.open = true;
                }

                if ui.add(MaterialButton::filled("Chips Stories")).clicked() {
                    self.chips_window.open = true;
                }

                if ui
                    .add(MaterialButton::filled("Data Table Stories"))
                    .clicked()
                {
                    self.datatable_window.open = true;
                }

                if ui.add(MaterialButton::filled("Dialog Stories")).clicked() {
                    self.dialog_window.open = true;
                }

                if ui.add(MaterialButton::filled("Drawer Stories")).clicked() {
                    self.drawer_window.open = true;
                }

                if ui
                    .add(MaterialButton::filled("Enhanced Card (Card2) Stories"))
                    .clicked()
                {
                    self.card2_window.open = true;
                }

                if ui.add(MaterialButton::filled("FAB Stories")).clicked() {
                    self.fab_window.open = true;
                }

                if ui
                    .add(MaterialButton::filled("Icon Button Stories"))
                    .clicked()
                {
                    self.iconbutton_window.open = true;
                }

                if ui
                    .add(MaterialButton::filled("Image List Stories"))
                    .clicked()
                {
                    self.imagelist_window.open = true;
                }

                if ui
                    .add(MaterialButton::filled("Layout Grid Stories"))
                    .clicked()
                {
                    self.layoutgrid_window.open = true;
                }

                if ui.add(MaterialButton::filled("List Stories")).clicked() {
                    self.list_window.open = true;
                }

                if ui.add(MaterialButton::filled("Menu Stories")).clicked() {
                    self.menu_window.open = true;
                }

                if ui.add(MaterialButton::filled("Progress Stories")).clicked() {
                    self.progress_window.open = true;
                }

                if ui.add(MaterialButton::filled("Radio Stories")).clicked() {
                    self.radio_window.open = true;
                }

                if ui.add(MaterialButton::filled("Select Stories")).clicked() {
                    self.select_window.open = true;
                }

                if ui.add(MaterialButton::filled("Slider Stories")).clicked() {
                    self.slider_window.open = true;
                }

                if ui.add(MaterialButton::filled("Snackbar Stories")).clicked() {
                    self.snackbar_window.open = true;
                }

                if ui.add(MaterialButton::filled("Switch Stories")).clicked() {
                    self.switch_window.open = true;
                }

                if ui.add(MaterialButton::filled("Tabs Stories")).clicked() {
                    self.tabs_window.open = true;
                }

                if ui
                    .add(MaterialButton::filled("Top App Bar Stories"))
                    .clicked()
                {
                    self.topappbar_window.open = true;
                }
            });

            ui.add_space(15.0);
        });

        // Show demo windows
        self.button_window.show(ctx);
        self.checkbox_window.show(ctx);
        self.chips_window.show(ctx);
        self.dialog_window.show(ctx);
        self.fab_window.show(ctx);
        self.iconbutton_window.show(ctx);
        self.list_window.show(ctx);
        self.menu_window.show(ctx);
        self.progress_window.show(ctx);
        self.radio_window.show(ctx);
        self.select_window.show(ctx);
        self.slider_window.show(ctx);
        self.switch_window.show(ctx);
        self.tabs_window.show(ctx);
        self.datatable_window.show(ctx);
        self.drawer_window.show(ctx);
        self.imagelist_window.show(ctx);
        self.layoutgrid_window.show(ctx);
        self.snackbar_window.show(ctx);
        self.topappbar_window.show(ctx);
        self.card2_window.show(ctx);

        // let image_bytes = include_bytes!("../../resources/imgur_image.png");
        // egui::Window::new("Test egui::Image")
        //     .open(&mut true)
        //     .default_size([640.0, 400.0])
        //     .show(ctx, |ui| {
        //         let available_width = ui.available_width();
        //         let target_height = available_width * 9.0 / 16.0; // Force 16:9 aspect ratio
        //         let image_widget = egui::Image::from_bytes("bytes://main_panel_image_imgur", image_bytes)
        //             .fit_to_exact_size(egui::Vec2::new(available_width, target_height));
        //         let _image_response = ui.add(image_widget);
        //     });
    }
}
