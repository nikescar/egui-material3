// ============================================================================
// Module:       search
// Description:  Material 3 search bar and search view (docked, full screen).
//
// Dependencies: egui; crate::{a11y, icon, material_theme, paint, tokens}
// ============================================================================

//! The Material 3 search bar and the search view it expands into.
//!
//! A [`MaterialSearch`] draws a 56 dp, fully rounded `surfaceContainerHigh`
//! bar with a leading icon, a placeholder and optional trailing actions.
//! When its text field takes focus (a click or Tab) the bar expands into a
//! search view: [`SearchViewMode::Docked`] opens a rounded panel over the
//! content below the bar, [`SearchViewMode::FullScreen`] covers the whole
//! window. The view shows a back button, the query, a clear button and the
//! caller's suggestions. Escape, the back button, a click outside a docked
//! view, Enter, or picking a suggestion collapse it again.
//!
//! The caller owns the query string and filters the suggestions; the widget
//! reports what happened in a [`SearchOutput`].
//!
//! Keyboard: Up and Down move a highlight through the suggestions while the
//! text field keeps focus, Enter picks the highlighted suggestion (or
//! submits the query when none is highlighted), and Tab reaches each
//! suggestion row, which Enter or Space picks.
//!
//! Values come from Compose's baseline (not the Expressive "contained")
//! tokens: `SearchBarTokens.kt`, `SearchViewTokens.kt` and `ListTokens.kt`,
//! with the layout constants of `SearchBar.kt` (`SearchBarMaxWidth`,
//! `DockedExpandedTableMinHeight`, `DockedExpandedTableMaxHeightScreenRatio`,
//! `SearchBarIconOffsetX`).
//!
//! ```no_run
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::search::{MaterialSearch, SearchViewMode};
//! let mut query = String::new();
//! let all = ["Apple", "Banana", "Cherry"];
//! let suggestions: Vec<&str> = all
//!     .iter()
//!     .copied()
//!     .filter(|s| s.to_lowercase().contains(&query.to_lowercase()))
//!     .collect();
//! let out = MaterialSearch::new("fruit_search", &mut query)
//!     .placeholder("Search fruit")
//!     .mode(SearchViewMode::Docked)
//!     .suggestions(suggestions)
//!     .show(ui);
//! if let Some(index) = out.selected {
//!     println!("picked suggestion {index}");
//! }
//! # });
//! ```

use std::fmt::Debug;
use std::hash::Hash;

use egui::accesskit::Role;
use egui::text::{LayoutJob, TextWrapping};
use egui::{
    Align, Align2, Area, AreaState, Color32, CornerRadius, Id, Key, LayerId, Layout, Modifiers,
    Order, Rect, Response, RichText, ScrollArea, Sense, Stroke, TextEdit, Ui, UiBuilder, Vec2,
};

use crate::icon::{paint_icon, Icon, SymbolOptions};
use crate::material_theme::MaterialTheme;
use crate::paint::{paint_focus_ring, paint_state_layer};
use crate::tokens::state::FOCUS_OPACITY;
use crate::tokens::{ColorRole, CornerSize, InteractionState, LayoutDirection, TypeRole};

/// `SearchBarTokens.ContainerHeight` (= `SearchBarDefaults.InputFieldHeight`):
/// the bar, and `SearchViewTokens.DockedHeaderContainerHeight`.
pub const BAR_HEIGHT: f32 = 56.0;
/// `SearchViewTokens.FullScreenHeaderContainerHeight`.
pub const FULL_SCREEN_HEADER_HEIGHT: f32 = 72.0;
/// `SearchBar.kt` `SearchBarMaxWidth`: the widest a bar grows by default.
pub const BAR_MAX_WIDTH: f32 = 720.0;
/// `SearchBarTokens.IconSize`.
const ICON_SIZE: f32 = 24.0;
/// The touch target around each icon: Compose places the leading and
/// trailing slots in 48 dp icon buttons.
const ICON_TARGET: f32 = 48.0;
/// The state-layer circle of an icon button (`IconButtonTokens` 40 dp).
const ICON_STATE_LAYER: f32 = 40.0;
/// `SearchBarTokens.LeadingSpace` / `TrailingSpace` (`MeasurementTokens.Space200`),
/// the same in `SearchViewTokens`.
const EDGE_SPACE: f32 = 16.0;
/// `SearchBarTokens.LeadingLeadingIconIconLabelSpace` and
/// `LabelTrailingTrailingIconIconSpace` (`Space200`).
const ICON_LABEL_SPACE: f32 = 16.0;
/// `SearchBar.kt` `DockedExpandedTableMinHeight`.
const DOCKED_RESULTS_MIN_HEIGHT: f32 = 240.0;
/// `SearchBar.kt` `DockedExpandedTableMaxHeightScreenRatio` (2/3).
const DOCKED_RESULTS_MAX_SCREEN_RATIO: f32 = 2.0 / 3.0;
/// `ListTokens.ItemOneLineContainerHeight`.
const ROW_ONE_LINE_HEIGHT: f32 = 56.0;
/// `ListTokens.ItemTwoLineContainerHeight`.
const ROW_TWO_LINE_HEIGHT: f32 = 72.0;
/// `ListTokens.ItemLeadingSpace` / `ItemTrailingSpace` (`Space200`).
const ROW_EDGE_SPACE: f32 = 16.0;
/// `ListTokens.ItemBetweenSpace` (`Space150`): leading icon to text.
const ROW_BETWEEN_SPACE: f32 = 12.0;
/// The divider between the view's header and its results (1 dp, `HorizontalDivider`).
const DIVIDER_THICKNESS: f32 = 1.0;

