use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, 
    epaint::{Shadow, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget, WidgetText, NumExt,
};

/// Material Design elevation component.
///
/// Elevation is the relative distance between two surfaces along the z-axis.
/// This component provides visual depth using shadows following Material Design 3 specifications.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Basic elevation level 1
/// ui.add(MaterialElevation::new().level(1).content(|ui| {
///     ui.label("Elevated content");
/// }));
///
/// // Higher elevation level 3
/// ui.add(MaterialElevation::new().level(3).content(|ui| {
///     ui.label("Higher elevation");
/// }));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialElevation<'a> {
    level: u8,
    content: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    text: Option<WidgetText>,
    min_size: Vec2,
    corner_radius: CornerRadius,
    background_color: Option<Color32>,
}

impl<'a> MaterialElevation<'a> {
    /// Create a new material elevation component.
    pub fn new() -> Self {
        Self {
            level: 0,
            content: None,
            text: None,
            min_size: Vec2::new(64.0, 64.0), // Default size from Material Web stories
            corner_radius: CornerRadius::from(16), // Default border radius
            background_color: None,
        }
    }

    /// Set the elevation level (0-5).
    /// Higher levels create more pronounced shadows.
    pub fn level(mut self, level: u8) -> Self {
        self.level = level.min(5); // Clamp to max level 5
        self
    }

    /// Set custom content for the elevated surface.
    pub fn content<F>(mut self, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.content = Some(Box::new(content));
        self
    }

    /// Set text content for the elevated surface.
    pub fn text(mut self, text: impl Into<WidgetText>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the minimum size of the elevated surface.
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the corner radius of the elevated surface.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Set the background color of the elevated surface.
    pub fn background_color(mut self, color: impl Into<Color32>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    fn get_shadow(&self) -> Shadow {
        // Material Design elevation shadows based on level
        // These values approximate Material Design 3 elevation specifications
        // As elevation increases, shadows get darker and spread more
        match self.level {
            0 => Shadow::NONE,
            1 => Shadow {
                offset: [0, 1],
                blur: 3,
                spread: 1,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 31), // ~12% opacity
            },
            2 => Shadow {
                offset: [0, 2],
                blur: 6,
                spread: 2,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 38), // ~15% opacity
            },
            3 => Shadow {
                offset: [0, 4],
                blur: 8,
                spread: 3,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 46), // ~18% opacity
            },
            4 => Shadow {
                offset: [0, 6],
                blur: 10,
                spread: 4,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 51), // ~20% opacity
            },
            5 => Shadow {
                offset: [0, 8],
                blur: 12,
                spread: 6,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 59), // ~23% opacity
            },
            _ => Shadow::NONE,
        }
    }
    
    fn get_shadow_layers(&self) -> Vec<Shadow> {
        // Multiple shadow layers for more realistic elevation effect
        // As elevation increases, we add more shadows with different properties
        match self.level {
            0 => vec![],
            1 => vec![
                Shadow {
                    offset: [0, 1],
                    blur: 2,
                    spread: 0,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                },
            ],
            2 => vec![
                Shadow {
                    offset: [0, 1],
                    blur: 2,
                    spread: 0,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                },
                Shadow {
                    offset: [0, 2],
                    blur: 4,
                    spread: 0,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 14),
                },
            ],
            3 => vec![
                Shadow {
                    offset: [0, 1],
                    blur: 3,
                    spread: 0,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                },
                Shadow {
                    offset: [0, 4],
                    blur: 8,
                    spread: 3,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 14),
                },
            ],
            4 => vec![
                Shadow {
                    offset: [0, 1],
                    blur: 3,
                    spread: 0,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                },
                Shadow {
                    offset: [0, 6],
                    blur: 10,
                    spread: 4,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 14),
                },
            ],
            5 => vec![
                Shadow {
                    offset: [0, 1],
                    blur: 3,
                    spread: 0,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                },
                Shadow {
                    offset: [0, 8],
                    blur: 12,
                    spread: 6,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 14),
                },
            ],
            _ => vec![],
        }
    }
}

impl<'a> Default for MaterialElevation<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialElevation<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let shadow_layers = self.get_shadow_layers();
        
        let MaterialElevation {
            level,
            content,
            text,
            min_size,
            corner_radius,
            background_color,
        } = self;

        // Material Design colors
        let default_bg = get_global_color("primaryContainer");
        let background = background_color.unwrap_or(default_bg);

        // Calculate the desired size
        let desired_size = if content.is_some() || text.is_some() {
            ui.available_size().at_least(min_size)
        } else {
            min_size
        };

        let response = ui.allocate_response(desired_size, Sense::hover());
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw multiple shadow layers for better elevation effect
            for shadow in shadow_layers.iter().rev() { // Draw from furthest to nearest
                let shadow_rect = Rect::from_min_size(
                    rect.min + Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32),
                    rect.size() + Vec2::splat(shadow.spread as f32 * 2.0),
                );
                
                // Draw shadow with increased opacity for higher levels
                ui.painter().rect(
                    shadow_rect,
                    corner_radius,
                    shadow.color,
                    egui::Stroke::NONE,
                    egui::epaint::StrokeKind::Outside,
                );
            }

            // Draw the elevated surface on top of shadows
            ui.painter().rect(
                rect,
                corner_radius,
                background,
                egui::Stroke::new(1.0, get_global_color("outline").linear_multiply(0.12)),
                egui::epaint::StrokeKind::Outside,
            );

            // Render content or text
            let content_rect = rect.shrink(8.0); // Add some padding
            
            if let Some(content_fn) = content {
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    ui.centered_and_justified(|ui| {
                        content_fn(ui);
                    });
                });
            } else if let Some(text) = text {
                let text_color = get_global_color("onPrimaryContainer");
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.colored_label(text_color, text.text());
                    });
                });
            } else {
                // Default content showing the elevation level
                let text_color = get_global_color("onPrimaryContainer");
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.colored_label(text_color, level.to_string());
                    });
                });
            }
        }

        response
    }
}