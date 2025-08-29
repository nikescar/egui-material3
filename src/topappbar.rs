use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius, Shadow},
    Rect, Response, Sense, Ui, Vec2, Widget,
};

/// Material Design top app bar variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopAppBarVariant {
    Regular,
    Medium,
    Large,
    CenterAligned,
}

/// Material Design top app bar component.
///
/// Top app bars display information and actions related to the current screen.
/// They provide structure and contain elements like titles, navigation, and actions.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let top_bar = MaterialTopAppBar::regular("My App")
///     .navigation_icon("menu", || println!("Menu clicked!"))
///     .action_icon("search", || println!("Search clicked!"))
///     .action_icon("more_vert", || println!("More clicked!"));
///
/// ui.add(top_bar);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialTopAppBar<'a> {
    variant: TopAppBarVariant,
    title: String,
    navigation_icon: Option<(String, Box<dyn Fn() + Send + Sync + 'a>)>,
    action_icons: Vec<(String, Box<dyn Fn() + Send + Sync + 'a>)>,
    height: f32,
    corner_radius: CornerRadius,
    elevation: Option<Shadow>,
    scrolled: bool,
    id_salt: Option<String>,
}

impl<'a> MaterialTopAppBar<'a> {
    /// Create a new regular top app bar.
    pub fn regular(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::Regular, title)
    }

    /// Create a new medium top app bar.
    pub fn medium(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::Medium, title)
    }

    /// Create a new large top app bar.
    pub fn large(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::Large, title)
    }

    /// Create a new center-aligned top app bar.
    pub fn center_aligned(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::CenterAligned, title)
    }

    fn new(variant: TopAppBarVariant, title: impl Into<String>) -> Self {
        let height = match variant {
            TopAppBarVariant::Regular | TopAppBarVariant::CenterAligned => 64.0,
            TopAppBarVariant::Medium => 112.0,
            TopAppBarVariant::Large => 152.0,
        };

        Self {
            variant,
            title: title.into(),
            navigation_icon: None,
            action_icons: Vec::new(),
            height,
            corner_radius: CornerRadius::ZERO,
            elevation: None,
            scrolled: false,
            id_salt: None,
        }
    }

    /// Add a navigation icon (typically hamburger menu or back arrow).
    pub fn navigation_icon<F>(mut self, icon: impl Into<String>, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.navigation_icon = Some((icon.into(), Box::new(callback)));
        self
    }

    /// Add an action icon to the app bar.
    pub fn action_icon<F>(mut self, icon: impl Into<String>, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.action_icons.push((icon.into(), Box::new(callback)));
        self
    }

    /// Set custom height.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set corner radius.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set elevation shadow.
    pub fn elevation(mut self, elevation: impl Into<Shadow>) -> Self {
        self.elevation = Some(elevation.into());
        self
    }

    /// Set scrolled state (affects elevation).
    pub fn scrolled(mut self, scrolled: bool) -> Self {
        self.scrolled = scrolled;
        self
    }

    /// Set unique ID salt to prevent ID clashes.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    fn get_app_bar_style(&self) -> (Color32, Option<Stroke>) {
        let md_surface = get_global_color("surface");
        let md_primary = get_global_color("primary");
        
        match self.variant {
            TopAppBarVariant::Regular | TopAppBarVariant::CenterAligned => {
                (md_surface, None)
            },
            TopAppBarVariant::Medium | TopAppBarVariant::Large => {
                (md_primary, None)
            },
        }
    }

    fn get_text_color(&self) -> Color32 {
        match self.variant {
            TopAppBarVariant::Regular | TopAppBarVariant::CenterAligned => {
                get_global_color("onSurface")
            },
            TopAppBarVariant::Medium | TopAppBarVariant::Large => {
                get_global_color("onPrimary")
            },
        }
    }

    fn get_icon_color(&self) -> Color32 {
        match self.variant {
            TopAppBarVariant::Regular | TopAppBarVariant::CenterAligned => {
                get_global_color("onSurfaceVariant")
            },
            TopAppBarVariant::Medium | TopAppBarVariant::Large => {
                get_global_color("onPrimary")
            },
        }
    }
}

