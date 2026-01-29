#![doc(hidden)]

use crate::menu::{Corner, FocusState, Positioning};
use crate::{menu, menu_item, MaterialButton, MaterialCheckbox};
use eframe::egui::{self, Rect, Window};

#[doc(hidden)]
pub struct MenuWindow {
    pub open: bool,
    // Knob options from TypeScript stories
    anchor_corner: Corner,
    menu_corner: Corner,
    default_focus: FocusState,
    positioning: Positioning,
    quick: bool,
    has_overflow: bool,
    stay_open_on_outside_click: bool,
    stay_open_on_focusout: bool,
    skip_restore_focus: bool,
    x_offset: f32,
    y_offset: f32,
    no_horizontal_flip: bool,
    no_vertical_flip: bool,
    typeahead_delay: f32,
    list_tab_index: i32,
    // Other controls
    keep_open: bool,
    disabled: bool,
    href: String,
    link_icon: String,
    // Menu states
    standard_menu_open: bool,
    link_menu_open: bool,
    submenu_open: bool,
    context_menu_open: bool,
    // Button rectangles for positioning
    items_button_rect: Option<Rect>,
    links_button_rect: Option<Rect>,
    submenu_button_rect: Option<Rect>,
    context_button_rect: Option<Rect>,
}

impl Default for MenuWindow {
    fn default() -> Self {
        Self {
            open: false,
            // Default knob values matching TypeScript stories
            anchor_corner: Corner::BottomLeft,
            menu_corner: Corner::TopLeft,
            default_focus: FocusState::None,
            positioning: Positioning::Absolute,
            quick: false,
            has_overflow: false,
            stay_open_on_outside_click: false,
            stay_open_on_focusout: false,
            skip_restore_focus: false,
            x_offset: 0.0,
            y_offset: 0.0,
            no_horizontal_flip: false,
            no_vertical_flip: false,
            typeahead_delay: 200.0,
            list_tab_index: -1,
            // Other controls
            keep_open: false,
            disabled: false,
            href: "https://google.com".to_string(),
            link_icon: "link".to_string(),
            // Menu states
            standard_menu_open: false,
            link_menu_open: false,
            submenu_open: false,
            context_menu_open: false,
            // Button rectangles
            items_button_rect: None,
            links_button_rect: None,
            submenu_button_rect: None,
            context_button_rect: None,
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

        // Close menus on Escape key
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.standard_menu_open = false;
            self.link_menu_open = false;
            self.submenu_open = false;
            self.context_menu_open = false;
        }

        // Show menus
        self.show_menus(ctx);
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.push_id("menu_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Menu Controls");

                if ui.button("Target").clicked() {
                    let _ = webbrowser::open("https://material-web.dev/components/menu/stories/");
                }
            });

            ui.separator();

            // Knobs from TypeScript stories
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.label("Anchor Corner:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.anchor_corner, Corner::TopLeft, "Top Left");
                        ui.selectable_value(&mut self.anchor_corner, Corner::TopRight, "Top Right");
                        ui.selectable_value(
                            &mut self.anchor_corner,
                            Corner::BottomLeft,
                            "Bottom Left",
                        );
                        ui.selectable_value(
                            &mut self.anchor_corner,
                            Corner::BottomRight,
                            "Bottom Right",
                        );
                    });
                });

                ui.group(|ui| {
                    ui.label("Menu Corner:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.menu_corner, Corner::TopLeft, "Top Left");
                        ui.selectable_value(&mut self.menu_corner, Corner::TopRight, "Top Right");
                        ui.selectable_value(
                            &mut self.menu_corner,
                            Corner::BottomLeft,
                            "Bottom Left",
                        );
                        ui.selectable_value(
                            &mut self.menu_corner,
                            Corner::BottomRight,
                            "Bottom Right",
                        );
                    });
                });
            });

            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.label("Default Focus:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.default_focus, FocusState::None, "None");
                        ui.selectable_value(
                            &mut self.default_focus,
                            FocusState::ListRoot,
                            "List Root",
                        );
                        ui.selectable_value(
                            &mut self.default_focus,
                            FocusState::FirstItem,
                            "First Item",
                        );
                    });
                });

                ui.group(|ui| {
                    ui.label("Positioning:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(
                            &mut self.positioning,
                            Positioning::Absolute,
                            "Absolute",
                        );
                        ui.selectable_value(&mut self.positioning, Positioning::Fixed, "Fixed");
                        ui.selectable_value(
                            &mut self.positioning,
                            Positioning::Document,
                            "Document",
                        );
                        ui.selectable_value(&mut self.positioning, Positioning::Popover, "Popover");
                    });
                });
            });

            ui.horizontal_wrapped(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.quick, "Quick"));
                ui.add(MaterialCheckbox::new(
                    &mut self.has_overflow,
                    "Has Overflow",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.stay_open_on_outside_click,
                    "Stay Open On Outside Click",
                ));
            });

            ui.horizontal_wrapped(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.stay_open_on_focusout,
                    "Stay Open On Focusout",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.skip_restore_focus,
                    "Skip Restore Focus",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.no_horizontal_flip,
                    "No Horizontal Flip",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.no_vertical_flip,
                    "No Vertical Flip",
                ));
            });

            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    ui.label("X Offset:");
                    ui.add(egui::Slider::new(&mut self.x_offset, -100.0..=100.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Y Offset:");
                    ui.add(egui::Slider::new(&mut self.y_offset, -100.0..=100.0));
                });
            });

            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Typeahead Delay:");
                    ui.add(egui::Slider::new(&mut self.typeahead_delay, 0.0..=1000.0));
                });

                ui.horizontal(|ui| {
                    ui.label("List Tab Index:");
                    ui.add(egui::Slider::new(&mut self.list_tab_index, -1..=10));
                });
            });

            ui.separator();

            // Original controls
            ui.horizontal_wrapped(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.keep_open, "Keep Open"));
                ui.add(MaterialCheckbox::new(&mut self.disabled, "Disabled"));
            });
        });

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
            // Menu with Items - opens below button (default positioning)
            let items_button = ui.add(MaterialButton::filled("Menu with Items"));
            self.items_button_rect = Some(items_button.rect);
            if items_button.clicked() {
                // Toggle menu instead of just opening
                self.standard_menu_open = !self.standard_menu_open;
                // Close other menus when opening this one
                if self.standard_menu_open {
                    self.link_menu_open = false;
                    self.submenu_open = false;
                    self.context_menu_open = false;
                }
            }
            ui.add_space(8.0);

            // Menu with Links - opens below button (default positioning)
            let links_button = ui.add(MaterialButton::filled("Menu with Links"));
            self.links_button_rect = Some(links_button.rect);
            if links_button.clicked() {
                // Toggle menu instead of just opening
                self.link_menu_open = !self.link_menu_open;
                // Close other menus when opening this one
                if self.link_menu_open {
                    self.standard_menu_open = false;
                    self.submenu_open = false;
                    self.context_menu_open = false;
                }
            }
            ui.add_space(8.0);

            // Menu with Sub-menus - opens above button
            let submenu_button = ui.add(MaterialButton::filled("Menu with Sub-menus"));
            self.submenu_button_rect = Some(submenu_button.rect);
            if submenu_button.clicked() {
                // Toggle menu instead of just opening
                self.submenu_open = !self.submenu_open;
                // Close other menus when opening this one
                if self.submenu_open {
                    self.standard_menu_open = false;
                    self.link_menu_open = false;
                    self.context_menu_open = false;
                }
            }
            ui.add_space(8.0);

            let context_button = ui.add(MaterialButton::filled("Context Menu"));
            self.context_button_rect = Some(context_button.rect);
            if context_button.clicked() {
                // Toggle menu instead of just opening
                self.context_menu_open = !self.context_menu_open;
                // Close other menus when opening this one
                if self.context_menu_open {
                    self.standard_menu_open = false;
                    self.link_menu_open = false;
                    self.submenu_open = false;
                }
            }
        });
    }

    fn show_menus(&mut self, ctx: &egui::Context) {
        // Standard Menu with Items - opens below button (default positioning)
        if self.standard_menu_open {
            let apple_item = self.create_menu_item("Apple", "apple");
            let apricot_item = self.create_menu_item("Apricot", "apricot");
            let avocado_item = self.create_menu_item("Avocado", "avocado");
            let green_apple_item = self.create_menu_item("Green Apple", "green_apple");
            let green_grapes_item = self.create_menu_item("Green Grapes", "green_grapes");
            let olive_item = self.create_menu_item("Olive", "olive");
            let orange_item = self.create_menu_item("Orange", "orange");

            let mut menu_builder = menu("standard_menu", &mut self.standard_menu_open)
                .item(apple_item)
                .item(apricot_item)
                .item(avocado_item)
                .item(green_apple_item)
                .item(green_grapes_item)
                .item(olive_item)
                .item(orange_item)
                .anchor_corner(self.anchor_corner)
                .menu_corner(self.menu_corner)
                .default_focus(self.default_focus)
                .positioning(self.positioning)
                .quick(self.quick)
                .has_overflow(self.has_overflow)
                .stay_open_on_outside_click(self.stay_open_on_outside_click)
                .stay_open_on_focusout(self.stay_open_on_focusout)
                .skip_restore_focus(self.skip_restore_focus)
                .x_offset(self.x_offset)
                .y_offset(self.y_offset)
                .no_horizontal_flip(self.no_horizontal_flip)
                .no_vertical_flip(self.no_vertical_flip)
                .typeahead_delay(self.typeahead_delay)
                .list_tab_index(self.list_tab_index);

            if let Some(rect) = self.items_button_rect {
                menu_builder = menu_builder.anchor_rect(rect);
            }

            menu_builder.show(ctx);
        }

        // Menu with Links - opens below button (default positioning)
        if self.link_menu_open {
            let apple_link = self.create_link_item("Apple", "apple");
            let apricot_link = self.create_link_item("Apricot", "apricot");
            let avocado_link = self.create_link_item("Avocado", "avocado");
            let green_apple_link = self.create_link_item("Green Apple", "green_apple");
            let green_grapes_link = self.create_link_item("Green Grapes", "green_grapes");
            let olive_link = self.create_link_item("Olive", "olive");
            let orange_link = self.create_link_item("Orange", "orange");

            let mut menu_builder = menu("link_menu", &mut self.link_menu_open)
                .item(apple_link)
                .item(apricot_link)
                .item(avocado_link)
                .item(menu_item("").divider_after(true)) // Divider
                .item(green_apple_link)
                .item(green_grapes_link)
                .item(olive_link)
                .item(orange_link)
                .anchor_corner(self.anchor_corner)
                .menu_corner(self.menu_corner)
                .default_focus(self.default_focus)
                .positioning(self.positioning)
                .quick(self.quick)
                .has_overflow(self.has_overflow)
                .stay_open_on_outside_click(self.stay_open_on_outside_click)
                .stay_open_on_focusout(self.stay_open_on_focusout)
                .skip_restore_focus(self.skip_restore_focus)
                .x_offset(self.x_offset)
                .y_offset(self.y_offset)
                .no_horizontal_flip(self.no_horizontal_flip)
                .no_vertical_flip(self.no_vertical_flip)
                .typeahead_delay(self.typeahead_delay)
                .list_tab_index(self.list_tab_index);

            if let Some(rect) = self.links_button_rect {
                menu_builder = menu_builder.anchor_rect(rect);
            }

            menu_builder.show(ctx);
        }

        // Sub-menu demonstration - opens above button by default now, but respects settings
        if self.submenu_open {
            let apple_sub = self.create_menu_item("Apple", "apple");
            let avocado_sub = self.create_menu_item("Avocado", "avocado");
            let orange_sub = self.create_menu_item("Orange", "orange");

            // For sub-menus, we default to TopLeft anchor, BottomLeft menu corner unless overridden
            let submenu_anchor = if self.anchor_corner == Corner::BottomLeft
                && self.menu_corner == Corner::TopLeft
            {
                Corner::TopLeft
            } else {
                self.anchor_corner
            };

            let submenu_menu_corner = if self.anchor_corner == Corner::BottomLeft
                && self.menu_corner == Corner::TopLeft
            {
                Corner::BottomLeft
            } else {
                self.menu_corner
            };

            let mut menu_builder = menu("submenu", &mut self.submenu_open)
                .item(
                    menu_item("Fruits")
                        .leading_icon("expand_more")
                        .on_click(|| println!("Fruits submenu clicked!")),
                )
                .item(
                    menu_item("Vegetables")
                        .leading_icon("expand_more")
                        .on_click(|| println!("Vegetables submenu clicked!")),
                )
                .item(apple_sub)
                .item(avocado_sub)
                .item(orange_sub)
                .anchor_corner(submenu_anchor)
                .menu_corner(submenu_menu_corner)
                .default_focus(self.default_focus)
                .positioning(self.positioning)
                .quick(self.quick)
                .has_overflow(self.has_overflow)
                .stay_open_on_outside_click(self.stay_open_on_outside_click)
                .stay_open_on_focusout(self.stay_open_on_focusout)
                .skip_restore_focus(self.skip_restore_focus)
                .x_offset(self.x_offset)
                .y_offset(self.y_offset)
                .no_horizontal_flip(self.no_horizontal_flip)
                .no_vertical_flip(self.no_vertical_flip)
                .typeahead_delay(self.typeahead_delay)
                .list_tab_index(self.list_tab_index);

            if let Some(rect) = self.submenu_button_rect {
                menu_builder = menu_builder.anchor_rect(rect);
            }

            menu_builder.show(ctx);
        }

        // Context Menu
        if self.context_menu_open {
            menu("context_menu", &mut self.context_menu_open)
                .item(
                    menu_item("Cut")
                        .leading_icon("cut")
                        .on_click(|| println!("Cut clicked!")),
                )
                .item(
                    menu_item("Copy")
                        .leading_icon("copy")
                        .on_click(|| println!("Copy clicked!")),
                )
                .item(
                    menu_item("Paste")
                        .leading_icon("paste")
                        .divider_after(true)
                        .on_click(|| println!("Paste clicked!")),
                )
                .item(
                    menu_item("Settings")
                        .leading_icon("settings")
                        .on_click(|| println!("Settings clicked!")),
                )
                .anchor_corner(self.anchor_corner)
                .menu_corner(self.menu_corner)
                .default_focus(self.default_focus)
                .positioning(self.positioning)
                .quick(self.quick)
                .has_overflow(self.has_overflow)
                .stay_open_on_outside_click(self.stay_open_on_outside_click)
                .stay_open_on_focusout(self.stay_open_on_focusout)
                .skip_restore_focus(self.skip_restore_focus)
                .x_offset(self.x_offset)
                .y_offset(self.y_offset)
                .no_horizontal_flip(self.no_horizontal_flip)
                .no_vertical_flip(self.no_vertical_flip)
                .typeahead_delay(self.typeahead_delay)
                .list_tab_index(self.list_tab_index)
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
        let mut item = menu_item(text).trailing_icon(&self.link_icon);
        if self.disabled {
            item = item.enabled(false);
        }
        item.on_click(move || println!("{} link clicked!", text))
    }
}
