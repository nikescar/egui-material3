// ============================================================================
// Module:       navigation_rail
// Description:  Material 3 navigation rail: collapsed, expanded and modal
//               expanded, with a header slot for a menu button or FAB.
//
// Dependencies: egui; crate::navigation, crate::tokens, crate::paint
// ============================================================================

//! [`NavigationRail`]: destinations along the start edge of a medium or
//! expanded window. This is the M3 Expressive rail (Compose's
//! `WideNavigationRail`); the baseline rail is its collapsed state.
//!
//! Values from the Jetpack Compose material3 token files
//! (`androidx.compose.material3.tokens`, `androidx-main`):
//!
//! - `NavigationRailCollapsedTokens`: `surface`, 96 dp wide, 44 dp
//!   `TopSpace`, 4 dp `ItemVerticalSpace`.
//! - `NavigationRailExpandedTokens`: 220–360 dp wide, `surface`; modal
//!   `surfaceContainer`, `CornerLarge`, `Level2`.
//! - `NavigationRailBaselineItemTokens`: 64 dp item height, 24 dp icons,
//!   `CornerFull` indicator, 40 dp `HeaderSpaceMinimum`.
//! - `NavigationRailVerticalItemTokens` (collapsed items): 56 × 32 dp
//!   indicator, 4 dp between indicator and label, `LabelMedium`.
//! - `NavigationRailHorizontalItemTokens` (expanded items): 56 dp indicator,
//!   16 dp leading and trailing space, 8 dp between icon and label,
//!   `LabelLarge`; `WideNavigationRail.kt` insets items 20 dp.
//! - `NavigationRailColorTokens`: indicator `secondaryContainer`, active icon
//!   `onSecondaryContainer`, active label `secondary`, inactive
//!   `onSurfaceVariant`, state layers `onSecondaryContainer`.
//! - `ScrimTokens`: the modal scrim is `scrim` at 32 %.

use egui::{
    Area, Color32, CornerRadius, Id, Key, Order, Pos2, Rect, Response, Sense, Ui, UiBuilder, Vec2,
    Widget,
};

use crate::navigation::{
    describe_item, modal_scrim, paint_badge, paint_icon, rove_focus, scrim_color, text_galley,
    NavigationItem, RoveAxis,
};
use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{
    animate_value_with_spring, ColorRole, Elevation, InteractionState, LayoutDirection, Shape,
    SpringSpeed, TypeRole,
};
use crate::MaterialTheme;

/// `NavigationRailCollapsedTokens.ContainerWidth`.
pub const COLLAPSED_WIDTH: f32 = 96.0;
/// `NavigationRailExpandedTokens.ContainerWidthMinimum`.
pub const EXPANDED_MIN_WIDTH: f32 = 220.0;
/// `NavigationRailExpandedTokens.ContainerWidthMaximum`.
pub const EXPANDED_MAX_WIDTH: f32 = 360.0;
/// `NavigationRailCollapsedTokens.TopSpace` (and the expanded `TopSpace`).
const TOP_SPACE: f32 = 44.0;
/// `NavigationRailBaselineItemTokens.HeaderSpaceMinimum`.
const HEADER_SPACE: f32 = 40.0;
/// `NavigationRailCollapsedTokens.ItemVerticalSpace`.
const ITEM_VERTICAL_SPACE: f32 = 4.0;
/// `NavigationRailBaselineItemTokens.ContainerHeight`: a collapsed item.
pub const ITEM_HEIGHT: f32 = 64.0;
/// `NavigationRailBaselineItemTokens.IconSize`.
pub const ICON_SIZE: f32 = 24.0;
/// `NavigationRailVerticalItemTokens.ActiveIndicatorWidth`.
pub const INDICATOR_WIDTH: f32 = 56.0;
/// `NavigationRailVerticalItemTokens.ActiveIndicatorHeight`.
pub const INDICATOR_HEIGHT: f32 = 32.0;
/// `NavigationRailHorizontalItemTokens.ActiveIndicatorHeight`: an expanded
/// item.
pub const EXPANDED_INDICATOR_HEIGHT: f32 = 56.0;
/// `NavigationRailVerticalItemTokens.IconLabelSpace`.
const INDICATOR_LABEL_SPACE: f32 = 4.0;
/// `NavigationRailHorizontalItemTokens.FullWidthLeadingSpace` / `TrailingSpace`.
const EXPANDED_EDGE_SPACE: f32 = 16.0;
/// `NavigationRailHorizontalItemTokens.IconLabelSpace`.
const EXPANDED_ICON_LABEL_SPACE: f32 = 8.0;
/// `WideNavigationRail.kt` `WNRItemHorizontalPadding`.
const EXPANDED_ITEM_INSET: f32 = 20.0;

