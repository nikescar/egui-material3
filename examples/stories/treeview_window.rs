#![doc(hidden)]

use crate::{MaterialButton, MaterialTreeView, TreeViewItem, TreeViewState};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub struct TreeViewWindow {
    pub open: bool,
    // Tree view states for different examples
    basic_state: TreeViewState,
    icon_state: TreeViewState,
    complex_state: TreeViewState,
    selectable_state: TreeViewState,
}

impl Default for TreeViewWindow {
    fn default() -> Self {
        Self {
            open: false,
            basic_state: TreeViewState::new(),
            icon_state: TreeViewState::new(),
            complex_state: TreeViewState::new(),
            selectable_state: TreeViewState::new(),
        }
    }
}

impl TreeViewWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("TreeView Stories")
            .open(&mut open)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("TreeView Controls");

            if ui.add(MaterialButton::filled("Target").small()).clicked() {
                let _ = webbrowser::open("https://framework7.io/vue/treeview");
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.add(MaterialButton::outlined("Expand All")).clicked() {
                self.basic_state.expand_all(&Self::create_basic_items());
                self.icon_state.expand_all(&Self::create_icon_items());
                self.complex_state.expand_all(&Self::create_complex_items());
                self.selectable_state.expand_all(&Self::create_selectable_items());
            }

            if ui.add(MaterialButton::outlined("Collapse All")).clicked() {
                self.basic_state.collapse_all();
                self.icon_state.collapse_all();
                self.complex_state.collapse_all();
                self.selectable_state.collapse_all();
            }

            if ui.add(MaterialButton::outlined("Clear Selections")).clicked() {
                self.basic_state.clear_selections();
                self.icon_state.clear_selections();
                self.complex_state.clear_selections();
                self.selectable_state.clear_selections();
            }
        });
    }

    fn render_examples(&mut self, ui: &mut Ui) {
        // Basic Tree View
        ui.heading("Basic TreeView");
        ui.label("Simple hierarchical tree structure:");

        ui.group(|ui| {
            let items = Self::create_basic_items();
            ui.add(MaterialTreeView::new(&items, &mut self.basic_state));
        });

        ui.add_space(20.0);

        // Tree View with Icons
        ui.heading("TreeView with Icons");
        ui.label("Tree items with Material Design icons:");

        ui.group(|ui| {
            let items = Self::create_icon_items();
            ui.add(MaterialTreeView::new(&items, &mut self.icon_state));
        });

        ui.add_space(20.0);

        // Complex Tree View
        ui.heading("Complex TreeView");
        ui.label("Multi-level nested tree structure:");

        ui.group(|ui| {
            let items = Self::create_complex_items();
            ui.add(MaterialTreeView::new(&items, &mut self.complex_state));
        });

        ui.add_space(20.0);

        // Selectable Tree View
        ui.heading("Selectable TreeView");
        ui.label("Tree with selectable items (click labels to select):");

        ui.group(|ui| {
            let items = Self::create_selectable_items();
            ui.add(MaterialTreeView::new(&items, &mut self.selectable_state));
        });

        // Show selected items
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Selected items:");
            let selected: Vec<_> = self.selectable_state.selected.iter()
                .filter(|(_, &selected)| selected)
                .map(|(id, _)| id.as_str())
                .collect();
            if selected.is_empty() {
                ui.label("None");
            } else {
                ui.label(selected.join(", "));
            }
        });

        ui.add_space(20.0);

        // File System Example
        ui.heading("File System Example");
        ui.label("Tree representing a file system structure:");

        ui.group(|ui| {
            let items = Self::create_filesystem_items();
            ui.add(MaterialTreeView::new(&items, &mut self.complex_state));
        });
    }

    fn create_basic_items() -> Vec<TreeViewItem> {
        vec![
            TreeViewItem::new("1", "Item 1")
                .child(TreeViewItem::new("1.1", "Item 1.1"))
                .child(TreeViewItem::new("1.2", "Item 1.2"))
                .child(TreeViewItem::new("1.3", "Item 1.3")),
            TreeViewItem::new("2", "Item 2")
                .child(TreeViewItem::new("2.1", "Item 2.1"))
                .child(TreeViewItem::new("2.2", "Item 2.2")),
            TreeViewItem::new("3", "Item 3"),
        ]
    }

    fn create_icon_items() -> Vec<TreeViewItem> {
        vec![
            TreeViewItem::new("documents", "Documents")
                .icon("folder")
                .child(TreeViewItem::new("work", "Work").icon("work"))
                .child(TreeViewItem::new("personal", "Personal").icon("person")),
            TreeViewItem::new("downloads", "Downloads")
                .icon("download")
                .child(TreeViewItem::new("images", "Images").icon("image"))
                .child(TreeViewItem::new("videos", "Videos").icon("video_library")),
            TreeViewItem::new("settings", "Settings")
                .icon("settings"),
        ]
    }

    fn create_complex_items() -> Vec<TreeViewItem> {
        vec![
            TreeViewItem::new("root", "Root")
                .icon("folder")
                .child(
                    TreeViewItem::new("level1-1", "Level 1.1")
                        .icon("folder")
                        .child(
                            TreeViewItem::new("level2-1", "Level 2.1")
                                .icon("folder")
                                .child(TreeViewItem::new("level3-1", "Level 3.1").icon("description"))
                                .child(TreeViewItem::new("level3-2", "Level 3.2").icon("description"))
                        )
                        .child(TreeViewItem::new("level2-2", "Level 2.2").icon("description"))
                )
                .child(
                    TreeViewItem::new("level1-2", "Level 1.2")
                        .icon("folder")
                        .child(TreeViewItem::new("level2-3", "Level 2.3").icon("description"))
                        .child(TreeViewItem::new("level2-4", "Level 2.4").icon("description"))
                ),
        ]
    }

    fn create_selectable_items() -> Vec<TreeViewItem> {
        vec![
            TreeViewItem::new("fruits", "Fruits")
                .icon("nature")
                .child(TreeViewItem::new("apple", "Apple").icon("circle"))
                .child(TreeViewItem::new("banana", "Banana").icon("circle"))
                .child(TreeViewItem::new("orange", "Orange").icon("circle")),
            TreeViewItem::new("vegetables", "Vegetables")
                .icon("local_florist")
                .child(TreeViewItem::new("carrot", "Carrot").icon("circle"))
                .child(TreeViewItem::new("potato", "Potato").icon("circle")),
            TreeViewItem::new("dairy", "Dairy")
                .icon("water_drop")
                .child(TreeViewItem::new("milk", "Milk").icon("circle"))
                .child(TreeViewItem::new("cheese", "Cheese").icon("circle")),
        ]
    }

    fn create_filesystem_items() -> Vec<TreeViewItem> {
        vec![
            TreeViewItem::new("home", "Home")
                .icon("home")
                .child(
                    TreeViewItem::new("documents", "Documents")
                        .icon("folder")
                        .child(TreeViewItem::new("report.pdf", "report.pdf").icon("description"))
                        .child(TreeViewItem::new("presentation.pptx", "presentation.pptx").icon("slideshow"))
                )
                .child(
                    TreeViewItem::new("pictures", "Pictures")
                        .icon("folder")
                        .child(TreeViewItem::new("vacation.jpg", "vacation.jpg").icon("image"))
                        .child(TreeViewItem::new("family.png", "family.png").icon("image"))
                )
                .child(
                    TreeViewItem::new("music", "Music")
                        .icon("folder")
                        .child(TreeViewItem::new("song1.mp3", "song1.mp3").icon("music_note"))
                        .child(TreeViewItem::new("song2.mp3", "song2.mp3").icon("music_note"))
                ),
            TreeViewItem::new("desktop", "Desktop")
                .icon("computer")
                .child(TreeViewItem::new("readme.txt", "readme.txt").icon("description")),
        ]
    }
}
