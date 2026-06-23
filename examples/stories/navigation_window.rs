#![doc(hidden)]

use crate::{navigation, NavigationHeader, NavigationItem};
use eframe::egui::{self, Window};

#[doc(hidden)]
pub struct NavigationWindow {
    pub open: bool,
    selected: String,
}

impl Default for NavigationWindow {
    fn default() -> Self {
        Self {
            open: false,
            selected: "#home".to_string(),
        }
    }
}

impl NavigationWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Navigation Stories")
            .open(&mut open)
            .default_size([420.0, 720.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Nala Navigation");
                    ui.label("Leo-style sidebar with animated active indicator.");
                    ui.add_space(8.0);
                    ui.label(format!("Selected: {}", self.selected));
                    ui.add_space(16.0);

                    ui.add(
                        navigation(&mut self.selected)
                            .width(300.0)
                            .header(
                                NavigationHeader {
                                    highlight: Some("Brave".to_string()),
                                    title: "Accounts".to_string(),
                                    icon: Some("shield".to_string()),
                                    ..Default::default()
                                },
                            )
                            .item(
                                NavigationItem::new("#home", "Home").icon("home"),
                            )
                            .item(
                                NavigationItem::new("#explore", "Explore").icon("explore"),
                            )
                            .item(
                                NavigationItem::new("#notifications", "Notifications")
                                    .icon("notifications"),
                            )
                            .divider()
                            .item(
                                NavigationItem::new("#settings", "Settings")
                                    .icon("settings")
                                    .child(
                                        NavigationItem::new("#settings-languages", "Languages")
                                            .icon("language"),
                                    )
                                    .child(
                                        NavigationItem::new("#settings-security", "Security")
                                            .icon("lock")
                                            .child(
                                                NavigationItem::new(
                                                    "#settings-security-account",
                                                    "Account",
                                                )
                                                .icon("person"),
                                            )
                                            .child(
                                                NavigationItem::new(
                                                    "#settings-security-tokens",
                                                    "Tokens",
                                                )
                                                .icon("token"),
                                            ),
                                    ),
                            )
                            .action(
                                NavigationItem::new("#help", "Help").icon("help_outline"),
                            )
                            .action(
                                NavigationItem::new("#logout", "Log out").icon("logout"),
                            ),
                    );
                });
            });
        self.open = open;
    }
}
