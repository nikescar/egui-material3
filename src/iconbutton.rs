// ============================================================================
// Module:       iconbutton
// Description:  Material 3 icon buttons: four styles, five sizes, three
//               widths, round and square shapes, and toggle colour sets.
//
// Dependencies: egui; crate::{a11y, icon, material_symbol, material_theme,
//               paint, tokens}
// ============================================================================

//! Material 3 icon buttons.
//!
//! Four styles ([`IconButtonVariant`]: standard, filled, filled tonal,
//! outlined), five sizes ([`IconButtonSize`]: XS 32, S 40, M 56, L 96,
//! XL 136), three widths ([`IconButtonWidth`]: narrow, default, wide) and two
//! shapes ([`IconButtonShape`]: round and square). The corners morph to the
//! pressed shape while the button is held, and a toggle button morphs to its
//! selected shape when it is on.
//!
//! Every value comes from the Compose material3 token files
//! (`androidx.compose.material3.tokens`, `androidx-main`):
//! `IconButtonXSmallTokens` .. `IconButtonXLargeTokens` for geometry and
//! shapes, `StandardIconButtonTokens`, `FilledIconButtonTokens`,
//! `FilledTonalIconButtonTokens` and `OutlinedIconButtonTokens` for colours,
//! and `MeasurementTokens` for the leading and trailing spaces. These are the
//! M3 Expressive token sets; the baseline 40 dp button is
//! [`IconButtonSize::Small`] with the default width and round shape.
//!
//! Buttons smaller than 48 dp keep a 48 dp touch target (Compose's
//! `minimumInteractiveComponentSize`): the response, and the space taken in
//! the layout, is at least 48 x 48 with the container centred in it.
//!
//! An icon-only button has nothing for a screen reader to announce but its
//! label: give every button a [`MaterialIconButton::label`]. When there is
//! none, a Material Symbol's name (`"delete"`) is used.

use egui::{
    Align2, Color32, CornerRadius, FontId, Painter, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    Vec2, Widget,
};

use crate::icon::{paint_icon, Icon, SymbolOptions, AUTO_MIRRORED};
use crate::material_symbol::get_material_symbol;
use crate::tokens::{
    animate_value_with_spring, state, ColorRole, CornerSize, InteractionState, LayoutDirection,
    SpringSpeed,
};
use crate::MaterialTheme;

/// `minimumInteractiveComponentSize`: the smallest touch target, 48 dp.
pub const MIN_TOUCH_TARGET: f32 = 48.0;

/// The style of an icon button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconButtonVariant {
    /// No container; the icon alone (`StandardIconButtonTokens`).
    Standard,
    /// A `primary` container, the highest emphasis (`FilledIconButtonTokens`).
    Filled,
    /// A `secondaryContainer` container (`FilledTonalIconButtonTokens`).
    FilledTonal,
    /// No fill, an `outlineVariant` border (`OutlinedIconButtonTokens`).
    Outlined,
}

impl IconButtonVariant {
    /// Every style.
    pub const ALL: [IconButtonVariant; 4] = [
        IconButtonVariant::Standard,
        IconButtonVariant::Filled,
        IconButtonVariant::FilledTonal,
        IconButtonVariant::Outlined,
    ];
}

/// The size of an icon button.
///
/// `f32` converts into a size: a token height (32, 40, 56, 96, 136) picks
/// that size, any other value is a [`IconButtonSize::Custom`] square
/// button, so `.size(48.0)` keeps working.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum IconButtonSize {
    /// `IconButtonXSmallTokens`: 32 high, 20 icon.
    ExtraSmall,
    /// `IconButtonSmallTokens`: 40 high, 24 icon (the baseline size).
    #[default]
    Small,
    /// `IconButtonMediumTokens`: 56 high, 24 icon.
    Medium,
    /// `IconButtonLargeTokens`: 96 high, 32 icon.
    Large,
    /// `IconButtonXLargeTokens`: 136 high, 40 icon.
    ExtraLarge,
    /// A square button of this side, with the icon size, shapes and outline
    /// width of the largest token size that fits in it.
    Custom(f32),
}

