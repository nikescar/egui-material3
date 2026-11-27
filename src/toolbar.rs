// ============================================================================
// Module:       toolbar
// Description:  Docked toolbar, and the action, FAB and overflow-menu pieces
//               shared by the app bars and toolbars.
//
// Dependencies: egui; crate::icon, crate::paint, crate::tokens, crate::a11y
// ============================================================================

//! The M3 Expressive docked toolbar ([`MaterialToolbar`]), and the pieces
//! every app bar and toolbar in this crate shares: 48 dp icon actions with a
//! 40 dp circular state layer, the FAB slot, the overflow menu and the
//! exit-always scroll offset.
//!
//! Values come from the Jetpack Compose material3 token files:
//! `IconButtonTokens` (StateLayerSize 40, IconSize 24, 48 dp touch target
//! from `minimumInteractiveComponentSize`), `FabPrimaryContainerTokens`
//! (ContainerHeight/Width 56, ContainerShape CornerLarge), `MenuTokens`
//! (ContainerShape CornerExtraSmall, ContainerColor SurfaceContainer,
//! ContainerElevation Level2, ListItemContainerHeight 48, ListItemLabelTextFont
//! LabelLarge) and the Expressive `DockedToolbarTokens` (ContainerHeight 64,
//! ContainerColor SurfaceContainer, ContainerLeadingSpace and
//! ContainerTrailingSpace 16).

use egui::{
    Align, Color32, CornerRadius, Frame, Id, Layout, Margin, Rect, Response, Role, Sense, Stroke,
    Ui, Vec2, Widget,
};

use crate::icon::{paint_icon, Icon, SymbolOptions, AUTO_MIRRORED};
use crate::paint::{paint_elevation, paint_focus_ring, paint_state_layer};
use crate::tokens::{ColorRole, Elevation, InteractionState, LayoutDirection, Shape, TypeRole};
use crate::MaterialTheme;

/// The touch target of an icon action: Compose's
/// `minimumInteractiveComponentSize`, 48 dp.
pub(crate) const ACTION_TARGET: f32 = 48.0;
/// `IconButtonTokens.StateLayerSize`: the visible circle, 40 dp.
pub(crate) const ACTION_STATE_LAYER: f32 = 40.0;
/// `IconButtonTokens.IconSize`: 24 dp.
pub(crate) const ICON_SIZE: f32 = 24.0;
/// `FabPrimaryContainerTokens.ContainerHeight` and `ContainerWidth`: 56 dp.
pub(crate) const FAB_SIZE: f32 = 56.0;

/// Symbol options for `icon` in the current layout direction: the icons
/// Material marks as directional are mirrored right-to-left.
pub(crate) fn symbol_options(theme: &MaterialTheme, icon: &Icon) -> SymbolOptions {
    let directional = match icon {
        Icon::Symbol(name) => AUTO_MIRRORED.contains(&name.as_str()),
        Icon::Svg(_) | Icon::Texture(_) => false,
    };
    SymbolOptions {
        mirrored: directional && theme.layout_direction == LayoutDirection::Rtl,
        ..SymbolOptions::default()
    }
}

/// How an icon action is coloured.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ActionColors {
    /// The icon (and state layer) colour.
    pub(crate) content: Color32,
    /// Container and icon colour when the action is a selected toggle.
    pub(crate) selected: (Color32, Color32),
}

impl ActionColors {
    /// Icon in `content`; a selected toggle uses the filled tonal icon
    /// button's selected pair (`FilledTonalIconButtonTokens`
    /// SelectedContainerColor SecondaryContainer, ToggleSelectedIconColor
    /// OnSecondaryContainer).
    pub(crate) fn new(theme: &MaterialTheme, content: Color32) -> Self {
        Self {
            content,
            selected: (
                theme.color(ColorRole::SecondaryContainer),
                theme.color(ColorRole::OnSecondaryContainer),
            ),
        }
    }
}

