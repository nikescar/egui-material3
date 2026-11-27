// ============================================================================
// Module:       navigation
// Description:  Items, badges, icons, keyboard roving and the modal scrim
//               shared by the navigation bar, rail, drawer and tabs.
//
// Dependencies: egui; crate::tokens, crate::a11y, crate::material_symbol
// ============================================================================

//! What the four navigation components share: the [`NavigationItem`] that
//! the bar and rail are built from, [`NavigationBadge`], and the crate's
//! helpers for icons, badges, arrow-key roving between items, the AccessKit
//! selected state, and the modal scrim.
//!
//! Values from the Jetpack Compose material3 token files
//! (`androidx.compose.material3.tokens`, `androidx-main`): `BadgeTokens`,
//! `ScrimTokens`, and the badge offsets in `Badge.kt`.

use egui::epaint::text::VariationCoords;
use egui::text::{LayoutJob, TextFormat};
use egui::{
    Align2, Area, Color32, Context, CornerRadius, EventFilter, FontId, Id, Key, Modifiers, Order,
    Painter, Pos2, Rect, Response, Role, Sense, Ui, Vec2,
};

use crate::tokens::{ColorRole, TypeRole};
use crate::MaterialTheme;

/// `ScrimTokens.ContainerOpacity`: the scrim is the `scrim` role at 32 %.
pub const SCRIM_OPACITY: f32 = 0.32;

/// `BadgeTokens.Size`: the small (dot) badge diameter.
pub const BADGE_SMALL_SIZE: f32 = 6.0;
/// `BadgeTokens.LargeSize`: the large (text) badge height and minimum width.
pub const BADGE_LARGE_SIZE: f32 = 16.0;
/// `Badge.kt` `BadgeWithContentHorizontalPadding`: text inset in a large badge.
const BADGE_LARGE_PADDING: f32 = 4.0;
/// `Badge.kt` `BadgeWithContentHorizontalOffset`: a large badge starts this far
/// before the anchor's end edge.
const BADGE_LARGE_HORIZONTAL_OFFSET: f32 = 12.0;
/// `Badge.kt` `BadgeWithContentVerticalOffset`: a large badge overlaps the
/// anchor's top by this much.
const BADGE_LARGE_VERTICAL_OFFSET: f32 = 14.0;
/// `Badge.kt` `BadgeOffset`: a small badge's offset from the anchor's top end.
const BADGE_SMALL_OFFSET: f32 = 6.0;

/// A badge on a navigation item or tab.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavigationBadge {
    /// A 6 dp dot (`BadgeTokens.Size`), for "something new".
    Small,
    /// A 16 dp pill with a short label (`BadgeTokens.LargeSize`), such as a
    /// count. Keep it to about four characters.
    Large(String),
}

/// One destination in a [`crate::NavigationBar`] or [`crate::NavigationRail`].
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationItem {
    pub(crate) icon: String,
    pub(crate) selected_icon: Option<String>,
    pub(crate) label: String,
    pub(crate) enabled: bool,
    pub(crate) badge: Option<NavigationBadge>,
}

impl NavigationItem {
    /// A destination with a Material Symbols icon name (such as `"home"`) or
    /// a literal glyph, and a label.
    pub fn new(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            selected_icon: None,
            label: label.into(),
            enabled: true,
            badge: None,
        }
    }

    /// A different icon while selected. Without one, the icon is drawn
    /// filled (the Material Symbols `FILL` axis at 1) when selected.
    pub fn selected_icon(mut self, icon: impl Into<String>) -> Self {
        self.selected_icon = Some(icon.into());
        self
    }

    /// Whether the destination can be chosen. A disabled item is drawn at
    /// 38 % and is skipped by the keyboard.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// A badge on the icon.
    pub fn badge(mut self, badge: NavigationBadge) -> Self {
        self.badge = Some(badge);
        self
    }

    /// The label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The icon and whether to fill it, for the selected or unselected state.
    pub(crate) fn icon_for(&self, selected: bool) -> (&str, bool) {
        match (&self.selected_icon, selected) {
            (Some(icon), true) => (icon.as_str(), false),
            (None, true) => (self.icon.as_str(), true),
            (Some(_) | None, false) => (self.icon.as_str(), false),
        }
    }
}

