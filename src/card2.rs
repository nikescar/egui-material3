use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Rect, Response, Sense, Ui, Vec2, Widget,
};
use crate::theme::get_global_color;

/// Material Design card component variants (enhanced version).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Card2Variant {
    Elevated,
    Filled,
    Outlined,
}

/// Enhanced Material Design card component.
///
/// This is an enhanced version of the card component with additional features
/// like media support, action areas, and improved layout options.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Enhanced card with media and actions
/// ui.add(MaterialCard2::elevated()
///     .header("Card Title", Some("Subtitle"))
///     .media_area(|ui| {
///         ui.label("Media content goes here");
///     })
///     .content(|ui| {
///         ui.label("Main card content");
///     })
///     .actions(|ui| {
///         if ui.button("Action 1").clicked() {
///             println!("Action 1 clicked!");
///         }
///     }));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialCard2<'a> {
    variant: Card2Variant,
    header_title: Option<String>,
    header_subtitle: Option<String>,
    media_content: Option<Box<dyn FnOnce(&mut Ui) -> Response + 'a>>,
    main_content: Option<Box<dyn FnOnce(&mut Ui) -> Response + 'a>>,
    actions_content: Option<Box<dyn FnOnce(&mut Ui) -> Response + 'a>>,
    min_size: Vec2,
    corner_radius: CornerRadius,
    clickable: bool,
    media_height: f32,
}

impl<'a> MaterialCard2<'a> {
    /// Create a new elevated material card.
    pub fn elevated() -> Self {
        Self::new_with_variant(Card2Variant::Elevated)
    }

    /// Create a new filled material card.
    pub fn filled() -> Self {
        Self::new_with_variant(Card2Variant::Filled)
    }

    /// Create a new outlined material card.
    pub fn outlined() -> Self {
        Self::new_with_variant(Card2Variant::Outlined)
    }

    fn new_with_variant(variant: Card2Variant) -> Self {
        Self {
            variant,
            header_title: None,
            header_subtitle: None,
            media_content: None,
            main_content: None,
            actions_content: None,
            min_size: Vec2::new(280.0, 200.0), // Larger default size for enhanced card
            corner_radius: CornerRadius::from(12.0),
            clickable: false,
            media_height: 160.0,
        }
    }

    /// Set card header with title and optional subtitle.
    pub fn header(mut self, title: impl Into<String>, subtitle: Option<impl Into<String>>) -> Self {
        self.header_title = Some(title.into());
        self.header_subtitle = subtitle.map(|s| s.into());
        self
    }

    /// Set media area content.
    pub fn media_area<F>(mut self, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.media_content = Some(Box::new(move |ui| {
            content(ui);
            ui.allocate_response(Vec2::ZERO, Sense::hover())
        }));
        self
    }

    /// Set media area height.
    pub fn media_height(mut self, height: f32) -> Self {
        self.media_height = height;
        self
    }

    /// Set main content for the card.
    pub fn content<F>(mut self, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.main_content = Some(Box::new(move |ui| {
            content(ui);
            ui.allocate_response(Vec2::ZERO, Sense::hover())
        }));
        self
    }

    /// Set actions area content.
    pub fn actions<F>(mut self, content: F) -> Self 
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.actions_content = Some(Box::new(move |ui| {
            content(ui);
            ui.allocate_response(Vec2::ZERO, Sense::hover())
        }));
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
        // Material Design theme colors
        let md_surface = get_global_color("surface");
        let md_surface_container_highest = get_global_color("surfaceContainerHighest");
        let md_outline_variant = get_global_color("outlineVariant");

        match self.variant {
            Card2Variant::Elevated => {
                // Elevated card: surface color with shadow
                (md_surface, None, true)
            },
            Card2Variant::Filled => {
                // Filled card: surface-container-highest color
                (md_surface_container_highest, None, false)
            },
            Card2Variant::Outlined => {
                // Outlined card: surface color with outline
                let stroke = Some(Stroke::new(1.0, md_outline_variant));
                (md_surface, stroke, false)
            },
        }
    }
}

