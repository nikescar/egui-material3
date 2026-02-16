use crate::theme::get_global_color;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke},
    Rect, Response, Sense, Ui, Vec2, Widget,
};

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
    elevation: Option<f32>,
    surface_tint_color: Option<Color32>,
    shadow_color: Option<Color32>,
    margin: f32,
    clip_behavior: bool,
    border_on_foreground: bool,
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
            elevation: None,
            surface_tint_color: None,
            shadow_color: None,
            margin: 4.0,
            clip_behavior: false,
            border_on_foreground: true,
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

    /// Set the elevation of the card.
    /// For Material 3: Elevated = 1.0, Filled = 0.0, Outlined = 0.0
    pub fn elevation(mut self, elevation: f32) -> Self {
        self.elevation = Some(elevation.max(0.0));
        self
    }

    /// Set the surface tint color for elevation overlay.
    /// In Material 3, this color is overlaid on the surface to indicate elevation.
    pub fn surface_tint_color(mut self, color: Color32) -> Self {
        self.surface_tint_color = Some(color);
        self
    }

    /// Set the shadow color.
    pub fn shadow_color(mut self, color: Color32) -> Self {
        self.shadow_color = Some(color);
        self
    }

    /// Set the margin around the card.
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    /// Set whether to clip the card content.
    pub fn clip_behavior(mut self, clip: bool) -> Self {
        self.clip_behavior = clip;
        self
    }

    /// Set whether the border should be painted on foreground.
    pub fn border_on_foreground(mut self, on_foreground: bool) -> Self {
        self.border_on_foreground = on_foreground;
        self
    }

    fn get_card_style(&self) -> (Color32, Option<Stroke>, f32) {
        // Material Design 3 theme colors and elevation defaults
        let md_surface = get_global_color("surface");
        let md_surface_container_low = get_global_color("surfaceContainerLow");
        let md_surface_container_highest = get_global_color("surfaceContainerHighest");
        let md_outline_variant = get_global_color("outlineVariant");

        match self.variant {
            Card2Variant::Elevated => {
                // Elevated card: surfaceContainerLow with 1.0 elevation
                let default_elevation = self.elevation.unwrap_or(1.0);
                (md_surface_container_low, None, default_elevation)
            }
            Card2Variant::Filled => {
                // Filled card: surfaceContainerHighest with 0.0 elevation
                let default_elevation = self.elevation.unwrap_or(0.0);
                (md_surface_container_highest, None, default_elevation)
            }
            Card2Variant::Outlined => {
                // Outlined card: surface with outline and 0.0 elevation
                let stroke = Some(Stroke::new(1.0, md_outline_variant));
                let default_elevation = self.elevation.unwrap_or(0.0);
                (md_surface, stroke, default_elevation)
            }
        }
    }

    /// Calculate surface tint overlay based on elevation level.
    /// Material 3 uses elevation levels: 0 (0%), 1 (5%), 2 (8%), 3 (11%), 4 (12%), 5 (14%)
    fn calculate_tint_overlay(&self, elevation: f32) -> f32 {
        let opacity = match elevation as i32 {
            0 => 0.0,
            1 => 0.05,
            2..=3 => 0.08,
            4..=6 => 0.11,
            7..=8 => 0.12,
            _ => 0.14,
        };
        opacity
    }

    /// Blend surface tint color with base color based on elevation.
    fn apply_surface_tint(&self, base_color: Color32, elevation: f32) -> Color32 {
        if elevation <= 0.0 {
            return base_color;
        }

        let tint_color = self.surface_tint_color.unwrap_or_else(|| get_global_color("primary"));
        let tint_opacity = self.calculate_tint_overlay(elevation);

        // Blend tint color over base color
        Color32::from_rgba_premultiplied(
            (base_color.r() as f32 * (1.0 - tint_opacity) + tint_color.r() as f32 * tint_opacity) as u8,
            (base_color.g() as f32 * (1.0 - tint_opacity) + tint_color.g() as f32 * tint_opacity) as u8,
            (base_color.b() as f32 * (1.0 - tint_opacity) + tint_color.b() as f32 * tint_opacity) as u8,
            255,
        )
    }
}

impl<'a> Default for MaterialCard2<'a> {
    fn default() -> Self {
        Self::elevated()
    }
}

