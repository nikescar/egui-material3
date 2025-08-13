use eframe::egui::{self, Ui, Window};
use crate::MaterialItem;

pub struct ItemWindow {
    pub open: bool,
    overline: String,
    trailing_supporting_text: String,
    leading_icon: bool,
    trailing_icon: bool,
}

impl Default for ItemWindow {
    fn default() -> Self {
        Self {
            open: false,
            overline: String::new(),
            trailing_supporting_text: String::new(),
            leading_icon: false,
            trailing_icon: false,
        }
    }
}

impl ItemWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Item Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_items(ui);
                    ui.add_space(30.0);
                    self.render_long_text_items(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Item Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/item/stories/");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Overline:");
            ui.text_edit_singleline(&mut self.overline);
        });

        ui.horizontal(|ui| {
            ui.label("Trailing supporting text:");
            ui.text_edit_singleline(&mut self.trailing_supporting_text);
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.leading_icon, "Leading icon");
            ui.checkbox(&mut self.trailing_icon, "Trailing icon");
        });
    }

    fn render_basic_items(&mut self, ui: &mut Ui) {
        ui.heading("Items");
        ui.label("Basic item layouts with different line configurations:");
        
        ui.add_space(10.0);

        // Container for items
        ui.horizontal_wrapped(|ui| {
            // Single line item
            let mut single_item = MaterialItem::new()
                .headline("Single line item")
                .width(300.0);

            if !self.overline.is_empty() {
                single_item = single_item.overline(&self.overline);
            }
            if !self.trailing_supporting_text.is_empty() {
                single_item = single_item.trailing_supporting_text(&self.trailing_supporting_text);
            }
            if self.leading_icon {
                single_item = single_item.leading_icon("📅");
            }
            if self.trailing_icon {
                single_item = single_item.trailing_icon("⭐");
            }

            ui.add(single_item);
            ui.add_space(32.0);

            // Two line item
            let mut two_line_item = MaterialItem::new()
                .headline("Two line item")
                .supporting_text("Supporting text")
                .width(300.0);

            if !self.overline.is_empty() {
                two_line_item = two_line_item.overline(&self.overline);
            }
            if !self.trailing_supporting_text.is_empty() {
                two_line_item = two_line_item.trailing_supporting_text(&self.trailing_supporting_text);
            }
            if self.leading_icon {
                two_line_item = two_line_item.leading_icon("📅");
            }
            if self.trailing_icon {
                two_line_item = two_line_item.trailing_icon("⭐");
            }

            ui.add(two_line_item);
            ui.add_space(32.0);

            // Three line item
            let mut three_line_item = MaterialItem::new()
                .headline("Three line item")
                .supporting_text("Second line text\nThird line text")
                .width(300.0);

            if !self.overline.is_empty() {
                three_line_item = three_line_item.overline(&self.overline);
            }
            if !self.trailing_supporting_text.is_empty() {
                three_line_item = three_line_item.trailing_supporting_text(&self.trailing_supporting_text);
            }
            if self.leading_icon {
                three_line_item = three_line_item.leading_icon("📅");
            }
            if self.trailing_icon {
                three_line_item = three_line_item.trailing_icon("⭐");
            }

            ui.add(three_line_item);
        });
    }

    fn render_long_text_items(&mut self, ui: &mut Ui) {
        ui.heading("Items with Long Text");
        ui.label("Items demonstrating different text wrapping behaviors:");
        
        ui.add_space(10.0);

        let lorem_ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus condimentum rhoncus est volutpat venenatis.";

        ui.horizontal_wrapped(|ui| {
            // Item with truncated text (nowrap simulation)
            let mut truncated_item = MaterialItem::new()
                .headline("Item with a truncated headline and supporting text.")
                .supporting_text(&format!("Supporting text. {}", lorem_ipsum))
                .width(300.0);

            if !self.overline.is_empty() {
                truncated_item = truncated_item.overline(&self.overline);
            }
            if !self.trailing_supporting_text.is_empty() {
                truncated_item = truncated_item.trailing_supporting_text(&self.trailing_supporting_text);
            }
            if self.leading_icon {
                truncated_item = truncated_item.leading_icon("📅");
            }
            if self.trailing_icon {
                truncated_item = truncated_item.trailing_icon("⭐");
            }

            ui.add(truncated_item);
            ui.add_space(32.0);

            // Item with clamped lines
            let mut clamped_item = MaterialItem::new()
                .headline("Item with clamped lines")
                .supporting_text(&format!("Supporting text that wraps up to two lines. {}", lorem_ipsum))
                .width(300.0);

            if !self.overline.is_empty() {
                clamped_item = clamped_item.overline(&self.overline);
            }
            if !self.trailing_supporting_text.is_empty() {
                clamped_item = clamped_item.trailing_supporting_text(&self.trailing_supporting_text);
            }
            if self.leading_icon {
                clamped_item = clamped_item.leading_icon("📅");
            }
            if self.trailing_icon {
                clamped_item = clamped_item.trailing_icon("⭐");
            }

            ui.add(clamped_item);
            ui.add_space(32.0);

            // Item with full wrapping text
            let mut wrapping_item = MaterialItem::new()
                .headline("Item that always shows long wrapping text.")
                .supporting_text(&format!("Supporting text. {}", lorem_ipsum))
                .width(300.0);

            if !self.overline.is_empty() {
                wrapping_item = wrapping_item.overline(&self.overline);
            }
            if !self.trailing_supporting_text.is_empty() {
                wrapping_item = wrapping_item.trailing_supporting_text(&self.trailing_supporting_text);
            }
            if self.leading_icon {
                wrapping_item = wrapping_item.leading_icon("📅");
            }
            if self.trailing_icon {
                wrapping_item = wrapping_item.trailing_icon("⭐");
            }

            ui.add(wrapping_item);
        });

        ui.add_space(20.0);

        // Additional examples
        ui.heading("Additional Examples");

        // Clickable item
        ui.label("Clickable item:");
        let clickable_response = ui.add(MaterialItem::new()
            .headline("Click me!")
            .supporting_text("This item is clickable")
            .leading_icon("👆")
            .clickable(true)
            .width(300.0));

        if clickable_response.clicked() {
            println!("Item clicked!");
        }

        ui.add_space(10.0);

        // Item without outline
        ui.label("Item without outline:");
        ui.add(MaterialItem::new()
            .headline("No outline")
            .supporting_text("This item has no border")
            .outlined(false)
            .width(300.0));

        ui.add_space(10.0);

        // Item with custom content
        ui.label("Item with various content:");
        ui.horizontal_wrapped(|ui| {
            ui.add(MaterialItem::new()
                .overline("OVERLINE")
                .headline("Headline text")
                .supporting_text("Supporting text goes here")
                .trailing_supporting_text("12:34")
                .leading_icon("📧")
                .trailing_icon("❯")
                .width(300.0));

            ui.add_space(16.0);

            ui.add(MaterialItem::new()
                .headline("Contact Name")
                .supporting_text("Last seen 2 hours ago")
                .trailing_supporting_text("3")
                .leading_content("AV")
                .trailing_content("📞")
                .width(300.0));
        });
        
        ui.add_space(30.0);
        
        // Add a long list of items to test scrolling behavior
        ui.heading("Long Item List (Test Scrolling)");
        ui.label("This list tests scrolling behavior and potential crashes:");
        
        ui.add_space(10.0);
        
        // Create a bounded scroll area to prevent crashes
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                for i in 1..=100 {
                    let response = ui.add(MaterialItem::new()
                        .headline(&format!("Item #{}", i))
                        .supporting_text(&format!("This is supporting text for item number {}", i))
                        .trailing_supporting_text(&format!("{:02}:00", i % 24))
                        .leading_icon("📄")
                        .trailing_icon("❯")
                        .clickable(true)
                        .width(ui.available_width() - 20.0));
                    
                    if response.clicked() {
                        println!("Clicked item #{}", i);
                    }
                    
                    // Add a small separator between items
                    if i < 100 {
                        ui.add_space(2.0);
                    }
                }
            });
    }
}