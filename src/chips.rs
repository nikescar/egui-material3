// ============================================================================
// Module:       chips
// Description:  Material 3 assist, filter, input and suggestion chips, and chip sets.
//
// Dependencies: egui; crate::{a11y, icon, image_utils, paint, tokens, MaterialTheme}
// ============================================================================

//! Material Design 3 chips: assist, filter, input and suggestion.
//!
//! Every value comes from the Jetpack Compose material3 token files
//! (`AssistChipTokens.kt`, `FilterChipTokens.kt`, `InputChipTokens.kt`,
//! `SuggestionChipTokens.kt`) and from `Chip.kt` for the layout values
//! Compose does not tokenise (content padding, spacing). The baseline
//! token set is implemented; [`MaterialChip::expressive`] switches to the
//! M3 Expressive shapes from `ChipsTokens.kt`.
//!
//! # Geometry
//!
//! - Height 32 (`ContainerHeight`), corner 8 (`ContainerShape` =
//!   `CornerSmall`).
//! - Leading and trailing icons 18 (`IconSize`, `LeadingIconSize`,
//!   `TrailingIconSize`); input-chip avatar 24, fully round (`AvatarSize`,
//!   `AvatarShape` = `CornerFull`).
//! - 8 between the icon and the label (`HorizontalSpacing`). The label sits
//!   16 from an edge with no icon (12 on input chips) and the icon 8 from
//!   the edge (4 for an input-chip avatar).
//!
//! # Colours
//!
//! | Chip | Container | Label | Leading icon |
//! |---|---|---|---|
//! | Assist | transparent, `outlineVariant` outline | `onSurface` | `primary` |
//! | Suggestion | transparent, `outlineVariant` outline | `onSurfaceVariant` | `primary` |
//! | Filter | transparent, `outlineVariant` outline | `onSurfaceVariant` | `primary` |
//! | Filter, selected | `secondaryContainer` | `onSecondaryContainer` | `onSecondaryContainer` |
//! | Input | transparent, `outlineVariant` outline | `onSurfaceVariant` | `onSurfaceVariant` |
//! | Input, selected | `secondaryContainer` | `onSecondaryContainer` | `primary` |
//!
//! Elevated chips fill with `surfaceContainerLow` and sit at Level 1,
//! rising to Level 2 on hover. Disabled chips keep a transparent container
//! with an `onSurface` outline at 12 % (elevated and selected ones fill
//! with `onSurface` at 12 %), and draw their content in `onSurface` at 38 %.
//!
//! # Accessibility
//!
//! Filter chips, and input chips bound with [`MaterialChip::selected`], are
//! checkboxes with a toggled state (Compose's `Role.Checkbox`); the others
//! are buttons. A removable chip's trailing icon is a separate button
//! labelled "Remove …", and Delete or Backspace on the focused chip removes
//! it too.

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use egui::epaint::RectShape;
use egui::load::SizedTexture;
use egui::text::LayoutJob;
use egui::{
    Color32, CornerRadius, InnerResponse, Key, Pos2, Rect, Response, Role, Sense, Shape, Stroke,
    StrokeKind, TextureHandle, Ui, Vec2, Widget,
};

use crate::icon::{paint_icon, Icon, SymbolOptions};
use crate::image_utils;
use crate::paint::{paint_elevation, paint_focus_ring_if_focused, paint_state_layer};
use crate::tokens::{
    animate_value_with_spring, ColorRole, CornerSize, Elevation, InteractionState, SpringSpeed,
    TypeRole,
};
use crate::MaterialTheme;

