// ============================================================================
// Module:       fab
// Description:  Material 3 floating action buttons: small, baseline, medium
//               and large FABs, and extended FABs that expand and collapse.
//
// Dependencies: egui; crate::{a11y, icon, iconbutton, material_theme, paint,
//               tokens}
// ============================================================================

//! Material 3 floating action buttons (FABs).
//!
//! [`MaterialFab`] is a FAB of four sizes ([`FabSize`]: small 40, baseline
//! 56, medium 80, large 96) and, with [`MaterialFab::text`], an extended FAB
//! of three (56, 80, 96) whose label slides away when
//! [`MaterialFab::expanded`] turns false.
//!
//! Values come from the Compose material3 token files
//! (`androidx.compose.material3.tokens`, `androidx-main`): `FabSmallTokens`,
//! `FabBaselineTokens`, `FabMediumTokens`, `FabLargeTokens`,
//! `ExtendedFabSmallTokens`, `ExtendedFabMediumTokens`,
//! `ExtendedFabLargeTokens`, and for colour and elevation
//! `FabPrimaryContainerTokens` (the Compose default), `FabSecondaryContainerTokens`,
//! `FabPrimaryTokens`, `FabSecondaryTokens`, `FabTertiaryTokens` and their
//! `ExtendedFab*` counterparts. Medium FABs and the medium and large extended
//! FABs are M3 Expressive tokens.
//!
//! Every FAB sits at `Level3` and rises to `Level4` on hover
//! (`ContainerElevation`, `HoveredContainerElevation`); a lowered FAB uses
//! Compose's `FloatingActionButtonDefaults.loweredElevation` (`Level1`,
//! `Level2` on hover).

use egui::{self, Color32, CornerRadius, Rect, Response, Sense, Ui, Vec2, Widget};

use crate::icon::Icon;
use crate::iconbutton::{icon_label, paint_widget_icon};
use crate::tokens::{
    animate_value_with_spring, state, ColorRole, CornerSize, Elevation, InteractionState,
    SpringSpeed, TypeRole,
};
use crate::MaterialTheme;

/// The colour scheme of a FAB.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FabVariant {
    /// `surfaceContainerHigh` with a `primary` icon: the retired M3 surface
    /// FAB (`FabSurfaceTokens`), kept for existing callers.
    Surface,
    /// `primary` / `onPrimary` (`FabPrimaryTokens`).
    Primary,
    /// `secondary` / `onSecondary` (`FabSecondaryTokens`).
    Secondary,
    /// `tertiary` / `onTertiary` (`FabTertiaryTokens`).
    Tertiary,
    /// `primaryContainer` / `onPrimaryContainer`
    /// (`FabPrimaryContainerTokens`): Compose's default FAB colours.
    PrimaryContainer,
    /// `secondaryContainer` / `onSecondaryContainer`
    /// (`FabSecondaryContainerTokens`).
    SecondaryContainer,
    /// `tertiaryContainer` / `onTertiaryContainer`, the tertiary member of
    /// the same container family.
    TertiaryContainer,
    /// A hard-coded brand blue (#4285F4) with white content, outside the
    /// theme.
    #[deprecated(
        note = "brand colours are not a Material role; use any variant with `container_color` and `content_color` (see the FAB story)"
    )]
    Branded,
}

impl FabVariant {
    /// Every colour scheme.
    pub const ALL: [FabVariant; 7] = [
        FabVariant::Surface,
        FabVariant::Primary,
        FabVariant::Secondary,
        FabVariant::Tertiary,
        FabVariant::PrimaryContainer,
        FabVariant::SecondaryContainer,
        FabVariant::TertiaryContainer,
    ];

    /// The container and content roles. The deprecated `Branded` variant
    /// has no roles and reports `primary` / `onPrimary`; it paints its
    /// fixed brand colours instead.
    #[expect(deprecated, reason = "matches the deprecated Branded variant")]
    pub const fn roles(self) -> (ColorRole, ColorRole) {
        match self {
            FabVariant::Branded => (ColorRole::Primary, ColorRole::OnPrimary),
            FabVariant::Surface => (ColorRole::SurfaceContainerHigh, ColorRole::Primary),
            FabVariant::Primary => (ColorRole::Primary, ColorRole::OnPrimary),
            FabVariant::Secondary => (ColorRole::Secondary, ColorRole::OnSecondary),
            FabVariant::Tertiary => (ColorRole::Tertiary, ColorRole::OnTertiary),
            FabVariant::PrimaryContainer => {
                (ColorRole::PrimaryContainer, ColorRole::OnPrimaryContainer)
            }
            FabVariant::SecondaryContainer => (
                ColorRole::SecondaryContainer,
                ColorRole::OnSecondaryContainer,
            ),
            FabVariant::TertiaryContainer => {
                (ColorRole::TertiaryContainer, ColorRole::OnTertiaryContainer)
            }
        }
    }
}

