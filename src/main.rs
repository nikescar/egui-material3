use eframe::egui::{self, Color32, Vec2};
use egui_material::*;
use std::sync::{Arc, Mutex};

/// Global theme context that can be shared across all Material components
#[derive(Clone, Debug)]
pub struct MaterialThemeContext {
    pub theme_mode: ThemeMode,
    pub source_color: Color32,
    pub hue: f32,
    pub chroma: f32,
    pub tone: f32,
}

impl Default for MaterialThemeContext {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Auto,
            source_color: Color32::from_rgb(103, 80, 164), // Material Purple default
            hue: 260.0,
            chroma: 36.0,
            tone: 40.0,
        }
    }
}

impl MaterialThemeContext {
    pub fn get_primary_color(&self) -> Color32 {
        self.source_color
    }
    
    pub fn get_secondary_color(&self) -> Color32 {
        self.adjust_color_hue(self.source_color, 30.0)
    }
    
    pub fn get_tertiary_color(&self) -> Color32 {
        self.adjust_color_hue(self.source_color, -30.0)
    }
    
    pub fn get_surface_color(&self, dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(24, 24, 24)
        } else {
            Color32::from_rgb(249, 249, 249)
        }
    }
    
    pub fn get_on_primary_color(&self) -> Color32 {
        if self.is_dark_color(self.source_color) { 
            Color32::WHITE 
        } else { 
            Color32::BLACK 
        }
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
    
    fn is_dark_color(&self, color: Color32) -> bool {
        let luminance = 0.299 * color.r() as f32 + 0.587 * color.g() as f32 + 0.114 * color.b() as f32;
        luminance < 128.0
    }
}

// Global theme context accessible by all components
static GLOBAL_THEME: std::sync::LazyLock<Arc<Mutex<MaterialThemeContext>>> = 
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(MaterialThemeContext::default())));

pub fn get_global_theme() -> Arc<Mutex<MaterialThemeContext>> {
    GLOBAL_THEME.clone()
}