/// `ContainerHeight` in every chip token file: 32.
const CONTAINER_HEIGHT: f32 = 32.0;
/// The height of [`MaterialChip::small`] chips. Not a Material 3 size: a
/// compact variant kept for dense layouts.
const SMALL_HEIGHT: f32 = 24.0;
/// `AssistChipTokens.IconSize`, `FilterChipTokens.IconSize`,
/// `InputChipTokens.LeadingIconSize` / `TrailingIconSize`,
/// `SuggestionChipTokens.LeadingIconSize`: 18.
const ICON_SIZE: f32 = 18.0;
/// `InputChipTokens.AvatarSize`: 24.
const AVATAR_SIZE: f32 = 24.0;
/// `AssistChipDefaults.ContentPadding` (and the filter and suggestion
/// ones): horizontal 8.
const CONTENT_PADDING: f32 = 8.0;
/// `inputChipPadding` in `Chip.kt`: 4 at the start next to an avatar or
/// with no leading icon, and at the end with no trailing icon.
const INPUT_COMPACT_PADDING: f32 = 4.0;
/// `*ChipDefaults.HorizontalSpacing`: 8 between icon and label.
const HORIZONTAL_SPACING: f32 = 8.0;
/// `FlatOutlineWidth` / `FlatUnselectedOutlineWidth` /
/// `UnselectedOutlineWidth`: 1. Selected chips have none
/// (`FlatSelectedOutlineWidth` 0).
const OUTLINE_WIDTH: f32 = 1.0;
/// `ElevatedDisabledContainerOpacity`, `FlatDisabledSelectedContainerOpacity`,
/// `InputChipTokens.DisabledSelectedContainerOpacity`: 0.12.
const DISABLED_CONTAINER_OPACITY: f32 = 0.12;
/// `FlatDisabledOutlineOpacity`, `FlatDisabledUnselectedOutlineOpacity`,
/// `DisabledUnselectedOutlineOpacity`: 0.12.
const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
/// `DisabledLabelTextOpacity`, `DisabledIconOpacity`,
/// `DisabledAvatarOpacity`: 0.38.
const DISABLED_CONTENT_OPACITY: f32 = 0.38;
/// Space between chips in a [`chip_set`]. Compose leaves the `FlowRow`
/// spacing to the app; the M3 chip guidelines use 8.
pub const CHIP_SET_SPACING: f32 = 8.0;

/// Material Design chip variants following Material Design 3 specifications
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChipVariant {
    /// Assist chips help users take actions or get information about their current context
    Assist,
    /// Filter chips let users refine content by selecting or deselecting options
    Filter,
    /// Input chips represent discrete pieces of information entered by a user
    Input,
    /// Suggestion chips help users discover relevant, actionable content
    Suggestion,
}

impl ChipVariant {
    /// Every variant.
    pub const ALL: [ChipVariant; 4] = [
        ChipVariant::Assist,
        ChipVariant::Filter,
        ChipVariant::Input,
        ChipVariant::Suggestion,
    ];

    /// Whether the variant has a selected state (filter and input chips).
    pub const fn is_selectable(self) -> bool {
        match self {
            ChipVariant::Filter | ChipVariant::Input => true,
            ChipVariant::Assist | ChipVariant::Suggestion => false,
        }
    }
}

/// Types of icons that can be displayed in chips.
#[deprecated(
    note = "chips take any `egui_material3::icon::Icon` through `MaterialChip::leading_icon`, \
            or PNG bytes through `leading_icon_png`"
)]
#[derive(Clone)]
pub enum IconType {
    /// Material Design icon using icon name or unicode
    MaterialIcon(String),
    /// Custom SVG icon data
    SvgData(String),
    /// PNG image data as bytes
    PngBytes(Vec<u8>),
    /// Pre-loaded egui texture handle
    Texture(TextureHandle),
}

/// The leading image of a chip.
enum ChipImage {
    /// A symbol, SVG or texture, drawn by [`paint_icon`].
    Icon(Icon),
    /// PNG bytes, decoded into a texture on first use.
    Png(Arc<[u8]>),
}