/// The size of a FAB.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FabSize {
    /// `FabSmallTokens`: 40, 24 icon, `CornerMedium`. Extended: the same as
    /// [`FabSize::Regular`].
    Small,
    /// `FabBaselineTokens`: 56, 24 icon, `CornerLarge`. Extended:
    /// `ExtendedFabSmallTokens`.
    #[default]
    Regular,
    /// `FabMediumTokens`: 80, 28 icon, `CornerLargeIncreased`. Extended:
    /// `ExtendedFabMediumTokens`.
    Medium,
    /// `FabLargeTokens`: 96, 36 icon, `CornerExtraLarge`. Extended:
    /// `ExtendedFabLargeTokens`.
    Large,
    /// The baseline extended FAB; the same as [`FabSize::Regular`] with a
    /// [`MaterialFab::text`] label.
    Extended,
}

impl FabSize {
    /// The four FAB sizes, smallest first.
    pub const ALL: [FabSize; 4] = [
        FabSize::Small,
        FabSize::Regular,
        FabSize::Medium,
        FabSize::Large,
    ];

    /// The geometry of the icon-only FAB of this size.
    pub const fn metrics(self) -> FabMetrics {
        match self {
            FabSize::Small => FabMetrics {
                size: 40.0,
                icon_size: 24.0,
                shape: CornerSize::Medium,
            },
            FabSize::Regular | FabSize::Extended => FabMetrics {
                size: 56.0,
                icon_size: 24.0,
                shape: CornerSize::Large,
            },
            FabSize::Medium => FabMetrics {
                size: 80.0,
                icon_size: 28.0,
                shape: CornerSize::LargeIncreased,
            },
            FabSize::Large => FabMetrics {
                size: 96.0,
                icon_size: 36.0,
                shape: CornerSize::ExtraLarge,
            },
        }
    }

    /// The geometry of the extended FAB of this size.
    pub const fn extended_metrics(self) -> ExtendedFabMetrics {
        match self {
            // ExtendedFabSmallTokens: Space100 gap, Space200 padding.
            FabSize::Small | FabSize::Regular | FabSize::Extended => ExtendedFabMetrics {
                height: 56.0,
                icon_size: 24.0,
                icon_label_space: 8.0,
                leading_space: 16.0,
                trailing_space: 16.0,
                shape: CornerSize::Large,
                label: TypeRole::LabelLarge,
            },
            // ExtendedFabMediumTokens: Space150 gap.
            FabSize::Medium => ExtendedFabMetrics {
                height: 80.0,
                icon_size: 28.0,
                icon_label_space: 12.0,
                leading_space: 26.0,
                trailing_space: 26.0,
                shape: CornerSize::LargeIncreased,
                label: TypeRole::TitleLarge,
            },
            // ExtendedFabLargeTokens: Space200 gap.
            FabSize::Large => ExtendedFabMetrics {
                height: 96.0,
                icon_size: 36.0,
                icon_label_space: 16.0,
                leading_space: 28.0,
                trailing_space: 28.0,
                shape: CornerSize::ExtraLarge,
                label: TypeRole::HeadlineSmall,
            },
        }
    }
}

/// The geometry of a FAB size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FabMetrics {
    /// `ContainerWidth` and `ContainerHeight`.
    pub size: f32,
    /// `IconSize`.
    pub icon_size: f32,
    /// `ContainerShape`.
    pub shape: CornerSize,
}

/// The geometry of an extended FAB size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtendedFabMetrics {
    /// `ContainerHeight` (the collapsed FAB is this wide too).
    pub height: f32,
    /// `IconSize`.
    pub icon_size: f32,
    /// `IconLabelSpace`.
    pub icon_label_space: f32,
    /// `LeadingSpace`.
    pub leading_space: f32,
    /// `TrailingSpace`.
    pub trailing_space: f32,
    /// `ContainerShape`.
    pub shape: CornerSize,
    /// `LabelText`.
    pub label: TypeRole,
}

