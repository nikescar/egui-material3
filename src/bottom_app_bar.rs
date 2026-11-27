// ============================================================================
// Module:       bottom_app_bar
// Description:  Material Design 3 bottom app bar with action icons and a
//               FAB slot.
//
// Dependencies: egui; crate::icon, crate::tokens, crate::toolbar
// ============================================================================

//! The Material Design 3 bottom app bar: up to four action icons at the
//! start and an optional FAB at the end, on an 80 dp `surfaceContainer`
//! bar.
//!
//! Values from Compose's `BottomAppBarTokens` (ContainerHeight 80,
//! ContainerColor SurfaceContainer, ContainerElevation Level2 shown as
//! tone, ContainerShape CornerNone, IconColor OnSurfaceVariant, IconSize
//! 24) and `AppBar.kt` (`BottomAppBarHorizontalPadding` and
//! `BottomAppBarVerticalPadding` 16 - 12 = 4, `FABHorizontalPadding`
//! 16 - 4 = 12, `FABVerticalPadding` 12 - 4 = 8). The FAB uses
//! `BottomAppBarDefaults.bottomAppBarFabColor` (SecondaryContainer, content
//! OnSecondaryContainer) and `bottomAppBarFabElevation` (Level0), in the
//! 56 dp `FabPrimaryContainerTokens` container.

use egui::{
    Color32, Id, Pos2, Rect, Response, Role, Sense, Ui, UiBuilder, Vec2, Widget, WidgetInfo,
};

use crate::icon::Icon;
use crate::tokens::{ColorRole, Elevation, LayoutDirection};
use crate::toolbar::{
    exit_always, fab, icon_action, scroll_area_offset, ActionColors, FabColors, ACTION_TARGET,
    FAB_SIZE,
};
use crate::MaterialTheme;

/// `BottomAppBarTokens.ContainerHeight`: 80 dp.
pub const BOTTOM_APP_BAR_HEIGHT: f32 = 80.0;
/// `BottomAppBarHorizontalPadding` and `BottomAppBarVerticalPadding`.
const PADDING: f32 = 4.0;
/// `FABHorizontalPadding`, inside [`PADDING`].
const FAB_END_PADDING: f32 = 12.0;
/// `FABVerticalPadding`, inside [`PADDING`].
const FAB_TOP_PADDING: f32 = 8.0;

struct BarAction<'a> {
    icon: Icon,
    label: String,
    callback: Box<dyn FnMut() + 'a>,
}

/// Material Design 3 bottom app bar.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::MaterialBottomAppBar;
///
/// ui.add(
///     MaterialBottomAppBar::new()
///         .action("check_box", "Select", || {})
///         .action("edit", "Edit", || {})
///         .fab("add", "New item", || {}),
/// );
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialBottomAppBar<'a> {
    actions: Vec<BarAction<'a>>,
    fab: Option<BarAction<'a>>,
    id_salt: Option<String>,
    container_color: Option<Color32>,
    hide_on_scroll: bool,
    content_offset: Option<f32>,
    scroll_area: Option<Id>,
}

impl Default for MaterialBottomAppBar<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> MaterialBottomAppBar<'a> {
    /// An empty bottom app bar.
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            fab: None,
            id_salt: None,
            container_color: None,
            hide_on_scroll: false,
            content_offset: None,
            scroll_area: None,
        }
    }

    /// Adds an action icon at the start, named `label` for screen readers.
    pub fn action(
        mut self,
        icon: impl Into<Icon>,
        label: impl Into<String>,
        callback: impl FnMut() + 'a,
    ) -> Self {
        self.actions.push(BarAction {
            icon: icon.into(),
            label: label.into(),
            callback: Box::new(callback),
        });
        self
    }

    /// Puts a FAB at the end, named `label` for screen readers.
    pub fn fab(
        mut self,
        icon: impl Into<Icon>,
        label: impl Into<String>,
        callback: impl FnMut() + 'a,
    ) -> Self {
        self.fab = Some(BarAction {
            icon: icon.into(),
            label: label.into(),
            callback: Box::new(callback),
        });
        self
    }

    /// Sets the id salt (needed for [`hide_on_scroll`](Self::hide_on_scroll)
    /// when several bars share a `Ui`).
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Overrides the container colour.
    pub fn container_color(mut self, color: Color32) -> Self {
        self.container_color = Some(color);
        self
    }

    /// Hides the bar as the content scrolls down and brings it back when
    /// it scrolls up (Compose `BottomAppBarDefaults.exitAlwaysScrollBehavior`).
    /// Needs [`content_offset`](Self::content_offset) or
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
}