impl From<f32> for IconButtonSize {
    fn from(side: f32) -> Self {
        IconButtonSize::TOKEN_SIZES
            .into_iter()
            .find(|size| (size.metrics().height - side).abs() < f32::EPSILON)
            .unwrap_or(IconButtonSize::Custom(side))
    }
}

/// The width of an icon button: the icon plus the leading and trailing
/// spaces of the size's `Narrow*`, `Default*` or `Wide*` tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconButtonWidth {
    /// `NarrowLeadingSpace` / `NarrowTrailingSpace`.
    Narrow,
    /// `DefaultLeadingSpace` / `DefaultTrailingSpace`: a square button.
    #[default]
    Default,
    /// `WideLeadingSpace` / `WideTrailingSpace`.
    Wide,
}

impl IconButtonWidth {
    /// Every width.
    pub const ALL: [IconButtonWidth; 3] = [
        IconButtonWidth::Narrow,
        IconButtonWidth::Default,
        IconButtonWidth::Wide,
    ];
}

/// The container shape of an icon button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconButtonShape {
    /// `ContainerShapeRound`: fully rounded.
    #[default]
    Round,
    /// `ContainerShapeSquare`: the size's square corner.
    Square,
}

/// The geometry of one icon button size, from its token file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IconButtonMetrics {
    /// `ContainerHeight`.
    pub height: f32,
    /// `IconSize`.
    pub icon_size: f32,
    /// `NarrowLeadingSpace` (equal to `NarrowTrailingSpace`).
    pub narrow_space: f32,
    /// `DefaultLeadingSpace` (equal to `DefaultTrailingSpace`).
    pub default_space: f32,
    /// `WideLeadingSpace` (equal to `WideTrailingSpace`).
    pub wide_space: f32,
    /// `OutlinedOutlineWidth`.
    pub outline_width: f32,
    /// `ContainerShapeSquare`.
    pub square_shape: CornerSize,
    /// `PressedContainerShape`.
    pub pressed_shape: CornerSize,
    /// `SelectedContainerShapeRound` (a selected toggle of round shape;
    /// `SelectedContainerShapeSquare` is always `CornerFull`).
    pub selected_round_shape: CornerSize,
}

impl IconButtonMetrics {
    /// The container width for `width`.
    pub fn width(&self, width: IconButtonWidth) -> f32 {
        let space = match width {
            IconButtonWidth::Narrow => self.narrow_space,
            IconButtonWidth::Default => self.default_space,
            IconButtonWidth::Wide => self.wide_space,
        };
        self.icon_size + 2.0 * space
    }
}

impl IconButtonSize {
    /// The five token sizes, smallest first.
    pub const TOKEN_SIZES: [IconButtonSize; 5] = [
        IconButtonSize::ExtraSmall,
        IconButtonSize::Small,
        IconButtonSize::Medium,
        IconButtonSize::Large,
        IconButtonSize::ExtraLarge,
    ];

