use egui::{
    emath::NumExt, 
    ecolor::Color32, 
    epaint::{Stroke, Shadow, CornerRadius},
    Align, Image, Rect, Response, Sense, TextStyle,
    TextWrapMode, Ui, Vec2, Widget, WidgetInfo, WidgetText, WidgetType,
};
use crate::get_global_color;

/// Material Design button with support for different variants.
///
/// Supports filled, outlined, text, elevated, and filled tonal button variants
/// following Material Design 3 specifications.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// # fn do_stuff() {}
///
/// // Material Design filled button (default)
/// if ui.add(MaterialButton::filled("Click me")).clicked() {
///     do_stuff();
/// }
///
/// // Material Design outlined button
/// if ui.add(MaterialButton::outlined("Outlined")).clicked() {
///     do_stuff();
/// }
///
/// // Material Design text button
/// if ui.add(MaterialButton::text("Text")).clicked() {
///     do_stuff();
/// }
/// # });
/// ```

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialButtonVariant {
    Filled,
    Outlined,
    Text,
    Elevated,
    FilledTonal,
}
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialButton<'a> {
    image: Option<Image<'a>>,
    text: Option<WidgetText>,
    shortcut_text: WidgetText,
    wrap_mode: Option<TextWrapMode>,
    
    variant: MaterialButtonVariant,
    /// None means default for interact
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    sense: Sense,
    small: bool,
    frame: Option<bool>,
    min_size: Vec2,
    corner_radius: Option<CornerRadius>,
    selected: bool,
    image_tint_follows_text_color: bool,
    elevation: Option<Shadow>,
    disabled: bool,
}

