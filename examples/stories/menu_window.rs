#![doc(hidden)]

use crate::menu::{Corner, FocusState, MenuButtonThemeData, MenuStyle, Positioning};
use crate::{menu, menu_item, MaterialButton, MaterialCheckbox};
use eframe::egui::{self, Color32, Rect, Window};

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
    // Component screen examples (ButtonAnchor / IconButtonAnchor)
    button_anchor_open: bool,
    button_anchor_rect: Option<Rect>,
    icon_anchor_open: bool,
    icon_anchor_rect: Option<Rect>,
    // MenuStyle customization
    custom_style_enabled: bool,
    custom_bg_color: [f32; 3],
    custom_elevation: f32,
    custom_corner_radius: f32,
    custom_padding: f32,
    // MenuButtonThemeData customization
    custom_button_theme_enabled: bool,
    custom_item_height: f32,
    custom_icon_size: f32,
    custom_padding_horizontal: f32,
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
            // Component screen examples
            button_anchor_open: false,
            button_anchor_rect: None,
            icon_anchor_open: false,
            icon_anchor_rect: None,
            // MenuStyle customization
            custom_style_enabled: false,
            custom_bg_color: [0.2, 0.2, 0.3],
            custom_elevation: 3.0,
            custom_corner_radius: 4.0,
            custom_padding: 8.0,
            // MenuButtonThemeData customization
            custom_button_theme_enabled: false,
            custom_item_height: 48.0,
            custom_icon_size: 24.0,
            custom_padding_horizontal: 12.0,
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
                    self.render_component_examples(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    self.render_style_controls(ui);
                    ui.add_space(10.0);
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
            self.button_anchor_open = false;
            self.icon_anchor_open = false;
        }

        // Show menus
        self.show_menus(ctx);
        self.show_component_menus(ctx);
    }

    /// Render component_screen.dart style examples: ButtonAnchor and IconButtonAnchor.
    fn render_component_examples(&mut self, ui: &mut egui::Ui) {
        ui.push_id("component_examples", |ui| {
            ui.heading("Menus");
            ui.label("Use MenuAnchor with MaterialMenu");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                // ButtonAnchorExample: tonal button that opens a menu with leading icons
                let show_menu_button = ui.add(MaterialButton::filled_tonal("Show menu"));
                self.button_anchor_rect = Some(show_menu_button.rect);
                if show_menu_button.clicked() {
                    self.button_anchor_open = !self.button_anchor_open;
                    if self.button_anchor_open {
                        self.icon_anchor_open = false;
                    }
                }

                ui.add_space(16.0);

                // IconButtonAnchorExample: icon button (more_vert) that opens a menu
                let icon_button = ui.add(MaterialButton::filled("\u{22EE}").small()); // vertical ellipsis
                self.icon_anchor_rect = Some(icon_button.rect);
                if icon_button.clicked() {
                    self.icon_anchor_open = !self.icon_anchor_open;
                    if self.icon_anchor_open {
                        self.button_anchor_open = false;
                    }
                }
            });
        });
    }

    /// Show menus from the component examples.
    fn show_component_menus(&mut self, ctx: &egui::Context) {
        let custom_style = self.build_custom_style();
        let custom_button_theme = self.build_custom_button_theme();

        // ButtonAnchorExample menu
        if self.button_anchor_open {
            let mut builder = menu("button_anchor_menu", &mut self.button_anchor_open)
                .item(
                    menu_item("Item 1")
                        .leading_icon("people")
                        .on_click(|| println!("Item 1 clicked!")),
                )
                .item(
                    menu_item("Item 2")
                        .leading_icon("eye")
                        .on_click(|| println!("Item 2 clicked!")),
                )
                .item(
                    menu_item("Item 3")
                        .leading_icon("refresh")
                        .on_click(|| println!("Item 3 clicked!")),
                );

            if let Some(rect) = self.button_anchor_rect {
                builder = builder.anchor_rect(rect);
            }
            if let Some(style) = &custom_style {
                builder = builder.style(style.clone());
            }
            if let Some(theme) = &custom_button_theme {
                builder = builder.button_theme(theme.clone());
            }

            builder.show(ctx);
        }

        // IconButtonAnchorExample menu
        if self.icon_anchor_open {
            let mut builder = menu("icon_anchor_menu", &mut self.icon_anchor_open)
                .item(menu_item("Menu 1").on_click(|| println!("Menu 1 clicked!")))
                .item(menu_item("Menu 2").on_click(|| println!("Menu 2 clicked!")))
                .item(
                    menu_item("Menu 3.1")
                        .on_click(|| println!("Menu 3.1 clicked!")),
                )
                .item(
                    menu_item("Menu 3.2")
                        .on_click(|| println!("Menu 3.2 clicked!")),
                )
                .item(
                    menu_item("Menu 3.3")
                        .on_click(|| println!("Menu 3.3 clicked!")),
                );

            if let Some(rect) = self.icon_anchor_rect {
                builder = builder.anchor_rect(rect);
            }
            if let Some(style) = &custom_style {
                builder = builder.style(style.clone());
            }
            if let Some(theme) = &custom_button_theme {
                builder = builder.button_theme(theme.clone());
            }

            builder.show(ctx);
        }
    }

    /// Render MenuStyle and MenuButtonThemeData controls.
    fn render_style_controls(&mut self, ui: &mut egui::Ui) {
        ui.push_id("style_controls", |ui| {
            ui.heading("Style Customization");

            ui.horizontal_wrapped(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.custom_style_enabled,
                    "Custom MenuStyle",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.custom_button_theme_enabled,
                    "Custom MenuButtonThemeData",
                ));
            });

            if self.custom_style_enabled {
                ui.group(|ui| {
                    ui.label("MenuStyle");
                    ui.horizontal(|ui| {
                        ui.label("Background:");
                        ui.color_edit_button_rgb(&mut self.custom_bg_color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Elevation:");
                        ui.add(egui::Slider::new(&mut self.custom_elevation, 0.0..=12.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Corner Radius:");
                        ui.add(egui::Slider::new(
                            &mut self.custom_corner_radius,
                            0.0..=24.0,
                        ));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Vertical Padding:");
                        ui.add(egui::Slider::new(&mut self.custom_padding, 0.0..=24.0));
                    });
                });
            }

            if self.custom_button_theme_enabled {
                ui.group(|ui| {
                    ui.label("MenuButtonThemeData");
                    ui.horizontal(|ui| {
                        ui.label("Item Height:");
                        ui.add(egui::Slider::new(&mut self.custom_item_height, 32.0..=72.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Icon Size:");
                        ui.add(egui::Slider::new(&mut self.custom_icon_size, 16.0..=48.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Horizontal Padding:");
                        ui.add(egui::Slider::new(
                            &mut self.custom_padding_horizontal,
                            4.0..=32.0,
                        ));
                    });
                });
            }
        });
    }

    fn build_custom_style(&self) -> Option<MenuStyle> {
        if !self.custom_style_enabled {
            return None;
        }
        Some(MenuStyle {
            background_color: Some(Color32::from_rgb(
                (self.custom_bg_color[0] * 255.0) as u8,
                (self.custom_bg_color[1] * 255.0) as u8,
                (self.custom_bg_color[2] * 255.0) as u8,
            )),
            elevation: Some(self.custom_elevation),
            corner_radius: Some(self.custom_corner_radius),
            padding: Some(self.custom_padding),
            ..MenuStyle::default()
        })
    }

    fn build_custom_button_theme(&self) -> Option<MenuButtonThemeData> {
        if !self.custom_button_theme_enabled {
            return None;
        }
        Some(MenuButtonThemeData {
            min_height: Some(self.custom_item_height),
            icon_size: Some(self.custom_icon_size),
            padding_horizontal: Some(self.custom_padding_horizontal),
            ..MenuButtonThemeData::default()
        })
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.push_id("menu_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Menu Controls");

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
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
        let custom_style = self.build_custom_style();
        let custom_button_theme = self.build_custom_button_theme();

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
            if let Some(style) = &custom_style {
                menu_builder = menu_builder.style(style.clone());
            }
            if let Some(theme) = &custom_button_theme {
                menu_builder = menu_builder.button_theme(theme.clone());
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
            if let Some(style) = &custom_style {
                menu_builder = menu_builder.style(style.clone());
            }
            if let Some(theme) = &custom_button_theme {
                menu_builder = menu_builder.button_theme(theme.clone());
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
            if let Some(style) = &custom_style {
                menu_builder = menu_builder.style(style.clone());
            }
            if let Some(theme) = &custom_button_theme {
                menu_builder = menu_builder.button_theme(theme.clone());
            }

            menu_builder.show(ctx);
        }

        // Context Menu
        if self.context_menu_open {
            let mut builder = menu("context_menu", &mut self.context_menu_open)
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
                .list_tab_index(self.list_tab_index);

            if let Some(rect) = self.context_button_rect {
                builder = builder.anchor_rect(rect);
            }
            if let Some(style) = &custom_style {
                builder = builder.style(style.clone());
            }
            if let Some(theme) = &custom_button_theme {
                builder = builder.button_theme(theme.clone());
            }

            builder.show(ctx);
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
