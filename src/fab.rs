use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

#[derive(Clone, Copy, PartialEq)]
pub enum FabVariant {
    Surface,
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FabSize {
    Small,
    Regular,
    Large,
    Extended,
}

pub struct MaterialFab<'a> {
    variant: FabVariant,
    size: FabSize,
    icon: Option<String>,
    text: Option<String>,
    enabled: bool,
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialFab<'a> {
    pub fn new(variant: FabVariant) -> Self {
        Self {
            variant,
            size: FabSize::Regular,
            icon: None,
            text: None,
            enabled: true,
            action: None,
        }
    }

    pub fn surface() -> Self {
        Self::new(FabVariant::Surface)
    }

    pub fn primary() -> Self {
        Self::new(FabVariant::Primary)
    }

    pub fn secondary() -> Self {
        Self::new(FabVariant::Secondary)
    }

    pub fn tertiary() -> Self {
        Self::new(FabVariant::Tertiary)
    }

    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self.size = FabSize::Extended;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn lowered(self, _lowered: bool) -> Self {
        // Placeholder for lowered state (elevation effect)
        // In a real implementation, this would affect the visual elevation
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

impl<'a> Widget for MaterialFab<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = match self.size {
            FabSize::Small => Vec2::splat(40.0),
            FabSize::Regular => Vec2::splat(56.0),
            FabSize::Large => Vec2::splat(96.0),
            FabSize::Extended => {
                let text_width = if let Some(ref text) = self.text {
                    ui.fonts(|fonts| {
                        fonts.glyph_width(&egui::FontId::default(), ' ') * text.len() as f32
                    })
                } else {
                    0.0
                };
                Vec2::new(text_width + 56.0, 56.0)
            }
        };

        let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());

        if response.clicked() && self.enabled {
            if let Some(action) = self.action {
                action();
            }
        }

        // Material Design colors
        let primary_color = Color32::from_rgb(103, 80, 164);
        let secondary_color = Color32::from_rgb(98, 91, 113);
        let tertiary_color = Color32::from_rgb(125, 82, 96);
        let surface = Color32::from_gray(if ui.visuals().dark_mode { 16 } else { 254 });
        let on_primary = Color32::WHITE;
        let on_surface = Color32::from_gray(if ui.visuals().dark_mode { 230 } else { 30 });

        let (bg_color, icon_color) = if !self.enabled {
            (
                Color32::from_gray(if ui.visuals().dark_mode { 31 } else { 245 }),
                Color32::from_gray(if ui.visuals().dark_mode { 68 } else { 189 }),
            )
        } else {
            match self.variant {
                FabVariant::Surface => {
                    if response.hovered() {
                        (
                            Color32::from_gray(if ui.visuals().dark_mode { 45 } else { 240 }),
                            on_surface,
                        )
                    } else {
                        (surface, on_surface)
                    }
                }
                FabVariant::Primary => {
                    if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                primary_color.r().saturating_add(20),
                                primary_color.g().saturating_add(20),
                                primary_color.b().saturating_add(20),
                                255,
                            ),
                            on_primary,
                        )
                    } else {
                        (primary_color, on_primary)
                    }
                }
                FabVariant::Secondary => {
                    if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                secondary_color.r().saturating_add(20),
                                secondary_color.g().saturating_add(20),
                                secondary_color.b().saturating_add(20),
                                255,
                            ),
                            on_primary,
                        )
                    } else {
                        (secondary_color, on_primary)
                    }
                }
                FabVariant::Tertiary => {
                    if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                tertiary_color.r().saturating_add(20),
                                tertiary_color.g().saturating_add(20),
                                tertiary_color.b().saturating_add(20),
                                255,
                            ),
                            on_primary,
                        )
                    } else {
                        (tertiary_color, on_primary)
                    }
                }
            }
        };

        // Draw shadow
        let shadow_rect = rect.expand(4.0);
        ui.painter().rect_filled(
            shadow_rect,
            rect.height() / 2.0,
            Color32::from_black_alpha(40),
        );

        // Draw FAB background
        ui.painter().rect_filled(
            rect,
            rect.height() / 2.0,
            bg_color,
        );

        // Draw content
        match self.size {
            FabSize::Extended => {
                // Draw icon and text
                let mut content_x = rect.min.x + 16.0;
                
                if let Some(_icon) = self.icon {
                    let icon_rect = Rect::from_min_size(
                        Pos2::new(content_x, rect.center().y - 12.0),
                        Vec2::splat(24.0),
                    );
                    ui.painter().circle_filled(icon_rect.center(), 8.0, icon_color);
                    content_x += 32.0;
                }

                if let Some(ref text) = self.text {
                    let text_pos = Pos2::new(content_x, rect.center().y);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        text,
                        egui::FontId::default(),
                        icon_color,
                    );
                }
            }
            _ => {
                // Draw centered icon
                if let Some(_icon) = self.icon {
                    let icon_size = match self.size {
                        FabSize::Small => 18.0,
                        FabSize::Large => 36.0,
                        _ => 24.0,
                    };
                    ui.painter().circle_filled(rect.center(), icon_size / 2.0, icon_color);
                } else {
                    // Default plus icon
                    let icon_size = match self.size {
                        FabSize::Small => 18.0,
                        FabSize::Large => 36.0,
                        _ => 24.0,
                    };
                    let center = rect.center();
                    let half_size = icon_size / 4.0;
                    
                    ui.painter().line_segment(
                        [
                            Pos2::new(center.x - half_size, center.y),
                            Pos2::new(center.x + half_size, center.y),
                        ],
                        Stroke::new(2.0, icon_color),
                    );
                    ui.painter().line_segment(
                        [
                            Pos2::new(center.x, center.y - half_size),
                            Pos2::new(center.x, center.y + half_size),
                        ],
                        Stroke::new(2.0, icon_color),
                    );
                }
            }
        }

        // Add ripple effect on click
        if response.hovered() && self.enabled {
            let ripple_color = Color32::from_rgba_premultiplied(icon_color.r(), icon_color.g(), icon_color.b(), 30);
            ui.painter().rect_filled(
                rect,
                rect.height() / 2.0,
                ripple_color,
            );
        }

        response
    }
}

pub fn fab_surface() -> MaterialFab<'static> {
    MaterialFab::surface()
}

pub fn fab_primary() -> MaterialFab<'static> {
    MaterialFab::primary()
}

pub fn fab_secondary() -> MaterialFab<'static> {
    MaterialFab::secondary()
}

pub fn fab_tertiary() -> MaterialFab<'static> {
    MaterialFab::tertiary()
}