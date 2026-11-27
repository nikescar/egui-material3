//! Material Design 3 Tabs Components
//!
//! Values from the Jetpack Compose material3 token files
//! (`androidx.compose.material3.tokens`, `androidx-main`):
//! `PrimaryNavigationTabTokens` and `SecondaryNavigationTabTokens`, plus the
//! layout constants in `Tab.kt` and `TabRow.kt`.
//!
//! ## Primary tabs
//! - **surface** (`ContainerColor`): the bar, unless made transparent
//! - **primary**: selected label, icon and indicator (3 dp, 3 dp top
//!   corners, as wide as the tab's content, at least 24 dp)
//! - **onSurfaceVariant**: unselected label and icon; **onSurface** while
//!   hovered, focused or pressed
//!
//! ## Secondary tabs
//! - **surface**: the bar, unless made transparent
//! - **onSurface**: selected label and icon; **primary**: the indicator
//!   (2 dp, the full tab width)
//! - **onSurfaceVariant**: unselected label and icon
//!
//! ## Both
//! - **outlineVariant**: the 1 dp divider under the bar
//! - **onSurface @ 38%**: disabled label and icon
//! - Labels are `TitleSmall`, icons 24 dp
//! - Height 48 dp (`ContainerHeight`), or 64 dp with icon and label
//!   (`LargeContainerHeight`)
//! - [`TabLayout::Fixed`] splits the width equally; [`TabLayout::Content`]
//!   sizes each tab to its label plus 16 dp each side (`HorizontalTextPadding`);
//!   [`TabLayout::Scrollable`] does the same with a 90 dp minimum and a
//!   52 dp start inset (`ScrollableTabRowMinTabWidth`,
//!   `ScrollableTabRowEdgeStartPadding`) and scrolls.
//! - The indicator slides between tabs with the `DefaultSpatial` spring.

use crate::navigation::{
    describe_item, paint_badge, paint_icon, rove_focus, text_galley, NavigationBadge, RoveAxis,
};
use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{
    animate_value_with_spring, ColorRole, InteractionState, LayoutDirection, SpringSpeed, TypeRole,
};
use crate::MaterialTheme;
use egui::epaint::CornerRadius;
use egui::{self, Color32, Id, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};

/// `PrimaryNavigationTabTokens.ContainerHeight` / `SecondaryNavigationTabTokens.ContainerHeight`.
pub const TAB_HEIGHT: f32 = 48.0;
/// `PrimaryNavigationTabTokens.LargeContainerHeight`: tabs with an icon and
/// a label.
pub const TAB_HEIGHT_WITH_ICON_AND_LABEL: f32 = 64.0;
/// `PrimaryNavigationTabTokens.IconSize`.
pub const TAB_ICON_SIZE: f32 = 24.0;
/// `PrimaryNavigationTabTokens.ActiveIndicatorHeight`.
pub const PRIMARY_INDICATOR_HEIGHT: f32 = 3.0;
/// `SecondaryNavigationTabTokens.ActiveIndicatorHeight`.
pub const SECONDARY_INDICATOR_HEIGHT: f32 = 2.0;
/// `PrimaryNavigationTabTokens.ActiveIndicatorShape`: 3 dp top corners.
const INDICATOR_TOP_ROUNDING: u8 = 3;
/// `TabRow.kt`: the primary indicator is at least 24 dp wide.
pub const PRIMARY_INDICATOR_MIN_WIDTH: f32 = 24.0;
/// `Tab.kt` `HorizontalTextPadding`.
pub const TAB_HORIZONTAL_PADDING: f32 = 16.0;
/// `TabRowDefaults.ScrollableTabRowMinTabWidth`.
pub const SCROLLABLE_MIN_TAB_WIDTH: f32 = 90.0;
/// `TabRowDefaults.ScrollableTabRowEdgeStartPadding`.
pub const SCROLLABLE_EDGE_PADDING: f32 = 52.0;
/// `HorizontalDivider`'s default thickness (`DividerTokens.Thickness`).
const DIVIDER_HEIGHT: f32 = 1.0;
/// Space between a stacked icon and label.
const ICON_LABEL_SPACE: f32 = 2.0;