/// How the search view opens when the bar expands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SearchViewMode {
    /// A panel the bar's width, opening over the content below the bar
    /// (`DockedSearchBar`; `SearchViewTokens.DockedContainerShape`,
    /// `CornerExtraLarge`).
    #[default]
    Docked,
    /// A view covering the whole window (`SearchBar` expanded;
    /// `SearchViewTokens.FullScreenContainerShape`, `CornerNone`).
    FullScreen,
}

/// One row in the search view's suggestion list.
#[derive(Clone, Debug, PartialEq)]
pub struct Suggestion {
    /// The text; picking the suggestion puts it in the query.
    pub text: String,
    /// An optional second line (`ListTokens.ItemSupportingText*`).
    pub supporting_text: Option<String>,
    /// An optional leading icon, such as `"history"`.
    pub icon: Option<Icon>,
}

impl Suggestion {
    /// A one-line suggestion.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            supporting_text: None,
            icon: None,
        }
    }

    /// Adds a second line.
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Adds a leading icon.
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl From<&str> for Suggestion {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for Suggestion {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&String> for Suggestion {
    fn from(text: &String) -> Self {
        Self::new(text.clone())
    }
}

/// What happened to a [`MaterialSearch`] this frame.
#[derive(Debug)]
pub struct SearchOutput {
    /// The bar's response (its full rect, collapsed or not).
    pub response: Response,
    /// Whether the search view is open after this frame.
    pub expanded: bool,
    /// The query was edited this frame.
    pub changed: bool,
    /// Enter was pressed in the text field with no suggestion highlighted:
    /// the query to search for.
    pub submitted: Option<String>,
    /// The index of the suggestion picked this frame (by click, Enter on
    /// the highlight, or Enter/Space on a focused row). The query already
    /// holds its text.
    pub selected: Option<usize>,
    /// The index of the trailing action clicked this frame.
    pub trailing_clicked: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SearchState {
    expanded: bool,
    highlighted: Option<usize>,
}

fn load_state(ctx: &egui::Context, id: Id) -> SearchState {
    ctx.data(|d| d.get_temp::<SearchState>(id))
        .unwrap_or_default()
}

fn store_state(ctx: &egui::Context, id: Id, state: SearchState) {
    ctx.data_mut(|d| d.insert_temp(id, state));
}

/// The root id of the search created with `id_salt`: global, so the
/// search keeps its state wherever it is laid out.
fn root_id(id_salt: impl Hash + Debug) -> Id {
    Id::unique(("egui_material3::search", id_salt))
}

/// The id of the search's text field, for the search created with `id_salt`.
pub fn text_field_id(id_salt: impl Hash + Debug) -> Id {
    root_id(id_salt).with("input")
}

/// Whether the search created with `id_salt` is expanded.
pub fn is_expanded(ctx: &egui::Context, id_salt: impl Hash + Debug) -> bool {
    load_state(ctx, root_id(id_salt)).expanded
}

/// Expands or collapses the search created with `id_salt`. Expanding moves
/// keyboard focus into its text field; collapsing takes it away.
pub fn set_expanded(ctx: &egui::Context, id_salt: impl Hash + Debug, expanded: bool) {
    let id = root_id(id_salt);
    let text_id = id.with("input");
    store_state(
        ctx,
        id,
        SearchState {
            expanded,
            highlighted: None,
        },
    );
    ctx.memory_mut(|m| {
        if expanded {
            m.request_focus(text_id);
        } else {
            m.surrender_focus(text_id);
        }
    });
    ctx.request_repaint();
}

/// A trailing action in the bar: an icon and its accessible label.
#[derive(Clone, Debug, PartialEq)]
struct TrailingAction {
    icon: Icon,
    label: String,
}

/// A Material 3 search bar that expands into a search view.
#[must_use = "call `.show(ui)` to draw the search bar"]
pub struct MaterialSearch<'a> {
    id: Id,
    query: &'a mut String,
    placeholder: String,
    mode: SearchViewMode,
    suggestions: Vec<Suggestion>,
    leading_icon: Icon,
    trailing: Vec<TrailingAction>,
    clear_button: bool,
    width: Option<f32>,
}

impl<'a> MaterialSearch<'a> {
    /// A search bar editing `query`. `id_salt` names it: keep it stable
    /// and unique among the searches on screen.
    pub fn new(id_salt: impl Hash + Debug, query: &'a mut String) -> Self {
        Self {
            id: root_id(id_salt),
            query,
            placeholder: "Search".to_owned(),
            mode: SearchViewMode::default(),
            suggestions: Vec::new(),
            leading_icon: Icon::from("search"),
            trailing: Vec::new(),
            clear_button: true,
            width: None,
        }
    }

    /// The placeholder shown while the query is empty; also the field's
    /// accessible name.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Docked or full-screen view.
    pub fn mode(mut self, mode: SearchViewMode) -> Self {
        self.mode = mode;
        self
    }

    /// The suggestions shown in the view, already filtered for the query.
    pub fn suggestions<S: Into<Suggestion>>(
        mut self,
        suggestions: impl IntoIterator<Item = S>,
    ) -> Self {
        self.suggestions = suggestions.into_iter().map(Into::into).collect();
        self
    }

    /// The icon at the start of the collapsed bar (default `"search"`).
    /// The expanded view always shows a back button there.
    pub fn leading_icon(mut self, icon: impl Into<Icon>) -> Self {
        self.leading_icon = icon.into();
        self
    }

    /// Adds a trailing action (an icon button labelled `label`), such as a
    /// microphone or an avatar. Clicks come back in
    /// [`SearchOutput::trailing_clicked`].
    pub fn trailing_icon(mut self, icon: impl Into<Icon>, label: impl Into<String>) -> Self {
        self.trailing.push(TrailingAction {
            icon: icon.into(),
            label: label.into(),
        });
        self
    }

    /// Whether the expanded view shows a clear button while the query is
    /// not empty (default true).
    pub fn clear_button(mut self, clear_button: bool) -> Self {
        self.clear_button = clear_button;
        self
    }

    /// A fixed width. By default the bar fills the available width, up to
    /// [`BAR_MAX_WIDTH`].
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Draws the bar, and the view when expanded.
    pub fn show(self, ui: &mut Ui) -> SearchOutput {
        let Self {
            id,
            query,
            placeholder,
            mode,
            suggestions,
            leading_icon,
            trailing,
            clear_button,
            width,
        } = self;
        let ctx = ui.ctx().clone();
        let theme = MaterialTheme::of(&ctx);
        let text_id = id.with("input");
        let mut state = load_state(&ctx, id);
        if state.highlighted.is_some_and(|i| i >= suggestions.len()) {
            state.highlighted = None;
        }

        let width = width
            .unwrap_or_else(|| ui.available_width().min(BAR_MAX_WIDTH))
            .max(0.0);
        // Not focusable: Tab goes straight to the text field.
        let (bar_rect, bar_response) =
            ui.allocate_exact_size(Vec2::new(width, BAR_HEIGHT), Sense::CLICK);
        crate::a11y::describe(&bar_response, Role::Search, true, &placeholder, None);

        let mut out = SearchOutput {
            response: bar_response.clone(),
            expanded: false,
            changed: false,
            submitted: None,
            selected: None,
            trailing_clicked: None,
        };

        let text_focused = ctx.memory(|m| m.has_focus(text_id));
        let mut keyboard_moved = false;
        if state.expanded && text_focused {
            let (down, up) = ui.input_mut(|i| {
                (
                    i.consume_key(Modifiers::NONE, Key::ArrowDown),
                    i.consume_key(Modifiers::NONE, Key::ArrowUp),
                )
            });
            if down && !suggestions.is_empty() {
                let last = suggestions.len().saturating_sub(1);
                state.highlighted = Some(state.highlighted.map_or(0, |i| (i + 1).min(last)));
                keyboard_moved = true;
            }
            if up {
                state.highlighted = state.highlighted.and_then(|i| i.checked_sub(1));
                keyboard_moved = true;
            }
        }

        let field = Field {
            id,
            text_id,
            placeholder: &placeholder,
            theme: &theme,
        };

        let mut collapse = false;
        if state.expanded {
            let view = View {
                field,
                mode,
                suggestions: &suggestions,
                trailing: &trailing,
                clear_button,
                bar_rect,
                highlighted: state.highlighted,
                scroll_to_highlight: keyboard_moved,
            };
            let result = view.show(&ctx, query);
            out.changed = result.changed;
            out.trailing_clicked = result.trailing_clicked;
            if result.changed {
                state.highlighted = None;
            }
            if result.back_clicked {
                collapse = true;
            }
            if let Some(i) = result.selected {
                select(query, &suggestions, i, &mut out);
                collapse = true;
            }
            if result.enter {
                match state.highlighted {
                    Some(i) => select(query, &suggestions, i, &mut out),
                    None => out.submitted = Some(query.clone()),
                }
                collapse = true;
            }
            if ui.input(|i| i.key_pressed(Key::Escape)) {
                collapse = true;
            }
            let pressed_outside = ui.input(|i| {
                i.pointer.any_pressed()
                    && i.pointer
                        .press_origin()
                        .is_some_and(|p| !result.rect.contains(p) && !bar_rect.contains(p))
            });
            if pressed_outside {
                collapse = true;
            }
            let focus_elsewhere = ctx
                .memory(|m| m.focused())
                .is_some_and(|f| f != text_id && !result.ids.contains(&f));
            if focus_elsewhere {
                collapse = true;
            }
        } else {
            let result = field.collapsed_bar(ui, bar_rect, query, &leading_icon, &trailing);
            out.changed = result.changed;
            out.trailing_clicked = result.trailing_clicked;
            if bar_response.clicked() {
                ctx.memory_mut(|m| m.request_focus(text_id));
            }
            if ctx.memory(|m| m.has_focus(text_id)) {
                state.expanded = true;
                state.highlighted = None;
                ctx.request_repaint();
            }
        }

        if collapse {
            state = SearchState::default();
            ctx.memory_mut(|m| m.surrender_focus(text_id));
            ctx.request_repaint();
        }
        out.expanded = state.expanded;
        store_state(&ctx, id, state);
        out
    }
}

fn select(query: &mut String, suggestions: &[Suggestion], index: usize, out: &mut SearchOutput) {
    if let Some(suggestion) = suggestions.get(index) {
        query.clone_from(&suggestion.text);
        out.selected = Some(index);
        out.changed = true;
    }
}

/// The x span `offset..offset + width` measured from the start edge of
/// `outer` (left in LTR, right in RTL), full height.
fn from_start(outer: Rect, offset: f32, width: f32, dir: LayoutDirection) -> Rect {
    let (min_x, max_x) = match dir {
        LayoutDirection::Ltr => (outer.left() + offset, outer.left() + offset + width),
        LayoutDirection::Rtl => (outer.right() - offset - width, outer.right() - offset),
    };
    Rect::from_x_y_ranges(min_x..=max_x.max(min_x), outer.y_range())
}

/// The same, measured from the end edge.
fn from_end(outer: Rect, offset: f32, width: f32, dir: LayoutDirection) -> Rect {
    let flipped = match dir {
        LayoutDirection::Ltr => LayoutDirection::Rtl,
        LayoutDirection::Rtl => LayoutDirection::Ltr,
    };
    from_start(outer, offset, width, flipped)
}

/// The 48 dp target whose 24 dp icon starts `offset` from `outer`'s start
/// (or end, with `end`), vertically centred.
fn icon_target(outer: Rect, offset: f32, end: bool, dir: LayoutDirection) -> Rect {
    let icon = if end {
        from_end(outer, offset, ICON_SIZE, dir)
    } else {
        from_start(outer, offset, ICON_SIZE, dir)
    };
    Rect::from_center_size(icon.center(), Vec2::splat(ICON_TARGET))
}

/// The start offset of the `index`th trailing icon, counting from the end:
/// adjacent icon buttons are one target apart.
fn trailing_offset(index: usize) -> f32 {
    EDGE_SPACE + ICON_TARGET * index as f32
}

/// The space the text gets between the start content and the trailing icons.
fn text_rect(outer: Rect, trailing_icons: usize, dir: LayoutDirection) -> Rect {
    let start = EDGE_SPACE + ICON_SIZE + ICON_LABEL_SPACE;
    let end = match trailing_icons {
        0 => EDGE_SPACE,
        n => trailing_offset(n.saturating_sub(1)) + ICON_SIZE + ICON_LABEL_SPACE,
    };
    let width = (outer.width() - start - end).max(0.0);
    from_start(outer, start, width, dir)
}

/// An icon button: a 48 dp target with a 40 dp state-layer circle.
fn icon_button(
    ui: &mut Ui,
    target: Rect,
    id: Id,
    icon: &Icon,
    label: &str,
    tint: Color32,
) -> Response {
    let response = ui.interact(target, id, Sense::click());
    crate::a11y::button(&response, true, label);
    let layer = Rect::from_center_size(target.center(), Vec2::splat(ICON_STATE_LAYER));
    let radius = CornerRadius::same((ICON_STATE_LAYER / 2.0) as u8);
    let painter = ui.painter();
    paint_state_layer(
        painter,
        layer,
        radius,
        tint,
        InteractionState::from_response(&response),
    );
    let icon_rect = Rect::from_center_size(target.center(), Vec2::splat(ICON_SIZE));
    paint_icon(painter, icon_rect, icon, &SymbolOptions::default(), tint);
    if response.has_focus() {
        // `IconButtonTokens.FocusIndicatorColor`: `secondary`.
        let ring = MaterialTheme::of(ui.ctx()).color(ColorRole::Secondary);
        paint_focus_ring(painter, layer, radius, ring);
    }
    response
}

struct Field<'t> {
    id: Id,
    text_id: Id,
    placeholder: &'t str,
    theme: &'t MaterialTheme,
}

struct BarResult {
    changed: bool,
    trailing_clicked: Option<usize>,
}

impl Field<'_> {
    /// The text field in `rect`. Returns its response.
    fn text_edit(&self, ui: &mut Ui, rect: Rect, query: &mut String) -> Response {
        let ctx = ui.ctx().clone();
        // `SearchBarTokens.InputTextFont` / `SupportingTextFont`: Body Large.
        let font = TypeRole::BodyLarge.style().font_id(&ctx);
        let hint = RichText::new(self.placeholder)
            .font(font.clone())
            // `SearchBarTokens.SupportingTextColor`.
            .color(self.theme.color(ColorRole::OnSurfaceVariant));
        let edit = TextEdit::singleline(query)
            .id(self.text_id)
            .font(font)
            // `SearchBarTokens.InputTextColor`.
            .text_color(self.theme.color(ColorRole::OnSurface))
            .hint_text(hint)
            .frame(egui::Frame::NONE)
            .margin(egui::Margin::ZERO)
            .desired_width(rect.width());
        let response = ui
            .scope_builder(
                UiBuilder::new()
                    .max_rect(rect)
                    .layout(Layout::left_to_right(Align::Center)),
                |ui| ui.add(edit),
            )
            .inner;
        let response = response.accessible_name(self.placeholder);
        ctx.accesskit_node_builder(response.id, |node| node.set_role(Role::SearchInput));
        response
    }

    /// The collapsed bar: container, leading icon, field, trailing actions.
    fn collapsed_bar(
        &self,
        ui: &mut Ui,
        rect: Rect,
        query: &mut String,
        leading_icon: &Icon,
        trailing: &[TrailingAction],
    ) -> BarResult {
        let theme = self.theme;
        let dir = theme.layout_direction;
        // `SearchBarTokens.ContainerShape`: `CornerFull`.
        let radius = CornerRadius::same(CornerSize::Full.radius(rect).round() as u8);
        let hovered = ui.rect_contains_pointer(rect);
        {
            let painter = ui.painter();
            // `SearchBarTokens.ContainerColor`.
            painter.rect_filled(rect, radius, theme.color(ColorRole::SurfaceContainerHigh));
            paint_state_layer(
                painter,
                rect,
                radius,
                theme.color(ColorRole::OnSurface),
                InteractionState {
                    hovered,
                    ..Default::default()
                },
            );
            let leading = icon_target(rect, EDGE_SPACE, false, dir);
            paint_icon(
                painter,
                Rect::from_center_size(leading.center(), Vec2::splat(ICON_SIZE)),
                leading_icon,
                &SymbolOptions::default(),
                // `SearchBarTokens.LeadingIconColor`.
                theme.color(ColorRole::OnSurface),
            );
        }
        let changed = self
            .text_edit(ui, text_rect(rect, trailing.len(), dir), query)
            .changed();
        let trailing_clicked = self.trailing_actions(ui, rect, trailing, 0);
        BarResult {
            changed,
            trailing_clicked,
        }
    }

    /// Paints the trailing actions, the first at `first_slot` from the end.
    fn trailing_actions(
        &self,
        ui: &mut Ui,
        rect: Rect,
        trailing: &[TrailingAction],
        first_slot: usize,
    ) -> Option<usize> {
        let dir = self.theme.layout_direction;
        // Laid out from the end: the last action sits nearest the edge.
        let count = trailing.len();
        let mut clicked = None;
        for (i, action) in trailing.iter().enumerate() {
            let slot = first_slot + count.saturating_sub(1).saturating_sub(i);
            let target = icon_target(rect, trailing_offset(slot), true, dir);
            let response = icon_button(
                ui,
                target,
                self.id.with(("trailing", i)),
                &action.icon,
                &action.label,
                // `SearchBarTokens.TrailingIconColor`.
                self.theme.color(ColorRole::OnSurfaceVariant),
            );
            if response.clicked() {
                clicked = Some(i);
            }
        }
        clicked
    }
}

