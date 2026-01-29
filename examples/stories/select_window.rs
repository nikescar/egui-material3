#![doc(hidden)]

use crate::select;
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct SelectWindow {
    pub open: bool,
    label: String,
    typeahead_delay: f32,
    quick: bool,
    required: bool,
    no_asterisk: bool,
    disabled: bool,
    error_text: String,
    supporting_text: String,
    error: bool,
    clamp_menu_width: bool,
    menu_align: String,
    menu_positioning: String,
    leading_icon: String,
    trailing_icon: String,
    // Select states
    filled_select_value: Option<usize>,
    outlined_select_value: Option<usize>,
    fruits_select: Option<usize>,
    countries_select: Option<usize>,
    long_text_select: Option<usize>,
    many_options_select: Option<usize>,
}

impl Default for SelectWindow {
    fn default() -> Self {
        Self {
            open: false,
            label: "Choose a fruit".to_string(),
            typeahead_delay: 200.0,
            quick: false,
            required: false,
            no_asterisk: true, // Hide red asterisk indicators
            disabled: false,
            error_text: String::new(),
            supporting_text: "Select your favorite fruit".to_string(),
            error: false,
            clamp_menu_width: false,
            menu_align: "start".to_string(),
            menu_positioning: "absolute".to_string(),
            leading_icon: String::new(),
            trailing_icon: String::new(),
            filled_select_value: Some(1), // Apple selected by default
            outlined_select_value: None,
            fruits_select: Some(0),
            countries_select: None,
            long_text_select: None,
            many_options_select: None,
        }
    }
}

