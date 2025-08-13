use eframe::egui::{self, Window};
use crate::{MaterialButton, menu, menu_item};

pub struct MenuWindow {
    pub open: bool,
    keep_open: bool,
    disabled: bool,
    href: String,
    link_icon: String,
    // Menu states
    standard_menu_open: bool,
    link_menu_open: bool,
    submenu_open: bool,
    context_menu_open: bool,
}

impl Default for MenuWindow {
    fn default() -> Self {
        Self {
            open: false,
            keep_open: false,
            disabled: false,
            href: "https://google.com".to_string(),
            link_icon: "link".to_string(),
            standard_menu_open: false,
            link_menu_open: false,
            submenu_open: false,
            context_menu_open: false,
        }
    }
}

impl MenuWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Menu Stories")
            .open(&mut open)
            .default_size([700.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_menu_triggers(ui);
                });
            });
        self.open = open;

        // Only close menu on explicit click or escape, not on mouse outside
        // This prevents the menu from closing when mouse cursor is outside
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.standard_menu_open = false;
            self.link_menu_open = false;
            self.submenu_open = false;
            self.context_menu_open = false;
        }
        
        // Handle explicit clicks outside to close any open menus
        if ctx.input(|i| i.pointer.any_click()) {
            let pointer_pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();
            // Only close if clicked in empty space, not on menu areas
            let screen_rect = ctx.screen_rect();
            if screen_rect.contains(pointer_pos) {
                // Check if click was on a menu button area - if not, close menus
                // This is a simplified check - in a real implementation you'd track button positions
                if pointer_pos.y > 100.0 && pointer_pos.x < 200.0 {
                    // Assume click was not on a menu button
                    self.standard_menu_open = false;
                    self.link_menu_open = false;
                    self.submenu_open = false;
                    self.context_menu_open = false;
                }
            }
        }
        
        // Show menus
        self.show_menus(ctx);
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Menu Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/menu/stories/");
            }
        });
        
        ui.checkbox(&mut self.keep_open, "Keep Open");
        ui.checkbox(&mut self.disabled, "Disabled");
        
        ui.horizontal(|ui| {
            ui.label("Link URL:");
            ui.text_edit_singleline(&mut self.href);
        });
        
        ui.horizontal(|ui| {
            ui.label("Link Icon:");
            ui.text_edit_singleline(&mut self.link_icon);
        });
    }

    fn render_menu_triggers(&mut self, ui: &mut egui::Ui) {
        ui.heading("Menu Types");
        
        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::filled("Menu with Items")).clicked() {
                self.standard_menu_open = true;
            }
            
            if ui.add(MaterialButton::filled("Menu with Links")).clicked() {
                self.link_menu_open = true;
            }
            
            if ui.add(MaterialButton::filled("Menu with Sub-menus")).clicked() {
                self.submenu_open = true;
            }
            
            if ui.add(MaterialButton::filled("Context Menu")).clicked() {
                self.context_menu_open = true;
            }
        });
    }

    fn show_menus(&mut self, ctx: &egui::Context) {
        // Standard Menu with Items
        if self.standard_menu_open {
            let apple_item = self.create_menu_item("Apple", "apple");
            let apricot_item = self.create_menu_item("Apricot", "apricot");
            let avocado_item = self.create_menu_item("Avocado", "avocado");
            let green_apple_item = self.create_menu_item("Green Apple", "green_apple");
            let green_grapes_item = self.create_menu_item("Green Grapes", "green_grapes");
            let olive_item = self.create_menu_item("Olive", "olive");
            let orange_item = self.create_menu_item("Orange", "orange");
            
            menu("standard_menu", &mut self.standard_menu_open)
                .item(apple_item)
                .item(apricot_item)
                .item(avocado_item)
                .item(green_apple_item)
                .item(green_grapes_item)
                .item(olive_item)
                .item(orange_item)
                .show(ctx);
        }

        // Menu with Links
        if self.link_menu_open {
            let apple_link = self.create_link_item("Apple", "apple");
            let apricot_link = self.create_link_item("Apricot", "apricot");
            let avocado_link = self.create_link_item("Avocado", "avocado");
            let green_apple_link = self.create_link_item("Green Apple", "green_apple");
            let green_grapes_link = self.create_link_item("Green Grapes", "green_grapes");
            let olive_link = self.create_link_item("Olive", "olive");
            let orange_link = self.create_link_item("Orange", "orange");
            
            menu("link_menu", &mut self.link_menu_open)
                .item(apple_link)
                .item(apricot_link)
                .item(avocado_link)
                .item(menu_item("").divider_after(true)) // Divider
                .item(green_apple_link)
                .item(green_grapes_link)
                .item(olive_link)
                .item(orange_link)
                .show(ctx);
        }

        // Sub-menu demonstration (simplified)
        if self.submenu_open {
            let apple_sub = self.create_menu_item("Apple", "apple");
            let avocado_sub = self.create_menu_item("Avocado", "avocado");
            let orange_sub = self.create_menu_item("Orange", "orange");
            
            menu("submenu", &mut self.submenu_open)
                .item(menu_item("Fruits")
                    .leading_icon("expand_more")
                    .on_click(|| println!("Fruits submenu clicked!")))
                .item(menu_item("Vegetables")
                    .leading_icon("expand_more")
                    .on_click(|| println!("Vegetables submenu clicked!")))
                .item(apple_sub)
                .item(avocado_sub)
                .item(orange_sub)
                .show(ctx);
        }

        // Context Menu
        if self.context_menu_open {
            menu("context_menu", &mut self.context_menu_open)
                .item(menu_item("Cut")
                    .leading_icon("cut")
                    .on_click(|| println!("Cut clicked!")))
                .item(menu_item("Copy")
                    .leading_icon("copy")
                    .on_click(|| println!("Copy clicked!")))
                .item(menu_item("Paste")
                    .leading_icon("paste")
                    .divider_after(true)
                    .on_click(|| println!("Paste clicked!")))
                .item(menu_item("Settings")
                    .leading_icon("settings")
                    .on_click(|| println!("Settings clicked!")))
                .show(ctx);
        }
    }

    fn create_menu_item<'a>(&self, text: &'a str, _id: &str) -> crate::MenuItem<'a> {
        let mut item = menu_item(text);
        if self.disabled {
            item = item.enabled(false);
        }
        item.on_click(move || println!("{} clicked!", text))
    }

    fn create_link_item<'a>(&self, text: &'a str, _id: &str) -> crate::MenuItem<'a> {
        let mut item = menu_item(text)
            .trailing_icon(&self.link_icon);
        if self.disabled {
            item = item.enabled(false);
        }
        item.on_click(move || println!("{} link clicked!", text))
    }
}