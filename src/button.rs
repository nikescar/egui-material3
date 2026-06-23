//! Nala / Leo button components
//!
//! Implements the Nala button kinds from Leo (`filled`, `outline`, `plain`, `plain-faint`, `hero`)
//! with legacy Material Design variant names kept for compatibility.

use crate::{
    get_global_color,
    material_symbol::material_symbol_text,
    theme::{get_global_theme, ThemeMode},
};
use egui::{
    ecolor::Color32,
    emath::NumExt,
    epaint::{CornerRadius, Galley, Shadow, Stroke},
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
/// Nala button kind (Leo component API)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MaterialButtonVariant {
    /// Filled — primary background, on-primary text
    #[default]
    Filled,
    /// Outlined — interactive text, interactive divider border
    Outlined,
    /// Plain text — interactive text, subtle hover fill
    Text,
    /// Plain faint — low-emphasis icon/text button
    PlainFaint,
    /// Hero — Brave CTA gradient-style button
    Hero,
    /// Elevated — legacy M3 elevated surface button
    Elevated,
    /// Filled tonal — legacy toned container button
    FilledTonal,
}

/// Nala button size (Leo component API)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MaterialButtonSize {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Jumbo,
}

pub type ButtonVariant = MaterialButtonVariant;
pub type ButtonSize = MaterialButtonSize;

const NALA_PILL_RADIUS: CornerRadius = CornerRadius::same(255);

struct NalaButtonLayout {
    min_height: f32,
    padding_x: f32,
    padding_y: f32,
    font_size: f32,
    icon_size: f32,
    icon_gap: f32,
}

impl MaterialButtonSize {
    fn layout(self) -> NalaButtonLayout {
        match self {
            MaterialButtonSize::Tiny => NalaButtonLayout {
                min_height: 28.0,
                padding_x: 8.0,
                padding_y: 4.0,
                font_size: 12.0,
                icon_size: 16.0,
                icon_gap: 4.0,
            },
            MaterialButtonSize::Small => NalaButtonLayout {
                min_height: 36.0,
                padding_x: 12.0,
                padding_y: 8.0,
                font_size: 12.0,
                icon_size: 18.0,
                icon_gap: 4.0,
            },
            MaterialButtonSize::Medium => NalaButtonLayout {
                min_height: 44.0,
                padding_x: 12.0,
                padding_y: 12.0,
                font_size: 14.0,
                icon_size: 20.0,
                icon_gap: 4.0,
            },
            MaterialButtonSize::Large => NalaButtonLayout {
                min_height: 52.0,
                padding_x: 16.0,
                padding_y: 12.0,
                font_size: 16.0,
                icon_size: 24.0,
                icon_gap: 4.0,
            },
            MaterialButtonSize::Jumbo => NalaButtonLayout {
                min_height: 60.0,
                padding_x: 16.0,
                padding_y: 16.0,
                font_size: 18.0,
                icon_size: 28.0,
                icon_gap: 8.0,
            },
        }
    }
}

fn is_dark_theme() -> bool {
    get_global_theme()
        .lock()
        .map(|theme| matches!(theme.theme_mode, ThemeMode::Dark))
        .unwrap_or(false)
}

fn nala_hover_shadow() -> Shadow {
    Shadow {
        offset: [0, 1],
        blur: 3,
        spread: 0,
        color: Color32::from_black_alpha(40),
    }
}

fn with_opacity(color: Color32, opacity: f32) -> Color32 {
    color.linear_multiply(opacity)
}

/// Vertical position for a galley so glyph bounds are centered in `rect`.
///
/// egui galleys include font line-box space above the visible glyphs; centering
/// on `galley.size()` makes labels look shifted down inside buttons.
fn center_galley_y(galley: &Galley, rect: Rect) -> f32 {
    rect.center().y - galley.mesh_bounds.center().y
}

fn galley_visual_height(galley: &Galley) -> f32 {
    galley.mesh_bounds.height()
}