/// An icon action in the 48 dp `target`: a 40 dp circular state layer,
/// the 24 dp icon, a focus ring, and an AccessKit button (or toggle, when
/// `selected` is `Some`) named `label`.
pub(crate) fn icon_action(
    ui: &mut Ui,
    target: Rect,
    id: Id,
    icon: &Icon,
    label: &str,
    colors: ActionColors,
    selected: Option<bool>,
) -> Response {
    let response = ui.interact(target, id, Sense::click());
    match selected {
        Some(selected) => crate::a11y::toggle(&response, Role::Button, true, label, selected),
        None => crate::a11y::button(&response, true, label),
    }
    if !ui.is_rect_visible(target) {
        return response;
    }
    let theme = MaterialTheme::of(ui.ctx());
    let circle = Rect::from_center_size(target.center(), Vec2::splat(ACTION_STATE_LAYER));
    let radius = ACTION_STATE_LAYER / 2.0;
    let painter = ui.painter();
    let content = if selected == Some(true) {
        painter.circle_filled(circle.center(), radius, colors.selected.0);
        colors.selected.1
    } else {
        colors.content
    };
    let state = InteractionState::from_response(&response);
    let opacity = state.opacity();
    if opacity > 0.0 {
        painter.circle_filled(circle.center(), radius, content.gamma_multiply(opacity));
    }
    let icon_rect = Rect::from_center_size(target.center(), Vec2::splat(ICON_SIZE));
    paint_icon(
        painter,
        icon_rect,
        icon,
        &symbol_options(&theme, icon),
        content,
    );
    if response.has_focus() {
        paint_focus_ring(
            painter,
            circle,
            CornerRadius::same(radius as u8),
            theme.color(ColorRole::Secondary),
        );
    }
    response
}

/// The colours of a FAB in a bar or toolbar.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FabColors {
    /// Container fill.
    pub(crate) container: Color32,
    /// Icon and state layer.
    pub(crate) content: Color32,
}

/// A 56 dp FAB in `rect`: CornerLarge container at `elevation`, a state
/// layer, the icon, a focus ring, and an AccessKit button named `label`.
pub(crate) fn fab(
    ui: &mut Ui,
    rect: Rect,
    id: Id,
    icon: &Icon,
    label: &str,
    colors: FabColors,
    elevation: Elevation,
) -> Response {
    let response = ui.interact(rect, id, Sense::click());
    crate::a11y::button(&response, true, label);
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let theme = MaterialTheme::of(ui.ctx());
    let corners = Shape::LARGE.corner_radius(rect, theme.layout_direction);
    let painter = ui.painter();
    paint_elevation(
        painter,
        rect,
        corners,
        elevation,
        theme.color(ColorRole::Shadow),
    );
    painter.rect_filled(rect, corners, colors.container);
    paint_state_layer(
        painter,
        rect,
        corners,
        colors.content,
        InteractionState::from_response(&response),
    );
    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(ICON_SIZE));
    paint_icon(
        painter,
        icon_rect,
        icon,
        &symbol_options(&theme, icon),
        colors.content,
    );
    if response.has_focus() {
        paint_focus_ring(painter, rect, corners, theme.color(ColorRole::Secondary));
    }
    response
}

/// The overflow menu under `button`, toggled by clicking it: one 48 dp row
/// per item (icon and label). Returns the index of the item clicked this
/// frame; the menu closes on any click.
pub(crate) fn overflow_menu(button: &Response, items: &[(&Icon, &str)]) -> Option<usize> {
    // `MenuTokens.ListItemContainerHeight`.
    const ITEM_HEIGHT: f32 = 48.0;
    // `MenuTokens` ListItemLeadingSpace / ListItemTrailingSpace and the gap
    // between the leading icon and the label: 12 dp each.
    const SPACE: f32 = 12.0;
    // Compose `DropdownMenuItemDefaults`: DropdownMenuItemMinWidth 112,
    // DropdownMenuItemMaxWidth 280; DropdownMenuVerticalPadding 8.
    const MIN_WIDTH: f32 = 112.0;
    const MAX_WIDTH: f32 = 280.0;
    const VERTICAL_PADDING: i8 = 8;

    let ctx = button.ctx.clone();
    let theme = MaterialTheme::of(&ctx);
    let container = theme.color(ColorRole::SurfaceContainer);
    let [_ambient, key] = Elevation::Level2.shadows(theme.color(ColorRole::Shadow));
    let frame = Frame::new()
        .fill(container)
        // `MenuTokens.ContainerShape`: CornerExtraSmall, 4 dp.
        .corner_radius(4)
        .shadow(key)
        .inner_margin(Margin::symmetric(0, VERTICAL_PADDING));
    let style = TypeRole::LabelLarge.style();
    let label_color = theme.color(ColorRole::OnSurface);
    let icon_color = theme.color(ColorRole::OnSurfaceVariant);

    let shown = egui::Popup::menu(button).frame(frame).show(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let galleys: Vec<_> = items
            .iter()
            .map(|(_, label)| {
                ui.painter()
                    .layout_job(style.layout_job(ui.ctx(), *label, label_color))
            })
            .collect();
        let widest = galleys.iter().map(|g| g.size().x).fold(0.0, f32::max);
        let width = (SPACE + ICON_SIZE + SPACE + widest + SPACE).clamp(MIN_WIDTH, MAX_WIDTH);
        let mut clicked = None;
        for (index, ((icon, label), galley)) in items.iter().zip(galleys).enumerate() {
            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(width, ITEM_HEIGHT), Sense::click());
            crate::a11y::describe(&response, Role::MenuItem, true, label, None);
            let painter = ui.painter();
            paint_state_layer(
                painter,
                rect,
                0,
                label_color,
                InteractionState::from_response(&response),
            );
            let rtl = theme.layout_direction == LayoutDirection::Rtl;
            let icon_x = if rtl {
                rect.max.x - SPACE - ICON_SIZE
            } else {
                rect.min.x + SPACE
            };
            let icon_rect = Rect::from_min_size(
                egui::pos2(icon_x, rect.center().y - ICON_SIZE / 2.0),
                Vec2::splat(ICON_SIZE),
            );
            paint_icon(
                painter,
                icon_rect,
                icon,
                &symbol_options(&theme, icon),
                icon_color,
            );
            let text_x = if rtl {
                icon_rect.min.x - SPACE - galley.size().x
            } else {
                icon_rect.max.x + SPACE
            };
            let text_pos = egui::pos2(text_x, rect.center().y - galley.size().y / 2.0);
            painter
                .with_clip_rect(rect)
                .galley(text_pos, galley, label_color);
            if response.clicked() {
                clicked = Some(index);
            }
        }
        clicked
    });
    shown.and_then(|inner| inner.inner)
}