/// The elevation of a FAB in `state`: `Level3`, `Level4` on hover; lowered
/// `Level1`, `Level2` on hover. Focus and press keep the resting level.
pub fn fab_elevation(state: InteractionState, lowered: bool) -> Elevation {
    let hovered = state.hovered && !state.pressed && !state.focused;
    match (lowered, hovered) {
        (false, false) => Elevation::Level3,
        (false, true) => Elevation::Level4,
        (true, false) => Elevation::Level1,
        (true, true) => Elevation::Level2,
    }
}

/// A Material 3 floating action button, or an extended FAB when it has a
/// [`text`](Self::text) label.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{FabSize, MaterialFab};
///
/// if ui.add(MaterialFab::primary_container().icon("edit").label("Compose")).clicked() {
///     // compose
/// }
/// let expanded = true; // e.g. the list is scrolled to the top
/// ui.add(
///     MaterialFab::primary_container()
///         .icon("edit")
///         .text("Compose")
///         .size(FabSize::Medium)
///         .expanded(expanded),
/// );
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialFab<'a> {
    variant: FabVariant,
    size: FabSize,
    icon: Option<Icon>,
    text: Option<String>,
    label: Option<String>,
    enabled: bool,
    lowered: bool,
    expanded: bool,
    container_color: Option<Color32>,
    content_color: Option<Color32>,
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialFab<'a> {
    /// A FAB of `variant` showing the `add` icon until another is set.
    pub fn new(variant: FabVariant) -> Self {
        Self {
            variant,
            size: FabSize::Regular,
            icon: None,
            text: None,
            label: None,
            enabled: true,
            lowered: false,
            expanded: true,
            container_color: None,
            content_color: None,
            action: None,
        }
    }

    /// A surface FAB.
    pub fn surface() -> Self {
        Self::new(FabVariant::Surface)
    }

    /// A primary FAB.
    pub fn primary() -> Self {
        Self::new(FabVariant::Primary)
    }

    /// A secondary FAB.
    pub fn secondary() -> Self {
        Self::new(FabVariant::Secondary)
    }

    /// A tertiary FAB.
    pub fn tertiary() -> Self {
        Self::new(FabVariant::Tertiary)
    }

    /// A primary-container FAB (the Compose default).
    pub fn primary_container() -> Self {
        Self::new(FabVariant::PrimaryContainer)
    }

    /// A secondary-container FAB.
    pub fn secondary_container() -> Self {
        Self::new(FabVariant::SecondaryContainer)
    }

    /// A tertiary-container FAB.
    pub fn tertiary_container() -> Self {
        Self::new(FabVariant::TertiaryContainer)
    }

    /// The size.
    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// The icon: a Material Symbol name, or any [`Icon`].
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// The label of an extended FAB.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// The accessible name. Defaults to the text, then the symbol name.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Whether the FAB reacts. Compose FABs have no disabled state; a
    /// disabled one here uses the common disabled colours (`onSurface` at
    /// 0.1 and 0.38) and no shadow.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Lowered elevation (`Level1`, `Level2` on hover), for a FAB on a
    /// surface that is itself raised.
    pub fn lowered(mut self, lowered: bool) -> Self {
        self.lowered = lowered;
        self
    }

    /// For an extended FAB: `false` collapses it to an icon-only FAB of the
    /// same height, animating the width with the default spatial spring.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// A FAB in the fixed brand blue.
    #[deprecated(
        note = "use any variant with `container_color` and `content_color` (see the FAB story)"
    )]
    #[expect(deprecated, reason = "builds the deprecated Branded variant")]
    pub fn branded() -> Self {
        Self::new(FabVariant::Branded)
    }

    /// Shows a list of filled paths as the icon.
    #[deprecated(note = "pass an SVG document as the icon: `.icon(Icon::svg(bytes))`")]
    #[expect(deprecated, reason = "takes the deprecated SvgIcon")]
    pub fn svg_icon(mut self, svg_icon: SvgIcon) -> Self {
        self.icon = Some(Icon::svg(svg_icon.to_svg_document().into_bytes()));
        self
    }

    /// Shows an SVG document given as text as the icon.
    pub fn svg_data(mut self, svg_data: impl Into<String>) -> Self {
        self.icon = Some(Icon::svg(svg_data.into().into_bytes()));
        self
    }

    /// Overrides the container colour, for brand colours outside the theme.
    pub fn container_color(mut self, color: Color32) -> Self {
        self.container_color = Some(color);
        self
    }

    /// Overrides the icon and label colour.
    pub fn content_color(mut self, color: Color32) -> Self {
        self.content_color = Some(color);
        self
    }

    /// Runs `f` when the FAB is clicked.
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

