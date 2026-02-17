use crate::{get_global_color, icons::icon_text};
use egui::{
    ecolor::Color32,
    emath::NumExt,
    epaint::{CornerRadius, Shadow, Stroke},
    Align, Image, Rect, Response, Sense, TextStyle, TextWrapMode, Ui, Vec2, Widget, WidgetInfo,
    WidgetText, WidgetType,
};

/// Material Design button with support for different variants.
///
/// Supports filled, outlined, text, elevated, and filled tonal button variants
/// following Material Design 3 specifications.
///
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// # fn do_stuff() {}
///
/// // Material Design filled button (default, high emphasis)
/// if ui.add(MaterialButton::filled("Click me")).clicked() {
///     do_stuff();
/// }
///
/// // Material Design outlined button (medium emphasis)
/// if ui.add(MaterialButton::outlined("Outlined")).clicked() {
///     do_stuff();
/// }
///
/// // Material Design text button (low emphasis)
/// if ui.add(MaterialButton::text("Text")).clicked() {
///     do_stuff();
/// }
///
/// // Material Design elevated button (medium emphasis with shadow)
/// if ui.add(MaterialButton::elevated("Elevated")).clicked() {
///     do_stuff();
/// }
///
/// // Material Design filled tonal button (medium emphasis, toned down)
/// if ui.add(MaterialButton::filled_tonal("Tonal")).clicked() {
///     do_stuff();
/// }
///
/// // Button with custom properties
/// if ui.add(
///     MaterialButton::filled("Custom")
///         .min_size(Vec2::new(120.0, 40.0))
///         .enabled(true)
///         .selected(false)
/// ).clicked() {
///     do_stuff();
/// }
/// # });
/// ```

/// Material Design button variants following Material Design 3 specifications
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialButtonVariant {
    /// Filled button - High emphasis, filled background with primary color
    Filled,
    /// Outlined button - Medium emphasis, transparent background with outline
    Outlined,
    /// Text button - Low emphasis, transparent background, no outline  
    Text,
    /// Elevated button - Medium emphasis, filled background with shadow elevation
    Elevated,
    /// Filled tonal button - Medium emphasis, filled background with secondary container color
    FilledTonal,
}

/// Material Design button widget implementing Material Design 3 button specifications
///
/// This widget provides a button that follows Material Design guidelines including:
/// - Proper color schemes for different variants
/// - Hover and pressed state animations
/// - Material Design typography
/// - Accessibility support
/// - Icon and text support
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialButton<'a> {
    /// Optional image/icon to display alongside or instead of text
    image: Option<Image<'a>>,
    /// Text content of the button
    text: Option<WidgetText>,
    /// Keyboard shortcut text displayed on the button (usually right-aligned)
    shortcut_text: WidgetText,
    /// Text wrapping behavior for long button text
    wrap_mode: Option<TextWrapMode>,

    /// Button variant (filled, outlined, text, elevated, filled tonal)
    variant: MaterialButtonVariant,
    /// Custom background fill color (None uses variant default)
    fill: Option<Color32>,
    /// Custom stroke/outline settings (None uses variant default)
    stroke: Option<Stroke>,
    /// Mouse/touch interaction sensitivity settings
    sense: Sense,
    /// Whether to render as a smaller compact button
    small: bool,
    /// Whether to show the button frame/background (None uses variant default)
    frame: Option<bool>,
    /// Minimum size constraints for the button
    min_size: Vec2,
    /// Custom corner radius (None uses Material Design default of 20dp/10px)
    corner_radius: Option<CornerRadius>,
    /// Whether the button appears in selected/pressed state
    selected: bool,
    /// If true, the tint of the image is multiplied by the widget text color.
    ///
    /// This makes sense for images that are white, that should have the same color as the text color.
    /// This will also make the icon color depend on hover state.
    ///
    /// Default: `false`.
    image_tint_follows_text_color: bool,
    /// Custom elevation shadow for the button (None uses variant default)
    elevation: Option<Shadow>,
    /// Whether the button is disabled (non-interactive)
    disabled: bool,
    /// Leading icon name (rendered using Material Symbols font)
    leading_icon: Option<String>,
    /// Trailing icon name (rendered using Material Symbols font)
    trailing_icon: Option<String>,
    /// Custom text color override (None uses variant default)
    text_color: Option<Color32>,
}