/// The text to draw for an icon: the Material Symbols glyph for a known
/// name, otherwise the string itself (an emoji or a glyph passed directly).
pub(crate) fn icon_text(icon: &str) -> String {
    crate::material_symbol::get_material_symbol(icon)
        .map(String::from)
        .unwrap_or_else(|| icon.to_owned())
}

/// Paints `icon` centred at `center`, `size` points tall. `filled` sets the
/// Material Symbols `FILL` variation axis to 1 (the selected style); fonts
/// without that axis ignore it.
pub(crate) fn paint_icon(
    painter: &Painter,
    center: Pos2,
    icon: &str,
    size: f32,
    color: Color32,
    filled: bool,
) -> Rect {
    let mut format = TextFormat {
        font_id: FontId::proportional(size),
        color,
        ..Default::default()
    };
    if filled {
        format.coords = VariationCoords::new([(b"FILL", 1.0)]);
    }
    let job = LayoutJob::single_section(icon_text(icon), format);
    let galley = painter.layout_job(job);
    let rect = Align2::CENTER_CENTER.anchor_size(center, galley.size());
    painter.galley(rect.min, galley, color);
    Rect::from_center_size(center, Vec2::splat(size))
}

/// Lays out `text` in `role`.
pub(crate) fn text_galley(
    painter: &Painter,
    role: TypeRole,
    text: &str,
    color: Color32,
) -> std::sync::Arc<egui::Galley> {
    let job = role.style().layout_job(painter.ctx(), text, color);
    painter.layout_job(job)
}

/// Paints `badge` on the top-end corner of `anchor` (normally the 24 dp
/// icon), placed as Compose's `BadgedBox` does. `end_is_right` is false in
/// right-to-left layouts.
pub(crate) fn paint_badge(
    painter: &Painter,
    theme: &MaterialTheme,
    anchor: Rect,
    badge: &NavigationBadge,
    end_is_right: bool,
) -> Rect {
    // BadgeTokens.Color / LargeColor: error; LargeLabelTextColor: onError.
    let container = theme.color(ColorRole::Error);
    match badge {
        NavigationBadge::Small => {
            let x = if end_is_right {
                anchor.max.x - BADGE_SMALL_OFFSET
            } else {
                anchor.min.x + BADGE_SMALL_OFFSET - BADGE_SMALL_SIZE
            };
            let rect = Rect::from_min_size(
                Pos2::new(x, anchor.min.y - BADGE_SMALL_SIZE + BADGE_SMALL_OFFSET),
                Vec2::splat(BADGE_SMALL_SIZE),
            );
            painter.rect_filled(rect, CornerRadius::same(3), container);
            rect
        }
        NavigationBadge::Large(text) => {
            let galley = text_galley(
                painter,
                TypeRole::LabelSmall,
                text,
                theme.color(ColorRole::OnError),
            );
            let width = (galley.size().x + BADGE_LARGE_PADDING * 2.0).max(BADGE_LARGE_SIZE);
            let x = if end_is_right {
                anchor.max.x - BADGE_LARGE_HORIZONTAL_OFFSET
            } else {
                anchor.min.x + BADGE_LARGE_HORIZONTAL_OFFSET - width
            };
            let rect = Rect::from_min_size(
                Pos2::new(
                    x,
                    anchor.min.y - BADGE_LARGE_SIZE + BADGE_LARGE_VERTICAL_OFFSET,
                ),
                Vec2::new(width, BADGE_LARGE_SIZE),
            );
            painter.rect_filled(rect, CornerRadius::same(8), container);
            let text_pos = Align2::CENTER_CENTER
                .anchor_size(rect.center(), galley.size())
                .min;
            painter.galley(text_pos, galley, theme.color(ColorRole::OnError));
            rect
        }
    }
}

/// Reports a navigation item to AccessKit: role `Tab` (what Compose gives
/// navigation bar, rail and drawer items and tabs), its label, enabled
/// state, and `selected`.
pub(crate) fn describe_item(response: &Response, enabled: bool, label: &str, selected: bool) {
    crate::a11y::describe(response, Role::Tab, enabled, label, Some(selected));
    response.ctx.accesskit_node_builder(response.id, |node| {
        node.set_selected(selected);
    });
}

/// Which arrow keys move between a group's items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RoveAxis {
    /// Left and Right (mirrored in right-to-left layouts).
    Horizontal,
    /// Up and Down.
    Vertical,
}

