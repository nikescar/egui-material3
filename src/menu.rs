//! Material Design 3 menus.
//!
//! A menu shows a list of choices on a temporary surface, anchored to the
//! control that opened it (or to the pointer, for a context menu).
//!
//! # Tokens
//!
//! The default ([`MenuVariant::Baseline`]) follows Compose's `MenuTokens.kt`
//! (`ContainerColor` = `SurfaceContainer`, `ContainerShape` =
//! `CornerExtraSmall`, `ContainerElevation` = `Level2`) and `Menu.kt`
//! (item height `MenuListItemContainerHeight` 48 dp, label `labelLarge`,
//! `DropdownMenuItemHorizontalPadding` 12 dp, `DropdownMenuVerticalPadding`
//! 8 dp, width between `DropdownMenuItemDefaultMinWidth` 112 dp and
//! `DropdownMenuItemDefaultMaxWidth` 280 dp, `MenuVerticalMargin` 48 dp and
//! `MenuHorizontalMargin` 8 dp from the window edges, icons
//! `ListTokens.ItemLeadingIconSize` 24 dp).
//!
//! The M3 Expressive styles ([`MenuVariant::Standard`] and
//! [`MenuVariant::Vibrant`], optionally [`MaterialMenu::segmented`]) follow
//! `StandardMenuTokens.kt`, `VibrantMenuTokens.kt` and
//! `SegmentedMenuTokens.kt`: `CornerLarge` container, `GroupPadding` 4 dp,
//! `Item` height 44 dp, `ItemLeadingSpace` 16 dp, `ItemBetweenSpace` 12 dp,
//! `ItemTopSpace`/`ItemBottomSpace` 8 dp, label `BodyLarge`, supporting
//! text `BodyMedium`, trailing text `LabelSmall`, icons 20 dp, item shape
//! `CornerExtraSmall`, selected item `CornerMedium` in
//! `ItemSelectedContainerColor`, and for segmented menus groups in
//! `CornerSmall` separated by `SegmentedGap` 2 dp.
//!
//! # Behaviour
//!
//! - The menu is as wide as its content needs, within 112–280 dp, and
//!   flips above or to the other side of its anchor when it would not fit.
//! - Items can carry a leading icon, supporting text, trailing text (a
//!   shortcut), a trailing icon, a check mark ([`MenuItem::checked`],
//!   [`MenuItem::toggles`]) or a radio mark ([`MenuItem::radio`]), and a
//!   cascading submenu ([`MenuItem::submenu`]) that opens on hover (after a
//!   short hover-intent delay), on click, or with the Right arrow.
//! - [`MaterialMenu::divider`] and [`MaterialMenu::group`] split items into
//!   groups.
//! - Keyboard: Up/Down (and Home/End) move the highlight, Enter or Space
//!   activates, Right opens a submenu, Left or Escape closes it, Escape on
//!   the top level closes the menu, and typing jumps to the first item
//!   starting with the typed text.
//! - AccessKit sees a `Menu` of `MenuItem`, `MenuItemCheckBox` and
//!   `MenuItemRadio` nodes with their checked state; disabled items are
//!   reported disabled and cannot be clicked.
//! - Ids derive from the id you give the menu, never from its position.

use std::sync::Arc;

use egui::{
    self, Align2, Color32, Context, CornerRadius, Event, EventFilter, Galley, Id, Key, Modifiers,
    Pos2, Rect, Response, Role, Sense, Stroke, Ui, Vec2,
};

use crate::material_symbol::material_symbol_text;
use crate::material_theme::MaterialTheme;
use crate::paint::{paint_elevation, paint_focus_ring};
use crate::tokens::state::{DISABLED_CONTENT_OPACITY, FOCUS_OPACITY, PRESSED_OPACITY};
use crate::tokens::{ColorRole, Elevation, InteractionState, LayoutDirection, TypeRole};

/// `DropdownMenuItemDefaultMinWidth`: 112 dp.
pub const MENU_MIN_WIDTH: f32 = 112.0;
/// `DropdownMenuItemDefaultMaxWidth`: 280 dp.
pub const MENU_MAX_WIDTH: f32 = 280.0;
/// `MenuListItemContainerHeight`: 48 dp, a baseline menu item.
pub const MENU_ITEM_HEIGHT: f32 = 48.0;
/// `SegmentedMenuTokens.Item`: 44 dp, an Expressive menu item.
pub const EXPRESSIVE_MENU_ITEM_HEIGHT: f32 = 44.0;
/// `MenuVerticalMargin`: 48 dp kept free above and below a menu.
const MENU_VERTICAL_MARGIN: f32 = 48.0;
/// `MenuHorizontalMargin`: 8 dp kept free beside a menu.
const MENU_HORIZONTAL_MARGIN: f32 = 8.0;
/// `DropdownMenuItemHorizontalPadding`: 12 dp inside an item and between
/// its icons and its label.
const ITEM_HORIZONTAL_PADDING: f32 = 12.0;
/// `DropdownMenuGroupDefaultMinHeight`: 32 dp, a group label row.
const GROUP_LABEL_HEIGHT: f32 = 32.0;
/// `SegmentedMenuTokens.ItemTopSpace` / `ItemBottomSpace`: 8 dp around an
/// item's text when it has supporting text.
const ITEM_VERTICAL_SPACE: f32 = 8.0;
/// `SegmentedMenuTokens.GroupPadding`: 4 dp around Expressive items.
const GROUP_PADDING: f32 = 4.0;
/// `SegmentedMenuTokens.SegmentedGap`: 2 dp between segmented groups.
const SEGMENTED_GAP: f32 = 2.0;
/// `DividerTokens.Thickness`.
const DIVIDER_THICKNESS: f32 = 1.0;
/// Space above and below a divider (`DropdownMenuVerticalPadding`).
const DIVIDER_SPACE: f32 = 8.0;
/// How long the pointer rests on an item before its submenu opens (or
/// another item's open submenu closes). Not a Compose token: a hover-intent
/// delay, so a submenu does not flash open while the pointer crosses items.
const SUBMENU_HOVER_DELAY: f64 = 0.2;

/// Corner position for menu positioning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    /// Top-left corner.
    TopLeft,
    /// Top-right corner.
    TopRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom-right corner.
    BottomRight,
}

/// Which item is highlighted when the menu opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusState {
    /// None: the first arrow key press highlights the first item.
    None,
    /// The menu itself; same as `None`.
    ListRoot,
    /// The first enabled item, ready for Enter.
    FirstItem,
}

/// Positioning mode for the menu. Kept for compatibility: every mode
/// places the menu in a foreground layer above the rest of the window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Positioning {
    /// Positioned relative to the anchor.
    Absolute,
    /// Positioned relative to the window.
    Fixed,
    /// Positioned relative to the document.
    Document,
    /// A popover layer.
    Popover,
}

/// The colour and shape family of a menu.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuVariant {
    /// The baseline M3 menu (`MenuTokens`): `surfaceContainer`, 4 dp corners.
    #[default]
    Baseline,
    /// M3 Expressive standard menu (`StandardMenuTokens`):
    /// `surfaceContainerLow`, selected items in `tertiaryContainer`.
    Standard,
    /// M3 Expressive vibrant menu (`VibrantMenuTokens`): `tertiaryContainer`,
    /// selected items in `tertiary`.
    Vibrant,
}

impl MenuVariant {
    fn is_expressive(self) -> bool {
        match self {
            MenuVariant::Baseline => false,
            MenuVariant::Standard | MenuVariant::Vibrant => true,
        }
    }
}

/// The visual properties that menus have in common.
///
/// Properties that are `None` use the Material 3 defaults of the menu's
/// [`MenuVariant`].
///
/// # M3 Defaults (baseline)
/// - `background_color`: `surfaceContainer`
/// - `shadow_color`: `shadow`
/// - `elevation`: `3.0` (`Level2`)
/// - `padding`: `8.0` (vertical)
/// - `corner_radius`: `4.0`
/// - `min_width`: `112.0`
/// - `max_width`: `280.0`
#[derive(Clone, Debug, Default)]
pub struct MenuStyle {
    /// The menu's background fill color.
    pub background_color: Option<Color32>,
    /// The shadow color of the menu's surface.
    pub shadow_color: Option<Color32>,
    /// The surface tint color of the menu. M3 no longer tints surfaces;
    /// kept for compatibility and unused.
    pub surface_tint_color: Option<Color32>,
    /// The elevation of the menu in dp (rounded to the nearest M3 level).
    pub elevation: Option<f32>,
    /// The vertical padding inside the menu.
    pub padding: Option<f32>,
    /// The minimum width of the menu.
    pub min_width: Option<f32>,
    /// The maximum width of the menu.
    pub max_width: Option<f32>,
    /// The corner radius of the menu shape.
    pub corner_radius: Option<f32>,
}

impl MenuStyle {
    /// Returns a copy of this `MenuStyle` where `None` fields are
    /// filled in from `other`.
    pub fn merge(&self, other: &MenuStyle) -> MenuStyle {
        MenuStyle {
            background_color: self.background_color.or(other.background_color),
            shadow_color: self.shadow_color.or(other.shadow_color),
            surface_tint_color: self.surface_tint_color.or(other.surface_tint_color),
            elevation: self.elevation.or(other.elevation),
            padding: self.padding.or(other.padding),
            min_width: self.min_width.or(other.min_width),
            max_width: self.max_width.or(other.max_width),
            corner_radius: self.corner_radius.or(other.corner_radius),
        }
    }