impl SelectWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Select Stories")
            .open(&mut open)
            .default_size([700.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_select_variants(ui);
                    ui.add_space(20.0);
                    self.render_select_examples(ui);
                    ui.add_space(20.0);
                    self.render_special_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.push_id("select_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Select Controls");

                if ui.button("Target").clicked() {
                    let _ = webbrowser::open("https://material-web.dev/components/select/stories/");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Label:");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.horizontal(|ui| {
                ui.label("Supporting Text:");
                ui.text_edit_singleline(&mut self.supporting_text);
            });

            ui.horizontal(|ui| {
                ui.label("Error Text:");
                ui.text_edit_singleline(&mut self.error_text);
            });

            ui.horizontal(|ui| {
                ui.label("Leading Icon:");
                ui.text_edit_singleline(&mut self.leading_icon);
            });

            ui.horizontal(|ui| {
                ui.label("Trailing Icon:");
                ui.text_edit_singleline(&mut self.trailing_icon);
            });

            ui.horizontal(|ui| {
                ui.label("Typeahead Delay (ms):");
                ui.add(egui::DragValue::new(&mut self.typeahead_delay).range(0.0..=2000.0));
            });

            ui.horizontal(|ui| {
                ui.label("Menu Align:");
                egui::ComboBox::from_id_salt("menu_align_combo")
                    .selected_text(&self.menu_align)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.menu_align, "start".to_string(), "start");
                        ui.selectable_value(&mut self.menu_align, "end".to_string(), "end");
                    });

                ui.label("Menu Positioning:");
                egui::ComboBox::from_id_salt("menu_positioning_combo")
                    .selected_text(&self.menu_positioning)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.menu_positioning,
                            "absolute".to_string(),
                            "absolute",
                        );
                        ui.selectable_value(
                            &mut self.menu_positioning,
                            "fixed".to_string(),
                            "fixed",
                        );
                        ui.selectable_value(
                            &mut self.menu_positioning,
                            "popover".to_string(),
                            "popover",
                        );
                    });
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.quick, "Quick");
                ui.checkbox(&mut self.required, "Required");
                ui.checkbox(&mut self.no_asterisk, "No Asterisk");
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.disabled, "Disabled");
                ui.checkbox(&mut self.error, "Error");
                ui.checkbox(&mut self.clamp_menu_width, "Clamp Menu Width");
            });
        });
    }

    fn render_select_variants(&mut self, ui: &mut egui::Ui) {
        ui.heading("Select Variants");

        ui.push_id("select_variants", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Filled Select:");
                    let mut filled_select = select(&mut self.filled_select_value)
                        .option(0, "")
                        .option(1, "Apple")
                        .option(2, "Apricot")
                        .option(3, "Apricots")
                        .option(4, "Avocado")
                        .option(5, "Green Apple")
                        .option(6, "Green Grapes")
                        .option(7, "Olive")
                        .option(8, "Orange")
                        .placeholder(&self.label);

                    if self.disabled {
                        filled_select = filled_select.enabled(false);
                    }

                    if self.error && !self.error_text.is_empty() {
                        filled_select = filled_select.error_text(&self.error_text);
                    }

                    if !self.supporting_text.is_empty() {
                        filled_select = filled_select.helper_text(&self.supporting_text);
                    }

                    ui.add(filled_select);
                });

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label("Outlined Select:");
                    let mut outlined_select = select(&mut self.outlined_select_value)
                        .option(0, "")
                        .option(1, "Apple")
                        .option(2, "Apricot")
                        .option(3, "Apricots")
                        .option(4, "Avocado")
                        .option(5, "Green Apple")
                        .option(6, "Green Grapes")
                        .option(7, "Olive")
                        .option(8, "Orange")
                        .placeholder(&self.label);

                    if self.disabled {
                        outlined_select = outlined_select.enabled(false);
                    }

                    if self.error && !self.error_text.is_empty() {
                        outlined_select = outlined_select.error_text(&self.error_text);
                    }
                    if !self.supporting_text.is_empty() {
                        outlined_select = outlined_select.helper_text(&self.supporting_text);
                    }

                    ui.add(outlined_select);
                });
            });
        });
    }

    fn render_select_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Select Examples");

        ui.push_id("select_examples", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Fruits:");
                    let mut fruits_select = select(&mut self.fruits_select)
                        .option(0, "Apple")
                        .option(1, "Banana")
                        .option(2, "Cherry")
                        .option(3, "Date")
                        .option(4, "Elderberry")
                        .option(5, "Fig")
                        .option(6, "Grape")
                        .placeholder("Select a fruit")
                        .keep_open_on_select(true); // Keep dropdown open after selection

                    if self.disabled {
                        fruits_select = fruits_select.enabled(false);
                    }

                    ui.add(fruits_select);
                });

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label("Countries:");
                    let mut countries_select = select(&mut self.countries_select)
                        .option(0, "United States")
                        .option(1, "Canada")
                        .option(2, "United Kingdom")
                        .option(3, "Germany")
                        .option(4, "France")
                        .option(5, "Japan")
                        .option(6, "Australia")
                        .option(7, "Brazil")
                        .placeholder("Select a country")
                        .keep_open_on_select(true); // Keep dropdown open after selection

                    if self.disabled {
                        countries_select = countries_select.enabled(false);
                    }

                    if self.required {
                        countries_select = countries_select.helper_text("Required field");
                    }

                    ui.add(countries_select);
                });
            });

            ui.add_space(10.0);

            // With icons example
            ui.label("Select with Icons:");
            let mut icon_select = select(&mut self.outlined_select_value)
                .option(0, "🍎 Apple")
                .option(1, "🍌 Banana")
                .option(2, "🍒 Cherry")
                .option(3, "🥝 Kiwi")
                .option(4, "🍊 Orange")
                .placeholder("Select fruit with emoji");

            if !self.leading_icon.is_empty() {
                icon_select = icon_select.leading_icon(&self.leading_icon);
            }

            if !self.trailing_icon.is_empty() {
                icon_select = icon_select.trailing_icon(&self.trailing_icon);
            }

            if self.disabled {
                icon_select = icon_select.enabled(false);
            }

            ui.add(icon_select);

            ui.add_space(10.0);

            // Display current selections
            ui.separator();
            ui.label("Current Selections:");
            ui.horizontal(|ui| {
                ui.label(format!("Filled: {:?}", self.filled_select_value));
                ui.label(format!("Outlined: {:?}", self.outlined_select_value));
                ui.label(format!("Fruits: {:?}", self.fruits_select));
                ui.label(format!("Countries: {:?}", self.countries_select));
                ui.label(format!("Long Text: {:?}", self.long_text_select));
                ui.label(format!("Many Options: {:?}", self.many_options_select));
            });
        }); // Close push_id block
    }

    fn render_special_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Special Cases - Text Wrapping & Scrolling");

        ui.push_id("special_examples", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Long Text Options (Text Wrapping):");
                    let mut long_text_select = select(&mut self.long_text_select)
                        .option(0, "Short option")
                        .option(1, "This is a very long option text that should wrap to multiple lines when the content size is bigger than the select menu width")
                        .option(2, "Another extremely long text option that demonstrates the text wrapping functionality when content exceeds the available menu width and needs to be displayed on multiple lines")
                        .option(3, "Medium length option text")
                        .option(4, "Very very very very very very very very long option that will definitely need text wrapping")
                        .placeholder("Select long text option")
                        .width(250.0);
                    
                    if self.disabled {
                        long_text_select = long_text_select.enabled(false);
                    }
                    
                    ui.add(long_text_select);
                });
                
                ui.add_space(20.0);
                
                ui.vertical(|ui| {
                    ui.label("Many Options (Scroll Attachment):");
                    let mut many_options_select = select(&mut self.many_options_select);
                    
                    // Add many options to test scrolling
                    for i in 1..=25 {
                        many_options_select = many_options_select.option(i, format!("Option {}: Item number {}", i, i));
                    }
                    
                    many_options_select = many_options_select
                        .placeholder("Select from many options")
                        .width(200.0);
                    
                    if self.disabled {
                        many_options_select = many_options_select.enabled(false);
                    }
                    
                    ui.add(many_options_select);
                    
                    ui.label("⚠️ This select tests scroll attachment to edge.");
                });
            });
            
        });
    }
}
