use eframe::egui::{self, Ui, Window, CentralPanel};
use crate::{MaterialButton, MaterialCheckbox, permanent_drawer, modal_drawer, dismissible_drawer};

pub struct DrawerWindow {
    pub open: bool,
    permanent_drawer_open: bool,
    modal_drawer_open: bool,
    dismissible_drawer_open: bool,
    show_header: bool,
    show_icons: bool,
    drawer_width: f32,
    selected_drawer: DrawerType,
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
        }
    }
}

impl DrawerWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        // Handle ESC key to close all drawers and the window
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.modal_drawer_open = false;
            self.dismissible_drawer_open = false;
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
            .show(ctx, |ui| {
                // Permanent drawer - always visible within this demo window
                ui.horizontal(|ui| {
                    // Left side - drawer
                    ui.vertical(|ui| {
                        ui.set_width(self.drawer_width);
                        ui.heading("Mail");
                        if self.show_header {
                            ui.label("email@material.io");
                            ui.separator();
                        }
                        
                        ui.selectable_label(true, if self.show_icons { "📥 Inbox" } else { "Inbox" });
                        ui.selectable_label(false, if self.show_icons { "📤 Sent" } else { "Sent" });
                        ui.selectable_label(false, if self.show_icons { "📄 Drafts" } else { "Drafts" });
                        ui.selectable_label(false, if self.show_icons { "🗑 Trash" } else { "Trash" });
                    });
                    
                    ui.separator();
                    
                    // Right side - content
                    ui.vertical(|ui| {
                        ui.heading("Permanent Drawer Demo");
                        ui.label("This drawer is always visible and adjusts the content layout.");
                        ui.label("Try resizing the window to see how the content adapts.");
                        ui.add_space(20.0);
                        ui.label("Content area scales with available space.");
                        ui.label("The drawer remains fixed at 256px width.");
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
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Show drawer conditionally
                    let show_drawer_in_demo = true; // Always show in demo for visibility
                    
                    if show_drawer_in_demo {
                        // Left side - drawer
                        ui.vertical(|ui| {
                            ui.set_width(self.drawer_width);
                            ui.heading("Navigation");
                            if self.show_header {
                                ui.label("App Menu");
                                ui.separator();
                            }
                            
                            ui.selectable_label(true, if self.show_icons { "📊 Dashboard" } else { "Dashboard" });
                            ui.selectable_label(false, if self.show_icons { "📈 Analytics" } else { "Analytics" });
                            ui.selectable_label(false, if self.show_icons { "📋 Reports" } else { "Reports" });
                            ui.selectable_label(false, if self.show_icons { "👥 Users" } else { "Users" });
                        });
                        
                        ui.separator();
                    }
                    
                    // Right side - content
                    ui.vertical(|ui| {
                        ui.heading("Dismissible Drawer Demo");
                        ui.label("This drawer can be toggled open/closed and adjusts the content layout.");
                        ui.label("Press ESC or click the toggle button to dismiss it.");
                        
                        ui.add_space(10.0);
                        if ui.button("Toggle Drawer").clicked() {
                            // In a real implementation, this would toggle the drawer
                            ui.label("Drawer toggle clicked!");
                        }
                        
                        ui.add_space(20.0);
                        ui.label("When dismissed, content expands to fill the full width.");
                        ui.label("When shown, content is constrained by the drawer width.");
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
            .show(ctx, |ui| {
                // Main content
                ui.heading("Modal Drawer Demo");
                ui.label("This drawer overlays the content with a semi-transparent scrim.");
                ui.label("Click the scrim or press ESC to close it.");
                
                ui.add_space(10.0);
                let mut show_modal_demo = false;
                if ui.button("Show Modal Drawer Example").clicked() {
                    show_modal_demo = true;
                }
                
                ui.add_space(20.0);
                ui.label("Modal drawer characteristics:");
                ui.label("• Overlays content without changing layout");
                ui.label("• Semi-transparent scrim blocks interaction");
                ui.label("• Drawer slides in from the side");
                ui.label("• Click outside or ESC to close");
                
                // Show a simple modal demonstration
                if show_modal_demo {
                    ui.separator();
                    ui.horizontal(|ui| {
                        // Simulate drawer overlay
                        ui.vertical(|ui| {
                            ui.set_width(self.drawer_width);
                            ui.style_mut().visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 200);
                            
                            ui.heading("Menu");
                            if self.show_header {
                                ui.label("Choose an option");
                                ui.separator();
                            }
                            
                            ui.selectable_label(true, if self.show_icons { "🏠 Home" } else { "Home" });
                            ui.selectable_label(false, if self.show_icons { "👤 Profile" } else { "Profile" });
                            ui.selectable_label(false, if self.show_icons { "⚙️ Settings" } else { "Settings" });
                            ui.selectable_label(false, if self.show_icons { "❓ Help" } else { "Help" });
                        });
                        
                        ui.label("← This simulates the modal drawer overlay");
                    });
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
                
                if ui.selectable_label(matches!(self.selected_drawer, DrawerType::Modal), "Modal").clicked() {
                    self.selected_drawer = DrawerType::Modal;
                    self.permanent_drawer_open = false;
                    self.modal_drawer_open = true;
                    self.dismissible_drawer_open = false;
                }
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
                        if ui.add(MaterialButton::filled("Toggle Modal")).clicked() {
                            self.modal_drawer_open = !self.modal_drawer_open;
                        }
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