#![doc(hidden)]

use crate::{list, list_item, MaterialButton};
use egui_material3::{ListTileTitleAlignment, VisualDensity};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct ListWindow {
    pub open: bool,
    disabled: bool,
    selected: bool,
    dense: bool,
    overline: String,
    trailing_supporting_text: String,
    leading_icon: bool,
    trailing_icon: bool,
    visual_density_mode: usize, // 0=Standard, 1=Comfortable, 2=Compact
    selected_items: Vec<bool>, // For selection demo
}

impl Default for ListWindow {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            selected: false,
            dense: false,
            overline: String::new(),
            trailing_supporting_text: String::new(),
            leading_icon: false,
            trailing_icon: false,
            visual_density_mode: 0,
            selected_items: vec![false; 5],
        }
    }
}

impl ListWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("List Stories")
            .open(&mut open)
            .default_size([800.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_standard_list(ui);
                    ui.add_space(20.0);
                    self.render_interactive_list(ui);
                    ui.add_space(20.0);
                    self.render_small_lists(ui);
                    ui.add_space(20.0);
                    self.render_narrow_lists(ui);
                    ui.add_space(20.0);
                    self.render_selection_demo(ui);
                    ui.add_space(20.0);
                    self.render_visual_density_comparison(ui);
                    ui.add_space(20.0);
                    self.render_real_world_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.push_id("list_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("List Controls");

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
                    let _ = webbrowser::open("https://material-web.dev/components/list/stories/");
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.disabled, "Disabled");
                ui.checkbox(&mut self.selected, "Selected");
                ui.checkbox(&mut self.dense, "Dense");
                ui.checkbox(&mut self.leading_icon, "Leading Icon");
                ui.checkbox(&mut self.trailing_icon, "Trailing Icon");
            });

            ui.horizontal(|ui| {
                ui.label("Visual Density:");
                ui.radio_value(&mut self.visual_density_mode, 0, "Standard");
                ui.radio_value(&mut self.visual_density_mode, 1, "Comfortable");
                ui.radio_value(&mut self.visual_density_mode, 2, "Compact");
            });
        });

        ui.push_id("list_text_controls", |ui| {
            ui.horizontal(|ui| {
                ui.label("Overline:");
                ui.text_edit_singleline(&mut self.overline);
            });

            ui.horizontal(|ui| {
                ui.label("Trailing Supporting Text:");
                ui.text_edit_singleline(&mut self.trailing_supporting_text);
            });
        });
    }

    fn get_visual_density(&self) -> VisualDensity {
        match self.visual_density_mode {
            1 => VisualDensity::COMFORTABLE,
            2 => VisualDensity::COMPACT,
            _ => VisualDensity::STANDARD,
        }
    }

    fn render_standard_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Standard List");

        let mut single_line_list = list().id("standard_list_main");
        let visual_density = self.get_visual_density();

        // Single line item
        let mut single_item = list_item("Single line item")
            .enabled(!self.disabled)
            .selected(self.selected)
            .dense(self.dense)
            .visual_density(visual_density);

        if self.leading_icon {
            single_item = single_item.leading_icon("folder".to_string());
        }
        if self.trailing_icon {
            single_item = single_item.trailing_icon("chevron_right".to_string());
        }
        if !self.overline.is_empty() {
            single_item = single_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            single_item = single_item.trailing_text(&self.trailing_supporting_text);
        }
        single_line_list =
            single_line_list.item(single_item.on_click(|| println!("Single line item clicked!")));

        // Two line item
        let mut two_line_item = list_item("Two line item")
            .secondary_text("Supporting text")
            .enabled(!self.disabled)
            .selected(self.selected)
            .dense(self.dense)
            .visual_density(visual_density);

        if self.leading_icon {
            two_line_item = two_line_item.leading_icon("image".to_string());
        }
        if self.trailing_icon {
            two_line_item = two_line_item.trailing_icon("info".to_string());
        }
        if !self.overline.is_empty() {
            two_line_item = two_line_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            two_line_item = two_line_item.trailing_text(&self.trailing_supporting_text);
        }
        single_line_list =
            single_line_list.item(two_line_item.on_click(|| println!("Two line item clicked!")));

        // Three line item
        let mut three_line_item = list_item("Three line item")
            .secondary_text("Second line text\nThird line text")
            .enabled(!self.disabled)
            .selected(self.selected)
            .dense(self.dense)
            .visual_density(visual_density);

        if self.leading_icon {
            three_line_item = three_line_item.leading_icon("description".to_string());
        }
        if self.trailing_icon {
            three_line_item = three_line_item.trailing_icon("more_vert".to_string());
        }
        if !self.overline.is_empty() {
            three_line_item = three_line_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            three_line_item = three_line_item.trailing_text(&self.trailing_supporting_text);
        }
        single_line_list = single_line_list
            .item(three_line_item.on_click(|| println!("Three line item clicked!")));

        ui.add(single_line_list);
    }

    fn render_interactive_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Interactive List");

        let mut interactive_list = list().id("interactive_list");
        let visual_density = self.get_visual_density();

        // Link item
        let mut link_item = list_item("Link item")
            .trailing_icon("open_in_new".to_string())
            .enabled(!self.disabled)
            .selected(self.selected)
            .dense(self.dense)
            .visual_density(visual_density);

        if self.leading_icon {
            link_item = link_item.leading_icon("link".to_string());
        }
        if !self.overline.is_empty() {
            link_item = link_item.overline(&self.overline);
        }
        interactive_list =
            interactive_list.item(link_item.on_click(|| println!("Link item clicked!")));

        // Button item
        let mut button_item = list_item("Button item")
            .enabled(!self.disabled)
            .selected(self.selected)
            .dense(self.dense)
            .visual_density(visual_density);

        if self.leading_icon {
            button_item = button_item.leading_icon("touch_app".to_string());
        }
        if self.trailing_icon {
            button_item = button_item.trailing_icon("arrow_forward".to_string());
        }
        if !self.overline.is_empty() {
            button_item = button_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            button_item = button_item.trailing_text(&self.trailing_supporting_text);
        }
        interactive_list =
            interactive_list.item(button_item.on_click(|| println!("Button item clicked!")));

        // Non-interactive item
        let mut non_interactive_item = list_item(&format!(
            "Non-interactive item{}",
            if self.disabled { " (disabled)" } else { "" }
        ))
        .enabled(!self.disabled)
        .selected(self.selected)
        .dense(self.dense)
        .visual_density(visual_density);

        if self.leading_icon {
            non_interactive_item = non_interactive_item.leading_icon("label".to_string());
        }
        if self.trailing_icon {
            non_interactive_item = non_interactive_item.trailing_icon("info_outline".to_string());
        }
        if !self.overline.is_empty() {
            non_interactive_item = non_interactive_item.overline(&self.overline);
        }
        if !self.trailing_supporting_text.is_empty() {
            non_interactive_item =
                non_interactive_item.trailing_text(&self.trailing_supporting_text);
        }
        interactive_list = interactive_list.item(non_interactive_item);

        ui.add(interactive_list);
    }

    fn render_small_lists(&mut self, ui: &mut egui::Ui) {
        ui.heading("Small Lists (Dense & Compact)");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Dense List:");
                let small_list = list()
                    .id("small_dense_list")
                    .item(
                        list_item("Inbox")
                            .leading_icon("inbox".to_string())
                            .trailing_text("12")
                            .dense(true)
                            .on_click(|| println!("Inbox clicked")),
                    )
                    .item(
                        list_item("Drafts")
                            .leading_icon("drafts".to_string())
                            .trailing_text("3")
                            .dense(true)
                            .on_click(|| println!("Drafts clicked")),
                    )
                    .item(
                        list_item("Sent")
                            .leading_icon("send".to_string())
                            .dense(true)
                            .on_click(|| println!("Sent clicked")),
                    )
                    .item(
                        list_item("Trash")
                            .leading_icon("delete".to_string())
                            .trailing_text("5")
                            .dense(true)
                            .on_click(|| println!("Trash clicked")),
                    );
                ui.add(small_list);
            });

            ui.vertical(|ui| {
                ui.label("Compact Visual Density:");
                let compact_list = list()
                    .id("compact_density_list")
                    .item(
                        list_item("Starred")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Starred clicked")),
                    )
                    .item(
                        list_item("Important")
                            .leading_icon("label_important".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Important clicked")),
                    )
                    .item(
                        list_item("Spam")
                            .leading_icon("report".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Spam clicked")),
                    )
                    .item(
                        list_item("Archive")
                            .leading_icon("archive".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Archive clicked")),
                    );
                ui.add(compact_list);
            });

            ui.vertical(|ui| {
                ui.label("Custom Small Height:");
                let custom_small_list = list()
                    .id("custom_small_list")
                    .item(
                        list_item("Tasks")
                            .leading_icon("task_alt".to_string())
                            .trailing_text("8")
                            .min_tile_height(40.0)
                            .min_vertical_padding(4.0)
                            .on_click(|| println!("Tasks clicked")),
                    )
                    .item(
                        list_item("Notes")
                            .leading_icon("note".to_string())
                            .trailing_text("15")
                            .min_tile_height(40.0)
                            .min_vertical_padding(4.0)
                            .on_click(|| println!("Notes clicked")),
                    )
                    .item(
                        list_item("Calendar")
                            .leading_icon("calendar_today".to_string())
                            .min_tile_height(40.0)
                            .min_vertical_padding(4.0)
                            .on_click(|| println!("Calendar clicked")),
                    )
                    .item(
                        list_item("Reminders")
                            .leading_icon("alarm".to_string())
                            .trailing_text("2")
                            .min_tile_height(40.0)
                            .min_vertical_padding(4.0)
                            .on_click(|| println!("Reminders clicked")),
                    );
                ui.add(custom_small_list);
            });
        });

        ui.add_space(10.0);
        ui.label("Dense lists with secondary text:");
        let dense_two_line = list()
            .id("dense_two_line_list")
            .item(
                list_item("Meeting Tomorrow")
                    .secondary_text("Conference Room A, 10:00 AM")
                    .leading_icon("event".to_string())
                    .dense(true)
                    .on_click(|| println!("Meeting clicked")),
            )
            .item(
                list_item("Project Deadline")
                    .secondary_text("Submit final report by Friday")
                    .leading_icon("assignment".to_string())
                    .dense(true)
                    .on_click(|| println!("Project clicked")),
            )
            .item(
                list_item("Team Standup")
                    .secondary_text("Daily sync at 9:30 AM")
                    .leading_icon("groups".to_string())
                    .dense(true)
                    .on_click(|| println!("Standup clicked")),
            );
        ui.add(dense_two_line);
    }

    fn render_narrow_lists(&mut self, ui: &mut egui::Ui) {
        ui.heading("Narrow Lists (Reduced Spacing)");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Narrow Leading Width:");
                let narrow_list = list()
                    .id("narrow_leading_list")
                    .item(
                        list_item("Photos")
                            .leading_icon("photo".to_string())
                            .trailing_text("1.2 GB")
                            .min_leading_width(32.0)
                            .horizontal_title_gap(8.0)
                            .on_click(|| println!("Photos clicked")),
                    )
                    .item(
                        list_item("Videos")
                            .leading_icon("videocam".to_string())
                            .trailing_text("5.8 GB")
                            .min_leading_width(32.0)
                            .horizontal_title_gap(8.0)
                            .on_click(|| println!("Videos clicked")),
                    )
                    .item(
                        list_item("Documents")
                            .leading_icon("description".to_string())
                            .trailing_text("245 MB")
                            .min_leading_width(32.0)
                            .horizontal_title_gap(8.0)
                            .on_click(|| println!("Documents clicked")),
                    )
                    .item(
                        list_item("Music")
                            .leading_icon("music_note".to_string())
                            .trailing_text("3.4 GB")
                            .min_leading_width(32.0)
                            .horizontal_title_gap(8.0)
                            .on_click(|| println!("Music clicked")),
                    );
                ui.add(narrow_list);
            });

            ui.vertical(|ui| {
                ui.label("Compact Horizontal Spacing:");
                let compact_spacing_list = list()
                    .id("compact_spacing_list")
                    .item(
                        list_item("Downloads")
                            .leading_icon("download".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Downloads clicked")),
                    )
                    .item(
                        list_item("Recent")
                            .leading_icon("history".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Recent clicked")),
                    )
                    .item(
                        list_item("Shared")
                            .leading_icon("people".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Shared clicked")),
                    )
                    .item(
                        list_item("Favorites")
                            .leading_icon("favorite".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Favorites clicked")),
                    );
                ui.add(compact_spacing_list);
            });

            ui.vertical(|ui| {
                ui.label("Ultra-Narrow (Combined):");
                let ultra_narrow_list = list()
                    .id("ultra_narrow_list")
                    .item(
                        list_item("Home")
                            .leading_icon("home".to_string())
                            .min_leading_width(28.0)
                            .horizontal_title_gap(4.0)
                            .min_vertical_padding(4.0)
                            .min_tile_height(36.0)
                            .on_click(|| println!("Home clicked")),
                    )
                    .item(
                        list_item("Work")
                            .leading_icon("work".to_string())
                            .min_leading_width(28.0)
                            .horizontal_title_gap(4.0)
                            .min_vertical_padding(4.0)
                            .min_tile_height(36.0)
                            .on_click(|| println!("Work clicked")),
                    )
                    .item(
                        list_item("Profile")
                            .leading_icon("person".to_string())
                            .min_leading_width(28.0)
                            .horizontal_title_gap(4.0)
                            .min_vertical_padding(4.0)
                            .min_tile_height(36.0)
                            .on_click(|| println!("Profile clicked")),
                    )
                    .item(
                        list_item("Settings")
                            .leading_icon("settings".to_string())
                            .min_leading_width(28.0)
                            .horizontal_title_gap(4.0)
                            .min_vertical_padding(4.0)
                            .min_tile_height(36.0)
                            .on_click(|| println!("Settings clicked")),
                    );
                ui.add(ultra_narrow_list);
            });
        });

        ui.add_space(10.0);
        ui.label("Narrow list with secondary text:");
        let narrow_two_line = list()
            .id("narrow_two_line_list")
            .item(
                list_item("Inbox")
                    .secondary_text("12 unread messages")
                    .leading_icon("mail".to_string())
                    .min_leading_width(32.0)
                    .horizontal_title_gap(8.0)
                    .min_vertical_padding(6.0)
                    .on_click(|| println!("Inbox clicked")),
            )
            .item(
                list_item("Drafts")
                    .secondary_text("3 drafts saved")
                    .leading_icon("edit".to_string())
                    .min_leading_width(32.0)
                    .horizontal_title_gap(8.0)
                    .min_vertical_padding(6.0)
                    .on_click(|| println!("Drafts clicked")),
            )
            .item(
                list_item("Sent")
                    .secondary_text("Last sent 2 hours ago")
                    .leading_icon("send".to_string())
                    .min_leading_width(32.0)
                    .horizontal_title_gap(8.0)
                    .min_vertical_padding(6.0)
                    .on_click(|| println!("Sent clicked")),
            );
        ui.add(narrow_two_line);
    }

    fn render_selection_demo(&mut self, ui: &mut egui::Ui) {
        ui.heading("Selection Demo (Click to Toggle)");

        let mut selection_list = list().id("selection_list");

        for i in 0..5 {
            let item_text = format!("Selectable Item {}", i + 1);
            let is_selected = self.selected_items[i];

            let item = list_item(&item_text)
                .leading_icon(if is_selected { "check_box".to_string() } else { "check_box_outline_blank".to_string() })
                .selected(is_selected)
                .on_click({
                    let items = &mut self.selected_items;
                    let idx = i;
                    move || {
                        // Note: This won't work as expected due to ownership
                        // In real app, use message passing or state management
                        println!("Item {} selection toggled", idx + 1);
                    }
                });

            selection_list = selection_list.item(item);

            // Handle selection toggle via UI interaction
            if ui.add(MaterialButton::filled(format!("Toggle Item {}", i + 1)).small()).clicked() {
                self.selected_items[i] = !self.selected_items[i];
            }
        }

        ui.add(selection_list);
    }

    fn render_visual_density_comparison(&mut self, ui: &mut egui::Ui) {
        ui.heading("Visual Density Comparison");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Standard");
                let standard_list = list()
                    .id("standard_list")
                    .item(
                        list_item("Item 1")
                            .leading_icon("inbox".to_string())
                            .visual_density(VisualDensity::STANDARD),
                    )
                    .item(
                        list_item("Item 2")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::STANDARD),
                    );
                ui.add(standard_list);
            });

            ui.vertical(|ui| {
                ui.label("Comfortable");
                let comfortable_list = list()
                    .id("comfortable_list")
                    .item(
                        list_item("Item 1")
                            .leading_icon("inbox".to_string())
                            .visual_density(VisualDensity::COMFORTABLE),
                    )
                    .item(
                        list_item("Item 2")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::COMFORTABLE),
                    );
                ui.add(comfortable_list);
            });

            ui.vertical(|ui| {
                ui.label("Compact");
                let compact_list = list()
                    .id("compact_list")
                    .item(
                        list_item("Item 1")
                            .leading_icon("inbox".to_string())
                            .visual_density(VisualDensity::COMPACT),
                    )
                    .item(
                        list_item("Item 2")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::COMPACT),
                    );
                ui.add(compact_list);
            });
        });
    }

    fn render_real_world_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Real-World Examples");

        ui.label("Settings List:");
        let settings_list = list()
            .id("settings_list")
            .item(
                list_item("Wi-Fi")
                    .secondary_text("Connected to Network")
                    .leading_icon("wifi".to_string())
                    .trailing_icon("chevron_right".to_string())
                    .on_click(|| println!("Wi-Fi settings")),
            )
            .item(
                list_item("Bluetooth")
                    .secondary_text("Off")
                    .leading_icon("bluetooth".to_string())
                    .trailing_icon("chevron_right".to_string())
                    .on_click(|| println!("Bluetooth settings")),
            )
            .item(
                list_item("Display")
                    .secondary_text("Brightness & Sleep")
                    .leading_icon("brightness_6".to_string())
                    .trailing_icon("chevron_right".to_string())
                    .on_click(|| println!("Display settings")),
            );
        ui.add(settings_list);

        ui.add_space(10.0);

        ui.label("Inbox List:");
        let inbox_list = list()
            .id("inbox_list")
            .item(
                list_item("Meeting Tomorrow")
                    .overline("John Doe")
                    .secondary_text("Don't forget about the meeting at 10 AM")
                    .leading_icon("mail".to_string())
                    .trailing_text("9:30 AM")
                    .on_click(|| println!("Open email")),
            )
            .item(
                list_item("Project Update")
                    .overline("Manager")
                    .secondary_text("The project is progressing well")
                    .leading_icon("mail".to_string())
                    .trailing_text("Yesterday")
                    .on_click(|| println!("Open email")),
            )
            .item(
                list_item("Welcome!")
                    .overline("System")
                    .secondary_text("Thank you for joining our platform")
                    .leading_icon("mail_outline".to_string())
                    .trailing_text("2 days ago")
                    .on_click(|| println!("Open email")),
            );
        ui.add(inbox_list);

        ui.add_space(10.0);

        ui.label("Music Playlist:");
        let playlist = list()
            .id("playlist")
            .item(
                list_item("Bohemian Rhapsody")
                    .secondary_text("Queen • A Night at the Opera")
                    .leading_icon("music_note".to_string())
                    .trailing_icon("more_vert".to_string())
                    .on_click(|| println!("Play song")),
            )
            .item(
                list_item("Hotel California")
                    .secondary_text("Eagles • Hotel California")
                    .leading_icon("music_note".to_string())
                    .trailing_icon("more_vert".to_string())
                    .selected(true)
                    .on_click(|| println!("Play song")),
            )
            .item(
                list_item("Stairway to Heaven")
                    .secondary_text("Led Zeppelin • Led Zeppelin IV")
                    .leading_icon("music_note".to_string())
                    .trailing_icon("more_vert".to_string())
                    .on_click(|| println!("Play song")),
            );
        ui.add(playlist);
    }
}
