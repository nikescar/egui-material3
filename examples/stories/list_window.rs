#![doc(hidden)]

use crate::{dialog, MaterialButton};
use eframe::egui::{self, Window};
use egui_material3::{ListItem, MaterialList, VisualDensity};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct ListItemData {
    title: String,
    description: String,
}

#[derive(Clone)]
struct EditableDrawerItemData {
    department: String,  // overline
    title: String,       // primary
    description: String, // secondary (can be multiline)
    team_size: usize,
    projects: usize,
    budget: String,
    location: String,
}

#[derive(Clone)]
enum ListAction {
    Edit(usize),
    Delete(usize),
}

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
    selected_items: Vec<bool>,  // For selection demo
    // Editable list state
    editable_items: Vec<ListItemData>,
    edit_dialog_open: bool,
    delete_dialog_open: bool,
    editing_index: Option<usize>,
    edit_title: String,
    edit_description: String,
    // Editable drawer list state
    editable_drawer_items: Vec<EditableDrawerItemData>,
    edit_drawer_dialog_open: bool,
    edit_drawer_title: String,
    edit_drawer_department: String,
    edit_drawer_description: String,
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
            editable_items: vec![
                ListItemData {
                    title: "Design mockups".to_string(),
                    description: "Create wireframes and high-fidelity designs for the new user dashboard. This includes mobile and desktop layouts with responsive breakpoints.".to_string(),
                },
                ListItemData {
                    title: "API Development".to_string(),
                    description: "Implement REST endpoints for user management".to_string(),
                },
                ListItemData {
                    title: "Database Migration".to_string(),
                    description: "Update schema to support new features".to_string(),
                },
                ListItemData {
                    title: "Testing".to_string(),
                    description: "Write comprehensive unit and integration tests for all new API endpoints and UI components. Include edge cases and error scenarios.".to_string(),
                },
            ],
            edit_dialog_open: false,
            delete_dialog_open: false,
            editing_index: None,
            edit_title: String::new(),
            edit_description: String::new(),
            editable_drawer_items: vec![
                EditableDrawerItemData {
                    department: "Engineering".to_string(),
                    title: "Backend Development".to_string(),
                    description: "Server infrastructure and API services development".to_string(),
                    team_size: 12,
                    projects: 3,
                    budget: "$120,000".to_string(),
                    location: "Building A, Floor 2".to_string(),
                },
                EditableDrawerItemData {
                    department: "Design".to_string(),
                    title: "UI/UX Design".to_string(),
                    description: "User interface and experience design for all products".to_string(),
                    team_size: 8,
                    projects: 5,
                    budget: "$85,000".to_string(),
                    location: "Building B, Floor 1".to_string(),
                },
                EditableDrawerItemData {
                    department: "Marketing".to_string(),
                    title: "Digital Marketing".to_string(),
                    description: "Online campaigns and analytics across multiple channels".to_string(),
                    team_size: 6,
                    projects: 4,
                    budget: "$65,000".to_string(),
                    location: "Building A, Floor 3".to_string(),
                },
            ],
            edit_drawer_dialog_open: false,
            edit_drawer_title: String::new(),
            edit_drawer_department: String::new(),
            edit_drawer_description: String::new(),
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
                    self.render_drawer_demo(ui);
                    ui.add_space(20.0);
                    self.render_text_indicator_drawer_demo(ui);
                    ui.add_space(20.0);
                    self.render_real_world_examples(ui);
                    ui.add_space(20.0);
                    self.render_full_width_example(ui);
                    ui.add_space(20.0);
                    self.render_editable_list(ui);
                    ui.add_space(20.0);
                    self.render_editable_drawer_list(ui);
                });
            });
        self.open = open;

        // Show dialogs
        self.show_edit_dialog(ctx);
        self.show_delete_dialog(ctx);
        self.show_edit_drawer_dialog(ctx);
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

        let mut single_line_list = MaterialList::new().id(egui::Id::unique("standard_list_main"));
        let visual_density = self.get_visual_density();

        // Single line item
        let mut single_item = ListItem::new("Single line item")
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
        let mut two_line_item = ListItem::new("Two line item")
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
        let mut three_line_item = ListItem::new("Three line item")
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

        let mut interactive_list = MaterialList::new().id(egui::Id::unique("interactive_list"));
        let visual_density = self.get_visual_density();

        // Link item
        let mut link_item = ListItem::new("Link item")
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
        let mut button_item = ListItem::new("Button item")
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
        let mut non_interactive_item = ListItem::new(format!(
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
                let small_list = MaterialList::new()
                    .id(egui::Id::unique("small_dense_list"))
                    .item(
                        ListItem::new("Inbox")
                            .leading_icon("inbox".to_string())
                            .trailing_text("12")
                            .dense(true)
                            .on_click(|| println!("Inbox clicked")),
                    )
                    .item(
                        ListItem::new("Drafts")
                            .leading_icon("drafts".to_string())
                            .trailing_text("3")
                            .dense(true)
                            .on_click(|| println!("Drafts clicked")),
                    )
                    .item(
                        ListItem::new("Sent")
                            .leading_icon("send".to_string())
                            .dense(true)
                            .on_click(|| println!("Sent clicked")),
                    )
                    .item(
                        ListItem::new("Trash")
                            .leading_icon("delete".to_string())
                            .trailing_text("5")
                            .dense(true)
                            .on_click(|| println!("Trash clicked")),
                    );
                ui.add(small_list);
            });

            ui.vertical(|ui| {
                ui.label("Compact Visual Density:");
                let compact_list = MaterialList::new()
                    .id(egui::Id::unique("compact_density_list"))
                    .item(
                        ListItem::new("Starred")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Starred clicked")),
                    )
                    .item(
                        ListItem::new("Important")
                            .leading_icon("label_important".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Important clicked")),
                    )
                    .item(
                        ListItem::new("Spam")
                            .leading_icon("report".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Spam clicked")),
                    )
                    .item(
                        ListItem::new("Archive")
                            .leading_icon("archive".to_string())
                            .visual_density(VisualDensity::COMPACT)
                            .on_click(|| println!("Archive clicked")),
                    );
                ui.add(compact_list);
            });

            ui.vertical(|ui| {
                ui.label("Custom Small Height:");
                let custom_small_list = MaterialList::new()
                    .id(egui::Id::unique("custom_small_list"))
                    .item(
                        ListItem::new("Tasks")
                            .leading_icon("task_alt".to_string())
                            .trailing_text("8")
                            .min_tile_height(40.0)
                            .min_vertical_padding(4.0)
                            .on_click(|| println!("Tasks clicked")),
                    )
                    .item(
                        ListItem::new("Notes")
                            .leading_icon("note".to_string())
                            .trailing_text("15")
                            .min_tile_height(40.0)
                            .min_vertical_padding(4.0)
                            .on_click(|| println!("Notes clicked")),
                    )
                    .item(
                        ListItem::new("Calendar")
                            .leading_icon("calendar_today".to_string())
                            .min_tile_height(40.0)
                            .min_vertical_padding(4.0)
                            .on_click(|| println!("Calendar clicked")),
                    )
                    .item(
                        ListItem::new("Reminders")
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
        let dense_two_line = MaterialList::new()
            .id(egui::Id::unique("dense_two_line_list"))
            .item(
                ListItem::new("Meeting Tomorrow")
                    .secondary_text("Conference Room A, 10:00 AM")
                    .leading_icon("event".to_string())
                    .dense(true)
                    .on_click(|| println!("Meeting clicked")),
            )
            .item(
                ListItem::new("Project Deadline")
                    .secondary_text("Submit final report by Friday")
                    .leading_icon("assignment".to_string())
                    .dense(true)
                    .on_click(|| println!("Project clicked")),
            )
            .item(
                ListItem::new("Team Standup")
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
                let narrow_list = MaterialList::new()
                    .id(egui::Id::unique("narrow_leading_list"))
                    .item(
                        ListItem::new("Photos")
                            .leading_icon("photo".to_string())
                            .trailing_text("1.2 GB")
                            .min_leading_width(32.0)
                            .horizontal_title_gap(8.0)
                            .on_click(|| println!("Photos clicked")),
                    )
                    .item(
                        ListItem::new("Videos")
                            .leading_icon("videocam".to_string())
                            .trailing_text("5.8 GB")
                            .min_leading_width(32.0)
                            .horizontal_title_gap(8.0)
                            .on_click(|| println!("Videos clicked")),
                    )
                    .item(
                        ListItem::new("Documents")
                            .leading_icon("description".to_string())
                            .trailing_text("245 MB")
                            .min_leading_width(32.0)
                            .horizontal_title_gap(8.0)
                            .on_click(|| println!("Documents clicked")),
                    )
                    .item(
                        ListItem::new("Music")
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
                let compact_spacing_list = MaterialList::new()
                    .id(egui::Id::unique("compact_spacing_list"))
                    .item(
                        ListItem::new("Downloads")
                            .leading_icon("download".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Downloads clicked")),
                    )
                    .item(
                        ListItem::new("Recent")
                            .leading_icon("history".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Recent clicked")),
                    )
                    .item(
                        ListItem::new("Shared")
                            .leading_icon("people".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Shared clicked")),
                    )
                    .item(
                        ListItem::new("Favorites")
                            .leading_icon("favorite".to_string())
                            .horizontal_title_gap(6.0)
                            .min_vertical_padding(6.0)
                            .on_click(|| println!("Favorites clicked")),
                    );
                ui.add(compact_spacing_list);
            });

            ui.vertical(|ui| {
                ui.label("Ultra-Narrow (Combined):");
                let ultra_narrow_list = MaterialList::new()
                    .id(egui::Id::unique("ultra_narrow_list"))
                    .item(
                        ListItem::new("Home")
                            .leading_icon("home".to_string())
                            .min_leading_width(28.0)
                            .horizontal_title_gap(4.0)
                            .min_vertical_padding(4.0)
                            .min_tile_height(36.0)
                            .on_click(|| println!("Home clicked")),
                    )
                    .item(
                        ListItem::new("Work")
                            .leading_icon("work".to_string())
                            .min_leading_width(28.0)
                            .horizontal_title_gap(4.0)
                            .min_vertical_padding(4.0)
                            .min_tile_height(36.0)
                            .on_click(|| println!("Work clicked")),
                    )
                    .item(
                        ListItem::new("Profile")
                            .leading_icon("person".to_string())
                            .min_leading_width(28.0)
                            .horizontal_title_gap(4.0)
                            .min_vertical_padding(4.0)
                            .min_tile_height(36.0)
                            .on_click(|| println!("Profile clicked")),
                    )
                    .item(
                        ListItem::new("Settings")
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
        let narrow_two_line = MaterialList::new()
            .id(egui::Id::unique("narrow_two_line_list"))
            .item(
                ListItem::new("Inbox")
                    .secondary_text("12 unread messages")
                    .leading_icon("mail".to_string())
                    .min_leading_width(32.0)
                    .horizontal_title_gap(8.0)
                    .min_vertical_padding(6.0)
                    .on_click(|| println!("Inbox clicked")),
            )
            .item(
                ListItem::new("Drafts")
                    .secondary_text("3 drafts saved")
                    .leading_icon("edit".to_string())
                    .min_leading_width(32.0)
                    .horizontal_title_gap(8.0)
                    .min_vertical_padding(6.0)
                    .on_click(|| println!("Drafts clicked")),
            )
            .item(
                ListItem::new("Sent")
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

        let mut selection_list = MaterialList::new().id(egui::Id::unique("selection_list"));

        for i in 0..5 {
            let item_text = format!("Selectable Item {}", i + 1);
            let is_selected = self.selected_items[i];

            let item = ListItem::new(&item_text)
                .leading_icon(if is_selected {
                    "check_box".to_string()
                } else {
                    "check_box_outline_blank".to_string()
                })
                .selected(is_selected)
                .on_click({
                    let _items = &mut self.selected_items;
                    let idx = i;
                    move || {
                        // Note: This won't work as expected due to ownership
                        // In real app, use message passing or state management
                        println!("Item {} selection toggled", idx + 1);
                    }
                });

            selection_list = selection_list.item(item);

            // Handle selection toggle via UI interaction
            if ui
                .add(MaterialButton::filled(format!("Toggle Item {}", i + 1)).small())
                .clicked()
            {
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
                let standard_list = MaterialList::new()
                    .id(egui::Id::unique("standard_list"))
                    .item(
                        ListItem::new("Item 1")
                            .leading_icon("inbox".to_string())
                            .visual_density(VisualDensity::STANDARD),
                    )
                    .item(
                        ListItem::new("Item 2")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::STANDARD),
                    );
                ui.add(standard_list);
            });

            ui.vertical(|ui| {
                ui.label("Comfortable");
                let comfortable_list = MaterialList::new()
                    .id(egui::Id::unique("comfortable_list"))
                    .item(
                        ListItem::new("Item 1")
                            .leading_icon("inbox".to_string())
                            .visual_density(VisualDensity::COMFORTABLE),
                    )
                    .item(
                        ListItem::new("Item 2")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::COMFORTABLE),
                    );
                ui.add(comfortable_list);
            });

            ui.vertical(|ui| {
                ui.label("Compact");
                let compact_list = MaterialList::new()
                    .id(egui::Id::unique("compact_list"))
                    .item(
                        ListItem::new("Item 1")
                            .leading_icon("inbox".to_string())
                            .visual_density(VisualDensity::COMPACT),
                    )
                    .item(
                        ListItem::new("Item 2")
                            .leading_icon("star".to_string())
                            .visual_density(VisualDensity::COMPACT),
                    );
                ui.add(compact_list);
            });
        });
    }

    fn render_drawer_demo(&mut self, ui: &mut egui::Ui) {
        ui.heading("List with Selective Drawers");
        ui.label("Click anywhere on the item (or the chevron) to expand/collapse drawer content");

        let drawer_list = MaterialList::new()
            .id(egui::Id::unique("drawer_demo_list"))
            .allow_drawer(true)
            .item(
                ListItem::new("Project Alpha")
                    .secondary_text("High priority")
                    .leading_icon("folder".to_string())
                    .drawer(|ui| {
                        ui.label("Project Details:");
                        ui.label("• Status: In Progress");
                        ui.label("• Deadline: Next Friday");
                        ui.label("• Team: 5 members");
                        ui.label("• Budget: $50,000");
                    }),
            )
            .item(
                ListItem::new("Project Beta")
                    .secondary_text("Medium priority")
                    .leading_icon("folder_open".to_string())
                    .drawer(|ui| {
                        ui.label("Project Details:");
                        ui.label("• Status: Planning");
                        ui.label("• Deadline: Next month");
                        ui.label("• Team: 3 members");
                        ui.label("• Budget: $30,000");
                    }),
            )
            .item(
                ListItem::new("Regular Item")
                    .secondary_text("No drawer content")
                    .leading_icon("description".to_string()),
            )
            .item(
                ListItem::new("Project Gamma")
                    .secondary_text("Low priority")
                    .leading_icon("folder_special".to_string())
                    .drawer_if(true, |ui| {
                        ui.label("Project Details:");
                        ui.label("• Status: On Hold");
                        ui.label("• Deadline: TBD");
                        ui.label("• Team: 2 members");
                        ui.label("• Budget: $15,000");
                    }),
            );

        ui.add(drawer_list);

        ui.add_space(10.0);
        ui.label("Conditional Drawers (drawer_if):");
        ui.label("Drawers shown only for active projects:");

        let projects = vec![
            ("Design System", "Active", true),
            ("API Migration", "Completed", false),
            ("Mobile App", "Active", true),
            ("Documentation", "Pending", false),
        ];

        let conditional_drawer_list = MaterialList::new()
            .id(egui::Id::unique("conditional_drawer_list"))
            .allow_drawer(true);

        let mut list = conditional_drawer_list;
        for (name, status, is_active) in projects {
            let item = ListItem::new(name)
                .secondary_text(format!("Status: {}", status))
                .leading_icon(if is_active {
                    "task_alt".to_string()
                } else {
                    "radio_button_unchecked".to_string()
                })
                .drawer_if(is_active, |ui| {
                    ui.label("Active Project Info:");
                    ui.label("• Last updated: Today");
                    ui.label("• Recent activity: 3 commits");
                    ui.label("• Next milestone: In 2 weeks");
                });
            list = list.item(item);
        }

        ui.add(list);
    }

    fn render_text_indicator_drawer_demo(&mut self, ui: &mut egui::Ui) {
        ui.heading("Drawer with Text Indicators & 5-Line Items");
        ui.label("Click anywhere to toggle. Icon labels show department names below folder icons.");

        let list_id = egui::Id::unique("text_indicator_drawer_list");

        // Get current drawer state to determine which items are expanded
        let state = ui
            .data_mut(|d| d.get_persisted::<egui_material3::MaterialListState>(list_id))
            .unwrap_or_default();

        // Department data: (department, role, description)
        let departments = [
            (
                "Engineering",
                "Backend Development",
                "Server infrastructure\nand API services",
            ),
            (
                "Design",
                "UI/UX Design",
                "User interface design\nand user experience",
            ),
            (
                "Marketing",
                "Digital Marketing",
                "Online campaigns\nand analytics",
            ),
            ("Sales", "Enterprise Sales", "B2B customer\nacquisition"),
        ];

        let mut list = MaterialList::new().id(list_id).allow_drawer(true);

        for (index, (dept, role, desc)) in departments.iter().enumerate() {
            let is_open = state.drawer_open_items.contains(&index);
            let indicator = if is_open {
                "Collapse ▼"
            } else {
                "Expand ▶"
            };

            // 5-line item: overline + primary + 2-line secondary + text indicator
            // Icon label shows department name below the folder icon
            let item = ListItem::new(*role)
                .overline(*dept)
                .secondary_text(*desc) // Contains \n for multi-line
                .trailing_text(indicator)
                .leading_icon("folder")
                .leading_icon_label(*dept) // Department name below icon
                .drawer(|ui| {
                    ui.label("Department Details:");
                    ui.label("• Team Size: 12 members");
                    ui.label("• Active Projects: 3");
                    ui.label("• Budget: $120,000");
                    ui.label("• Location: Building A, Floor 2");
                    ui.label("• Last Updated: Today at 3:45 PM");
                });

            list = list.item(item);
        }

        ui.add(list);
    }

    fn render_real_world_examples(&mut self, ui: &mut egui::Ui) {
        ui.heading("Real-World Examples");

        ui.label("Settings List:");
        let settings_list = MaterialList::new()
            .id(egui::Id::unique("settings_list"))
            .item(
                ListItem::new("Wi-Fi")
                    .secondary_text("Connected to Network")
                    .leading_icon("wifi".to_string())
                    .trailing_icon("chevron_right".to_string())
                    .on_click(|| println!("Wi-Fi settings")),
            )
            .item(
                ListItem::new("Bluetooth")
                    .secondary_text("Off")
                    .leading_icon("bluetooth".to_string())
                    .trailing_icon("chevron_right".to_string())
                    .on_click(|| println!("Bluetooth settings")),
            )
            .item(
                ListItem::new("Display")
                    .secondary_text("Brightness & Sleep")
                    .leading_icon("brightness_6".to_string())
                    .trailing_icon("chevron_right".to_string())
                    .on_click(|| println!("Display settings")),
            );
        ui.add(settings_list);

        ui.add_space(10.0);

        ui.label("Inbox List:");
        let inbox_list = MaterialList::new()
            .id(egui::Id::unique("inbox_list"))
            .item(
                ListItem::new("Meeting Tomorrow")
                    .overline("John Doe")
                    .secondary_text("Don't forget about the meeting at 10 AM")
                    .leading_icon("mail".to_string())
                    .trailing_text("9:30 AM")
                    .on_click(|| println!("Open email")),
            )
            .item(
                ListItem::new("Project Update")
                    .overline("Manager")
                    .secondary_text("The project is progressing well")
                    .leading_icon("mail".to_string())
                    .trailing_text("Yesterday")
                    .on_click(|| println!("Open email")),
            )
            .item(
                ListItem::new("Welcome!")
                    .overline("System")
                    .secondary_text("Thank you for joining our platform")
                    .leading_icon("mail_outline".to_string())
                    .trailing_text("2 days ago")
                    .on_click(|| println!("Open email")),
            );
        ui.add(inbox_list);

        ui.add_space(10.0);

        ui.label("Music Playlist:");
        let playlist = MaterialList::new()
            .id(egui::Id::unique("playlist"))
            .item(
                ListItem::new("Bohemian Rhapsody")
                    .secondary_text("Queen • A Night at the Opera")
                    .leading_icon("music_note".to_string())
                    .trailing_icon("more_vert".to_string())
                    .on_click(|| println!("Play song")),
            )
            .item(
                ListItem::new("Hotel California")
                    .secondary_text("Eagles • Hotel California")
                    .leading_icon("music_note".to_string())
                    .trailing_icon("more_vert".to_string())
                    .selected(true)
                    .on_click(|| println!("Play song")),
            )
            .item(
                ListItem::new("Stairway to Heaven")
                    .secondary_text("Led Zeppelin • Led Zeppelin IV")
                    .leading_icon("music_note".to_string())
                    .trailing_icon("more_vert".to_string())
                    .on_click(|| println!("Play song")),
            );
        ui.add(playlist);
    }

    fn render_full_width_example(&mut self, ui: &mut egui::Ui) {
        ui.heading("Full Width List");
        ui.label("This list fills the available width using .fill_width(true)");

        let visual_density = self.get_visual_density();

        let full_width_list = MaterialList::new()
            .id(egui::Id::unique("full_width_list"))
            .fill_width(true)
            .item(
                ListItem::new("Dashboard")
                    .secondary_text("View analytics and metrics")
                    .leading_icon("dashboard".to_string())
                    .trailing_icon("arrow_forward".to_string())
                    .visual_density(visual_density)
                    .on_click(|| println!("Dashboard clicked")),
            )
            .item(
                ListItem::new("Reports")
                    .secondary_text("Monthly performance reports")
                    .leading_icon("assessment".to_string())
                    .trailing_icon("arrow_forward".to_string())
                    .visual_density(visual_density)
                    .on_click(|| println!("Reports clicked")),
            )
            .item(
                ListItem::new("Settings")
                    .secondary_text("Configure application preferences")
                    .leading_icon("settings".to_string())
                    .trailing_icon("arrow_forward".to_string())
                    .visual_density(visual_density)
                    .on_click(|| println!("Settings clicked")),
            )
            .item(
                ListItem::new("Account")
                    .secondary_text("Manage your profile and billing")
                    .leading_icon("account_circle".to_string())
                    .trailing_icon("arrow_forward".to_string())
                    .visual_density(visual_density)
                    .on_click(|| println!("Account clicked")),
            );

        ui.add(full_width_list);
    }

    fn render_editable_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Editable List with Action Buttons");
        ui.label("Demonstrates custom trailing widgets with Edit/Delete buttons and form dialog integration");

        let action_signal: Arc<Mutex<Option<ListAction>>> = Arc::new(Mutex::new(None));

        let mut list = MaterialList::new().id(egui::Id::unique("editable_list"));

        for (index, item) in self.editable_items.iter().enumerate() {
            let action_clone = Arc::clone(&action_signal);
            let idx = index;

            let list_item = ListItem::new(&item.title)
                .secondary_text(&item.description)
                .leading_icon("article".to_string())
                .trailing_widget(move |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;

                        if ui.add(MaterialButton::filled("Edit").small()).clicked() {
                            if let Ok(mut signal) = action_clone.lock() {
                                *signal = Some(ListAction::Edit(idx));
                            }
                        }

                        let action_clone2 = Arc::clone(&action_clone);
                        if ui.add(MaterialButton::filled("Delete").small()).clicked() {
                            if let Ok(mut signal) = action_clone2.lock() {
                                *signal = Some(ListAction::Delete(idx));
                            }
                        }
                    });
                });

            list = list.item(list_item);
        }

        ui.add(list);

        // Handle action after render
        let action = {
            if let Ok(mut signal) = action_signal.lock() {
                signal.take()
            } else {
                None
            }
        };

        if let Some(action) = action {
            match action {
                ListAction::Edit(idx) => {
                    self.editing_index = Some(idx);
                    if let Some(item) = self.editable_items.get(idx) {
                        self.edit_title = item.title.clone();
                        self.edit_description = item.description.clone();
                    }
                    self.edit_dialog_open = true;
                }
                ListAction::Delete(idx) => {
                    self.editing_index = Some(idx);
                    self.delete_dialog_open = true;
                }
            }
        }
    }

    fn show_edit_dialog(&mut self, ctx: &egui::Context) {
        if !self.edit_dialog_open {
            return;
        }

        let was_open = self.edit_dialog_open;
        let save_clicked = Arc::new(Mutex::new(false));
        let save_clone = Arc::clone(&save_clicked);

        dialog(
            egui::Id::unique("edit_item_dialog"),
            "Edit Item",
            &mut self.edit_dialog_open,
        )
        .max_width(560.0)
        .content(|ui, _open| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.edit_title);
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Description:");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.edit_description)
                            .desired_width(f32::INFINITY)
                            .desired_rows(3),
                    );
                });
            });
        })
        .action("Cancel", || {})
        .primary_action("Save", move || {
            if let Ok(mut clicked) = save_clone.lock() {
                *clicked = true;
            }
        })
        .show(ctx);

        // Commit changes only if Save was clicked
        if was_open && !self.edit_dialog_open {
            let should_save = save_clicked.lock().map(|v| *v).unwrap_or(false);
            if should_save && !self.edit_title.is_empty() {
                if let Some(idx) = self.editing_index {
                    if let Some(item) = self.editable_items.get_mut(idx) {
                        item.title = self.edit_title.clone();
                        item.description = self.edit_description.clone();
                    }
                }
            }
        }
    }

    fn show_delete_dialog(&mut self, ctx: &egui::Context) {
        if !self.delete_dialog_open {
            return;
        }

        let item_title = self
            .editing_index
            .and_then(|idx| self.editable_items.get(idx))
            .map(|item| item.title.as_str())
            .unwrap_or("this item");

        let was_open = self.delete_dialog_open;
        let delete_confirmed = Arc::new(Mutex::new(false));
        let delete_clone = Arc::clone(&delete_confirmed);

        dialog(
            egui::Id::unique("delete_item_dialog"),
            "Delete Item?",
            &mut self.delete_dialog_open,
        )
        .icon("delete_outline")
        .max_width(350.0)
        .content(|ui, _open| {
            ui.label(format!(
                "Are you sure you want to delete '{}'? This action cannot be undone.",
                item_title
            ));
        })
        .action("Cancel", || {})
        .primary_action("Delete", move || {
            if let Ok(mut confirmed) = delete_clone.lock() {
                *confirmed = true;
            }
        })
        .show(ctx);

        // Handle deletion only if Delete was confirmed
        if was_open && !self.delete_dialog_open {
            let should_delete = delete_confirmed.lock().map(|v| *v).unwrap_or(false);
            if should_delete {
                if let Some(idx) = self.editing_index {
                    if idx < self.editable_items.len() {
                        self.editable_items.remove(idx);
                    }
                }
            }
            self.editing_index = None;
        }
    }

    fn render_editable_drawer_list(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0); // Add spacing to prevent overlap with previous list
        ui.heading("Editable Drawer List with 5-Line Items");
        ui.label("Combines Edit/Delete buttons, expandable drawers, and 5-line layout (overline + primary + 2-line secondary + text indicator)");

        let list_id = "editable_drawer_list";
        let state = ui
            .data_mut(|d| {
                d.get_persisted::<egui_material3::MaterialListState>(egui::Id::unique(list_id))
            })
            .unwrap_or_default();

        let action_signal: Arc<Mutex<Option<ListAction>>> = Arc::new(Mutex::new(None));

        let mut list = MaterialList::new()
            .id(egui::Id::unique(list_id))
            .allow_drawer(true);

        for (index, item) in self.editable_drawer_items.iter().enumerate() {
            let action_clone = Arc::clone(&action_signal);
            let idx = index;
            let is_open = state.drawer_open_items.contains(&index);
            let indicator = if is_open {
                "Collapse ▼"
            } else {
                "Expand ▶"
            };

            let team_size = item.team_size;
            let projects = item.projects;
            let budget = item.budget.clone();
            let location = item.location.clone();

            let list_item = ListItem::new(&item.title)
                .overline(&item.department)
                .secondary_text(&item.description)
                .leading_icon("folder".to_string())
                .leading_icon_label(&item.department)
                .drawer(move |ui| {
                    ui.label("Department Details:");
                    ui.label(format!("• Team Size: {} members", team_size));
                    ui.label(format!("• Active Projects: {}", projects));
                    ui.label(format!("• Budget: {}", budget));
                    ui.label(format!("• Location: {}", location));
                })
                .trailing_widget(move |ui| {
                    // Vertical layout: indicator at top, buttons at center
                    ui.vertical(|ui| {
                        // Indicator at top
                        ui.add_space(2.0);
                        ui.label(indicator);

                        // Spacer to push buttons to center
                        ui.add_space(8.0);

                        // Buttons horizontally aligned
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;

                            if ui.add(MaterialButton::filled("Edit").small()).clicked() {
                                if let Ok(mut signal) = action_clone.lock() {
                                    *signal = Some(ListAction::Edit(idx));
                                }
                            }

                            let action_clone2 = Arc::clone(&action_clone);
                            if ui.add(MaterialButton::filled("Delete").small()).clicked() {
                                if let Ok(mut signal) = action_clone2.lock() {
                                    *signal = Some(ListAction::Delete(idx));
                                }
                            }
                        });
                    });
                });

            list = list.item(list_item);
        }

        ui.add(list);

        // Handle action after render
        let action = {
            if let Ok(mut signal) = action_signal.lock() {
                signal.take()
            } else {
                None
            }
        };

        if let Some(action) = action {
            match action {
                ListAction::Edit(idx) => {
                    self.editing_index = Some(idx);
                    if let Some(item) = self.editable_drawer_items.get(idx) {
                        self.edit_drawer_title = item.title.clone();
                        self.edit_drawer_department = item.department.clone();
                        self.edit_drawer_description = item.description.clone();
                    }
                    self.edit_drawer_dialog_open = true;
                }
                ListAction::Delete(idx) => {
                    if idx < self.editable_drawer_items.len() {
                        self.editable_drawer_items.remove(idx);
                    }
                }
            }
        }
    }

    fn show_edit_drawer_dialog(&mut self, ctx: &egui::Context) {
        if !self.edit_drawer_dialog_open {
            return;
        }

        let was_open = self.edit_drawer_dialog_open;
        let save_clicked = Arc::new(Mutex::new(false));
        let save_clone = Arc::clone(&save_clicked);

        dialog(
            egui::Id::unique("edit_drawer_dialog"),
            "Edit Department",
            &mut self.edit_drawer_dialog_open,
        )
        .max_width(560.0)
        .content(|ui, _open| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Department:");
                    ui.text_edit_singleline(&mut self.edit_drawer_department);
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.edit_drawer_title);
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Description:");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.edit_drawer_description)
                            .desired_width(f32::INFINITY)
                            .desired_rows(3),
                    );
                });
            });
        })
        .action("Cancel", || {})
        .primary_action("Save", move || {
            if let Ok(mut clicked) = save_clone.lock() {
                *clicked = true;
            }
        })
        .show(ctx);

        // Commit changes only if Save was clicked
        if was_open && !self.edit_drawer_dialog_open {
            let should_save = save_clicked.lock().map(|v| *v).unwrap_or(false);
            if should_save && !self.edit_drawer_title.is_empty() {
                if let Some(idx) = self.editing_index {
                    if let Some(item) = self.editable_drawer_items.get_mut(idx) {
                        item.title = self.edit_drawer_title.clone();
                        item.department = self.edit_drawer_department.clone();
                        item.description = self.edit_drawer_description.clone();
                    }
                }
            }
        }
    }
}
