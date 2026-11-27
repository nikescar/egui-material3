// ============================================================================
// Module:       floating_toolbar
// Description:  Material 3 Expressive floating toolbar: horizontal or
//               vertical, standard or vibrant, with an optional FAB.
//
// Dependencies: egui; crate::icon, crate::paint, crate::tokens, crate::toolbar
// ============================================================================

//! The M3 Expressive floating toolbar: a pill of icon actions floating
//! over the content, horizontal or vertical, in the standard or vibrant
//! colour scheme, with an optional FAB beside it. It can hide while the
//! content scrolls down.
//!
//! Values from Compose's `FloatingToolbarDefaults` and the Expressive
//! `FloatingToolbarTokens`: ContainerSize 64, ContainerShape CornerFull
//! (corner radius = height / 2), ContainerElevation Level3,
//! ContentPadding 8, item gap 4, `ToolbarToFabGap` 8, `ScreenOffset` 16.
//! Standard colours: container SurfaceContainer, icons OnSurfaceVariant
//! (`StandardIconButtonTokens.IconColor`), FAB SecondaryContainer.
//! Vibrant colours: container PrimaryContainer, icons OnPrimaryContainer,
//! FAB TertiaryContainer. The FAB is the 56 dp
//! `FabPrimaryContainerTokens` container at Level3.

use egui::{
    Area, CornerRadius, Id, Order, Pos2, Rect, Response, Role, Sense, Ui, Vec2, Widget, WidgetInfo,
};

use crate::icon::Icon;
use crate::paint::paint_elevation;
use crate::tokens::{ColorRole, Elevation, LayoutDirection};
use crate::toolbar::{
    exit_always, fab, icon_action, scroll_area_offset, ActionColors, FabColors, ACTION_TARGET,
    FAB_SIZE,
};
use crate::MaterialTheme;

/// `FloatingToolbarDefaults.ContainerSize`: the toolbar's height
/// (horizontal) or width (vertical), 64 dp.
pub const FLOATING_TOOLBAR_SIZE: f32 = 64.0;
/// `FloatingToolbarDefaults.ContentPadding`.
const CONTENT_PADDING: f32 = 8.0;
/// The gap between items.
const ITEM_GAP: f32 = 4.0;
/// `FloatingToolbarDefaults.ToolbarToFabGap`.
const FAB_GAP: f32 = 8.0;
/// `FloatingToolbarDefaults.ScreenOffset`: distance from the edge it floats
/// along.
const SCREEN_OFFSET: f32 = 16.0;

/// Which way a floating toolbar runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatingToolbarOrientation {
    /// A row, floating above the bottom edge.
    #[default]
    Horizontal,
    /// A column, floating beside the end edge.
    Vertical,
}

/// A floating toolbar's colour scheme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatingToolbarStyle {
    /// `surfaceContainer` container.
    #[default]
    Standard,
    /// `primaryContainer` container, for more emphasis.
    Vibrant,
}

enum Item<'a> {
    Action {
        icon: Icon,
        label: String,
        callback: Box<dyn FnMut() + 'a>,
    },
    Toggle {
        icon: Icon,
        label: String,
        value: &'a mut bool,
    },
}

struct FabSlot<'a> {
    icon: Icon,
    label: String,
    callback: Box<dyn FnMut() + 'a>,
}

/// Material 3 Expressive floating toolbar.
///
/// Add it inline with `ui.add(...)`, or float it over the content with
/// [`show`](Self::show).
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{FloatingToolbarStyle, MaterialFloatingToolbar};
///
/// let mut bold = false;
/// ui.add(
///     MaterialFloatingToolbar::horizontal()
///         .style(FloatingToolbarStyle::Vibrant)
///         .toggle("format_bold", "Bold", &mut bold)
///         .action("format_list_bulleted", "List", || {})
///         .fab("add", "Insert", || {}),
/// );
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialFloatingToolbar<'a> {
    orientation: FloatingToolbarOrientation,
    style: FloatingToolbarStyle,
    items: Vec<Item<'a>>,
    fab: Option<FabSlot<'a>>,
    id_salt: Option<String>,
    hide_on_scroll: bool,
    content_offset: Option<f32>,
    scroll_area: Option<Id>,
}

impl<'a> MaterialFloatingToolbar<'a> {
    /// A floating toolbar running `orientation`.
    pub fn new(orientation: FloatingToolbarOrientation) -> Self {
        Self {
            orientation,
            style: FloatingToolbarStyle::Standard,
            items: Vec::new(),
            fab: None,
            id_salt: None,
            hide_on_scroll: false,
            content_offset: None,
            scroll_area: None,
        }
    }

    /// A horizontal floating toolbar.
    pub fn horizontal() -> Self {
        Self::new(FloatingToolbarOrientation::Horizontal)
    }