    /// The geometry of this size.
    pub fn metrics(self) -> IconButtonMetrics {
        match self {
            // IconButtonXSmallTokens. Spaces: Space50 / Space75 / Space125.
            IconButtonSize::ExtraSmall => IconButtonMetrics {
                height: 32.0,
                icon_size: 20.0,
                narrow_space: 4.0,
                default_space: 6.0,
                wide_space: 10.0,
                outline_width: 1.0,
                square_shape: CornerSize::Medium,
                pressed_shape: CornerSize::Small,
                selected_round_shape: CornerSize::Medium,
            },
            // IconButtonSmallTokens. Spaces: Space50 / Space100 / Space175.
            IconButtonSize::Small => IconButtonMetrics {
                height: 40.0,
                icon_size: 24.0,
                narrow_space: 4.0,
                default_space: 8.0,
                wide_space: 14.0,
                outline_width: 1.0,
                square_shape: CornerSize::Medium,
                pressed_shape: CornerSize::Small,
                selected_round_shape: CornerSize::Medium,
            },
            // IconButtonMediumTokens. Spaces: Space150 / Space200 / Space300.
            IconButtonSize::Medium => IconButtonMetrics {
                height: 56.0,
                icon_size: 24.0,
                narrow_space: 12.0,
                default_space: 16.0,
                wide_space: 24.0,
                outline_width: 1.0,
                square_shape: CornerSize::Large,
                pressed_shape: CornerSize::Medium,
                selected_round_shape: CornerSize::Large,
            },
            // IconButtonLargeTokens. Spaces: Space200 / Space400 / Space600.
            IconButtonSize::Large => IconButtonMetrics {
                height: 96.0,
                icon_size: 32.0,
                narrow_space: 16.0,
                default_space: 32.0,
                wide_space: 48.0,
                outline_width: 2.0,
                square_shape: CornerSize::ExtraLarge,
                pressed_shape: CornerSize::Large,
                selected_round_shape: CornerSize::ExtraLarge,
            },
            // IconButtonXLargeTokens. Spaces: Space400 / Space600 / Space900.
            IconButtonSize::ExtraLarge => IconButtonMetrics {
                height: 136.0,
                icon_size: 40.0,
                narrow_space: 32.0,
                default_space: 48.0,
                wide_space: 72.0,
                outline_width: 3.0,
                square_shape: CornerSize::ExtraLarge,
                pressed_shape: CornerSize::Large,
                selected_round_shape: CornerSize::ExtraLarge,
            },
            IconButtonSize::Custom(side) => {
                let side = if side.is_finite() { side.max(0.0) } else { 0.0 };
                let base = IconButtonSize::TOKEN_SIZES
                    .into_iter()
                    .map(IconButtonSize::metrics)
                    .take_while(|m| m.height <= side)
                    .last()
                    .unwrap_or(IconButtonSize::ExtraSmall.metrics());
                let icon_size = base.icon_size.min(side);
                let space = ((side - icon_size) / 2.0).max(0.0);
                IconButtonMetrics {
                    height: side,
                    icon_size,
                    narrow_space: space.min(base.narrow_space),
                    default_space: space,
                    wide_space: space.max(base.wide_space),
                    ..base
                }
            }
        }
    }
}

/// Container, icon and outline colours for one state of an icon button.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IconButtonColors {
    /// The container fill (transparent for standard and outlined).
    pub container: Color32,
    /// The icon, and the state layer.
    pub content: Color32,
    /// The outline (transparent except for outlined).
    pub outline: Color32,
}

