#![doc(hidden)]

use crate::{
    center_aligned_top_app_bar, large_top_app_bar, medium_top_app_bar, top_app_bar, MaterialButton,
    MaterialCheckbox, MaterialTopAppBar,
};
use eframe::egui::{self, Color32, Ui, Window};

#[doc(hidden)]
pub struct TopAppBarWindow {
    pub open: bool,
    title_text: String,
    show_navigation: bool,
    show_actions: bool,
    is_scrolled: bool,
    custom_height: f32,
    use_custom_height: bool,
    use_custom_colors: bool,
}

impl Default for TopAppBarWindow {
    fn default() -> Self {
        Self {
            open: false,
            title_text: "My Application".to_string(),
            show_navigation: true,
            show_actions: true,
            is_scrolled: false,
            custom_height: 64.0,
            use_custom_height: false,
            use_custom_colors: false,
        }
    }
}

impl TopAppBarWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Top App Bar Stories")
            .open(&mut open)
            .default_size([1000.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_top_app_bar_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.push_id("topappbar_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Top App Bar Controls");
                if ui.add(MaterialButton::filled("Target").small()).clicked() {
                    let _ = webbrowser::open("https://m3.material.io/components/top-app-bar/overview");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.title_text);
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.show_navigation, "Show Navigation Icon"));
                ui.add(MaterialCheckbox::new(&mut self.show_actions, "Show Action Icons"));
                ui.add(MaterialCheckbox::new(&mut self.is_scrolled, "Scrolled State"));
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.use_custom_height, "Custom Height"));
                if self.use_custom_height {
                    ui.add(egui::Slider::new(&mut self.custom_height, 48.0..=200.0).suffix("px"));
                }
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.use_custom_colors, "Custom Colors"));
            });
        });
    }

    fn render_top_app_bar_examples(&mut self, ui: &mut Ui) {
        ui.heading("Regular Top App Bar");
        ui.label("Standard app bar with surface color background. 64dp height, titleLarge (22sp).");

        let mut regular_bar = top_app_bar(&self.title_text)
            .id_salt("regular_topappbar")
            .scrolled(self.is_scrolled);

        if self.use_custom_height {
            regular_bar = regular_bar.height(self.custom_height);
        }

        if self.use_custom_colors {
            regular_bar = regular_bar
                .background_color(Color32::from_rgb(103, 80, 164))
                .foreground_color(Color32::WHITE);
        }

        if self.show_navigation {
            regular_bar = regular_bar.navigation_icon("menu", || println!("Navigation clicked!"));
        }

        if self.show_actions {
            regular_bar = regular_bar
                .action_icon("search", || println!("Search clicked!"))
                .action_icon("notifications", || println!("Notifications clicked!"))
                .action_icon("account_circle", || println!("Account clicked!"))
                .action_icon("more_vert", || println!("More clicked!"));
        }

        ui.add(regular_bar);

        ui.add_space(30.0);

        ui.heading("Center Aligned Top App Bar");
        ui.label("App bar with centered title, typically used for simple layouts.");

        let mut center_bar = center_aligned_top_app_bar(&self.title_text)
            .id_salt("center_topappbar")
            .scrolled(self.is_scrolled);

        if self.use_custom_height {
            center_bar = center_bar.height(self.custom_height);
        }

        if self.show_navigation {
            center_bar = center_bar.navigation_icon("arrow_back", || println!("Back clicked!"));
        }

        if self.show_actions {
            center_bar = center_bar
                .action_icon("share", || println!("Share clicked!"))
                .action_icon("favorite", || println!("Favorite clicked!"))
                .action_icon("more_vert", || println!("More clicked!"));
        }

        ui.add(center_bar);

        ui.add_space(30.0);

        ui.heading("Medium Top App Bar");
        ui.label("112dp height. Expanded title uses headlineSmall (24sp) with 20dp bottom padding.");

        let mut medium_bar = medium_top_app_bar(&self.title_text)
            .id_salt("medium_topappbar")
            .scrolled(self.is_scrolled);

        if self.show_navigation {
            medium_bar = medium_bar.navigation_icon("menu", || println!("Navigation clicked!"));
        }

        if self.show_actions {
            medium_bar = medium_bar
                .action_icon("search", || println!("Search clicked!"))
                .action_icon("settings", || println!("Settings clicked!"))
                .action_icon("more_vert", || println!("More clicked!"));
        }

        ui.add(medium_bar);

        ui.add_space(30.0);

        ui.heading("Large Top App Bar");
        ui.label("152dp height. Expanded title uses headlineMedium (28sp) with 28dp bottom padding.");

        let mut large_bar = large_top_app_bar(&self.title_text)
            .id_salt("large_topappbar")
            .scrolled(self.is_scrolled);

        if self.show_navigation {
            large_bar = large_bar.navigation_icon("menu", || println!("Navigation clicked!"));
        }

        if self.show_actions {
            large_bar = large_bar
                .action_icon("search", || println!("Search clicked!"))
                .action_icon("favorite", || println!("Favorite clicked!"))
                .action_icon("share", || println!("Share clicked!"))
                .action_icon("more_vert", || println!("More clicked!"));
        }

        ui.add(large_bar);

        ui.add_space(30.0);

        ui.heading("About Page Demo");
        ui.label("Demonstrates TopAppBar in an about page context, inspired by Flutter's AboutDialog.");

        let about_bar = top_app_bar("About")
            .id_salt("about_topappbar")
            .navigation_icon("arrow_back", || println!("About: Back clicked!"))
            .action_icon("more_vert", || println!("About: More clicked!"));

        ui.add(about_bar);

        // Simulated about content below the app bar
        ui.push_id("about_content", |ui| {
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(24.0);
                ui.vertical(|ui| {
                    ui.heading("egui-material3");
                    ui.label("Version 0.1.0");
                    ui.add_space(8.0);
                    ui.label("Material Design 3 components for egui.");
                    ui.add_space(4.0);
                    ui.label("Copyright 2024. All rights reserved.");
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        if ui.add(MaterialButton::text("View Licenses")).clicked() {
                            println!("View Licenses clicked!");
                        }
                        if ui.add(MaterialButton::text("Close")).clicked() {
                            println!("Close clicked!");
                        }
                    });
                });
            });
        });

        ui.add_space(30.0);

        ui.heading("Scrolled-Under Demo");
        ui.label("Toggle scroll state to see background change from surface to surfaceContainer.");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Simulate Scroll")).clicked() {
                self.is_scrolled = !self.is_scrolled;
            }
            if ui.add(MaterialButton::outlined("Reset Title")).clicked() {
                self.title_text = "My Application".to_string();
            }
            if ui.add(MaterialButton::text("Long Title")).clicked() {
                self.title_text = "Very Long Application Title That Might Overflow".to_string();
            }
        });

        ui.add_space(10.0);

        let demo_bar = top_app_bar("Demo App")
            .id_salt("demo_topappbar")
            .navigation_icon("menu", || println!("Demo: Menu clicked!"))
            .action_icon("notifications", || println!("Demo: Notifications clicked!"))
            .action_icon("settings", || println!("Demo: Settings clicked!"))
            .action_icon("account_circle", || println!("Demo: Account clicked!"))
            .scrolled(self.is_scrolled);

        ui.add(demo_bar);

        ui.add_space(30.0);

        ui.heading("Color Override Demo");
        ui.label("Custom background and foreground colors using background_color/foreground_color.");

        let custom_bar = MaterialTopAppBar::regular("Custom Colors")
            .id_salt("custom_color_topappbar")
            .background_color(Color32::from_rgb(103, 80, 164))
            .foreground_color(Color32::WHITE)
            .navigation_icon("arrow_back", || println!("Custom: Back clicked!"))
            .action_icon("search", || println!("Custom: Search clicked!"))
            .action_icon("more_vert", || println!("Custom: More clicked!"));

        ui.add(custom_bar);

        ui.add_space(30.0);

        ui.heading("Usage Guidelines");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Regular:");
                ui.label("  Most common variant");
                ui.label("  Surface color background");
                ui.label("  Good for most apps");
                ui.label("  Standard 64dp height");
            });

            ui.vertical(|ui| {
                ui.label("Center Aligned:");
                ui.label("  Simple layouts");
                ui.label("  Centered title");
                ui.label("  Minimal action icons");
                ui.label("  Clean aesthetic");
            });

            ui.vertical(|ui| {
                ui.label("Medium/Large:");
                ui.label("  Landing pages");
                ui.label("  Surface color background");
                ui.label("  Strong visual hierarchy");
                ui.label("  Expanded title area");
            });
        });

        ui.add_space(20.0);

        ui.heading("M3 Color Spec");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Default:");
                ui.label("  Background: surface");
                ui.label("  Title: onSurface");
                ui.label("  Leading icon: onSurface");
                ui.label("  Action icons: onSurfaceVariant");
            });

            ui.vertical(|ui| {
                ui.label("Scrolled Under:");
                ui.label("  Background: surfaceContainer");
                ui.label("  Elevation: 3dp shadow");
                ui.label("  Other colors unchanged");
            });
        });
    }
}