/// Leo `color-mix(in srgb, primary N%, transparent)` for hover fills on plain-faint.
fn primary_tint(primary: Color32, amount: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        primary.r(),
        primary.g(),
        primary.b(),
        (amount * 255.0).round() as u8,
    )
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
    /// Whether to render as a smaller compact button (deprecated — prefer [`Self::size`])
    small: bool,
    /// Nala button size
    size: MaterialButtonSize,
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
    /// Leading icon SVG data (rendered as texture, takes precedence over leading_icon)
    leading_svg: Option<String>,
    /// Trailing icon SVG data (rendered as texture, takes precedence over trailing_icon)
    trailing_svg: Option<String>,
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

    /// Plain text button (Nala `plain` kind — alias of [`Self::text`])
    pub fn plain(text: impl Into<WidgetText>) -> Self {
        Self::text(text)
    }

    /// Plain faint button (Nala `plain-faint` kind)
    pub fn plain_faint(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::PlainFaint, text)
    }

    /// Hero CTA button (Nala `hero` kind)
    pub fn hero(text: impl Into<WidgetText>) -> Self {
        Self::new_with_variant(MaterialButtonVariant::Hero, text)
    }

    /// Outlined button (Nala `outline` kind — alias of [`Self::outlined`])
    pub fn outline(text: impl Into<WidgetText>) -> Self {
        Self::outlined(text)
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
            size: MaterialButtonSize::Medium,
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
        self.size = MaterialButtonSize::Small;
        self.small = true;
        self
    }

    /// Set the Nala button size.
    #[inline]
    pub fn size(mut self, size: MaterialButtonSize) -> Self {
        self.size = size;
        self.small = matches!(size, MaterialButtonSize::Small | MaterialButtonSize::Tiny);
        self
    }

    /// Set the button variant.
    #[inline]
    pub fn variant(mut self, variant: MaterialButtonVariant) -> Self {
        self.variant = variant;
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
            small: _legacy_small,
            size,
            frame,
            min_size,
            corner_radius,
            selected,
            image_tint_follows_text_color,
            elevation,
            disabled,
            leading_icon,
            trailing_icon,
            leading_svg,
            trailing_svg,
            text_color: custom_text_color,
        } = self;

        let layout = if _legacy_small && size == MaterialButtonSize::Medium {
            MaterialButtonSize::Small.layout()
        } else {
            size.layout()
        };

        let dark = is_dark_theme();
        let primary = get_global_color("primary");
        let on_primary = get_global_color("onPrimary");
        let _secondary_container = get_global_color("secondaryContainer");
        let on_secondary_container = get_global_color("onSecondaryContainer");
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");

        // Leo semantic colors
        let interactive = primary;
        let divider_interactive = if dark {
            Color32::from_rgb(118, 134, 236) // primary.60 dark
        } else {
            Color32::from_rgb(188, 198, 243) // primary.80 light
        };
        let hover_surface = if dark {
            Color32::from_rgb(70, 70, 73) // neutral.30 dark
        } else {
            Color32::from_rgb(228, 228, 229) // neutral.20 light
        };
        let disabled_bg = if dark {
            Color32::from_rgba_unmultiplied(235, 238, 240, 51) // rgba(235,238,240,0.2)
        } else {
            Color32::from_rgba_unmultiplied(70, 70, 74, 51) // rgba(70,70,74,0.2)
        };
        let disabled_text = if dark {
            Color32::from_rgba_unmultiplied(235, 238, 240, 128)
        } else {
            Color32::from_rgba_unmultiplied(33, 39, 42, 128)
        };
        let hero_fill = Color32::from_rgb(255, 64, 0);

        let (default_fill, default_stroke, default_corner_radius, hover_shadow) = match variant {
            MaterialButtonVariant::Filled => (
                Some(primary),
                Some(Stroke::NONE),
                NALA_PILL_RADIUS,
                Some(nala_hover_shadow()),
            ),
            MaterialButtonVariant::Outlined => (
                Some(Color32::TRANSPARENT),
                Some(Stroke::new(1.0, divider_interactive)),
                NALA_PILL_RADIUS,
                Some(nala_hover_shadow()),
            ),
            MaterialButtonVariant::Text => (
                Some(blend_overlay(surface, primary, 0.05)),
                Some(Stroke::NONE),
                NALA_PILL_RADIUS,
                None,
            ),
            MaterialButtonVariant::PlainFaint => (
                Some(Color32::TRANSPARENT),
                Some(Stroke::NONE),
                NALA_PILL_RADIUS,
                None,
            ),
            MaterialButtonVariant::Hero => (
                Some(hero_fill),
                Some(Stroke::NONE),
                NALA_PILL_RADIUS,
                None,
            ),
            MaterialButtonVariant::Elevated => (
                Some(surface),
                Some(Stroke::NONE),
                NALA_PILL_RADIUS,
                Some(nala_hover_shadow()),
            ),
            MaterialButtonVariant::FilledTonal => (
                Some(blend_overlay(surface, primary, 0.05)),
                Some(Stroke::NONE),
                NALA_PILL_RADIUS,
                None,
            ),
        };

        let frame = frame.unwrap_or(!matches!(variant, MaterialButtonVariant::PlainFaint));

        // Load SVG textures early if provided (takes precedence over font icons)
        let leading_svg_texture = leading_svg.and_then(|svg_data| {
            crate::image_utils::create_texture_from_svg(ui.ctx(), &svg_data, &format!("btn_lead_{}", svg_data.len())).ok()
        });
        let trailing_svg_texture = trailing_svg.and_then(|svg_data| {
            crate::image_utils::create_texture_from_svg(ui.ctx(), &svg_data, &format!("btn_trail_{}", svg_data.len())).ok()
        });

        let button_text_style = match size {
            MaterialButtonSize::Tiny | MaterialButtonSize::Small => {
                TextStyle::Name("ButtonSmall".into())
            }
            MaterialButtonSize::Large => TextStyle::Name("ButtonLarge".into()),
            MaterialButtonSize::Jumbo => TextStyle::Name("ButtonJumbo".into()),
            MaterialButtonSize::Medium => TextStyle::Name("Button".into()),
        };

        // Build icon galleys early (only if no SVG provided)
        let leading_icon_galley = if leading_svg_texture.is_none() {
            leading_icon.map(|name| {
                let icon_str: WidgetText = material_symbol_text(&name).into();
                icon_str.into_galley(
                    ui,
                    Some(TextWrapMode::Extend),
                    f32::INFINITY,
                    button_text_style.clone(),
                )
            })
        } else {
            None
        };
        let trailing_icon_galley = if trailing_svg_texture.is_none() {
            trailing_icon.map(|name| {
                let icon_str: WidgetText = material_symbol_text(&name).into();
                icon_str.into_galley(
                    ui,
                    Some(TextWrapMode::Extend),
                    f32::INFINITY,
                    button_text_style.clone(),
                )
            })
        } else {
            None
        };

        let button_padding_left = layout.padding_x;
        let button_padding_right = layout.padding_x;
        let button_padding_y = layout.padding_y;
        let min_button_height = layout.min_height;
        let icon_spacing = layout.icon_gap;
        let svg_icon_size = layout.icon_size;

        // Nala text colors per kind
        let resolved_text_color = if disabled {
            match variant {
                MaterialButtonVariant::Text => on_surface,
                MaterialButtonVariant::PlainFaint => disabled_text,
                _ => disabled_text,
            }
        } else if let Some(custom) = custom_text_color {
            custom
        } else {
            match variant {
                MaterialButtonVariant::Filled | MaterialButtonVariant::Hero => on_primary,
                MaterialButtonVariant::Outlined | MaterialButtonVariant::Text => interactive,
                MaterialButtonVariant::PlainFaint => on_surface,
                MaterialButtonVariant::Elevated => on_surface,
                MaterialButtonVariant::FilledTonal => on_secondary_container,
            }
        };

        let space_available_for_image = if let Some(_text) = &text {
            Vec2::splat(layout.font_size)
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
        if let Some(galley) = &leading_icon_galley {
            text_wrap_width -= galley.size().x + icon_spacing;
        }
        if leading_svg_texture.is_some() {
            text_wrap_width -= svg_icon_size + icon_spacing;
        }
        if let Some(galley) = &trailing_icon_galley {
            text_wrap_width -= galley.size().x + icon_spacing;
        }
        if trailing_svg_texture.is_some() {
            text_wrap_width -= svg_icon_size + icon_spacing;
        }

        // Note: we don't wrap the shortcut text
        let shortcut_galley = (!shortcut_text.is_empty()).then(|| {
            shortcut_text.into_galley(
                ui,
                Some(TextWrapMode::Extend),
                f32::INFINITY,
                button_text_style.clone(),
            )
        });

        if let Some(shortcut_galley) = &shortcut_galley {
            text_wrap_width -= gap_before_shortcut_text + shortcut_galley.size().x;
        }

        let galley = text.map(|text| {
            text.into_galley(
                ui,
                wrap_mode,
                text_wrap_width,
                button_text_style.clone(),
            )
        });

        let mut desired_size = Vec2::ZERO;

        // Leading icon (font or SVG)
        if let Some(lg) = &leading_icon_galley {
            desired_size.x += lg.size().x;
            desired_size.y = desired_size.y.max(galley_visual_height(lg));
        }
        if leading_svg_texture.is_some() {
            desired_size.x += svg_icon_size;
            desired_size.y = desired_size.y.max(svg_icon_size);
        }

        // Image
        if image.is_some() {
            if leading_icon_galley.is_some() || leading_svg_texture.is_some() {
                desired_size.x += icon_spacing;
            }
            desired_size.x += image_size.x;
            desired_size.y = desired_size.y.max(image_size.y);
        }

        // Gap between leading content and text
        if (leading_icon_galley.is_some() || leading_svg_texture.is_some() || image.is_some()) && galley.is_some() {
            desired_size.x += icon_spacing;
        }

        if let Some(galley) = &galley {
            desired_size.x += galley.size().x;
            desired_size.y = desired_size.y.max(galley_visual_height(galley));
        }

        // Trailing icon (font or SVG)
        if let Some(tg) = &trailing_icon_galley {
            if galley.is_some() || image.is_some() || leading_icon_galley.is_some() || leading_svg_texture.is_some() {
                desired_size.x += icon_spacing;
            }
            desired_size.x += tg.size().x;
            desired_size.y = desired_size.y.max(galley_visual_height(tg));
        }
        if trailing_svg_texture.is_some() {
            if galley.is_some() || image.is_some() || leading_icon_galley.is_some() || leading_svg_texture.is_some() {
                desired_size.x += icon_spacing;
            }
            desired_size.x += svg_icon_size;
            desired_size.y = desired_size.y.max(svg_icon_size);
        }

        if let Some(shortcut_galley) = &shortcut_galley {
            desired_size.x += gap_before_shortcut_text + shortcut_galley.size().x;
            desired_size.y = desired_size.y.max(galley_visual_height(shortcut_galley));
        }

        desired_size.x += button_padding_left + button_padding_right;
        desired_size.y += 2.0 * button_padding_y;
        desired_size.y = desired_size.y.at_least(min_button_height);
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
            let mut text_paint_color = resolved_text_color;

            if disabled {
                frame_fill = match variant {
                    MaterialButtonVariant::Filled
                    | MaterialButtonVariant::Hero
                    | MaterialButtonVariant::Elevated
                    | MaterialButtonVariant::FilledTonal => disabled_bg,
                    _ => frame_fill,
                };
                frame_stroke.color = disabled_bg;
                frame_stroke.width = if matches!(variant, MaterialButtonVariant::Outlined) {
                    1.0
                } else {
                    0.0
                };
            } else if response.hovered() {
                match variant {
                    MaterialButtonVariant::Filled | MaterialButtonVariant::Hero => {}
                    MaterialButtonVariant::Outlined => {
                        frame_fill = hover_surface;
                        frame_stroke.color = if dark {
                            Color32::from_rgb(91, 103, 232)
                        } else {
                            Color32::from_rgb(21, 25, 45)
                        };
                    }
                    MaterialButtonVariant::Text => {
                        frame_fill = blend_overlay(surface, primary, 0.10);
                    }
                    MaterialButtonVariant::PlainFaint => {
                        frame_fill = primary_tint(primary, if dark { 0.10 } else { 0.05 });
                    }
                    MaterialButtonVariant::Elevated => {
                        frame_fill = hover_surface;
                    }
                    MaterialButtonVariant::FilledTonal => {
                        frame_fill = blend_overlay(frame_fill, primary, 0.10);
                    }
                }
            }

            if !disabled && response.is_pointer_button_down_on() {
                text_paint_color = with_opacity(text_paint_color, 0.75);
                frame_fill = with_opacity(frame_fill, 0.75);
                frame_stroke.color = with_opacity(frame_stroke.color, 0.75);
            }

            if !disabled && response.hovered() {
                if let Some(shadow) = hover_shadow {
                    let shadow = Shadow {
                        offset: [shadow.offset[0], shadow.offset[1] + 1],
                        blur: shadow.blur + 2,
                        spread: shadow.spread,
                        color: shadow.color,
                    };
                    let shadow_offset =
                        Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32);
                    let shadow_rect = rect.expand2(frame_expansion).translate(shadow_offset);
                    ui.painter().rect_filled(shadow_rect, frame_cr, shadow.color);
                }
            } else if let Some(shadow) = &elevation {
                let shadow_offset = Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32);
                let shadow_rect = rect.expand2(frame_expansion).translate(shadow_offset);
                ui.painter().rect_filled(shadow_rect, frame_cr, shadow.color);
            }

            ui.painter().rect(
                rect.expand2(frame_expansion),
                frame_cr,
                frame_fill,
                frame_stroke,
                egui::epaint::StrokeKind::Outside,
            );

            let mut cursor_x = rect.min.x + button_padding_left;
            let content_rect = Rect::from_min_max(
                egui::pos2(rect.min.x + button_padding_left, rect.min.y + button_padding_y),
                egui::pos2(rect.max.x - button_padding_right, rect.max.y - button_padding_y),
            );

            // Draw leading icon (font icon)
            if let Some(leading_galley) = &leading_icon_galley {
                let icon_y = center_galley_y(leading_galley, content_rect);
                let icon_pos = egui::pos2(cursor_x, icon_y);
                ui.painter()
                    .galley(icon_pos, leading_galley.clone(), text_paint_color);
                cursor_x += leading_galley.size().x + icon_spacing;
            }

            // Draw leading icon (SVG texture)
            if let Some(texture) = &leading_svg_texture {
                let icon_y = content_rect.center().y - svg_icon_size / 2.0;
                let icon_rect = Rect::from_min_size(
                    egui::pos2(cursor_x, icon_y),
                    Vec2::splat(svg_icon_size),
                );
                ui.painter().image(
                    texture.id(),
                    icon_rect,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE, // Use WHITE to preserve original SVG colors (e.g., emoji)
                );
                cursor_x += svg_icon_size;
                // Add spacing only if there's content after the icon
                if image.is_some() || galley.is_some() || trailing_icon_galley.is_some() || trailing_svg_texture.is_some() || shortcut_galley.is_some() {
                    cursor_x += icon_spacing;
                }
            }

            // Draw image
            if let Some(image) = &image {
                let mut image_pos = ui
                    .layout()
                    .align_size_within_rect(
                        image_size,
                        content_rect,
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
            let has_text = galley.is_some();
            if let Some(galley) = galley {
                let mut text_pos = egui::pos2(cursor_x, center_galley_y(&galley, content_rect));
                // Center text if no leading/trailing elements
                if leading_icon_galley.is_none()
                    && leading_svg_texture.is_none()
                    && image.is_none()
                    && trailing_icon_galley.is_none()
                    && trailing_svg_texture.is_none()
                    && shortcut_galley.is_none()
                {
                    text_pos = ui
                        .layout()
                        .align_size_within_rect(galley.size(), content_rect)
                        .min;
                    text_pos.y = center_galley_y(&galley, content_rect);
                }

                cursor_x = text_pos.x + galley.size().x;
                ui.painter().galley(text_pos, galley, text_paint_color);
            }

            // Draw trailing icon (font icon)
            if let Some(trailing_galley) = &trailing_icon_galley {
                cursor_x += icon_spacing;
                let icon_y = center_galley_y(trailing_galley, content_rect);
                let icon_pos = egui::pos2(cursor_x, icon_y);
                ui.painter()
                    .galley(icon_pos, trailing_galley.clone(), text_paint_color);
            }

            // Draw trailing icon (SVG texture)
            if let Some(texture) = &trailing_svg_texture {
                // Add spacing before the icon if there's content before it
                if has_text || image.is_some() || leading_icon_galley.is_some() || leading_svg_texture.is_some() {
                    cursor_x += icon_spacing;
                }
                let icon_y = content_rect.center().y - svg_icon_size / 2.0;
                let icon_rect = Rect::from_min_size(
                    egui::pos2(cursor_x, icon_y),
                    Vec2::splat(svg_icon_size),
                );
                ui.painter().image(
                    texture.id(),
                    icon_rect,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE, // Use WHITE to preserve original SVG colors (e.g., emoji)
                );
            }

            // Draw shortcut text
            if let Some(shortcut_galley) = shortcut_galley {
                let layout = if ui.layout().is_horizontal() {
                    ui.layout().with_main_align(Align::Max)
                } else {
                    ui.layout().with_cross_align(Align::Max)
                };
                let mut shortcut_text_pos = layout
                    .align_size_within_rect(shortcut_galley.size(), content_rect)
                    .min;
                shortcut_text_pos.y = center_galley_y(&shortcut_galley, content_rect);
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
