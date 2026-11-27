// ============================================================================
// Module:       topappbar
// Description:  Material Design 3 top app bars: small, center-aligned,
//               medium, large and the Expressive flexible variants.
//
// Dependencies: egui; crate::icon, crate::tokens, crate::toolbar
// ============================================================================

//! Material Design 3 top app bars.
//!
//! Heights, type roles and colours come from the Jetpack Compose material3
//! token files:
//!
//! | Variant | Tokens | Height | Title | Subtitle |
//! |---|---|---|---|---|
//! | [`Regular`](TopAppBarVariant::Regular), [`CenterAligned`](TopAppBarVariant::CenterAligned) | `TopAppBarSmallTokens`, `TopAppBarSmallCenteredTokens`, Expressive `AppBarSmallTokens` | 64 | TitleLarge | LabelMedium |
//! | [`Medium`](TopAppBarVariant::Medium) | `TopAppBarMediumTokens` | 112 | HeadlineSmall | none |
//! | [`Large`](TopAppBarVariant::Large) | `TopAppBarLargeTokens` | 152 | HeadlineMedium | none |
//! | [`MediumFlexible`](TopAppBarVariant::MediumFlexible) | Expressive `AppBarMediumFlexibleTokens` | 112, 136 with subtitle | HeadlineMedium | LabelLarge |
//! | [`LargeFlexible`](TopAppBarVariant::LargeFlexible) | Expressive `AppBarLargeFlexibleTokens` | 120, 152 with subtitle | DisplaySmall | TitleMedium |
//!
//! Every variant collapses to the 64 dp small bar. The container is
//! `surface` (`ContainerColor`) and animates to `surfaceContainer`
//! (`OnScrollContainerColor`) when content scrolls under it; title
//! `onSurface`, subtitle `onSurfaceVariant`, navigation icon `onSurface`,
//! action icons `onSurfaceVariant` (`TrailingIconColor`), all icons 24 dp
//! in 48 dp targets with a 40 dp circular state layer
//! (`IconButtonTokens.StateLayerSize`). Layout constants from Compose's
//! `AppBar.kt`: `TopAppBarHorizontalPadding` 4, title inset 16,
//! `MediumTitleBottomPadding` 24 and `LargeTitleBottomPadding` 28 (to the
//! title baseline), and `TopTitleAlphaEasing` (0.8, 0, 0.8, 0.15).
//!
//! # Scroll behaviours
//!
//! [`TopAppBarScrollBehavior`] mirrors Compose's `pinnedScrollBehavior`,
//! `enterAlwaysScrollBehavior` and `exitUntilCollapsedScrollBehavior`. The
//! bar reads the content's scroll offset through a hook: pass it with
//! [`MaterialTopAppBar::content_offset`], or name the
//! [`egui::ScrollArea`] with [`MaterialTopAppBar::scroll_area`] and
//! [`crate::scroll_area_id`].
//!
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::{scroll_area_id, MaterialTopAppBar, TopAppBarScrollBehavior};
//!
//! ui.add(
//!     MaterialTopAppBar::large("Inbox")
//!         .id_salt("inbox_bar")
//!         .scroll_behavior(TopAppBarScrollBehavior::ExitUntilCollapsed)
//!         .scroll_area(scroll_area_id(ui, "inbox")),
//! );
//! egui::ScrollArea::vertical().id_salt("inbox").show(ui, |ui| {
//!     for i in 0..100 {
//!         ui.label(format!("Message {i}"));
//!     }
//! });
//! # });
//! ```

use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Shadow},
    text::{LayoutJob, TextWrapping},
    Galley, Id, Pos2, Rect, Response, Role, Sense, Ui, UiBuilder, Vec2, Widget, WidgetInfo,
};
use std::sync::Arc;

use crate::icon::Icon;
use crate::tokens::{
    animate_value_with_spring, ColorRole, CubicBezier, LayoutDirection, SpringSpeed, TypeRole,
};
use crate::toolbar::{
    exit_always, icon_action, overflow_menu, scroll_area_offset, ActionColors, ACTION_TARGET,
};
use crate::MaterialTheme;

/// The height every top app bar collapses to: `TopAppBarSmallTokens`
/// (and Expressive `AppBarSmallTokens`) `ContainerHeight`, 64 dp.
pub const COLLAPSED_HEIGHT: f32 = 64.0;