struct View<'t> {
    field: Field<'t>,
    mode: SearchViewMode,
    suggestions: &'t [Suggestion],
    trailing: &'t [TrailingAction],
    clear_button: bool,
    bar_rect: Rect,
    highlighted: Option<usize>,
    scroll_to_highlight: bool,
}

struct ViewResult {
    rect: Rect,
    ids: Vec<Id>,
    changed: bool,
    enter: bool,
    back_clicked: bool,
    selected: Option<usize>,
    trailing_clicked: Option<usize>,
}

impl View<'_> {
    fn show(&self, ctx: &egui::Context, query: &mut String) -> ViewResult {
        let screen = ctx.content_rect();
        let (origin, width, header_height, radius) = match self.mode {
            SearchViewMode::Docked => (
                self.bar_rect.min,
                self.bar_rect.width(),
                BAR_HEIGHT,
                // `SearchViewTokens.DockedContainerShape`: `CornerExtraLarge`.
                CornerSize::ExtraLarge.value().unwrap_or(28.0),
            ),
            SearchViewMode::FullScreen => (
                screen.min,
                screen.width(),
                FULL_SCREEN_HEADER_HEIGHT,
                // `SearchViewTokens.FullScreenContainerShape`: `CornerNone`.
                0.0,
            ),
        };
        let rows_height: f32 = self.suggestions.iter().map(row_height).sum();
        let results_height = match self.mode {
            SearchViewMode::Docked => {
                let max = (screen.height() * DOCKED_RESULTS_MAX_SCREEN_RATIO
                    - header_height
                    - DIVIDER_THICKNESS)
                    .max(DOCKED_RESULTS_MIN_HEIGHT);
                rows_height.clamp(DOCKED_RESULTS_MIN_HEIGHT, max)
            }
            SearchViewMode::FullScreen => {
                (screen.bottom() - origin.y - header_height - DIVIDER_THICKNESS).max(0.0)
            }
        };
        let total = Vec2::new(width, header_height + DIVIDER_THICKNESS + results_height);

        let area_id = self.field.id.with("view");
        // An area seen for the first time does an invisible sizing pass,
        // which would drop the text field's focus for a frame. The size is
        // known, so give the area its state up front.
        if AreaState::load(ctx, area_id).is_none() {
            let state = AreaState {
                pivot_pos: Some(origin),
                pivot: Align2::LEFT_TOP,
                size: Some(total),
                interactable: true,
                last_became_visible_at: None,
            };
            let layer = LayerId::new(Order::Foreground, area_id);
            ctx.memory_mut(|m| m.areas_mut().set_state(layer, state));
        }
        let area = Area::new(area_id)
            .order(Order::Foreground)
            .fixed_pos(origin)
            .show(ctx, |ui| {
                self.contents(ui, query, total, header_height, radius)
            });
        let mut result = area.inner;
        result.rect = area.response.rect;
        result
    }

    fn contents(
        &self,
        ui: &mut Ui,
        query: &mut String,
        total: Vec2,
        header_height: f32,
        radius: f32,
    ) -> ViewResult {
        let theme = self.field.theme;
        let dir = theme.layout_direction;
        // Takes the whole view so clicks do not reach what is under it.
        let (rect, _) = ui.allocate_exact_size(total, Sense::click());
        let corner = CornerRadius::same(radius.round().clamp(0.0, 255.0) as u8);
        // `SearchViewTokens.ContainerColor`.
        ui.painter()
            .rect_filled(rect, corner, theme.color(ColorRole::SurfaceContainerHigh));

        let header = Rect::from_min_size(rect.min, Vec2::new(rect.width(), header_height));
        let mut ids = Vec::new();

        // `SearchViewTokens.HeaderLeadingIconColor`.
        let back_id = self.field.id.with("back");
        let back = icon_button(
            ui,
            icon_target(header, EDGE_SPACE, false, dir),
            back_id,
            &Icon::from("arrow_back"),
            "Back",
            theme.color(ColorRole::OnSurface),
        );
        ids.push(back_id);

        let show_clear = self.clear_button && !query.is_empty();
        let trailing_icons = self.trailing.len() + usize::from(show_clear);
        let edit = self
            .field
            .text_edit(ui, text_rect(header, trailing_icons, dir), query);
        let enter = edit.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
        let mut changed = edit.changed();

        // The clear button sits nearest the field, the caller's actions after it.
        if show_clear {
            let clear_id = self.field.id.with("clear");
            let slot = self.trailing.len();
            let clear = icon_button(
                ui,
                icon_target(header, trailing_offset(slot), true, dir),
                clear_id,
                &Icon::from("close"),
                "Clear search",
                // `SearchViewTokens.HeaderTrailingIconColor`.
                theme.color(ColorRole::OnSurfaceVariant),
            );
            ids.push(clear_id);
            if clear.clicked() {
                query.clear();
                changed = true;
                ui.memory_mut(|m| m.request_focus(self.field.text_id));
            }
        }
        let trailing_clicked = self.field.trailing_actions(ui, header, self.trailing, 0);
        ids.extend((0..self.trailing.len()).map(|i| self.field.id.with(("trailing", i))));

        // `SearchViewTokens.DividerColor`.
        let divider_y = header.bottom() + DIVIDER_THICKNESS / 2.0;
        ui.painter().hline(
            rect.x_range(),
            divider_y,
            Stroke::new(DIVIDER_THICKNESS, theme.color(ColorRole::Outline)),
        );

        let results = Rect::from_min_max(
            egui::pos2(rect.left(), header.bottom() + DIVIDER_THICKNESS),
            rect.max,
        );
        let selected = self.results(ui, results, corner, &mut ids);

        ViewResult {
            rect,
            ids,
            changed,
            enter,
            back_clicked: back.clicked(),
            selected,
            trailing_clicked,
        }
    }

    fn results(
        &self,
        ui: &mut Ui,
        rect: Rect,
        corner: CornerRadius,
        ids: &mut Vec<Id>,
    ) -> Option<usize> {
        let list = ui.interact(rect, self.field.id.with("results"), Sense::hover());
        crate::a11y::describe(&list, Role::ListBox, true, "Suggestions", None);
        let bottom_corners = CornerRadius {
            nw: 0,
            ne: 0,
            ..corner
        };
        let mut selected = None;
        ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
            ui.set_clip_rect(rect.intersect(ui.clip_rect()));
            ScrollArea::vertical()
                .id_salt(self.field.id.with("results_scroll"))
                .max_height(rect.height())
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    let last = self.suggestions.len().saturating_sub(1);
                    for (i, suggestion) in self.suggestions.iter().enumerate() {
                        let row_corner = if i == last {
                            bottom_corners
                        } else {
                            CornerRadius::ZERO
                        };
                        let row_id = self.field.id.with(("suggestion", i));
                        ids.push(row_id);
                        let response = self.row(ui, row_id, i, suggestion, row_corner);
                        if response.clicked() {
                            selected = Some(i);
                        }
                    }
                });
        });
        selected
    }

    fn row(
        &self,
        ui: &mut Ui,
        id: Id,
        index: usize,
        suggestion: &Suggestion,
        corner: CornerRadius,
    ) -> Response {
        let theme = self.field.theme;
        let dir = theme.layout_direction;
        let ctx = ui.ctx().clone();
        let size = Vec2::new(ui.available_width(), row_height(suggestion));
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        let response = ui.interact(rect, id, Sense::click());
        let highlighted = self.highlighted == Some(index);
        crate::a11y::describe(&response, Role::ListBoxOption, true, &suggestion.text, None);
        // An option reports the highlight as `selected` (egui's
        // `WidgetInfo::selected` maps to `toggled`, which is for checkboxes).
        ui.ctx()
            .accesskit_node_builder(response.id, |node| node.set_selected(highlighted));
        if highlighted && self.scroll_to_highlight {
            ui.scroll_to_rect(rect, None);
        }

        let painter = ui.painter();
        let on_surface = theme.color(ColorRole::OnSurface);
        if highlighted {
            painter.rect_filled(rect, corner, on_surface.gamma_multiply(FOCUS_OPACITY));
        }
        paint_state_layer(
            painter,
            rect,
            corner,
            on_surface,
            InteractionState::from_response(&response),
        );

        let mut text_start = ROW_EDGE_SPACE;
        if let Some(icon) = &suggestion.icon {
            let slot = from_start(rect, ROW_EDGE_SPACE, ICON_SIZE, dir);
            let icon_rect = Rect::from_center_size(slot.center(), Vec2::splat(ICON_SIZE));
            // `ListTokens.ItemLeadingIconColor`.
            let tint = theme.color(ColorRole::OnSurfaceVariant);
            paint_icon(painter, icon_rect, icon, &SymbolOptions::default(), tint);
            text_start += ICON_SIZE + ROW_BETWEEN_SPACE;
        }
        let text_width = (rect.width() - text_start - ROW_EDGE_SPACE).max(0.0);
        let text_span = from_start(rect, text_start, text_width, dir);

        // `ListTokens.ItemLabelTextFont` / `ItemLabelTextColor`.
        let label = one_line(
            TypeRole::BodyLarge
                .style()
                .layout_job(&ctx, suggestion.text.clone(), on_surface),
            text_width,
        );
        let label = painter.layout_job(label);
        // `ListTokens.ItemSupportingTextFont` / `ItemSupportingTextColor`.
        let supporting = suggestion.supporting_text.as_ref().map(|text| {
            painter.layout_job(one_line(
                TypeRole::BodyMedium.style().layout_job(
                    &ctx,
                    text.clone(),
                    theme.color(ColorRole::OnSurfaceVariant),
                ),
                text_width,
            ))
        });
        let block_height = label.size().y + supporting.as_ref().map_or(0.0, |s| s.size().y);
        let mut y = rect.center().y - block_height / 2.0;
        let x_for = |w: f32| match dir {
            LayoutDirection::Ltr => text_span.left(),
            LayoutDirection::Rtl => text_span.right() - w,
        };
        let label_height = label.size().y;
        painter.galley(egui::pos2(x_for(label.size().x), y), label, on_surface);
        y += label_height;
        if let Some(supporting) = supporting {
            let x = x_for(supporting.size().x);
            painter.galley(egui::pos2(x, y), supporting, on_surface);
        }
        response
    }
}

