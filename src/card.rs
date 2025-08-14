use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget,
};
use crate::get_global_color;

/// Material Design card component variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardVariant {
    Elevated,
    Filled,
    Outlined,
}

/// Material Design card component.
///
/// Cards contain content and actions about a single subject.
/// They provide structure and layout for content display following Material Design 3 specifications.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Basic elevated card
/// ui.add(MaterialCard::elevated()
///     .min_size(egui::vec2(192.0, 128.0))
///     .content(|ui| {
///         ui.label("Card content");
///     }));
///
/// // Outlined card with actions
/// ui.add(MaterialCard::outlined()
///     .min_size(egui::vec2(192.0, 128.0))
///     .content(|ui| {
///         ui.vertical(|ui| {
///             ui.label("Card with actions");
///             if ui.button("Action").clicked() {
///                 println!("Card action clicked!");
///             }
///         });
///     }));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialCard<'a> {
    variant: CardVariant,
    content: Option<Box<dyn FnOnce(&mut Ui) -> Response + 'a>>,
    min_size: Vec2,
    corner_radius: CornerRadius,
    clickable: bool,
}

impl<'a> MaterialCard<'a> {
    /// Create a new elevated material card.
    pub fn elevated() -> Self {
        Self::new_with_variant(CardVariant::Elevated)
    }

    /// Create a new filled material card.
    pub fn filled() -> Self {
        Self::new_with_variant(CardVariant::Filled)
    }

    /// Create a new outlined material card.
    pub fn outlined() -> Self {
        Self::new_with_variant(CardVariant::Outlined)
    }

    fn new_with_variant(variant: CardVariant) -> Self {
        Self {
            variant,
            content: None,
            min_size: Vec2::new(192.0, 128.0), // Default size from Material Web stories
            corner_radius: CornerRadius::from(12), // Material Design card corner radius
            clickable: false,
        }
    }

    /// Set custom content for the card.
    pub fn content<F, R>(mut self, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) -> R + 'a,
        R: Into<Response>,
    {
        self.content = Some(Box::new(move |ui| content(ui).into()));
        self
    }

    /// Set the minimum size of the card.
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the corner radius of the card.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Make the card clickable.
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    fn get_card_style(&self) -> (Color32, Option<Stroke>, bool) {
        // Material Design colors from theme
        let md_surface = get_global_color("surface");
        let md_surface_container_low = get_global_color("surfaceContainerLow");
        let md_surface_container_highest = get_global_color("surfaceContainerHighest");
        let md_outline_variant = get_global_color("outlineVariant");

        match self.variant {
            CardVariant::Elevated => {
                // Elevated card: surface color with shadow
                (md_surface, None, true)
            },
            CardVariant::Filled => {
                // Filled card: surface-container-highest color
                (md_surface_container_highest, None, false)
            },
            CardVariant::Outlined => {
                // Outlined card: surface color with outline
                let stroke = Some(Stroke::new(1.0, md_outline_variant));
                (md_surface, stroke, false)
            },
        }
    }
}

impl<'a> Default for MaterialCard<'a> {
    fn default() -> Self {
        Self::elevated()
    }
}

impl Widget for MaterialCard<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (background_color, stroke, has_shadow) = self.get_card_style();
        
        let MaterialCard {
            variant: _,
            content,
            min_size,
            corner_radius,
            clickable,
        } = self;

        let sense = if clickable {
            Sense::click()
        } else {
            Sense::hover()
        };

        let desired_size = ui.available_size().max(min_size);
        let mut response = ui.allocate_response(desired_size, sense);
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw shadow if present (for elevated cards)
            if has_shadow {
                let shadow_rect = Rect::from_min_size(
                    rect.min + Vec2::new(0.0, 1.0),
                    rect.size(),
                );
                ui.painter().rect_filled(
                    shadow_rect,
                    corner_radius,
                    Color32::from_rgba_unmultiplied(0, 0, 0, 20),
                );
            }

            // Draw card background
            ui.painter().rect_filled(rect, corner_radius, background_color);

            // Draw outline if present (for outlined cards)
            if let Some(stroke) = stroke {
                ui.painter().rect_stroke(rect, corner_radius, stroke, egui::epaint::StrokeKind::Outside);
            }

            // Draw content
            if let Some(content_fn) = content {
                let content_rect = rect.shrink(16.0); // Add padding
                
                let content_response = ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    content_fn(ui)
                });

                response = response.union(content_response.response);
            }
        }

        response
    }
}

/// Convenience function to create an elevated card.
pub fn elevated_card() -> MaterialCard<'static> {
    MaterialCard::elevated()
}

/// Convenience function to create a filled card.
pub fn filled_card() -> MaterialCard<'static> {
    MaterialCard::filled()
}

/// Convenience function to create an outlined card.
pub fn outlined_card() -> MaterialCard<'static> {
    MaterialCard::outlined()
}