/// `TopAppBarHorizontalPadding`: between the bar's edges and its icons.
const HORIZONTAL_PADDING: f32 = 4.0;
/// The title's start inset with no navigation icon (16 dp from the edge).
const TITLE_INSET: f32 = 16.0;

/// Material Design top app bar variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopAppBarVariant {
    /// Small: 64 dp, title at the start.
    Regular,
    /// Medium: 112 dp, the title on a second row.
    Medium,
    /// Large: 152 dp, the title on a second row.
    Large,
    /// Small with the title centred.
    CenterAligned,
    /// M3 Expressive medium flexible: 112 dp, 136 dp with a subtitle.
    MediumFlexible,
    /// M3 Expressive large flexible: 120 dp, 152 dp with a subtitle.
    LargeFlexible,
}

/// The type roles and geometry of one variant.
#[derive(Clone, Copy, Debug)]
struct Metrics {
    height: f32,
    title: TypeRole,
    subtitle: Option<TypeRole>,
    /// Bottom padding to the expanded title's baseline; `None` for a
    /// single-row bar.
    bottom_padding: Option<f32>,
}

impl TopAppBarVariant {
    /// Every variant.
    pub const ALL: [TopAppBarVariant; 6] = [
        TopAppBarVariant::Regular,
        TopAppBarVariant::CenterAligned,
        TopAppBarVariant::Medium,
        TopAppBarVariant::Large,
        TopAppBarVariant::MediumFlexible,
        TopAppBarVariant::LargeFlexible,
    ];

    fn metrics(self, with_subtitle: bool) -> Metrics {
        match self {
            TopAppBarVariant::Regular | TopAppBarVariant::CenterAligned => Metrics {
                height: COLLAPSED_HEIGHT,
                title: TypeRole::TitleLarge,
                subtitle: Some(TypeRole::LabelMedium),
                bottom_padding: None,
            },
            TopAppBarVariant::Medium => Metrics {
                height: 112.0,
                title: TypeRole::HeadlineSmall,
                subtitle: None,
                bottom_padding: Some(24.0),
            },
            TopAppBarVariant::Large => Metrics {
                height: 152.0,
                title: TypeRole::HeadlineMedium,
                subtitle: None,
                bottom_padding: Some(28.0),
            },
            TopAppBarVariant::MediumFlexible => Metrics {
                height: if with_subtitle { 136.0 } else { 112.0 },
                title: TypeRole::HeadlineMedium,
                subtitle: Some(TypeRole::LabelLarge),
                bottom_padding: Some(24.0),
            },
            TopAppBarVariant::LargeFlexible => Metrics {
                height: if with_subtitle { 152.0 } else { 120.0 },
                title: TypeRole::DisplaySmall,
                subtitle: Some(TypeRole::TitleMedium),
                bottom_padding: Some(28.0),
            },
        }
    }

    /// The expanded container height, with or without a subtitle. A
    /// subtitle only changes the flexible variants; the baseline medium and
    /// large bars have none and ignore it.
    pub fn expanded_height(self, with_subtitle: bool) -> f32 {
        self.metrics(with_subtitle).height
    }
}

/// How a top app bar reacts to its content scrolling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TopAppBarScrollBehavior {
    /// Stays put; only its colour changes when content scrolls under it
    /// (Compose `pinnedScrollBehavior`).
    #[default]
    Pinned,
    /// Scrolls away with the content and comes back as soon as the content
    /// scrolls back up (Compose `enterAlwaysScrollBehavior`). A small bar
    /// hides entirely; medium and large bars collapse to 64 dp.
    EnterAlways,
    /// Medium and large bars collapse to 64 dp as the content scrolls, and
    /// expand again only when it returns to the top (Compose
    /// `exitUntilCollapsedScrollBehavior`). A small bar stays pinned.
    ExitUntilCollapsed,
}

/// A navigation or action icon.
struct Action<'a> {
    icon: Icon,
    label: String,
    callback: Box<dyn Fn() + Send + Sync + 'a>,
}

