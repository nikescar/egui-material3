use eframe::egui::{self, Window};
use crate::{tabs_primary, tabs_secondary};

pub struct TabsWindow {
    pub open: bool,
    active_tab_index: usize,
    auto_activate: bool,
    inline_icon: bool,
    content: String,
    // Tab states
    primary_selected: usize,
    secondary_selected: usize,
    music_primary_selected: usize,
    travel_secondary_selected: usize,
    custom_selected: usize,
    scrolling_selected: usize,
    nested_primary_selected: usize,
    nested_secondary_selected: usize,
}

impl Default for TabsWindow {
    fn default() -> Self {
        Self {
            open: false,
            active_tab_index: 0,
            auto_activate: true,
            inline_icon: false,
            content: "both".to_string(),
            primary_selected: 0,
            secondary_selected: 0,
            music_primary_selected: 0,
            travel_secondary_selected: 0,
            custom_selected: 0,
            scrolling_selected: 0,
            nested_primary_selected: 0,
            nested_secondary_selected: 0,
        }
    }
}

impl TabsWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Tabs Stories")
            .open(&mut open)
            .default_size([800.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_primary_tabs(ui);
                    ui.add_space(20.0);
                    self.render_secondary_tabs(ui);
                    ui.add_space(20.0);
                    self.render_scrolling_tabs(ui);
                    ui.add_space(20.0);
                    self.render_custom_tabs(ui);
                    ui.add_space(20.0);
                    self.render_nested_tabs(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Tabs Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/tabs/stories/");
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Active Tab Index:");
            ui.add(egui::Slider::new(&mut self.active_tab_index, 0..=4));
        });
        
        ui.checkbox(&mut self.auto_activate, "Auto Activate");
        ui.checkbox(&mut self.inline_icon, "Inline Icon");
        
        ui.horizontal(|ui| {
            ui.label("Content:");
            egui::ComboBox::from_label("")
                .selected_text(&self.content)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.content, "both".to_string(), "Both");
                    ui.selectable_value(&mut self.content, "icon".to_string(), "Icon");
                    ui.selectable_value(&mut self.content, "label".to_string(), "Label");
                });
        });
    }

    fn render_primary_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Primary Tabs");
        
        // Music instrument tabs
        ui.add(tabs_primary(&mut self.music_primary_selected)
            .tab("🎹 Keyboard")
            .tab("🎸 Guitar")
            .tab("🥁 Drums")
            .tab("🎻 Bass")
            .tab("🎷 Saxophone"));
        
        // Content for selected tab
        let content = match self.music_primary_selected {
            0 => "Keyboard content - Play beautiful melodies",
            1 => "Guitar content - Strum your favorite chords",
            2 => "Drums content - Keep the rhythm alive",
            3 => "Bass content - Lay down the foundation",
            4 => "Saxophone content - Smooth jazz vibes",
            _ => "No content selected",
        };
        
        ui.add_space(10.0);
        ui.label(content);
    }

    fn render_secondary_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Secondary Tabs");
        
        // Travel tabs
        ui.add(tabs_secondary(&mut self.travel_secondary_selected)
            .tab("✈️ Travel")
            .tab("🏨 Hotel")
            .tab("🥾 Activities")
            .tab("🍽️ Food"));
        
        // Content for selected tab
        let content = match self.travel_secondary_selected {
            0 => "Travel content - Plan your next adventure",
            1 => "Hotel content - Find the perfect accommodation",
            2 => "Activities content - Discover exciting experiences",
            3 => "Food content - Taste local cuisine",
            _ => "No content selected",
        };
        
        ui.add_space(10.0);
        ui.label(content);
    }

    fn render_scrolling_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Scrolling Tabs");
        ui.label("(Simulated with many tabs)");
        
        // Create many tabs to demonstrate scrolling
        ui.add(tabs_primary(&mut self.scrolling_selected)
            .tab("Tab 1")
            .tab("Tab 2")
            .tab("Tab 3")
            .tab("Tab 4")
            .tab("Tab 5")
            .tab("Tab 6")
            .tab("Tab 7")
            .tab("Tab 8")
            .tab("Tab 9")
            .tab("Tab 10"));
        
        ui.add_space(10.0);
        ui.label(format!("Selected tab: {}", self.scrolling_selected + 1));
    }

    fn render_custom_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Custom Styled Tabs");
        
        // Custom themed tabs
        ui.add(tabs_primary(&mut self.custom_selected)
            .tab("✈️ Travel")
            .tab("🏨 Hotel")
            .tab("🥾 Activities")
            .tab("🍽️ Food"));
        
        // Content with custom styling
        let content = match self.custom_selected {
            0 => "Custom Travel - Enhanced travel planning experience",
            1 => "Custom Hotel - Premium accommodation search",
            2 => "Custom Activities - Curated activity recommendations",
            3 => "Custom Food - Gourmet dining experiences",
            _ => "Custom content",
        };
        
        ui.add_space(10.0);
        ui.colored_label(egui::Color32::from_rgb(103, 80, 164), content);
    }

    fn render_nested_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Primary and Secondary Tabs");
        
        // Primary tabs
        ui.add(tabs_primary(&mut self.nested_primary_selected)
            .tab("🎬 Movies")
            .tab("📸 Photos")
            .tab("🎵 Music"));
        
        ui.add_space(10.0);
        
        // Secondary tabs based on primary selection
        match self.nested_primary_selected {
            0 => {
                ui.label("Movies - Secondary tabs:");
                ui.add(tabs_secondary(&mut self.nested_secondary_selected)
                    .tab("Star Wars")
                    .tab("Avengers")
                    .tab("Jaws")
                    .tab("Frozen"));
                
                let movie_content = match self.nested_secondary_selected {
                    0 => "Star Wars - A galaxy far, far away...",
                    1 => "Avengers - Earth's Mightiest Heroes",
                    2 => "Jaws - You're gonna need a bigger boat",
                    3 => "Frozen - Let it go, let it go...",
                    _ => "Select a movie",
                };
                ui.label(movie_content);
            }
            1 => {
                ui.label("Photos - Secondary tabs:");
                ui.add(tabs_secondary(&mut self.nested_secondary_selected)
                    .tab("Yosemite")
                    .tab("Mona Lisa")
                    .tab("Swiss Alps")
                    .tab("Niagara Falls"));
                
                let photo_content = match self.nested_secondary_selected {
                    0 => "Yosemite - Breathtaking natural landscapes",
                    1 => "Mona Lisa - Leonardo da Vinci's masterpiece",
                    2 => "Swiss Alps - Majestic mountain peaks",
                    3 => "Niagara Falls - Powerful cascading waters",
                    _ => "Select a photo category",
                };
                ui.label(photo_content);
            }
            2 => {
                ui.label("Music - Secondary tabs:");
                ui.add(tabs_secondary(&mut self.nested_secondary_selected)
                    .tab("Rock")
                    .tab("Ambient")
                    .tab("Soundscapes")
                    .tab("White Noise"));
                
                let music_content = match self.nested_secondary_selected {
                    0 => "Rock - High energy guitar-driven music",
                    1 => "Ambient - Atmospheric and atmospheric sounds",
                    2 => "Soundscapes - Natural and synthetic environments",
                    3 => "White Noise - Consistent background sound",
                    _ => "Select a music genre",
                };
                ui.label(music_content);
            }
            _ => {
                ui.label("Select a primary tab to see secondary options");
            }
        }
        
        ui.add_space(10.0);
        
        // Dynamic tabs demonstration
        ui.separator();
        ui.label("Dynamic Tab Controls:");
        ui.horizontal(|ui| {
            if ui.button("← Previous Tab").clicked() {
                if self.nested_primary_selected > 0 {
                    self.nested_primary_selected -= 1;
                    self.nested_secondary_selected = 0; // Reset secondary
                }
            }
            
            if ui.button("Next Tab →").clicked() {
                if self.nested_primary_selected < 2 {
                    self.nested_primary_selected += 1;
                    self.nested_secondary_selected = 0; // Reset secondary
                }
            }
        });
    }
}