    /// A vertical floating toolbar.
    pub fn vertical() -> Self {
        Self::new(FloatingToolbarOrientation::Vertical)
    }

    /// Picks the standard or vibrant colours.
    pub fn style(mut self, style: FloatingToolbarStyle) -> Self {
        self.style = style;
        self
    }

    /// Adds an icon action, named `label` for screen readers.
    pub fn action(
        mut self,
        icon: impl Into<Icon>,
        label: impl Into<String>,
        callback: impl FnMut() + 'a,
    ) -> Self {
        self.items.push(Item::Action {
            icon: icon.into(),
            label: label.into(),
            callback: Box::new(callback),
        });
        self
    }

    /// Adds an icon toggle bound to `value`, shown filled when on.
    pub fn toggle(
        mut self,
        icon: impl Into<Icon>,
        label: impl Into<String>,
        value: &'a mut bool,
    ) -> Self {
        self.items.push(Item::Toggle {
            icon: icon.into(),
            label: label.into(),
            value,
        });
        self
    }

    /// Puts a FAB after the toolbar (to its end, or below a vertical one).
    pub fn fab(
        mut self,
        icon: impl Into<Icon>,
        label: impl Into<String>,
        callback: impl FnMut() + 'a,
    ) -> Self {
        self.fab = Some(FabSlot {
            icon: icon.into(),
            label: label.into(),
            callback: Box::new(callback),
        });
        self
    }

    /// Sets the id salt.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// With [`show`](Self::show), slides the toolbar off its edge as the
    /// content scrolls down and back as it scrolls up (Compose
    /// `FloatingToolbarDefaults.exitAlwaysScrollBehavior`). Needs
    /// [`content_offset`](Self::content_offset) or
    /// [`scroll_area`](Self::scroll_area).
    pub fn hide_on_scroll(mut self, hide: bool) -> Self {
        self.hide_on_scroll = hide;
        self
    }

    /// The content's vertical scroll offset.
    pub fn content_offset(mut self, offset: f32) -> Self {
        self.content_offset = Some(offset);
        self
    }

    /// Follows the [`egui::ScrollArea`] whose id is `id` (see
    /// [`crate::MaterialTopAppBar::scroll_area`]).
    pub fn scroll_area(mut self, id: Id) -> Self {
        self.scroll_area = Some(id);
        self
    }

    /// The toolbar's container size, without the FAB.
    pub fn container_size(&self) -> Vec2 {
        let n = self.items.len() as f32;
        let main = 2.0 * CONTENT_PADDING + n * ACTION_TARGET + (n - 1.0).max(0.0) * ITEM_GAP;
        match self.orientation {
            FloatingToolbarOrientation::Horizontal => Vec2::new(main, FLOATING_TOOLBAR_SIZE),
            FloatingToolbarOrientation::Vertical => Vec2::new(FLOATING_TOOLBAR_SIZE, main),
        }
    }

    /// The whole size, FAB included.
    fn total_size(&self) -> Vec2 {
        let container = self.container_size();
        let extra = if self.fab.is_some() {
            FAB_GAP + FAB_SIZE
        } else {
            0.0
        };
        match self.orientation {
            FloatingToolbarOrientation::Horizontal => container + Vec2::new(extra, 0.0),
            FloatingToolbarOrientation::Vertical => container + Vec2::new(0.0, extra),
        }
    }

    fn id(&self, ui: &Ui) -> Id {
        match &self.id_salt {
            Some(salt) => ui.make_persistent_id(salt),
            None => ui.next_auto_id(),
        }
    }