/// Material Design top app bar component.
///
/// Top app bars display information and actions related to the current
/// screen: a navigation icon, a title (and on most variants a subtitle),
/// and action icons. Actions that do not fit, or beyond
/// [`max_actions`](Self::max_actions), move into a "More options" menu.
/// The title is cut to one line with an ellipsis rather than overlapping
/// the actions.
///
/// # Examples
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::MaterialTopAppBar;
///
/// let top_bar = MaterialTopAppBar::regular("My App")
///     .navigation("menu", "Open navigation", || println!("Menu clicked!"))
///     .action("search", "Search", || println!("Search clicked!"))
///     .action("more_vert", "More", || println!("More clicked!"));
///
/// ui.add(top_bar);
/// # });
/// ```
///
/// Using material symbol constants:
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::material_symbol::{ICON_MENU, ICON_SEARCH};
/// use egui_material3::MaterialTopAppBar;
///
/// let top_bar = MaterialTopAppBar::regular("My App")
///     .navigation_icon_char(ICON_MENU, || println!("Menu clicked!"))
///     .action_icon_char(ICON_SEARCH, || println!("Search clicked!"));
///
/// ui.add(top_bar);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialTopAppBar<'a> {
    variant: TopAppBarVariant,
    title: String,
    subtitle: Option<String>,
    navigation: Option<Action<'a>>,
    actions: Vec<Action<'a>>,
    height: Option<f32>,
    corner_radius: CornerRadius,
    elevation: Option<Shadow>,
    scrolled: bool,
    id_salt: Option<String>,
    background_color: Option<Color32>,
    foreground_color: Option<Color32>,
    title_spacing: Option<f32>,
    leading_width: Option<f32>,
    scroll_behavior: TopAppBarScrollBehavior,
    content_offset: Option<f32>,
    scroll_area: Option<Id>,
    max_actions: usize,
}