/// Material Design 3 chip.
///
/// Chips are compact elements that represent an input, attribute, or action.
///
/// ```rust
/// # use egui_material3::*;
/// # egui::__run_test_ui(|ui| {
/// // Assist chip: a contextual action.
/// if ui.add(MaterialChip::assist("Settings").leading_icon("settings")).clicked() {
///     // Open settings
/// }
///
/// // Filter chip: toggles `photos` when clicked.
/// let mut photos = false;
/// ui.add(MaterialChip::filter("Photos", &mut photos));
///
/// // Input chip with a separate remove target.
/// if MaterialChip::input("john@example.com").removable(true).show(ui).removed() {
///     // Drop the address
/// }
///
/// // Suggestion chip.
/// ui.add(MaterialChip::suggestion("Try this feature"));
/// # });
/// ```
#[must_use = "add it with `ui.add(...)` or `.show(ui)`"]
pub struct MaterialChip<'a> {
    text: String,
    variant: ChipVariant,
    selected: Option<&'a mut bool>,
    enabled: bool,
    soft_disabled: bool,
    elevated: bool,
    removable: bool,
    leading: Option<ChipImage>,
    avatar: bool,
    is_small: bool,
    expressive: bool,
    action: Option<Box<dyn Fn() + 'a>>,
    on_remove: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialChip<'a> {
    /// Create a new chip with specified text and variant
    pub fn new(text: impl Into<String>, variant: ChipVariant) -> Self {
        Self {
            text: text.into(),
            variant,
            selected: None,
            enabled: true,
            soft_disabled: false,
            elevated: false,
            removable: false,
            leading: None,
            avatar: false,
            is_small: false,
            expressive: false,
            action: None,
            on_remove: None,
        }
    }

    /// An assist chip: a contextual action or piece of information.
    pub fn assist(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Assist)
    }

    /// A filter chip bound to `selected`: clicking it (or Space / Enter
    /// while focused) toggles the value. A selected filter chip shows a
    /// check mark in place of its leading icon.
    pub fn filter(text: impl Into<String>, selected: &'a mut bool) -> Self {
        Self::new(text, ChipVariant::Filter).selected(selected)
    }

    /// An input chip: a piece of information the user entered (a tag, a
    /// contact). Often [`removable`](Self::removable) and with an
    /// [`avatar`](Self::avatar).
    pub fn input(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Input)
    }

    /// A suggestion chip: a suggested reply or search.
    pub fn suggestion(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Suggestion)
    }

    /// Binds the selected state of a filter or input chip to `selected`;
    /// a click toggles it. Assist and suggestion chips have no selected
    /// state in Material 3 and ignore the binding.
    pub fn selected(mut self, selected: &'a mut bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// An elevated chip: a `surfaceContainerLow` container at Level 1
    /// (Level 2 on hover) instead of an outline.
    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    /// Enable or disable the chip. A disabled chip takes no clicks and no
    /// focus, and draws in the disabled colours.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        if enabled {
            self.soft_disabled = false;
        }
        self
    }

    /// Soft-disabled: drawn and reported as disabled and takes no clicks,
    /// but stays in the keyboard focus order so it can be discovered.
    pub fn soft_disabled(mut self, soft_disabled: bool) -> Self {
        self.soft_disabled = soft_disabled;
        if soft_disabled {
            self.enabled = false;
        }
        self
    }

    /// A compact 24-high chip. Not a Material 3 size.
    pub fn small(mut self) -> Self {
        self.is_small = true;
        self
    }

    /// Shows a trailing remove icon with its own hit target. Clicking it,
    /// or pressing Delete or Backspace while the chip has focus, reports
    /// [`ChipResponse::removed`] and calls [`on_remove`](Self::on_remove).
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    /// The leading icon: a Material Symbols name (`"event"`), a codepoint,
    /// or any [`Icon`].
    pub fn leading_icon(mut self, icon: impl Into<Icon>) -> Self {
        self.leading = Some(ChipImage::Icon(icon.into()));
        self
    }

    /// The leading icon from an SVG document. A monochrome SVG takes the
    /// chip's icon colour; a multi-colour one keeps its colours.
    pub fn leading_icon_svg(mut self, svg_data: impl Into<String>) -> Self {
        self.leading = Some(ChipImage::Icon(Icon::svg(svg_data.into().into_bytes())));
        self
    }

    /// The leading icon from PNG bytes, drawn in its own colours.
    pub fn leading_icon_png(mut self, png_bytes: Vec<u8>) -> Self {
        self.leading = Some(ChipImage::Png(png_bytes.into()));
        self
    }

    /// The leading icon from a loaded texture, drawn in its own colours.
    pub fn leading_icon_texture(mut self, texture: TextureHandle) -> Self {
        self.leading = Some(ChipImage::Icon(Icon::Texture(SizedTexture::from_handle(
            &texture,
        ))));
        self
    }

    /// Draws the leading image as an avatar: 24, fully round, in its own
    /// colours (`InputChipTokens.AvatarSize`, `AvatarShape`). Avatars
    /// belong on input chips, which then start 4 from the edge.
    pub fn avatar(mut self, avatar: bool) -> Self {
        self.avatar = avatar;
        self
    }

    /// M3 Expressive shapes (`ChipsTokens.kt`): corner 12 when unselected,
    /// fully round when selected, 8 while pressed, morphing with the fast
    /// spatial spring of the theme's motion scheme.
    pub fn expressive(mut self, expressive: bool) -> Self {
        self.expressive = expressive;
        self
    }

    /// Called when the chip is clicked (after a filter or input chip has
    /// toggled its selected state).
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }

    /// Called when a [`removable`](Self::removable) chip is removed.
    pub fn on_remove<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.on_remove = Some(Box::new(f));
        self
    }

    /// Adds the chip and returns its response together with the remove
    /// target's.
    pub fn show(self, ui: &mut Ui) -> ChipResponse {
        show_chip(self, ui)
    }
}