impl Widget for MaterialTopAppBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (background_color, border_stroke) = self.get_app_bar_style();
        let text_color = self.get_text_color();
        let icon_color = self.get_icon_color();
        
        let MaterialTopAppBar {
            variant,
            title,
            navigation_icon,
            action_icons,
            height,
            corner_radius,
            elevation,
            scrolled,
            id_salt,
        } = self;

        let desired_size = Vec2::new(ui.available_width(), height);
        let mut response = ui.allocate_response(desired_size, Sense::hover());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw elevation shadow if present and scrolled
            if scrolled || variant == TopAppBarVariant::Regular {
                if let Some(_shadow) = elevation {
                    let shadow_rect = rect.translate(Vec2::new(0.0, 1.0));
                    ui.painter().rect_filled(
                        shadow_rect,
                        corner_radius,
                        Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                    );
                }
            }

            // Draw app bar background
            ui.painter().rect_filled(rect, corner_radius, background_color);
            
            // Draw border if present
            if let Some(stroke) = border_stroke {
                ui.painter().rect_stroke(rect, corner_radius, stroke, egui::epaint::StrokeKind::Outside);
            }

            let icon_size = 24.0;
            let icon_padding = 12.0;
            let icon_total_size = icon_size + icon_padding * 2.0;
            
            let mut left_x = rect.min.x + 4.0;
            let icon_y = rect.min.y + (64.0 - icon_total_size) / 2.0; // Always center in top 64px

            // Draw navigation icon
            if let Some((nav_icon, nav_callback)) = navigation_icon {
                let nav_rect = Rect::from_min_size(
                    egui::pos2(left_x, icon_y),
                    Vec2::splat(icon_total_size)
                );
                
                let nav_id = if let Some(ref salt) = id_salt {
                    egui::Id::new((salt, "nav_icon"))
                } else {
                    egui::Id::new(("top_app_bar_nav", &title))
                };
                let nav_response = ui.interact(nav_rect, nav_id, Sense::click());
                
                // Icon background on hover
                if nav_response.hovered() {
                    let hover_color = Color32::from_rgba_unmultiplied(icon_color.r(), icon_color.g(), icon_color.b(), 20);
                    ui.painter().rect_filled(nav_rect, CornerRadius::from(20.0), hover_color);
                }
                
                // Draw different navigation icons based on icon name
                let icon_center = nav_rect.center();
                match nav_icon.as_str() {
                    "arrow_back" | "arrow_back_ios" => {
                        // Back arrow
                        ui.painter().line_segment([icon_center + Vec2::new(4.0, -6.0), icon_center + Vec2::new(-2.0, 0.0)], Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(-2.0, 0.0), icon_center + Vec2::new(4.0, 6.0)], Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(-2.0, 0.0), icon_center + Vec2::new(6.0, 0.0)], Stroke::new(2.0, icon_color));
                    },
                    "close" => {
                        // X close icon
                        ui.painter().line_segment([icon_center + Vec2::new(-6.0, -6.0), icon_center + Vec2::new(6.0, 6.0)], Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(-6.0, 6.0), icon_center + Vec2::new(6.0, -6.0)], Stroke::new(2.0, icon_color));
                    },
                    "menu" | _ => {
                        // Hamburger menu (default)
                        let line_width = 16.0;
                        let line_height = 2.0;
                        let line_spacing = 4.0;
                        
                        for i in 0..3 {
                            let y_offset = (i as f32 - 1.0) * line_spacing;
                            let line_rect = Rect::from_center_size(
                                icon_center + Vec2::new(0.0, y_offset),
                                Vec2::new(line_width, line_height)
                            );
                            ui.painter().rect_filled(line_rect, CornerRadius::from(1.0), icon_color);
                        }
                    },
                }
                
                if nav_response.clicked() {
                    nav_callback();
                }
                
                left_x += icon_total_size;
                response = response.union(nav_response);
            }

            // Calculate title position
            let title_font_size = match variant {
                TopAppBarVariant::Regular => 20.0,
                TopAppBarVariant::CenterAligned => 20.0,
                TopAppBarVariant::Medium => 24.0,
                TopAppBarVariant::Large => 28.0,
            };
            
            let title_y = match variant {
                TopAppBarVariant::Regular | TopAppBarVariant::CenterAligned => {
                    rect.min.y + (64.0 - title_font_size) / 2.0
                },
                TopAppBarVariant::Medium => {
                    rect.min.y + height - 32.0
                },
                TopAppBarVariant::Large => {
                    rect.min.y + height - 40.0
                },
            };

            let title_x = match variant {
                TopAppBarVariant::CenterAligned => {
                    // Center the title
                    let title_galley = ui.painter().layout_no_wrap(
                        title.clone(),
                        egui::FontId::proportional(title_font_size),
                        text_color
                    );
                    rect.center().x - title_galley.size().x / 2.0
                },
                _ => {
                    left_x + 8.0
                },
            };

            // Draw title
            ui.painter().text(
                egui::pos2(title_x, title_y),
                egui::Align2::LEFT_TOP,
                &title,
                egui::FontId::proportional(title_font_size),
                text_color
            );

            // Draw action icons
            let mut right_x = rect.max.x - 4.0;
            
            for (action_index, (action_icon, action_callback)) in action_icons.iter().enumerate().rev() {
                right_x -= icon_total_size;
                
                let action_rect = Rect::from_min_size(
                    egui::pos2(right_x, icon_y),
                    Vec2::splat(icon_total_size)
                );
                
                let action_id = if let Some(ref salt) = id_salt {
                    egui::Id::new((salt, "action_icon", action_index))
                } else {
                    egui::Id::new(("top_app_bar_action", &title, action_index))
                };
                let action_response = ui.interact(action_rect, action_id, Sense::click());
                
                // Icon background on hover
                if action_response.hovered() {
                    let hover_color = Color32::from_rgba_unmultiplied(icon_color.r(), icon_color.g(), icon_color.b(), 20);
                    ui.painter().rect_filled(action_rect, CornerRadius::from(20.0), hover_color);
                }
                
                // Draw different action icons based on icon name
                let icon_center = action_rect.center();
                match action_icon.as_str() {
                    "search" => {
                        // Search icon (magnifying glass)
                        ui.painter().circle_stroke(icon_center + Vec2::new(-2.0, -2.0), 6.0, Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(3.0, 3.0), icon_center + Vec2::new(6.0, 6.0)], Stroke::new(2.0, icon_color));
                    },
                    "favorite" | "favorite_border" => {
                        // Heart icon
                        let heart_points = [
                            icon_center + Vec2::new(0.0, 2.0),
                            icon_center + Vec2::new(-4.0, -2.0),
                            icon_center + Vec2::new(-2.0, -4.0),
                            icon_center + Vec2::new(0.0, -2.0),
                            icon_center + Vec2::new(2.0, -4.0),
                            icon_center + Vec2::new(4.0, -2.0),
                        ];
                        for i in 0..heart_points.len() {
                            let next_i = (i + 1) % heart_points.len();
                            ui.painter().line_segment([heart_points[i], heart_points[next_i]], Stroke::new(1.5, icon_color));
                        }
                    },
                    "share" => {
                        // Share icon (arrow with dots)
                        ui.painter().line_segment([icon_center + Vec2::new(-4.0, 2.0), icon_center + Vec2::new(4.0, -2.0)], Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(2.0, -4.0), icon_center + Vec2::new(4.0, -2.0)], Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(2.0, 0.0), icon_center + Vec2::new(4.0, -2.0)], Stroke::new(2.0, icon_color));
                        ui.painter().circle_filled(icon_center + Vec2::new(-6.0, 4.0), 2.0, icon_color);
                        ui.painter().circle_filled(icon_center + Vec2::new(6.0, -4.0), 2.0, icon_color);
                    },
                    "notifications" | "notifications_none" => {
                        // Notification bell
                        ui.painter().circle_stroke(icon_center, 6.0, Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(-2.0, -8.0), icon_center + Vec2::new(2.0, -8.0)], Stroke::new(2.0, icon_color));
                        ui.painter().line_segment([icon_center + Vec2::new(-2.0, 6.0), icon_center + Vec2::new(2.0, 6.0)], Stroke::new(3.0, icon_color));
                    },
                    "account_circle" | "person" => {
                        // Person icon
                        ui.painter().circle_stroke(icon_center, 8.0, Stroke::new(2.0, icon_color));
                        ui.painter().circle_filled(icon_center + Vec2::new(0.0, -3.0), 3.0, icon_color);
                        ui.painter().circle_stroke(icon_center + Vec2::new(0.0, 2.0), 5.0, Stroke::new(2.0, icon_color));
                    },
                    "settings" => {
                        // Gear icon
                        ui.painter().circle_stroke(icon_center, 4.0, Stroke::new(2.0, icon_color));
                        for i in 0..8 {
                            let angle = i as f32 * std::f32::consts::PI / 4.0;
                            let start = icon_center + Vec2::new(angle.cos() * 6.0, angle.sin() * 6.0);
                            let end = icon_center + Vec2::new(angle.cos() * 8.0, angle.sin() * 8.0);
                            ui.painter().line_segment([start, end], Stroke::new(2.0, icon_color));
                        }
                    },
                    "more_vert" | _ => {
                        // Three vertical dots (default)
                        for i in 0..3 {
                            let y_offset = (i as f32 - 1.0) * 3.0;
                            ui.painter().circle_filled(
                                icon_center + Vec2::new(0.0, y_offset),
                                1.5,
                                icon_color
                            );
                        }
                    },
                }
                
                if action_response.clicked() {
                    action_callback();
                }
                
                response = response.union(action_response);
            }
        }

        response
    }
}

/// Convenience function to create a regular top app bar.
pub fn top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::regular(title)
}

/// Convenience function to create a center-aligned top app bar.
pub fn center_aligned_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::center_aligned(title)
}

/// Convenience function to create a medium top app bar.
pub fn medium_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::medium(title)
}

/// Convenience function to create a large top app bar.
pub fn large_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::large(title)
}