impl<'a> Default for MaterialCard2<'a> {
    fn default() -> Self {
        Self::elevated()
    }
}

impl Widget for MaterialCard2<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (background_color, stroke, has_shadow) = self.get_card_style();
        
        let MaterialCard2 {
            variant: _,
            header_title,
            header_subtitle,
            media_content,
            main_content,
            actions_content,
            min_size,
            corner_radius,
            clickable,
            media_height,
        } = self;

        let sense = if clickable {
            Sense::click()
        } else {
            Sense::hover()
        };

        // Calculate total height based on content
        let header_height = if header_title.is_some() { 72.0 } else { 0.0 };
        let media_height_actual = if media_content.is_some() { media_height } else { 0.0 };
        let content_height = 80.0; // Default content area height
        let actions_height = if actions_content.is_some() { 52.0 } else { 0.0 };
        
        let total_height = header_height + media_height_actual + content_height + actions_height;
        let card_size = Vec2::new(min_size.x, total_height.max(min_size.y));
        
        let desired_size = ui.available_size().max(card_size);
        let mut response = ui.allocate_response(desired_size, sense);
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw shadow if present (for elevated cards)
            if has_shadow {
                let shadow_rect = Rect::from_min_size(
                    rect.min + Vec2::new(0.0, 2.0),
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

            let mut current_y = rect.min.y;

            // Draw header
            if let Some(title) = &header_title {
                let header_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), header_height)
                );
                
                // Title
                let title_pos = egui::pos2(rect.min.x + 16.0, current_y + 16.0);
                ui.painter().text(
                    title_pos,
                    egui::Align2::LEFT_TOP,
                    title,
                    egui::FontId::proportional(20.0),
                    get_global_color("onSurface")
                );
                
                // Subtitle if present
                if let Some(subtitle) = &header_subtitle {
                    let subtitle_pos = egui::pos2(rect.min.x + 16.0, current_y + 44.0);
                    ui.painter().text(
                        subtitle_pos,
                        egui::Align2::LEFT_TOP,
                        subtitle,
                        egui::FontId::proportional(14.0),
                        get_global_color("onSurfaceVariant")
                    );
                }
                
                current_y += header_height;
            }

            // Draw media area
            if let Some(media_fn) = media_content {
                let media_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), media_height)
                );
                
                // Clip media content to card bounds
                let media_response = ui.scope_builder(
                    egui::UiBuilder::new().max_rect(media_rect),
                    |ui| {
                        // Draw media background
                        ui.painter().rect_filled(
                            media_rect,
                            CornerRadius::ZERO,
                            get_global_color("surfaceVariant")
                        );
                        
                        media_fn(ui)
                    }
                );
                
                response = response.union(media_response.response);
                current_y += media_height;
            }

            // Draw main content
            if let Some(content_fn) = main_content {
                let content_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), content_height)
                );
                
                let content_response = ui.scope_builder(
                    egui::UiBuilder::new().max_rect(content_rect.shrink(16.0)),
                    |ui| {
                        content_fn(ui)
                    }
                );
                
                response = response.union(content_response.response);
                current_y += content_height;
            }

            // Draw actions area
            if let Some(actions_fn) = actions_content {
                let actions_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), actions_height)
                );
                
                let actions_response = ui.scope_builder(
                    egui::UiBuilder::new().max_rect(actions_rect.shrink2(Vec2::new(8.0, 8.0))),
                    |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            actions_fn(ui)
                        }).inner
                    }
                );
                
                response = response.union(actions_response.response);
            }
        }

        response
    }
}

/// Convenience function to create an elevated enhanced card.
pub fn elevated_card2() -> MaterialCard2<'static> {
    MaterialCard2::elevated()
}

/// Convenience function to create a filled enhanced card.
pub fn filled_card2() -> MaterialCard2<'static> {
    MaterialCard2::filled()
}

/// Convenience function to create an outlined enhanced card.
pub fn outlined_card2() -> MaterialCard2<'static> {
    MaterialCard2::outlined()
}