pub fn update_global_theme(theme: MaterialThemeContext) {
    if let Ok(mut global_theme) = GLOBAL_THEME.lock() {
        *global_theme = theme;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ThemeMode {
    Light,
    Dark,
    Auto,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Auto
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Material Design Components Demo",
        options,
        Box::new(|_cc| Ok(Box::<MaterialApp>::default())),
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
    ripple: MaterialRipple,
    // Theme changer controls (now using global theme)
    color_picker_open: bool,
    // Demo windows
    button_window: ButtonWindow,
    card_window: CardWindow,
    checkbox_window: CheckboxWindow,
    chips_window: ChipsWindow,
    dialog_window: DialogWindow,
    divider_window: DividerWindow,
    elevation_window: ElevationWindow,
    fab_window: FabWindow,
    iconbutton_window: IconButtonWindow,
    field_window: FieldWindow,
    focus_window: FocusWindow,
    item_window: ItemWindow,
    list_window: ListWindow,
    menu_window: MenuWindow,
    navigationbar_window: NavigationBarWindow,
    progress_window: ProgressWindow,
    radio_window: RadioWindow,
    ripple_window: RippleWindow,
    segmentedbuttonset_window: SegmentedButtonSetWindow,
    select_window: SelectWindow,
    slider_window: SliderWindow,
    switch_window: SwitchWindow,
    tabs_window: TabsWindow,
    textfield_window: TextFieldWindow,
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
            ripple: MaterialRipple::default(),
            color_picker_open: false,
            button_window: ButtonWindow::default(),
            card_window: CardWindow::default(),
            checkbox_window: CheckboxWindow::default(),
            chips_window: ChipsWindow::default(),
            dialog_window: DialogWindow::default(),
            divider_window: DividerWindow::default(),
            elevation_window: ElevationWindow::default(),
            fab_window: FabWindow::default(),
            iconbutton_window: IconButtonWindow::default(),
            field_window: FieldWindow::default(),
            focus_window: FocusWindow::default(),
            item_window: ItemWindow::default(),
            list_window: ListWindow::default(),
            menu_window: MenuWindow::default(),
            navigationbar_window: NavigationBarWindow::default(),
            progress_window: ProgressWindow::default(),
            radio_window: RadioWindow::default(),
            ripple_window: RippleWindow::default(),
            segmentedbuttonset_window: SegmentedButtonSetWindow::default(),
            select_window: SelectWindow::default(),
            slider_window: SliderWindow::default(),
            switch_window: SwitchWindow::default(),
            tabs_window: TabsWindow::default(),
            textfield_window: TextFieldWindow::default(),
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
        F: FnOnce(&mut MaterialThemeContext)
    {
        if let Ok(mut theme) = get_global_theme().lock() {
            update_fn(&mut *theme);
        }
    }

    fn update_hct_from_color(&self) {
        let theme = self.get_theme();
        let source_color = theme.source_color;
        
        // Convert RGB to HSV (approximation of HCT)
        let r = source_color.r() as f32 / 255.0;
        let g = source_color.g() as f32 / 255.0;
        let b = source_color.b() as f32 / 255.0;
        
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        // Hue
        let hue = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        
        let hue = if hue < 0.0 { hue + 360.0 } else { hue };
        
        // Chroma (approximated as saturation * 150)
        let chroma = if max == 0.0 { 0.0 } else { (delta / max) * 150.0 };
        
        // Tone (approximated as value * 100)
        let tone = max * 100.0;
        
        self.update_theme(|theme| {
            theme.hue = hue;
            theme.chroma = chroma;
            theme.tone = tone;
        });
    }
    
    fn update_color_from_hct(&self) {
        let theme = self.get_theme();
        
        // Convert HCT (approximated as HSV) to RGB
        let h = theme.hue / 60.0;
        let s = (theme.chroma / 150.0).clamp(0.0, 1.0);
        let v = (theme.tone / 100.0).clamp(0.0, 1.0);
        
        let c = v * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = v - c;
        
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
        
        let source_color = Color32::from_rgb(r, g, b);
        
        self.update_theme(|theme| {
            theme.source_color = source_color;
        });
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
        
        // Generate Material Design 3 color scheme from source color
        let primary_color = theme.get_primary_color();
        
        // Create color variants for Material Design system
        let _primary_light = self.lighten_color(primary_color, 0.2);
        let _primary_dark = self.darken_color(primary_color, 0.2);
        let on_primary = theme.get_on_primary_color();
        
        // Secondary colors (derived from primary with adjusted hue/chroma)
        let _secondary_color = theme.get_secondary_color();
        let _tertiary_color = theme.get_tertiary_color();
        
        // Surface colors
        let _surface_tint = Color32::from_rgba_unmultiplied(
            primary_color.r(),
            primary_color.g(),
            primary_color.b(),
            10,
        );
        
        // Apply colors to visuals
        visuals.selection.bg_fill = primary_color;
        visuals.selection.stroke.color = primary_color;
        visuals.hyperlink_color = primary_color;
        
        // Button and widget colors
        visuals.widgets.noninteractive.bg_fill = if visuals.dark_mode {
            Color32::from_gray(30)
        } else {
            Color32::from_gray(245)
        };
        
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
        
        // Window background tinting
        if visuals.dark_mode {
            visuals.window_fill = Color32::from_rgb(16, 16, 16);
            visuals.panel_fill = Color32::from_rgb(24, 24, 24);
        } else {
            visuals.window_fill = Color32::from_rgb(254, 247, 255);
            visuals.panel_fill = Color32::from_rgb(249, 249, 249);
        }
        
        // Apply surface tinting
        visuals.extreme_bg_color = if visuals.dark_mode {
            Color32::from_rgb(16, 16, 20)
        } else {
            Color32::from_rgb(255, 251, 254)
        };
        
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
        let luminance = 0.299 * color.r() as f32 + 0.587 * color.g() as f32 + 0.114 * color.b() as f32;
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
}

impl eframe::App for MaterialApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme based on settings
        self.apply_theme(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = self.get_theme();
            
            ui.heading("Material Design Components Demo");
            ui.add_space(10.0);
            
            // Enhanced Theme Changer Controls (Material Design Style)
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Theme Controls");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📋").on_hover_text("Copy current theme to clipboard").clicked() {
                            let theme_string = format!(
                                "Theme: {:?}, Source Color: #{:02X}{:02X}{:02X}, Hue: {:.0}°, Chroma: {:.0}, Tone: {:.0}",
                                theme.theme_mode, 
                                theme.source_color.r(), 
                                theme.source_color.g(), 
                                theme.source_color.b(),
                                theme.hue,
                                theme.chroma, 
                                theme.tone
                            );
                            ui.ctx().copy_text(theme_string);
                        }
                    });
                });
                
                ui.add_space(8.0);
                
                // Hex Source Color Section
                ui.group(|ui| {
                    let mut style = (**ui.style()).clone();
                    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgba_unmultiplied(
                        theme.source_color.r(),
                        theme.source_color.g(),
                        theme.source_color.b(),
                        20
                    );
                    ui.set_style(style);
                    
                    ui.horizontal(|ui| {
                        ui.label("Hex Source Color");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Color picker button styled as circular
                            let color_button_size = Vec2::splat(32.0);
                            let (rect, response) = ui.allocate_exact_size(color_button_size, egui::Sense::click());
                            
                            if ui.is_rect_visible(rect) {
                                let painter = ui.painter();
                                
                                // Draw circular color swatch
                                painter.circle_filled(
                                    rect.center(),
                                    rect.width() * 0.5,
                                    theme.source_color,
                                );
                                
                                // Draw border
                                painter.circle_stroke(
                                    rect.center(),
                                    rect.width() * 0.5,
                                    egui::Stroke::new(1.0, Color32::from_gray(128)),
                                );
                                
                                // Draw focus ring if hovered
                                if response.hovered() {
                                    painter.circle_stroke(
                                        rect.center(),
                                        rect.width() * 0.5 + 2.0,
                                        egui::Stroke::new(2.0, theme.source_color),
                                    );
                                }
                            }
                            
                            if response.clicked() {
                                self.color_picker_open = !self.color_picker_open;
                            }
                            
                            // Hex value display
                            ui.monospace(format!("#{:02X}{:02X}{:02X}", 
                                theme.source_color.r(), 
                                theme.source_color.g(), 
                                theme.source_color.b()));
                        });
                    });
                    
                    if self.color_picker_open {
                        ui.separator();
                        
                        // Color picker with immediate updates
                        let mut temp_color = theme.source_color;
                        let color_changed = ui.color_edit_button_srgba(&mut temp_color).changed();
                        if color_changed {
                            self.update_theme(|theme| {
                                theme.source_color = temp_color;
                            });
                            self.update_hct_from_color();
                        }
                    }
                });
                
                ui.add_space(8.0);
                
                // HCT Sliders Section  
                ui.group(|ui| {
                    let mut style = (**ui.style()).clone();
                    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgba_unmultiplied(
                        theme.source_color.r(),
                        theme.source_color.g(),
                        theme.source_color.b(),
                        20
                    );
                    ui.set_style(style);
                    
                    ui.label("HCT Color Space Controls");
                    
                    ui.add_space(4.0);
                    
                    // Hue Slider with gradient background
                    ui.horizontal(|ui| {
                        ui.label("Hue");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("{:.0}°", theme.hue));
                            let mut temp_hue = theme.hue;
                            let hue_response = ui.add_sized(
                                [200.0, 20.0],
                                egui::Slider::new(&mut temp_hue, 0.0..=360.0)
                                    .show_value(false)
                            );
                            
                            if hue_response.changed() {
                                self.update_theme(|theme| {
                                    theme.hue = temp_hue;
                                });
                                self.update_color_from_hct();
                            }
                        });
                    });
                    
                    // Chroma Slider
                    ui.horizontal(|ui| {
                        ui.label("Chroma");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("{:.0}", theme.chroma));
                            let mut temp_chroma = theme.chroma;
                            let chroma_response = ui.add_sized(
                                [200.0, 20.0],
                                egui::Slider::new(&mut temp_chroma, 0.0..=150.0)
                                    .show_value(false)
                            );
                            
                            if chroma_response.changed() {
                                self.update_theme(|theme| {
                                    theme.chroma = temp_chroma;
                                });
                                self.update_color_from_hct();
                            }
                        });
                    });
                    
                    // Tone Slider
                    ui.horizontal(|ui| {
                        ui.label("Tone");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("{:.0}", theme.tone));
                            let mut temp_tone = theme.tone;
                            let tone_response = ui.add_sized(
                                [200.0, 20.0],
                                egui::Slider::new(&mut temp_tone, 0.0..=100.0)
                                    .show_value(false)
                            );
                            
                            if tone_response.changed() {
                                self.update_theme(|theme| {
                                    theme.tone = temp_tone;
                                });
                                self.update_color_from_hct();
                            }
                        });
                    });
                });
                
                ui.add_space(8.0);
                
                // Theme Mode Selector (Segmented Button Style)
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
                
                // Color Preview Section
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Color Preview:");
                    
                    // Primary color swatch
                    let swatch_size = Vec2::new(24.0, 24.0);
                    let (rect, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
                    if ui.is_rect_visible(rect) {
                        ui.painter().rect_filled(rect, 4.0, theme.source_color);
                        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, Color32::GRAY), egui::epaint::StrokeKind::Middle);
                    }
                    
                    // Secondary colors
                    let secondary = theme.get_secondary_color();
                    let (rect, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
                    if ui.is_rect_visible(rect) {
                        ui.painter().rect_filled(rect, 4.0, secondary);
                        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, Color32::GRAY), egui::epaint::StrokeKind::Middle);
                    }
                    
                    let tertiary = theme.get_tertiary_color();
                    let (rect, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
                    if ui.is_rect_visible(rect) {
                        ui.painter().rect_filled(rect, 4.0, tertiary);
                        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, Color32::GRAY), egui::epaint::StrokeKind::Middle);
                    }
                });
            });
            
            ui.add_space(15.0);
            ui.label("Demo Windows:");
            ui.horizontal_wrapped(|ui| {
                if ui.add(MaterialButton::filled("Button Stories")).clicked() {
                    self.button_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Card Stories")).clicked() {
                    self.card_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Checkbox Stories")).clicked() {
                    self.checkbox_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Chips Stories")).clicked() {
                    self.chips_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Dialog Stories")).clicked() {
                    self.dialog_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Divider Stories")).clicked() {
                    self.divider_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Elevation Stories")).clicked() {
                    self.elevation_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("FAB Stories")).clicked() {
                    self.fab_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Icon Button Stories")).clicked() {
                    self.iconbutton_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Field Stories")).clicked() {
                    self.field_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Focus Stories")).clicked() {
                    self.focus_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Item Stories")).clicked() {
                    self.item_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("List Stories")).clicked() {
                    self.list_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Menu Stories")).clicked() {
                    self.menu_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Navigation Bar Stories")).clicked() {
                    self.navigationbar_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Progress Stories")).clicked() {
                    self.progress_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Radio Stories")).clicked() {
                    self.radio_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Ripple Stories")).clicked() {
                    self.ripple_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Segmented Button Set Stories")).clicked() {
                    self.segmentedbuttonset_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Select Stories")).clicked() {
                    self.select_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Slider Stories")).clicked() {
                    self.slider_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Switch Stories")).clicked() {
                    self.switch_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Tabs Stories")).clicked() {
                    self.tabs_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Text Field Stories")).clicked() {
                    self.textfield_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Data Table Stories")).clicked() {
                    self.datatable_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Drawer Stories")).clicked() {
                    self.drawer_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Image List Stories")).clicked() {
                    self.imagelist_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Layout Grid Stories")).clicked() {
                    self.layoutgrid_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Snackbar Stories")).clicked() {
                    self.snackbar_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Top App Bar Stories")).clicked() {
                    self.topappbar_window.open = true;
                }
                
                if ui.add(MaterialButton::filled("Enhanced Card (Card2) Stories")).clicked() {
                    self.card2_window.open = true;
                }
            });
        });

        // Show demo windows
        self.button_window.show(ctx);
        self.card_window.show(ctx);
        self.checkbox_window.show(ctx);
        self.chips_window.show(ctx);
        self.dialog_window.show(ctx);
        self.divider_window.show(ctx);
        self.elevation_window.show(ctx);
        self.fab_window.show(ctx);
        self.iconbutton_window.show(ctx);
        self.field_window.show(ctx);
        self.focus_window.show(ctx);
        self.item_window.show(ctx);
        self.list_window.show(ctx);
        self.menu_window.show(ctx);
        self.navigationbar_window.show(ctx);
        self.progress_window.show(ctx);
        self.radio_window.show(ctx);
        self.ripple_window.show(ctx);
        self.segmentedbuttonset_window.show(ctx);
        self.select_window.show(ctx);
        self.slider_window.show(ctx);
        self.switch_window.show(ctx);
        self.tabs_window.show(ctx);
        self.textfield_window.show(ctx);
        self.datatable_window.show(ctx);
        self.drawer_window.show(ctx);
        self.imagelist_window.show(ctx);
        self.layoutgrid_window.show(ctx);
        self.snackbar_window.show(ctx);
        self.topappbar_window.show(ctx);
        self.card2_window.show(ctx);

    }
}
