#![doc(hidden)]

use crate::{tabs_primary, tabs_secondary, MaterialButton};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct TabsWindow {
    pub open: bool,
    active_tab_index: usize,
    auto_activate: bool,
    inline_icon: bool,
    content: String,
    tabs_enabled: bool,
    // Tab states
    _primary_selected: usize,
    _secondary_selected: usize,
    music_primary_selected: usize,
    travel_secondary_selected: usize,
    custom_selected: usize,
    scrolling_selected: usize,
    nested_primary_selected: usize,
    nested_secondary_selected: usize,
    m3_primary_selected: usize,
    m3_secondary_selected: usize,
    icon_text_primary_selected: usize,
    icon_text_secondary_selected: usize,
}

impl Default for TabsWindow {
    fn default() -> Self {
        Self {
            open: false,
            active_tab_index: 0,
            auto_activate: true,
            inline_icon: false,
            content: "both".to_string(),
            tabs_enabled: true,
            _primary_selected: 0,
            _secondary_selected: 0,
            music_primary_selected: 0,
            travel_secondary_selected: 0,
            custom_selected: 0,
            scrolling_selected: 0,
            nested_primary_selected: 0,
            nested_secondary_selected: 0,
            m3_primary_selected: 0,
            m3_secondary_selected: 0,
            icon_text_primary_selected: 0,
            icon_text_secondary_selected: 0,
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
                    self.render_m3_demo_tabs(ui);
                    ui.add_space(20.0);
                    self.render_icon_text_tabs(ui);
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
        ui.push_id("tabs_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Tabs Controls");

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
                    let _ = webbrowser::open("https://material-web.dev/components/tabs/stories/");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Active Tab Index:");
                ui.add(egui::Slider::new(&mut self.active_tab_index, 0..=4));
            });

