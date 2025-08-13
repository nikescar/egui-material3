use eframe::egui::{self, Ui, Window};
use crate::{MaterialDrawer, DrawerVariant, MaterialButton, MaterialCheckbox, standard_drawer, modal_drawer, dismissible_drawer};

pub struct DrawerWindow {
    pub open: bool,
    standard_drawer_open: bool,
    modal_drawer_open: bool,
    dismissible_drawer_open: bool,
    show_header: bool,
    show_icons: bool,
    drawer_width: f32,
}

impl Default for DrawerWindow {
    fn default() -> Self {
        Self {
            open: false,
            standard_drawer_open: true,
            modal_drawer_open: false,
            dismissible_drawer_open: false,
            show_header: true,
            show_icons: true,
            drawer_width: 256.0,
        }
    }
}

impl DrawerWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
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
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Drawer Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://material-web.dev/components/drawer/stories/");
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
            if ui.add(MaterialButton::filled("Toggle Standard")).clicked() {
                self.standard_drawer_open = !self.standard_drawer_open;
            }
            if ui.add(MaterialButton::filled("Toggle Modal")).clicked() {
                self.modal_drawer_open = !self.modal_drawer_open;
            }
            if ui.add(MaterialButton::filled("Toggle Dismissible")).clicked() {
                self.dismissible_drawer_open = !self.dismissible_drawer_open;
            }
        });
    }

    fn render_drawer_examples(&mut self, ui: &mut Ui) {
        ui.heading("Standard Drawer");
        ui.label("Always visible, allows interaction with both drawer and main content.");
        
        let mut standard_drawer_widget = standard_drawer(&mut self.standard_drawer_open)
            .width(self.drawer_width);

        if self.show_header {
            standard_drawer_widget = standard_drawer_widget.header("Mail", Some("email@material.io"));
        }

        standard_drawer_widget = standard_drawer_widget
            .item_with_callback(
                "Inbox", 
                if self.show_icons { Some("inbox") } else { None }, 
                true,
                || println!("Inbox clicked!")
            )
            .item_with_callback(
                "Sent", 
                if self.show_icons { Some("send") } else { None }, 
                false,
                || println!("Sent clicked!")
            )
            .item_with_callback(
                "Drafts", 
                if self.show_icons { Some("drafts") } else { None }, 
                false,
                || println!("Drafts clicked!")
            )
            .item_with_callback(
                "Trash", 
                if self.show_icons { Some("delete") } else { None }, 
                false,
                || println!("Trash clicked!")
            );

        ui.add(standard_drawer_widget);

        ui.add_space(30.0);

        ui.heading("Modal Drawer");
        ui.label("Overlays the main content with a scrim. Click scrim or press ESC to close.");
        
        if self.modal_drawer_open {
            let mut modal_drawer = modal_drawer(&mut self.modal_drawer_open)
                .width(self.drawer_width);

            if self.show_header {
                modal_drawer = modal_drawer.header("Navigation", Some("Choose a destination"));
            }

            modal_drawer = modal_drawer
                .item_with_callback(
                    "Home", 
                    if self.show_icons { Some("home") } else { None }, 
                    true,
                    || println!("Home clicked!")
                )
                .item_with_callback(
                    "Profile", 
                    if self.show_icons { Some("person") } else { None }, 
                    false,
                    || println!("Profile clicked!")
                )
                .item_with_callback(
                    "Settings", 
                    if self.show_icons { Some("settings") } else { None }, 
                    false,
                    || println!("Settings clicked!")
                )
                .item_with_callback(
                    "Help", 
                    if self.show_icons { Some("help") } else { None }, 
                    false,
                    || println!("Help clicked!")
                )
                .item_with_callback(
                    "Logout", 
                    if self.show_icons { Some("logout") } else { None }, 
                    false,
                    || println!("Logout clicked!")
                );

            ui.add(modal_drawer);
        } else {
            ui.label("Modal drawer is currently closed. Click 'Toggle Modal' to open it.");
        }

        ui.add_space(30.0);

        ui.heading("Dismissible Drawer");
        ui.label("Slides in from the side and can be dismissed. Does not affect main content layout.");
        
        if self.dismissible_drawer_open {
            let mut dismissible_drawer = dismissible_drawer(&mut self.dismissible_drawer_open)
                .width(self.drawer_width);

            if self.show_header {
                dismissible_drawer = dismissible_drawer.header("App Menu", Some("Navigation"));
            }

            dismissible_drawer = dismissible_drawer
                .item_with_callback(
                    "Dashboard", 
                    if self.show_icons { Some("dashboard") } else { None }, 
                    true,
                    || println!("Dashboard clicked!")
                )
                .item_with_callback(
                    "Analytics", 
                    if self.show_icons { Some("analytics") } else { None }, 
                    false,
                    || println!("Analytics clicked!")
                )
                .item_with_callback(
                    "Reports", 
                    if self.show_icons { Some("assessment") } else { None }, 
                    false,
                    || println!("Reports clicked!")
                )
                .item_with_callback(
                    "Users", 
                    if self.show_icons { Some("group") } else { None }, 
                    false,
                    || println!("Users clicked!")
                )
                .item_with_callback(
                    "Administration", 
                    if self.show_icons { Some("admin_panel_settings") } else { None }, 
                    false,
                    || println!("Administration clicked!")
                );

            ui.add(dismissible_drawer);
        } else {
            ui.label("Dismissible drawer is currently closed. Click 'Toggle Dismissible' to open it.");
        }

        ui.add_space(30.0);

        ui.heading("Drawer with Grouped Items");
        ui.label("Example of drawer with different sections and dividers.");
        
        let grouped_drawer = standard_drawer(&mut self.standard_drawer_open)
            .width(self.drawer_width)
            .header("Email App", Some("user@example.com"))
            .item_with_callback("Inbox", Some("inbox"), true, || println!("Inbox clicked!"))
            .item_with_callback("Starred", Some("star"), false, || println!("Starred clicked!"))
            .item_with_callback("Sent", Some("send"), false, || println!("Sent clicked!"))
            .item_with_callback("Drafts", Some("drafts"), false, || println!("Drafts clicked!"))
            // In a real implementation, you'd add dividers and subheaders here
            .item_with_callback("All Mail", Some("mail"), false, || println!("All Mail clicked!"))
            .item_with_callback("Trash", Some("delete"), false, || println!("Trash clicked!"))
            .item_with_callback("Spam", Some("report"), false, || println!("Spam clicked!"));

        ui.add(grouped_drawer);

        ui.add_space(20.0);

        ui.heading("Interactive Demo");
        ui.label("Try the different drawer variants and see how they behave:");
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Standard:");
                ui.label("• Always visible");
                ui.label("• Content alongside");
                ui.label("• Desktop/tablet use");
            });
            
            ui.vertical(|ui| {
                ui.label("Modal:");
                ui.label("• Overlays content");
                ui.label("• Blocks interaction");
                ui.label("• Mobile friendly");
            });
            
            ui.vertical(|ui| {
                ui.label("Dismissible:");
                ui.label("• Slides in/out");
                ui.label("• Can be dismissed");
                ui.label("• Flexible usage");
            });
        });
    }
}