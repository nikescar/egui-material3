// ============================================================================
// Module:       fab_menu
// Description:  Material 3 FAB menu: a toggle FAB that morphs into a close
//               button and opens a stack of labelled items.
//
// Dependencies: egui; crate::{a11y, fab, icon, iconbutton, material_theme,
//               paint, tokens}
// ============================================================================

//! The Material 3 FAB menu (M3 Expressive).
//!
//! A toggle FAB that, when clicked, morphs into a round close button and
//! opens a stack of pill-shaped items above it, staggered from the button
//! outwards. Clicking an item, clicking the close button or pressing Escape
//! closes the menu; Escape returns focus to the button.
//!
//! Geometry from Compose's `FabMenuBaselineTokens` (close button 56 dp,
//! `CornerFull`, `Level3`, 20 dp icon; items 56 dp high, `CornerFull`,
//! `TitleMedium`, 24 dp leading and trailing space, 24 dp icon, 8 dp
//! icon-label space, 4 dp between items, 8 dp between the items and the
//! close button). Colours from `FabMenuPrimaryContainerTokens`,
//! `FabMenuSecondaryContainerTokens` and `FabMenuTertiaryContainerTokens`
//! (items), and from `FloatingActionButtonMenu.kt`'s toggle colours (the
//! container animates from the `*Container` role to the solid role, the
//! icon from `on*Container` to `on*`).

use egui::{
    Align2, Area, CornerRadius, Id, Key, Modifiers, Order, Rect, Response, Sense, Ui, Vec2,
};

use crate::fab::{fab_elevation, FabSize};
use crate::icon::Icon;
use crate::iconbutton::paint_widget_icon;
use crate::tokens::{
    animate_value_with_spring, motion, ColorRole, CubicBezier, Elevation, InteractionState,
    LayoutDirection, SpringSpeed, TypeRole,
};
use crate::MaterialTheme;

/// `FabMenuBaselineTokens.CloseButtonContainerWidth` / `Height`.
pub const CLOSE_BUTTON_SIZE: f32 = 56.0;
/// `FabMenuBaselineTokens.CloseButtonIconSize`.
pub const CLOSE_BUTTON_ICON_SIZE: f32 = 20.0;
/// `FabMenuBaselineTokens.CloseButtonBetweenSpace` (`Space100`).
pub const CLOSE_BUTTON_BETWEEN_SPACE: f32 = 8.0;
/// `FabMenuBaselineTokens.ItemContainerHeight`.
pub const ITEM_HEIGHT: f32 = 56.0;
/// `FabMenuBaselineTokens.ItemBetweenSpace` (`Space50`).
pub const ITEM_BETWEEN_SPACE: f32 = 4.0;
/// `FabMenuBaselineTokens.ItemLeadingSpace` and `ItemTrailingSpace`
/// (`Space300`).
pub const ITEM_HORIZONTAL_SPACE: f32 = 24.0;
/// `FabMenuBaselineTokens.ItemIconSize`.
pub const ITEM_ICON_SIZE: f32 = 24.0;
/// `FabMenuBaselineTokens.ItemIconLabelSpace` (`Space100`).
pub const ITEM_ICON_LABEL_SPACE: f32 = 8.0;

/// How long one item takes to appear (`MotionTokens.DurationShort4`).
const ITEM_DURATION: f32 = motion::DURATION_SHORT4.as_secs_f32();
/// The delay between items (`MotionTokens.DurationShort1`).
const ITEM_STAGGER: f32 = motion::DURATION_SHORT1.as_secs_f32();

/// The colour scheme of a FAB menu.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FabMenuColors {
    /// `primaryContainer` items; the button goes from `primaryContainer` to
    /// `primary`.
    #[default]
    Primary,
    /// `secondaryContainer` items; the button goes from
    /// `secondaryContainer` to `secondary`.
    Secondary,
    /// `tertiaryContainer` items; the button goes from `tertiaryContainer`
    /// to `tertiary`.
    Tertiary,
}