    fn resolve(&self, theme: &MaterialTheme, variant: MenuVariant) -> ResolvedMenuStyle {
        let (container, corner, padding) = match variant {
            // MenuTokens.ContainerColor / ContainerShape; DropdownMenuVerticalPadding.
            MenuVariant::Baseline => (ColorRole::SurfaceContainer, 4.0, 8.0),
            // StandardMenuTokens.ContainerColor; SegmentedMenuTokens.ContainerShape
            // (CornerLarge) and GroupPadding.
            MenuVariant::Standard => (ColorRole::SurfaceContainerLow, 16.0, GROUP_PADDING),
            // VibrantMenuTokens.ContainerColor.
            MenuVariant::Vibrant => (ColorRole::TertiaryContainer, 16.0, GROUP_PADDING),
        };
        let min_width = self.min_width.unwrap_or(MENU_MIN_WIDTH).max(0.0);
        ResolvedMenuStyle {
            background_color: self
                .background_color
                .unwrap_or_else(|| theme.color(container)),
            shadow_color: self
                .shadow_color
                .unwrap_or_else(|| theme.color(ColorRole::Shadow)),
            elevation: elevation_level(self.elevation.unwrap_or(Elevation::Level2.dp())),
            padding: self.padding.unwrap_or(padding).max(0.0),
            min_width,
            max_width: self.max_width.unwrap_or(MENU_MAX_WIDTH).max(min_width),
            corner_radius: self.corner_radius.unwrap_or(corner).max(0.0),
        }
    }
}

/// The M3 elevation level nearest to `dp`.
fn elevation_level(dp: f32) -> Elevation {
    Elevation::ALL
        .into_iter()
        .min_by(|a, b| {
            (a.dp() - dp)
                .abs()
                .partial_cmp(&(b.dp() - dp).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(Elevation::Level2)
}

/// Fully resolved menu style with no optional values.
struct ResolvedMenuStyle {
    background_color: Color32,
    shadow_color: Color32,
    elevation: Elevation,
    padding: f32,
    min_width: f32,
    max_width: f32,
    corner_radius: f32,
}

/// Theme data for menus created by `MaterialMenu`.
#[derive(Clone, Debug, Default)]
pub struct MenuThemeData {
    /// The `MenuStyle` for submenus.
    pub style: Option<MenuStyle>,
}

/// Theme data for menu bars.
#[derive(Clone, Debug, Default)]
pub struct MenuBarThemeData {
    /// The `MenuStyle` for the menu bar.
    pub style: Option<MenuStyle>,
}

/// Theme data for menu items. `None` fields use the M3 defaults of the
/// menu's [`MenuVariant`].
///
/// # M3 Defaults (baseline)
/// - `foreground_color`: `onSurface`
/// - `icon_color`: `onSurfaceVariant`
/// - `disabled_foreground_color`: `onSurface` at 38% opacity
/// - `disabled_icon_color`: `onSurface` at 38% opacity
/// - `hover_overlay_opacity`: `0.08`
/// - `pressed_overlay_opacity`: `0.10`
/// - `min_height`: `48.0`
/// - `icon_size`: `24.0`
/// - `padding_horizontal`: `12.0`
#[derive(Clone, Debug, Default)]
pub struct MenuButtonThemeData {
    /// Text/foreground color for enabled menu items.
    pub foreground_color: Option<Color32>,
    /// Icon color for enabled menu items.
    pub icon_color: Option<Color32>,
    /// Text/foreground color for disabled menu items.
    pub disabled_foreground_color: Option<Color32>,
    /// Icon color for disabled menu items.
    pub disabled_icon_color: Option<Color32>,
    /// Opacity of hover overlay (0.0 to 1.0).
    pub hover_overlay_opacity: Option<f32>,
    /// Opacity of pressed overlay (0.0 to 1.0).
    pub pressed_overlay_opacity: Option<f32>,
    /// Font for menu item text (the label type role's size by default).
    pub text_font: Option<egui::FontId>,
    /// Minimum height of a menu item.
    pub min_height: Option<f32>,
    /// Size of leading/trailing icons.
    pub icon_size: Option<f32>,
    /// Horizontal padding inside each menu item.
    pub padding_horizontal: Option<f32>,
}

impl MenuButtonThemeData {
    fn resolve(&self, theme: &MaterialTheme, variant: MenuVariant) -> ResolvedItemTheme {
        let on_surface = theme.color(ColorRole::OnSurface);
        // StandardMenuTokens / VibrantMenuTokens Item*Color; Menu.kt colours.
        let (label, icon, supporting, selected_container, selected_content) = match variant {
            MenuVariant::Baseline => (
                ColorRole::OnSurface,
                ColorRole::OnSurfaceVariant,
                ColorRole::OnSurfaceVariant,
                ColorRole::SecondaryContainer,
                ColorRole::OnSecondaryContainer,
            ),
            MenuVariant::Standard => (
                ColorRole::OnSurface,
                ColorRole::OnSurfaceVariant,
                ColorRole::OnSurfaceVariant,
                ColorRole::TertiaryContainer,
                ColorRole::OnTertiaryContainer,
            ),
            MenuVariant::Vibrant => (
                ColorRole::OnTertiaryContainer,
                ColorRole::OnTertiaryContainer,
                ColorRole::OnTertiaryContainer,
                ColorRole::Tertiary,
                ColorRole::OnTertiary,
            ),
        };
        let disabled = on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY);
        let (label_role, default_height, default_icon) = if variant.is_expressive() {
            // SegmentedMenuTokens.ItemLabelTextFont, Item, ItemLeadingIconSize.
            (TypeRole::BodyLarge, EXPRESSIVE_MENU_ITEM_HEIGHT, 20.0)
        } else {
            // Menu.kt labelLarge; MenuListItemContainerHeight; ListTokens.ItemLeadingIconSize.
            (TypeRole::LabelLarge, MENU_ITEM_HEIGHT, 24.0)
        };
        ResolvedItemTheme {
            foreground_color: self.foreground_color.unwrap_or_else(|| theme.color(label)),
            icon_color: self.icon_color.unwrap_or_else(|| theme.color(icon)),
            supporting_color: theme.color(supporting),
            selected_container: theme.color(selected_container),
            selected_content: theme.color(selected_content),
            disabled_foreground_color: self.disabled_foreground_color.unwrap_or(disabled),
            disabled_icon_color: self.disabled_icon_color.unwrap_or(disabled),
            hover_overlay_opacity: self
                .hover_overlay_opacity
                .unwrap_or(crate::tokens::state::HOVER_OPACITY),
            pressed_overlay_opacity: self.pressed_overlay_opacity.unwrap_or(PRESSED_OPACITY),
            text_size: self.text_font.as_ref().map(|font| font.size),
            label_role,
            min_height: self.min_height.unwrap_or(default_height).max(0.0),
            icon_size: self.icon_size.unwrap_or(default_icon).max(0.0),
            padding_horizontal: self
                .padding_horizontal
                .unwrap_or(ITEM_HORIZONTAL_PADDING)
                .max(0.0),
            expressive: variant.is_expressive(),
        }
    }
}

/// Fully resolved item theme with no optional values.
struct ResolvedItemTheme {
    foreground_color: Color32,
    icon_color: Color32,
    supporting_color: Color32,
    selected_container: Color32,
    selected_content: Color32,
    disabled_foreground_color: Color32,
    disabled_icon_color: Color32,
    hover_overlay_opacity: f32,
    pressed_overlay_opacity: f32,
    text_size: Option<f32>,
    label_role: TypeRole,
    min_height: f32,
    icon_size: f32,
    padding_horizontal: f32,
    expressive: bool,
}

/// What a menu item's leading mark shows.
enum ItemKind<'a> {
    Plain,
    Check(bool),
    BoundCheck(&'a mut bool),
    Radio(bool),
}

/// One row of a menu: an item, a divider or a group label.
pub enum MenuEntry<'a> {
    /// A selectable item.
    Item(MenuItem<'a>),
    /// A divider line (in a segmented menu, a gap between groups).
    Divider,
    /// A group label: a short, non-selectable heading.
    Group(String),
}

impl<'a> From<MenuItem<'a>> for MenuEntry<'a> {
    fn from(item: MenuItem<'a>) -> Self {
        MenuEntry::Item(item)
    }
}

/// Material Design menu component.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{menu_item, MaterialMenu};
///
/// let mut menu_open = false;
/// let button = ui.button("Edit");
/// if button.clicked() {
///     menu_open = true;
/// }
/// MaterialMenu::new(egui::Id::unique("edit_menu"), &mut menu_open)
///     .anchor_rect(button.rect)
///     .item(menu_item("Cut").leading_icon("content_cut").trailing_text("Ctrl+X"))
///     .item(menu_item("Copy").leading_icon("content_copy").trailing_text("Ctrl+C"))
///     .divider()
///     .item(menu_item("Paste").enabled(false))
///     .show(ui.ctx());
/// # });
/// ```
#[must_use = "Call `.show(ctx)` to display the menu"]
pub struct MaterialMenu<'a> {
    id: Id,
    open: Option<&'a mut bool>,
    anchor_rect: Option<Rect>,
    entries: Vec<MenuEntry<'a>>,
    anchor_corner: Corner,
    menu_corner: Corner,
    default_focus: FocusState,
    stay_open_on_outside_click: bool,
    skip_restore_focus: bool,
    x_offset: f32,
    y_offset: f32,
    no_horizontal_flip: bool,
    no_vertical_flip: bool,
    typeahead_delay: f32,
    variant: MenuVariant,
    segmented: bool,
    menu_style: Option<MenuStyle>,
    button_theme: Option<MenuButtonThemeData>,
}

