use eframe::egui::{Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use crate::get_global_color;

#[derive(Clone, Copy, PartialEq)]
pub enum IconButtonVariant {
    Standard,
    Filled,
    FilledTonal,
    Outlined,
}

pub struct MaterialIconButton<'a> {
    icon: String,
    variant: IconButtonVariant,
    selected: Option<&'a mut bool>,
    enabled: bool,
    size: f32,
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialIconButton<'a> {
    pub fn new(icon: impl Into<String>, variant: IconButtonVariant) -> Self {
        Self {
            icon: icon.into(),
            variant,
            selected: None,
            enabled: true,
            size: 40.0,
            action: None,
        }
    }

    pub fn standard(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::Standard)
    }

    pub fn filled(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::Filled)
    }

    pub fn filled_tonal(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::FilledTonal)
    }

    pub fn outlined(icon: impl Into<String>) -> Self {
        Self::new(icon, IconButtonVariant::Outlined)
    }

    pub fn toggle(icon: impl Into<String>, selected: &'a mut bool) -> Self {
        let mut button = Self::standard(icon);
        button.selected = Some(selected);
        button
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
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

impl<'a> Widget for MaterialIconButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        let is_selected = self.selected.as_ref().map_or(false, |s| **s);

        if response.clicked() && self.enabled {
            if let Some(selected) = self.selected {
                *selected = !*selected;
                response.mark_changed();
            }
            if let Some(action) = self.action {
                action();
            }
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
        let secondary_container = get_global_color("secondaryContainer");
        let on_secondary_container = get_global_color("onSecondaryContainer");
        let _surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let (bg_color, icon_color, border_color) = if !self.enabled {
            (
                get_global_color("surfaceContainer"),
                get_global_color("outline"),
                Color32::TRANSPARENT,
            )
        } else {
            match self.variant {
                IconButtonVariant::Standard => {
                    if is_selected {
                        (Color32::TRANSPARENT, primary_color, Color32::TRANSPARENT)
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20),
                            on_surface,
                            Color32::TRANSPARENT,
                        )
                    } else {
                        (Color32::TRANSPARENT, on_surface_variant, Color32::TRANSPARENT)
                    }
                }
                IconButtonVariant::Filled => {
                    if is_selected {
                        (primary_color, get_global_color("onPrimary"), Color32::TRANSPARENT)
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                primary_color.r().saturating_add(20),
                                primary_color.g().saturating_add(20),
                                primary_color.b().saturating_add(20),
                                255,
                            ),
                            get_global_color("onPrimary"),
                            Color32::TRANSPARENT,
                        )
                    } else {
                        (primary_color, Color32::WHITE, Color32::TRANSPARENT)
                    }
                }
                IconButtonVariant::FilledTonal => {
                    if is_selected {
                        (secondary_container, on_secondary_container, Color32::TRANSPARENT)
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                secondary_container.r().saturating_sub(10),
                                secondary_container.g().saturating_sub(10),
                                secondary_container.b().saturating_sub(10),
                                255,
                            ),
                            on_secondary_container,
                            Color32::TRANSPARENT,
                        )
                    } else {
                        (secondary_container, on_secondary_container, Color32::TRANSPARENT)
                    }
                }
                IconButtonVariant::Outlined => {
                    if is_selected {
                        (
                            Color32::from_rgba_premultiplied(primary_color.r(), primary_color.g(), primary_color.b(), 24),
                            primary_color,
                            primary_color,
                        )
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(on_surface.r(), on_surface.g(), on_surface.b(), 20),
                            on_surface_variant,
                            outline,
                        )
                    } else {
                        (Color32::TRANSPARENT, on_surface_variant, outline)
                    }
                }
            }
        };

        // Draw background
        if bg_color != Color32::TRANSPARENT {
            ui.painter().rect_filled(
                rect,
                rect.height() / 2.0,
                bg_color,
            );
        }

        // Draw border for outlined variant
        if border_color != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                rect.height() / 2.0,
                Stroke::new(1.0, border_color),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Draw icon using our icon system
        let icon_size = self.size * 0.6;
        let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
        
        let icon_widget = crate::icon::MaterialIcon::new(&self.icon)
            .size(icon_size)
            .color(icon_color);
        
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(icon_rect), |ui| {
            ui.add(icon_widget);
        });

        // Add ripple effect on hover
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

pub fn icon_button_standard(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::standard(icon)
}

pub fn icon_button_filled(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::filled(icon)
}

pub fn icon_button_filled_tonal(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::filled_tonal(icon)
}

pub fn icon_button_outlined(icon: impl Into<String>) -> MaterialIconButton<'static> {
    MaterialIconButton::outlined(icon)
}

pub fn icon_button_toggle(icon: impl Into<String>, selected: &mut bool) -> MaterialIconButton {
    MaterialIconButton::toggle(icon, selected)
}