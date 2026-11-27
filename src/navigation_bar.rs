// ============================================================================
// Module:       navigation_bar
// Description:  Material 3 navigation bar: three to five destinations along
//               the bottom of a compact window.
//
// Dependencies: egui; crate::navigation, crate::tokens, crate::paint
// ============================================================================

//! [`NavigationBar`]: the bottom bar of a compact window.
//!
//! Values from the Jetpack Compose material3 token files
//! (`androidx.compose.material3.tokens`, `androidx-main`):
//!
//! - `NavigationBarTokens`: container `surfaceContainer`, 80 dp
//!   (`TallContainerHeight`, the baseline bar) or 64 dp (`ContainerHeight`,
//!   the Expressive flexible bar), 24 dp icons, indicator `secondaryContainer`
//!   with `CornerFull`, active icon `onSecondaryContainer`, active label
//!   `secondary`, inactive icon and label `onSurfaceVariant`.
//! - `NavigationBarVerticalItemTokens`: a 56 × 32 dp indicator, 4 dp between
//!   indicator and label, `LabelMedium`. (Older releases of the baseline
//!   tokens had a 64 dp wide indicator; `androidx-main` uses 56 dp for both
//!   the baseline and the Expressive bar, and so does this.)
//! - `NavigationBarHorizontalItemTokens`: the Expressive horizontal item for
//!   medium windows, a 40 dp indicator with 16 dp leading and trailing space
//!   and 4 dp between icon and label.
//! - `NavigationRailColorTokens.Item*StateLayer`: `onSecondaryContainer`
//!   state layers (the bar's token file has none; the rail's are used).
//!
//! The indicator grows from the centre when an item is selected (Compose's
//! `FastSpatial` spring for size, `DefaultEffects` for opacity).

use egui::{Color32, CornerRadius, Id, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};

use crate::navigation::{
    describe_item, paint_badge, paint_icon, rove_focus, text_galley, NavigationItem, RoveAxis,
};
use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{
    animate_value_with_spring, ColorRole, InteractionState, LayoutDirection, SpringSpeed, TypeRole,
};
use crate::MaterialTheme;

/// `NavigationBarTokens.TallContainerHeight`: the baseline bar.
pub const BAR_HEIGHT: f32 = 80.0;
/// `NavigationBarTokens.ContainerHeight`: the Expressive flexible bar.
pub const FLEXIBLE_BAR_HEIGHT: f32 = 64.0;
/// `NavigationBarTokens.ItemIconSize`.
pub const ICON_SIZE: f32 = 24.0;
/// `NavigationBarVerticalItemTokens.ActiveIndicatorWidth`.
pub const INDICATOR_WIDTH: f32 = 56.0;
/// `NavigationBarVerticalItemTokens.ActiveIndicatorHeight`.
pub const INDICATOR_HEIGHT: f32 = 32.0;
/// `NavigationBarHorizontalItemTokens.ActiveIndicatorHeight`.
pub const HORIZONTAL_INDICATOR_HEIGHT: f32 = 40.0;
/// `NavigationBarVerticalItemTokens.ActiveIndicatorIconLabelSpace`.
const INDICATOR_LABEL_SPACE: f32 = 4.0;
/// `NavigationBarHorizontalItemTokens.ActiveIndicatorLeadingSpace` and
/// `TrailingSpace`.
const HORIZONTAL_EDGE_SPACE: f32 = 16.0;
/// `NavigationBarHorizontalItemTokens.ActiveIndicatorIconLabelSpace`.
const HORIZONTAL_ICON_LABEL_SPACE: f32 = 4.0;

/// The kind of navigation bar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum NavigationBarVariant {
    /// The baseline 80 dp bar: icon above label.
    #[default]
    Baseline,
    /// The Expressive 64 dp flexible bar: icon above label.
    Flexible,
    /// The Expressive 64 dp flexible bar with the icon beside the label,
    /// for medium-width windows.
    FlexibleHorizontal,
}

