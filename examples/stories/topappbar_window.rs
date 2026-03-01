#![doc(hidden)]

use crate::{
    center_aligned_top_app_bar, large_top_app_bar, medium_top_app_bar, menu, menu_item,
    top_app_bar, MaterialButton, MaterialCheckbox, MaterialTopAppBar,
};
use egui_material3::material_symbol::{
    ICON_ACCOUNT_CIRCLE, ICON_ADD, ICON_ARROW_BACK, ICON_CLOSE, ICON_CLOUD_UPLOAD,
    ICON_DELETE, ICON_DOWNLOAD, ICON_EDIT, ICON_EMAIL, ICON_FAVORITE, ICON_FILTER_LIST,
    ICON_HELP, ICON_IMAGE, ICON_INFO, ICON_MAIL, ICON_MENU, ICON_MORE_VERT, ICON_MUSIC_NOTE,
    ICON_NOTIFICATIONS, ICON_PHOTO, ICON_REFRESH, ICON_SEARCH, ICON_SETTINGS, ICON_SHARE,
    ICON_SORT, ICON_UPLOAD,
};
use eframe::egui::{self, Color32, Rect, Ui, Window};
use std::sync::atomic::{AtomicBool, Ordering};

// Atomic flags for cross-callback state toggling
static SEARCH_TOGGLE: AtomicBool = AtomicBool::new(false);
static MENU_TOGGLE: AtomicBool = AtomicBool::new(false);

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
    // Search state
    search_open: bool,
    search_text: String,
    // Menu state
    menu_open: bool,
    menu_anchor_rect: Option<Rect>,
    menu_selected: String,
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
            search_open: false,
            search_text: String::new(),
            menu_open: false,
            menu_anchor_rect: None,
            menu_selected: String::new(),
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

        // Check atomic toggle flags set by callbacks
        if SEARCH_TOGGLE.swap(false, Ordering::Relaxed) {
            self.search_open = !self.search_open;
            if self.search_open {
                self.search_text.clear();
            }
        }
        if MENU_TOGGLE.swap(false, Ordering::Relaxed) {
            self.menu_open = !self.menu_open;
        }

        self.show_navigation_menu(ctx);
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.push_id("topappbar_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Top App Bar Controls");
                if ui
                    .add(MaterialButton::filled("Target").small())
                    .clicked()
                {
                    let _ =
                        webbrowser::open("https://m3.material.io/components/top-app-bar/overview");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.title_text);
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
                    &mut self.is_scrolled,
                    "Scrolled State",
                ));
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.use_custom_height,
                    "Custom Height",
                ));
                if self.use_custom_height {
                    ui.add(
                        egui::Slider::new(&mut self.custom_height, 48.0..=200.0).suffix("px"),
                    );
                }
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.use_custom_colors,
                    "Custom Colors",
                ));
            });
        });
    }

    fn render_top_app_bar_examples(&mut self, ui: &mut Ui) {
        // --- Regular ---
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
            regular_bar = regular_bar.navigation_icon_char(ICON_MENU, || println!("Navigation clicked!"));
        }

        if self.show_actions {
            regular_bar = regular_bar
                .action_icon_char(ICON_SEARCH, || println!("Search clicked!"))
                .action_icon_char(ICON_NOTIFICATIONS, || println!("Notifications clicked!"))
                .action_icon_char(ICON_ACCOUNT_CIRCLE, || println!("Account clicked!"))
                .action_icon_char(ICON_MORE_VERT, || println!("More clicked!"));
        }

        ui.add(regular_bar);

        ui.add_space(30.0);

        // --- Center Aligned ---
        ui.heading("Center Aligned Top App Bar");
        ui.label("App bar with centered title, typically used for simple layouts.");

        let mut center_bar = center_aligned_top_app_bar(&self.title_text)
            .id_salt("center_topappbar")
            .scrolled(self.is_scrolled);

        if self.use_custom_height {
            center_bar = center_bar.height(self.custom_height);
        }

        if self.show_navigation {
            center_bar = center_bar.navigation_icon_char(ICON_ARROW_BACK, || println!("Back clicked!"));
        }

        if self.show_actions {
            center_bar = center_bar
                .action_icon_char(ICON_SHARE, || println!("Share clicked!"))
                .action_icon_char(ICON_FAVORITE, || println!("Favorite clicked!"))
                .action_icon_char(ICON_MORE_VERT, || println!("More clicked!"));
        }

        ui.add(center_bar);

        ui.add_space(30.0);

        // --- Medium ---
        ui.heading("Medium Top App Bar");
        ui.label(
            "112dp height. Expanded title uses headlineSmall (24sp) with 20dp bottom padding.",
        );

        let mut medium_bar = medium_top_app_bar(&self.title_text)
            .id_salt("medium_topappbar")
            .scrolled(self.is_scrolled);

        if self.show_navigation {
            medium_bar = medium_bar.navigation_icon_char(ICON_MENU, || println!("Navigation clicked!"));
        }

        if self.show_actions {
            medium_bar = medium_bar
                .action_icon_char(ICON_SEARCH, || println!("Search clicked!"))
                .action_icon_char(ICON_SETTINGS, || println!("Settings clicked!"))
                .action_icon_char(ICON_MORE_VERT, || println!("More clicked!"));
        }

        ui.add(medium_bar);

        ui.add_space(30.0);

        // --- Large ---
        ui.heading("Large Top App Bar");
        ui.label(
            "152dp height. Expanded title uses headlineMedium (28sp) with 28dp bottom padding.",
        );

        let mut large_bar = large_top_app_bar(&self.title_text)
            .id_salt("large_topappbar")
            .scrolled(self.is_scrolled);

        if self.show_navigation {
            large_bar = large_bar.navigation_icon_char(ICON_MENU, || println!("Navigation clicked!"));
        }

        if self.show_actions {
            large_bar = large_bar
                .action_icon_char(ICON_SEARCH, || println!("Search clicked!"))
                .action_icon_char(ICON_FAVORITE, || println!("Favorite clicked!"))
                .action_icon_char(ICON_SHARE, || println!("Share clicked!"))
                .action_icon_char(ICON_MORE_VERT, || println!("More clicked!"));
        }

        ui.add(large_bar);

        ui.add_space(30.0);

        // --- About Page Demo ---
        ui.heading("About Page Demo");
        ui.label(
            "Demonstrates TopAppBar in an about page context, inspired by Flutter's AboutDialog.",
        );

        let about_bar = top_app_bar("About")
            .id_salt("about_topappbar")
            .navigation_icon_char(ICON_ARROW_BACK, || println!("About: Back clicked!"))
            .action_icon_char(ICON_MORE_VERT, || println!("About: More clicked!"));

        ui.add(about_bar);

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

        // --- Interactive Demo with search + menu ---
        ui.heading("Interactive Demo");
        ui.label(
            "Click the menu icon to open a navigation menu. Click search to open a search field.",
        );

        let interactive_bar = top_app_bar(&self.title_text)
            .id_salt("interactive_topappbar")
            .navigation_icon_char(ICON_MENU, || {
                MENU_TOGGLE.store(true, Ordering::Relaxed);
            })
            .action_icon_char(ICON_SEARCH, || {
                SEARCH_TOGGLE.store(true, Ordering::Relaxed);
            })
            .action_icon_char(ICON_NOTIFICATIONS, || println!("Interactive: Notifications"))
            .action_icon_char(ICON_ACCOUNT_CIRCLE, || println!("Interactive: Account"))
            .action_icon_char(ICON_MORE_VERT, || println!("Interactive: More"))
            .scrolled(self.is_scrolled);

        let response = ui.add(interactive_bar);

        // Capture the appbar rect for positioning the navigation menu
        self.menu_anchor_rect = Some(Rect::from_min_size(
            response.rect.left_top(),
            egui::vec2(56.0, 64.0),
        ));

        // Show search text field below the bar when active
        if self.search_open {
            ui.horizontal(|ui| {
                ui.label("Search:");
                let te_response = ui.text_edit_singleline(&mut self.search_text);
                if te_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    println!("Search for: {}", self.search_text);
                }
                if ui.add(MaterialButton::text("Close")).clicked() {
                    self.search_open = false;
                    self.search_text.clear();
                }
            });
        }

        if !self.menu_selected.is_empty() {
            ui.label(format!("Menu selected: {}", self.menu_selected));
        }

        ui.add_space(20.0);

        // Buttons that affect the bars above
        ui.horizontal(|ui| {
            if ui
                .add(MaterialButton::filled("Simulate Scroll"))
                .clicked()
            {
                self.is_scrolled = !self.is_scrolled;
            }
            if ui.add(MaterialButton::outlined("Reset Title")).clicked() {
                self.title_text = "My Application".to_string();
            }
            if ui.add(MaterialButton::text("Long Title")).clicked() {
                self.title_text = "Very Long Application Title That Might Overflow".to_string();
            }
        });

        ui.add_space(30.0);

        // --- Color Override Demo ---
        ui.heading("Color Override Demo");
        ui.label(
            "Custom background and foreground colors using background_color/foreground_color.",
        );

        let custom_bar = MaterialTopAppBar::regular("Custom Colors")
            .id_salt("custom_color_topappbar")
            .background_color(Color32::from_rgb(103, 80, 164))
            .foreground_color(Color32::WHITE)
            .navigation_icon_char(ICON_ARROW_BACK, || println!("Custom: Back clicked!"))
            .action_icon_char(ICON_SEARCH, || println!("Custom: Search clicked!"))
            .action_icon_char(ICON_MORE_VERT, || println!("Custom: More clicked!"));

        ui.add(custom_bar);

        ui.add_space(30.0);

        // --- Mail App Demo ---
        ui.heading("Mail App Example");
        ui.label(
            "Email/Mail application with compose, refresh, and search actions.",
        );

        let mail_bar = top_app_bar("Inbox")
            .id_salt("mail_topappbar")
            .navigation_icon_char(ICON_MENU, || println!("Mail: Menu clicked!"))
            .action_icon_char(ICON_SEARCH, || println!("Mail: Search clicked!"))
            .action_icon_char(ICON_REFRESH, || println!("Mail: Refresh clicked!"))
            .action_icon_char(ICON_MORE_VERT, || println!("Mail: More clicked!"));

        ui.add(mail_bar);

        ui.add_space(30.0);

        // --- Photo Gallery Demo ---
        ui.heading("Photo Gallery Example");
        ui.label(
            "Photo gallery app with upload, filter, and sort capabilities.",
        );

        let photo_bar = top_app_bar("Photos")
            .id_salt("photo_topappbar")
            .navigation_icon_char(ICON_MENU, || println!("Photos: Menu clicked!"))
            .action_icon_char(ICON_CLOUD_UPLOAD, || println!("Photos: Upload clicked!"))
            .action_icon_char(ICON_FILTER_LIST, || println!("Photos: Filter clicked!"))
            .action_icon_char(ICON_SORT, || println!("Photos: Sort clicked!"))
            .action_icon_char(ICON_MORE_VERT, || println!("Photos: More clicked!"));

        ui.add(photo_bar);

        ui.add_space(30.0);

        // --- Music Player Demo ---
        ui.heading("Music Player Example");
        ui.label(
            "Music player with search and favorite actions.",
        );

        let music_bar = top_app_bar("Now Playing")
            .id_salt("music_topappbar")
            .navigation_icon_char(ICON_ARROW_BACK, || println!("Music: Back clicked!"))
            .action_icon_char(ICON_SEARCH, || println!("Music: Search clicked!"))
            .action_icon_char(ICON_FAVORITE, || println!("Music: Favorite clicked!"))
            .action_icon_char(ICON_SHARE, || println!("Music: Share clicked!"))
            .action_icon_char(ICON_MORE_VERT, || println!("Music: More clicked!"));

        ui.add(music_bar);

        ui.add_space(30.0);

        // --- Editor Demo ---
        ui.heading("Document Editor Example");
        ui.label(
            "Document editor with edit, upload, and download actions.",
        );

        let editor_bar = top_app_bar("Document.txt")
            .id_salt("editor_topappbar")
            .navigation_icon_char(ICON_CLOSE, || println!("Editor: Close clicked!"))
            .action_icon_char(ICON_DOWNLOAD, || println!("Editor: Download clicked!"))
            .action_icon_char(ICON_UPLOAD, || println!("Editor: Upload clicked!"))
            .action_icon_char(ICON_SHARE, || println!("Editor: Share clicked!"))
            .action_icon_char(ICON_MORE_VERT, || println!("Editor: More clicked!"));

        ui.add(editor_bar);

        ui.add_space(30.0);

        // --- Detail View Demo ---
        ui.heading("Detail View Example");
        ui.label(
            "Detail view with edit, delete, and info actions.",
        );

        let detail_bar = top_app_bar("Item Details")
            .id_salt("detail_topappbar")
            .navigation_icon_char(ICON_ARROW_BACK, || println!("Detail: Back clicked!"))
            .action_icon_char(ICON_EDIT, || println!("Detail: Edit clicked!"))
            .action_icon_char(ICON_DELETE, || println!("Detail: Delete clicked!"))
            .action_icon_char(ICON_INFO, || println!("Detail: Info clicked!"));

        ui.add(detail_bar);

        ui.add_space(30.0);

        // --- Help Page Demo ---
        ui.heading("Help Page Example");
        ui.label(
            "Help page with back navigation and search.",
        );

        let help_bar = center_aligned_top_app_bar("Help & Support")
            .id_salt("help_topappbar")
            .navigation_icon_char(ICON_ARROW_BACK, || println!("Help: Back clicked!"))
            .action_icon_char(ICON_SEARCH, || println!("Help: Search clicked!"));

        ui.add(help_bar);
    }

    fn show_navigation_menu(&mut self, ctx: &egui::Context) {
        if !self.menu_open {
            return;
        }

        let home_item = menu_item("Home")
            .leading_icon("home")
            .on_click(|| println!("Home clicked!"));
        let profile_item = menu_item("Profile")
            .leading_icon("person")
            .on_click(|| println!("Profile clicked!"));
        let settings_item = menu_item("Settings")
            .leading_icon("settings")
            .on_click(|| println!("Settings clicked!"));
        let help_item = menu_item("Help & Feedback")
            .leading_icon("help")
            .on_click(|| println!("Help & Feedback clicked!"));

        let mut menu_builder = menu("topappbar_nav_menu", &mut self.menu_open)
            .item(home_item)
            .item(profile_item)
            .item(menu_item("").divider_after(true))
            .item(settings_item)
            .item(help_item);

        if let Some(rect) = self.menu_anchor_rect {
            menu_builder = menu_builder.anchor_rect(rect);
        }

        menu_builder.show(ctx);
    }
}