/// Material Design tabs component.
///
/// Tabs organize content across different screens, data sets, and other interactions.
/// They allow users to navigate between related groups of content.
///
/// The response is `changed()` when a click selects a different tab. Tab
/// reaches each enabled tab; the Left and Right arrows move
/// between tabs, and Space or Enter selects the focused one.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selected_tab = 0;
///
/// ui.add(MaterialTabs::primary(&mut selected_tab)
///     .tab("Home")
///     .tab("Profile")
///     .tab("Settings"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialTabs<'a> {
    /// Reference to the currently selected tab index
    selected: &'a mut usize,
    /// List of tab items
    tabs: Vec<TabItem>,
    /// Whether the tabs are enabled for interaction
    enabled: bool,
    /// Visual variant of the tabs (primary or secondary)
    variant: TabVariant,
    /// Optional salt for generating unique IDs
    id_salt: Option<String>,
    /// Optional custom height for the tab bar
    height: Option<f32>,
    /// How tabs share the width
    layout: TabLayout,
    /// The bar's fill
    container: Container,
    /// Whether to draw the divider under the bar
    divider: bool,
    /// Scrollable layout: inset before the first tab
    edge_padding: f32,
    /// Scrollable layout: narrowest tab
    min_tab_width: f32,
}

/// The bar's fill.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Container {
    /// The theme's `surface`.
    Theme,
    /// A colour of the caller's.
    Color(Color32),
    /// Nothing: whatever is behind shows through.
    Transparent,
}

/// One tab.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialTabs, NavigationBadge, TabItem};
/// let mut selected = 0;
/// ui.add(
///     MaterialTabs::primary(&mut selected)
///         .item(TabItem::new("Inbox").badge(NavigationBadge::Large("12".into())))
///         .item(TabItem::new("Archive").enabled(false)),
/// );
/// # });
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    /// Display label for the tab
    label: String,
    /// Optional icon for the tab
    icon: Option<String>,
    /// Optional custom color (background, text)
    color: Option<(Color32, Color32)>,
    /// Whether this tab can be selected
    enabled: bool,
    /// Optional badge on the icon, or on the label when there is no icon
    badge: Option<NavigationBadge>,
}

impl TabItem {
    /// A tab with a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            color: None,
            enabled: true,
            badge: None,
        }
    }

    /// A Material Symbols icon name (such as `"home"`) or a literal glyph,
    /// drawn above the label.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// A custom background and content colour.
    pub fn color(mut self, background: Color32, content: Color32) -> Self {
        self.color = Some((background, content));
        self
    }

    /// Whether the tab can be selected. A disabled tab is drawn at 38 %, is
    /// skipped by the keyboard and does not respond to clicks.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// A badge on the icon, or on the label when there is no icon.
    pub fn badge(mut self, badge: NavigationBadge) -> Self {
        self.badge = Some(badge);
        self
    }
}

/// Visual variants for tabs component.
#[derive(Clone, Copy, PartialEq)]
pub enum TabVariant {
    /// Primary tabs: top-level destinations, a content-wide indicator
    Primary,
    /// Secondary tabs: within a content area, a full-width indicator
    Secondary,
}

/// How tabs share the bar's width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TabLayout {
    /// Every tab gets an equal share of the full width.
    #[default]
    Fixed,
    /// Each tab is as wide as its content plus 16 dp each side, from the
    /// start edge.
    Content,
    /// Like `Content`, with a minimum tab width and a start inset, in a
    /// horizontal scroll area that keeps the selected tab in view.
    Scrollable,
}

impl<'a> MaterialTabs<'a> {
    /// Create a new tabs component.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected tab index
    /// * `variant` - Visual variant (Primary or Secondary)
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// let tabs = MaterialTabs::new(&mut tab_index, TabVariant::Primary);
    /// # });
    /// ```
    pub fn new(selected: &'a mut usize, variant: TabVariant) -> Self {
        Self {
            selected,
            tabs: Vec::new(),
            enabled: true,
            variant,
            id_salt: None,
            height: None,
            layout: TabLayout::default(),
            container: Container::Theme,
            divider: true,
            edge_padding: SCROLLABLE_EDGE_PADDING,
            min_tab_width: SCROLLABLE_MIN_TAB_WIDTH,
        }
    }