/// Individual menu item.
pub struct MenuItem<'a> {
    text: String,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    supporting_text: Option<String>,
    trailing_text: Option<String>,
    enabled: bool,
    divider_after: bool,
    keep_open: bool,
    kind: ItemKind<'a>,
    submenu: Vec<MenuEntry<'a>>,
    action: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> MaterialMenu<'a> {
    /// A menu shown while `*open` is true. Clicking an item, clicking
    /// outside or pressing Escape sets it to false.
    pub fn new(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self::with_open(id.into(), Some(open))
    }

    /// A context menu: no open flag of its own; call
    /// [`MaterialMenu::show_context`] with the response it belongs to.
    pub fn context(id: impl Into<Id>) -> Self {
        Self::with_open(id.into(), None)
    }

    fn with_open(id: Id, open: Option<&'a mut bool>) -> Self {
        Self {
            id,
            open,
            anchor_rect: None,
            entries: Vec::new(),
            anchor_corner: Corner::BottomLeft,
            menu_corner: Corner::TopLeft,
            default_focus: FocusState::None,
            stay_open_on_outside_click: false,
            skip_restore_focus: false,
            x_offset: 0.0,
            y_offset: 0.0,
            no_horizontal_flip: false,
            no_vertical_flip: false,
            typeahead_delay: 200.0,
            variant: MenuVariant::Baseline,
            segmented: false,
            menu_style: None,
            button_theme: None,
        }
    }

    /// The rectangle the menu is anchored to (usually the button that
    /// opened it). Without one the menu is centred in the window.
    pub fn anchor_rect(mut self, rect: Rect) -> Self {
        self.anchor_rect = Some(rect);
        self
    }

    /// Adds an item.
    pub fn item(mut self, item: MenuItem<'a>) -> Self {
        self.entries.push(MenuEntry::Item(item));
        self
    }

    /// Adds a divider: a line between groups of items (a gap in a
    /// segmented menu).
    pub fn divider(mut self) -> Self {
        self.entries.push(MenuEntry::Divider);
        self
    }

    /// Starts a new group with a label row. Groups after the first are
    /// separated from the one before by a divider (or a gap when segmented).
    pub fn group(mut self, label: impl Into<String>) -> Self {
        if self.entries.iter().any(|e| matches!(e, MenuEntry::Item(_))) {
            self.entries.push(MenuEntry::Divider);
        }
        self.entries.push(MenuEntry::Group(label.into()));
        self
    }

    /// Adds any entry.
    pub fn entry(mut self, entry: impl Into<MenuEntry<'a>>) -> Self {
        self.entries.push(entry.into());
        self
    }

    /// The colour and shape family. Default [`MenuVariant::Baseline`].
    pub fn variant(mut self, variant: MenuVariant) -> Self {
        self.variant = variant;
        self
    }

    /// M3 Expressive segmented menu: every group sits in its own container,
    /// separated by a 2 dp gap. Implies [`MenuVariant::Standard`] when the
    /// variant is still baseline.
    pub fn segmented(mut self, segmented: bool) -> Self {
        self.segmented = segmented;
        if segmented && self.variant == MenuVariant::Baseline {
            self.variant = MenuVariant::Standard;
        }
        self
    }

    /// Set the menu style, overriding Material Design 3 defaults.
    pub fn style(mut self, style: MenuStyle) -> Self {
        self.menu_style = Some(style);
        self
    }

    /// Set the item theme, overriding Material Design 3 defaults.
    pub fn button_theme(mut self, theme: MenuButtonThemeData) -> Self {
        self.button_theme = Some(theme);
        self
    }

    /// Set the elevation (shadow) of the menu, in dp; rounded to the
    /// nearest M3 elevation level.
    pub fn elevation(mut self, elevation: f32) -> Self {
        let style = self.menu_style.get_or_insert_with(MenuStyle::default);
        style.elevation = Some(elevation);
        self
    }

    /// The corner of the anchor the menu attaches to. Default bottom-left.
    pub fn anchor_corner(mut self, corner: Corner) -> Self {
        self.anchor_corner = corner;
        self
    }

    /// The corner of the menu placed at the anchor corner. Default top-left.
    pub fn menu_corner(mut self, corner: Corner) -> Self {
        self.menu_corner = corner;
        self
    }

    /// Which item is highlighted when the menu opens.
    pub fn default_focus(mut self, focus: FocusState) -> Self {
        self.default_focus = focus;
        self
    }

    /// Kept for compatibility; every mode draws the menu in a foreground
    /// layer.
    pub fn positioning(self, _positioning: Positioning) -> Self {
        self
    }

    /// Kept for compatibility; the menu does not animate.
    pub fn quick(self, _quick: bool) -> Self {
        self
    }

    /// Kept for compatibility; a menu taller than the window always
    /// scrolls.
    pub fn has_overflow(self, _has_overflow: bool) -> Self {
        self
    }

    /// Keep the menu open when clicking outside of it.
    pub fn stay_open_on_outside_click(mut self, stay_open: bool) -> Self {
        self.stay_open_on_outside_click = stay_open;
        self
    }

    /// Kept for compatibility; the menu closes on outside clicks and
    /// Escape, not on focus changes.
    pub fn stay_open_on_focusout(self, _stay_open: bool) -> Self {
        self
    }

    /// Do not give keyboard focus back to the previously focused widget
    /// when the menu closes.
    pub fn skip_restore_focus(mut self, skip: bool) -> Self {
        self.skip_restore_focus = skip;
        self
    }

    /// Horizontal offset from the anchor.
    pub fn x_offset(mut self, offset: f32) -> Self {
        self.x_offset = offset;
        self
    }

    /// Vertical offset from the anchor.
    pub fn y_offset(mut self, offset: f32) -> Self {
        self.y_offset = offset;
        self
    }

    /// Never flip to the other side of the anchor horizontally.
    pub fn no_horizontal_flip(mut self, no_flip: bool) -> Self {
        self.no_horizontal_flip = no_flip;
        self
    }

    /// Never flip above the anchor.
    pub fn no_vertical_flip(mut self, no_flip: bool) -> Self {
        self.no_vertical_flip = no_flip;
        self
    }

    /// How long typed characters accumulate for typeahead, in milliseconds.
    pub fn typeahead_delay(mut self, delay: f32) -> Self {
        self.typeahead_delay = delay.max(0.0);
        self
    }

    /// Kept for compatibility; menu items are reached with the arrow keys.
    pub fn list_tab_index(self, _index: i32) -> Self {
        self
    }

    /// Shows the menu, if open.
    pub fn show(mut self, ctx: &Context) {
        let Some(open) = self.open.take() else {
            return;
        };
        if !*open {
            reset_state(ctx, self.id);
            return;
        }
        self.show_open(ctx, open);
    }

    /// Shows this menu as the context menu of `response`: it opens at the
    /// pointer on a secondary click (or on the context-menu key while the
    /// widget has focus) and closes like any menu.
    pub fn show_context(mut self, response: &Response) {
        let ctx = response.ctx.clone();
        let key = self.id.with("context_menu_at");
        let menu_key = response.has_focus()
            && ctx.input(|i| i.modifiers.shift_only() && i.key_pressed(Key::F10));
        if response.secondary_clicked() || menu_key {
            let pos = if response.secondary_clicked() {
                ctx.input(|i| i.pointer.interact_pos())
                    .unwrap_or(response.rect.center())
            } else {
                response.rect.left_bottom()
            };
            ctx.data_mut(|d| d.insert_temp(key, pos));
            reset_state(&ctx, self.id);
        }
        let Some(pos) = ctx.data(|d| d.get_temp::<Pos2>(key)) else {
            return;
        };
        let mut open = true;
        self.anchor_rect = Some(Rect::from_min_size(pos, Vec2::ZERO));
        self.anchor_corner = Corner::TopLeft;
        self.menu_corner = Corner::TopLeft;
        self.open = None;
        let id = self.id;
        self.show_open(&ctx, &mut open);
        if !open {
            ctx.data_mut(|d| d.remove::<Pos2>(key));
            reset_state(&ctx, id);
        }
    }

    fn show_open(mut self, ctx: &Context, open: &mut bool) {
        let theme = MaterialTheme::of(ctx);
        let style = self
            .menu_style
            .clone()
            .unwrap_or_default()
            .resolve(&theme, self.variant);
        let items = self
            .button_theme
            .clone()
            .unwrap_or_default()
            .resolve(&theme, self.variant);
        let root = root_id(self.id);

        // Frames since opening: a click spans two frames, so outside-click
        // detection waits until the opening click is over.
        let now = ctx.input(|i| i.time);
        let mut top: TopState = ctx
            .data(|d| d.get_temp(root.with("top")))
            .unwrap_or_default();
        let just_opened = top.frames_open == 0;
        if just_opened {
            top.restore_focus = ctx.memory(|m| m.focused());
            let mut level = LevelState::default();
            if self.default_focus == FocusState::FirstItem {
                level.active = next_enabled(&self.entries, None, 1);
            }
            ctx.data_mut(|d| d.insert_temp(level_id(root, &[]), level));
        }
        top.frames_open = top.frames_open.saturating_add(1);

        let config = LevelConfig {
            style: &style,
            items: &items,
            theme: &theme,
            segmented: self.segmented,
            typeahead_delay: f64::from(self.typeahead_delay) / 1000.0,
            now,
        };
        let screen = ctx.content_rect();
        let size = level_size(ctx, &self.entries, &config);
        let placement = match self.anchor_rect {
            Some(anchor) => place_at_anchor(
                anchor,
                size,
                AnchorSpec {
                    anchor_corner: self.anchor_corner,
                    menu_corner: self.menu_corner,
                    offset: Vec2::new(self.x_offset, self.y_offset),
                    no_horizontal_flip: self.no_horizontal_flip,
                    no_vertical_flip: self.no_vertical_flip,
                },
                screen,
            ),
            None => Rect::from_center_size(screen.center(), size),
        };
        let mut outcome = Outcome::default();
        let close_self = show_level(
            ctx,
            root,
            &[],
            &mut self.entries,
            placement,
            &config,
            &mut outcome,
        );

        let mut close = outcome.close_all || close_self;
        if !close && !self.stay_open_on_outside_click && top.frames_open > 2 {
            let clicked_at = ctx.input(|i| {
                if i.pointer.any_pressed() {
                    i.pointer.interact_pos()
                } else {
                    None
                }
            });
            if let Some(pos) = clicked_at {
                let inside = outcome.rects.iter().any(|r| r.contains(pos))
                    || self.anchor_rect.is_some_and(|a| a.contains(pos));
                if !inside {
                    close = true;
                }
            }
        }

        if close {
            *open = false;
            if !self.skip_restore_focus {
                if let Some(previous) = top.restore_focus {
                    ctx.memory_mut(|m| m.request_focus(previous));
                }
            }
            reset_state(ctx, self.id);
        } else {
            ctx.data_mut(|d| d.insert_temp(root.with("top"), top));
        }
    }
}

