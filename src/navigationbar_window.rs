use eframe::egui::{self, Ui, Window};
use crate::{MaterialNavigationBar, NavigationTab, navigation_tab};

pub struct NavigationBarWindow {
    pub open: bool,
    active_index: usize,
    hide_inactive_labels: bool,
    label: String,
    show_badge: bool,
    badge_value: String,
    active_icon: String,
    inactive_icon: String,
}

impl Default for NavigationBarWindow {
    fn default() -> Self {
        Self {
            open: false,
            active_index: 1, // Match Material Web default
            hide_inactive_labels: false,
            label: "Tab".to_string(),
            show_badge: false,
            badge_value: "3".to_string(),
            active_icon: "⭐".to_string(),
            inactive_icon: "☆".to_string(),
        }
    }
}

impl NavigationBarWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Navigation Bar Stories")
            .open(&mut open)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_navigation_bar_example(ui);
                    ui.add_space(30.0);
                    self.render_additional_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Navigation Bar Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://m3.material.io/components/navigation-bar/overview");
            }
        });

        ui.checkbox(&mut self.hide_inactive_labels, "hideInactiveLabels");

        ui.horizontal(|ui| {
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);
        });

        ui.checkbox(&mut self.show_badge, "showBadge");

        ui.horizontal(|ui| {
            ui.label("Badge value:");
            ui.text_edit_singleline(&mut self.badge_value);
        });

        ui.horizontal(|ui| {
            ui.label("Active icon:");
            ui.text_edit_singleline(&mut self.active_icon);
        });

        ui.horizontal(|ui| {
            ui.label("Inactive icon:");
            ui.text_edit_singleline(&mut self.inactive_icon);
        });
    }

    fn render_navigation_bar_example(&mut self, ui: &mut Ui) {
        ui.heading("Navigation Bar");
        ui.label("Interactive navigation bar matching Material Web example:");
        
        ui.add_space(10.0);

        // Create navigation bar with configurable tabs
        let mut nav_bar = MaterialNavigationBar::new(&mut self.active_index)
            .hide_inactive_labels(self.hide_inactive_labels)
            .width(400.0);

        // Add 4 tabs as in the Material Web example
        for i in 0..4 {
            let mut tab = NavigationTab::new(&self.label, &self.active_icon)
                .icons(&self.active_icon, &self.inactive_icon);

            if self.show_badge && !self.badge_value.is_empty() {
                tab = tab.badge(&self.badge_value);
            }

            nav_bar = nav_bar.add_tab(tab);
        }

        ui.add(nav_bar);

        ui.add_space(10.0);
        ui.label(format!("Currently active tab: {}", self.active_index));
    }

    fn render_additional_examples(&mut self, ui: &mut Ui) {
        ui.heading("Additional Examples");

        // Basic navigation bar
        ui.label("Basic navigation bar:");
        let mut basic_active = 0;
        ui.add(MaterialNavigationBar::new(&mut basic_active)
            .tab("Home", "🏠")
            .tab("Search", "🔍")
            .tab("Favorites", "❤️")
            .tab("Profile", "👤")
            .width(400.0));

        ui.add_space(20.0);

        // Navigation bar with badges
        ui.label("Navigation bar with badges:");
        let mut badge_active = 1;
        ui.add(MaterialNavigationBar::new(&mut badge_active)
            .add_tab(navigation_tab("Messages", "💬").badge("5"))
            .add_tab(navigation_tab("Notifications", "🔔").badge("12"))
            .add_tab(navigation_tab("Updates", "🔄").badge("new"))
            .width(400.0));

        ui.add_space(20.0);

        // Navigation bar with hidden inactive labels
        ui.label("Navigation bar with hidden inactive labels:");
        let mut hidden_labels_active = 0;
        ui.add(MaterialNavigationBar::new(&mut hidden_labels_active)
            .tab("Home", "🏠")
            .tab("Explore", "🧭")
            .tab("Library", "📚")
            .tab("Account", "⚙️")
            .hide_inactive_labels(true)
            .width(400.0));

        ui.add_space(20.0);

        // Navigation bar with different icons for active/inactive states
        ui.label("Navigation bar with different active/inactive icons:");
        let mut different_icons_active = 2;
        ui.add(MaterialNavigationBar::new(&mut different_icons_active)
            .tab_with_icons("Home", "🏠", "🏡")
            .tab_with_icons("Search", "🔍", "🔎")
            .tab_with_icons("Bookmarks", "⭐", "☆")
            .tab_with_icons("Settings", "⚙️", "🔧")
            .width(400.0));

        ui.add_space(20.0);

        // Full width navigation bar
        ui.label("Full width navigation bar:");
        let mut full_width_active = 1;
        ui.add(MaterialNavigationBar::new(&mut full_width_active)
            .tab("Overview", "📊")
            .tab("Analytics", "📈")
            .tab("Reports", "📋")
            .tab("Settings", "⚙️"));

        ui.add_space(10.0);

        // Show interaction feedback
        ui.separator();
        ui.label("💡 Tip: Click on different tabs to see the active state change!");
        ui.label("The navigation bar supports:");
        ui.label("• Active and inactive states with different styling");
        ui.label("• Badges with custom values");
        ui.label("• Optional hiding of inactive labels");
        ui.label("• Responsive width adjustment");
        ui.label("• Hover effects");
    }
}