impl<'a> MaterialTopAppBar<'a> {
    /// A small top app bar.
    pub fn regular(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::Regular, title)
    }

    /// A medium top app bar.
    pub fn medium(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::Medium, title)
    }

    /// A large top app bar.
    pub fn large(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::Large, title)
    }

    /// A center-aligned top app bar.
    pub fn center_aligned(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::CenterAligned, title)
    }

    /// An M3 Expressive medium flexible top app bar.
    pub fn medium_flexible(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::MediumFlexible, title)
    }

    /// An M3 Expressive large flexible top app bar.
    pub fn large_flexible(title: impl Into<String>) -> Self {
        Self::new(TopAppBarVariant::LargeFlexible, title)
    }

    /// A top app bar of `variant`.
    pub fn new(variant: TopAppBarVariant, title: impl Into<String>) -> Self {
        Self {
            variant,
            title: title.into(),
            subtitle: None,
            navigation: None,
            actions: Vec::new(),
            height: None,
            corner_radius: CornerRadius::ZERO,
            elevation: None,
            scrolled: false,
            id_salt: None,
            background_color: None,
            foreground_color: None,
            title_spacing: None,
            leading_width: None,
            scroll_behavior: TopAppBarScrollBehavior::Pinned,
            content_offset: None,
            scroll_area: None,
            max_actions: usize::MAX,
        }
    }

    /// Sets the subtitle, shown under the title on the small,
    /// center-aligned and flexible variants (the baseline medium and large
    /// bars have none).
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Sets the navigation icon (a menu or back arrow), named `label` for
    /// screen readers.
    pub fn navigation<F>(
        mut self,
        icon: impl Into<Icon>,
        label: impl Into<String>,
        callback: F,
    ) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.navigation = Some(Action {
            icon: icon.into(),
            label: label.into(),
            callback: Box::new(callback),
        });
        self
    }

    /// Sets the navigation icon by Material Symbols name; the name is its
    /// accessible label. Prefer [`navigation`](Self::navigation).
    pub fn navigation_icon<F>(self, icon: impl Into<String>, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        let icon = icon.into();
        self.navigation(icon.clone(), icon, callback)
    }

    /// Sets the navigation icon by Material Symbols codepoint. Prefer
    /// [`navigation`](Self::navigation), which takes a label.
    pub fn navigation_icon_char<F>(self, icon: char, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.navigation(icon, icon.to_string(), callback)
    }

    /// Adds an action icon, named `label` for screen readers and in the
    /// overflow menu.
    pub fn action<F>(mut self, icon: impl Into<Icon>, label: impl Into<String>, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.actions.push(Action {
            icon: icon.into(),
            label: label.into(),
            callback: Box::new(callback),
        });
        self
    }

    /// Adds an action icon by Material Symbols name; the name is its
    /// label. Prefer [`action`](Self::action).
    pub fn action_icon<F>(self, icon: impl Into<String>, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        let icon = icon.into();
        self.action(icon.clone(), icon, callback)
    }

    /// Adds an action icon by Material Symbols codepoint. Prefer
    /// [`action`](Self::action), which takes a label.
    pub fn action_icon_char<F>(self, icon: char, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.action(icon, icon.to_string(), callback)
    }

    /// Shows at most `count` icons at the end of the bar (the overflow
    /// button counts as one); the rest go into the overflow menu. Actions
    /// also overflow when they would take more than half the bar's width.
    pub fn max_actions(mut self, count: usize) -> Self {
        self.max_actions = count;
        self
    }

    /// Overrides the expanded height.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the container's corner radius.
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    /// Paints `elevation` under the bar while content is scrolled under
    /// it. Off by default: M3 marks the scrolled state with colour only.
    pub fn elevation(mut self, elevation: impl Into<Shadow>) -> Self {
        self.elevation = Some(elevation.into());
        self
    }

    /// Forces the scrolled-under state (container `surfaceContainer`),
    /// whatever the scroll offset.
    pub fn scrolled(mut self, scrolled: bool) -> Self {
        self.scrolled = scrolled;
        self
    }

    /// Sets the id salt. Give each bar one when using a scroll behaviour,
    /// so its scroll state survives layout changes.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Overrides the container colour.
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Overrides the title and navigation icon colour.
    pub fn foreground_color(mut self, color: Color32) -> Self {
        self.foreground_color = Some(color);
        self
    }

    /// Sets the gap between the navigation icon's slot (or the bar's start
    /// edge) and the title.
    pub fn title_spacing(mut self, spacing: f32) -> Self {
        self.title_spacing = Some(spacing);
        self
    }

    /// Sets the width of the navigation icon's slot (default 48).
    pub fn leading_width(mut self, width: f32) -> Self {
        self.leading_width = Some(width);
        self
    }

    /// Has no effect: M3 top app bars change colour, not elevation, when
    /// scrolled. Use [`elevation`](Self::elevation) for a shadow.
    #[deprecated(note = "has no effect; use `elevation` for a scrolled-under shadow")]
    pub fn scrolled_under_elevation(self, _elevation: f32) -> Self {
        self
    }

    /// Has no effect: M3 no longer tints elevated surfaces.
    #[deprecated(note = "has no effect; M3 uses surface-container roles, not tint")]
    pub fn surface_tint_color(self, _color: Color32) -> Self {
        self
    }

    /// Sets how the bar reacts to its content scrolling.
    pub fn scroll_behavior(mut self, behavior: TopAppBarScrollBehavior) -> Self {
        self.scroll_behavior = behavior;
        self
    }

    /// The content's vertical scroll offset (for example
    /// `ScrollAreaOutput::state.clamped_offset().y` from the last frame).
    pub fn content_offset(mut self, offset: f32) -> Self {
        self.content_offset = Some(offset);
        self
    }

    /// Follows the [`egui::ScrollArea`] whose id is `id`: give the scroll
    /// area `.id_salt(salt)` and pass [`crate::scroll_area_id`]`(ui, salt)`
    /// from the same `ui`.
    pub fn scroll_area(mut self, id: Id) -> Self {
        self.scroll_area = Some(id);
        self
    }
}

/// A one-line `role` layout of `text`, cut with an ellipsis at `max_width`.
fn one_line(
    ctx: &egui::Context,
    role: TypeRole,
    text: &str,
    color: Color32,
    max_width: f32,
) -> LayoutJob {
    let mut job = role.style().layout_job(ctx, text, color);
    job.wrap = TextWrapping::truncate_at_width(max_width.max(0.0));
    job
}

/// The baseline of a galley's last row, from its top.
fn last_baseline(galley: &Galley) -> f32 {
    galley
        .rows
        .last()
        .and_then(|row| row.row.glyphs.first().map(|g| row.pos.y + g.pos.y))
        .unwrap_or(galley.size().y * 0.8)
}

/// A title, and maybe a subtitle under it.
struct TitleBlock {
    title: Arc<Galley>,
    subtitle: Option<Arc<Galley>>,
}

impl TitleBlock {
    fn size(&self) -> Vec2 {
        let sub = self.subtitle.as_ref().map_or(Vec2::ZERO, |s| s.size());
        Vec2::new(self.title.size().x.max(sub.x), self.title.size().y + sub.y)
    }

