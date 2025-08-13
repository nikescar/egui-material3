use eframe::egui::{self, Ui, Window};
use crate::{MaterialTopAppBar, TopAppBarVariant, MaterialButton, MaterialCheckbox, top_app_bar, center_aligned_top_app_bar, medium_top_app_bar, large_top_app_bar};

pub struct TopAppBarWindow {
    pub open: bool,
    title_text: String,
    show_navigation: bool,
    show_actions: bool,
    is_scrolled: bool,
    custom_height: f32,
    use_custom_height: bool,
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
        ui.horizontal(|ui| {
            ui.heading("Top App Bar Controls");
            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/top-app-bar/stories/");
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
    }

    fn render_top_app_bar_examples(&mut self, ui: &mut Ui) {
        ui.heading("Regular Top App Bar");
        ui.label("Standard app bar with consistent height and surface color.");
        
        let mut regular_bar = top_app_bar(&self.title_text)
            .scrolled(self.is_scrolled);
            
        if self.use_custom_height {
            regular_bar = regular_bar.height(self.custom_height);
        }
        
        if self.show_navigation {
            regular_bar = regular_bar.navigation_icon("menu", || println!("Navigation clicked!"));
        }
        
        if self.show_actions {
            regular_bar = regular_bar
                .action_icon("search", || println!("Search clicked!"))
                .action_icon("favorite", || println!("Favorite clicked!"))
                .action_icon("more_vert", || println!("More clicked!"));
        }
        
        ui.add(regular_bar);

        ui.add_space(30.0);
        
        ui.heading("Center Aligned Top App Bar");
        ui.label("App bar with centered title, typically used for simple layouts.");
        
        let mut center_bar = center_aligned_top_app_bar(&self.title_text)
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
                .action_icon("more_vert", || println!("More clicked!"));
        }
        
        ui.add(center_bar);

        ui.add_space(30.0);
        
        ui.heading("Medium Top App Bar");
        ui.label("Taller app bar with title at the bottom, provides more visual weight.");
        
        let mut medium_bar = medium_top_app_bar(&self.title_text)
            .scrolled(self.is_scrolled);
            
        if self.show_navigation {
            medium_bar = medium_bar.navigation_icon("menu", || println!("Navigation clicked!"));
        }
        
        if self.show_actions {
            medium_bar = medium_bar
                .action_icon("search", || println!("Search clicked!"))
                .action_icon("more_vert", || println!("More clicked!"));
        }
        
        ui.add(medium_bar);

        ui.add_space(30.0);
        
        ui.heading("Large Top App Bar");
        ui.label("Tallest app bar variant, creates strong visual hierarchy.");
        
        let mut large_bar = large_top_app_bar(&self.title_text)
            .scrolled(self.is_scrolled);
            
        if self.show_navigation {
            large_bar = large_bar.navigation_icon("menu", || println!("Navigation clicked!"));
        }
        
        if self.show_actions {
            large_bar = large_bar
                .action_icon("search", || println!("Search clicked!"))
                .action_icon("favorite", || println!("Favorite clicked!"))
                .action_icon("more_vert", || println!("More clicked!"));
        }
        
        ui.add(large_bar);

        ui.add_space(30.0);
        
        ui.heading("Interactive Demo");
        
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
        
        // Demo with different contexts
        let demo_bar = top_app_bar("Demo App")
            .navigation_icon("menu", || println!("Demo: Menu clicked!"))
            .action_icon("notifications", || println!("Demo: Notifications clicked!"))
            .action_icon("account_circle", || println!("Demo: Account clicked!"))
            .scrolled(self.is_scrolled);
        
        ui.add(demo_bar);

        ui.add_space(30.0);
        
        ui.heading("Usage Guidelines");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Regular:");
                ui.label("• Most common variant");
                ui.label("• Surface color background");
                ui.label("• Good for most apps");
                ui.label("• Standard 64dp height");
            });
            
            ui.vertical(|ui| {
                ui.label("Center Aligned:");
                ui.label("• Simple layouts");
                ui.label("• Centered title");
                ui.label("• Minimal action icons");
                ui.label("• Clean aesthetic");
            });
            
            ui.vertical(|ui| {
                ui.label("Medium/Large:");
                ui.label("• Landing pages");
                ui.label("• Primary color background");
                ui.label("• Strong visual hierarchy");
                ui.label("• Hero sections");
            });
        });
        
        ui.add_space(20.0);
        
        ui.heading("Best Practices");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Do:");
                ui.label("• Keep titles concise");
                ui.label("• Use appropriate icons");
                ui.label("• Maintain consistent spacing");
                ui.label("• Consider scroll behavior");
            });
            
            ui.vertical(|ui| {
                ui.label("Don't:");
                ui.label("• Overcrowd with actions");
                ui.label("• Use unclear icons");
                ui.label("• Ignore accessibility");
                ui.label("• Break visual hierarchy");
            });
        });
    }
}