impl NavigationBarVariant {
    /// The bar height.
    pub const fn height(self) -> f32 {
        match self {
            NavigationBarVariant::Baseline => BAR_HEIGHT,
            NavigationBarVariant::Flexible | NavigationBarVariant::FlexibleHorizontal => {
                FLEXIBLE_BAR_HEIGHT
            }
        }
    }
}

/// A Material 3 navigation bar.
///
/// Fills the available width and splits it equally between its items.
/// Clicking an item selects it; the response is `changed()` then. Tab
/// reaches the bar, and the arrow keys move between its items.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{NavigationBadge, NavigationBar, NavigationItem};
/// let mut selected = 0;
/// ui.add(
///     NavigationBar::new(&mut selected)
///         .item(NavigationItem::new("home", "Home"))
///         .item(NavigationItem::new("mail", "Mail").badge(NavigationBadge::Large("3".into())))
///         .item(NavigationItem::new("settings", "Settings")),
/// );
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct NavigationBar<'a> {
    selected: &'a mut usize,
    items: Vec<NavigationItem>,
    variant: NavigationBarVariant,
    id_salt: Option<Id>,
    container_color: Option<Color32>,
}

impl<'a> NavigationBar<'a> {
    /// A bar whose selected item index is `selected`.
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
            variant: NavigationBarVariant::default(),
            id_salt: None,
            container_color: None,
        }
    }

    /// Adds a destination.
    pub fn item(mut self, item: NavigationItem) -> Self {
        self.items.push(item);
        self
    }

    /// The kind of bar.
    pub fn variant(mut self, variant: NavigationBarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Salt for the bar's id, when its position in the ui is not stable.
    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.id_salt = Some(Id::unique(salt));
        self
    }

    /// A container colour instead of `surfaceContainer`.
    pub fn container_color(mut self, color: Color32) -> Self {
        self.container_color = Some(color);
        self
    }
}

impl Widget for NavigationBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let height = self.variant.height();
        let (rect, mut response) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
        let id = self.id_salt.unwrap_or(response.id);
        let painter = ui.painter_at(rect);
        painter.rect_filled(
            rect,
            CornerRadius::ZERO,
            self.container_color
                .unwrap_or_else(|| theme.color(ColorRole::SurfaceContainer)),
        );

        let count = self.items.len();
        let item_width = rect.width() / count.max(1) as f32;
        let mut ids = Vec::with_capacity(count);
        let mut changed = false;
        for (index, item) in self.items.iter().enumerate() {
            let slot = if rtl { count - 1 - index } else { index };
            let item_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + slot as f32 * item_width, rect.min.y),
                Vec2::new(item_width, height),
            );
            let item_id = id.with(("navigation_bar_item", index));
            let item_response = ui.interact(
                item_rect,
                item_id,
                crate::a11y::sense(item.enabled, Sense::click()),
            );
            if item_response.clicked() && *self.selected != index {
                *self.selected = index;
                changed = true;
            }
            let selected = *self.selected == index;
            describe_item(&item_response, item.enabled, &item.label, selected);
            paint_item(
                &painter,
                &theme,
                &item_response,
                item,
                selected,
                self.variant,
                rtl,
            );
            ids.push((item_id, item.enabled));
        }
        rove_focus(ui, &ids, RoveAxis::Horizontal, rtl);
        if changed {
            response.mark_changed();
        }
        response
    }
}

