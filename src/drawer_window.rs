use eframe::egui::{self, Ui, Window};
use crate::{MaterialButton, MaterialCheckbox};
use crate::theme::get_global_color;

pub struct DrawerWindow {
    pub open: bool,
    permanent_drawer_open: bool,
    modal_drawer_open: bool,
    dismissible_drawer_open: bool,
    show_header: bool,
    show_icons: bool,
    drawer_width: f32,
    selected_drawer: DrawerType,
    // New state variables for drawer demos
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
            permanent_drawer_open: false, // Changed from true to false
            modal_drawer_open: false,
            dismissible_drawer_open: false,
            show_header: true,
            show_icons: true,
            drawer_width: 256.0,
            selected_drawer: DrawerType::Permanent,
            // Initialize new state variables
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
        // Show as a separate demo window
        let mut demo_open = self.permanent_drawer_open;
        Window::new("Permanent Drawer Demo")
            .open(&mut demo_open)
            .default_size([800.0, 600.0])
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Left side drawer - fixed width
                    ui.vertical(|ui| {
                        ui.set_width(self.drawer_width);
                        ui.set_min_width(self.drawer_width);
                        ui.set_max_width(self.drawer_width);
                        
                        // Header
                        ui.vertical_centered(|ui| {
                            ui.heading("Mail");
                            if self.show_header {
                                ui.colored_label(get_global_color("onSurfaceVariant"), "email@material.io");
                                ui.add_space(8.0);
                            }
                        });
                        
                        ui.add_space(16.0);
                        
                        // Menu items with proper Material Design styling
                        let item_height = 48.0;
                        let menu_items = [
                            ("Inbox", "📥"),
                            ("Sent", "📤"), 
                            ("Drafts", "📄"),
                            ("Trash", "🗑"),
                        ];
                        
                        for (item_name, icon) in &menu_items {
                            let is_selected = self.selected_menu_item == *item_name;
                            
                            // Create a proper menu item with correct sizing
                            let item_rect = ui.allocate_space([ui.available_width(), item_height].into()).1;
                            let response = ui.interact(item_rect, egui::Id::new(*item_name), egui::Sense::click());
                            
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
                                self.selected_menu_item = item_name.to_string();
                            }
                        }
                        
                        // Fill remaining vertical space
                        ui.allocate_space([ui.available_width(), ui.available_height()].into());
                    });
                    
                    ui.separator();
                    
                    // Right side - main content area
                    ui.vertical(|ui| {
                        ui.heading(format!("{} - Permanent Drawer Demo", self.selected_menu_item));
                        ui.colored_label(get_global_color("onSurfaceVariant"), "This drawer is always visible and adjusts the content layout.");
                        
                        ui.add_space(20.0);
                        
                        // Content changes based on selected menu item
                        match self.selected_menu_item.as_str() {
                            "Inbox" => {
                                ui.heading("📥 Inbox");
                                ui.label("Welcome to your inbox! Here you'll find all your incoming messages.");
                                ui.add_space(10.0);
                                ui.label("• New message from John Doe");
                                ui.label("• Meeting invitation from Sarah");
                                ui.label("• Newsletter from Tech Blog");
                                ui.label("• System notification");
                            },
                            "Sent" => {
                                ui.heading("📤 Sent Messages");
                                ui.label("Here are the messages you've sent recently:");
                                ui.add_space(10.0);
                                ui.label("• Reply to project update");
                                ui.label("• Meeting confirmation");
                                ui.label("• Weekly report submission");
                                ui.label("• Thank you note to team");
                            },
                            "Drafts" => {
                                ui.heading("📄 Draft Messages");
                                ui.label("Your unsent draft messages:");
                                ui.add_space(10.0);
                                ui.label("• Incomplete email to client");
                                ui.label("• Follow-up message draft");
                                ui.label("• Proposal outline");
                                ui.label("• Feedback notes");
                            },
                            "Trash" => {
                                ui.heading("🗑 Deleted Messages");
                                ui.label("Recently deleted messages (can be restored):");
                                ui.add_space(10.0);
                                ui.label("• Old newsletter");
                                ui.label("• Spam message");
                                ui.label("• Outdated meeting invite");
                                ui.label("• Duplicate notification");
                            },
                            _ => {
                                ui.label("Select a menu item from the sidebar");
                            }
                        }
                        
                        ui.add_space(20.0);
                        ui.colored_label(get_global_color("onSurfaceVariant"), "Features:");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "• Click different items in the sidebar to see content change");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "• Drawer remains fixed and always visible");
                        ui.colored_label(get_global_color("onSurfaceVariant"), "• Content area scales with available space");
                    });
                });
            });
        self.permanent_drawer_open = demo_open;
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
                        if ui.button("☰").clicked() {
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
                        if ui.button("Toggle Drawer").clicked() {
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
                        if ui.button("☰").clicked() {
                            self.is_modal_sidebar_open = !self.is_modal_sidebar_open;
                        }
                        ui.colored_label(get_global_color("onSurface"), "Top Menu - Modal Drawer Demo");
                    });
                });

                // Main content area (always visible)
                ui.vertical(|ui| {
                    ui.heading(format!("{} - Modal Drawer Demo", self.modal_selected_menu_item));
                    ui.colored_label(get_global_color("onSurfaceVariant"), "This drawer overlays the content with a semi-transparent scrim.");
                    ui.colored_label(get_global_color("onSurfaceVariant"), "Click the hamburger menu (☰) in the top menu to show the modal sidebar.");
                    
                    ui.add_space(20.0);
                    ui.label(format!("Modal sidebar is currently: {}", 
                        if self.is_modal_sidebar_open { "OPEN" } else { "CLOSED" }));
                    
                    ui.add_space(10.0);
                    if ui.button("Toggle Modal Drawer").clicked() {
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
                        },
                        "Profile" => {
                            ui.heading("👤 Profile");
                            ui.label("Manage your profile information and preferences:");
                            ui.add_space(10.0);
                            ui.label("• Personal information");
                            ui.label("• Avatar and display name");
                            ui.label("• Contact details");
                            ui.label("• Privacy settings");
                        },
                        "Settings" => {
                            ui.heading("⚙️ Settings");
                            ui.label("Configure application settings:");
                            ui.add_space(10.0);
                            ui.label("• Theme preferences");
                            ui.label("• Notification settings");
                            ui.label("• Security options");
                            ui.label("• Data management");
                        },
                        "Help" => {
                            ui.heading("❓ Help");
                            ui.label("Get assistance and support:");
                            ui.add_space(10.0);
                            ui.label("• Documentation");
                            ui.label("• FAQ and tutorials");
                            ui.label("• Contact support");
                            ui.label("• Community forums");
                        },
                        "Dashboard" => {
                            ui.heading("📊 Dashboard");
                            ui.label("Overview of system metrics and key performance indicators:");
                            ui.add_space(10.0);
                            ui.label("• Active users: 1,234");
                            ui.label("• Total revenue: $45,678");
                            ui.label("• System uptime: 99.9%");
                            ui.label("• Recent activity: 156 actions");
                        },
                        "Analytics" => {
                            ui.heading("📈 Analytics");
                            ui.label("Data analytics and insights for informed decision making:");
                            ui.add_space(10.0);
                            ui.label("• Page views: 25,678 this week");
                            ui.label("• Bounce rate: 32%");
                            ui.label("• Average session: 4m 23s");
                            ui.label("• Conversion rate: 2.4%");
                        },
                        "Reports" => {
                            ui.heading("📋 Reports");
                            ui.label("Generated reports and documents for analysis:");
                            ui.add_space(10.0);
                            ui.label("• Monthly sales report");
                            ui.label("• User engagement analysis");
                            ui.label("• Performance metrics summary");
                            ui.label("• Quality assurance report");
                        },
                        "Users" => {
                            ui.heading("👥 Users");
                            ui.label("User management and account information:");
                            ui.add_space(10.0);
                            ui.label("• Total users: 5,432");
                            ui.label("• Active today: 234");
                            ui.label("• New registrations: 45");
                            ui.label("• Premium subscribers: 1,234");
                        },
                        "Messages" => {
                            ui.heading("💬 Messages");
                            ui.label("Communication and messaging center:");
                            ui.add_space(10.0);
                            ui.label("• Unread messages: 12");
                            ui.label("• Sent today: 8");
                            ui.label("• Team conversations: 4");
                            ui.label("• Direct messages: 16");
                        },
                        "Calendar" => {
                            ui.heading("📅 Calendar");
                            ui.label("Schedule and event management:");
                            ui.add_space(10.0);
                            ui.label("• Upcoming meetings: 5");
                            ui.label("• Today's events: 3");
                            ui.label("• This week: 12 events");
                            ui.label("• Reminders: 8 pending");
                        },
                        "Files" => {
                            ui.heading("📁 Files");
                            ui.label("File storage and document management:");
                            ui.add_space(10.0);
                            ui.label("• Total files: 2,456");
                            ui.label("• Storage used: 4.2 GB");
                            ui.label("• Recent uploads: 23");
                            ui.label("• Shared folders: 8");
                        },
                        "Tasks" => {
                            ui.heading("✅ Tasks");
                            ui.label("Task management and productivity tracking:");
                            ui.add_space(10.0);
                            ui.label("• Open tasks: 15");
                            ui.label("• Completed today: 7");
                            ui.label("• Overdue: 2");
                            ui.label("• This week: 23 tasks");
                        },
                        _ => {
                            ui.label("Select a menu item from the modal sidebar");
                        }
                    }
                    
                    ui.add_space(20.0);
                    ui.colored_label(get_global_color("onSurfaceVariant"), "Modal drawer characteristics:");
                    ui.colored_label(get_global_color("onSurfaceVariant"), "• Overlays content without changing layout");
                    ui.colored_label(get_global_color("onSurfaceVariant"), "• Semi-transparent scrim blocks interaction");
                    ui.colored_label(get_global_color("onSurfaceVariant"), "• Drawer slides in from the side");
                    ui.colored_label(get_global_color("onSurfaceVariant"), "• Click outside or ESC to close");
                    ui.colored_label(get_global_color("onSurfaceVariant"), "• Click menu items to change content");
                });

                // Modal overlay (when open) - appears on top with constrained size
                if self.is_modal_sidebar_open {
                    // Get the window bounds for constrained overlay
                    let available_rect = ui.max_rect();
                    
                    // Semi-transparent scrim over available window content only
                    let scrim_response = ui.allocate_response(available_rect.size(), egui::Sense::click());
                    
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
                                            ui.colored_label(get_global_color("onSurfaceVariant"), "Choose an option");
                                            ui.add_space(8.0);
                                        }
                                    });
                                    
                                    ui.add_space(16.0);
                                    
                                    // Scrollable content area - constrained to available height
                                    let _header_height = 80.0; // Approximate height of header section
                                    let button_height = 40.0; // Height for close button
                                    let available_height = ui.available_height() - button_height - 20.0; // Reserve space for button and padding
                                    
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
                                                let is_selected = self.modal_selected_menu_item == *item_name;
                                                
                                                // Create a proper menu item with correct sizing
                                                let item_rect = ui.allocate_space([ui.available_width(), item_height].into()).1;
                                                let response = ui.interact(item_rect, egui::Id::new(format!("modal_{}", item_name)), egui::Sense::click());
                                                
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
                                                    self.modal_selected_menu_item = item_name.to_string();
                                                }
                                            }
                                        });
                                    
                                    ui.add_space(20.0);
                                    
                                    // Fixed close button at bottom
                                    if ui.button("Close Modal").clicked() {
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

                if ui.button("Target").clicked() {
                    let _ = webbrowser::open("https://material-components.github.io/material-components-web-catalog/#/component/drawer");
                }
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(&mut self.show_header, "Show Header"));
                ui.add(MaterialCheckbox::new(&mut self.show_icons, "Show Icons"));
            });

            ui.horizontal(|ui| {
                ui.label("Drawer Width:");
                ui.add(egui::Slider::new(&mut self.drawer_width, 200.0..=400.0).suffix("px"));
            });

            ui.horizontal(|ui| {
                ui.label("Demo Type:");
                
                if ui.selectable_label(matches!(self.selected_drawer, DrawerType::Permanent), "Permanent").clicked() {
                    self.selected_drawer = DrawerType::Permanent;
                    self.permanent_drawer_open = true;
                    self.modal_drawer_open = false;
                    self.dismissible_drawer_open = false;
                }
                
                if ui.selectable_label(matches!(self.selected_drawer, DrawerType::Dismissible), "Dismissible").clicked() {
                    self.selected_drawer = DrawerType::Dismissible;
                    self.permanent_drawer_open = false;
                    self.modal_drawer_open = false;
                    self.dismissible_drawer_open = true;
                }
                
                // if ui.selectable_label(matches!(self.selected_drawer, DrawerType::Modal), "Modal").clicked() {
                //     self.selected_drawer = DrawerType::Modal;
                //     self.permanent_drawer_open = false;
                //     self.modal_drawer_open = true;
                //     self.dismissible_drawer_open = false;
                // }
            });

            ui.horizontal(|ui| {
                match self.selected_drawer {
                    DrawerType::Permanent => {
                        if ui.add(MaterialButton::filled("Toggle Permanent")).clicked() {
                            self.permanent_drawer_open = !self.permanent_drawer_open;
                        }
                    }
                    DrawerType::Dismissible => {
                        if ui.add(MaterialButton::filled("Toggle Dismissible")).clicked() {
                            self.dismissible_drawer_open = !self.dismissible_drawer_open;
                        }
                    }
                    DrawerType::Modal => {
                        //if ui.add(MaterialButton::filled("Toggle Modal")).clicked() {
                            // self.modal_drawer_open = !self.modal_drawer_open;
                        //}
                    }
                }
            });
        });
    }

    fn render_drawer_examples(&mut self, ui: &mut Ui) {
        ui.heading("Material Design Drawer Types");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("🔒 Permanent Drawer:");
                ui.label("• Always visible");
                ui.label("• Content alongside");
                ui.label("• Best for desktop/tablet");
                ui.label("• Wide screens (≥1280px)");
            });
            
            ui.vertical(|ui| {
                ui.label("↔️ Dismissible Drawer:");
                ui.label("• Can be toggled");
                ui.label("• Slides in/out");
                ui.label("• Adjusts content layout");
                ui.label("• Medium screens (≥960px)");
            });
            
            ui.vertical(|ui| {
                ui.label("📱 Modal Drawer:");
                ui.label("• Overlays content");
                ui.label("• Semi-transparent scrim");
                ui.label("• Blocks interaction");
                ui.label("• Small screens (<960px)");
            });
        });

        ui.add_space(20.0);

        ui.heading("Interactive Demo");
        ui.label("Select a drawer type above to see it in action:");
        
        match self.selected_drawer {
            DrawerType::Permanent => {
                ui.label("🔒 Permanent Drawer: Always visible, content adjusts around it.");
            }
            DrawerType::Dismissible => {
                ui.label("↔️ Dismissible Drawer: Toggle to slide in/out, content adjusts.");
            }
            DrawerType::Modal => {
                ui.label("📱 Modal Drawer: Overlays content with scrim, click outside to close.");
            }
        }

        ui.add_space(20.0);

        ui.heading("Material Design Specifications");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("📏 Dimensions:");
                ui.label("• Standard width: 256dp");
                ui.label("• Rail width: 72dp");
                ui.label("• Header height: 64dp");
                ui.label("• Item height: 48dp");
            });

            ui.vertical(|ui| {
                ui.label("🎨 Styling:");
                ui.label("• Surface color background");
                ui.label("• Primary color for active items");
                ui.label("• Material icons (24dp)");
                ui.label("• 16dp horizontal padding");
            });

            ui.vertical(|ui| {
                ui.label("⚡ Behavior:");
                ui.label("• Smooth slide animations");
                ui.label("• ESC key support");
                ui.label("• Focus management");
                ui.label("• Accessibility support");
            });
        });

        ui.add_space(20.0);

        ui.heading("Usage Guidelines");
        ui.label("Choose the appropriate drawer type based on your layout needs:");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.strong("Use Permanent when:");
                ui.label("• Wide desktop layouts");
                ui.label("• Navigation is primary");
                ui.label("• Content benefits from the space");
                ui.label("• Users frequently navigate");
            });

            ui.vertical(|ui| {
                ui.strong("Use Dismissible when:");
                ui.label("• Medium-width layouts");
                ui.label("• Optional navigation");
                ui.label("• Content needs full width");
                ui.label("• Responsive design needed");
            });

            ui.vertical(|ui| {
                ui.strong("Use Modal when:");
                ui.label("• Mobile/narrow layouts");
                ui.label("• Temporary navigation");
                ui.label("• Focus on main content");
                ui.label("• Simple app structure");
            });
        });
    }
}