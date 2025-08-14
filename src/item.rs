use crate::theme::get_global_color;
use egui::{
    ecolor::Color32, 
    epaint::{Stroke, CornerRadius},
    Align, Layout, Response, Sense, TextStyle, Ui, Vec2, Widget, WidgetText,
};

/// Material Design item layout component.
///
/// An item layout component that can be used inside list items to give them
/// their customizable structure. This component provides structured layouts
/// for text and content following Material Design 3 specifications.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // Single line item
/// ui.add(MaterialItem::new()
///     .headline("Single line item"));
///
/// // Two line item with supporting text
/// ui.add(MaterialItem::new()
///     .headline("Two line item")
///     .supporting_text("Supporting text"));
///
/// // Three line item with multiple content
/// ui.add(MaterialItem::new()
///     .headline("Three line item")
///     .supporting_text("Second line text\nThird line text")
///     .leading_icon("📄")
///     .trailing_icon("⭐"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialItem {
    headline: Option<WidgetText>,
    overline: Option<WidgetText>,
    supporting_text: Option<WidgetText>,
    trailing_supporting_text: Option<WidgetText>,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    leading_content: Option<String>,
    trailing_content: Option<String>,
    multiline: Option<bool>,
    width: f32,
    clickable: bool,
    outlined: bool,
}

impl MaterialItem {
    /// Create a new material item.
    pub fn new() -> Self {
        Self {
            headline: None,
            overline: None,
            supporting_text: None,
            trailing_supporting_text: None,
            leading_icon: None,
            trailing_icon: None,
            leading_content: None,
            trailing_content: None,
            multiline: None,
            width: 300.0, // Default width from Material Web stories
            clickable: false,
            outlined: true,
        }
    }

    /// Set the headline text (main content).
    pub fn headline(mut self, text: impl Into<WidgetText>) -> Self {
        self.headline = Some(text.into());
        self
    }

    /// Set the overline text (appears above headline).
    pub fn overline(mut self, text: impl Into<WidgetText>) -> Self {
        self.overline = Some(text.into());
        self
    }

    /// Set the supporting text (appears below headline).
    pub fn supporting_text(mut self, text: impl Into<WidgetText>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set the trailing supporting text (small text at the end).
    pub fn trailing_supporting_text(mut self, text: impl Into<WidgetText>) -> Self {
        self.trailing_supporting_text = Some(text.into());
        self
    }

    /// Set a leading icon.
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set a trailing icon.
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set custom leading content.
    pub fn leading_content(mut self, content: impl Into<String>) -> Self {
        self.leading_content = Some(content.into());
        self
    }

    /// Set custom trailing content.
    pub fn trailing_content(mut self, content: impl Into<String>) -> Self {
        self.trailing_content = Some(content.into());
        self
    }

    /// Explicitly set multiline mode.
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = Some(multiline);
        self
    }

    /// Set the width of the item.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Make the item clickable.
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set whether to show an outline around the item.
    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    fn is_multiline(&self) -> bool {
        if let Some(multiline) = self.multiline {
            return multiline;
        }

        // Auto-detect multiline based on content
        let text_slots = [
            &self.overline,
            &self.headline,
            &self.supporting_text,
        ];

        let slots_with_content = text_slots.iter()
            .filter(|slot| slot.is_some())
            .count();

        slots_with_content > 1
    }

    fn get_item_height(&self) -> f32 {
        if self.is_multiline() {
            // Material Design multiline item height
            if self.supporting_text.is_some() && 
               self.supporting_text.as_ref().unwrap().text().contains('\n') {
                88.0 // Three+ line item
            } else {
                72.0 // Two line item
            }
        } else {
            56.0 // Single line item
        }
    }
}

impl Default for MaterialItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MaterialItem {
    fn ui(self, ui: &mut Ui) -> Response {
        // Calculate height first
        let item_height = self.get_item_height();
        
        let MaterialItem {
            headline,
            overline,
            supporting_text,
            trailing_supporting_text,
            leading_icon,
            trailing_icon,
            leading_content,
            trailing_content,
            multiline: _,
            width,
            clickable,
            outlined,
        } = self;

        // Material Design colors
        let md_outline = get_global_color("outline");
        let md_on_surface = get_global_color("onSurface");
        let md_on_surface_variant = get_global_color("onSurfaceVariant");
        let desired_size = Vec2::new(width, item_height);

        let sense = if clickable {
            Sense::click()
        } else {
            Sense::hover()
        };

        let response = ui.allocate_response(desired_size, sense);
        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw outline if enabled
            if outlined {
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::from(16),
                    Stroke::new(1.0, md_outline),
                    egui::epaint::StrokeKind::Outside,
                );
            }

            // Content area with padding
            let content_rect = rect.shrink2(Vec2::new(16.0, 12.0));
            
            // Layout the content
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                ui.horizontal(|ui| {
                    // Leading content (icons, etc.)
                    if let Some(icon) = &leading_icon {
                        ui.vertical_centered(|ui| {
                            ui.colored_label(md_on_surface_variant, icon);
                        });
                        ui.add_space(16.0);
                    } else if let Some(content) = &leading_content {
                        ui.vertical_centered(|ui| {
                            ui.label(content);
                        });
                        ui.add_space(16.0);
                    }

                    // Main text content
                    ui.vertical(|ui| {
                        let reserved_width = if trailing_supporting_text.is_some() || trailing_icon.is_some() || trailing_content.is_some() { 80.0 } else { 0.0 };
                        let content_width = (ui.available_width() - reserved_width).max(120.0); // Minimum 120px for text content
                        ui.set_width(content_width);
                        
                        // Overline
                        if let Some(overline_text) = &overline {
                            ui.style_mut().override_text_style = Some(TextStyle::Small);
                            ui.colored_label(md_on_surface_variant, overline_text.text());
                        }

                        // Headline
                        if let Some(headline_text) = &headline {
                            ui.style_mut().override_text_style = Some(TextStyle::Body);
                            ui.colored_label(md_on_surface, headline_text.text());
                        }

                        // Supporting text
                        if let Some(supporting_text_content) = &supporting_text {
                            ui.style_mut().override_text_style = Some(TextStyle::Small);
                            let lines: Vec<&str> = supporting_text_content.text().lines().collect();
                            for line in lines {
                                ui.colored_label(md_on_surface_variant, line);
                            }
                        }
                    });

                    // Trailing content
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if let Some(icon) = &trailing_icon {
                            ui.colored_label(md_on_surface_variant, icon);
                        } else if let Some(content) = &trailing_content {
                            ui.label(content);
                        }

                        if let Some(trailing_text) = &trailing_supporting_text {
                            ui.add_space(8.0);
                            ui.style_mut().override_text_style = Some(TextStyle::Small);
                            ui.colored_label(md_on_surface_variant, trailing_text.text());
                        }
                    });
                });
            });
        }

        response
    }
}

/// Convenience function to create a new material item.
pub fn material_item() -> MaterialItem {
    MaterialItem::new()
}