fn root_id(id: Id) -> Id {
    id.with("material_menu")
}

fn level_id(root: Id, path: &[usize]) -> Id {
    root.with(("level", path.to_vec()))
}

/// Forgets a menu's highlight, open submenus and frame count.
fn reset_state(ctx: &Context, id: Id) {
    let root = root_id(id);
    ctx.data_mut(|d| {
        d.remove::<TopState>(root.with("top"));
        d.remove::<LevelState>(level_id(root, &[]));
    });
}

#[derive(Clone, Debug, Default)]
struct TopState {
    frames_open: u32,
    restore_focus: Option<Id>,
}

#[derive(Clone, Debug, Default)]
struct LevelState {
    /// Highlighted entry (keyboard or hover).
    active: Option<usize>,
    /// Entry whose submenu is open.
    open_sub: Option<usize>,
    /// Entry the pointer rests on, and since when.
    hover_since: Option<(usize, f64)>,
    typeahead: String,
    typeahead_at: f64,
}

#[derive(Default)]
struct Outcome {
    rects: Vec<Rect>,
    close_all: bool,
}

struct LevelConfig<'t> {
    style: &'t ResolvedMenuStyle,
    items: &'t ResolvedItemTheme,
    theme: &'t MaterialTheme,
    segmented: bool,
    typeahead_delay: f64,
    now: f64,
}

struct AnchorSpec {
    anchor_corner: Corner,
    menu_corner: Corner,
    offset: Vec2,
    no_horizontal_flip: bool,
    no_vertical_flip: bool,
}

fn corner_point(rect: Rect, corner: Corner) -> Pos2 {
    match corner {
        Corner::TopLeft => rect.left_top(),
        Corner::TopRight => rect.right_top(),
        Corner::BottomLeft => rect.left_bottom(),
        Corner::BottomRight => rect.right_bottom(),
    }
}

fn flip_vertical(corner: Corner) -> Corner {
    match corner {
        Corner::TopLeft => Corner::BottomLeft,
        Corner::TopRight => Corner::BottomRight,
        Corner::BottomLeft => Corner::TopLeft,
        Corner::BottomRight => Corner::TopRight,
    }
}

fn flip_horizontal(corner: Corner) -> Corner {
    match corner {
        Corner::TopLeft => Corner::TopRight,
        Corner::TopRight => Corner::TopLeft,
        Corner::BottomLeft => Corner::BottomRight,
        Corner::BottomRight => Corner::BottomLeft,
    }
}

/// The menu rect for `size` attached to `anchor`, flipped to the other
/// side of the anchor when it would leave `screen`, then kept inside the
/// window margins.
fn place_at_anchor(anchor: Rect, size: Vec2, spec: AnchorSpec, screen: Rect) -> Rect {
    let rect_for = |anchor_corner: Corner, menu_corner: Corner| {
        let point = corner_point(anchor, anchor_corner);
        let origin = match menu_corner {
            Corner::TopLeft => point,
            Corner::TopRight => point - Vec2::new(size.x, 0.0),
            Corner::BottomLeft => point - Vec2::new(0.0, size.y),
            Corner::BottomRight => point - size,
        };
        Rect::from_min_size(origin + spec.offset, size)
    };
    let bounds = screen.shrink2(Vec2::new(MENU_HORIZONTAL_MARGIN, 0.0));
    let (mut anchor_corner, mut menu_corner) = (spec.anchor_corner, spec.menu_corner);
    let mut rect = rect_for(anchor_corner, menu_corner);
    let overflow_y =
        |r: Rect| (r.max.y - bounds.max.y).max(0.0) + (bounds.min.y - r.min.y).max(0.0);
    let overflow_x =
        |r: Rect| (r.max.x - bounds.max.x).max(0.0) + (bounds.min.x - r.min.x).max(0.0);
    if !spec.no_vertical_flip && overflow_y(rect) > 0.0 {
        let flipped = rect_for(flip_vertical(anchor_corner), flip_vertical(menu_corner));
        if overflow_y(flipped) < overflow_y(rect) {
            anchor_corner = flip_vertical(anchor_corner);
            menu_corner = flip_vertical(menu_corner);
            rect = flipped;
        }
    }
    if !spec.no_horizontal_flip && overflow_x(rect) > 0.0 {
        let flipped = rect_for(flip_horizontal(anchor_corner), flip_horizontal(menu_corner));
        if overflow_x(flipped) < overflow_x(rect) {
            rect = flipped;
        }
    }
    keep_inside(rect, screen)
}

/// Moves `rect` inside the window margins (shrinking its height when the
/// window is too short; the menu then scrolls).
fn keep_inside(rect: Rect, screen: Rect) -> Rect {
    let vertical_margin = if screen.height() > 4.0 * MENU_VERTICAL_MARGIN {
        MENU_VERTICAL_MARGIN
    } else {
        0.0
    };
    let bounds = screen.shrink2(Vec2::new(MENU_HORIZONTAL_MARGIN, vertical_margin));
    let height = rect.height().min(bounds.height().max(0.0));
    let width = rect.width();
    let x = rect
        .min
        .x
        .min(bounds.max.x - width)
        .max(bounds.min.x.min(rect.min.x));
    let y = rect.min.y.min(bounds.max.y - height).max(bounds.min.y);
    Rect::from_min_size(Pos2::new(x, y), Vec2::new(width, height))
}

/// Laid-out text of one item.
struct ItemText {
    label: Arc<Galley>,
    supporting: Option<Arc<Galley>>,
    trailing: Option<Arc<Galley>>,
}

fn label_style(items: &ResolvedItemTheme) -> crate::tokens::TypeStyle {
    let mut style = items.label_role.style();
    if let Some(size) = items.text_size {
        style.size = size;
    }
    style
}

fn layout_text(ctx: &Context, text: &str, role: crate::tokens::TypeStyle) -> Arc<Galley> {
    let job = role.layout_job(ctx, text, Color32::PLACEHOLDER);
    ctx.fonts_mut(|f| f.layout_job(job))
}

fn item_text(ctx: &Context, item: &MenuItem<'_>, items: &ResolvedItemTheme) -> ItemText {
    let trailing_role = if items.expressive {
        // SegmentedMenuTokens.ItemTrailingSupportingTextFont.
        TypeRole::LabelSmall
    } else {
        items.label_role
    };
    ItemText {
        label: layout_text(ctx, &item.text, label_style(items)),
        supporting: item
            .supporting_text
            .as_deref()
            // SegmentedMenuTokens.ItemSupportingTextFont.
            .map(|t| layout_text(ctx, t, TypeRole::BodyMedium.style())),
        trailing: item
            .trailing_text
            .as_deref()
            .map(|t| layout_text(ctx, t, trailing_role.style())),
    }
}

fn has_leading(item: &MenuItem<'_>) -> bool {
    item.leading_icon.is_some()
        || match item.kind {
            ItemKind::Plain => false,
            ItemKind::Check(_) | ItemKind::BoundCheck(_) | ItemKind::Radio(_) => true,
        }
}

fn has_trailing_icon(item: &MenuItem<'_>) -> bool {
    item.trailing_icon.is_some() || !item.submenu.is_empty()
}