    /// Create a primary tabs component.
    ///
    /// Primary tabs have a filled background and are more prominent.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected tab index
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// let tabs = MaterialTabs::primary(&mut tab_index);
    /// # });
    /// ```
    pub fn primary(selected: &'a mut usize) -> Self {
        Self::new(selected, TabVariant::Primary)
    }

    /// Create a secondary tabs component.
    ///
    /// Secondary tabs have an outlined style and are less prominent than primary tabs.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected tab index
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// let tabs = MaterialTabs::secondary(&mut tab_index);
    /// # });
    /// ```
    pub fn secondary(selected: &'a mut usize) -> Self {
        Self::new(selected, TabVariant::Secondary)
    }

    /// Add a tab with just a text label.
    ///
    /// # Arguments
    /// * `label` - Text label for the tab
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab("Home")
    ///     .tab("About"));
    /// # });
    /// ```
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.tabs.push(TabItem::new(label));
        self
    }

    /// Add a tab with both an icon and text label.
    ///
    /// # Arguments
    /// * `label` - Text label for the tab
    /// * `icon` - Icon identifier (e.g., "home", "person", "settings")
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab_with_icon("Home", "home")
    ///     .tab_with_icon("Profile", "person"));
    /// # });
    /// ```
    pub fn tab_with_icon(mut self, label: impl Into<String>, icon: impl Into<String>) -> Self {
        self.tabs.push(TabItem::new(label).icon(icon));
        self
    }

    /// Add a tab with custom background and text color.
    ///
    /// # Arguments
    /// * `label` - Text label for the tab
    /// * `bg_color` - Background color for the tab
    /// * `text_color` - Text color for the tab
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab_with_color("Red", egui::Color32::from_rgb(255, 0, 0), egui::Color32::WHITE)
    ///     .tab_with_color("Blue", egui::Color32::from_rgb(0, 0, 255), egui::Color32::WHITE));
    /// # });
    /// ```
    pub fn tab_with_color(
        mut self,
        label: impl Into<String>,
        bg_color: Color32,
        text_color: Color32,
    ) -> Self {
        self.tabs
            .push(TabItem::new(label).color(bg_color, text_color));
        self
    }

    /// Set whether the tabs are enabled for interaction. To disable one tab,
    /// add it with [`MaterialTabs::item`] and [`TabItem::enabled`].
    ///
    /// # Arguments
    /// * `enabled` - `true` to enable tabs, `false` to disable
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab("Home")
    ///     .tab("Profile")
    ///     .tab("Settings")
    ///     .enabled(false));
    /// # });
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set an optional salt for generating unique IDs for the tabs.
    ///
    /// This is useful if you have multiple instances of tabs and want to avoid ID collisions.
    ///
    /// # Arguments
    /// * `salt` - Salt string to use in ID generation
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut tab_index = 0;
    /// ui.add(MaterialTabs::primary(&mut tab_index)
    ///     .tab("Home")
    ///     .tab("Profile")
    ///     .tab("Settings")
    ///     .id_salt("unique_salt"));
    /// # });
    /// ```
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Set a custom height for the tab bar.
    ///
    /// Default height is 48 dp, or 64 dp for tabs with both icons and labels.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Add a tab built with [`TabItem`]: a disabled tab, a badge, or any mix.
    pub fn item(mut self, item: TabItem) -> Self {
        self.tabs.push(item);
        self
    }

    /// How the tabs share the width. Default [`TabLayout::Fixed`].
    pub fn layout(mut self, layout: TabLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Shorthand for `.layout(TabLayout::Scrollable)`.
    pub fn scrollable(self) -> Self {
        self.layout(TabLayout::Scrollable)
    }

    /// Scrollable layout: the inset before the first tab (default 52 dp).
    pub fn edge_padding(mut self, padding: f32) -> Self {
        self.edge_padding = padding.max(0.0);
        self
    }

    /// Scrollable layout: the narrowest a tab gets (default 90 dp).
    pub fn min_tab_width(mut self, width: f32) -> Self {
        self.min_tab_width = width.max(0.0);
        self
    }

    /// Fill the bar with `color` instead of the theme's `surface`.
    pub fn container_color(mut self, color: Color32) -> Self {
        self.container = Container::Color(color);
        self
    }

    /// Leave the bar unfilled, so the tabs sit on whatever is behind them.
    pub fn transparent(mut self) -> Self {
        self.container = Container::Transparent;
        self
    }

    /// Whether to draw the 1 dp divider under the bar (default true).
    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = divider;
        self
    }
}

