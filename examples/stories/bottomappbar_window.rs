#![doc(hidden)]

use crate::{bottom_app_bar, MaterialBottomAppBar, MaterialButton, MaterialCheckbox};
use eframe::egui::{self, Color32, Ui, Window};

#[doc(hidden)]
pub struct BottomAppBarWindow {
    pub open: bool,
    custom_height: f32,
    use_custom_height: bool,
    use_custom_colors: bool,
    show_navigation: bool,
    show_actions: bool,
    show_fab_notch: bool,
    elevation: f32,
    padding_horizontal: f32,
    padding_vertical: f32,
}

impl Default for BottomAppBarWindow {
    fn default() -> Self {
        Self {
            open: false,
            custom_height: 80.0,
            use_custom_height: false,
            use_custom_colors: false,
            show_navigation: true,
            show_actions: true,
            show_fab_notch: false,
            elevation: 3.0,
            padding_horizontal: 16.0,
            padding_vertical: 12.0,
        }
    }
}

impl BottomAppBarWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Bottom App Bar Stories")
            .open(&mut open)
            .default_size([900.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_bottom_app_bar_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.push_id("bottomappbar_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Bottom App Bar Controls");
                if ui
                    .add(MaterialButton::filled("Target").small())
                    .clicked()
                {
                    let _ = webbrowser::open(
                        "https://m3.material.io/components/bottom-app-bar/overview",
                    );
                }
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.show_navigation,
                    "Show Navigation Icon",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.show_actions,
                    "Show Action Icons",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.show_fab_notch,
                    "Show FAB Notch",
                ));
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.use_custom_height,
                    "Custom Height",
                ));
                if self.use_custom_height {
                    ui.add(egui::Slider::new(&mut self.custom_height, 56.0..=120.0).suffix("px"));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Elevation:");
                ui.add(egui::Slider::new(&mut self.elevation, 0.0..=12.0).suffix("dp"));
            });

            ui.horizontal(|ui| {
                ui.label("Padding H:");
                ui.add(egui::Slider::new(&mut self.padding_horizontal, 0.0..=32.0).suffix("px"));
                ui.label("V:");
                ui.add(egui::Slider::new(&mut self.padding_vertical, 0.0..=24.0).suffix("px"));
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.use_custom_colors,
                    "Custom Colors",
                ));
            });
        });
    }

    fn render_bottom_app_bar_examples(&mut self, ui: &mut Ui) {
        // --- Basic Bottom App Bar ---
        ui.heading("Basic Bottom App Bar");
        ui.label("Standard bottom app bar with surfaceContainer background. 80dp height default.");

        let mut basic_bar = bottom_app_bar().id_salt("basic_bottomappbar");

        if self.use_custom_height {
            basic_bar = basic_bar.height(self.custom_height);
        }

        basic_bar = basic_bar
            .elevation(self.elevation)
            .padding(self.padding_horizontal, self.padding_vertical);

        if self.use_custom_colors {
            basic_bar = basic_bar
                .background_color(Color32::from_rgb(103, 80, 164))
                .foreground_color(Color32::WHITE);
        }

        if self.show_navigation {
            basic_bar = basic_bar.navigation_icon("menu", || {
                println!("Menu clicked");
            });
        }

        if self.show_actions {
            basic_bar = basic_bar
                .action_icon("search", || {
                    println!("Search clicked!");
                })
                .action_icon("favorite", || {
                    println!("Favorite clicked!");
                })
                .action_icon("more_vert", || {
                    println!("More clicked!");
                });
        }

        if self.show_fab_notch {
            basic_bar = basic_bar.fab_notch(true);
        }

        ui.add(basic_bar);

        ui.add_space(30.0);

        // --- Navigation Drawer Bar ---
        ui.heading("With Navigation Drawer Icon");
        ui.label("Bottom app bar commonly used with a navigation drawer on mobile.");

        let drawer_bar = MaterialBottomAppBar::new()
            .id_salt("drawer_bottomappbar")
            .elevation(self.elevation)
            .navigation_icon("menu", || println!("Open drawer"))
            .action_icon("search", || println!("Search"))
            .action_icon("share", || println!("Share"));

        ui.add(drawer_bar);

        ui.add_space(30.0);

        // --- Actions Only Bar ---
        ui.heading("Actions Only");
        ui.label("Bottom app bar with only action buttons, no navigation icon.");

        let actions_bar = MaterialBottomAppBar::new()
            .id_salt("actions_bottomappbar")
            .elevation(self.elevation)
            .action_icon("archive", || println!("Archive"))
            .action_icon("delete", || println!("Delete"))
            .action_icon("reply", || println!("Reply"))
            .action_icon("more_vert", || println!("More"));

        ui.add(actions_bar);

        ui.add_space(30.0);

        // --- With FAB Notch ---
        ui.heading("With FAB Notch");
        ui.label(
            "Bottom app bar with a cutout for a floating action button (FAB). \
             The FAB would typically sit at the end.",
        );

        let fab_bar = MaterialBottomAppBar::new()
            .id_salt("fab_bottomappbar")
            .elevation(self.elevation)
            .navigation_icon("menu", || println!("Menu"))
            .action_icon("search", || println!("Search"))
            .action_icon("favorite", || println!("Favorite"))
            .fab_notch(true)
            .notch_margin(8.0);

        ui.add(fab_bar);

        ui.add_space(30.0);

        // --- Compact Height ---
        ui.heading("Compact Height");
        ui.label("Bottom app bar with a smaller height for compact layouts.");

        let compact_bar = MaterialBottomAppBar::new()
            .id_salt("compact_bottomappbar")
            .height(64.0)
            .padding(12.0, 8.0)
            .elevation(self.elevation)
            .navigation_icon("arrow_back", || println!("Back"))
            .action_icon("check", || println!("Confirm"))
            .action_icon("close", || println!("Cancel"));

        ui.add(compact_bar);

        ui.add_space(30.0);

        // --- Custom Colors ---
        ui.heading("Custom Colors");
        ui.label("Bottom app bar with custom background and foreground colors.");

        let custom_bar = MaterialBottomAppBar::new()
            .id_salt("custom_color_bottomappbar")
            .elevation(self.elevation)
            .background_color(Color32::from_rgb(18, 18, 18))
            .foreground_color(Color32::from_rgb(187, 134, 252))
            .navigation_icon("home", || println!("Home"))
            .action_icon("notifications", || println!("Notifications"))
            .action_icon("account_circle", || println!("Account"));

        ui.add(custom_bar);

        ui.add_space(30.0);

        // --- Email App Example ---
        ui.heading("Email App Example");
        ui.label("Typical bottom app bar for an email application with common actions.");

        let email_bar = MaterialBottomAppBar::new()
            .id_salt("email_bottomappbar")
            .elevation(self.elevation)
            .navigation_icon("menu", || println!("Open drawer"))
            .action_icon("archive", || println!("Archive email"))
            .action_icon("delete", || println!("Delete email"))
            .action_icon("mail", || println!("Mark as read"))
            .action_icon("more_vert", || println!("More options"));

        ui.add(email_bar);

        ui.add_space(30.0);

        // --- Shopping App Example ---
        ui.heading("Shopping App Example");
        ui.label("Bottom app bar for a shopping app with cart and filters.");

        let shopping_bar = MaterialBottomAppBar::new()
            .id_salt("shopping_bottomappbar")
            .elevation(self.elevation)
            .background_color(Color32::from_rgb(255, 245, 238))
            .foreground_color(Color32::from_rgb(98, 0, 238))
            .navigation_icon("filter_list", || println!("Filters"))
            .action_icon("favorite_border", || println!("Favorites"))
            .action_icon("shopping_cart", || println!("Cart"))
            .action_icon("more_vert", || println!("More"));

        ui.add(shopping_bar);

        ui.add_space(30.0);

        // --- Zero Elevation ---
        ui.heading("Zero Elevation (Flat)");
        ui.label("Bottom app bar with no shadow, appearing flat against the background.");

        let flat_bar = MaterialBottomAppBar::new()
            .id_salt("flat_bottomappbar")
            .elevation(0.0)
            .navigation_icon("home", || println!("Home"))
            .action_icon("explore", || println!("Explore"))
            .action_icon("bookmark", || println!("Bookmarks"))
            .action_icon("person", || println!("Profile"));

        ui.add(flat_bar);

        ui.add_space(30.0);

        // --- High Elevation ---
        ui.heading("High Elevation");
        ui.label("Bottom app bar with elevated shadow for emphasis.");

        let elevated_bar = MaterialBottomAppBar::new()
            .id_salt("elevated_bottomappbar")
            .elevation(12.0)
            .navigation_icon("menu", || println!("Menu"))
            .action_icon("settings", || println!("Settings"))
            .action_icon("help", || println!("Help"));

        ui.add(elevated_bar);

        ui.add_space(20.0);

        // Interactive controls
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Reset to Defaults")).clicked() {
                self.custom_height = 80.0;
                self.use_custom_height = false;
                self.use_custom_colors = false;
                self.show_navigation = true;
                self.show_actions = true;
                self.show_fab_notch = false;
                self.elevation = 3.0;
                self.padding_horizontal = 16.0;
                self.padding_vertical = 12.0;
            }

            if ui
                .add(MaterialButton::outlined("Toggle Navigation"))
                .clicked()
            {
                self.show_navigation = !self.show_navigation;
            }

            if ui.add(MaterialButton::text("Toggle Actions")).clicked() {
                self.show_actions = !self.show_actions;
            }
        });
    }
}