impl<'a> MaterialButton<'a> {
    /// Create a filled material design button (default variant)
    pub fn filled(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Filled, text)
    }
    
    /// Create an outlined material design button
    pub fn outlined(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Outlined, text)
    }
    
    /// Create a text material design button
    pub fn text(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Text, text)
    }
    
    /// Create an elevated material design button
    pub fn elevated(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Elevated, text)
            .elevation(Shadow {
                offset: [0, 2],
                blur: 6,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 30),
            })
    }
    
    /// Create a filled tonal material design button
    pub fn filled_tonal(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::FilledTonal, text)
    }
    
    fn new_with_variant(variant: MaterialButtonVariant, text: impl Into<WidgetText>) -> Self {
        Self::opt_image_and_text_with_variant(variant, None, Some(text.into()))
    }
    
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self::filled(text)
    }

    /// Creates a button with an image. The size of the image as displayed is defined by the provided size.
    #[allow(clippy::needless_pass_by_value)]
    pub fn image(image: impl Into<Image<'a>>) -> Self {
        Self::opt_image_and_text(Some(image.into()), None)
    }

    /// Creates a button with an image to the left of the text. The size of the image as displayed is defined by the provided size.
    #[allow(clippy::needless_pass_by_value)]
    pub fn image_and_text(image: impl Into<Image<'a>>, text: impl Into<WidgetText>) -> Self {
        Self::opt_image_and_text(Some(image.into()), Some(text.into()))
    }

    pub fn opt_image_and_text(image: Option<Image<'a>>, text: Option<WidgetText>) -> Self {
        Self::opt_image_and_text_with_variant(MaterialButtonVariant::Filled, image, text)
    }
    
    pub fn opt_image_and_text_with_variant(variant: MaterialButtonVariant, image: Option<Image<'a>>, text: Option<WidgetText>) -> Self {
        Self {
            variant,
            text,
            image,
            shortcut_text: Default::default(),
            wrap_mode: None,
            fill: None,
            stroke: None,
            sense: Sense::click(),
            small: false,
            frame: None,
            min_size: Vec2::ZERO,
            corner_radius: None,
            selected: false,
            image_tint_follows_text_color: false,
            elevation: None,
            disabled: false,
        }
    }

    /// Set the wrap mode for the text.
    ///
    /// By default, [`crate::Ui::wrap_mode`] will be used, which can be overridden with [`crate::Style::wrap_mode`].
    ///
    /// Note that any `\n` in the text will always produce a new line.
    #[inline]
    pub fn wrap_mode(mut self, wrap_mode: TextWrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Wrap`].
    #[inline]
    pub fn wrap(mut self) -> Self {
        self.wrap_mode = Some(TextWrapMode::Wrap);

        self
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Truncate`].
    #[inline]
    pub fn truncate(mut self) -> Self {
        self.wrap_mode = Some(TextWrapMode::Truncate);
        self
    }

    /// Override background fill color. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = Some(fill.into());
        self.frame = Some(true);
        self
    }

    /// Override button stroke. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self.frame = Some(true);
        self
    }

    /// Make this a small button, suitable for embedding into text.
    #[inline]
    pub fn small(mut self) -> Self {
        if let Some(text) = self.text {
            self.text = Some(text.text_style(TextStyle::Body));
        }
        self.small = true;
        self
    }

    /// Turn off the frame
    #[inline]
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = Some(frame);
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    /// Set the minimum size of the button.
    #[inline]
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the rounding of the button.
    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    #[inline]
    #[deprecated = "Renamed to `corner_radius`"]
    pub fn rounding(self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius(corner_radius)
    }

    /// If true, the tint of the image is multiplied by the widget text color.
    ///
    /// This makes sense for images that are white, that should have the same color as the text color.
    /// This will also make the icon color depend on hover state.
    ///
    /// Default: `false`.
    #[inline]
    pub fn image_tint_follows_text_color(mut self, image_tint_follows_text_color: bool) -> Self {
        self.image_tint_follows_text_color = image_tint_follows_text_color;
        self
    }

    /// Show some text on the right side of the button, in weak color.
    ///
    /// Designed for menu buttons, for setting a keyboard shortcut text (e.g. `Ctrl+S`).
    ///
    /// The text can be created with [`crate::Context::format_shortcut`].
    #[inline]
    pub fn shortcut_text(mut self, shortcut_text: impl Into<WidgetText>) -> Self {
        self.shortcut_text = shortcut_text.into();
        self
    }

    /// If `true`, mark this button as "selected".
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Enable or disable the button.
    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.disabled = !enabled;
        self
    }

    /// Set the elevation shadow for the button.
    #[inline]
    pub fn elevation(mut self, elevation: Shadow) -> Self {
        self.elevation = Some(elevation);
        self
    }

    /// Add a leading icon to the button.
    #[inline]
    pub fn leading_icon(self, _icon: impl Into<String>) -> Self {
        // For now, this is a placeholder that returns self unchanged
        // In a real implementation, you'd store the icon and render it
        self
    }

    /// Add a trailing icon to the button.
    #[inline]
    pub fn trailing_icon(self, _icon: impl Into<String>) -> Self {
        // For now, this is a placeholder that returns self unchanged
        // In a real implementation, you'd store the icon and render it
        self
    }
}