impl Widget for MaterialFab<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let icon = self.icon.unwrap_or_else(|| Icon::from("add"));
        let text = self.text.filter(|t| !t.is_empty());

        let (container_role, content_role) = self.variant.roles();
        let (role_container, role_content) = if is_branded(self.variant) {
            (BRAND_BLUE, Color32::WHITE)
        } else {
            (theme.color(container_role), theme.color(content_role))
        };
        let (container, content) = if self.enabled {
            (
                self.container_color.unwrap_or(role_container),
                self.content_color.unwrap_or(role_content),
            )
        } else {
            let on_surface = theme.color(ColorRole::OnSurface);
            (
                on_surface.gamma_multiply(state::DISABLED_CONTAINER_OPACITY),
                on_surface.gamma_multiply(state::DISABLED_CONTENT_OPACITY),
            )
        };

        // Size: an icon FAB, or an extended FAB between its collapsed
        // (square) and expanded widths.
        let anim_id = ui.next_auto_id().with("fab_expanded");
        let (size, icon_size, shape, label_galley, expansion, extended) = match &text {
            Some(text) => {
                let m = self.size.extended_metrics();
                let galley = ui.painter().layout_job(m.label.style().layout_job(
                    ui.ctx(),
                    text.as_str(),
                    content,
                ));
                let expansion = animate_value_with_spring(
                    ui.ctx(),
                    anim_id,
                    if self.expanded { 1.0 } else { 0.0 },
                    theme.motion.spatial(SpringSpeed::Default),
                )
                .max(0.0);
                let full = m.leading_space
                    + m.icon_size
                    + m.icon_label_space
                    + galley.size().x
                    + m.trailing_space;
                let width = (m.height + (full - m.height) * expansion).max(m.height);
                (
                    Vec2::new(width, m.height),
                    m.icon_size,
                    m.shape,
                    Some(galley),
                    expansion,
                    Some(m),
                )
            }
            None => {
                let m = self.size.metrics();
                (Vec2::splat(m.size), m.icon_size, m.shape, None, 0.0, None)
            }
        };

        let (rect, response) =
            ui.allocate_exact_size(size, crate::a11y::sense(self.enabled, Sense::click()));
        let label = self
            .label
            .clone()
            .or_else(|| text.clone())
            .unwrap_or_else(|| icon_label(&icon).to_owned());
        crate::a11y::button(&response, self.enabled, &label);

        if response.clicked() && self.enabled {
            if let Some(action) = &self.action {
                action();
            }
        }

        if !ui.is_rect_visible(rect.expand(16.0)) {
            return response;
        }

        let interaction = InteractionState::from_response(&response);
        let radius = shape.radius(rect);
        let corner_radius = CornerRadius::same(radius.round().clamp(0.0, 255.0) as u8);
        let painter = ui.painter();

        if self.enabled {
            crate::paint::paint_elevation(
                painter,
                rect,
                corner_radius,
                fab_elevation(interaction, self.lowered),
                theme.color(ColorRole::Shadow),
            );
        }
        painter.rect_filled(rect, corner_radius, container);
        crate::paint::paint_state_layer(painter, rect, corner_radius, content, interaction);

        match (extended, label_galley) {
            (Some(m), Some(galley)) => {
                // The icon moves from the centre (collapsed) to the leading
                // space (expanded); the label fades in after it.
                let collapsed_x = rect.center().x;
                let expanded_x = rect.min.x + m.leading_space + m.icon_size / 2.0;
                let t = expansion.min(1.0);
                let icon_x = collapsed_x + (expanded_x - collapsed_x) * t;
                let icon_rect = Rect::from_center_size(
                    egui::pos2(icon_x, rect.center().y),
                    Vec2::splat(icon_size),
                );
                paint_widget_icon(
                    painter,
                    icon_rect,
                    &icon,
                    false,
                    content,
                    theme.layout_direction,
                );
                if t > 0.0 {
                    let pos = egui::pos2(
                        icon_rect.max.x + m.icon_label_space,
                        rect.center().y - galley.size().y / 2.0,
                    );
                    let clipped = painter.with_clip_rect(rect.intersect(painter.clip_rect()));
                    clipped.galley(pos, galley, content.gamma_multiply(t * t));
                }
            }
            (Some(_), None) | (None, Some(_)) | (None, None) => {
                let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
                paint_widget_icon(
                    painter,
                    icon_rect,
                    &icon,
                    false,
                    content,
                    theme.layout_direction,
                );
            }
        }

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