/// Per-tab measurements, before placing.
struct Measured {
    label: std::sync::Arc<egui::Galley>,
    /// The content's width: label or icon, whichever is wider.
    content_width: f32,
}

impl<'a> Widget for MaterialTabs<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let has_icons = self.tabs.iter().any(|t| t.icon.is_some());
        let has_labels = self.tabs.iter().any(|t| !t.label.is_empty());
        let tab_height = self.height.unwrap_or(if has_icons && has_labels {
            TAB_HEIGHT_WITH_ICON_AND_LABEL
        } else {
            TAB_HEIGHT
        });

        let measured: Vec<Measured> = self
            .tabs
            .iter()
            .map(|tab| {
                let label = text_galley(
                    ui.painter(),
                    TypeRole::TitleSmall,
                    &tab.label,
                    Color32::PLACEHOLDER,
                );
                let icon_width = if tab.icon.is_some() {
                    TAB_ICON_SIZE
                } else {
                    0.0
                };
                let content_width = label.size().x.max(icon_width);
                Measured {
                    label,
                    content_width,
                }
            })
            .collect();

        let available = ui.available_width();
        let count = self.tabs.len();
        let (widths, edge): (Vec<f32>, f32) = match self.layout {
            TabLayout::Fixed => (vec![available / count.max(1) as f32; count], 0.0),
            TabLayout::Content => (
                measured
                    .iter()
                    .map(|m| m.content_width + TAB_HORIZONTAL_PADDING * 2.0)
                    .collect(),
                0.0,
            ),
            TabLayout::Scrollable => (
                measured
                    .iter()
                    .map(|m| {
                        (m.content_width + TAB_HORIZONTAL_PADDING * 2.0).max(self.min_tab_width)
                    })
                    .collect(),
                self.edge_padding,
            ),
        };
        match self.layout {
            TabLayout::Fixed | TabLayout::Content => {
                self.show_row(ui, &theme, &measured, &widths, edge, tab_height)
            }
            TabLayout::Scrollable => {
                let salt = match &self.id_salt {
                    Some(salt) => Id::unique((salt, "tabs_scroll")),
                    None => ui.next_auto_id().with("tabs_scroll"),
                };
                egui::ScrollArea::horizontal()
                    .id_salt(salt)
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        self.show_row(ui, &theme, &measured, &widths, edge, tab_height)
                    })
                    .inner
            }
        }
    }
}