    /// Floats the toolbar over `ui`'s area: centred 16 dp above the bottom
    /// edge (horizontal) or 16 dp inside the end edge (vertical), in a
    /// foreground [`Area`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let ctx = ui.ctx().clone();
        let theme = MaterialTheme::of(&ctx);
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let id = self.id(ui);
        let size = self.total_size();
        let bounds = ui.max_rect();
        let hidden = if self.hide_on_scroll {
            let offset = self
                .content_offset
                .or_else(|| self.scroll_area.map(|area| scroll_area_offset(&ctx, area)))
                .unwrap_or(0.0);
            let limit = match self.orientation {
                FloatingToolbarOrientation::Horizontal => size.y,
                FloatingToolbarOrientation::Vertical => size.x,
            } + SCREEN_OFFSET;
            exit_always(&ctx, id, offset, limit)
        } else {
            0.0
        };
        let pos = match self.orientation {
            FloatingToolbarOrientation::Horizontal => Pos2::new(
                bounds.center().x - size.x / 2.0,
                bounds.max.y - SCREEN_OFFSET - size.y + hidden,
            ),
            FloatingToolbarOrientation::Vertical if rtl => Pos2::new(
                bounds.min.x + SCREEN_OFFSET - hidden,
                bounds.center().y - size.y / 2.0,
            ),
            FloatingToolbarOrientation::Vertical => Pos2::new(
                bounds.max.x - SCREEN_OFFSET - size.x + hidden,
                bounds.center().y - size.y / 2.0,
            ),
        };
        Area::new(id.with("area"))
            .order(Order::Foreground)
            .fixed_pos(pos)
            .constrain(false)
            .show(&ctx, |ui| {
                let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
                self.draw(ui, rect, id)
            })
            .inner
    }

    fn draw(self, ui: &mut Ui, rect: Rect, id: Id) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let container_size = self.container_size();
        let MaterialFloatingToolbar {
            orientation,
            style,
            items,
            fab: fab_slot,
            ..
        } = self;
        let horizontal = orientation == FloatingToolbarOrientation::Horizontal;
        let container = if horizontal && rtl {
            Rect::from_min_size(
                Pos2::new(rect.max.x - container_size.x, rect.min.y),
                container_size,
            )
        } else {
            Rect::from_min_size(rect.min, container_size)
        };

        let (container_role, content_role, fab_roles) = match style {
            FloatingToolbarStyle::Standard => (
                ColorRole::SurfaceContainer,
                ColorRole::OnSurfaceVariant,
                (
                    ColorRole::SecondaryContainer,
                    ColorRole::OnSecondaryContainer,
                ),
            ),
            FloatingToolbarStyle::Vibrant => (
                ColorRole::PrimaryContainer,
                ColorRole::OnPrimaryContainer,
                (ColorRole::TertiaryContainer, ColorRole::OnTertiaryContainer),
            ),
        };

        let mut response = ui.interact(container, id.with("container"), Sense::hover());
        response.widget_info(|| WidgetInfo::labeled(Role::Toolbar, true, ""));
        // CornerFull: half the cross-axis size.
        let radius = (FLOATING_TOOLBAR_SIZE / 2.0).round().clamp(0.0, 255.0) as u8;
        let corners = CornerRadius::same(radius);
        if ui.is_rect_visible(rect) {
            paint_elevation(
                ui.painter(),
                container,
                corners,
                Elevation::Level3,
                theme.color(ColorRole::Shadow),
            );
            ui.painter()
                .rect_filled(container, corners, theme.color(container_role));
        }

        let colors = ActionColors::new(&theme, theme.color(content_role));
        let step = ACTION_TARGET + ITEM_GAP;
        for (index, item) in items.into_iter().enumerate() {
            let along = CONTENT_PADDING + index as f32 * step;
            let min = if horizontal {
                let x = if rtl {
                    container.max.x - along - ACTION_TARGET
                } else {
                    container.min.x + along
                };
                Pos2::new(x, container.min.y + CONTENT_PADDING)
            } else {
                Pos2::new(container.min.x + CONTENT_PADDING, container.min.y + along)
            };
            let target = Rect::from_min_size(min, Vec2::splat(ACTION_TARGET));
            let item_id = id.with(("item", index));
            let r = match item {
                Item::Action {
                    icon,
                    label,
                    mut callback,
                } => {
                    let r = icon_action(ui, target, item_id, &icon, &label, colors, None);
                    if r.clicked() {
                        callback();
                    }
                    r
                }
                Item::Toggle { icon, label, value } => {
                    // Flip before painting, so the frame that sees the click
                    // already shows (and reports) the new state.
                    let clicked = ui.ctx().read_response(item_id).is_some_and(|r| r.clicked());
                    if clicked {
                        *value = !*value;
                    }
                    icon_action(ui, target, item_id, &icon, &label, colors, Some(*value))
                }
            };
            response = response.union(r);
        }

        if let Some(mut slot) = fab_slot {
            let min = if horizontal {
                let x = if rtl {
                    container.min.x - FAB_GAP - FAB_SIZE
                } else {
                    container.max.x + FAB_GAP
                };
                Pos2::new(x, container.center().y - FAB_SIZE / 2.0)
            } else {
                Pos2::new(
                    container.center().x - FAB_SIZE / 2.0,
                    container.max.y + FAB_GAP,
                )
            };
            let r = fab(
                ui,
                Rect::from_min_size(min, Vec2::splat(FAB_SIZE)),
                id.with("fab"),
                &slot.icon,
                &slot.label,
                FabColors {
                    container: theme.color(fab_roles.0),
                    content: theme.color(fab_roles.1),
                },
                Elevation::Level3,
            );
            if r.clicked() {
                (slot.callback)();
            }
            response = response.union(r);
        }
        response
    }
}

impl Widget for MaterialFloatingToolbar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = self.id(ui);
        let (rect, _) = ui.allocate_exact_size(self.total_size(), Sense::hover());
        self.draw(ui, rect, id)
    }
}

/// [`MaterialFloatingToolbar::horizontal`].
pub fn floating_toolbar<'a>() -> MaterialFloatingToolbar<'a> {
    MaterialFloatingToolbar::horizontal()
}