/// What [`MaterialChip::show`] returns.
#[derive(Clone, Debug)]
pub struct ChipResponse {
    /// The chip body: clicks, hover and focus.
    pub response: Response,
    /// The remove target of a removable chip.
    pub remove: Option<Response>,
    removed: bool,
}

impl ChipResponse {
    /// The chip was removed this frame: its remove icon was clicked, or
    /// Delete or Backspace was pressed while it had focus.
    pub fn removed(&self) -> bool {
        self.removed
    }

    /// The chip body was clicked.
    pub fn clicked(&self) -> bool {
        self.response.clicked()
    }

    /// A filter or input chip toggled its selected state this frame.
    pub fn changed(&self) -> bool {
        self.response.changed()
    }
}

/// Where the parts of a chip go, relative to its left edge.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ChipLayout {
    /// Total width.
    width: f32,
    /// Left edge of the leading icon or avatar.
    leading_x: f32,
    /// Left edge of the label.
    label_x: f32,
    /// Left edge of the trailing icon, when there is one.
    trailing_x: Option<f32>,
}

impl ChipLayout {
    /// The `Chip.kt` layout: content padding, then the leading slot (empty
    /// or not, it is followed by the 8 spacing), the label, the 8 spacing,
    /// the trailing slot, and the end padding.
    fn new(
        variant: ChipVariant,
        leading_size: Option<f32>,
        avatar: bool,
        trailing: bool,
        label_width: f32,
    ) -> Self {
        let (start, end) = match variant {
            ChipVariant::Input => (
                if avatar || leading_size.is_none() {
                    INPUT_COMPACT_PADDING
                } else {
                    CONTENT_PADDING
                },
                if trailing {
                    CONTENT_PADDING
                } else {
                    INPUT_COMPACT_PADDING
                },
            ),
            ChipVariant::Assist | ChipVariant::Filter | ChipVariant::Suggestion => {
                (CONTENT_PADDING, CONTENT_PADDING)
            }
        };
        let trailing_width = if trailing { ICON_SIZE } else { 0.0 };
        let label_x = start + leading_size.unwrap_or(0.0) + HORIZONTAL_SPACING;
        let width = label_x + label_width + HORIZONTAL_SPACING + trailing_width + end;
        Self {
            width,
            leading_x: start,
            label_x,
            trailing_x: trailing.then_some(width - end - trailing_width),
        }
    }
}

/// Resolved chip colours.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ChipColors {
    container: Color32,
    outline: Option<Color32>,
    label: Color32,
    leading: Color32,
    trailing: Color32,
}

