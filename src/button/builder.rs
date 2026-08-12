//! Builder methods for MaterialButton

use super::{types::*, MaterialButton};
use crate::{get_global_color, material_symbol::material_symbol_text};
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Shadow, Stroke},
    Image, Sense, TextStyle, TextWrapMode, Vec2, WidgetText,
};

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
            leading_svg: None,
            trailing_svg: None,
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

    /// Override background fill color.
    ///
    /// Overrides variant-based M3 color roles (primary, surface, secondaryContainer).
    /// Note: This will override hover/press state layer effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = Some(fill.into());
        self.frame = Some(true);
        self
    }

    /// Override button stroke.
    ///
    /// Overrides variant-based M3 outline color role.
    /// Note: This will override hover/press state effects on the border.
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

    /// Add a leading SVG icon to the button (rendered before the text).
    ///
    /// Takes SVG data as a string. This takes precedence over `leading_icon`.
    #[inline]
    pub fn leading_svg(mut self, svg_data: impl Into<String>) -> Self {
        self.leading_svg = Some(svg_data.into());
        self
    }

    /// Add a trailing SVG icon to the button (rendered after the text).
    ///
    /// Takes SVG data as a string. This takes precedence over `trailing_icon`.
    #[inline]
    pub fn trailing_svg(mut self, svg_data: impl Into<String>) -> Self {
        self.trailing_svg = Some(svg_data.into());
        self
    }

    /// Override the text color for this button.
    ///
    /// Overrides variant-based M3 color roles:
    /// - Filled: onPrimary
    /// - Outlined/Text/Elevated: onSurface
    /// - FilledTonal: onSecondaryContainer
    ///
    /// Icon colors also follow this override.
    #[inline]
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }
}