/// The brand blue of the deprecated `Branded` variant.
const BRAND_BLUE: Color32 = Color32::from_rgb(66, 133, 244);

#[expect(deprecated, reason = "detects the deprecated Branded variant")]
fn is_branded(variant: FabVariant) -> bool {
    variant == FabVariant::Branded
}

/// An icon made of filled SVG paths.
#[deprecated(note = "pass an SVG document as the icon: `Icon::svg(bytes)`")]
#[derive(Clone, Debug)]
pub struct SvgIcon {
    /// The paths, painted in order.
    #[expect(deprecated, reason = "holds the deprecated SvgPath")]
    pub paths: Vec<SvgPath>,
    /// The size of the paths' coordinate space.
    pub viewbox_size: Vec2,
}

/// One filled path of an [`SvgIcon`].
#[deprecated(note = "pass an SVG document as the icon: `Icon::svg(bytes)`")]
#[derive(Clone, Debug)]
pub struct SvgPath {
    /// The SVG path data (`d` attribute).
    pub path: String,
    /// The fill colour.
    pub fill: Color32,
}

#[expect(deprecated, reason = "converts the deprecated SvgIcon")]
impl SvgIcon {
    /// The icon as an SVG document.
    fn to_svg_document(&self) -> String {
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">"#,
            self.viewbox_size.x, self.viewbox_size.y
        );
        for p in &self.paths {
            let escaped = p
                .path
                .replace('&', "&amp;")
                .replace('"', "&quot;")
                .replace('<', "&lt;");
            svg.push_str(&format!(
                r##"<path d="{escaped}" fill="#{:02x}{:02x}{:02x}" fill-opacity="{}"/>"##,
                p.fill.r(),
                p.fill.g(),
                p.fill.b(),
                f32::from(p.fill.a()) / 255.0
            ));
        }
        svg.push_str("</svg>");
        svg
    }
}

/// A FAB in the fixed brand blue.
#[deprecated(
    note = "use any variant with `container_color` and `content_color` (see the FAB story)"
)]
#[expect(deprecated, reason = "builds the deprecated Branded FAB")]
pub fn fab_branded() -> MaterialFab<'static> {
    MaterialFab::branded()
}

/// The four-colour mark the deprecated branded FAB showed.
#[deprecated(note = "pass an SVG document as the icon: `Icon::svg(bytes)`")]
#[expect(deprecated, reason = "builds the deprecated SvgIcon")]
pub fn google_branded_icon() -> SvgIcon {
    let path = |path: &str, fill| SvgPath {
        path: path.to_owned(),
        fill,
    };
    SvgIcon {
        paths: vec![
            path("M16 16v14h4V20z", Color32::from_rgb(52, 168, 83)),
            path("M30 16H20l-4 4h14z", Color32::from_rgb(66, 133, 244)),
            path("M6 16v4h10l4-4z", Color32::from_rgb(251, 188, 5)),
            path("M20 16V6h-4v14z", Color32::from_rgb(234, 67, 53)),
        ],
        viewbox_size: Vec2::new(36.0, 36.0),
    }
}

/// A surface FAB.
pub fn fab_surface() -> MaterialFab<'static> {
    MaterialFab::surface()
}

/// A primary FAB.
pub fn fab_primary() -> MaterialFab<'static> {
    MaterialFab::primary()
}

/// A secondary FAB.
pub fn fab_secondary() -> MaterialFab<'static> {
    MaterialFab::secondary()
}

/// A tertiary FAB.
pub fn fab_tertiary() -> MaterialFab<'static> {
    MaterialFab::tertiary()
}

/// A primary-container FAB (the Compose default).
pub fn fab_primary_container() -> MaterialFab<'static> {
    MaterialFab::primary_container()
}

/// A secondary-container FAB.
pub fn fab_secondary_container() -> MaterialFab<'static> {
    MaterialFab::secondary_container()
}

/// A tertiary-container FAB.
pub fn fab_tertiary_container() -> MaterialFab<'static> {
    MaterialFab::tertiary_container()
}