impl IconButtonColors {
    /// The colours of `variant` in `theme`. `selected` is `None` for a plain
    /// button and `Some(on)` for a toggle button.
    pub fn resolve(
        theme: &MaterialTheme,
        variant: IconButtonVariant,
        selected: Option<bool>,
        enabled: bool,
    ) -> Self {
        let c = |role| theme.color(role);
        let none = Color32::TRANSPARENT;
        // `*IconButtonTokens.DisabledColor` (OnSurface) at `DisabledOpacity`
        // 0.38, and `DisabledContainerColor` (OnSurface) at
        // `DisabledContainerOpacity` 0.1.
        let disabled_content =
            c(ColorRole::OnSurface).gamma_multiply(state::DISABLED_CONTENT_OPACITY);
        let disabled_container =
            c(ColorRole::OnSurface).gamma_multiply(state::DISABLED_CONTAINER_OPACITY);
        let colors = |container, content, outline| IconButtonColors {
            container,
            content,
            outline,
        };
        match (variant, selected, enabled) {
            // StandardIconButtonTokens: Color / UnselectedColor
            // OnSurfaceVariant, SelectedColor Primary.
            (IconButtonVariant::Standard, Some(true), true) => {
                colors(none, c(ColorRole::Primary), none)
            }
            (IconButtonVariant::Standard, None | Some(false), true) => {
                colors(none, c(ColorRole::OnSurfaceVariant), none)
            }
            (IconButtonVariant::Standard, _, false) => colors(none, disabled_content, none),
            // FilledIconButtonTokens: Primary / OnPrimary; unselected toggle
            // SurfaceContainer / OnSurfaceVariant.
            (IconButtonVariant::Filled, None | Some(true), true) => {
                colors(c(ColorRole::Primary), c(ColorRole::OnPrimary), none)
            }
            (IconButtonVariant::Filled, Some(false), true) => colors(
                c(ColorRole::SurfaceContainer),
                c(ColorRole::OnSurfaceVariant),
                none,
            ),
            // FilledTonalIconButtonTokens: SecondaryContainer /
            // OnSecondaryContainer; selected toggle Secondary / OnSecondary.
            (IconButtonVariant::FilledTonal, None | Some(false), true) => colors(
                c(ColorRole::SecondaryContainer),
                c(ColorRole::OnSecondaryContainer),
                none,
            ),
            (IconButtonVariant::FilledTonal, Some(true), true) => {
                colors(c(ColorRole::Secondary), c(ColorRole::OnSecondary), none)
            }
            (IconButtonVariant::Filled | IconButtonVariant::FilledTonal, _, false) => {
                colors(disabled_container, disabled_content, none)
            }
            // OutlinedIconButtonTokens: OnSurfaceVariant with an
            // OutlineVariant border; selected toggle InverseSurface /
            // InverseOnSurface with no border.
            (IconButtonVariant::Outlined, None | Some(false), true) => colors(
                none,
                c(ColorRole::OnSurfaceVariant),
                c(ColorRole::OutlineVariant),
            ),
            (IconButtonVariant::Outlined, Some(true), true) => colors(
                c(ColorRole::InverseSurface),
                c(ColorRole::InverseOnSurface),
                none,
            ),
            // `UnselectedDisabledOutlineColor` OutlineVariant;
            // `SelectedDisabledContainerColor` OnSurface at 0.1.
            (IconButtonVariant::Outlined, None | Some(false), false) => {
                colors(none, disabled_content, c(ColorRole::OutlineVariant))
            }
            (IconButtonVariant::Outlined, Some(true), false) => {
                colors(disabled_container, disabled_content, none)
            }
        }
    }
}

/// Paints `icon` centred in `rect` for a widget: a Material Symbol by name
/// or codepoint, an SVG or a texture through [`paint_icon`], filled when
/// `filled`, mirrored in right-to-left layouts for directional symbols. A
/// symbol string that is neither a name nor a Material Symbols codepoint
/// (an emoji, a letter) is drawn as text, so callers that passed emoji
/// strings keep working.
pub(crate) fn paint_widget_icon(
    painter: &Painter,
    rect: Rect,
    icon: &Icon,
    filled: bool,
    tint: Color32,
    direction: LayoutDirection,
) {
    if let Icon::Symbol(text) = icon {
        let is_name = get_material_symbol(text).is_some()
            || text
                .strip_suffix("_rtl")
                .is_some_and(|base| get_material_symbol(base).is_some());
        if !is_name && !is_symbol_codepoint(text) {
            if !text.is_empty() {
                let size = rect.width().min(rect.height());
                painter.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    text,
                    FontId::proportional(size),
                    tint,
                );
            }
            return;
        }
    }
    let mirrored = direction == LayoutDirection::Rtl
        && matches!(icon, Icon::Symbol(name) if AUTO_MIRRORED.contains(&name.as_str()));
    let options = SymbolOptions {
        filled,
        mirrored,
        ..SymbolOptions::default()
    };
    paint_icon(painter, rect, icon, &options, tint);
}

/// One character in a private use area, where Material Symbols puts its
/// glyphs.
fn is_symbol_codepoint(text: &str) -> bool {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => {
            ('\u{E000}'..='\u{F8FF}').contains(&c) || ('\u{F0000}'..='\u{10FFFD}').contains(&c)
        }
        (Some(_), Some(_)) | (None, _) => false,
    }
}

/// The accessible name for an icon: the symbol's name when it is one.
pub(crate) fn icon_label(icon: &Icon) -> &str {
    match icon {
        Icon::Symbol(name) if get_material_symbol(name).is_some() => name,
        Icon::Symbol(_) | Icon::Svg(_) | Icon::Texture(_) => "",
    }
}