/// The header slot: a menu button that toggles the rail, a FAB, or both.
/// Called with the header's ui, once per rail drawn (twice in a frame
/// while a modal rail is open: the collapsed rail and the overlay).
pub type RailHeader<'a> = Box<dyn FnMut(&mut Ui) + 'a>;

/// A Material 3 navigation rail.
///
/// Takes the full available height. Collapsed it is 96 dp wide with the
/// labels under the icons; expanded it is 220–360 dp with the labels beside
/// them. A modal rail stays collapsed in the layout and opens over the page
/// with a scrim; clicking the scrim or pressing Escape collapses it.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{NavigationItem, NavigationRail};
/// let mut selected = 0;
/// let mut expanded = false;
/// ui.add(
///     NavigationRail::new(&mut selected)
///         .expanded(&mut expanded)
///         .item(NavigationItem::new("inbox", "Inbox"))
///         .item(NavigationItem::new("send", "Sent")),
/// );
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct NavigationRail<'a> {
    selected: &'a mut usize,
    items: Vec<NavigationItem>,
    expanded: Option<&'a mut bool>,
    modal: bool,
    expanded_width: f32,
    header: Option<RailHeader<'a>>,
    id_salt: Option<Id>,
}

impl<'a> NavigationRail<'a> {
    /// A rail whose selected item index is `selected`.
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
            expanded: None,
            modal: false,
            expanded_width: EXPANDED_MIN_WIDTH,
            header: None,
            id_salt: None,
        }
    }

    /// Adds a destination.
    pub fn item(mut self, item: NavigationItem) -> Self {
        self.items.push(item);
        self
    }

    /// Binds the expanded state. Without it the rail is always collapsed.
    pub fn expanded(mut self, expanded: &'a mut bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    /// Opens the expanded rail over the page, with a scrim, instead of
    /// pushing the page aside.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// The expanded width, clamped to 220–360 dp.
    pub fn expanded_width(mut self, width: f32) -> Self {
        self.expanded_width = width.clamp(EXPANDED_MIN_WIDTH, EXPANDED_MAX_WIDTH);
        self
    }

    /// Content above the items: a menu button, a FAB, or both.
    pub fn header(mut self, header: impl FnMut(&mut Ui) + 'a) -> Self {
        self.header = Some(Box::new(header));
        self
    }

    /// Salt for the rail's id, when its position in the ui is not stable.
    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.id_salt = Some(Id::unique(salt));
        self
    }
}

/// What one pass of drawing the rail's contents needs.
struct RailPass<'r> {
    theme: &'r MaterialTheme,
    id: Id,
    items: &'r [NavigationItem],
    selected: usize,
    /// Labels beside the icons rather than under them.
    horizontal: bool,
    rtl: bool,
    /// The modal overlay rather than the rail in the layout.
    overlay: bool,
}