/// The colours of a chip, from the `*ChipTokens.kt` files.
fn chip_colors(
    theme: &MaterialTheme,
    variant: ChipVariant,
    selected: bool,
    elevated: bool,
    enabled: bool,
) -> ChipColors {
    let c = |role| theme.color(role);
    let selected = selected && variant.is_selectable();
    if !enabled {
        let on_surface = c(ColorRole::OnSurface);
        let content = on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY);
        let container = if elevated || selected {
            on_surface.gamma_multiply(DISABLED_CONTAINER_OPACITY)
        } else {
            Color32::TRANSPARENT
        };
        let outline =
            (!elevated && !selected).then(|| on_surface.gamma_multiply(DISABLED_OUTLINE_OPACITY));
        return ChipColors {
            container,
            outline,
            label: content,
            leading: content,
            trailing: content,
        };
    }
    let unselected_container = if elevated {
        c(ColorRole::SurfaceContainerLow)
    } else {
        Color32::TRANSPARENT
    };
    let outline = (!elevated).then(|| c(ColorRole::OutlineVariant));
    match (variant, selected) {
        // AssistChipTokens: LabelTextColor OnSurface, IconColor Primary.
        (ChipVariant::Assist, _) => ChipColors {
            container: unselected_container,
            outline,
            label: c(ColorRole::OnSurface),
            leading: c(ColorRole::Primary),
            trailing: c(ColorRole::Primary),
        },
        // SuggestionChipTokens: LabelTextColor OnSurfaceVariant,
        // LeadingIconColor Primary.
        (ChipVariant::Suggestion, _) => ChipColors {
            container: unselected_container,
            outline,
            label: c(ColorRole::OnSurfaceVariant),
            leading: c(ColorRole::Primary),
            trailing: c(ColorRole::OnSurfaceVariant),
        },
        // FilterChipTokens.Unselected*: label and trailing OnSurfaceVariant,
        // leading Primary.
        (ChipVariant::Filter, false) => ChipColors {
            container: unselected_container,
            outline,
            label: c(ColorRole::OnSurfaceVariant),
            leading: c(ColorRole::Primary),
            trailing: c(ColorRole::OnSurfaceVariant),
        },
        // FilterChipTokens.Selected*: SecondaryContainer, everything
        // OnSecondaryContainer.
        (ChipVariant::Filter, true) => ChipColors {
            container: c(ColorRole::SecondaryContainer),
            outline: None,
            label: c(ColorRole::OnSecondaryContainer),
            leading: c(ColorRole::OnSecondaryContainer),
            trailing: c(ColorRole::OnSecondaryContainer),
        },
        // InputChipTokens.Unselected*: everything OnSurfaceVariant.
        (ChipVariant::Input, false) => ChipColors {
            container: unselected_container,
            outline,
            label: c(ColorRole::OnSurfaceVariant),
            leading: c(ColorRole::OnSurfaceVariant),
            trailing: c(ColorRole::OnSurfaceVariant),
        },
        // InputChipTokens.Selected*: SecondaryContainer, label and trailing
        // OnSecondaryContainer, leading Primary.
        (ChipVariant::Input, true) => ChipColors {
            container: c(ColorRole::SecondaryContainer),
            outline: None,
            label: c(ColorRole::OnSecondaryContainer),
            leading: c(ColorRole::Primary),
            trailing: c(ColorRole::OnSecondaryContainer),
        },
    }
}

/// The container elevation: `Elevated*ContainerElevation` (Level 1,
/// Level 2 on hover, Level 0 disabled), and for flat chips
/// `FlatSelectedHoverContainerElevation` (Level 1) on a hovered selected
/// filter chip, else `FlatContainerElevation` (Level 0).
fn chip_elevation(
    variant: ChipVariant,
    selected: bool,
    elevated: bool,
    enabled: bool,
    state: InteractionState,
) -> Elevation {
    let hovered = state.hovered && !state.pressed;
    if !enabled {
        Elevation::Level0
    } else if elevated {
        if hovered {
            Elevation::Level2
        } else {
            Elevation::Level1
        }
    } else if variant == ChipVariant::Filter && selected && hovered {
        Elevation::Level1
    } else {
        Elevation::Level0
    }
}