impl Widget for MaterialBottomAppBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialBottomAppBar {
            mut actions,
            fab: mut fab_action,
            id_salt,
            container_color,
            hide_on_scroll,
            content_offset,
            scroll_area,
        } = self;
        let ctx = ui.ctx().clone();
        let theme = MaterialTheme::of(&ctx);
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let id = match &id_salt {
            Some(salt) => ui.make_persistent_id(salt),
            None => ui.next_auto_id(),
        };
        let hidden = if hide_on_scroll {
            let offset = content_offset
                .or_else(|| scroll_area.map(|area| scroll_area_offset(&ctx, area)))
                .unwrap_or(0.0);
            exit_always(&ctx, id, offset, BOTTOM_APP_BAR_HEIGHT)
        } else {
            0.0
        };

        let desired = Vec2::new(ui.available_width(), BOTTOM_APP_BAR_HEIGHT - hidden);
        let mut response = ui.allocate_response(desired, Sense::hover());
        let rect = response.rect;
        response.widget_info(|| WidgetInfo::labeled(Role::Toolbar, true, ""));
        if !ui.is_rect_visible(rect) || rect.height() <= 0.0 {
            return response;
        }
        let clip = rect.intersect(ui.clip_rect());
        let mut bar_ui = ui.new_child(UiBuilder::new().max_rect(rect).id_salt(id));
        bar_ui.set_clip_rect(clip);
        let container = container_color.unwrap_or_else(|| theme.color(ColorRole::SurfaceContainer));
        bar_ui.painter().rect_filled(rect, 0, container);

        // The content stays laid out at full height, anchored to the top
        // edge, and is cut off from below as the bar hides.
        let full = Rect::from_min_size(rect.min, Vec2::new(rect.width(), BOTTOM_APP_BAR_HEIGHT));
        let x_at = |start: f32, w: f32| {
            if rtl {
                full.max.x - start - w
            } else {
                full.min.x + start
            }
        };
        let colors = ActionColors::new(&theme, theme.color(ColorRole::OnSurfaceVariant));
        for (index, action) in actions.iter_mut().enumerate() {
            let start = PADDING + index as f32 * ACTION_TARGET;
            let target = Rect::from_min_size(
                Pos2::new(
                    x_at(start, ACTION_TARGET),
                    full.center().y - ACTION_TARGET / 2.0,
                ),
                Vec2::splat(ACTION_TARGET),
            );
            let r = icon_action(
                &mut bar_ui,
                target,
                id.with(("action", index)),
                &action.icon,
                &action.label,
                colors,
                None,
            );
            if r.clicked() {
                (action.callback)();
            }
            response = response.union(r);
        }
        if let Some(action) = fab_action.as_mut() {
            let start = PADDING + FAB_END_PADDING;
            let fab_rect = Rect::from_min_size(
                Pos2::new(
                    if rtl {
                        full.min.x + start
                    } else {
                        full.max.x - start - FAB_SIZE
                    },
                    full.min.y + PADDING + FAB_TOP_PADDING,
                ),
                Vec2::splat(FAB_SIZE),
            );
            let r = fab(
                &mut bar_ui,
                fab_rect,
                id.with("fab"),
                &action.icon,
                &action.label,
                FabColors {
                    container: theme.color(ColorRole::SecondaryContainer),
                    content: theme.color(ColorRole::OnSecondaryContainer),
                },
                Elevation::Level0,
            );
            if r.clicked() {
                (action.callback)();
            }
            response = response.union(r);
        }
        response
    }
}

/// [`MaterialBottomAppBar::new`].
pub fn bottom_app_bar<'a>() -> MaterialBottomAppBar<'a> {
    MaterialBottomAppBar::new()
}