/// Paints one item: indicator, state layer, icon, badge, label and focus
/// ring.
fn paint_item(
    painter: &egui::Painter,
    theme: &MaterialTheme,
    response: &Response,
    item: &NavigationItem,
    selected: bool,
    variant: NavigationBarVariant,
    rtl: bool,
) {
    let ctx = &response.ctx;
    let rect = response.rect;
    let target = if selected { 1.0 } else { 0.0 };
    let size_progress = animate_value_with_spring(
        ctx,
        response.id.with("indicator_size"),
        target,
        theme.motion.spatial(SpringSpeed::Fast),
    )
    .max(0.0);
    let alpha_progress = animate_value_with_spring(
        ctx,
        response.id.with("indicator_alpha"),
        target,
        theme.motion.effects(SpringSpeed::Default),
    )
    .clamp(0.0, 1.0);

    let disabled = |c: Color32| c.gamma_multiply(DISABLED_CONTENT_OPACITY);
    let on_surface = theme.color(ColorRole::OnSurface);
    let (icon_color, label_color) = match (item.enabled, selected) {
        (false, _) => (disabled(on_surface), disabled(on_surface)),
        (true, true) => (
            theme.color(ColorRole::OnSecondaryContainer),
            theme.color(ColorRole::Secondary),
        ),
        (true, false) => (
            theme.color(ColorRole::OnSurfaceVariant),
            theme.color(ColorRole::OnSurfaceVariant),
        ),
    };
    let label = text_galley(painter, TypeRole::LabelMedium, &item.label, label_color);
    let (icon, filled) = item.icon_for(selected);

    // The indicator's full rect and where the icon and label go.
    let (indicator, icon_center, label_pos) = match variant {
        NavigationBarVariant::Baseline | NavigationBarVariant::Flexible => {
            let content_height = INDICATOR_HEIGHT + INDICATOR_LABEL_SPACE + label.size().y;
            let top = rect.min.y + ((rect.height() - content_height) / 2.0).max(4.0);
            let indicator = Rect::from_center_size(
                Pos2::new(rect.center().x, top + INDICATOR_HEIGHT / 2.0),
                Vec2::new(INDICATOR_WIDTH, INDICATOR_HEIGHT),
            );
            let label_pos = Pos2::new(
                rect.center().x - label.size().x / 2.0,
                indicator.max.y + INDICATOR_LABEL_SPACE,
            );
            (indicator, indicator.center(), label_pos)
        }
        NavigationBarVariant::FlexibleHorizontal => {
            let width = HORIZONTAL_EDGE_SPACE * 2.0
                + ICON_SIZE
                + HORIZONTAL_ICON_LABEL_SPACE
                + label.size().x;
            let indicator = Rect::from_center_size(
                rect.center(),
                Vec2::new(width.min(rect.width()), HORIZONTAL_INDICATOR_HEIGHT),
            );
            let icon_x = if rtl {
                indicator.max.x - HORIZONTAL_EDGE_SPACE - ICON_SIZE / 2.0
            } else {
                indicator.min.x + HORIZONTAL_EDGE_SPACE + ICON_SIZE / 2.0
            };
            let label_x = if rtl {
                icon_x - ICON_SIZE / 2.0 - HORIZONTAL_ICON_LABEL_SPACE - label.size().x
            } else {
                icon_x + ICON_SIZE / 2.0 + HORIZONTAL_ICON_LABEL_SPACE
            };
            let icon_center = Pos2::new(icon_x, indicator.center().y);
            let label_pos = Pos2::new(label_x, indicator.center().y - label.size().y / 2.0);
            (indicator, icon_center, label_pos)
        }
    };
    let pill = CornerRadius::same((indicator.height() / 2.0).round().clamp(0.0, 255.0) as u8);

    if size_progress > 0.0 && alpha_progress > 0.0 {
        let grown = Rect::from_center_size(
            indicator.center(),
            Vec2::new(indicator.width() * size_progress, indicator.height()),
        );
        painter.rect_filled(
            grown,
            pill,
            theme
                .color(ColorRole::SecondaryContainer)
                .gamma_multiply(alpha_progress),
        );
    }
    crate::paint::paint_state_layer(
        painter,
        indicator,
        pill,
        theme.color(ColorRole::OnSecondaryContainer),
        InteractionState::from_response(response),
    );
    let icon_rect = paint_icon(painter, icon_center, icon, ICON_SIZE, icon_color, filled);
    if let Some(badge) = &item.badge {
        paint_badge(painter, theme, icon_rect, badge, !rtl);
    }
    painter.galley(label_pos, label, label_color);
    if response.has_focus() {
        crate::paint::paint_focus_ring(
            &response.ctx.layer_painter(painter.layer_id()),
            indicator,
            pill,
            theme.color(ColorRole::Secondary),
        );
    }
}