/// The corner radius to aim for: `ContainerShape` (`CornerSmall`) in the
/// baseline set; `ChipsTokens.UnselectedShape` (`CornerMedium`),
/// `SelectedShape` (`CornerFull`) and `PressedShape` (`CornerSmall`) in
/// the Expressive set.
fn target_radius(expressive: bool, selected: bool, pressed: bool, height: f32) -> f32 {
    let size = if !expressive || pressed {
        CornerSize::Small
    } else if selected {
        CornerSize::Full
    } else {
        CornerSize::Medium
    };
    size.value().unwrap_or(height / 2.0).min(height / 2.0)
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

/// Loads an image into a texture for drawing in its own colours: a PNG, an
/// SVG (for avatars, which are clipped round), or an existing texture.
fn image_texture(ctx: &egui::Context, image: &ChipImage) -> Option<SizedTexture> {
    match image {
        ChipImage::Png(bytes) => {
            let name = format!("chip_png_{:016x}", hash_bytes(bytes));
            image_utils::create_texture_from_png_bytes(ctx, bytes, &name)
                .ok()
                .map(|t| SizedTexture::from_handle(&t))
        }
        ChipImage::Icon(Icon::Svg(bytes)) => {
            let svg = std::str::from_utf8(bytes).ok()?;
            let name = format!("chip_svg_{:016x}", hash_bytes(bytes));
            image_utils::create_texture_from_svg(ctx, svg, &name)
                .ok()
                .map(|t| SizedTexture::from_handle(&t))
        }
        ChipImage::Icon(Icon::Texture(texture)) => Some(*texture),
        ChipImage::Icon(Icon::Symbol(_)) => None,
    }
}

/// Paints the leading image into `rect`.
fn paint_leading(
    ui: &Ui,
    rect: Rect,
    image: &ChipImage,
    avatar: bool,
    tint: Color32,
    enabled: bool,
) {
    let painter = ui.painter();
    // Images keep their own colours; disabled ones fade like the content.
    let image_tint = if enabled {
        Color32::WHITE
    } else {
        Color32::WHITE.gamma_multiply(DISABLED_CONTENT_OPACITY)
    };
    let avatar_texture = if avatar {
        image_texture(ui.ctx(), image)
    } else {
        None
    };
    if let Some(texture) = avatar_texture {
        let radius = CornerRadius::same((rect.height() / 2.0).round().clamp(0.0, 255.0) as u8);
        let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
        painter.add(RectShape::filled(rect, radius, image_tint).with_texture(texture.id, uv));
        return;
    }
    match image {
        ChipImage::Icon(icon @ (Icon::Symbol(_) | Icon::Svg(_))) => {
            paint_icon(painter, rect, icon, &SymbolOptions::default(), tint);
        }
        ChipImage::Icon(icon @ Icon::Texture(_)) => {
            paint_icon(painter, rect, icon, &SymbolOptions::default(), image_tint);
        }
        ChipImage::Png(_) => {
            if let Some(texture) = image_texture(ui.ctx(), image) {
                let icon = Icon::Texture(texture);
                paint_icon(painter, rect, &icon, &SymbolOptions::default(), image_tint);
            }
        }
    }
}

/// Maps a point of a 24×24 icon viewport into `rect`.
fn icon_point(rect: Rect, x: f32, y: f32) -> Pos2 {
    let scale = rect.width().min(rect.height()) / 24.0;
    let origin = rect.center() - Vec2::splat(12.0 * scale);
    Pos2::new(origin.x + x * scale, origin.y + y * scale)
}

/// The Material Symbols `check` glyph as a stroked path, so a selected
/// filter chip shows its check mark whether or not the symbols font is
/// installed.
fn paint_check(ui: &Ui, rect: Rect, color: Color32) {
    let width = 2.0 * rect.width() / 24.0;
    let points = vec![
        icon_point(rect, 4.1, 12.7),
        icon_point(rect, 9.0, 17.6),
        icon_point(rect, 20.3, 6.3),
    ];
    ui.painter()
        .add(Shape::line(points, Stroke::new(width, color)));
}

/// The Material Symbols `close` glyph as two strokes.
fn paint_close(ui: &Ui, rect: Rect, color: Color32) {
    let stroke = Stroke::new(2.0 * rect.width() / 24.0, color);
    let painter = ui.painter();
    painter.line_segment(
        [icon_point(rect, 5.7, 5.7), icon_point(rect, 18.3, 18.3)],
        stroke,
    );
    painter.line_segment(
        [icon_point(rect, 18.3, 5.7), icon_point(rect, 5.7, 18.3)],
        stroke,
    );
}

fn show_chip(chip: MaterialChip<'_>, ui: &mut Ui) -> ChipResponse {
    let MaterialChip {
        text,
        variant,
        mut selected,
        enabled,
        soft_disabled,
        elevated,
        removable,
        leading,
        avatar,
        is_small,
        expressive,
        action,
        on_remove,
    } = chip;
    let theme = MaterialTheme::of(ui.ctx());
    let selectable = variant.is_selectable() && selected.is_some();
    let was_selected = selectable && selected.as_deref().is_some_and(|s| *s);

    // The label: LabelLarge (`LabelTextFont`), unless the caller set an
    // explicit text style for this ui. The colour is filled in at paint
    // time, once the interaction state is known.
    let job = match ui.style().override_text_style.clone() {
        Some(style) => LayoutJob::simple_singleline(
            text.clone(),
            style.resolve(ui.style()),
            Color32::PLACEHOLDER,
        ),
        None => {
            TypeRole::LabelLarge
                .style()
                .layout_job(ui.ctx(), text.clone(), Color32::PLACEHOLDER)
        }
    };
    let galley = ui.painter().layout_job(job);

    let height = if is_small {
        SMALL_HEIGHT
    } else {
        CONTAINER_HEIGHT
    };
    // A selected filter chip shows a check mark, so its leading slot is
    // filled; the check mark is an icon, never an avatar.
    let shows_check = |is_selected: bool| variant == ChipVariant::Filter && is_selected;
    let leading_size = |is_selected: bool| {
        if shows_check(is_selected) {
            Some(ICON_SIZE)
        } else if leading.is_some() {
            Some(if avatar { AVATAR_SIZE } else { ICON_SIZE })
        } else {
            None
        }
    };
    let layout_for = |is_selected: bool| {
        ChipLayout::new(
            variant,
            leading_size(is_selected),
            avatar && !shows_check(is_selected),
            removable,
            galley.size().x,
        )
    };

    let sense = if enabled {
        Sense::click()
    } else if soft_disabled {
        Sense::focusable_noninteractive()
    } else {
        Sense::hover()
    };
    let initial_layout = layout_for(was_selected);
    let (rect, mut response) =
        ui.allocate_exact_size(Vec2::new(initial_layout.width, height), sense);

    // The remove target: the trailing icon and the space around it, full
    // height, registered after the body so it takes the clicks over it.
    let remove = initial_layout.trailing_x.map(|x| {
        let target = Rect::from_min_max(
            Pos2::new(rect.min.x + x - HORIZONTAL_SPACING / 2.0, rect.min.y),
            rect.max,
        );
        let remove_response = ui.interact(
            target,
            response.id.with("remove"),
            crate::a11y::sense(enabled, Sense::click()),
        );
        crate::a11y::button(&remove_response, enabled, &format!("Remove {text}"));
        remove_response
    });

    let delete_pressed = response.has_focus()
        && ui.input(|i| i.key_pressed(Key::Delete) || i.key_pressed(Key::Backspace));
    let removed =
        enabled && removable && (remove.as_ref().is_some_and(Response::clicked) || delete_pressed);

    if response.clicked() && enabled {
        if selectable {
            if let Some(selected) = selected.as_deref_mut() {
                *selected = !*selected;
                response.mark_changed();
            }
        }
        if let Some(action) = &action {
            action();
        }
    }
    if removed {
        if let Some(on_remove) = &on_remove {
            on_remove();
        }
    }

    let is_selected = selectable && selected.as_deref().is_some_and(|s| *s);
    if selectable {
        crate::a11y::toggle(&response, Role::CheckBox, enabled, &text, is_selected);
    } else {
        crate::a11y::button(&response, enabled, &text);
    }

    // Paint. The size was allocated before a click could toggle the
    // selection, so a filter chip's check mark appears with the next frame's
    // layout; paint this frame with the layout it was allocated with.
    let layout = initial_layout;
    let painted_selected = if layout_for(is_selected) == initial_layout {
        is_selected
    } else {
        was_selected
    };
    let state = InteractionState::from_response(&response);
    let colors = chip_colors(&theme, variant, is_selected, elevated, enabled);
    let target = target_radius(expressive, is_selected, state.pressed, height);
    let radius = if expressive {
        animate_value_with_spring(
            ui.ctx(),
            response.id.with("shape"),
            target,
            theme.motion.spatial(SpringSpeed::Fast),
        )
    } else {
        target
    };
    let corner_radius = CornerRadius::same(radius.round().clamp(0.0, 255.0) as u8);
    if layout_for(is_selected) != initial_layout {
        ui.ctx().request_repaint();
    }

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let elevation = chip_elevation(variant, is_selected, elevated, enabled, state);
        paint_elevation(
            painter,
            rect,
            corner_radius,
            elevation,
            theme.color(ColorRole::Shadow),
        );
        painter.rect_filled(rect, corner_radius, colors.container);
        paint_state_layer(painter, rect, corner_radius, colors.label, state);
        if let Some(outline) = colors.outline {
            painter.rect_stroke(
                rect,
                corner_radius,
                Stroke::new(OUTLINE_WIDTH, outline),
                StrokeKind::Inside,
            );
        }

        let slot = |x: f32, size: f32| {
            Rect::from_min_size(
                Pos2::new(rect.min.x + x, rect.center().y - size / 2.0),
                Vec2::splat(size),
            )
        };
        if shows_check(painted_selected) {
            paint_check(ui, slot(layout.leading_x, ICON_SIZE), colors.leading);
        } else if let (Some(image), Some(size)) = (&leading, leading_size(painted_selected)) {
            paint_leading(
                ui,
                slot(layout.leading_x, size),
                image,
                avatar,
                colors.leading,
                enabled,
            );
        }

        let text_pos = Pos2::new(
            rect.min.x + layout.label_x,
            rect.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(text_pos, galley, colors.label);

        if let Some(x) = layout.trailing_x {
            paint_close(ui, slot(x, ICON_SIZE), colors.trailing);
        }
        paint_focus_ring_if_focused(ui.painter(), &response, corner_radius);
    }

    ChipResponse {
        response,
        remove,
        removed,
    }
}

impl Widget for MaterialChip<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

/// Lays chips out in a wrapping row with [`CHIP_SET_SPACING`] between
/// them and between rows.
///
/// ```rust
/// # use egui_material3::*;
/// # egui::__run_test_ui(|ui| {
/// let mut tags = [true, false, true];
/// chip_set(ui, |ui| {
///     for (tag, on) in ["Rust", "egui", "Material"].into_iter().zip(&mut tags) {
///         ui.add(filter_chip(tag, on));
///     }
/// });
/// # });
/// ```
pub fn chip_set<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::splat(CHIP_SET_SPACING);
        add_contents(ui)
    })
}

