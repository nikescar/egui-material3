#![doc(hidden)]

use crate::{select, MaterialButton, SelectVariant};
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
    // New options
    variant: SelectVariant,
    enable_filter: bool,
    enable_search: bool,
    border_radius: f32,
    menu_width: f32,
    menu_max_height: f32,
    use_custom_menu_width: bool,
    use_custom_menu_max_height: bool,
    // Select states
    filled_select_value: Option<usize>,
    outlined_select_value: Option<usize>,
    fruits_select: Option<usize>,
    countries_select: Option<usize>,
    long_text_select: Option<usize>,
    many_options_select: Option<usize>,
    // New variant examples
    variant_demo_filled: Option<usize>,
    variant_demo_outlined: Option<usize>,
    filter_demo: Option<usize>,
    validation_demo: Option<usize>,
    custom_style_demo: Option<usize>,
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
            error_text: "This field is required".to_string(),
            supporting_text: "Select your favorite fruit".to_string(),
            error: false,
            clamp_menu_width: false,
            menu_align: "start".to_string(),
            menu_positioning: "absolute".to_string(),
            leading_icon: String::new(),
            trailing_icon: String::new(),
            // New options
            variant: SelectVariant::Filled,
            enable_filter: false,
            enable_search: true,
            border_radius: 8.0,
            menu_width: 200.0,
            menu_max_height: 300.0,
            use_custom_menu_width: false,
            use_custom_menu_max_height: false,
            // Select states
            filled_select_value: Some(1), // Apple selected by default
            outlined_select_value: None,
            fruits_select: Some(0),
            countries_select: None,
            long_text_select: None,
            many_options_select: None,
            // New variant examples
            variant_demo_filled: Some(1),
            variant_demo_outlined: Some(1),
            filter_demo: None,
            validation_demo: None,
            custom_style_demo: Some(2),
        }
    }
}