impl FabMenuColors {
    /// Every colour scheme.
    pub const ALL: [FabMenuColors; 3] = [
        FabMenuColors::Primary,
        FabMenuColors::Secondary,
        FabMenuColors::Tertiary,
    ];

    /// The item container and content roles (`ListItemContainerColor`,
    /// `ListItemIconColor` / `ListItemLabelTextColor`), which are also the
    /// closed button's colours.
    pub const fn item_roles(self) -> (ColorRole, ColorRole) {
        match self {
            FabMenuColors::Primary => (ColorRole::PrimaryContainer, ColorRole::OnPrimaryContainer),
            FabMenuColors::Secondary => (
                ColorRole::SecondaryContainer,
                ColorRole::OnSecondaryContainer,
            ),
            FabMenuColors::Tertiary => {
                (ColorRole::TertiaryContainer, ColorRole::OnTertiaryContainer)
            }
        }
    }

    /// The open (close button) container and icon roles.
    pub const fn open_roles(self) -> (ColorRole, ColorRole) {
        match self {
            FabMenuColors::Primary => (ColorRole::Primary, ColorRole::OnPrimary),
            FabMenuColors::Secondary => (ColorRole::Secondary, ColorRole::OnSecondary),
            FabMenuColors::Tertiary => (ColorRole::Tertiary, ColorRole::OnTertiary),
        }
    }
}

/// The elevation of a menu item: `Level0` at rest
/// (`ItemContainerElevation`), `Level4` hovered, `Level3` focused or
/// pressed.
fn item_elevation(state: InteractionState) -> Elevation {
    if state.pressed || state.focused {
        Elevation::Level3
    } else if state.hovered {
        Elevation::Level4
    } else {
        Elevation::Level0
    }
}

/// What a FAB menu reported this frame.
#[derive(Debug)]
pub struct FabMenuResponse {
    /// The toggle button.
    pub toggle: Response,
    /// The index of the item clicked this frame, if any.
    pub clicked: Option<usize>,
}

/// A Material 3 FAB menu.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::FabMenu;
///
/// let mut open = false;
/// let response = FabMenu::new(&mut open)
///     .item("reply", "Reply")
///     .item("forward", "Forward")
///     .show(ui);
/// if response.clicked == Some(0) {
///     // reply
/// }
/// # });
/// ```
#[must_use = "Call `.show(ui)` to draw the menu"]
pub struct FabMenu<'a> {
    expanded: &'a mut bool,
    items: Vec<(Icon, String)>,
    colors: FabMenuColors,
    icon: Icon,
    close_icon: Icon,
    size: FabSize,
    label: String,
    id_salt: Option<String>,
}

impl<'a> FabMenu<'a> {
    /// A closed or open menu bound to `expanded`, with the `add` icon.
    pub fn new(expanded: &'a mut bool) -> Self {
        Self {
            expanded,
            items: Vec::new(),
            colors: FabMenuColors::Primary,
            icon: Icon::from("add"),
            close_icon: Icon::from("close"),
            size: FabSize::Regular,
            label: "Menu".to_owned(),
            id_salt: None,
        }
    }

    /// Adds an item with `icon` and `label`, below the ones added before.
    pub fn item(mut self, icon: impl Into<Icon>, label: impl Into<String>) -> Self {
        self.items.push((icon.into(), label.into()));
        self
    }

    /// The colour scheme.
    pub fn colors(mut self, colors: FabMenuColors) -> Self {
        self.colors = colors;
        self
    }

