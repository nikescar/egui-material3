//! Builder methods for MaterialButton

use super::{types::*, MaterialButton};
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Shadow, Stroke},
    Image, Role, Sense, TextWrapMode, Vec2, WidgetText,
};

impl<'a> MaterialButton<'a> {
    /// Create a filled Material Design button with high emphasis
    ///
    /// Filled buttons have the most visual impact and should be used for
    /// the primary action in a set of buttons.
    ///
    /// ## Material Design Spec
    /// - Background: `primary`; label and icons: `onPrimary`
    /// - Elevation: level 0, level 1 on hover
    /// - Shape: fully rounded (see [`Self::shape`])
    pub fn filled(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Filled, text)
    }

    /// Create an outlined Material Design button with medium emphasis
    ///
    /// Outlined buttons are medium-emphasis buttons. They contain actions
    /// that are important but aren't the primary action in an app.
    ///
    /// ## Material Design Spec
    /// - Background: transparent; label and icons: `onSurfaceVariant`
    /// - Outline: `outlineVariant`, 1 dp (2 dp at Large, 3 dp at ExtraLarge)
    pub fn outlined(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Outlined, text)
    }

    /// Create a text Material Design button with low emphasis
    ///
    /// Text buttons are used for the least important actions in a UI.
    /// They're often used for secondary actions.
    ///
    /// ## Material Design Spec
    /// - Background: transparent; label and icons: `primary`
    /// - No outline or elevation
    pub fn text(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Text, text)
    }

    /// Create an elevated Material Design button with medium emphasis
    ///
    /// Elevated buttons are essentially filled buttons with a shadow.
    /// Use them to add separation between button and background.
    ///
    /// ## Material Design Spec
    /// - Background: `surfaceContainerLow`; label and icons: `primary`
    /// - Elevation: level 1, level 2 on hover
    pub fn elevated(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Elevated, text)
    }

    /// Create a filled tonal Material Design button with medium emphasis
    ///
    /// Filled tonal buttons are used to convey a secondary action that is
    /// still important, but not the primary action.
    ///
    /// ## Material Design Spec
    /// - Background: `secondaryContainer`; label and icons: `onSecondaryContainer`
    /// - Elevation: level 0, level 1 on hover
    pub fn filled_tonal(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::FilledTonal, text)
    }

    /// Internal constructor that creates a button with the specified variant and text
    fn new_with_variant(variant: MaterialButtonVariant, text: impl Into<WidgetText>) -> Self {
        Self::opt_image_and_text_with_variant(variant, None, Some(text.into()))
    }

    /// A filled button; the same as [`Self::filled`].
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self::filled(text)
    }

    /// Creates a button with an image. The size of the image as displayed is defined by the provided size.
    pub fn image(image: impl Into<Image<'a>>) -> Self {
        Self::opt_image_and_text(Some(image.into()), None)
    }

    /// Creates a button with an image to the left of the text. The size of the image as displayed is defined by the provided size.
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
            size: None,
            shape: ButtonShape::Round,
            fill: None,
            stroke: None,
            sense: Sense::click(),
            frame: None,
            min_size: Vec2::ZERO,
            width: None,
            padding: None,
            corner_radius: None,
            segment: None,
            toggle: ToggleState::None,
            role: Role::Button,
            image_tint_follows_text_color: false,
            elevation: None,
            disabled: false,
            leading_icon: None,
            trailing_icon: None,
            leading_svg: None,
            trailing_svg: None,
            text_color: None,
            label: None,
        }
    }

    /// Set the size (XS to XL). Height, padding, label style, icon size,
    /// outline width and corner radii all follow the size's tokens.
    ///
    /// Without a size the button uses the baseline metrics: 40 dp high with
    /// 24 dp padding (16 dp beside an icon).
    #[inline]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the resting shape: [`ButtonShape::Round`] (default) or
    /// [`ButtonShape::Square`]. Either one morphs to the size's pressed
    /// shape while held, and toggle buttons morph to the selected shape.
    #[inline]
    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    /// Make this a toggle button bound to `selected`: clicks flip it, the
    /// button shows the style's selected or unselected colours and shape,
    /// and screen readers hear a toggle that is on or off.
    #[inline]
    pub fn toggle(mut self, selected: &'a mut bool) -> Self {
        self.toggle = ToggleState::Bound(selected);
        self
    }

    /// The accessible label, for a button whose only content is an icon or
    /// an image. Defaults to the button text.
    #[inline]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
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
    /// Replaces the style's container colour token. The state layer is
    /// still composited over it.
    #[inline]
    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = Some(fill.into());
        self.frame = Some(true);
        self
    }

    /// Override button stroke.
    ///
    /// Replaces the style's outline (outlined buttons) or adds one.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self.frame = Some(true);
        self
    }

    /// Make this a compact button: shorthand for
    /// `.size(ButtonSize::ExtraSmall)` (32 dp high, 12 dp padding).
    #[inline]
    pub fn small(self) -> Self {
        self.size(ButtonSize::ExtraSmall)
    }

    /// Turn off the frame (container and outline).
    #[inline]
    #[deprecated = "use `MaterialButton::text` for a button without a container"]
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = Some(frame);
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    #[inline]
    #[deprecated = "M3 buttons are clicked; allocate a custom widget for drag handles"]
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

    /// Set the rounding of the button. This turns off the shape tokens and
    /// the pressed and selected shape morph.
    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    /// Old name of [`Self::corner_radius`].
    #[inline]
    #[deprecated = "Renamed to `corner_radius`"]
    pub fn rounding(self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius(corner_radius)
    }

    /// If true, the tint of the image is multiplied by the widget text color.
    ///
    /// This makes sense for images that are white, that should have the same color as the text color.
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
    #[deprecated = "M3 buttons have no shortcut text; use a menu item for that"]
    pub fn shortcut_text(mut self, shortcut_text: impl Into<WidgetText>) -> Self {
        self.shortcut_text = shortcut_text.into();
        self
    }

    /// Show this button as a toggle button in the `selected` state, without
    /// binding it: clicks do not change it. Use [`Self::toggle`] to have
    /// clicks flip a flag.
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.toggle = ToggleState::Display(selected);
        self
    }

    /// Enable or disable the button. A disabled button cannot be clicked
    /// or focused.
    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.disabled = !enabled;
        self
    }

    /// Replace the token elevation with a custom shadow.
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
    /// Replaces the style's label colour token (for example `onPrimary` on
    /// a filled button). The disabled colour still applies.
    ///
    /// Icon colors also follow this override.
    #[inline]
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Force the button to exactly `width` points (button groups use it to
    /// grow a pressed item and shrink its neighbours).
    #[inline]
    pub(crate) fn exact_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Override the token side padding (split buttons have their own).
    #[inline]
    pub(crate) fn padding(mut self, leading: f32, trailing: f32) -> Self {
        self.padding = Some((leading, trailing));
        self
    }

    /// Round the button as a segment of a connected row.
    #[inline]
    pub(crate) fn segment(mut self, segment: SegmentCorners) -> Self {
        self.segment = Some(segment);
        self
    }

    /// The AccessKit role (a single-select group's items are radio buttons).
    #[inline]
    pub(crate) fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }
}