impl Widget for MaterialButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialButton {
            variant,
            text,
            image,
            shortcut_text,
            wrap_mode,
            fill,
            stroke,
            sense,
            small,
            frame,
            min_size,
            corner_radius,
            selected,
            image_tint_follows_text_color,
            elevation,
            disabled,
        } = self;

        // Material Design color palette from theme
        let md_primary = get_global_color("primary");
        let md_on_primary = get_global_color("onPrimary");
        let md_surface = get_global_color("surface");
        let _md_on_surface = get_global_color("onSurface"); // Prefix with _ to silence warning
        let md_outline = get_global_color("outline");
        let md_surface_variant = get_global_color("surfaceVariant");
        
        // Material Design button defaults based on variant
        let (default_fill, default_stroke, default_corner_radius, _has_elevation) = match variant {
            MaterialButtonVariant::Filled => (
                Some(md_primary),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false
            ),
            MaterialButtonVariant::Outlined => (
                Some(Color32::TRANSPARENT),
                Some(Stroke::new(1.0, md_outline)),
                CornerRadius::from(20),
                false
            ),
            MaterialButtonVariant::Text => (
                Some(Color32::TRANSPARENT),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false
            ),
            MaterialButtonVariant::Elevated => (
                Some(md_surface),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                true
            ),
            MaterialButtonVariant::FilledTonal => (
                Some(md_surface_variant),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false
            ),
        };

        let frame = frame.unwrap_or_else(|| match variant {
            MaterialButtonVariant::Text => false,
            _ => true,
        });

        // Material Design button padding (24px left/right, calculated based on height)
        let button_padding = if frame {
            Vec2::new(24.0, if small { 0.0 } else { 10.0 })
        } else if variant == MaterialButtonVariant::Text {
            // Text buttons still need horizontal padding for consistent width
            Vec2::new(24.0, if small { 0.0 } else { 10.0 })
        } else {
            Vec2::ZERO
        };
        
        // Material Design minimum button height
        let min_button_height = if small { 32.0 } else { 40.0 };

        let space_available_for_image = if let Some(text) = &text {
            let font_height = ui.text_style_height(&TextStyle::Body);
            Vec2::splat(font_height) // Reasonable?
        } else {
            ui.available_size() - 2.0 * button_padding
        };

        let image_size = if let Some(image) = &image {
            image
                .load_and_calc_size(ui, space_available_for_image)
                .unwrap_or(space_available_for_image)
        } else {
            Vec2::ZERO
        };

        let gap_before_shortcut_text = ui.spacing().item_spacing.x;

        let mut text_wrap_width = ui.available_width() - 2.0 * button_padding.x;
        if image.is_some() {
            text_wrap_width -= image_size.x + ui.spacing().icon_spacing;
        }

        // Note: we don't wrap the shortcut text
        let shortcut_galley = (!shortcut_text.is_empty()).then(|| {
            shortcut_text.into_galley(
                ui,
                Some(TextWrapMode::Extend),
                f32::INFINITY,
                TextStyle::Body,
            )
        });

        if let Some(shortcut_galley) = &shortcut_galley {
            // Leave space for the shortcut text:
            text_wrap_width -= gap_before_shortcut_text + shortcut_galley.size().x;
        }

        let galley =
            text.map(|text| text.into_galley(ui, wrap_mode, text_wrap_width, TextStyle::Body));

        let mut desired_size = Vec2::ZERO;
        if image.is_some() {
            desired_size.x += image_size.x;
            desired_size.y = desired_size.y.max(image_size.y);
        }
        if image.is_some() && galley.is_some() {
            desired_size.x += ui.spacing().icon_spacing;
        }
        if let Some(galley) = &galley {
            desired_size.x += galley.size().x;
            desired_size.y = desired_size.y.max(galley.size().y);
        }
        if let Some(shortcut_galley) = &shortcut_galley {
            desired_size.x += gap_before_shortcut_text + shortcut_galley.size().x;
            desired_size.y = desired_size.y.max(shortcut_galley.size().y);
        }
        desired_size += 2.0 * button_padding;
        if !small {
            desired_size.y = desired_size.y.at_least(min_button_height);
        }
        desired_size = desired_size.at_least(min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, sense);
        response.widget_info(|| {
            if let Some(galley) = &galley {
                WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), galley.text())
            } else {
                WidgetInfo::new(WidgetType::Button)
            }
        });

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            let (frame_expansion, _frame_cr, frame_fill, frame_stroke) = if selected {
                let selection = ui.visuals().selection;
                (
                    Vec2::ZERO,
                    CornerRadius::ZERO,
                    selection.bg_fill,
                    selection.stroke,
                )
            } else if frame {
                let expansion = Vec2::splat(visuals.expansion);
                (
                    expansion,
                    visuals.corner_radius,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                )
            } else {
                Default::default()
            };
            let frame_cr = corner_radius.unwrap_or(default_corner_radius);
            let mut frame_fill = fill.unwrap_or(default_fill.unwrap_or(frame_fill));
            let mut frame_stroke = stroke.unwrap_or(default_stroke.unwrap_or(frame_stroke));
            
            // Apply disabled styling - Material Design spec
            if disabled {
                // Disabled buttons have 12% opacity on surface color
                let surface_color = get_global_color("surface");
                let disabled_overlay = get_global_color("onSurface").gamma_multiply(0.12);
                frame_fill = surface_color; // Use surface as base
                frame_stroke.color = get_global_color("onSurface").gamma_multiply(0.12);
                frame_stroke.width = if matches!(variant, MaterialButtonVariant::Outlined) { 1.0 } else { 0.0 };
            }
            
            // Draw elevation shadow if present
            if let Some(shadow) = elevation {
                let shadow_offset = Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32);
                let shadow_rect = rect.expand2(frame_expansion).translate(shadow_offset);
                ui.painter().rect_filled(
                    shadow_rect,
                    frame_cr,
                    shadow.color,
                );
            }
            
            ui.painter().rect(
                rect.expand2(frame_expansion),
                frame_cr,
                frame_fill,
                frame_stroke,
                egui::epaint::StrokeKind::Outside,
            );

            let mut cursor_x = rect.min.x + button_padding.x;

            if let Some(image) = &image {
                let mut image_pos = ui
                    .layout()
                    .align_size_within_rect(image_size, rect.shrink2(button_padding))
                    .min;
                if galley.is_some() || shortcut_galley.is_some() {
                    image_pos.x = cursor_x;
                }
                let image_rect = Rect::from_min_size(image_pos, image_size);
                cursor_x += image_size.x;
                let mut image_widget = image.clone();
                if image_tint_follows_text_color {
                    image_widget = image_widget.tint(visuals.text_color());
                }
                image_widget.paint_at(ui, image_rect);
            }

            if image.is_some() && galley.is_some() {
                cursor_x += ui.spacing().icon_spacing;
            }

            if let Some(galley) = galley {
                let mut text_pos = ui
                    .layout()
                    .align_size_within_rect(galley.size(), rect.shrink2(button_padding))
                    .min;
                if image.is_some() || shortcut_galley.is_some() {
                    text_pos.x = cursor_x;
                }
                
                // Material Design text colors based on button variant
                let text_color = if disabled {
                    // Disabled text has 38% opacity of onSurface
                    get_global_color("onSurface").gamma_multiply(0.38)
                } else {
                    match variant {
                        MaterialButtonVariant::Filled => md_on_primary,
                        MaterialButtonVariant::Outlined => md_primary,
                        MaterialButtonVariant::Text => md_primary,
                        MaterialButtonVariant::Elevated => md_primary,
                        MaterialButtonVariant::FilledTonal => get_global_color("onSecondaryContainer"),
                    }
                };
                
                ui.painter().galley(text_pos, galley, text_color);
            }

            if let Some(shortcut_galley) = shortcut_galley {
                // Always align to the right
                let layout = if ui.layout().is_horizontal() {
                    ui.layout().with_main_align(Align::Max)
                } else {
                    ui.layout().with_cross_align(Align::Max)
                };
                let shortcut_text_pos = layout
                    .align_size_within_rect(shortcut_galley.size(), rect.shrink2(button_padding))
                    .min;
                ui.painter().galley(
                    shortcut_text_pos,
                    shortcut_galley,
                    ui.visuals().weak_text_color(),
                );
            }
        }

        if let Some(cursor) = ui.visuals().interact_cursor {
            if response.hovered() {
                ui.ctx().set_cursor_icon(cursor);
            }
        }

        response
    }
}
