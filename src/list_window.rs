use eframe::egui::{self, Window};
use crate::{list, list_item};

pub struct ListWindow {
    pub open: bool,
    disabled: bool,
    overline: String,
    trailing_supporting_text: String,
    leading_icon: bool,
    trailing_icon: bool,
}

impl Default for ListWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            overline: String::new(),
            trailing_supporting_text: String::new(),
            leading_icon: false,
            trailing_icon: false,
        }
    }
}

impl ListWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("List Stories")
            .open(&mut open)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_standard_list(ui);
                    ui.add_space(20.0);
                    self.render_interactive_list(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("List Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/list/stories/");
            }
        });
        
        ui.checkbox(&mut self.disabled, "Disabled");
        ui.checkbox(&mut self.leading_icon, "Leading Icon");
        ui.checkbox(&mut self.trailing_icon, "Trailing Icon");
        
        ui.horizontal(|ui| {
            ui.label("Overline:");
            ui.text_edit_singleline(&mut self.overline);
        });
        
        ui.horizontal(|ui| {
            ui.label("Trailing Supporting Text:");
            ui.text_edit_singleline(&mut self.trailing_supporting_text);
        });
    }

    fn render_standard_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Standard List");
        
        let mut single_line_list = list();
        
        // Single line item
        let mut single_item = list_item("Single line item");
        if self.disabled {
            single_item = single_item.enabled(false);
        }
        if self.leading_icon {
            single_item = single_item.leading_icon("event");
        }
        if self.trailing_icon {
            single_item = single_item.trailing_icon("star");
        }
        if !self.overline.is_empty() {
            single_item = single_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            single_item = single_item.trailing_text(&self.trailing_supporting_text);
        }
        single_line_list = single_line_list.item(single_item.on_click(|| println!("Single line item clicked!")));
        
        // Two line item
        let mut two_line_item = list_item("Two line item")
            .secondary_text("Supporting text");
        if self.disabled {
            two_line_item = two_line_item.enabled(false);
        }
        if self.leading_icon {
            two_line_item = two_line_item.leading_icon("event");
        }
        if self.trailing_icon {
            two_line_item = two_line_item.trailing_icon("star");
        }
        if !self.overline.is_empty() {
            two_line_item = two_line_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            two_line_item = two_line_item.trailing_text(&self.trailing_supporting_text);
        }
        single_line_list = single_line_list.item(two_line_item.on_click(|| println!("Two line item clicked!")));
        
        // Three line item
        let mut three_line_item = list_item("Three line item")
            .secondary_text("Second line text\nThird line text");
        if self.disabled {
            three_line_item = three_line_item.enabled(false);
        }
        if self.leading_icon {
            three_line_item = three_line_item.leading_icon("event");
        }
        if self.trailing_icon {
            three_line_item = three_line_item.trailing_icon("star");
        }
        if !self.overline.is_empty() {
            three_line_item = three_line_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            three_line_item = three_line_item.trailing_text(&self.trailing_supporting_text);
        }
        single_line_list = single_line_list.item(three_line_item.on_click(|| println!("Three line item clicked!")));
        
        ui.add(single_line_list);
    }

    fn render_interactive_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Interactive List");
        
        let mut interactive_list = list();
        
        // Link item
        let mut link_item = list_item("Link item")
            .trailing_icon("link");
        if self.disabled {
            link_item = link_item.enabled(false);
        }
        if self.leading_icon {
            link_item = link_item.leading_icon("event");
        }
        if !self.overline.is_empty() {
            link_item = link_item.overline(&self.overline);
        }
        interactive_list = interactive_list.item(link_item.on_click(|| println!("Link item clicked!")));
        
        // Button item
        let mut button_item = list_item("Button item");
        if self.disabled {
            button_item = button_item.enabled(false);
        }
        if self.leading_icon {
            button_item = button_item.leading_icon("event");
        }
        if self.trailing_icon {
            button_item = button_item.trailing_icon("star");
        }
        if !self.overline.is_empty() {
            button_item = button_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            button_item = button_item.trailing_text(&self.trailing_supporting_text);
        }
        interactive_list = interactive_list.item(button_item.on_click(|| println!("Button item clicked!")));
        
        // Non-interactive item
        let mut non_interactive_item = list_item(&format!(
            "Non-interactive item{}",
            if self.disabled { " (disabled)" } else { "" }
        ));
        if self.disabled {
            non_interactive_item = non_interactive_item.enabled(false);
        }
        if self.leading_icon {
            non_interactive_item = non_interactive_item.leading_icon("event");
        }
        if self.trailing_icon {
            non_interactive_item = non_interactive_item.trailing_icon("star");
        }
        if !self.overline.is_empty() {
            non_interactive_item = non_interactive_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            non_interactive_item = non_interactive_item.trailing_text(&self.trailing_supporting_text);
        }
        interactive_list = interactive_list.item(non_interactive_item);
        
        ui.add(interactive_list);
    }
}