impl Widget for MaterialCard2<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (base_color, stroke, elevation) = self.get_card_style();
        let shadow_color = self.shadow_color.unwrap_or_else(|| get_global_color("shadow"));
        
        // Apply surface tint overlay based on elevation
        let background_color = self.apply_surface_tint(base_color, elevation);

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
            elevation: _,
            surface_tint_color: _,
            shadow_color: _,
            margin,
            clip_behavior,
            border_on_foreground,
        } = self;

        let sense = if clickable {
            Sense::click()
        } else {
            Sense::hover()
        };

        // Calculate total height based on content
        let header_height = if header_title.is_some() { 72.0 } else { 0.0 };
        let media_height_actual = if media_content.is_some() {
            media_height
        } else {
            0.0
        };
        let content_height = 80.0; // Default content area height
        let actions_height = if actions_content.is_some() { 52.0 } else { 0.0 };

        let total_height = header_height + media_height_actual + content_height + actions_height;
        let card_size = Vec2::new(min_size.x, total_height.max(min_size.y));

        // Apply margin to available space
        let available_with_margin = ui.available_size() - Vec2::new(
            margin * 2.0,
            margin * 2.0,
        );
        let desired_size = available_with_margin.max(card_size);
        
        let (margin_rect, mut response) = ui.allocate_exact_size(desired_size + Vec2::new(
            margin * 2.0,
            margin * 2.0,
        ), sense);
        
        // Apply margin inset
        let rect = Rect::from_min_size(
            margin_rect.min + Vec2::new(margin, margin),
            desired_size,
        );

        if ui.is_rect_visible(rect) {
            // Draw shadow based on elevation
            if elevation > 0.0 {
                let shadow_offset = (elevation * 0.5).min(4.0);
                let shadow_blur = elevation * 0.5;
                let shadow_alpha = (elevation * 3.0).min(30.0) as u8;
                
                let shadow_rect = Rect::from_min_size(
                    rect.min + Vec2::new(0.0, shadow_offset),
                    rect.size(),
                );
                ui.painter().rect_filled(
                    shadow_rect,
                    corner_radius,
                    Color32::from_rgba_unmultiplied(
                        shadow_color.r(),
                        shadow_color.g(),
                        shadow_color.b(),
                        shadow_alpha,
                    ),
                );
            }

            // Draw border behind if needed
            if !border_on_foreground {
                if let Some(stroke) = &stroke {
                    ui.painter().rect_stroke(
                        rect,
                        corner_radius,
                        *stroke,
                        egui::epaint::StrokeKind::Outside,
                    );
                }
            }

            // Draw card background
            ui.painter()
                .rect_filled(rect, corner_radius, background_color);

            let mut current_y = rect.min.y;

            // Draw header
            if let Some(title) = &header_title {
                let _header_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), header_height),
                );

                // Title
                let title_pos = egui::pos2(rect.min.x + 16.0, current_y + 16.0);
                ui.painter().text(
                    title_pos,
                    egui::Align2::LEFT_TOP,
                    title,
                    egui::FontId::proportional(20.0),
                    get_global_color("onSurface"),
                );

                // Subtitle if present
                if let Some(subtitle) = &header_subtitle {
                    let subtitle_pos = egui::pos2(rect.min.x + 16.0, current_y + 44.0);
                    ui.painter().text(
                        subtitle_pos,
                        egui::Align2::LEFT_TOP,
                        subtitle,
                        egui::FontId::proportional(14.0),
                        get_global_color("onSurfaceVariant"),
                    );
                }

                current_y += header_height;
            }

            // Draw media area
            if let Some(media_fn) = media_content {
                let media_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), media_height),
                );

                // Clip media content to card bounds
                let mut media_ui_builder = egui::UiBuilder::new().max_rect(media_rect);
                if clip_behavior {
                    // Enable clipping for media area
                    media_ui_builder = media_ui_builder.sense(Sense::hover());
                }
                
                let media_response = ui.scope_builder(media_ui_builder, |ui| {
                    // Draw media background
                    ui.painter().rect_filled(
                        media_rect,
                        CornerRadius::ZERO,
                        get_global_color("surfaceVariant"),
                    );

                    media_fn(ui)
                });

                response = response.union(media_response.response);
                current_y += media_height;
            }

            // Draw main content
            if let Some(content_fn) = main_content {
                let content_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), content_height),
                );

                let content_response = ui.scope_builder(
                    egui::UiBuilder::new().max_rect(content_rect.shrink(16.0)),
                    |ui| content_fn(ui),
                );

                response = response.union(content_response.response);
                current_y += content_height;
            }

            // Draw actions area
            if let Some(actions_fn) = actions_content {
                let actions_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x, current_y),
                    Vec2::new(rect.width(), actions_height),
                );

                let actions_response = ui.scope_builder(
                    egui::UiBuilder::new().max_rect(actions_rect.shrink2(Vec2::new(8.0, 8.0))),
                    |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            actions_fn(ui)
                        })
                        .inner
                    },
                );

                response = response.union(actions_response.response);
            }

            // Draw border on foreground if needed
            if border_on_foreground {
                if let Some(stroke) = stroke {
                    ui.painter().rect_stroke(
                        rect,
                        corner_radius,
                        stroke,
                        egui::epaint::StrokeKind::Outside,
                    );
                }
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