impl Widget for NavigationRail<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let id = self.id_salt.unwrap_or_else(|| ui.next_auto_id());
        let is_expanded = self.expanded.as_deref().copied().unwrap_or(false);
        let inline_expanded = is_expanded && !self.modal;
        let target_width = if inline_expanded {
            self.expanded_width
        } else {
            COLLAPSED_WIDTH
        };
        let width = animate_value_with_spring(
            ui.ctx(),
            id.with("width"),
            target_width,
            theme.motion.spatial(SpringSpeed::Default),
        )
        .clamp(COLLAPSED_WIDTH, EXPANDED_MAX_WIDTH);
        let (rect, mut response) =
            ui.allocate_exact_size(Vec2::new(width, ui.available_height()), Sense::hover());
        ui.painter()
            .rect_filled(rect, CornerRadius::ZERO, theme.color(ColorRole::Surface));

        let mut changed = false;
        let pass = RailPass {
            theme: &theme,
            id,
            items: &self.items,
            selected: *self.selected,
            // Switch to side labels once there is room for them.
            horizontal: width >= EXPANDED_MIN_WIDTH,
            rtl,
            overlay: false,
        };
        let modal_open = self.modal && is_expanded;
        let mut child = ui.new_child(UiBuilder::new().id_salt("rail").max_rect(rect));
        if let Some(index) = pass.show(&mut child, rect, &mut self.header, !modal_open) {
            *self.selected = index;
            changed = true;
        }

        if modal_open {
            let ctx = ui.ctx().clone();
            let scrim = modal_scrim(&ctx, id.with("scrim"), scrim_color(&theme, 1.0));
            let close = scrim.clicked() || ctx.input_mut(|i| i.key_pressed(Key::Escape));
            let screen = ctx.content_rect();
            let overlay_width = self.expanded_width;
            let x = if rtl {
                screen.max.x - overlay_width
            } else {
                screen.min.x
            };
            let overlay = Rect::from_min_size(
                Pos2::new(x, screen.min.y),
                Vec2::new(overlay_width, screen.height()),
            );
            let layer_id = id.with("modal");
            ctx.move_to_top(egui::LayerId::new(Order::Foreground, layer_id));
            let pass = RailPass {
                horizontal: true,
                overlay: true,
                selected: *self.selected,
                ..pass
            };
            let clicked = Area::new(layer_id)
                .order(Order::Foreground)
                .fade_in(false)
                .fixed_pos(overlay.min)
                .show(&ctx, |ui| {
                    let radius = Shape::LARGE.corner_radius(overlay, theme.layout_direction);
                    crate::paint::paint_elevation(
                        ui.painter(),
                        overlay,
                        radius,
                        Elevation::Level2,
                        theme.color(ColorRole::Shadow),
                    );
                    ui.painter().rect_filled(
                        overlay,
                        radius,
                        theme.color(ColorRole::SurfaceContainer),
                    );
                    // The overlay takes the pointer over its whole area.
                    ui.allocate_rect(overlay, Sense::click());
                    let mut child =
                        ui.new_child(UiBuilder::new().id_salt("modal_rail").max_rect(overlay));
                    pass.show(&mut child, overlay, &mut self.header, true)
                })
                .inner;
            if let Some(index) = clicked {
                *self.selected = index;
                changed = true;
            }
            if close || clicked.is_some() {
                if let Some(expanded) = self.expanded.as_deref_mut() {
                    *expanded = false;
                }
            }
        }

        if changed {
            response.mark_changed();
        }
        response
    }
}