fn row_height(suggestion: &Suggestion) -> f32 {
    if suggestion.supporting_text.is_some() {
        ROW_TWO_LINE_HEIGHT
    } else {
        ROW_ONE_LINE_HEIGHT
    }
}

/// `job` cut to one row of at most `width`, ending in an ellipsis.
fn one_line(mut job: LayoutJob, width: f32) -> LayoutJob {
    job.wrap = TextWrapping {
        max_width: width,
        max_rows: 1,
        break_anywhere: true,
        overflow_character: Some('…'),
    };
    job
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_and_end_spans_mirror_in_rtl() {
        let outer = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(300.0, 56.0));
        let ltr = from_start(outer, 16.0, 24.0, LayoutDirection::Ltr);
        let rtl = from_start(outer, 16.0, 24.0, LayoutDirection::Rtl);
        assert_eq!(
            ltr.x_range(),
            egui::Rangef::new(16.0, 40.0),
            "LTR start is left"
        );
        assert_eq!(
            rtl.x_range(),
            egui::Rangef::new(260.0, 284.0),
            "RTL start is right"
        );
        let end = from_end(outer, 16.0, 24.0, LayoutDirection::Ltr);
        assert_eq!(end.x_range(), rtl.x_range(), "LTR end is RTL start");
    }

    #[test]
    fn text_leaves_room_for_trailing_icons() {
        let outer = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(400.0, 56.0));
        let none = text_rect(outer, 0, LayoutDirection::Ltr);
        assert_eq!(none.left(), 56.0, "16 + 24 + 16 before the text");
        assert_eq!(none.right(), 384.0, "16 after the text with no icon");
        let two = text_rect(outer, 2, LayoutDirection::Ltr);
        assert_eq!(
            two.right(),
            400.0 - (16.0 + 48.0 + 24.0 + 16.0),
            "two icons"
        );
    }
}