/// A Material 3 icon button.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{IconButtonShape, IconButtonSize, MaterialIconButton};
///
/// if ui.add(MaterialIconButton::standard("delete").label("Delete")).clicked() {
///     // delete
/// }
/// let mut starred = false;
/// ui.add(
///     MaterialIconButton::filled("star")
///         .toggleable(&mut starred)
///         .size(IconButtonSize::Medium)
///         .shape(IconButtonShape::Square)
///         .label("Star"),
/// );
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialIconButton<'a> {
    icon: Icon,
    variant: IconButtonVariant,
    selected: Option<&'a mut bool>,
    label: Option<String>,
    enabled: bool,
    size: IconButtonSize,
    width: IconButtonWidth,
    shape: IconButtonShape,
    icon_color_override: Option<Color32>,
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialIconButton<'a> {
    /// An icon button of `variant` showing `icon` (a Material Symbol name,
    /// or any [`Icon`]).
    pub fn new(icon: impl Into<Icon>, variant: IconButtonVariant) -> Self {
        Self {
            icon: icon.into(),
            variant,
            selected: None,
            label: None,
            enabled: true,
            size: IconButtonSize::Small,
            width: IconButtonWidth::Default,
            shape: IconButtonShape::Round,
            icon_color_override: None,
            action: None,
        }
    }

    /// A standard icon button: no container.
    pub fn standard(icon: impl Into<Icon>) -> Self {
        Self::new(icon, IconButtonVariant::Standard)
    }

    /// A filled icon button.
    pub fn filled(icon: impl Into<Icon>) -> Self {
        Self::new(icon, IconButtonVariant::Filled)
    }

    /// A filled tonal icon button.
    pub fn filled_tonal(icon: impl Into<Icon>) -> Self {
        Self::new(icon, IconButtonVariant::FilledTonal)
    }

    /// An outlined icon button.
    pub fn outlined(icon: impl Into<Icon>) -> Self {
        Self::new(icon, IconButtonVariant::Outlined)
    }

    /// A standard toggle icon button bound to `selected`. Any style can
    /// toggle with [`Self::toggleable`].
    pub fn toggle(icon: impl Into<Icon>, selected: &'a mut bool) -> Self {
        Self::standard(icon).toggleable(selected)
    }

    /// Makes this a toggle button bound to `selected`: a click flips it, the
    /// style's selected colours and shape show when it is on, the icon is
    /// drawn filled, and AccessKit reports the toggled state.
    pub fn toggleable(mut self, selected: &'a mut bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// The size: an [`IconButtonSize`], or a side in points.
    pub fn size(mut self, size: impl Into<IconButtonSize>) -> Self {
        self.size = size.into();
        self
    }

    /// The width variant.
    pub fn width(mut self, width: IconButtonWidth) -> Self {
        self.width = width;
        self
    }

    /// The container shape.
    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.shape = shape;
        self
    }

    /// `true` for the square shape, `false` for round.
    pub fn container(self, square: bool) -> Self {
        self.shape(if square {
            IconButtonShape::Square
        } else {
            IconButtonShape::Round
        })
    }

    /// The accessible name. Every icon-only button needs one.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Whether the button reacts. A disabled button takes no focus and
    /// reports no clicks.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Shows the SVG file at `path`. The file is read on every call; prefer
    /// loading it once and passing [`Icon::svg`] to [`Self::new`].
    pub fn svg(mut self, path: impl Into<String>) -> Self {
        if let Ok(bytes) = std::fs::read(path.into()) {
            self.icon = Icon::svg(bytes);
        }
        self
    }

    /// Shows an SVG document given as text.
    pub fn svg_data(mut self, svg_content: impl Into<String>) -> Self {
        self.icon = Icon::svg(svg_content.into().into_bytes());
        self
    }

    /// Overrides the icon colour of the enabled button.
    pub fn icon_color(mut self, color: Color32) -> Self {
        self.icon_color_override = Some(color);
        self
    }

    /// Runs `f` when the button is clicked.
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