/// Arrow-key roving within one group of items, given in order with their
/// enabled state: while an item has keyboard focus, the arrows of `axis`
/// move focus to the next or previous enabled item (wrapping), and Home and
/// End to the first and last. Returns the index focus moved to.
pub(crate) fn rove_focus(
    ui: &Ui,
    items: &[(Id, bool)],
    axis: RoveAxis,
    right_to_left: bool,
) -> Option<usize> {
    let focused = items
        .iter()
        .position(|(id, _)| ui.memory(|m| m.has_focus(*id)))?;
    let (focused_id, _) = items.get(focused)?;
    // Keep egui's own arrow-key focus navigation out of the group.
    ui.memory_mut(|m| {
        m.set_focus_lock_filter(
            *focused_id,
            EventFilter {
                horizontal_arrows: axis == RoveAxis::Horizontal,
                vertical_arrows: axis == RoveAxis::Vertical,
                ..Default::default()
            },
        );
    });
    let (next_key, previous_key) = match (axis, right_to_left) {
        (RoveAxis::Horizontal, false) => (Key::ArrowRight, Key::ArrowLeft),
        (RoveAxis::Horizontal, true) => (Key::ArrowLeft, Key::ArrowRight),
        (RoveAxis::Vertical, false | true) => (Key::ArrowDown, Key::ArrowUp),
    };
    let pressed = |key| ui.ctx().input_mut(|i| i.consume_key(Modifiers::NONE, key));
    let enabled = |index: &usize| items.get(*index).is_some_and(|(_, enabled)| *enabled);
    let count = items.len();
    let target = if pressed(next_key) {
        (1..count)
            .map(|step| (focused + step) % count)
            .find(|index| enabled(index))
    } else if pressed(previous_key) {
        (1..count)
            .map(|step| (focused + count - step) % count)
            .find(|index| enabled(index))
    } else if pressed(Key::Home) {
        (0..count).find(|index| enabled(index))
    } else if pressed(Key::End) {
        (0..count).rev().find(|index| enabled(index))
    } else {
        None
    }?;
    let (target_id, _) = items.get(target)?;
    ui.memory_mut(|m| m.request_focus(*target_id));
    Some(target)
}

/// The modal scrim colour at `fraction` of its full strength (for fades).
pub(crate) fn scrim_color(theme: &MaterialTheme, fraction: f32) -> Color32 {
    theme
        .color(ColorRole::Scrim)
        .gamma_multiply(SCRIM_OPACITY * fraction.clamp(0.0, 1.0))
}

/// Shows a scrim over the whole content rect, above panels and windows
/// (`Order::Foreground`), so it dims the page and takes the clicks meant for
/// it. Show the modal surface afterwards, in a foreground area moved to the
/// top. Returns the scrim's response (clicked when the page was clicked).
pub(crate) fn modal_scrim(ctx: &Context, id: Id, color: Color32) -> Response {
    let screen = ctx.content_rect();
    Area::new(id)
        .order(Order::Foreground)
        .fade_in(false)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let response = ui.allocate_response(screen.size(), Sense::click());
            ui.painter().rect_filled(screen, CornerRadius::ZERO, color);
            response
        })
        .inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_item_without_selected_icon_is_filled() {
        let item = NavigationItem::new("home", "Home");
        assert_eq!(item.icon_for(true), ("home", true), "selected: filled");
        assert_eq!(item.icon_for(false), ("home", false), "unselected: outline");
        let item = item.selected_icon("house");
        assert_eq!(
            item.icon_for(true),
            ("house", false),
            "a selected icon replaces the fill"
        );
    }

    #[test]
    fn unknown_icon_names_draw_as_given() {
        assert_eq!(icon_text("🏠"), "🏠", "a literal glyph passes through");
        assert_ne!(icon_text("home"), "home", "a symbol name maps to its glyph");
    }

    #[test]
    fn scrim_is_32_percent() {
        let theme = MaterialTheme::baseline(false);
        let scrim = scrim_color(&theme, 1.0);
        let expected = theme
            .color(ColorRole::Scrim)
            .gamma_multiply(SCRIM_OPACITY)
            .a();
        assert_eq!(scrim.a(), expected, "ScrimTokens.ContainerOpacity");
        assert_eq!(expected, 82, "0.32 of an opaque scrim");
    }
}