impl<'a> MaterialButton<'a> {
    /// Create a filled Material Design button with high emphasis
    ///
    /// Filled buttons have the most visual impact and should be used for
    /// the primary action in a set of buttons.
    ///
    /// ## Material Design Spec
    /// - Background: Primary color
    /// - Text: On-primary color  
    /// - Elevation: 0dp (no shadow)
    /// - Corner radius: 20dp
    pub fn filled(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Filled, text)
    }

    /// Create an outlined Material Design button with medium emphasis
    ///
    /// Outlined buttons are medium-emphasis buttons. They contain actions
    /// that are important but aren't the primary action in an app.
    ///
    /// ## Material Design Spec  
    /// - Background: Transparent
    /// - Text: Primary color
    /// - Outline: 1dp primary color
    /// - Corner radius: 20dp
    pub fn outlined(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Outlined, text)
    }

    /// Create a text Material Design button with low emphasis
    ///
    /// Text buttons are used for the least important actions in a UI.
    /// They're often used for secondary actions.
    ///
    /// ## Material Design Spec
    /// - Background: Transparent  
    /// - Text: Primary color
    /// - No outline or elevation
    /// - Corner radius: 20dp
    pub fn text(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Text, text)
    }

    /// Create an elevated Material Design button with medium emphasis
    ///
    /// Elevated buttons are essentially filled buttons with a shadow.
    /// Use them to add separation between button and background.
    ///
    /// ## Material Design Spec
    /// - Background: Surface color
    /// - Text: Primary color
    /// - Elevation: 1dp shadow
    /// - Corner radius: 20dp  
    pub fn elevated(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Elevated, text).elevation(Shadow {
            offset: [0, 2],
            blur: 6,
            spread: 0,
            color: Color32::from_rgba_unmultiplied(0, 0, 0, 30),
        })
    }

    /// Create a filled tonal Material Design button with medium emphasis
    ///
    /// Filled tonal buttons are used to convey a secondary action that is
    /// still important, but not the primary action.
    ///
    /// ## Material Design Spec
    /// - Background: Secondary container color
    /// - Text: On-secondary-container color
    /// - Elevation: 0dp (no shadow)
    /// - Corner radius: 20dp
    pub fn filled_tonal(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::FilledTonal, text)
    }

    /// Internal constructor that creates a button with the specified variant and text
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

    /// Creates a button with an image. The size of the image as displayed is defined by the provided size.
    ///
    /// Use this when you need both or either an image and text, or when text might be None.
    ///
    /// ## Parameters
    /// - `image`: Optional icon/image to display
    /// - `text`: Optional text content
    pub fn opt_image_and_text(image: Option<Image<'a>>, text: Option<WidgetText>) -> Self {
        Self::opt_image_and_text_with_variant(MaterialButtonVariant::Filled, image, text)
    }

    /// Create a Material Design button with specific variant and optional image and text
    ///
    /// This is the most flexible constructor allowing full control over button content.
    ///
    /// ## Parameters
    /// - `variant`: The Material Design button variant to use
    /// - `image`: Optional icon/image to display  
    /// - `text`: Optional text content
    pub fn opt_image_and_text_with_variant(
        variant: MaterialButtonVariant,
        image: Option<Image<'a>>,
        text: Option<WidgetText>,
    ) -> Self {
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
            leading_icon: None,
            trailing_icon: None,
            text_color: None,
        }
    }

    /// Set the wrap mode for the text.
    ///
    /// By default, [`egui::Ui::wrap_mode`] will be used, which can be overridden with [`egui::Style::wrap_mode`].
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
    /// The text can be created with [`egui::Context::format_shortcut`].
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

    /// Add a leading icon to the button (rendered before the text).
    ///
    /// Uses Material Symbols icon font. Pass the icon name (e.g., "upload", "search").
    #[inline]
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Add a trailing icon to the button (rendered after the text).
    ///
    /// Uses Material Symbols icon font. Pass the icon name (e.g., "arrow_forward", "open_in_new").
    #[inline]
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Override the text color for this button.
    ///
    /// When set, overrides the variant-based text color.
    /// Icon colors also follow this override.
    #[inline]
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
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
            leading_icon,
            trailing_icon,
            text_color: custom_text_color,
        } = self;

        // Material Design color palette from theme
        let md_primary = get_global_color("primary");
        let md_surface_tint = get_global_color("surfaceTint");
        let md_on_primary = get_global_color("onPrimary");
        let md_primary_container = get_global_color("primaryContainer");
        let md_on_primary_container = get_global_color("onPrimaryContainer");
        let md_secondary = get_global_color("secondary");
        let md_on_secondary = get_global_color("onSecondary");
        let md_secondary_container = get_global_color("secondaryContainer");
        let md_on_secondary_container = get_global_color("onSecondaryContainer");
        let md_tertiary = get_global_color("tertiary");
        let md_on_tertiary = get_global_color("onTertiary");
        let md_tertiary_container = get_global_color("tertiaryContainer");
        let md_on_tertiary_container = get_global_color("onTertiaryContainer");
        let md_error = get_global_color("error");
        let md_on_error = get_global_color("onError");
        let md_error_container = get_global_color("errorContainer");
        let md_on_error_container = get_global_color("onErrorContainer");
        let md_background = get_global_color("background");
        let md_on_background = get_global_color("onBackground");
        let md_surface = get_global_color("surface");
        let md_on_surface = get_global_color("onSurface");
        let md_surface_variant = get_global_color("surfaceVariant");
        let md_on_surface_variant = get_global_color("onSurfaceVariant");
        let md_outline = get_global_color("outline");
        let md_outline_variant = get_global_color("outlineVariant");
        let md_shadow = get_global_color("shadow");
        let md_scrim = get_global_color("scrim");
        let md_inverse_surface = get_global_color("inverseSurface");
        let md_inverse_on_surface = get_global_color("inverseOnSurface");
        let md_inverse_primary = get_global_color("inversePrimary");
        let md_primary_fixed = get_global_color("primaryFixed");
        let md_on_primary_fixed = get_global_color("onPrimaryFixed");
        let md_primary_fixed_dim = get_global_color("primaryFixedDim");
        let md_on_primary_fixed_variant = get_global_color("onPrimaryFixedVariant");
        let md_secondary_fixed = get_global_color("secondaryFixed");
        let md_on_secondary_fixed = get_global_color("onSecondaryFixed");
        let md_secondary_fixed_dim = get_global_color("secondaryFixedDim");
        let md_on_secondary_fixed_variant = get_global_color("onSecondaryFixedVariant");
        let md_tertiary_fixed = get_global_color("tertiaryFixed");
        let md_on_tertiary_fixed = get_global_color("onTertiaryFixed");
        let md_tertiary_fixed_dim = get_global_color("tertiaryFixedDim");
        let md_on_tertiary_fixed_variant = get_global_color("onTertiaryFixedVariant");
        let md_surface_dim = get_global_color("surfaceDim");
        let md_surface_bright = get_global_color("surfaceBright");
        let md_surface_container_lowest = get_global_color("surfaceContainerLowest");
        let md_surface_container_low = get_global_color("surfaceContainerLow");
        let md_surface_container = get_global_color("surfaceContainer");
        let md_surface_container_high = get_global_color("surfaceContainerHigh");
        let md_surface_container_highest = get_global_color("surfaceContainerHighest");

        // Material Design button defaults based on variant
        let (default_fill, default_stroke, default_corner_radius, _has_elevation) = match variant {
            MaterialButtonVariant::Filled => (
                Some(md_primary),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false,
            ),
            MaterialButtonVariant::Outlined => (
                Some(Color32::TRANSPARENT),
                Some(Stroke::new(1.0, md_outline)),
                CornerRadius::from(20),
                false,
            ),
            MaterialButtonVariant::Text => (
                Some(Color32::TRANSPARENT),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false,
            ),
            MaterialButtonVariant::Elevated => (
                Some(md_surface),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                true,
            ),
            MaterialButtonVariant::FilledTonal => (
                Some(md_surface_variant),
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false,
            ),
        };

        let frame = frame.unwrap_or_else(|| match variant {
            MaterialButtonVariant::Text => false,
            _ => true,
        });

        // Material Design button padding
        // With leading icon: 16px left, 24px right
        // With trailing icon: 24px left, 16px right
        // With both icons: 16px left, 16px right
        // No icons: 24px left, 24px right
        let has_leading = leading_icon.is_some() || image.is_some();
        let has_trailing = trailing_icon.is_some();
        let padding_left = if has_leading { 16.0 } else { 24.0 };
        let padding_right = if has_trailing { 16.0 } else { 24.0 };
        let button_padding_left;
        let button_padding_right;
        let button_padding_y;
        if frame || variant == MaterialButtonVariant::Text {
            button_padding_left = padding_left;
            button_padding_right = padding_right;
            button_padding_y = if small { 0.0 } else { 10.0 };
        } else {
            button_padding_left = 0.0;
            button_padding_right = 0.0;
            button_padding_y = 0.0;
        }

        // Material Design minimum button height
        let min_button_height = if small { 32.0 } else { 40.0 };
        let icon_spacing = 8.0; // Material Design icon-to-text gap

        // Resolve the variant-based text color (used for text and icons)
        let resolved_text_color = if disabled {
            md_background.gamma_multiply(0.38)
        } else if let Some(custom) = custom_text_color {
            custom
        } else {
            match variant {
                MaterialButtonVariant::Filled => md_background,
                MaterialButtonVariant::Outlined => md_on_background,
                MaterialButtonVariant::Text => md_on_background,
                MaterialButtonVariant::Elevated => md_on_background,
                MaterialButtonVariant::FilledTonal => get_global_color("onSecondaryContainer"),
            }
        };

        // Build leading icon galley
        let leading_icon_galley = leading_icon.map(|name| {
            let icon_str: WidgetText = icon_text(&name).into();
            icon_str.into_galley(ui, Some(TextWrapMode::Extend), f32::INFINITY, TextStyle::Body)
        });

        // Build trailing icon galley
        let trailing_icon_galley = trailing_icon.map(|name| {
            let icon_str: WidgetText = icon_text(&name).into();
            icon_str.into_galley(ui, Some(TextWrapMode::Extend), f32::INFINITY, TextStyle::Body)
        });

        let space_available_for_image = if let Some(_text) = &text {
            let font_height = ui.text_style_height(&TextStyle::Body);
            Vec2::splat(font_height)
        } else {
            let total_h_padding = button_padding_left + button_padding_right;
            ui.available_size() - Vec2::new(total_h_padding, 2.0 * button_padding_y)
        };

        let image_size = if let Some(image) = &image {
            image
                .load_and_calc_size(ui, space_available_for_image)
                .unwrap_or(space_available_for_image)
        } else {
            Vec2::ZERO
        };

        let gap_before_shortcut_text = ui.spacing().item_spacing.x;

        let mut text_wrap_width = ui.available_width() - button_padding_left - button_padding_right;
        if image.is_some() {
            text_wrap_width -= image_size.x + icon_spacing;
        }
        if leading_icon_galley.is_some() {
            text_wrap_width -= leading_icon_galley.as_ref().unwrap().size().x + icon_spacing;
        }
        if trailing_icon_galley.is_some() {
            text_wrap_width -= trailing_icon_galley.as_ref().unwrap().size().x + icon_spacing;
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
            text_wrap_width -= gap_before_shortcut_text + shortcut_galley.size().x;
        }

        let galley =
            text.map(|text| text.into_galley(ui, wrap_mode, text_wrap_width, TextStyle::Body));

        let mut desired_size = Vec2::ZERO;

        // Leading icon
        if let Some(lg) = &leading_icon_galley {
            desired_size.x += lg.size().x;
            desired_size.y = desired_size.y.max(lg.size().y);
        }

        // Image
        if image.is_some() {
            if leading_icon_galley.is_some() {
                desired_size.x += icon_spacing;
            }
            desired_size.x += image_size.x;
            desired_size.y = desired_size.y.max(image_size.y);
        }

        // Gap between leading content and text
        if (leading_icon_galley.is_some() || image.is_some()) && galley.is_some() {
            desired_size.x += icon_spacing;
        }

        if let Some(galley) = &galley {
            desired_size.x += galley.size().x;
            desired_size.y = desired_size.y.max(galley.size().y);
        }

        // Trailing icon
        if let Some(tg) = &trailing_icon_galley {
            if galley.is_some() || image.is_some() || leading_icon_galley.is_some() {
                desired_size.x += icon_spacing;
            }
            desired_size.x += tg.size().x;
            desired_size.y = desired_size.y.max(tg.size().y);
        }

        if let Some(shortcut_galley) = &shortcut_galley {
            desired_size.x += gap_before_shortcut_text + shortcut_galley.size().x;
            desired_size.y = desired_size.y.max(shortcut_galley.size().y);
        }

        desired_size.x += button_padding_left + button_padding_right;
        desired_size.y += 2.0 * button_padding_y;
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
                let surface_color = get_global_color("surface");
                frame_fill = surface_color;
                frame_stroke.color = md_on_surface.gamma_multiply(0.12);
                frame_stroke.width = if matches!(variant, MaterialButtonVariant::Outlined) {
                    1.0
                } else {
                    0.0
                };
            }

            // Material Design state layers (hover/press overlays)
            if !disabled {
                let state_layer_color = resolved_text_color;
                if response.is_pointer_button_down_on() {
                    // Pressed: 12% overlay
                    frame_fill = blend_overlay(frame_fill, state_layer_color, 0.12);
                } else if response.hovered() {
                    // Hovered: 8% overlay
                    frame_fill = blend_overlay(frame_fill, state_layer_color, 0.08);
                }
            }

            // Draw elevation shadow if present
            if let Some(shadow) = &elevation {
                // Hover elevation boost for elevated buttons
                let shadow = if !disabled && response.hovered() {
                    Shadow {
                        offset: [shadow.offset[0], shadow.offset[1] + 2],
                        blur: shadow.blur + 4,
                        spread: shadow.spread,
                        color: shadow.color,
                    }
                } else {
                    *shadow
                };
                let shadow_offset = Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32);
                let shadow_rect = rect.expand2(frame_expansion).translate(shadow_offset);
                ui.painter()
                    .rect_filled(shadow_rect, frame_cr, shadow.color);
            }

            ui.painter().rect(
                rect.expand2(frame_expansion),
                frame_cr,
                frame_fill,
                frame_stroke,
                egui::epaint::StrokeKind::Outside,
            );

            let mut cursor_x = rect.min.x + button_padding_left;
            let content_rect_y_min = rect.min.y + button_padding_y;
            let content_rect_y_max = rect.max.y - button_padding_y;
            let content_height = content_rect_y_max - content_rect_y_min;

            // Draw leading icon
            if let Some(leading_galley) = &leading_icon_galley {
                let icon_y =
                    content_rect_y_min + (content_height - leading_galley.size().y) / 2.0;
                let icon_pos = egui::pos2(cursor_x, icon_y);
                ui.painter()
                    .galley(icon_pos, leading_galley.clone(), resolved_text_color);
                cursor_x += leading_galley.size().x + icon_spacing;
            }

            // Draw image
            if let Some(image) = &image {
                let mut image_pos = ui
                    .layout()
                    .align_size_within_rect(
                        image_size,
                        Rect::from_min_max(
                            egui::pos2(cursor_x, content_rect_y_min),
                            egui::pos2(rect.max.x - button_padding_right, content_rect_y_max),
                        ),
                    )
                    .min;
                if galley.is_some() || shortcut_galley.is_some() || trailing_icon_galley.is_some() {
                    image_pos.x = cursor_x;
                }
                let image_rect = Rect::from_min_size(image_pos, image_size);
                cursor_x += image_size.x + icon_spacing;
                let mut image_widget = image.clone();
                if image_tint_follows_text_color {
                    image_widget = image_widget.tint(visuals.text_color());
                }
                image_widget.paint_at(ui, image_rect);
            }

            // Draw main text
            if let Some(galley) = galley {
                let text_y = content_rect_y_min + (content_height - galley.size().y) / 2.0;
                let mut text_pos = egui::pos2(cursor_x, text_y);
                // Center text if no leading/trailing elements
                if leading_icon_galley.is_none()
                    && image.is_none()
                    && trailing_icon_galley.is_none()
                    && shortcut_galley.is_none()
                {
                    text_pos = ui
                        .layout()
                        .align_size_within_rect(
                            galley.size(),
                            Rect::from_min_max(
                                egui::pos2(
                                    rect.min.x + button_padding_left,
                                    content_rect_y_min,
                                ),
                                egui::pos2(
                                    rect.max.x - button_padding_right,
                                    content_rect_y_max,
                                ),
                            ),
                        )
                        .min;
                }

                cursor_x = text_pos.x + galley.size().x;
                ui.painter().galley(text_pos, galley, resolved_text_color);
            }

            // Draw trailing icon
            if let Some(trailing_galley) = &trailing_icon_galley {
                cursor_x += icon_spacing;
                let icon_y =
                    content_rect_y_min + (content_height - trailing_galley.size().y) / 2.0;
                let icon_pos = egui::pos2(cursor_x, icon_y);
                ui.painter()
                    .galley(icon_pos, trailing_galley.clone(), resolved_text_color);
            }

            // Draw shortcut text
            if let Some(shortcut_galley) = shortcut_galley {
                let layout = if ui.layout().is_horizontal() {
                    ui.layout().with_main_align(Align::Max)
                } else {
                    ui.layout().with_cross_align(Align::Max)
                };
                let shortcut_text_pos = layout
                    .align_size_within_rect(
                        shortcut_galley.size(),
                        Rect::from_min_max(
                            egui::pos2(rect.min.x + button_padding_left, content_rect_y_min),
                            egui::pos2(rect.max.x - button_padding_right, content_rect_y_max),
                        ),
                    )
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

/// Blend an overlay color on top of a base color with given opacity.
fn blend_overlay(base: Color32, overlay: Color32, opacity: f32) -> Color32 {
    let alpha = (opacity * 255.0) as u8;
    let overlay_with_alpha = Color32::from_rgba_unmultiplied(overlay.r(), overlay.g(), overlay.b(), alpha);
    // Simple alpha blending
    let inv_alpha = 255 - alpha;
    Color32::from_rgba_unmultiplied(
        ((base.r() as u16 * inv_alpha as u16 + overlay_with_alpha.r() as u16 * alpha as u16) / 255) as u8,
        ((base.g() as u16 * inv_alpha as u16 + overlay_with_alpha.g() as u16 * alpha as u16) / 255) as u8,
        ((base.b() as u16 * inv_alpha as u16 + overlay_with_alpha.b() as u16 * alpha as u16) / 255) as u8,
        base.a(),
    )
}