impl RailPass<'_> {
    /// Draws the header and items into `rect`; returns the item clicked.
    fn show(
        &self,
        ui: &mut Ui,
        rect: Rect,
        header: &mut Option<RailHeader<'_>>,
        interactive: bool,
    ) -> Option<usize> {
        let mut y = rect.min.y + TOP_SPACE;
        if let Some(header) = header.as_mut() {
            let header_rect = Rect::from_min_max(Pos2::new(rect.min.x, y), rect.max);
            let mut header_ui = ui.new_child(
                UiBuilder::new()
                    .id_salt("header")
                    .max_rect(header_rect)
                    .layout(egui::Layout::top_down(egui::Align::Center)),
            );
            header(&mut header_ui);
            y = header_ui.min_rect().max.y + HEADER_SPACE;
        }

        let mut clicked = None;
        let mut ids = Vec::with_capacity(self.items.len());
        for (index, item) in self.items.iter().enumerate() {
            let height = if self.horizontal {
                EXPANDED_INDICATOR_HEIGHT
            } else {
                ITEM_HEIGHT
            };
            let item_rect =
                Rect::from_min_size(Pos2::new(rect.min.x, y), Vec2::new(rect.width(), height));
            y += height + ITEM_VERTICAL_SPACE;
            let item_id = self.id.with(("navigation_rail_item", self.overlay, index));
            let sense = if interactive {
                crate::a11y::sense(item.enabled, Sense::click())
            } else {
                Sense::hover()
            };
            let response = ui.interact(item_rect, item_id, sense);
            let selected = self.selected == index;
            if interactive {
                describe_item(&response, item.enabled, &item.label, selected);
                if response.clicked() && self.selected != index {
                    clicked = Some(index);
                }
                ids.push((item_id, item.enabled));
            }
            self.paint_item(ui, &response, item, selected);
        }
        rove_focus(ui, &ids, RoveAxis::Vertical, self.rtl);
        clicked
    }

    fn paint_item(&self, ui: &Ui, response: &Response, item: &NavigationItem, selected: bool) {
        let theme = self.theme;
        let painter = ui.painter();
        let rect = response.rect;
        let disabled = |c: Color32| c.gamma_multiply(DISABLED_CONTENT_OPACITY);
        let (icon_color, label_color) = match (item.enabled, selected) {
            (false, _) => {
                let c = disabled(theme.color(ColorRole::OnSurface));
                (c, c)
            }
            (true, true) => (
                theme.color(ColorRole::OnSecondaryContainer),
                theme.color(ColorRole::Secondary),
            ),
            (true, false) => (
                theme.color(ColorRole::OnSurfaceVariant),
                theme.color(ColorRole::OnSurfaceVariant),
            ),
        };
        let (icon, filled) = item.icon_for(selected);
        let (indicator, icon_center, label, label_pos) = if self.horizontal {
            let label = text_galley(painter, TypeRole::LabelLarge, &item.label, label_color);
            let width = (EXPANDED_EDGE_SPACE * 2.0
                + ICON_SIZE
                + EXPANDED_ICON_LABEL_SPACE
                + label.size().x)
                .min(rect.width() - EXPANDED_ITEM_INSET * 2.0);
            let x = if self.rtl {
                rect.max.x - EXPANDED_ITEM_INSET - width
            } else {
                rect.min.x + EXPANDED_ITEM_INSET
            };
            let indicator = Rect::from_min_size(
                Pos2::new(x, rect.min.y),
                Vec2::new(width, EXPANDED_INDICATOR_HEIGHT),
            );
            let icon_x = if self.rtl {
                indicator.max.x - EXPANDED_EDGE_SPACE - ICON_SIZE / 2.0
            } else {
                indicator.min.x + EXPANDED_EDGE_SPACE + ICON_SIZE / 2.0
            };
            let label_x = if self.rtl {
                icon_x - ICON_SIZE / 2.0 - EXPANDED_ICON_LABEL_SPACE - label.size().x
            } else {
                icon_x + ICON_SIZE / 2.0 + EXPANDED_ICON_LABEL_SPACE
            };
            let label_pos = Pos2::new(label_x, indicator.center().y - label.size().y / 2.0);
            (
                indicator,
                Pos2::new(icon_x, indicator.center().y),
                label,
                label_pos,
            )
        } else {
            let label = text_galley(painter, TypeRole::LabelMedium, &item.label, label_color);
            let content = INDICATOR_HEIGHT + INDICATOR_LABEL_SPACE + label.size().y;
            let top = rect.min.y + ((rect.height() - content) / 2.0).max(0.0);
            let indicator = Rect::from_center_size(
                Pos2::new(rect.center().x, top + INDICATOR_HEIGHT / 2.0),
                Vec2::new(INDICATOR_WIDTH, INDICATOR_HEIGHT),
            );
            let label_pos = Pos2::new(
                rect.center().x - label.size().x / 2.0,
                indicator.max.y + INDICATOR_LABEL_SPACE,
            );
            (indicator, indicator.center(), label, label_pos)
        };
        let pill = CornerRadius::same((indicator.height() / 2.0).round().clamp(0.0, 255.0) as u8);
        let progress = animate_value_with_spring(
            &response.ctx,
            self.id.with(("indicator", item.label.as_str())),
            if selected { 1.0 } else { 0.0 },
            theme.motion.effects(SpringSpeed::Default),
        )
        .clamp(0.0, 1.0);
        if progress > 0.0 {
            painter.rect_filled(
                indicator,
                pill,
                theme
                    .color(ColorRole::SecondaryContainer)
                    .gamma_multiply(progress),
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
            paint_badge(painter, theme, icon_rect, badge, !self.rtl);
        }
        painter.galley(label_pos, label, label_color);
        if response.has_focus() {
            crate::paint::paint_focus_ring(
                painter,
                indicator,
                pill,
                theme.color(ColorRole::Secondary),
            );
        }
    }
}