#[derive(Clone, Copy, Debug, Default)]
struct ExitAlways {
    last_content_offset: Option<f32>,
    hidden: f32,
}

/// Exit-always scroll behaviour (Compose's `enterAlwaysScrollBehavior` for
/// top app bars, `exitAlwaysScrollBehavior` for bottom app bars and
/// floating toolbars): how far, in `0..=limit`, the bar keyed by `id` is
/// scrolled away for a scroll area now at `content_offset`. Scrolling the
/// content down hides the bar by the same distance, scrolling up brings it
/// back at once; at the top of the content it is fully shown.
pub(crate) fn exit_always(ctx: &egui::Context, id: Id, content_offset: f32, limit: f32) -> f32 {
    let id = id.with("egui_material3::exit_always");
    let mut state: ExitAlways = ctx.data(|d| d.get_temp(id)).unwrap_or_default();
    let delta = state
        .last_content_offset
        .map_or(0.0, |last| content_offset - last);
    let limit = limit.max(0.0);
    state.hidden = if content_offset <= 0.0 {
        0.0
    } else {
        (state.hidden + delta).clamp(0.0, limit)
    };
    state.last_content_offset = Some(content_offset);
    ctx.data_mut(|d| d.insert_temp(id, state));
    state.hidden
}

/// The vertical offset of the [`egui::ScrollArea`] whose id is `id`, as
/// of its last frame (0 if it has not been shown). This is the scroll hook
/// for the scroll behaviours; get the id with [`scroll_area_id`].
pub fn scroll_area_offset(ctx: &egui::Context, id: Id) -> f32 {
    egui::scroll_area::State::load(ctx, id).map_or(0.0, |state| state.clamped_offset().y)
}

/// The id that `egui::ScrollArea::vertical().id_salt(id_salt)` gets when
/// shown in `ui`, for the app bars' and toolbars' `scroll_area` hooks.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{scroll_area_id, MaterialTopAppBar, TopAppBarScrollBehavior};
///
/// ui.add(
///     MaterialTopAppBar::large("Inbox")
///         .scroll_behavior(TopAppBarScrollBehavior::ExitUntilCollapsed)
///         .scroll_area(scroll_area_id(ui, "inbox")),
/// );
/// egui::ScrollArea::vertical().id_salt("inbox").show(ui, |ui| {
///     ui.label("Message");
/// });
/// # });
/// ```
pub fn scroll_area_id(ui: &Ui, id_salt: impl std::hash::Hash + std::fmt::Debug) -> Id {
    // `ScrollArea` wraps its salt in an `IdSalt` before deriving the id.
    ui.make_persistent_id(egui::IdSalt::new(id_salt))
}

// ---------------------------------------------------------------------------
// Docked toolbar