    /// The closed button's icon.
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = icon.into();
        self
    }

    /// The open button's icon (`close` by default).
    pub fn close_icon(mut self, icon: impl Into<Icon>) -> Self {
        self.close_icon = icon.into();
        self
    }

    /// The closed button's size: [`FabSize::Regular`] (56),
    /// [`FabSize::Medium`] (80) or [`FabSize::Large`] (96). Small and
    /// extended act as regular.
    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// The button's accessible name ("Menu" by default); AccessKit reports
    /// whether the menu is open as the button's toggled state.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Distinguishes several menus in one `Ui`.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Draws the button in `ui`, and the open items in a foreground layer
    /// above it.
    pub fn show(self, ui: &mut Ui) -> FabMenuResponse {
        let theme = MaterialTheme::of(ui.ctx());
        let id = match &self.id_salt {
            Some(salt) => ui.make_persistent_id(salt),
            None => ui.next_auto_id(),
        };
        let metrics = match self.size {
            FabSize::Medium | FabSize::Large => self.size.metrics(),
            FabSize::Small | FabSize::Regular | FabSize::Extended => FabSize::Regular.metrics(),
        };
        let (_, outer) = ui.allocate_space(Vec2::splat(metrics.size));
        let mut toggle = ui.interact(outer, id.with("toggle"), Sense::click());

        let mut expanded = *self.expanded;
        if toggle.clicked() {
            expanded = !expanded;
            toggle.mark_changed();
        }
        if expanded && ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape)) {
            expanded = false;
            toggle.mark_changed();
            toggle.request_focus();
        }
        crate::a11y::toggle(&toggle, egui::Role::Button, true, &self.label, expanded);

        // The button morphs with the fast spatial spring; the items run on a
        // linear clock, each easing in over ITEM_DURATION after a stagger.
        let morph = animate_value_with_spring(
            ui.ctx(),
            id.with("morph"),
            if expanded { 1.0 } else { 0.0 },
            theme.motion.spatial(SpringSpeed::Fast),
        );
        let n = self.items.len();
        let total = ITEM_DURATION + ITEM_STAGGER * n.saturating_sub(1) as f32;
        let clock = ui
            .ctx()
            .animate_bool_with_time(id.with("items"), expanded, total);

        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let close_rect = if rtl {
            Rect::from_min_size(
                outer.left_bottom() - Vec2::new(0.0, CLOSE_BUTTON_SIZE),
                Vec2::splat(CLOSE_BUTTON_SIZE),
            )
        } else {
            Rect::from_min_size(
                outer.right_bottom() - Vec2::splat(CLOSE_BUTTON_SIZE),
                Vec2::splat(CLOSE_BUTTON_SIZE),
            )
        };

        let mut clicked = None;
        if clock > 0.0 && n > 0 {
            clicked = self.show_items(ui, id, close_rect, rtl, clock, total, expanded);
            if clicked.is_some() {
                expanded = false;
                toggle.mark_changed();
            }
        }

        self.paint_toggle(ui, &toggle, outer, close_rect, morph);
        *self.expanded = expanded;
        FabMenuResponse { toggle, clicked }
    }

    fn paint_toggle(&self, ui: &Ui, toggle: &Response, outer: Rect, close_rect: Rect, morph: f32) {
        if !ui.is_rect_visible(outer.expand(16.0)) {
            return;
        }
        let theme = MaterialTheme::of(ui.ctx());
        let t = morph.clamp(0.0, 1.0);
        let rect = Rect::from_min_max(
            outer.min.lerp(close_rect.min, morph),
            outer.max.lerp(close_rect.max, morph),
        );
        let closed_radius = match self.size {
            FabSize::Medium | FabSize::Large => self.size.metrics().shape.radius(outer),
            FabSize::Small | FabSize::Regular | FabSize::Extended => {
                FabSize::Regular.metrics().shape.radius(outer)
            }
        };
        let full = rect.width().min(rect.height()).max(0.0) / 2.0;
        let radius = (closed_radius + (full - closed_radius) * t).clamp(0.0, full);
        let corner_radius = CornerRadius::same(radius.round().clamp(0.0, 255.0) as u8);

        let (closed_container, closed_content) = self.colors.item_roles();
        let (open_container, open_content) = self.colors.open_roles();
        let container = theme
            .color(closed_container)
            .lerp_to_gamma(theme.color(open_container), t);
        let content = theme
            .color(closed_content)
            .lerp_to_gamma(theme.color(open_content), t);

        let interaction = InteractionState::from_response(toggle);
        let painter = ui.painter();
        // `CloseButtonContainerElevation` Level3, Level4 on hover like a FAB.
        crate::paint::paint_elevation(
            painter,
            rect,
            corner_radius,
            fab_elevation(interaction, false),
            theme.color(ColorRole::Shadow),
        );
        painter.rect_filled(rect, corner_radius, container);
        crate::paint::paint_state_layer(painter, rect, corner_radius, content, interaction);

        let icon_size =
            metrics_icon(self.size) + (CLOSE_BUTTON_ICON_SIZE - metrics_icon(self.size)) * t;
        let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
        if t < 1.0 {
            paint_widget_icon(
                painter,
                icon_rect,
                &self.icon,
                false,
                content.gamma_multiply(1.0 - t),
                theme.layout_direction,
            );
        }
        if t > 0.0 {
            paint_widget_icon(
                painter,
                icon_rect,
                &self.close_icon,
                false,
                content.gamma_multiply(t),
                theme.layout_direction,
            );
        }
        if toggle.has_focus() {
            crate::paint::paint_focus_ring(
                painter,
                rect,
                corner_radius,
                theme.color(ColorRole::Secondary),
            );
        }
    }

    /// Draws the items above `close_rect` and returns the clicked one.
    #[expect(
        clippy::too_many_arguments,
        reason = "private helper splitting `show`; the values are all per-frame layout state"
    )]
    fn show_items(
        &self,
        ui: &Ui,
        id: Id,
        close_rect: Rect,
        rtl: bool,
        clock: f32,
        total: f32,
        interactive: bool,
    ) -> Option<usize> {
        let theme = MaterialTheme::of(ui.ctx());
        let (container_role, content_role) = self.colors.item_roles();
        let container = theme.color(container_role);
        let content = theme.color(content_role);
        let shadow = theme.color(ColorRole::Shadow);
        let label_style = TypeRole::TitleMedium.style();
        let n = self.items.len();

        let (anchor, pivot) = if rtl {
            (
                close_rect.left_top() - Vec2::new(0.0, CLOSE_BUTTON_BETWEEN_SPACE),
                Align2::LEFT_BOTTOM,
            )
        } else {
            (
                close_rect.right_top() - Vec2::new(0.0, CLOSE_BUTTON_BETWEEN_SPACE),
                Align2::RIGHT_BOTTOM,
            )
        };

        let mut clicked = None;
        Area::new(id.with("fab_menu_items"))
            .order(Order::Foreground)
            .pivot(pivot)
            .fixed_pos(anchor)
            .show(ui.ctx(), |ui| {
                let galleys: Vec<_> = self
                    .items
                    .iter()
                    .map(|(_, label)| {
                        ui.painter().layout_job(label_style.layout_job(
                            ui.ctx(),
                            label.as_str(),
                            content,
                        ))
                    })
                    .collect();
                let widths: Vec<f32> = galleys
                    .iter()
                    .map(|g| {
                        2.0 * ITEM_HORIZONTAL_SPACE
                            + ITEM_ICON_SIZE
                            + ITEM_ICON_LABEL_SPACE
                            + g.size().x
                    })
                    .collect();
                let block_width = widths.iter().copied().fold(0.0, f32::max);
                let block_height =
                    n as f32 * ITEM_HEIGHT + n.saturating_sub(1) as f32 * ITEM_BETWEEN_SPACE;
                let (block, _) =
                    ui.allocate_exact_size(Vec2::new(block_width, block_height), Sense::hover());

                let elapsed = clock * total;
                let ease = CubicBezier::EMPHASIZED_DECELERATE;
                for (i, (((icon, label), galley), width)) in self
                    .items
                    .iter()
                    .zip(galleys)
                    .zip(widths.iter().copied())
                    .enumerate()
                {
                    // Item 0 is the top one; the one nearest the button
                    // appears first and leaves last.
                    let from_button = n - 1 - i;
                    let local = ((elapsed - from_button as f32 * ITEM_STAGGER) / ITEM_DURATION)
                        .clamp(0.0, 1.0);
                    let p = ease.transform(local);
                    if p <= 0.0 {
                        continue;
                    }
                    let top = block.min.y + i as f32 * (ITEM_HEIGHT + ITEM_BETWEEN_SPACE);
                    let x = if rtl {
                        block.min.x
                    } else {
                        block.max.x - width
                    };
                    let rect =
                        Rect::from_min_size(egui::pos2(x, top), Vec2::new(width, ITEM_HEIGHT));
                    let response = ui.interact(
                        rect,
                        id.with(("fab_menu_item", i)),
                        crate::a11y::sense(interactive, Sense::click()),
                    );
                    crate::a11y::describe(
                        &response,
                        egui::Role::MenuItem,
                        interactive,
                        label,
                        None,
                    );
                    if interactive && response.clicked() {
                        clicked = Some(i);
                    }

                    let drawn = rect.translate(Vec2::new(0.0, (1.0 - p) * 16.0));
                    let corner_radius =
                        CornerRadius::same((ITEM_HEIGHT / 2.0).round().clamp(0.0, 255.0) as u8);
                    let interaction = InteractionState::from_response(&response);
                    let painter = ui.painter();
                    crate::paint::paint_elevation(
                        painter,
                        drawn,
                        corner_radius,
                        item_elevation(interaction),
                        shadow.gamma_multiply(p),
                    );
                    painter.rect_filled(drawn, corner_radius, container.gamma_multiply(p));
                    crate::paint::paint_state_layer(
                        painter,
                        drawn,
                        corner_radius,
                        content.gamma_multiply(p),
                        interaction,
                    );
                    let (icon_x, label_x) = if rtl {
                        let icon_x = drawn.max.x - ITEM_HORIZONTAL_SPACE - ITEM_ICON_SIZE / 2.0;
                        (
                            icon_x,
                            icon_x - ITEM_ICON_SIZE / 2.0 - ITEM_ICON_LABEL_SPACE - galley.size().x,
                        )
                    } else {
                        let icon_x = drawn.min.x + ITEM_HORIZONTAL_SPACE + ITEM_ICON_SIZE / 2.0;
                        (
                            icon_x,
                            icon_x + ITEM_ICON_SIZE / 2.0 + ITEM_ICON_LABEL_SPACE,
                        )
                    };
                    let icon_rect = Rect::from_center_size(
                        egui::pos2(icon_x, drawn.center().y),
                        Vec2::splat(ITEM_ICON_SIZE),
                    );
                    paint_widget_icon(
                        painter,
                        icon_rect,
                        icon,
                        false,
                        content.gamma_multiply(p),
                        theme.layout_direction,
                    );
                    let label_pos = egui::pos2(label_x, drawn.center().y - galley.size().y / 2.0);
                    painter.galley(label_pos, galley, content.gamma_multiply(p));
                    if response.has_focus() {
                        crate::paint::paint_focus_ring(
                            painter,
                            drawn,
                            corner_radius,
                            theme.color(ColorRole::Secondary),
                        );
                    }
                }
            });
        clicked
    }
}

/// The closed button's icon size.
fn metrics_icon(size: FabSize) -> f32 {
    match size {
        FabSize::Medium | FabSize::Large => size.metrics().icon_size,
        FabSize::Small | FabSize::Regular | FabSize::Extended => {
            FabSize::Regular.metrics().icon_size
        }
    }
}