/// Creates an assist chip with the given text.
pub fn assist_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::assist(text)
}

/// Creates a filter chip with the given text, bound to `selected`.
pub fn filter_chip(text: impl Into<String>, selected: &mut bool) -> MaterialChip<'_> {
    MaterialChip::filter(text, selected)
}

/// Creates an input chip with the given text.
pub fn input_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::input(text)
}

/// Creates a suggestion chip with the given text.
pub fn suggestion_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::suggestion(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    /// Regression test for nikescar/egui-material3#15: clicking a filter
    /// chip toggles it and calls its `on_click` callback.
    #[test]
    fn test_filter_chip_calls_on_click() {
        let ctx = egui::Context::default();
        let mut selected = false;
        let clicks = Cell::new(0);
        let frame = |events: Vec<egui::Event>, selected: &mut bool| {
            let mut rect = Rect::NOTHING;
            let input = egui::RawInput {
                events,
                ..Default::default()
            };
            ctx.run_ui(input, |ui| {
                let chip = MaterialChip::filter("Filter", selected)
                    .on_click(|| clicks.set(clicks.get() + 1));
                rect = ui.add(chip).rect;
            })
            .textures_delta
            .clear();
            rect
        };
        let pos = frame(Vec::new(), &mut selected).center();
        let button = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        frame(
            vec![egui::Event::PointerMoved(pos), button(true)],
            &mut selected,
        );
        frame(vec![button(false)], &mut selected);
        assert!(selected, "clicking a filter chip should select it");
        assert_eq!(
            clicks.get(),
            1,
            "clicking a filter chip should call on_click"
        );
    }

    /// `Chip.kt` paddings: 16 to the label with no icon (12 on input
    /// chips), icon 8 from the edge and 8 from the label, avatar 4 from
    /// the edge, trailing icon 8 from the label and 8 from the edge.
    #[test]
    fn layout_follows_compose_paddings() {
        let plain = ChipLayout::new(ChipVariant::Assist, None, false, false, 50.0);
        assert_eq!(plain.label_x, 16.0, "label 16 from the start");
        assert_eq!(plain.width, 82.0, "label 16 from both edges");

        let icon = ChipLayout::new(ChipVariant::Filter, Some(18.0), false, false, 50.0);
        assert_eq!(icon.leading_x, 8.0, "icon 8 from the start");
        assert_eq!(icon.label_x, 34.0, "label 8 after the icon");

        let input = ChipLayout::new(ChipVariant::Input, None, false, false, 50.0);
        assert_eq!(input.label_x, 12.0, "input label 12 from the start");
        assert_eq!(input.width, 74.0, "input label 12 from both edges");

        let avatar = ChipLayout::new(ChipVariant::Input, Some(24.0), true, true, 50.0);
        assert_eq!(avatar.leading_x, 4.0, "avatar 4 from the start");
        assert_eq!(avatar.label_x, 36.0, "label 8 after the avatar");
        assert_eq!(
            avatar.trailing_x,
            Some(36.0 + 50.0 + 8.0),
            "trailing icon 8 after the label"
        );
        assert_eq!(
            avatar.width,
            36.0 + 50.0 + 8.0 + 18.0 + 8.0,
            "trailing icon 8 from the end"
        );
    }

    #[test]
    fn expressive_radius() {
        assert_eq!(target_radius(false, true, false, 32.0), 8.0, "baseline 8");
        assert_eq!(
            target_radius(true, false, false, 32.0),
            12.0,
            "unselected 12"
        );
        assert_eq!(
            target_radius(true, true, false, 32.0),
            16.0,
            "selected full"
        );
        assert_eq!(target_radius(true, true, true, 32.0), 8.0, "pressed 8");
    }
}