/// Surface levels for a [`MaterialToolbar`] container.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarElevation {
    /// The `surface` role.
    Level0,
    /// The `surfaceContainerLow` role.
    Level1,
    /// The `surfaceContainer` role (the docked toolbar's default).
    Level2,
    /// The `surfaceContainerHigh` role.
    Level3,
}

/// Material 3 Expressive docked toolbar: a full-width, 64 dp bar in
/// `surfaceContainer` holding buttons and icon buttons, docked to the top
/// or bottom of a screen.
///
/// `DockedToolbarTokens`: ContainerHeight 64, ContainerColor
/// SurfaceContainer, ContainerShape CornerNone, ContainerLeadingSpace and
/// ContainerTrailingSpace 16. Items before a [`spacer`](Self::spacer) are
/// laid out from the start, items after it from the end.
///
/// For the M3 bottom app bar with a FAB slot, see
/// [`crate::MaterialBottomAppBar`]; for the floating toolbar, see
/// [`crate::MaterialFloatingToolbar`].
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{toolbar, MaterialButton, icon_button_standard};
///
/// ui.add(toolbar()
///     .item(MaterialButton::text("Home"))
///     .spacer()
///     .item_fn(|ui| ui.add(icon_button_standard("search"))));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialToolbar<'a> {
    items: Vec<ToolbarItem<'a>>,
    top: bool,
    tabbar: bool,
    outline: bool,
    bg_color: Option<Color32>,
    elevation: Option<ToolbarElevation>,
    min_height: f32,
    item_spacing: f32,
    padding: Vec2,
}

/// An item in the toolbar.
enum ToolbarItem<'a> {
    /// A widget (button, icon button, ...).
    Widget(Box<dyn FnOnce(&mut Ui) -> Response + 'a>),
    /// Items after this are laid out from the end.
    Spacer,
}

impl Default for MaterialToolbar<'_> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            top: false,
            tabbar: false,
            outline: false,
            bg_color: None,
            elevation: None,
            // `DockedToolbarTokens.ContainerHeight`.
            min_height: 64.0,
            // Not a token: egui's gap between neighbouring items.
            item_spacing: 8.0,
            // `DockedToolbarTokens.ContainerLeadingSpace` / `TrailingSpace`
            // 16; 8 above and below centres a 48 dp item in 64.
            padding: Vec2::new(16.0, 8.0),
        }
    }
}

impl<'a> MaterialToolbar<'a> {
    /// A docked toolbar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a widget.
    ///
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, MaterialButton};
    ///
    /// ui.add(toolbar()
    ///     .item(MaterialButton::text("Home"))
    ///     .item(MaterialButton::text("Profile")));
    /// # });
    /// ```
    pub fn item<W>(mut self, widget: W) -> Self
    where
        W: Widget + 'a,
    {
        self.items
            .push(ToolbarItem::Widget(Box::new(move |ui| ui.add(widget))));
        self
    }

    /// Adds a closure that draws an item.
    ///
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, icon_button_standard};
    ///
    /// ui.add(toolbar()
    ///     .item_fn(|ui| ui.add(icon_button_standard("home")))
    ///     .item_fn(|ui| ui.add(icon_button_standard("search"))));
    /// # });
    /// ```
    pub fn item_fn<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Ui) -> Response + 'a,
    {
        self.items.push(ToolbarItem::Widget(Box::new(f)));
        self
    }

    /// Lays out the items after this one from the end of the toolbar.
    ///
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, MaterialButton};
    ///
    /// ui.add(toolbar()
    ///     .item(MaterialButton::text("Left"))
    ///     .spacer()
    ///     .item(MaterialButton::text("Right")));
    /// # });
    /// ```
    pub fn spacer(mut self) -> Self {
        self.items.push(ToolbarItem::Spacer);
        self
    }

    /// Docks the toolbar at the top of a screen: the divider (if
    /// [`outline`](Self::outline) is on) goes on its bottom edge.
    pub fn top(mut self, top: bool) -> Self {
        self.top = top;
        self
    }

    /// Gives every item an equal share of the width.
    pub fn tabbar(mut self, tabbar: bool) -> Self {
        self.tabbar = tabbar;
        self
    }

    /// Has no effect: items draw their own icons.
    #[deprecated(note = "has no effect; items draw their own icons and labels")]
    pub fn tabbar_icons(self, _show: bool) -> Self {
        self
    }

    /// Has no effect: items draw their own labels.
    #[deprecated(note = "has no effect; items draw their own icons and labels")]
    pub fn tabbar_labels(self, _show: bool) -> Self {
        self
    }

    /// Draws a 1 dp `outlineVariant` divider on the edge facing the content.
    /// Off by default: the M3 docked toolbar has none.
    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    /// Overrides the container colour.
    pub fn bg_color(mut self, color: Color32) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Picks the container's surface role (default `surfaceContainer`).
    ///
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::{toolbar, ToolbarElevation};
    ///
    /// ui.add(toolbar().elevation(ToolbarElevation::Level3));
    /// # });
    /// ```
    pub fn elevation(mut self, elevation: ToolbarElevation) -> Self {
        self.elevation = Some(elevation);
        self
    }

    /// Sets the height (default 64).
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = height;
        self
    }

    /// Sets the gap between items (default 8).
    pub fn item_spacing(mut self, spacing: f32) -> Self {
        self.item_spacing = spacing;
        self
    }

    /// Sets the inner padding (default 16 horizontally, 8 vertically).
    pub fn padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }
}