fn item_height(item: &MenuItem<'_>, items: &ResolvedItemTheme) -> f32 {
    if item.supporting_text.is_some() {
        let label = label_style(items).line_height;
        let supporting = TypeRole::BodyMedium.style().line_height;
        (ITEM_VERTICAL_SPACE * 2.0 + label + supporting).max(items.min_height)
    } else {
        items.min_height
    }
}

/// Heights of every row, in entry order.
fn row_heights(entries: &[MenuEntry<'_>], items: &ResolvedItemTheme, segmented: bool) -> Vec<f32> {
    entries
        .iter()
        .map(|entry| match entry {
            MenuEntry::Item(item) => {
                item_height(item, items)
                    + if item.divider_after {
                        divider_height(segmented)
                    } else {
                        0.0
                    }
            }
            MenuEntry::Divider => divider_height(segmented),
            MenuEntry::Group(_) => GROUP_LABEL_HEIGHT,
        })
        .collect()
}

fn divider_height(segmented: bool) -> f32 {
    if segmented {
        SEGMENTED_GAP + 2.0 * GROUP_PADDING
    } else {
        DIVIDER_THICKNESS + 2.0 * DIVIDER_SPACE
    }
}

/// The content width an item needs (between the item's own edges).
fn item_width(item: &MenuItem<'_>, text: &ItemText, items: &ResolvedItemTheme, lead: bool) -> f32 {
    let pad = items.padding_horizontal;
    let mut width = pad * 2.0;
    if lead {
        width += items.icon_size + ITEM_HORIZONTAL_PADDING;
    }
    let supporting = text.supporting.as_ref().map_or(0.0, |g| g.size().x);
    width += text.label.size().x.max(supporting);
    if let Some(trailing) = &text.trailing {
        width += ITEM_HORIZONTAL_PADDING + trailing.size().x;
    }
    if has_trailing_icon(item) {
        width += ITEM_HORIZONTAL_PADDING + items.icon_size;
    }
    width
}

/// Horizontal inset of items from the container edge.
fn item_inset(config: &LevelConfig<'_>) -> f32 {
    if config.items.expressive {
        GROUP_PADDING
    } else {
        0.0
    }
}

fn level_size(ctx: &Context, entries: &[MenuEntry<'_>], config: &LevelConfig<'_>) -> Vec2 {
    let lead = entries.iter().any(|e| match e {
        MenuEntry::Item(item) => has_leading(item),
        MenuEntry::Divider | MenuEntry::Group(_) => false,
    });
    let content = entries
        .iter()
        .map(|entry| match entry {
            MenuEntry::Item(item) => item_width(
                item,
                &item_text(ctx, item, config.items),
                config.items,
                lead,
            ),
            MenuEntry::Group(label) => {
                layout_text(ctx, label, TypeRole::LabelMedium.style())
                    .size()
                    .x
                    + 2.0 * config.items.padding_horizontal
            }
            MenuEntry::Divider => 0.0,
        })
        .fold(0.0, f32::max);
    let width =
        (content + 2.0 * item_inset(config)).clamp(config.style.min_width, config.style.max_width);
    let height = row_heights(entries, config.items, config.segmented)
        .iter()
        .sum::<f32>()
        + 2.0 * config.style.padding;
    Vec2::new(width, height)
}

/// Index of the next enabled item after `from` (or the first, when `from`
/// is `None`), stepping by `step` (+1 or -1) and wrapping.
fn next_enabled(entries: &[MenuEntry<'_>], from: Option<usize>, step: isize) -> Option<usize> {
    let count = entries.len();
    if count == 0 {
        return None;
    }
    let len = count as isize;
    let start = match from {
        Some(index) => index as isize,
        None if step > 0 => -1,
        None => len,
    };
    (1..=len)
        .map(|k| (start + k * step).rem_euclid(len) as usize)
        .find(|&i| is_enabled_item(entries, i))
}

fn is_enabled_item(entries: &[MenuEntry<'_>], index: usize) -> bool {
    matches!(entries.get(index), Some(MenuEntry::Item(item)) if item.enabled)
}

/// Mirrors `rect` within `row` for right-to-left layouts.
fn mirror(rect: Rect, row: Rect, direction: LayoutDirection) -> Rect {
    match direction {
        LayoutDirection::Ltr => rect,
        LayoutDirection::Rtl => {
            let min_x = row.min.x + row.max.x - rect.max.x;
            Rect::from_min_size(Pos2::new(min_x, rect.min.y), rect.size())
        }
    }
}

fn paint_radio(painter: &egui::Painter, rect: Rect, selected: bool, color: Color32) {
    // The radio_button glyph: a 20-unit ring of 2 units around a 10-unit dot.
    let center = rect.center();
    let scale = rect.width() / 24.0;
    painter.circle_stroke(center, 9.0 * scale, Stroke::new(2.0 * scale, color));
    if selected {
        painter.circle_filled(center, 5.0 * scale, color);
    }
}

/// What the keyboard asked the level to do this frame.
#[derive(Default)]
struct Keys {
    activate: bool,
    open_sub: bool,
    close_sub: bool,
    escape: bool,
}

/// Reads this level's keys (only the deepest open level does).
fn handle_keys(
    ctx: &Context,
    entries: &[MenuEntry<'_>],
    state: &mut LevelState,
    config: &LevelConfig<'_>,
) -> Keys {
    let none = Modifiers::NONE;
    let (open_key, close_key) = match config.theme.layout_direction {
        LayoutDirection::Ltr => (Key::ArrowRight, Key::ArrowLeft),
        LayoutDirection::Rtl => (Key::ArrowLeft, Key::ArrowRight),
    };
    let mut keys = Keys::default();
    let mut typed = String::new();
    ctx.input_mut(|i| {
        if i.consume_key(none, Key::ArrowDown) {
            state.active = next_enabled(entries, state.active, 1);
        }
        if i.consume_key(none, Key::ArrowUp) {
            state.active = next_enabled(entries, state.active, -1);
        }
        if i.consume_key(none, Key::Home) {
            state.active = next_enabled(entries, None, 1);
        }
        if i.consume_key(none, Key::End) {
            state.active = next_enabled(entries, None, -1);
        }
        keys.activate = i.consume_key(none, Key::Enter) || i.consume_key(none, Key::Space);
        keys.open_sub = i.consume_key(none, open_key);
        keys.close_sub = i.consume_key(none, close_key);
        keys.escape = i.consume_key(none, Key::Escape);
        i.events.retain(|event| {
            if let Event::Text(text) = event {
                typed.push_str(text);
                false
            } else {
                true
            }
        });
    });
    let typed = typed.trim().to_lowercase();
    if !typed.is_empty() {
        if config.now - state.typeahead_at > config.typeahead_delay {
            state.typeahead.clear();
        }
        state.typeahead.push_str(&typed);
        state.typeahead_at = config.now;
        // A repeated single letter cycles through the items starting with it.
        let cycling = state.typeahead.chars().count() == 1;
        let start = if cycling {
            state.active
        } else {
            state.active.map(|a| a.saturating_sub(1))
        };
        let count = entries.len();
        let begin = start.map_or(0, |s| s + 1);
        let found = (0..count).map(|k| (begin + k) % count.max(1)).find(|&i| {
            matches!(entries.get(i), Some(MenuEntry::Item(item))
                if item.enabled && item.text.to_lowercase().starts_with(state.typeahead.as_str()))
        });
        if found.is_some() {
            state.active = found;
        }
    }
    keys
}

/// Draws one menu level (and its open submenu, recursively). Returns
/// whether this level asks to close itself (Left/Escape in a submenu, or
/// Escape at the top).
fn show_level(
    ctx: &Context,
    root: Id,
    path: &[usize],
    entries: &mut [MenuEntry<'_>],
    rect: Rect,
    config: &LevelConfig<'_>,
    outcome: &mut Outcome,
) -> bool {
    let id = level_id(root, path);
    let mut state: LevelState = ctx.data(|d| d.get_temp(id)).unwrap_or_default();
    let direction = config.theme.layout_direction;
    let is_deepest = state.open_sub.is_none();
    let keys = if is_deepest {
        handle_keys(ctx, entries, &mut state, config)
    } else {
        Keys::default()
    };
    let mut close_self = keys.escape || (keys.close_sub && !path.is_empty());

    let heights = row_heights(entries, config.items, config.segmented);
    let lead = entries.iter().any(|e| match e {
        MenuEntry::Item(item) => has_leading(item),
        MenuEntry::Divider | MenuEntry::Group(_) => false,
    });
    let texts: Vec<Option<ItemText>> = entries
        .iter()
        .map(|entry| match entry {
            MenuEntry::Item(item) => Some(item_text(ctx, item, config.items)),
            MenuEntry::Divider | MenuEntry::Group(_) => None,
        })
        .collect();

    let style = config.style;
    let items = config.items;
    let pad = style.padding;
    let inset = item_inset(config);
    let mut clicked: Option<usize> = None;
    let mut hovered_row: Option<usize> = None;
    let mut row_rects: Vec<Rect> = Vec::with_capacity(entries.len());

    egui::Area::new(id)
        .order(egui::Order::Foreground)
        .fixed_pos(rect.min)
        .constrain(false)
        .interactable(true)
        .show(ctx, |ui| {
            let container = rect;
            let content_height: f32 = heights.iter().sum::<f32>() + 2.0 * pad;
            let corner = CornerRadius::same(style.corner_radius.round().clamp(0.0, 255.0) as u8);
            let painter = ui.painter().clone();
            if config.segmented {
                // One container per group, separated by gaps: painted per
                // group below.
            } else {
                paint_elevation(
                    &painter,
                    container,
                    corner,
                    style.elevation,
                    style.shadow_color,
                );
                painter.rect_filled(container, corner, style.background_color);
            }
            let (menu_rect, menu_response) =
                ui.allocate_exact_size(container.size(), Sense::hover());
            crate::a11y::describe(&menu_response, Role::Menu, true, "", None);

            if config.segmented {
                paint_segment_groups(&painter, entries, &heights, menu_rect, config);
            }

            let scroll = content_height > container.height() + 0.5;
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(menu_rect)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            child.set_clip_rect(menu_rect.intersect(ui.clip_rect()));
            let mut draw_rows = |ui: &mut Ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                ui.add_space(pad);
                for (index, entry) in entries.iter_mut().enumerate() {
                    let height = heights.get(index).copied().unwrap_or(0.0);
                    let (row, _) = ui
                        .allocate_exact_size(Vec2::new(menu_rect.width(), height), Sense::hover());
                    row_rects.push(row);
                    match entry {
                        MenuEntry::Divider => {
                            if !config.segmented {
                                let y = row.center().y;
                                let line = Rect::from_min_max(
                                    Pos2::new(row.min.x, y - DIVIDER_THICKNESS / 2.0),
                                    Pos2::new(row.max.x, y + DIVIDER_THICKNESS / 2.0),
                                );
                                ui.painter().rect_filled(
                                    line,
                                    0.0,
                                    config.theme.color(ColorRole::OutlineVariant),
                                );
                            }
                        }
                        MenuEntry::Group(label) => {
                            let galley = layout_text(ctx, label, TypeRole::LabelMedium.style());
                            let text_rect = Rect::from_min_size(
                                Pos2::new(
                                    row.min.x + inset + items.padding_horizontal,
                                    row.center().y - galley.size().y / 2.0,
                                ),
                                galley.size(),
                            );
                            let text_rect = mirror(text_rect, row, direction);
                            ui.painter()
                                .galley(text_rect.min, galley, items.supporting_color);
                        }
                        MenuEntry::Item(item) => {
                            let Some(Some(text)) = texts.get(index) else {
                                continue;
                            };
                            let item_rect = Rect::from_min_size(
                                Pos2::new(row.min.x + inset, row.min.y),
                                Vec2::new(row.width() - 2.0 * inset, item_height(item, items)),
                            );
                            let response = ui.interact(
                                item_rect,
                                id.with(("item", index)),
                                crate::a11y::sense(item.enabled, Sense::click()),
                            );
                            let (role, checked) = match &item.kind {
                                ItemKind::Plain => (Role::MenuItem, None),
                                ItemKind::Check(on) => (Role::MenuItemCheckBox, Some(*on)),
                                ItemKind::BoundCheck(on) => (Role::MenuItemCheckBox, Some(**on)),
                                ItemKind::Radio(on) => (Role::MenuItemRadio, Some(*on)),
                            };
                            crate::a11y::describe(
                                &response,
                                role,
                                item.enabled,
                                &item.text,
                                checked,
                            );
                            if response.hovered() && item.enabled {
                                hovered_row = Some(index);
                            }
                            if response.clicked() {
                                clicked = Some(index);
                            }
                            let active = state.active == Some(index);
                            if active && scroll && is_deepest {
                                response.scroll_to_me(None);
                            }
                            paint_item(
                                ui,
                                item,
                                text,
                                item_rect,
                                &response,
                                active,
                                state.open_sub == Some(index),
                                lead,
                                config,
                            );
                            if active && is_deepest {
                                // Keyboard focus follows the highlight, so
                                // screen readers announce it; the arrow keys
                                // stay with the menu.
                                let item_id = response.id;
                                ctx.memory_mut(|m| {
                                    m.request_focus(item_id);
                                    m.set_focus_lock_filter(
                                        item_id,
                                        EventFilter {
                                            tab: true,
                                            horizontal_arrows: true,
                                            vertical_arrows: true,
                                            escape: true,
                                        },
                                    );
                                });
                            }
                            if item.divider_after {
                                let y = item_rect.max.y + divider_height(config.segmented) / 2.0;
                                if !config.segmented {
                                    let line = Rect::from_min_max(
                                        Pos2::new(row.min.x, y - DIVIDER_THICKNESS / 2.0),
                                        Pos2::new(row.max.x, y + DIVIDER_THICKNESS / 2.0),
                                    );
                                    ui.painter().rect_filled(
                                        line,
                                        0.0,
                                        config.theme.color(ColorRole::OutlineVariant),
                                    );
                                }
                            }
                        }
                    }
                }
                ui.add_space(pad);
            };
            if scroll {
                egui::ScrollArea::vertical()
                    .id_salt(id.with("scroll"))
                    .max_height(menu_rect.height())
                    .auto_shrink([false, false])
                    .show(&mut child, |ui| draw_rows(ui));
            } else {
                draw_rows(&mut child);
            }
        });
    outcome.rects.push(rect);

    // Pointer: hovering highlights; resting opens a submenu (hover intent).
    if let Some(index) = hovered_row {
        state.active = Some(index);
        match state.hover_since {
            Some((row, _)) if row == index => {}
            Some(_) | None => state.hover_since = Some((index, config.now)),
        }
    } else {
        state.hover_since = None;
    }
    if let Some((row, since)) = state.hover_since {
        if config.now - since >= SUBMENU_HOVER_DELAY && state.open_sub != Some(row) {
            let has_sub =
                matches!(entries.get(row), Some(MenuEntry::Item(item)) if !item.submenu.is_empty());
            state.open_sub = if has_sub { Some(row) } else { None };
            if has_sub {
                reset_level(ctx, root, path, row);
            }
        } else if state.open_sub != Some(row) {
            ctx.request_repaint_after(std::time::Duration::from_secs_f64(SUBMENU_HOVER_DELAY));
        }
    }

    // Keyboard and click activation.
    let keyboard_target = if keys.activate || keys.open_sub {
        state.active
    } else {
        None
    };
    for (index, from_keyboard) in [(clicked, false), (keyboard_target, true)]
        .into_iter()
        .filter_map(|(i, k)| i.map(|i| (i, k)))
    {
        let Some(MenuEntry::Item(item)) = entries.get_mut(index) else {
            continue;
        };
        if !item.enabled {
            continue;
        }
        if !item.submenu.is_empty() {
            if state.open_sub != Some(index) {
                state.open_sub = Some(index);
                reset_level(ctx, root, path, index);
            }
            if from_keyboard {
                let child_id = level_id(root, &child_path(path, index));
                let first = next_enabled(&item.submenu, None, 1);
                ctx.data_mut(|d| {
                    d.insert_temp(
                        child_id,
                        LevelState {
                            active: first,
                            ..LevelState::default()
                        },
                    );
                });
            }
            continue;
        }
        if from_keyboard && !keys.activate {
            // Right arrow on an item without a submenu does nothing.
            continue;
        }
        match &mut item.kind {
            ItemKind::BoundCheck(on) => **on = !**on,
            ItemKind::Plain | ItemKind::Check(_) | ItemKind::Radio(_) => {}
        }
        if let Some(action) = item.action.as_mut() {
            action();
        }
        if !item.keep_open {
            outcome.close_all = true;
        }
    }

    // The open submenu, beside its item.
    if let Some(sub_index) = state.open_sub {
        let anchor_row = row_rects.get(sub_index).copied();
        match (entries.get_mut(sub_index), anchor_row) {
            (Some(MenuEntry::Item(item)), Some(row)) if !item.submenu.is_empty() => {
                let sub_path = child_path(path, sub_index);
                let size = level_size(ctx, &item.submenu, config);
                let sub_rect = place_submenu(rect, row, size, ctx.content_rect(), direction, pad);
                if show_level(
                    ctx,
                    root,
                    &sub_path,
                    &mut item.submenu,
                    sub_rect,
                    config,
                    outcome,
                ) {
                    state.open_sub = None;
                    state.active = Some(sub_index);
                }
            }
            (Some(_) | None, _) => state.open_sub = None,
        }
    }

    if keys.escape && path.is_empty() {
        close_self = true;
    }
    ctx.data_mut(|d| d.insert_temp(id, state));
    close_self
}

fn child_path(path: &[usize], index: usize) -> Vec<usize> {
    let mut child = path.to_vec();
    child.push(index);
    child
}

/// Clears the state of the submenu at `index`, and below it.
fn reset_level(ctx: &Context, root: Id, path: &[usize], index: usize) {
    let child = level_id(root, &child_path(path, index));
    ctx.data_mut(|d| d.remove::<LevelState>(child));
}

/// Places a submenu beside `row` of the parent menu at `parent`, on the
/// end side, or the start side when it would not fit.
fn place_submenu(
    parent: Rect,
    row: Rect,
    size: Vec2,
    screen: Rect,
    direction: LayoutDirection,
    pad: f32,
) -> Rect {
    let top = row.min.y - pad;
    let after = Rect::from_min_size(Pos2::new(parent.max.x, top), size);
    let before = Rect::from_min_size(Pos2::new(parent.min.x - size.x, top), size);
    let (preferred, other) = match direction {
        LayoutDirection::Ltr => (after, before),
        LayoutDirection::Rtl => (before, after),
    };
    let fits = |r: Rect| r.min.x >= screen.min.x && r.max.x <= screen.max.x;
    let chosen = if fits(preferred) || !fits(other) {
        preferred
    } else {
        other
    };
    keep_inside(chosen, screen)
}

/// The vertical extent of each group container in a segmented menu.
fn group_spans(
    entries: &[MenuEntry<'_>],
    heights: &[f32],
    menu_rect: Rect,
    pad: f32,
    items: &ResolvedItemTheme,
) -> Vec<(f32, f32)> {
    // Group spans [top, bottom) between dividers. A divider row is
    // GROUP_PADDING + SEGMENTED_GAP + GROUP_PADDING tall: the end of one
    // group's padding, the gap, the start of the next group's padding.
    let mut spans: Vec<(f32, f32)> = Vec::new();
    let mut y = menu_rect.min.y + pad;
    let mut start = menu_rect.min.y;
    let mut has_rows = false;
    for (entry, height) in entries.iter().zip(heights) {
        let break_at = match entry {
            MenuEntry::Divider => Some(y),
            MenuEntry::Item(item) if item.divider_after => Some(y + item_height(item, items)),
            MenuEntry::Item(_) | MenuEntry::Group(_) => None,
        };
        let is_item_break = matches!(entry, MenuEntry::Item(_));
        y += height;
        match break_at {
            Some(at) => {
                if has_rows || is_item_break {
                    spans.push((start, at + GROUP_PADDING));
                }
                start = at + GROUP_PADDING + SEGMENTED_GAP;
                has_rows = false;
            }
            None => has_rows = true,
        }
    }
    if has_rows {
        spans.push((start, y + pad));
    }
    spans
}

/// Paints the per-group containers of a segmented menu.
fn paint_segment_groups(
    painter: &egui::Painter,
    entries: &[MenuEntry<'_>],
    heights: &[f32],
    menu_rect: Rect,
    config: &LevelConfig<'_>,
) {
    let spans = group_spans(
        entries,
        heights,
        menu_rect,
        config.style.padding,
        config.items,
    );
    // SegmentedMenuTokens.ContainerShape (outer corners, CornerLarge) and
    // GroupShape (inner corners, CornerSmall).
    let outer = config.style.corner_radius.round().clamp(0.0, 255.0) as u8;
    let inner = 8;
    let count = spans.len();
    for (i, (top, bottom)) in spans.into_iter().enumerate() {
        let rect = Rect::from_min_max(
            Pos2::new(menu_rect.min.x, top.max(menu_rect.min.y)),
            Pos2::new(menu_rect.max.x, bottom.min(menu_rect.max.y)),
        );
        let first = i == 0;
        let last = i + 1 == count;
        let corners = CornerRadius {
            nw: if first { outer } else { inner },
            ne: if first { outer } else { inner },
            sw: if last { outer } else { inner },
            se: if last { outer } else { inner },
        };
        paint_elevation(
            painter,
            rect,
            corners,
            config.style.elevation,
            config.style.shadow_color,
        );
        painter.rect_filled(rect, corners, config.style.background_color);
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "private painter; every input is used"
)]
fn paint_item(
    ui: &Ui,
    item: &MenuItem<'_>,
    text: &ItemText,
    rect: Rect,
    response: &Response,
    active: bool,
    submenu_open: bool,
    lead: bool,
    config: &LevelConfig<'_>,
) {
    let items = config.items;
    let painter = ui.painter();
    let direction = config.theme.layout_direction;
    let selected = match &item.kind {
        ItemKind::Plain => false,
        ItemKind::Check(on) | ItemKind::Radio(on) => *on,
        ItemKind::BoundCheck(on) => **on,
    };
    // Expressive menus fill selected items; the baseline menu marks them
    // with the check or radio only.
    let filled = selected && items.expressive;
    let (label_color, icon_color, supporting_color) = if !item.enabled {
        (
            items.disabled_foreground_color,
            items.disabled_icon_color,
            items.disabled_foreground_color,
        )
    } else if filled {
        (
            items.selected_content,
            items.selected_content,
            items.selected_content,
        )
    } else {
        (
            items.foreground_color,
            items.icon_color,
            items.supporting_color,
        )
    };
    // SegmentedMenuTokens.ItemShape (CornerExtraSmall) and ItemSelectedShape
    // (CornerMedium); baseline items are square.
    let corner = if !items.expressive {
        CornerRadius::ZERO
    } else if filled {
        CornerRadius::same(12)
    } else {
        CornerRadius::same(4)
    };
    if filled {
        let container = if item.enabled {
            items.selected_container
        } else {
            items
                .selected_container
                .gamma_multiply(DISABLED_CONTENT_OPACITY)
        };
        painter.rect_filled(rect, corner, container);
    }
    if item.enabled {
        let state = InteractionState::from_response(response);
        let opacity = if state.pressed {
            items.pressed_overlay_opacity
        } else if active || submenu_open {
            FOCUS_OPACITY.max(if state.hovered {
                items.hover_overlay_opacity
            } else {
                0.0
            })
        } else if state.hovered {
            items.hover_overlay_opacity
        } else {
            0.0
        };
        if opacity > 0.0 {
            painter.rect_filled(rect, corner, label_color.gamma_multiply(opacity));
        }
        if response.has_focus() && response.ctx.memory(|m| m.focused()) == Some(response.id) {
            let ring =
                rect.shrink(crate::paint::FOCUS_RING_OFFSET + crate::paint::FOCUS_RING_WIDTH);
            paint_focus_ring(
                painter,
                ring,
                corner,
                config.theme.color(ColorRole::Secondary),
            );
        }
    }

    let pad = items.padding_horizontal;
    let icon = items.icon_size;
    let mut x = rect.min.x + pad;
    let center_y = rect.center().y;
    let icon_rect =
        |x: f32| Rect::from_min_size(Pos2::new(x, center_y - icon / 2.0), Vec2::splat(icon));
    if lead {
        let slot = mirror(icon_rect(x), rect, direction);
        match &item.kind {
            ItemKind::Check(on) if *on => {
                crate::segmented_button::paint_check(painter, slot, icon_color)
            }
            ItemKind::BoundCheck(on) if **on => {
                crate::segmented_button::paint_check(painter, slot, icon_color);
            }
            ItemKind::Radio(on) => paint_radio(painter, slot, *on, icon_color),
            ItemKind::Plain | ItemKind::Check(_) | ItemKind::BoundCheck(_) => {
                if let Some(name) = &item.leading_icon {
                    painter.text(
                        slot.center(),
                        Align2::CENTER_CENTER,
                        material_symbol_text(name),
                        egui::FontId::proportional(icon),
                        icon_color,
                    );
                }
            }
        }
        x += icon + ITEM_HORIZONTAL_PADDING;
    }

    // Trailing elements, from the end inwards.
    let mut end = rect.max.x - pad;
    let trailing_icon = if item.submenu.is_empty() {
        item.trailing_icon.clone()
    } else {
        Some(
            match direction {
                LayoutDirection::Ltr => "arrow_right",
                LayoutDirection::Rtl => "arrow_left",
            }
            .to_owned(),
        )
    };
    if let Some(name) = trailing_icon {
        let slot = mirror(icon_rect(end - icon), rect, direction);
        painter.text(
            slot.center(),
            Align2::CENTER_CENTER,
            material_symbol_text(&name),
            egui::FontId::proportional(icon),
            icon_color,
        );
        end -= icon + ITEM_HORIZONTAL_PADDING;
    }
    if let Some(trailing) = &text.trailing {
        let size = trailing.size();
        let slot = Rect::from_min_size(Pos2::new(end - size.x, center_y - size.y / 2.0), size);
        let slot = mirror(slot, rect, direction);
        painter.galley(slot.min, trailing.clone(), supporting_color);
        end -= size.x + ITEM_HORIZONTAL_PADDING;
    }

    // Label (and supporting text below it).
    let available = (end - x).max(0.0);
    let label_size = text.label.size();
    let supporting_size = text.supporting.as_ref().map_or(Vec2::ZERO, |g| g.size());
    let block = label_size.y + supporting_size.y;
    let top = center_y - block / 2.0;
    let clip = Rect::from_min_max(
        Pos2::new(x, rect.min.y),
        Pos2::new(x + available, rect.max.y),
    );
    let clip = mirror(clip, rect, direction);
    let text_painter = painter.with_clip_rect(clip.intersect(painter.clip_rect()));
    let label_rect = mirror(
        Rect::from_min_size(Pos2::new(x, top), label_size),
        rect,
        direction,
    );
    text_painter.galley(label_rect.min, text.label.clone(), label_color);
    if let Some(supporting) = &text.supporting {
        let supporting_rect = mirror(
            Rect::from_min_size(Pos2::new(x, top + label_size.y), supporting_size),
            rect,
            direction,
        );
        text_painter.galley(supporting_rect.min, supporting.clone(), supporting_color);
    }
}

impl<'a> MenuItem<'a> {
    /// A menu item showing `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            leading_icon: None,
            trailing_icon: None,
            supporting_text: None,
            trailing_text: None,
            enabled: true,
            divider_after: false,
            keep_open: false,
            kind: ItemKind::Plain,
            submenu: Vec::new(),
            action: None,
        }
    }

    /// A leading icon (a Material Symbols name, e.g. `"content_copy"`).
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// A trailing icon (a Material Symbols name). An item with a submenu
    /// shows the submenu arrow instead.
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// A second line of smaller text below the label.
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Text at the end of the item, usually a keyboard shortcut.
    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    /// Enable or disable the menu item. A disabled item cannot be clicked
    /// or highlighted.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Draw a divider after this item.
    pub fn divider_after(mut self, divider: bool) -> Self {
        self.divider_after = divider;
        self
    }

    /// Keep the menu open after this item is activated (for check and
    /// radio items toggled several times in a row).
    pub fn keep_open(mut self, keep_open: bool) -> Self {
        self.keep_open = keep_open;
        self
    }

    /// A check item showing `checked`. Update your state in
    /// [`MenuItem::on_click`], or use [`MenuItem::toggles`].
    pub fn checked(mut self, checked: bool) -> Self {
        self.kind = ItemKind::Check(checked);
        self
    }

    /// A check item bound to `value`: activating it flips the flag.
    pub fn toggles(mut self, value: &'a mut bool) -> Self {
        self.kind = ItemKind::BoundCheck(value);
        self
    }

    /// A radio item showing `selected`. Update your state in
    /// [`MenuItem::on_click`].
    pub fn radio(mut self, selected: bool) -> Self {
        self.kind = ItemKind::Radio(selected);
        self
    }

    /// A cascading submenu, opened on hover, click or the Right arrow.
    pub fn submenu<I>(mut self, items: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<MenuEntry<'a>>,
    {
        self.submenu = items.into_iter().map(Into::into).collect();
        self
    }

    /// The action run when the item is activated (click, Enter or Space).
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: FnMut() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

/// Shorthand for [`MaterialMenu::new`].
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut menu_open = false;
/// egui_material3::menu(egui::Id::unique("context_menu"), &mut menu_open).show(ui.ctx());
/// # });
/// ```
pub fn menu(id: impl Into<egui::Id>, open: &mut bool) -> MaterialMenu<'_> {
    MaterialMenu::new(id, open)
}

/// Shorthand for [`MenuItem::new`].
///
/// ```rust
/// let item = egui_material3::menu_item("Copy")
///     .leading_icon("content_copy")
///     .on_click(|| println!("Copy action"));
/// ```
pub fn menu_item<'a>(text: impl Into<String>) -> MenuItem<'a> {
    MenuItem::new(text)
}

/// Shows `menu` as the context menu of `response` (secondary click, or
/// Shift+F10 while it has focus). Shorthand for
/// [`MaterialMenu::show_context`].
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{context_menu, menu_item, MaterialMenu};
///
/// let response = ui.label("Right-click me");
/// context_menu(
///     &response,
///     MaterialMenu::context(egui::Id::unique("label_menu")).item(menu_item("Copy")),
/// );
/// # });
/// ```
pub fn context_menu(response: &Response, menu: MaterialMenu<'_>) {
    menu.show_context(response);
}

#[cfg(test)]
mod id_tests {
    use super::*;
    use std::cell::Cell;

    /// Regression test for nikescar/egui-material3#16: two menus at the same
    /// x do not share item ids, so clicking an item of one does not click
    /// the other's.
    #[test]
    fn test_menus_at_same_x_have_distinct_item_ids() {
        let ctx = egui::Context::default();
        let a_clicks = Cell::new(0);
        let b_clicks = Cell::new(0);
        let anchor_a = Rect::from_min_size(Pos2::new(100.0, 50.0), Vec2::new(10.0, 10.0));
        let anchor_b = Rect::from_min_size(Pos2::new(100.0, 400.0), Vec2::new(10.0, 10.0));
        let frame = |events: Vec<egui::Event>| {
            let mut open_a = true;
            let mut open_b = true;
            let input = egui::RawInput {
                events,
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 1200.0))),
                ..Default::default()
            };
            ctx.run_ui(input, |ui| {
                menu(egui::Id::unique("menu_a"), &mut open_a)
                    .anchor_rect(anchor_a)
                    .stay_open_on_outside_click(true)
                    .item(menu_item("A").on_click(|| a_clicks.set(a_clicks.get() + 1)))
                    .show(ui.ctx());
                menu(egui::Id::unique("menu_b"), &mut open_b)
                    .anchor_rect(anchor_b)
                    .stay_open_on_outside_click(true)
                    .item(menu_item("B").on_click(|| b_clicks.set(b_clicks.get() + 1)))
                    .show(ui.ctx());
            })
            .textures_delta
            .clear();
        };
        // Inside menu A's first item: below the anchor, past the padding.
        let pos = Pos2::new(anchor_a.min.x + 40.0, anchor_a.max.y + 28.0);
        let button = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        frame(Vec::new());
        frame(Vec::new());
        frame(vec![egui::Event::PointerMoved(pos), button(true)]);
        frame(vec![button(false)]);
        assert_eq!(a_clicks.get(), 1, "clicking menu A's item should click it");
        assert_eq!(b_clicks.get(), 0, "menu B's item must not be clicked too");
    }

    #[test]
    fn width_is_clamped_to_the_m3_range() {
        let screen = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 1000.0));
        let anchor = Rect::from_min_size(Pos2::new(950.0, 100.0), Vec2::new(40.0, 20.0));
        let spec = AnchorSpec {
            anchor_corner: Corner::BottomLeft,
            menu_corner: Corner::TopLeft,
            offset: Vec2::ZERO,
            no_horizontal_flip: false,
            no_vertical_flip: false,
        };
        let rect = place_at_anchor(anchor, Vec2::new(200.0, 100.0), spec, screen);
        assert!(
            rect.max.x <= 1000.0 - MENU_HORIZONTAL_MARGIN,
            "flipped to stay on screen: {rect:?}"
        );
        assert_eq!(
            rect.max.x, anchor.max.x,
            "flipped menu ends at the anchor's right edge"
        );
    }

    #[test]
    fn menu_flips_above_an_anchor_near_the_bottom() {
        let screen = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 1000.0));
        let anchor = Rect::from_min_size(Pos2::new(100.0, 900.0), Vec2::new(40.0, 20.0));
        let spec = AnchorSpec {
            anchor_corner: Corner::BottomLeft,
            menu_corner: Corner::TopLeft,
            offset: Vec2::ZERO,
            no_horizontal_flip: false,
            no_vertical_flip: false,
        };
        let rect = place_at_anchor(anchor, Vec2::new(200.0, 300.0), spec, screen);
        assert_eq!(
            rect.max.y, anchor.min.y,
            "the menu sits above the anchor: {rect:?}"
        );
    }

    #[test]
    fn next_enabled_skips_disabled_items_and_wraps() {
        let entries: Vec<MenuEntry<'_>> = vec![
            menu_item("a").into(),
            MenuEntry::Divider,
            menu_item("b").enabled(false).into(),
            menu_item("c").into(),
        ];
        assert_eq!(next_enabled(&entries, None, 1), Some(0), "first enabled");
        assert_eq!(
            next_enabled(&entries, Some(0), 1),
            Some(3),
            "skips divider and disabled"
        );
        assert_eq!(next_enabled(&entries, Some(3), 1), Some(0), "wraps forward");
        assert_eq!(
            next_enabled(&entries, Some(0), -1),
            Some(3),
            "wraps backward"
        );
        assert_eq!(next_enabled(&entries, None, -1), Some(3), "last enabled");
    }

    #[test]
    fn segmented_groups_are_two_dp_apart() {
        let theme = MaterialTheme::baseline(false);
        let items = MenuButtonThemeData::default().resolve(&theme, MenuVariant::Standard);
        let entries: Vec<MenuEntry<'_>> = vec![
            menu_item("a").into(),
            menu_item("b").divider_after(true).into(),
            menu_item("c").into(),
            MenuEntry::Divider,
            menu_item("d").into(),
        ];
        let heights = row_heights(&entries, &items, true);
        let rect = Rect::from_min_size(
            Pos2::ZERO,
            Vec2::new(200.0, heights.iter().sum::<f32>() + 8.0),
        );
        let spans = group_spans(&entries, &heights, rect, GROUP_PADDING, &items);
        assert_eq!(spans.len(), 3, "three groups: {spans:?}");
        for pair in spans.windows(2) {
            if let [(_, end), (start, _)] = pair {
                assert_eq!(start - end, SEGMENTED_GAP, "2 dp between groups: {spans:?}");
            }
        }
        assert_eq!(
            spans.first().map(|s| s.0),
            Some(0.0),
            "first group starts at the top"
        );
        assert_eq!(
            spans.last().map(|s| s.1),
            Some(rect.max.y),
            "last group ends at the bottom"
        );
    }

    #[test]
    fn elevation_rounds_to_the_nearest_level() {
        assert_eq!(elevation_level(3.0), Elevation::Level2, "3 dp is Level2");
        assert_eq!(
            elevation_level(7.0),
            Elevation::Level3,
            "7 dp is nearest Level3"
        );
        assert_eq!(
            elevation_level(40.0),
            Elevation::Level5,
            "large values cap at Level5"
        );
    }
}