impl MaterialTabs<'_> {
    fn show_row(
        self,
        ui: &mut Ui,
        theme: &MaterialTheme,
        measured: &[Measured],
        widths: &[f32],
        edge: f32,
        tab_height: f32,
    ) -> Response {
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let tabs_width: f32 = widths.iter().sum();
        let row_width = (tabs_width + edge * 2.0).max(ui.available_width());
        let (rect, mut response) =
            ui.allocate_exact_size(Vec2::new(row_width, tab_height), Sense::hover());
        let painter = ui.painter_at(rect);
        let id_base = response.id;

        let fill = match self.container {
            Container::Theme => Some(theme.color(ColorRole::Surface)),
            Container::Color(color) => Some(color),
            Container::Transparent => None,
        };
        if let Some(fill) = fill {
            painter.rect_filled(rect, 0.0, fill);
        }

        let primary = theme.color(ColorRole::Primary);
        let on_surface = theme.color(ColorRole::OnSurface);
        let on_surface_variant = theme.color(ColorRole::OnSurfaceVariant);
        // The selected content colour, which the state layer uses too.
        let selected_color = match self.variant {
            TabVariant::Primary => primary,
            TabVariant::Secondary => on_surface,
        };

        let mut any_clicked = false;
        let mut ids = Vec::with_capacity(self.tabs.len());
        let mut indicator_target = None;
        let mut tab_rects = Vec::with_capacity(self.tabs.len());
        let mut offset = edge;
        for (index, (tab, m)) in self.tabs.iter().zip(measured).enumerate() {
            let width = widths.get(index).copied().unwrap_or(0.0);
            let x = if rtl {
                rect.max.x - offset - width
            } else {
                rect.min.x + offset
            };
            offset += width;
            let tab_rect =
                Rect::from_min_size(Pos2::new(x, rect.min.y), Vec2::new(width, tab_height));

            // Create unique ID for each tab using optional salt
            let tab_id = if let Some(ref salt) = self.id_salt {
                egui::Id::unique((salt, "tab", index))
            } else {
                id_base.with(("tab", index))
            };
            let enabled = self.enabled && tab.enabled;
            let tab_response = ui.interact(
                tab_rect,
                tab_id,
                crate::a11y::sense(enabled, Sense::click()),
            );
            if tab_response.clicked() && *self.selected != index {
                *self.selected = index;
                any_clicked = true;
            }
            if self.layout == TabLayout::Scrollable
                && (tab_response.clicked() || tab_response.gained_focus())
            {
                ui.scroll_to_rect(tab_rect, None);
            }
            let is_selected = *self.selected == index;
            describe_item(&tab_response, enabled, &tab.label, is_selected);
            ids.push((tab_id, enabled));
            tab_rects.push(tab_rect);

            if let Some((background, _)) = tab.color {
                painter.rect_filled(tab_rect, 0.0, background);
            }
            let state = InteractionState::from_response(&tab_response);
            let active = state.hovered || state.focused || state.pressed;
            let content_color = match (enabled, tab.color, is_selected, active) {
                (false, Some((_, text)), _, _) => text.gamma_multiply(DISABLED_CONTENT_OPACITY),
                (false, None, _, _) => on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY),
                (true, Some((_, text)), _, _) => text,
                (true, None, true, _) => selected_color,
                (true, None, false, true) => on_surface,
                (true, None, false, false) => on_surface_variant,
            };
            crate::paint::paint_state_layer(&painter, tab_rect, 0, selected_color, state);

            // Icon above label, the pair centred.
            let label_size = m.label.size();
            let icon_height = if tab.icon.is_some() {
                TAB_ICON_SIZE
            } else {
                0.0
            };
            let gap = if tab.icon.is_some() && !tab.label.is_empty() {
                ICON_LABEL_SPACE
            } else {
                0.0
            };
            let label_height = if tab.label.is_empty() {
                0.0
            } else {
                label_size.y
            };
            let top = tab_rect.center().y - (icon_height + gap + label_height) / 2.0;
            let mut badge_anchor = None;
            if let Some(icon) = &tab.icon {
                let icon_rect = paint_icon(
                    &painter,
                    Pos2::new(tab_rect.center().x, top + TAB_ICON_SIZE / 2.0),
                    icon,
                    TAB_ICON_SIZE,
                    content_color,
                    is_selected,
                );
                badge_anchor = Some(icon_rect);
            }
            if !tab.label.is_empty() {
                let label_rect = Rect::from_min_size(
                    Pos2::new(
                        tab_rect.center().x - label_size.x / 2.0,
                        top + icon_height + gap,
                    ),
                    label_size,
                );
                painter.galley(label_rect.min, m.label.clone(), content_color);
                badge_anchor = badge_anchor.or(Some(label_rect));
            }
            if let (Some(badge), Some(anchor)) = (&tab.badge, badge_anchor) {
                paint_badge(&painter, theme, anchor, badge, !rtl);
            }
            if tab_response.has_focus() {
                let inset = crate::paint::FOCUS_RING_OFFSET + crate::paint::FOCUS_RING_WIDTH;
                crate::paint::paint_focus_ring(
                    &painter,
                    tab_rect.shrink(inset),
                    CornerRadius::same(8),
                    theme.color(ColorRole::Secondary),
                );
            }

            if is_selected {
                indicator_target = Some(match self.variant {
                    TabVariant::Primary => {
                        let w = m.content_width.max(PRIMARY_INDICATOR_MIN_WIDTH).min(width);
                        (tab_rect.center().x - w / 2.0, w)
                    }
                    TabVariant::Secondary => (tab_rect.min.x, width),
                });
            }
        }

        // The divider, then the indicator on top of it.
        if self.divider {
            let divider_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.max.y - DIVIDER_HEIGHT),
                Vec2::new(rect.width(), DIVIDER_HEIGHT),
            );
            painter.rect_filled(divider_rect, 0.0, theme.color(ColorRole::OutlineVariant));
        }
        if let Some((target_x, target_width)) = indicator_target {
            // Animate relative to the row, so scrolling does not animate.
            let spring = theme.motion.spatial(SpringSpeed::Default);
            let x = rect.min.x
                + animate_value_with_spring(
                    ui.ctx(),
                    id_base.with("indicator_x"),
                    target_x - rect.min.x,
                    spring,
                );
            let width = animate_value_with_spring(
                ui.ctx(),
                id_base.with("indicator_width"),
                target_width,
                spring,
            )
            .max(0.0);
            let (height, rounding) = match self.variant {
                TabVariant::Primary => (
                    PRIMARY_INDICATOR_HEIGHT,
                    CornerRadius {
                        nw: INDICATOR_TOP_ROUNDING,
                        ne: INDICATOR_TOP_ROUNDING,
                        sw: 0,
                        se: 0,
                    },
                ),
                TabVariant::Secondary => (SECONDARY_INDICATOR_HEIGHT, CornerRadius::ZERO),
            };
            let color = if self.enabled {
                primary
            } else {
                on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY)
            };
            let indicator_rect =
                Rect::from_min_size(Pos2::new(x, rect.max.y - height), Vec2::new(width, height));
            painter.rect_filled(indicator_rect, rounding, color);
        }

        let roved = rove_focus(ui, &ids, RoveAxis::Horizontal, rtl);
        if let Some(target) = roved.and_then(|index| tab_rects.get(index)) {
            if self.layout == TabLayout::Scrollable {
                ui.scroll_to_rect(*target, None);
            }
        }
        if any_clicked {
            response.mark_changed();
        }
        response
    }
}