            ui.checkbox(&mut self.auto_activate, "Auto Activate");
            ui.checkbox(&mut self.inline_icon, "Inline Icon");
            ui.checkbox(&mut self.tabs_enabled, "Enabled");

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
        });
    }

    fn render_primary_tabs(&mut self, ui: &mut egui::Ui) {
        ui.push_id("primary_tabs_section", |ui| {
            ui.heading("Primary Tabs");

            // Music instrument tabs
            ui.add(
                tabs_primary(&mut self.music_primary_selected)
                    .id_salt("music_primary")
                    .tab("🎹 Keyboard")
                    .tab("🎸 Guitar")
                    .tab("🥁 Drums")
                    .tab("🎻 Bass")
                    .tab("🎷 Saxophone"),
            );

            // Enhanced content for selected tab
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            match self.music_primary_selected {
                0 => {
                    ui.heading("🎹 Keyboard");
                    ui.label("Master the art of keyboard playing with these beautiful melodies:");
                    ui.add_space(8.0);
                    ui.label("• Classical compositions: Bach, Mozart, Chopin");
                    ui.label("• Modern pieces: Jazz standards and pop hits");
                    ui.label("• Technique: Scales, arpeggios, and finger exercises");
                    ui.label("• Recommended practice: 30 minutes daily");
                }
                1 => {
                    ui.heading("🎸 Guitar");
                    ui.label("Strum your way to guitar mastery:");
                    ui.add_space(8.0);
                    ui.label("• Basic chords: G, C, D, Em, Am");
                    ui.label("• Strumming patterns: Down-up, fingerpicking");
                    ui.label("• Popular songs: Wonderwall, Hotel California");
                    ui.label("• Equipment: Acoustic vs. Electric guitars");
                }
                2 => {
                    ui.heading("🥁 Drums");
                    ui.label("Keep the rhythm alive with powerful beats:");
                    ui.add_space(8.0);
                    ui.label("• Basic beats: 4/4 time, rock, jazz, funk");
                    ui.label("• Rudiments: Paradiddles, flams, rolls");
                    ui.label("• Kit setup: Kick, snare, hi-hat, toms");
                    ui.label("• Timing: Use metronome for practice");
                }
                3 => {
                    ui.heading("🎻 Bass");
                    ui.label("Lay down the foundation with deep bass lines:");
                    ui.add_space(8.0);
                    ui.label("• Playing techniques: Fingerstyle, slap, pick");
                    ui.label("• Music theory: Root notes, walking bass");
                    ui.label("• Rhythm section: Locking in with drums");
                    ui.label("• Genres: Rock, funk, jazz, reggae");
                }
                4 => {
                    ui.heading("🎷 Saxophone");
                    ui.label("Create smooth jazz vibes with saxophone:");
                    ui.add_space(8.0);
                    ui.label("• Saxophone types: Alto, tenor, soprano, baritone");
                    ui.label("• Breathing technique: Diaphragmatic breathing");
                    ui.label("• Embouchure: Proper mouthpiece positioning");
                    ui.label("• Jazz standards: Take Five, Giant Steps");
                }
                _ => {
                    ui.label("No instrument selected");
                }
            }
        });
    }

    fn render_secondary_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Secondary Tabs");

        // Travel tabs
        ui.add(
            tabs_secondary(&mut self.travel_secondary_selected)
                .id_salt("travel_secondary")
                .tab("✈️ Travel")
                .tab("🏨 Hotel")
                .tab("🥾 Activities")
                .tab("🍽️ Food"),
        );

        // Enhanced content for selected tab
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        match self.travel_secondary_selected {
            0 => {
                ui.heading("✈️ Travel Planning");
                ui.label("Plan your next adventure with these travel tips:");
                ui.add_space(8.0);
                ui.label("• Booking flights: Compare prices, flexible dates");
                ui.label("• Travel documents: Passport, visa requirements");
                ui.label("• Packing essentials: Weather-appropriate clothing");
                ui.label("• Travel insurance: Medical and trip coverage");
            }
            1 => {
                ui.heading("🏨 Hotel Accommodation");
                ui.label("Find the perfect place to stay:");
                ui.add_space(8.0);
                ui.label("• Hotel types: Luxury, boutique, budget, hostel");
                ui.label("• Amenities: WiFi, pool, gym, breakfast");
                ui.label("• Location: City center, airport, attractions");
                ui.label("• Reviews: Check ratings and guest feedback");
            }
            2 => {
                ui.heading("🥾 Activities & Adventures");
                ui.label("Discover exciting experiences at your destination:");
                ui.add_space(8.0);
                ui.label("• Outdoor activities: Hiking, biking, water sports");
                ui.label("• Cultural experiences: Museums, tours, shows");
                ui.label("• Adventure sports: Skiing, diving, climbing");
                ui.label("• Local experiences: Cooking classes, workshops");
            }
            3 => {
                ui.heading("🍽️ Local Cuisine");
                ui.label("Taste authentic local flavors:");
                ui.add_space(8.0);
                ui.label("• Street food: Safe vendors, popular dishes");
                ui.label("• Fine dining: Michelin-starred restaurants");
                ui.label("• Local markets: Fresh ingredients, specialties");
                ui.label("• Food tours: Guided culinary experiences");
            }
            _ => {
                ui.label("No travel option selected");
            }
        }
    }

    fn render_scrolling_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Scrolling Tabs");
        ui.label("(Simulated with many tabs)");

        // Create many tabs to demonstrate scrolling
        ui.add(
            tabs_primary(&mut self.scrolling_selected)
                .id_salt("scrolling_primary")
                .tab("Tab 1")
                .tab("Tab 2")
                .tab("Tab 3")
                .tab("Tab 4")
                .tab("Tab 5")
                .tab("Tab 6")
                .tab("Tab 7")
                .tab("Tab 8")
                .tab("Tab 9")
                .tab("Tab 10"),
        );

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading(format!("Content for Tab {}", self.scrolling_selected + 1));
        ui.label(format!(
            "You have selected tab number {} out of 10 available tabs.",
            self.scrolling_selected + 1
        ));
        ui.add_space(8.0);

        match self.scrolling_selected {
            0..=2 => {
                ui.label("📊 First section - Basic information and getting started guides");
                ui.label("• Introduction to the system");
                ui.label("• Setup instructions");
            }
            3..=5 => {
                ui.label("⚙️ Middle section - Configuration and advanced features");
                ui.label("• Advanced settings");
                ui.label("• Customization options");
            }
            6..=9 => {
                ui.label("🚀 Final section - Expert features and troubleshooting");
                ui.label("• Expert configuration");
                ui.label("• Performance optimization");
            }
            _ => {
                ui.label("Unknown tab selected");
            }
        }
    }

    fn render_custom_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Custom Styled Tabs");

        // Custom themed tabs
        ui.add(
            tabs_primary(&mut self.custom_selected)
                .id_salt("custom_primary")
                .tab("✈️ Travel")
                .tab("🏨 Hotel")
                .tab("🥾 Activities")
                .tab("🍽️ Food"),
        );

        // Enhanced content with custom styling
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        match self.custom_selected {
            0 => {
                ui.colored_label(
                    egui::Color32::from_rgb(103, 80, 164),
                    "✈️ Custom Travel Experience",
                );
                ui.label("Enhanced travel planning with personalized recommendations:");
                ui.add_space(8.0);
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• AI-powered destination suggestions",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Real-time weather and travel alerts",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Integrated booking and itinerary management",
                );
            }
            1 => {
                ui.colored_label(
                    egui::Color32::from_rgb(103, 80, 164),
                    "🏨 Premium Hotel Search",
                );
                ui.label("Luxury accommodation finder with exclusive benefits:");
                ui.add_space(8.0);
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• VIP room upgrades and early check-in",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Personalized concierge services",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Loyalty points and exclusive rates",
                );
            }
            2 => {
                ui.colored_label(
                    egui::Color32::from_rgb(103, 80, 164),
                    "🥾 Curated Activity Hub",
                );
                ui.label("Handpicked experiences tailored to your interests:");
                ui.add_space(8.0);
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Local expert-guided tours",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Adventure difficulty matching",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Social group activities",
                );
            }
            3 => {
                ui.colored_label(
                    egui::Color32::from_rgb(103, 80, 164),
                    "🍽️ Gourmet Dining Network",
                );
                ui.label("Exclusive access to world-class culinary experiences:");
                ui.add_space(8.0);
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Chef's table reservations",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Wine pairing recommendations",
                );
                ui.colored_label(
                    egui::Color32::from_rgb(156, 39, 176),
                    "• Private cooking masterclasses",
                );
            }
            _ => {
                ui.colored_label(
                    egui::Color32::from_rgb(103, 80, 164),
                    "Custom premium content",
                );
            }
        }
    }

    fn render_m3_demo_tabs(&mut self, ui: &mut egui::Ui) {
        ui.push_id("m3_demo_section", |ui| {
            ui.heading("M3 Tabs (from component_screen.dart)");
            ui.label("Primary tabs - text only, label-width indicator");

            ui.add(
                tabs_primary(&mut self.m3_primary_selected)
                    .id_salt("m3_primary")
                    .tab("Video")
                    .tab("Photos")
                    .tab("Audio")
                    .enabled(self.tabs_enabled),
            );

            ui.add_space(10.0);

            match self.m3_primary_selected {
                0 => ui.label("Video content area"),
                1 => ui.label("Photos content area"),
                2 => ui.label("Audio content area"),
                _ => ui.label("Select a tab"),
            };

            ui.add_space(16.0);
            ui.label("Secondary tabs - text only, full-width underline");

            ui.add(
                tabs_secondary(&mut self.m3_secondary_selected)
                    .id_salt("m3_secondary")
                    .tab("Video")
                    .tab("Photos")
                    .tab("Audio")
                    .enabled(self.tabs_enabled),
            );

            ui.add_space(10.0);

            match self.m3_secondary_selected {
                0 => ui.label("Video content area"),
                1 => ui.label("Photos content area"),
                2 => ui.label("Audio content area"),
                _ => ui.label("Select a tab"),
            };
        });
    }

    fn render_icon_text_tabs(&mut self, ui: &mut egui::Ui) {
        ui.push_id("icon_text_section", |ui| {
            ui.heading("Icon + Text Tabs (72px height)");
            ui.label("Primary tabs with icon and text");

            ui.add(
                tabs_primary(&mut self.icon_text_primary_selected)
                    .id_salt("icon_text_primary")
                    .tab_with_icon("Video", "\u{1F3AC}")
                    .tab_with_icon("Photos", "\u{1F4F7}")
                    .tab_with_icon("Audio", "\u{1F3B5}")
                    .enabled(self.tabs_enabled),
            );

            ui.add_space(10.0);

            match self.icon_text_primary_selected {
                0 => ui.label("Video content with icon tabs"),
                1 => ui.label("Photos content with icon tabs"),
                2 => ui.label("Audio content with icon tabs"),
                _ => ui.label("Select a tab"),
            };

            ui.add_space(16.0);
            ui.label("Secondary tabs with icon and text");

            ui.add(
                tabs_secondary(&mut self.icon_text_secondary_selected)
                    .id_salt("icon_text_secondary")
                    .tab_with_icon("Video", "\u{1F3AC}")
                    .tab_with_icon("Photos", "\u{1F4F7}")
                    .tab_with_icon("Audio", "\u{1F3B5}")
                    .enabled(self.tabs_enabled),
            );

            ui.add_space(10.0);

            match self.icon_text_secondary_selected {
                0 => ui.label("Video content with icon tabs"),
                1 => ui.label("Photos content with icon tabs"),
                2 => ui.label("Audio content with icon tabs"),
                _ => ui.label("Select a tab"),
            };
        });
    }

    fn render_nested_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("Primary and Secondary Tabs");

        // Primary tabs
        ui.add(
            tabs_primary(&mut self.nested_primary_selected)
                .id_salt("nested_primary")
                .tab("🎬 Movies")
                .tab("📸 Photos")
                .tab("🎵 Music"),
        );

        ui.add_space(10.0);

        // Secondary tabs based on primary selection
        match self.nested_primary_selected {
            0 => {
                ui.label("Movies - Secondary tabs:");
                ui.add(
                    tabs_secondary(&mut self.nested_secondary_selected)
                        .id_salt("nested_movies_secondary")
                        .tab("Star Wars")
                        .tab("Avengers")
                        .tab("Jaws")
                        .tab("Frozen"),
                );

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
                ui.add(
                    tabs_secondary(&mut self.nested_secondary_selected)
                        .id_salt("nested_photos_secondary")
                        .tab("Yosemite")
                        .tab("Mona Lisa")
                        .tab("Swiss Alps")
                        .tab("Niagara Falls"),
                );

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
                ui.add(
                    tabs_secondary(&mut self.nested_secondary_selected)
                        .id_salt("nested_music_secondary")
                        .tab("Rock")
                        .tab("Ambient")
                        .tab("Soundscapes")
                        .tab("White Noise"),
                );

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
            if ui.add(MaterialButton::filled("← Previous Tab").small()).clicked() {
                if self.nested_primary_selected > 0 {
                    self.nested_primary_selected -= 1;
                    self.nested_secondary_selected = 0; // Reset secondary
                }
            }

            if ui.add(MaterialButton::filled("Next Tab →").small()).clicked() {
                if self.nested_primary_selected < 2 {
                    self.nested_primary_selected += 1;
                    self.nested_secondary_selected = 0; // Reset secondary
                }
            }
        });
    }
}