    /// Paints the block with its top at `y`, each line placed by `anchor`.
    fn paint(self, painter: &egui::Painter, anchor: Anchor, y: f32) {
        let place = |width: f32| match anchor {
            Anchor::Start(x) => x,
            Anchor::End(x) => x - width,
            Anchor::Center(x) => x - width / 2.0,
        };
        let title_height = self.title.size().y;
        let title_x = place(self.title.size().x);
        painter.galley(Pos2::new(title_x, y), self.title, Color32::PLACEHOLDER);
        if let Some(sub) = self.subtitle {
            let sub_x = place(sub.size().x);
            painter.galley(
                Pos2::new(sub_x, y + title_height),
                sub,
                Color32::PLACEHOLDER,
            );
        }
    }
}

/// Where the lines of a [`TitleBlock`] go horizontally.
#[derive(Clone, Copy, Debug)]
enum Anchor {
    /// Left edges at x.
    Start(f32),
    /// Right edges at x (right-to-left start).
    End(f32),
    /// Centred on x.
    Center(f32),
}

/// The anchor for text starting at `x` in the given direction.
fn start_anchor(x: f32, rtl: bool) -> Anchor {
    if rtl {
        Anchor::End(x)
    } else {
        Anchor::Start(x)
    }
}

impl Widget for MaterialTopAppBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialTopAppBar {
            variant,
            title,
            subtitle,
            navigation,
            actions,
            height,
            corner_radius,
            elevation,
            scrolled,
            id_salt,
            background_color,
            foreground_color,
            title_spacing,
            leading_width,
            scroll_behavior,
            content_offset,
            scroll_area,
            max_actions,
        } = self;

        let ctx = ui.ctx().clone();
        let theme = MaterialTheme::of(&ctx);
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let id = match &id_salt {
            Some(salt) => ui.make_persistent_id(salt),
            None => ui.next_auto_id(),
        };
        let metrics = variant.metrics(subtitle.is_some());
        let subtitle = subtitle.filter(|_| metrics.subtitle.is_some());
        let two_rows = metrics.bottom_padding.is_some();
        let expanded = height.unwrap_or(metrics.height).max(0.0);
        let collapsed = COLLAPSED_HEIGHT.min(expanded);
        let range = expanded - collapsed;

        // The scroll hook.
        let offset = content_offset
            .or_else(|| scroll_area.map(|area| scroll_area_offset(&ctx, area)))
            .unwrap_or(0.0);
        let hidden = match scroll_behavior {
            TopAppBarScrollBehavior::Pinned => 0.0,
            TopAppBarScrollBehavior::EnterAlways => {
                let limit = if two_rows { range } else { expanded };
                exit_always(&ctx, id, offset, limit)
            }
            TopAppBarScrollBehavior::ExitUntilCollapsed if two_rows => offset.clamp(0.0, range),
            TopAppBarScrollBehavior::ExitUntilCollapsed => 0.0,
        };
        let bar_height = expanded - hidden;
        let collapsed_fraction = if two_rows && range > 0.0 {
            (hidden / range).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Container colour: animated to surfaceContainer when content is
        // under the bar, and following the collapse of a two-row bar
        // (Compose lerps with `FastOutLinearInEasing`).
        const FAST_OUT_LINEAR_IN: CubicBezier = CubicBezier::new(0.4, 0.0, 1.0, 1.0);
        const TOP_TITLE_ALPHA: CubicBezier = CubicBezier::new(0.8, 0.0, 0.8, 0.15);
        let overlapped = scrolled || offset > 0.5;
        let animated = animate_value_with_spring(
            &ctx,
            id.with("container_color"),
            if overlapped { 1.0 } else { 0.0 },
            theme.motion.effects(SpringSpeed::Default),
        );
        let color_fraction = animated
            .max(FAST_OUT_LINEAR_IN.transform(collapsed_fraction))
            .clamp(0.0, 1.0);
        let container = background_color.unwrap_or_else(|| {
            theme
                .color(ColorRole::Surface)
                .lerp_to_gamma(theme.color(ColorRole::SurfaceContainer), color_fraction)
        });
        let title_color = foreground_color.unwrap_or_else(|| theme.color(ColorRole::OnSurface));
        let subtitle_color = theme.color(ColorRole::OnSurfaceVariant);
        let nav_colors = ActionColors::new(
            &theme,
            foreground_color.unwrap_or_else(|| theme.color(ColorRole::OnSurface)),
        );
        let action_colors = ActionColors::new(&theme, theme.color(ColorRole::OnSurfaceVariant));

        let desired = Vec2::new(ui.available_width(), bar_height);
        let mut response = ui.allocate_response(desired, Sense::hover());
        let rect = response.rect;
        {
            let label = title.clone();
            response.widget_info(move || WidgetInfo::labeled(Role::Toolbar, true, label.clone()));
        }
        if !ui.is_rect_visible(rect) || rect.height() <= 0.0 {
            return response;
        }

        // Everything is clipped to the bar, which shrinks as it scrolls.
        let clip = rect.intersect(ui.clip_rect());
        let mut bar_ui = ui.new_child(UiBuilder::new().max_rect(rect).id_salt(id));
        bar_ui.set_clip_rect(clip);

        if let Some(shadow) = elevation {
            if color_fraction > 0.0 {
                let shadow = Shadow {
                    color: shadow.color.gamma_multiply(color_fraction),
                    ..shadow
                };
                bar_ui.painter().add(shadow.as_shape(rect, corner_radius));
            }
        }
        bar_ui.painter().rect_filled(rect, corner_radius, container);

        // The top row: a single-row bar keeps its content at the bottom as
        // it scrolls away; a two-row bar keeps it at the top.
        let row_height = if two_rows { collapsed } else { expanded };
        let row_top = if two_rows {
            rect.min.y
        } else {
            rect.max.y - row_height
        };
        let row = Rect::from_min_size(
            Pos2::new(rect.min.x, row_top),
            Vec2::new(rect.width(), row_height),
        );
        let width = rect.width();
        // x of a span `start..start + w` measured from the start edge.
        let x_at = |start: f32, w: f32| {
            if rtl {
                rect.max.x - start - w
            } else {
                rect.min.x + start
            }
        };
        let target_at = |start: f32| {
            Rect::from_min_size(
                Pos2::new(
                    x_at(start, ACTION_TARGET),
                    row.center().y - ACTION_TARGET / 2.0,
                ),
                Vec2::splat(ACTION_TARGET),
            )
        };

        // Navigation icon.
        let slot = leading_width.unwrap_or(ACTION_TARGET);
        let start_used = match &navigation {
            Some(nav) => {
                let r = icon_action(
                    &mut bar_ui,
                    target_at(HORIZONTAL_PADDING),
                    id.with("navigation"),
                    &nav.icon,
                    &nav.label,
                    nav_colors,
                    None,
                );
                if r.clicked() {
                    (nav.callback)();
                }
                response = response.union(r);
                HORIZONTAL_PADDING + slot
            }
            None => 0.0,
        };
        let title_start = match (title_spacing, navigation.is_some()) {
            (Some(spacing), true) => start_used + spacing,
            (Some(spacing), false) => spacing,
            (None, true) => start_used.max(TITLE_INSET),
            (None, false) => TITLE_INSET,
        };

        // Actions, from the end; the ones that do not fit overflow.
        let fit = ((width / 2.0 - HORIZONTAL_PADDING) / ACTION_TARGET)
            .floor()
            .max(1.0) as usize;
        let slots = max_actions.min(fit).max(1);
        let (shown, overflow) = if actions.len() > slots {
            (slots - 1, true)
        } else {
            (actions.len(), false)
        };
        let (visible, hidden_actions) = actions.split_at(shown.min(actions.len()));
        let icon_count = visible.len() + usize::from(overflow);
        let end_used = if icon_count > 0 {
            HORIZONTAL_PADDING + icon_count as f32 * ACTION_TARGET
        } else {
            0.0
        };
        let mut from_end = HORIZONTAL_PADDING + icon_count as f32 * ACTION_TARGET;
        for (index, action) in visible.iter().enumerate() {
            from_end -= ACTION_TARGET;
            let start = width - from_end - ACTION_TARGET;
            let r = icon_action(
                &mut bar_ui,
                target_at(start),
                id.with(("action", index)),
                &action.icon,
                &action.label,
                action_colors,
                None,
            );
            if r.clicked() {
                (action.callback)();
            }
            response = response.union(r);
        }
        if overflow {
            let start = width - HORIZONTAL_PADDING - ACTION_TARGET;
            let more = Icon::from("more_vert");
            let r = icon_action(
                &mut bar_ui,
                target_at(start),
                id.with("overflow"),
                &more,
                "More options",
                action_colors,
                None,
            );
            let items: Vec<(&Icon, &str)> = hidden_actions
                .iter()
                .map(|a| (&a.icon, a.label.as_str()))
                .collect();
            if let Some(action) = overflow_menu(&r, &items).and_then(|i| hidden_actions.get(i)) {
                (action.callback)();
            }
            response = response.union(r);
        }

        // The title in the top row: always for a single-row bar; fading in
        // as a two-row bar collapses.
        let top_alpha = if two_rows {
            TOP_TITLE_ALPHA.transform(collapsed_fraction)
        } else {
            1.0
        };
        if top_alpha > 0.0 {
            let centered = variant == TopAppBarVariant::CenterAligned;
            let max_width = if centered {
                width - 2.0 * title_start.max(end_used)
            } else {
                width - title_start - end_used
            };
            let block = TitleBlock {
                title: bar_ui.painter().layout_job(one_line(
                    &ctx,
                    TypeRole::TitleLarge,
                    &title,
                    title_color.gamma_multiply(top_alpha),
                    max_width,
                )),
                subtitle: subtitle.as_ref().map(|s| {
                    bar_ui.painter().layout_job(one_line(
                        &ctx,
                        TypeRole::LabelMedium,
                        s,
                        subtitle_color.gamma_multiply(top_alpha),
                        max_width,
                    ))
                }),
            };
            let size = block.size();
            let y = row.center().y - size.y / 2.0;
            let anchor = if centered {
                Anchor::Center(rect.center().x)
            } else if rtl {
                Anchor::End(rect.max.x - title_start)
            } else {
                Anchor::Start(rect.min.x + title_start)
            };
            block.paint(bar_ui.painter(), anchor, y);
        }

        // The expanded title row of a two-row bar.
        if let Some(bottom_padding) = metrics.bottom_padding {
            let bottom_alpha = 1.0 - collapsed_fraction;
            if bottom_alpha > 0.0 {
                let inset = title_spacing.unwrap_or(TITLE_INSET);
                let max_width = width - 2.0 * inset;
                let block = TitleBlock {
                    title: bar_ui.painter().layout_job(one_line(
                        &ctx,
                        metrics.title,
                        &title,
                        title_color.gamma_multiply(bottom_alpha),
                        max_width,
                    )),
                    subtitle: subtitle.as_ref().zip(metrics.subtitle).map(|(s, role)| {
                        bar_ui.painter().layout_job(one_line(
                            &ctx,
                            role,
                            s,
                            subtitle_color.gamma_multiply(bottom_alpha),
                            max_width,
                        ))
                    }),
                };
                // The last line's baseline sits `bottom_padding` above the
                // bottom (Compose places the title by its baseline).
                let last = block.subtitle.as_ref().unwrap_or(&block.title);
                let above_last = block.size().y - last.size().y;
                let y = rect.max.y - bottom_padding - last_baseline(last) - above_last;
                let x = if rtl {
                    rect.max.x - inset
                } else {
                    rect.min.x + inset
                };
                let row_clip =
                    Rect::from_min_max(Pos2::new(rect.min.x, row.max.y), rect.max).intersect(clip);
                // Let the title reach a little above the second row, so a
                // tall title with a subtitle is not cut while expanded.
                let row_clip = row_clip.with_min_y(row_clip.min.y - 8.0);
                block.paint(
                    &bar_ui.painter().with_clip_rect(row_clip),
                    start_anchor(x, rtl),
                    y,
                );
            }
        }

        response
    }
}

/// [`MaterialTopAppBar::regular`].
pub fn top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::regular(title)
}

/// [`MaterialTopAppBar::center_aligned`].
pub fn center_aligned_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::center_aligned(title)
}

/// [`MaterialTopAppBar::medium`].
pub fn medium_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::medium(title)
}

/// [`MaterialTopAppBar::large`].
pub fn large_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::large(title)
}

/// [`MaterialTopAppBar::medium_flexible`].
pub fn medium_flexible_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::medium_flexible(title)
}

/// [`MaterialTopAppBar::large_flexible`].
pub fn large_flexible_top_app_bar(title: impl Into<String>) -> MaterialTopAppBar<'static> {
    MaterialTopAppBar::large_flexible(title)
}