impl Widget for MaterialToolbar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let role = match self.elevation.unwrap_or(ToolbarElevation::Level2) {
            ToolbarElevation::Level0 => ColorRole::Surface,
            ToolbarElevation::Level1 => ColorRole::SurfaceContainerLow,
            ToolbarElevation::Level2 => ColorRole::SurfaceContainer,
            ToolbarElevation::Level3 => ColorRole::SurfaceContainerHigh,
        };
        let bg_color = self.bg_color.unwrap_or_else(|| theme.color(role));

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), self.min_height),
            Sense::hover(),
        );
        response.widget_info(|| egui::WidgetInfo::labeled(Role::Toolbar, true, ""));
        if !ui.is_rect_visible(rect) {
            return response;
        }
        ui.painter().rect_filled(rect, CornerRadius::ZERO, bg_color);
        if self.outline {
            let y = if self.top { rect.max.y } else { rect.min.y };
            ui.painter().hline(
                rect.x_range(),
                y,
                Stroke::new(1.0, theme.color(ColorRole::OutlineVariant)),
            );
        }

        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let (start, end) = if rtl {
            (
                Layout::right_to_left(Align::Center),
                Layout::left_to_right(Align::Center),
            )
        } else {
            (
                Layout::left_to_right(Align::Center),
                Layout::right_to_left(Align::Center),
            )
        };
        let inner = rect.shrink2(self.padding);
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(start));
        child.spacing_mut().item_spacing.x = self.item_spacing;

        if self.tabbar {
            let widgets: Vec<_> = self
                .items
                .into_iter()
                .filter_map(|item| match item {
                    ToolbarItem::Widget(f) => Some(f),
                    ToolbarItem::Spacer => None,
                })
                .collect();
            if !widgets.is_empty() {
                let count = widgets.len() as f32;
                let gaps = self.item_spacing * (count - 1.0);
                let width = ((inner.width() - gaps) / count).max(0.0);
                for widget in widgets {
                    child.allocate_ui_with_layout(
                        Vec2::new(width, inner.height()),
                        Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            widget(ui);
                        },
                    );
                }
            }
        } else {
            let mut before = Vec::new();
            let mut after = Vec::new();
            let mut past_spacer = false;
            for item in self.items {
                match item {
                    ToolbarItem::Widget(f) if past_spacer => after.push(f),
                    ToolbarItem::Widget(f) => before.push(f),
                    ToolbarItem::Spacer => past_spacer = true,
                }
            }
            for widget in before {
                widget(&mut child);
            }
            if !after.is_empty() {
                let rest = child.available_rect_before_wrap();
                let mut end_ui = child.new_child(egui::UiBuilder::new().max_rect(rest).layout(end));
                end_ui.spacing_mut().item_spacing.x = self.item_spacing;
                // An end-to-start layout places the first item at the edge;
                // reverse so they read in the order they were added.
                for widget in after.into_iter().rev() {
                    widget(&mut end_ui);
                }
            }
        }
        response
    }
}

/// [`MaterialToolbar::new`].
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{toolbar, MaterialButton, icon_button_standard};
///
/// ui.add(toolbar()
///     .item(MaterialButton::text("Home"))
///     .item_fn(|ui| ui.add(icon_button_standard("settings"))));
/// # });
/// ```
pub fn toolbar<'a>() -> MaterialToolbar<'a> {
    MaterialToolbar::new()
}

/// [`MaterialToolbar::new`], under the M3 Expressive name.
pub fn docked_toolbar<'a>() -> MaterialToolbar<'a> {
    MaterialToolbar::new()
}