impl Widget for MaterialIconButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let metrics = self.size.metrics();
        let visual = Vec2::new(metrics.width(self.width), metrics.height);
        let target = visual.max(Vec2::splat(MIN_TOUCH_TARGET));
        let (outer, mut response) =
            ui.allocate_exact_size(target, crate::a11y::sense(self.enabled, Sense::click()));
        let rect = Rect::from_center_size(outer.center(), visual);

        let label = self
            .label
            .clone()
            .unwrap_or_else(|| icon_label(&self.icon).to_owned());
        let mut is_selected = self.selected.as_deref().copied();

        if response.clicked() && self.enabled {
            if let Some(selected) = self.selected {
                *selected = !*selected;
                is_selected = Some(*selected);
                response.mark_changed();
            }
            if let Some(action) = &self.action {
                action();
            }
        }
        match is_selected {
            Some(on) => {
                crate::a11y::toggle(&response, egui::Role::Button, self.enabled, &label, on)
            }
            None => crate::a11y::button(&response, self.enabled, &label),
        }

        if !ui.is_rect_visible(outer) {
            return response;
        }

        let theme = MaterialTheme::of(ui.ctx());
        let mut colors = IconButtonColors::resolve(&theme, self.variant, is_selected, self.enabled);
        if let (Some(color), true) = (self.icon_color_override, self.enabled) {
            colors.content = color;
        }
        let interaction = InteractionState::from_response(&response);

        // Shape: rest, selected (toggle on) or pressed, morphing with the
        // fast spatial spring.
        let full = rect.width().min(rect.height()) / 2.0;
        let rest = match self.shape {
            IconButtonShape::Round => full,
            IconButtonShape::Square => metrics.square_shape.radius(rect),
        };
        let selected_radius = match self.shape {
            IconButtonShape::Round => metrics.selected_round_shape.radius(rect),
            IconButtonShape::Square => full,
        };
        let target_radius = if interaction.pressed {
            metrics.pressed_shape.radius(rect)
        } else if is_selected == Some(true) {
            selected_radius
        } else {
            rest
        };
        let radius = animate_value_with_spring(
            ui.ctx(),
            response.id.with("icon_button_shape"),
            target_radius,
            theme.motion.spatial(SpringSpeed::Fast),
        )
        .clamp(0.0, full);
        let corner_radius = CornerRadius::same(radius.round().clamp(0.0, 255.0) as u8);

        let painter = ui.painter();
        if colors.container != Color32::TRANSPARENT {
            painter.rect_filled(rect, corner_radius, colors.container);
        }
        if colors.outline != Color32::TRANSPARENT {
            painter.rect_stroke(
                rect,
                corner_radius,
                Stroke::new(metrics.outline_width, colors.outline),
                StrokeKind::Inside,
            );
        }
        crate::paint::paint_state_layer(painter, rect, corner_radius, colors.content, interaction);

        let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(metrics.icon_size));
        paint_widget_icon(
            painter,
            icon_rect,
            &self.icon,
            is_selected == Some(true),
            colors.content,
            theme.layout_direction,
        );

        if response.has_focus() {
            crate::paint::paint_focus_ring(
                painter,
                rect,
                corner_radius,
                theme.color(ColorRole::Secondary),
            );
        }

        response
    }
}

/// A standard icon button.
pub fn icon_button_standard(icon: impl Into<Icon>) -> MaterialIconButton<'static> {
    MaterialIconButton::standard(icon)
}

/// A filled icon button.
pub fn icon_button_filled(icon: impl Into<Icon>) -> MaterialIconButton<'static> {
    MaterialIconButton::filled(icon)
}

/// A filled tonal icon button.
pub fn icon_button_filled_tonal(icon: impl Into<Icon>) -> MaterialIconButton<'static> {
    MaterialIconButton::filled_tonal(icon)
}

/// An outlined icon button.
pub fn icon_button_outlined(icon: impl Into<Icon>) -> MaterialIconButton<'static> {
    MaterialIconButton::outlined(icon)
}

/// A standard toggle icon button bound to `selected`.
pub fn icon_button_toggle(icon: impl Into<Icon>, selected: &mut bool) -> MaterialIconButton<'_> {
    MaterialIconButton::toggle(icon, selected)
}