/// Convenience function to create primary tabs.
///
/// Shorthand for `MaterialTabs::primary()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected tab index
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut tab_index = 0;
/// ui.add(tabs_primary(&mut tab_index)
///     .tab("Tab 1")
///     .tab("Tab 2"));
/// # });
/// ```
pub fn tabs_primary<'a>(selected: &'a mut usize) -> MaterialTabs<'a> {
    MaterialTabs::primary(selected)
}

/// Convenience function to create secondary tabs.
///
/// Shorthand for `MaterialTabs::secondary()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected tab index
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut tab_index = 0;
/// ui.add(tabs_secondary(&mut tab_index)
///     .tab("Tab 1")
///     .tab("Tab 2"));
/// # });
/// ```
pub fn tabs_secondary<'a>(selected: &'a mut usize) -> MaterialTabs<'a> {
    MaterialTabs::secondary(selected)
}

#[cfg(test)]
mod id_tests {
    use super::*;

    /// Regression test for nikescar/egui-material3#16: two unsalted tab bars
    /// do not share tab ids, so clicking a tab in one leaves the other alone.
    #[test]
    fn test_unsalted_tab_bars_are_independent() {
        let ctx = egui::Context::default();
        let mut first = 0;
        let mut second = 0;
        let frame = |events: Vec<egui::Event>, first: &mut usize, second: &mut usize| {
            let mut rect = Rect::NOTHING;
            let input = egui::RawInput {
                events,
                ..Default::default()
            };
            ctx.run_ui(input, |ui| {
                rect = ui
                    .add(MaterialTabs::primary(first).tab("One").tab("Two"))
                    .rect;
                ui.add(MaterialTabs::primary(second).tab("One").tab("Two"));
            })
            .textures_delta
            .clear();
            rect
        };
        let rect = frame(Vec::new(), &mut first, &mut second);
        // The second tab of the first bar.
        let pos = Pos2::new(rect.min.x + rect.width() * 0.75, rect.center().y);
        let button = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        frame(
            vec![egui::Event::PointerMoved(pos), button(true)],
            &mut first,
            &mut second,
        );
        frame(vec![button(false)], &mut first, &mut second);
        assert_eq!(
            first, 1,
            "clicking the first bar's second tab should select it"
        );
        assert_eq!(second, 0, "the second tab bar must not change");
    }
}