impl SelectWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Select Stories")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_variant_comparison(ui);
                    ui.add_space(20.0);
                    self.render_select_variants(ui);
                    ui.add_space(20.0);
                    self.render_select_examples(ui);
                    ui.add_space(20.0);
                    self.render_validation_examples(ui);
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

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
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
            
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Variant:");
                if ui.radio(matches!(self.variant, SelectVariant::Filled), "Filled").clicked() {
                    self.variant = SelectVariant::Filled;
                }
                if ui.radio(matches!(self.variant, SelectVariant::Outlined), "Outlined").clicked() {
                    self.variant = SelectVariant::Outlined;
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Border Radius:");
                ui.add(egui::Slider::new(&mut self.border_radius, 0.0..=28.0));
            });
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.use_custom_menu_width, "Custom Menu Width:");
                if self.use_custom_menu_width {
                    ui.add(egui::Slider::new(&mut self.menu_width, 100.0..=400.0));
                }
            });
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.use_custom_menu_max_height, "Custom Menu Max Height:");
                if self.use_custom_menu_max_height {
                    ui.add(egui::Slider::new(&mut self.menu_max_height, 100.0..=500.0));
                }
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
                ui.checkbox(&mut self.enable_filter, "Enable Filter");
                ui.checkbox(&mut self.enable_search, "Enable Search");
            });
        });
    }
    
    fn render_variant_comparison(&mut self, ui: &mut egui::Ui) {
        ui.heading("Variant Comparison - Filled vs Outlined");
        
        ui.push_id("variant_comparison", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Filled Variant:");
                    let mut filled_demo = select(&mut self.variant_demo_filled)
                        .variant(SelectVariant::Filled)
                        .label("Favorite Color")
                        .option(0, "Red")
                        .option(1, "Blue")
                        .option(2, "Green")
                        .option(3, "Yellow")
                        .option(4, "Purple")
                        .helper_text("Select your favorite color")
                        .width(250.0);
                        
                    if self.disabled {
                        filled_demo = filled_demo.enabled(false);
                    }
                    if self.error {
                        filled_demo = filled_demo.error_text(&self.error_text);
                    }
                    
                    ui.add(filled_demo);
                });
                
                ui.add_space(30.0);
                
                ui.vertical(|ui| {
                    ui.label("Outlined Variant:");
                    let mut outlined_demo = select(&mut self.variant_demo_outlined)
                        .variant(SelectVariant::Outlined)
                        .label("Favorite Color")
                        .option(0, "Red")
                        .option(1, "Blue")
                        .option(2, "Green")
                        .option(3, "Yellow")
                        .option(4, "Purple")
                        .helper_text("Select your favorite color")
                        .width(250.0);
                        
                    if self.disabled {
                        outlined_demo = outlined_demo.enabled(false);
                    }
                    if self.error {
                        outlined_demo = outlined_demo.error_text(&self.error_text);
                    }
                    
                    ui.add(outlined_demo);
                });
            });
        });
    }
    
    fn render_validation_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Validation Examples");
        
        ui.push_id("validation_examples", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Required Field (with error):");
                    
                    // Check validation state before creating the select
                    let should_show_error = self.validation_demo.is_none() && self.required;
                    
                    let validation_select = select(&mut self.validation_demo)
                        .variant(SelectVariant::Outlined)
                        .label("Country *")
                        .option(0, "United States")
                        .option(1, "Canada")
                        .option(2, "United Kingdom")
                        .option(3, "Germany")
                        .option(4, "France")
                        .placeholder("Please select a country")
                        .error_text(if should_show_error {
                            "This field is required"
                        } else {
                            ""
                        })
                        .required(self.required)
                        .width(250.0);
                    
                    ui.add(validation_select);
                    
                    if ui.button("Validate").clicked() {
                        if self.validation_demo.is_none() {
                            // Trigger error state
                        }
                    }
                });
                
                ui.add_space(30.0);
                
                ui.vertical(|ui| {
                    ui.label("Custom Styling:");
                    let custom_select = select(&mut self.custom_style_demo)
                        .variant(self.variant)
                        .label("Size")
                        .option(0, "Extra Small")
                        .option(1, "Small")
                        .option(2, "Medium")
                        .option(3, "Large")
                        .option(4, "Extra Large")
                        .helper_text("Choose your size")
                        .width(250.0)
                        .border_radius(self.border_radius);
                    
                    let custom_select = if self.use_custom_menu_width {
                        custom_select.menu_width(self.menu_width)
                    } else {
                        custom_select
                    };
                    
                    let custom_select = if self.use_custom_menu_max_height {
                        custom_select.menu_max_height(self.menu_max_height)
                    } else {
                        custom_select
                    };
                    
                    ui.add(custom_select);
                });
            });
        });
    }

    fn render_select_variants(&mut self, ui: &mut egui::Ui) {
        ui.heading("Select Variants with Labels");

        ui.push_id("select_variants", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Filled Select with Label:");
                    let mut filled_select = select(&mut self.filled_select_value)
                        .variant(SelectVariant::Filled)
                        .label(&self.label)
                        .option(0, "")
                        .option(1, "Apple")
                        .option(2, "Apricot")
                        .option(3, "Apricots")
                        .option(4, "Avocado")
                        .option(5, "Green Apple")
                        .option(6, "Green Grapes")
                        .option(7, "Olive")
                        .option(8, "Orange");

                    if self.disabled {
                        filled_select = filled_select.enabled(false);
                    }

                    if self.error && !self.error_text.is_empty() {
                        filled_select = filled_select.error_text(&self.error_text);
                    }

                    if !self.supporting_text.is_empty() && !self.error {
                        filled_select = filled_select.helper_text(&self.supporting_text);
                    }

                    ui.add(filled_select);
                });

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label("Outlined Select with Label:");
                    let mut outlined_select = select(&mut self.outlined_select_value)
                        .variant(SelectVariant::Outlined)
                        .label(&self.label)
                        .option(0, "")
                        .option(1, "Apple")
                        .option(2, "Apricot")
                        .option(3, "Apricots")
                        .option(4, "Avocado")
                        .option(5, "Green Apple")
                        .option(6, "Green Grapes")
                        .option(7, "Olive")
                        .option(8, "Orange");

                    if self.disabled {
                        outlined_select = outlined_select.enabled(false);
                    }

                    if self.error && !self.error_text.is_empty() {
                        outlined_select = outlined_select.error_text(&self.error_text);
                    }
                    if !self.supporting_text.is_empty() && !self.error {
                        outlined_select = outlined_select.helper_text(&self.supporting_text);
                    }

                    ui.add(outlined_select);
                });
            });
        });
    }

    fn render_select_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Select Examples with Advanced Features");

        ui.push_id("select_examples", |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Fruits (with filtering):");
                    let mut fruits_select = select(&mut self.fruits_select)
                        .variant(self.variant)
                        .label("Fruit")
                        .option(0, "Apple")
                        .option(1, "Banana")
                        .option(2, "Cherry")
                        .option(3, "Date")
                        .option(4, "Elderberry")
                        .option(5, "Fig")
                        .option(6, "Grape")
                        .option(7, "Honeydew")
                        .option(8, "Kiwi")
                        .option(9, "Lemon")
                        .placeholder("Select a fruit")
                        .enable_filter(self.enable_filter)
                        .enable_search(self.enable_search)
                        .keep_open_on_select(true)
                        .width(250.0);

                    if self.disabled {
                        fruits_select = fruits_select.enabled(false);
                    }

                    ui.add(fruits_select);
                });

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label("Countries:");
                    let mut countries_select = select(&mut self.countries_select)
                        .variant(self.variant)
                        .label("Country")
                        .option(0, "United States")
                        .option(1, "Canada")
                        .option(2, "United Kingdom")
                        .option(3, "Germany")
                        .option(4, "France")
                        .option(5, "Japan")
                        .option(6, "Australia")
                        .option(7, "Brazil")
                        .option(8, "India")
                        .option(9, "China")
                        .placeholder("Select a country")
                        .enable_search(self.enable_search)
                        .width(250.0);

                    if self.disabled {
                        countries_select = countries_select.enabled(false);
                    }

                    if self.required {
                        countries_select = countries_select
                            .helper_text("Required field")
                            .required(true);
                    }

                    ui.add(countries_select);
                });
            });

            ui.add_space(10.0);

            // With icons example
            ui.label("Select with Icons:");
            let mut icon_select = select(&mut self.outlined_select_value)
                .variant(self.variant)
                .label("Fruit with Emoji")
                .option(0, "🍎 Apple")
                .option(1, "🍌 Banana")
                .option(2, "🍒 Cherry")
                .option(3, "🥝 Kiwi")
                .option(4, "🍊 Orange")
                .placeholder("Select fruit with emoji")
                .width(300.0);

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
            });
            ui.horizontal(|ui| {
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
                        .variant(self.variant)
                        .label("Description")
                        .option(0, "Short option")
                        .option(1, "This is a very long option text that should wrap to multiple lines when the content size is bigger than the select menu width")
                        .option(2, "Another extremely long text option that demonstrates the text wrapping functionality when content exceeds the available menu width and needs to be displayed on multiple lines")
                        .option(3, "Medium length option text")
                        .option(4, "Very very very very very very very very long option that will definitely need text wrapping")
                        .placeholder("Select long text option")
                        .width(250.0)
                        .border_radius(self.border_radius);
                    
                    if self.disabled {
                        long_text_select = long_text_select.enabled(false);
                    }
                    
                    if self.use_custom_menu_width {
                        long_text_select = long_text_select.menu_width(self.menu_width);
                    }
                    
                    ui.add(long_text_select);
                });
                
                ui.add_space(20.0);
                
                ui.vertical(|ui| {
                    ui.label("Many Options (Scroll Attachment):");
                    let mut many_options_select = select(&mut self.many_options_select)
                        .variant(self.variant)
                        .label("Option");
                    
                    // Add many options to test scrolling
                    for i in 1..=25 {
                        many_options_select = many_options_select.option(i, format!("Option {}: Item number {}", i, i));
                    }
                    
                    many_options_select = many_options_select
                        .placeholder("Select from many options")
                        .width(220.0)
                        .border_radius(self.border_radius);
                    
                    if self.disabled {
                        many_options_select = many_options_select.enabled(false);
                    }
                    
                    if self.use_custom_menu_max_height {
                        many_options_select = many_options_select.menu_max_height(self.menu_max_height);
                    }
                    
                    ui.add(many_options_select);
                    
                    ui.label("⚠️ This select tests scroll attachment to edge.");
                });
            });
            
        });
    }
}
