#![doc(hidden)]

use crate::theme::get_global_color;
use crate::{MaterialButton, MaterialCheckbox, DrawerItem};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub struct DrawerWindow {
    pub open: bool,
    permanent_drawer_open: bool,
    modal_drawer_open: bool,
    dismissible_drawer_open: bool,
    show_header: bool,
    show_icons: bool,
    show_badges: bool,
    show_sections: bool,
    drawer_width: f32,
    drawer_elevation: f32,
    scrim_opacity: f32,
    corner_radius: f32,
    selected_drawer: DrawerType,
    use_material3: bool,
    // State for different drawer demos
    is_dismissible_sidebar_open: bool,
    is_modal_sidebar_open: bool,
    selected_menu_item: String,
    dismissible_selected_menu_item: String,
    modal_selected_menu_item: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DrawerType {
    Permanent,
    Modal,
    Dismissible,
}

impl Default for DrawerWindow {
    fn default() -> Self {
        Self {
            open: false,
            permanent_drawer_open: false,
            modal_drawer_open: false,
            dismissible_drawer_open: false,
            show_header: true,
            show_icons: true,
            show_badges: false,
            show_sections: true,
            drawer_width: 360.0,
            drawer_elevation: 1.0,
            scrim_opacity: 0.54,
            corner_radius: 16.0,
            selected_drawer: DrawerType::Permanent,
            use_material3: true,
            is_dismissible_sidebar_open: false,
            is_modal_sidebar_open: false,
            selected_menu_item: "Inbox".to_string(),
            dismissible_selected_menu_item: "Dashboard".to_string(),
            modal_selected_menu_item: "Home".to_string(),
        }
    }
}

impl DrawerWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        // Handle ESC key to close all drawers and the window
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.modal_drawer_open = false;
            self.dismissible_drawer_open = false;
            self.is_dismissible_sidebar_open = false;
            self.is_modal_sidebar_open = false;
            // Don't close the main window on ESC, only the drawers
        }

        // Main drawer stories window
        let mut open = self.open;
        Window::new("Drawer Stories")
            .open(&mut open)
            .default_size([1200.0, 800.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_drawer_examples(ui);
                });
            });
        self.open = open;

        // Only show drawer demos when specifically requested and in demo mode
        self.show_active_drawer_demo(ctx);
    }

    fn show_active_drawer_demo(&mut self, ctx: &egui::Context) {
        // Show the selected drawer type as a separate demo window
        match self.selected_drawer {
            DrawerType::Permanent => {
                if self.permanent_drawer_open {
                    self.show_permanent_drawer_demo(ctx);
                }
            }
            DrawerType::Modal => {
                if self.modal_drawer_open {
                    self.show_modal_drawer_demo(ctx);
                }
            }
            DrawerType::Dismissible => {
                if self.dismissible_drawer_open {
                    self.show_dismissible_drawer_demo(ctx);
                }
            }
        }
    }

    fn show_permanent_drawer_demo(&mut self, ctx: &egui::Context) {
        let mut demo_open = self.permanent_drawer_open;
        Window::new("🔒 Permanent Drawer Demo")
            .open(&mut demo_open)
            .default_size([1000.0, 800.0])
            .resizable(true)
            .show(ctx, |ui| {
                let available_width = ui.available_width();
                let available_height = ui.available_height();
                let drawer_width = self.drawer_width.min(available_width * 0.4); // Max 40% of window
                
                // Allocate full height for the horizontal layout
                ui.allocate_ui_with_layout(
                    egui::vec2(available_width, available_height),
                    egui::Layout::left_to_right(egui::Align::Min),
                    |ui| {
                    // Left side drawer - fixed width
                    ui.allocate_ui_with_layout(
                        egui::vec2(drawer_width, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(drawer_width);
                            
                            egui::ScrollArea::vertical()
                                .id_source("permanent_drawer_scroll")
                                .show(ui, |ui| {
                                // Header
                                if self.show_header {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(16.0);
                                        ui.heading("Mail");
                                        ui.colored_label(get_global_color("onSurfaceVariant"), "email@material.io");
                                        ui.add_space(16.0);
                                    });
                                }
                                
                                // Multi-section navigation demonstration
                                if self.show_sections {
                                    // Mail section
                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(28.0);
                                        ui.colored_label(
                                            get_global_color("onSurfaceVariant"),
                                            "Mail"
                                        );
                                    });
                                    ui.add_space(8.0);

                                    self.render_navigation_items(ui, &self.create_mail_items());

                                    // Divider
                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(8.0);

                                    // Labels section
                                    ui.horizontal(|ui| {
                                        ui.add_space(28.0);
                                        ui.colored_label(
                                            get_global_color("onSurfaceVariant"),
                                            "Labels"
                                        );
                                    });
                                    ui.add_space(8.0);

                                    self.render_navigation_items(ui, &self.create_label_items());
                                } else {
                                    // Simple flat list
                                    ui.add_space(16.0);
                                    self.render_navigation_items(ui, &self.create_mail_items());
                                }
                            });
                        },
                    );
                    
                    ui.separator();
                    
                    // Right side - main content area (takes remaining space)
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .id_source("permanent_drawer_content_scroll")
                                .show(ui, |ui| {
                                ui.add_space(20.0);
                                ui.heading(format!("{} - Content Area", self.selected_menu_item));
                                ui.colored_label(
                                    get_global_color("onSurfaceVariant"),
                                    "The permanent drawer is always visible and adjusts the content layout."
                                );
                                
                                ui.add_space(20.0);
                                
                                // Dynamic content based on selection
                                self.render_content_for_selection(&self.selected_menu_item, ui);
                                
                                ui.add_space(20.0);
                                ui.separator();
                                ui.add_space(10.0);
                                
                                ui.colored_label(get_global_color("onSurfaceVariant"), "✨ Features:");
                                ui.label("• Click different items in the sidebar to see content change");
                                ui.label("• Drawer remains fixed and always visible");
                                ui.label("• Content area scales with available space");
                                ui.label("• Hover states and active indicators");
                                ui.label("• Section-based organization (Mail / Labels)");
                                ui.label("• Badge support for notifications");
                            });
                        },
                    );
                    });
            });
        self.permanent_drawer_open = demo_open;
    }

    /// Renders navigation items with Material 3 styling
    fn render_navigation_items(&mut self, ui: &mut Ui, items: &[DrawerItem]) {
        for item in items {
            let is_selected = item.active;
            let item_height = 56.0;
            let horizontal_padding = 12.0;

            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), item_height),
                egui::Sense::click(),
            );

            // Background for active/hover states
            if is_selected {
                let container_rect = rect.shrink2(egui::vec2(horizontal_padding, 12.0));
                ui.painter().rect_filled(
                    container_rect,
                    self.corner_radius.min(28.0),
                    get_global_color("secondaryContainer"),
                );
            } else if response.hovered() {
                let container_rect = rect.shrink2(egui::vec2(horizontal_padding, 12.0));
                ui.painter().rect_filled(
                    container_rect,
                    self.corner_radius.min(28.0),
                    get_global_color("onSurface").linear_multiply(0.08),
                );
            }

            // Content layout
            let mut cursor_x = rect.min.x + horizontal_padding + 16.0;

            // Icon
            if let Some(icon) = &item.icon {
                let icon_center = egui::pos2(cursor_x + 12.0, rect.center().y);
                let text_color = if is_selected {
                    get_global_color("onSecondaryContainer")
                } else {
                    get_global_color("onSurfaceVariant")
                };

                ui.painter().text(
                    icon_center,
                    egui::Align2::CENTER_CENTER,
                    icon,
                    egui::FontId::proportional(20.0),
                    text_color,
                );
                cursor_x += 40.0;
            }

            // Label
            let text_color = if is_selected {
                get_global_color("onSecondaryContainer")
            } else {
                get_global_color("onSurfaceVariant")
            };

            ui.painter().text(
                egui::pos2(cursor_x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &item.text,
                egui::FontId::proportional(14.0),
                text_color,
            );

            // Badge
            if let Some(badge) = &item.badge {
                let badge_x = rect.max.x - horizontal_padding - 30.0;
                let badge_center = egui::pos2(badge_x, rect.center().y);
                
                ui.painter().circle_filled(
                    badge_center,
                    12.0,
                    get_global_color("error"),
                );
                
                ui.painter().text(
                    badge_center,
                    egui::Align2::CENTER_CENTER,
                    badge,
                    egui::FontId::proportional(11.0),
                    get_global_color("onError"),
                );
            }

            // Handle click
            if response.clicked() {
                self.selected_menu_item = item.text.clone();
            }
        }
    }

    /// Renders content for a selection
    fn render_content_for_selection(&self, selection: &str, ui: &mut Ui) {
        match selection {
            "Inbox" => {
                ui.heading("📥 Inbox");
                ui.label("Welcome to your inbox! Here you'll find all your incoming messages.");
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label("📧 New message from John Doe");
                    ui.label("💼 Meeting invitation from Sarah");
                    ui.label("📰 Newsletter from Tech Blog");
                    ui.label("⚙️ System notification");
                    ui.label("🎉 Congratulations on your achievement!");
                });
            },
            "Outbox" => {
                ui.heading("📤 Outbox");
                ui.label("Messages you've sent recently:");
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label("✅ Reply to project update");
                    ui.label("🤝 Meeting confirmation");
                    ui.label("📊 Weekly report submission");
                    ui.label("💌 Thank you note to team");
                });
            },
            "Favorites" => {
                ui.heading("⭐ Favorites");
                ui.label("Your starred and important messages:");
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label("🌟 Important client communication");
                    ui.label("📌 Pinned project details");
                    ui.label("🎯 Key milestones and deadlines");
                    ui.label("💡 Saved ideas and inspiration");
                });
            },
            "Trash" => {
                ui.heading("🗑️ Trash");
                ui.label("Recently deleted messages (can be restored):");
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label("📧 Old newsletter");
                    ui.label("⚠️ Spam message");
                    ui.label("📅 Outdated meeting invite");
                    ui.label("🔄 Duplicate notification");
                });
            },
            "Family" | "School" | "Work" => {
                ui.heading(format!("🔖 {}", selection));
                ui.label(format!("Messages labeled as '{}'", selection));
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(format!("📝 {} related conversations", selection));
                    ui.label(format!("📎 {} documents and files", selection));
                    ui.label(format!("👥 {} group messages", selection));
                    ui.label(format!("📅 {} calendar events", selection));
                });
            },
            _ => {
                ui.label("Select a menu item from the navigation drawer");
            }
        }
    }

    fn show_dismissible_drawer_demo(&mut self, ctx: &egui::Context) {
        // Show as a separate demo window
        let mut demo_open = self.dismissible_drawer_open;
        Window::new("Dismissible Drawer Demo")
            .open(&mut demo_open)
            .default_size([800.0, 600.0])
            .resizable(true)
            .show(ctx, |ui| {
                // Top menu with hamburger button
                egui::TopBottomPanel::top("dismissible_top_panel").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.add(MaterialButton::filled("☰").small()).clicked() {
                            self.is_dismissible_sidebar_open = !self.is_dismissible_sidebar_open;
                        }
                        ui.colored_label(get_global_color("onSurface"), "Top Menu - Dismissible Drawer Demo");
                    });
                });

                ui.horizontal(|ui| {
                    // Conditionally show left side drawer
                    if self.is_dismissible_sidebar_open {
                        ui.vertical(|ui| {
                            ui.set_width(self.drawer_width);
                            ui.set_min_width(self.drawer_width);
                            ui.set_max_width(self.drawer_width);
                            
                            // Header
                            ui.vertical_centered(|ui| {
                                ui.heading("Navigation");
                                if self.show_header {
                                    ui.colored_label(get_global_color("onSurfaceVariant"), "App Menu");
                                    ui.add_space(8.0);
                                }
                            });
                            
                            ui.add_space(16.0);
                            
                            // Menu items with proper Material Design styling
                            let item_height = 48.0;
                            let menu_items = [
                                ("Dashboard", "📊"),
                                ("Analytics", "📈"),
                                ("Reports", "📋"),
                                ("Users", "👥"),
                            ];
                            
                            for (item_name, icon) in &menu_items {
                                let is_selected = self.dismissible_selected_menu_item == *item_name;
                                
                                // Create a proper menu item with correct sizing
                                let item_rect = ui.allocate_space([ui.available_width(), item_height].into()).1;
                                let response = ui.interact(item_rect, egui::Id::new(format!("dismissible_{}", item_name)), egui::Sense::click());
                                
                                // Style based on selection and hover state
                                if is_selected {
                                    ui.painter().rect_filled(
                                        item_rect,
                                        egui::CornerRadius::same(12),
                                        get_global_color("primaryContainer"),
                                    );
                                } else if response.hovered() {
                                    ui.painter().rect_filled(
                                        item_rect,
                                        egui::CornerRadius::same(12),
                                        get_global_color("surfaceVariant").linear_multiply(0.08),
                                    );
                                }
                                
                                // Draw icon and text
                                let text_color = if is_selected {
                                    get_global_color("onPrimaryContainer")
                                } else {
                                    get_global_color("onSurface")
                                };
                                
                                let text_y = item_rect.min.y + (item_height - 20.0) / 2.0;
                                let mut text_x = item_rect.min.x + 16.0;
                                
                                if self.show_icons {
                                    ui.painter().text(
                                        egui::pos2(text_x, text_y),
                                        egui::Align2::LEFT_CENTER,
                                        icon,
                                        egui::FontId::proportional(16.0),
                                        text_color,
                                    );
                                    text_x += 32.0;
                                }
                                
                                ui.painter().text(
                                    egui::pos2(text_x, text_y),
                                    egui::Align2::LEFT_CENTER,
                                    item_name,
                                    egui::FontId::proportional(14.0),
                                    text_color,
                                );
                                
                                if response.clicked() {
                                    self.dismissible_selected_menu_item = item_name.to_string();
                                }
                            }
                            
                            // Fill remaining vertical space
                            ui.allocate_space([ui.available_width(), ui.available_height()].into());
                        });
                        
                        ui.separator();
                    }

                    // Main content area - takes remaining space
                    ui.vertical(|ui| {
                        ui.heading(format!("{} - Dismissible Drawer Demo", self.dismissible_selected_menu_item));
                        ui.colored_label(get_global_color("onSurfaceVariant"), "This drawer can be toggled open/closed and adjusts the content layout.");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "Click the hamburger menu (☰) in the top menu to toggle the sidebar.");
                        
                        ui.add_space(20.0);
                        ui.label(format!("Sidebar is currently: {}", 
                            if self.is_dismissible_sidebar_open { "OPEN" } else { "CLOSED" }));
                        
                        ui.add_space(10.0);
                        if ui.add(MaterialButton::filled("Toggle Drawer").small()).clicked() {
                            self.is_dismissible_sidebar_open = !self.is_dismissible_sidebar_open;
                        }
                        
                        ui.add_space(20.0);
                        
                        // Content changes based on selected menu item
                        match self.dismissible_selected_menu_item.as_str() {
                            "Dashboard" => {
                                ui.heading("📊 Dashboard");
                                ui.label("Welcome to your dashboard! Here's an overview of your system.");
                                ui.add_space(10.0);
                                ui.label("• Active users: 1,234");
                                ui.label("• Total revenue: $45,678");
                                ui.label("• System uptime: 99.9%");
                                ui.label("• Recent activity: 156 actions");
                            },
                            "Analytics" => {
                                ui.heading("📈 Analytics");
                                ui.label("Data analytics and insights:");
                                ui.add_space(10.0);
                                ui.label("• Page views: 25,678 this week");
                                ui.label("• Bounce rate: 32%");
                                ui.label("• Average session: 4m 23s");
                                ui.label("• Conversion rate: 2.4%");
                            },
                            "Reports" => {
                                ui.heading("📋 Reports");
                                ui.label("Generated reports and documents:");
                                ui.add_space(10.0);
                                ui.label("• Monthly sales report");
                                ui.label("• User engagement analysis");
                                ui.label("• Performance metrics summary");
                                ui.label("• Quality assurance report");
                            },
                            "Users" => {
                                ui.heading("👥 Users");
                                ui.label("User management and information:");
                                ui.add_space(10.0);
                                ui.label("• Total users: 5,432");
                                ui.label("• Active today: 234");
                                ui.label("• New registrations: 45");
                                ui.label("• Premium subscribers: 1,234");
                            },
                            _ => {
                                ui.label("Select a menu item from the sidebar");
                            }
                        }
                        
                        ui.add_space(20.0);
                        ui.colored_label(get_global_color("onSurfaceVariant"), "Behavior:");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "• When dismissed, content expands to fill the full width");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "• When shown, content is constrained by the drawer width");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "• Click sidebar items to change content");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "• Press ESC to close the drawer");
                    });
                });
            });
        self.dismissible_drawer_open = demo_open;
    }

    fn show_modal_drawer_demo(&mut self, ctx: &egui::Context) {
        // Show as a separate demo window
        let mut demo_open = self.modal_drawer_open;
        Window::new("Modal Drawer Demo")
            .open(&mut demo_open)
            .default_size([800.0, 600.0])
            .resizable(true)
            .show(ctx, |ui| {
                // Top menu with hamburger button
                egui::TopBottomPanel::top("modal_top_panel").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.add(MaterialButton::filled("☰").small()).clicked() {
                            self.is_modal_sidebar_open = !self.is_modal_sidebar_open;
                        }
                        ui.colored_label(
                            get_global_color("onSurface"),
                            "Top Menu - Modal Drawer Demo",
                        );
                    });
                });

                // Main content area (always visible)
                ui.vertical(|ui| {
                    ui.heading(format!(
                        "{} - Modal Drawer Demo",
                        self.modal_selected_menu_item
                    ));
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "This drawer overlays the content with a semi-transparent scrim.",
                    );
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "Click the hamburger menu (☰) in the top menu to show the modal sidebar.",
                    );

                    ui.add_space(20.0);
                    ui.label(format!(
                        "Modal sidebar is currently: {}",
                        if self.is_modal_sidebar_open {
                            "OPEN"
                        } else {
                            "CLOSED"
                        }
                    ));

                    ui.add_space(10.0);
                    if ui.add(MaterialButton::filled("Toggle Modal Drawer").small()).clicked() {
                        // self.is_modal_sidebar_open = !self.is_modal_sidebar_open;
                    }

                    ui.add_space(20.0);

                    // Content changes based on selected menu item
                    match self.modal_selected_menu_item.as_str() {
                        "Home" => {
                            ui.heading("🏠 Home");
                            ui.label("Welcome to the home page! This is your starting point.");
                            ui.add_space(10.0);
                            ui.label("• Recent activities");
                            ui.label("• Quick actions");
                            ui.label("• System overview");
                            ui.label("• Navigation shortcuts");
                        }
                        "Profile" => {
                            ui.heading("👤 Profile");
                            ui.label("Manage your profile information and preferences:");
                            ui.add_space(10.0);
                            ui.label("• Personal information");
                            ui.label("• Avatar and display name");
                            ui.label("• Contact details");
                            ui.label("• Privacy settings");
                        }
                        "Settings" => {
                            ui.heading("⚙️ Settings");
                            ui.label("Configure application settings:");
                            ui.add_space(10.0);
                            ui.label("• Theme preferences");
                            ui.label("• Notification settings");
                            ui.label("• Security options");
                            ui.label("• Data management");
                        }
                        "Help" => {
                            ui.heading("❓ Help");
                            ui.label("Get assistance and support:");
                            ui.add_space(10.0);
                            ui.label("• Documentation");
                            ui.label("• FAQ and tutorials");
                            ui.label("• Contact support");
                            ui.label("• Community forums");
                        }
                        "Dashboard" => {
                            ui.heading("📊 Dashboard");
                            ui.label("Overview of system metrics and key performance indicators:");
                            ui.add_space(10.0);
                            ui.label("• Active users: 1,234");
                            ui.label("• Total revenue: $45,678");
                            ui.label("• System uptime: 99.9%");
                            ui.label("• Recent activity: 156 actions");
                        }
                        "Analytics" => {
                            ui.heading("📈 Analytics");
                            ui.label("Data analytics and insights for informed decision making:");
                            ui.add_space(10.0);
                            ui.label("• Page views: 25,678 this week");
                            ui.label("• Bounce rate: 32%");
                            ui.label("• Average session: 4m 23s");
                            ui.label("• Conversion rate: 2.4%");
                        }
                        "Reports" => {
                            ui.heading("📋 Reports");
                            ui.label("Generated reports and documents for analysis:");
                            ui.add_space(10.0);
                            ui.label("• Monthly sales report");
                            ui.label("• User engagement analysis");
                            ui.label("• Performance metrics summary");
                            ui.label("• Quality assurance report");
                        }
                        "Users" => {
                            ui.heading("👥 Users");
                            ui.label("User management and account information:");
                            ui.add_space(10.0);
                            ui.label("• Total users: 5,432");
                            ui.label("• Active today: 234");
                            ui.label("• New registrations: 45");
                            ui.label("• Premium subscribers: 1,234");
                        }
                        "Messages" => {
                            ui.heading("💬 Messages");
                            ui.label("Communication and messaging center:");
                            ui.add_space(10.0);
                            ui.label("• Unread messages: 12");
                            ui.label("• Sent today: 8");
                            ui.label("• Team conversations: 4");
                            ui.label("• Direct messages: 16");
                        }
                        "Calendar" => {
                            ui.heading("📅 Calendar");
                            ui.label("Schedule and event management:");
                            ui.add_space(10.0);
                            ui.label("• Upcoming meetings: 5");
                            ui.label("• Today's events: 3");
                            ui.label("• This week: 12 events");
                            ui.label("• Reminders: 8 pending");
                        }
                        "Files" => {
                            ui.heading("📁 Files");
                            ui.label("File storage and document management:");
                            ui.add_space(10.0);
                            ui.label("• Total files: 2,456");
                            ui.label("• Storage used: 4.2 GB");
                            ui.label("• Recent uploads: 23");
                            ui.label("• Shared folders: 8");
                        }
                        "Tasks" => {
                            ui.heading("✅ Tasks");
                            ui.label("Task management and productivity tracking:");
                            ui.add_space(10.0);
                            ui.label("• Open tasks: 15");
                            ui.label("• Completed today: 7");
                            ui.label("• Overdue: 2");
                            ui.label("• This week: 23 tasks");
                        }
                        _ => {
                            ui.label("Select a menu item from the modal sidebar");
                        }
                    }

                    ui.add_space(20.0);
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "Modal drawer characteristics:",
                    );
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "• Overlays content without changing layout",
                    );
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "• Semi-transparent scrim blocks interaction",
                    );
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "• Drawer slides in from the side",
                    );
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "• Click outside or ESC to close",
                    );
                    ui.colored_label(
                        get_global_color("onSurfaceVariant"),
                        "• Click menu items to change content",
                    );
                });

                // Modal overlay (when open) - appears on top with constrained size
                if self.is_modal_sidebar_open {
                    // Get the window bounds for constrained overlay
                    let available_rect = ui.max_rect();

                    // Semi-transparent scrim over available window content only
                    let scrim_response =
                        ui.allocate_response(available_rect.size(), egui::Sense::click());

                    ui.painter().rect_filled(
                        available_rect,
                        egui::CornerRadius::ZERO,
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 128),
                    );

                    // Left-sided modal drawer panel - constrained to available space
                    let drawer_width = self.drawer_width.min(available_rect.width() * 0.8); // Max 80% of window width
                    let drawer_rect = egui::Rect::from_min_size(
                        available_rect.min,
                        egui::Vec2::new(drawer_width, available_rect.height()),
                    );

                    ui.scope_builder(egui::UiBuilder::new().max_rect(drawer_rect), |ui| {
                        egui::Frame::new()
                            .fill(get_global_color("surface"))
                            .stroke(egui::Stroke::new(1.0, get_global_color("outline")))
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    // Fixed header section
                                    ui.vertical_centered(|ui| {
                                        ui.heading("Menu");
                                        if self.show_header {
                                            ui.colored_label(
                                                get_global_color("onSurfaceVariant"),
                                                "Choose an option",
                                            );
                                            ui.add_space(8.0);
                                        }
                                    });

                                    ui.add_space(16.0);

                                    // Scrollable content area - constrained to available height
                                    let _header_height = 80.0; // Approximate height of header section
                                    let button_height = 40.0; // Height for close button
                                    let available_height =
                                        ui.available_height() - button_height - 20.0; // Reserve space for button and padding

                                    egui::ScrollArea::vertical()
                                        .max_height(available_height)
                                        .show(ui, |ui| {
                                            // Menu items with proper Material Design styling
                                            let item_height = 48.0;
                                            let menu_items = [
                                                ("Home", "🏠"),
                                                ("Profile", "👤"),
                                                ("Settings", "⚙️"),
                                                ("Help", "❓"),
                                                // Add more items to demonstrate scrolling
                                                ("Dashboard", "📊"),
                                                ("Analytics", "📈"),
                                                ("Reports", "📋"),
                                                ("Users", "👥"),
                                                ("Messages", "💬"),
                                                ("Calendar", "📅"),
                                                ("Files", "📁"),
                                                ("Tasks", "✅"),
                                            ];

                                            for (item_name, icon) in &menu_items {
                                                let is_selected =
                                                    self.modal_selected_menu_item == *item_name;

                                                // Create a proper menu item with correct sizing
                                                let item_rect = ui
                                                    .allocate_space(
                                                        [ui.available_width(), item_height].into(),
                                                    )
                                                    .1;
                                                let response = ui.interact(
                                                    item_rect,
                                                    egui::Id::new(format!("modal_{}", item_name)),
                                                    egui::Sense::click(),
                                                );

                                                // Style based on selection and hover state
                                                if is_selected {
                                                    ui.painter().rect_filled(
                                                        item_rect,
                                                        egui::CornerRadius::same(12),
                                                        get_global_color("primaryContainer"),
                                                    );
                                                } else if response.hovered() {
                                                    ui.painter().rect_filled(
                                                        item_rect,
                                                        egui::CornerRadius::same(12),
                                                        get_global_color("surfaceVariant")
                                                            .linear_multiply(0.08),
                                                    );
                                                }

                                                // Draw icon and text
                                                let text_color = if is_selected {
                                                    get_global_color("onPrimaryContainer")
                                                } else {
                                                    get_global_color("onSurface")
                                                };

                                                let text_y =
                                                    item_rect.min.y + (item_height - 20.0) / 2.0;
                                                let mut text_x = item_rect.min.x + 16.0;

                                                if self.show_icons {
                                                    ui.painter().text(
                                                        egui::pos2(text_x, text_y),
                                                        egui::Align2::LEFT_CENTER,
                                                        icon,
                                                        egui::FontId::proportional(16.0),
                                                        text_color,
                                                    );
                                                    text_x += 32.0;
                                                }

                                                ui.painter().text(
                                                    egui::pos2(text_x, text_y),
                                                    egui::Align2::LEFT_CENTER,
                                                    item_name,
                                                    egui::FontId::proportional(14.0),
                                                    text_color,
                                                );

                                                if response.clicked() {
                                                    self.modal_selected_menu_item =
                                                        item_name.to_string();
                                                }
                                            }
                                        });

                                    ui.add_space(20.0);

                                    // Fixed close button at bottom
                                    if ui.add(MaterialButton::filled("Close Modal").small()).clicked() {
                                        self.is_modal_sidebar_open = false;
                                    }
                                });
                            });
                    });

                    // Close modal if scrim area (not on the drawer) is clicked
                    if scrim_response.clicked() {
                        if let Some(pointer_pos) = scrim_response.interact_pointer_pos() {
                            if !drawer_rect.contains(pointer_pos) {
                                self.is_modal_sidebar_open = false;
                            }
                        }
                    }
                }
            });

        self.modal_drawer_open = demo_open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.push_id("drawer_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Drawer Controls");

                if ui.add(MaterialButton::filled("📖 Material Design Spec").small()).clicked() {
                    let _ = webbrowser::open("https://m3.material.io/components/navigation-drawer/overview");
                }
            });

            ui.separator();

            // Visual Options
            ui.heading("Visual Options");
            ui.horizontal_wrapped(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.show_header, "Show Header"));
                ui.add(MaterialCheckbox::new(&mut self.show_icons, "Show Icons"));
                ui.add(MaterialCheckbox::new(&mut self.show_badges, "Show Badges"));
                ui.add(MaterialCheckbox::new(&mut self.show_sections, "Show Sections"));
                ui.add(MaterialCheckbox::new(&mut self.use_material3, "Material 3 Theme"));
            });

            ui.add_space(8.0);

            // Dimension Controls
            ui.horizontal(|ui| {
                ui.label("Drawer Width:");
                ui.add(egui::Slider::new(&mut self.drawer_width, 200.0..=500.0).suffix("px"));
            });

            ui.horizontal(|ui| {
                ui.label("Elevation:");
                ui.add(egui::Slider::new(&mut self.drawer_elevation, 0.0..=24.0).suffix("dp"));
            });

            ui.horizontal(|ui| {
                ui.label("Corner Radius:");
                ui.add(egui::Slider::new(&mut self.corner_radius, 0.0..=28.0).suffix("px"));
            });

            ui.horizontal(|ui| {
                ui.label("Scrim Opacity:");
                ui.add(egui::Slider::new(&mut self.scrim_opacity, 0.0..=1.0));
            });

            ui.add_space(8.0);
            ui.separator();

            // Drawer Type Selection
            ui.heading("Drawer Type");
            ui.horizontal(|ui| {
                if ui.selectable_label(matches!(self.selected_drawer, DrawerType::Permanent), "🔒 Permanent").clicked() {
                    self.selected_drawer = DrawerType::Permanent;
                    self.permanent_drawer_open = true;
                    self.modal_drawer_open = false;
                    self.dismissible_drawer_open = false;
                }
                
                if ui.selectable_label(matches!(self.selected_drawer, DrawerType::Dismissible), "↔️ Dismissible").clicked() {
                    self.selected_drawer = DrawerType::Dismissible;
                    self.permanent_drawer_open = false;
                    self.modal_drawer_open = false;
                    self.dismissible_drawer_open = true;
                }
                
                if ui.selectable_label(matches!(self.selected_drawer, DrawerType::Modal), "📱 Modal").clicked() {
                    self.selected_drawer = DrawerType::Modal;
                    self.permanent_drawer_open = false;
                    self.modal_drawer_open = true;
                    self.dismissible_drawer_open = false;
                }
            });

            ui.add_space(8.0);

            // Toggle Buttons
            ui.horizontal(|ui| {
                match self.selected_drawer {
                    DrawerType::Permanent => {
                        if ui.add(MaterialButton::filled("Toggle Permanent Demo")).clicked() {
                            self.permanent_drawer_open = !self.permanent_drawer_open;
                        }
                    }
                    DrawerType::Dismissible => {
                        if ui.add(MaterialButton::filled("Toggle Dismissible Demo")).clicked() {
                            self.dismissible_drawer_open = !self.dismissible_drawer_open;
                        }
                    }
                    DrawerType::Modal => {
                        if ui.add(MaterialButton::filled("Toggle Modal Demo")).clicked() {
                            self.modal_drawer_open = !self.modal_drawer_open;
                        }
                    }
                }

                if ui.add(MaterialButton::text("Reset Settings")).clicked() {
                    *self = Self { open: self.open, ..Default::default() };
                }
            });
        });
    }

    fn render_drawer_examples(&mut self, ui: &mut Ui) {
        
    }

    /// Helper to create navigation items based on current settings
    fn create_mail_items(&self) -> Vec<DrawerItem> {
        let items_data = [
            ("Inbox", "📥", 5),
            ("Outbox", "📤", 0),
            ("Favorites", "⭐", 2),
            ("Trash", "🗑️", 0),
        ];

        items_data.iter().map(|(name, icon, badge_count)| {
            let mut item = DrawerItem::new(*name)
                .active(self.selected_menu_item == *name);

            if self.show_icons {
                item = item.icon(*icon);
            }

            if self.show_badges && *badge_count > 0 {
                item = item.badge(badge_count.to_string());
            }

            item
        }).collect()
    }

    /// Helper to create label section items
    fn create_label_items(&self) -> Vec<DrawerItem> {
        let items_data = [
            ("Family", "🔖", 0),
            ("School", "🔖", 1),
            ("Work", "🔖", 3),
        ];

        items_data.iter().map(|(name, icon, badge_count)| {
            let mut item = DrawerItem::new(*name)
                .active(self.selected_menu_item == *name);

            if self.show_icons {
                item = item.icon(*icon);
            }

            if self.show_badges && *badge_count > 0 {
                item = item.badge(badge_count.to_string());
            }

            item
        }).collect()
    }
}
