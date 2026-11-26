//! Material Design 3 radio buttons, radio groups and radio list tiles.
//!
//! Geometry, colours and opacities follow Compose's `RadioButtonTokens`:
//!
//! - a 20 dp icon: a ring with a 2 dp stroke drawn *inside* it and, when
//!   selected, a 12 dp dot that grows in over 100 ms; centred in a 48 dp
//!   touch target;
//! - a 40 dp circular state layer in `primary` (selected) or `onSurface`;
//! - selected: `primary`; unselected: `onSurfaceVariant`, `onSurface` when
//!   hovered, focused or pressed; disabled: `onSurface` at 38 %.
//!
//! [`MaterialRadioGroup`] is one tab stop: Tab reaches the selected option
//! (or the first), and the arrow keys move the selection and the focus
//! within the group, wrapping at the ends, as WAI-ARIA radio groups do.
//!
//! [`RadioListTile`] is a list item (Compose's `ListTokens`): 56 dp one-line
//! and 72 dp two-line, 16 dp leading and trailing space, 12 dp between the
//! radio and the text, label in `BodyLarge` / `onSurface`, supporting text
//! in `BodyMedium` / `onSurfaceVariant`, and a selected item in
//! `secondaryContainer` with `CornerLarge` corners.

use crate::checkbox::{focus_ring, full_corner, label_color, label_galley, TOUCH_TARGET};
use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{ColorRole, InteractionState, LayoutDirection, TypeRole};
use crate::MaterialTheme;
use egui::{
    self, text::TextWrapping, Color32, CornerRadius, EventFilter, Id, Key, Modifiers, Painter,
    Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget,
};

/// `RadioButtonTokens.IconSize`.
pub const ICON_SIZE: f32 = 20.0;
/// Ring stroke width, drawn inside the icon.
pub const RING_WIDTH: f32 = 2.0;
/// Diameter of the selected dot (Compose's `RadioButtonDotSize`).
pub const DOT_SIZE: f32 = 12.0;
/// `RadioButtonTokens.StateLayerSize`.
pub const STATE_LAYER_SIZE: f32 = 40.0;
/// How long the dot takes to grow or shrink (Compose's `RadioAnimationDuration`).
const DOT_ANIMATION_SECONDS: f32 = 0.1;

/// `ListTokens.ItemOneLineContainerHeight`.
pub const LIST_ONE_LINE_HEIGHT: f32 = 56.0;
/// `ListTokens.ItemTwoLineContainerHeight`.
pub const LIST_TWO_LINE_HEIGHT: f32 = 72.0;
/// `ListTokens.ItemLeadingSpace` and `ItemTrailingSpace`.
pub const LIST_EDGE_SPACE: f32 = 16.0;
/// `ListTokens.ItemBetweenSpace`.
pub const LIST_BETWEEN_SPACE: f32 = 12.0;
/// `ListTokens.ItemSelectedContainerShape` (`CornerLarge`).
pub const LIST_SELECTED_CORNER: u8 = 16;
/// `ListTokens.ItemSelectedDisabledContainerOpacity`.
const LIST_SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
/// How much shorter a dense tile is (density −2, 4 dp per step).
const DENSE_REDUCTION: f32 = 8.0;

/// Material Design 3 radio button.
///
/// ```rust
/// # use egui_material3::MaterialRadio;
/// # egui::__run_test_ui(|ui| {
/// let mut selected = Some(0);
/// ui.add(MaterialRadio::new(&mut selected, 0, "Option 1"));
/// ui.add(MaterialRadio::new(&mut selected, 1, "Option 2"));
/// # });
/// ```
///
/// Standalone radios are each a tab stop; put them in a
/// [`MaterialRadioGroup`] for one tab stop and arrow-key navigation.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialRadio<'a, T: PartialEq + Clone> {
    selected: &'a mut Option<T>,
    value: T,
    text: String,
    enabled: bool,
    toggleable: bool,
    fill_color: Option<Color32>,
    overlay_color: Option<Color32>,
    background_color: Option<Color32>,
    inner_radius: Option<f32>,
    splash_radius: Option<f32>,
    id: Option<Id>,
    /// In a group: `Some(true)` for the group's tab stop, `Some(false)` for
    /// the others (clickable, reachable by arrows, skipped by Tab).
    roving: Option<bool>,
}

/// A set of radio buttons, one selected, with a single tab stop.
///
/// ```rust
/// # use egui_material3::MaterialRadioGroup;
/// # egui::__run_test_ui(|ui| {
/// let mut selected = Some(0);
/// ui.add(
///     MaterialRadioGroup::new(&mut selected)
///         .option(0, "First Option")
///         .option(1, "Second Option")
///         .option(2, "Third Option"),
/// );
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialRadioGroup<'a, T: PartialEq + Clone> {
    selected: &'a mut Option<T>,
    options: Vec<RadioOption<T>>,
    enabled: bool,
    toggleable: bool,
    id_salt: Option<egui::IdSalt>,
}

/// One option of a [`MaterialRadioGroup`].
pub struct RadioOption<T: PartialEq + Clone> {
    text: String,
    value: T,
}

impl<'a, T: PartialEq + Clone> MaterialRadio<'a, T> {
    /// A radio button selecting `value` into `selected`, labelled `text`.
    pub fn new(selected: &'a mut Option<T>, value: T, text: impl Into<String>) -> Self {
        Self {
            selected,
            value,
            text: text.into(),
            enabled: true,
            toggleable: false,
            fill_color: None,
            overlay_color: None,
            background_color: None,
            inner_radius: None,
            splash_radius: None,
            id: None,
            roving: None,
        }
    }

    /// Enables or disables the radio button. A disabled one is dimmed, is
    /// not focusable and ignores clicks.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Whether clicking the selected radio clears the selection.
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    /// Overrides the selected colour (default `primary`).
    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Overrides the state-layer colour (default `primary` when selected,
    /// `onSurface` otherwise). The state opacity is applied on top.
    pub fn overlay_color(mut self, color: Color32) -> Self {
        self.overlay_color = Some(color);
        self
    }

    /// Fills the icon's circle behind the ring (default transparent).
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Overrides the radius of the selected dot (default 6 dp).
    pub fn inner_radius(mut self, radius: f32) -> Self {
        self.inner_radius = Some(radius);
        self
    }

    /// Overrides the radius of the state layer (default 20 dp).
    pub fn splash_radius(mut self, radius: f32) -> Self {
        self.splash_radius = Some(radius);
        self
    }
}

/// Colour of a radio icon (ring and dot) in a state.
fn radio_color(
    theme: &MaterialTheme,
    selected: bool,
    enabled: bool,
    active: bool,
    fill: Option<Color32>,
) -> Color32 {
    if !enabled {
        theme
            .color(ColorRole::OnSurface)
            .gamma_multiply(DISABLED_CONTENT_OPACITY)
    } else if selected {
        fill.unwrap_or(theme.color(ColorRole::Primary))
    } else if active {
        theme.color(ColorRole::OnSurface)
    } else {
        theme.color(ColorRole::OnSurfaceVariant)
    }
}

/// Paints the 20 dp radio icon at `center`: the ring, inside the icon, and
/// a dot of `dot_radius`.
fn paint_radio_icon(painter: &Painter, center: Pos2, color: Color32, dot_radius: f32) {
    painter.circle_stroke(
        center,
        (ICON_SIZE - RING_WIDTH) / 2.0,
        Stroke::new(RING_WIDTH, color),
    );
    if dot_radius > 0.0 {
        painter.circle_filled(center, dot_radius, color);
    }
}

/// The animated dot radius for a radio whose response id is `id`.
fn dot_radius(ui: &Ui, id: Id, selected: bool, full: f32) -> f32 {
    full * ui
        .ctx()
        .animate_bool_with_time(id.with("dot"), selected, DOT_ANIMATION_SECONDS)
}

impl<T: PartialEq + Clone> Widget for MaterialRadio<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let text_color = label_color(&theme, self.enabled);
        let galley = label_galley(ui, &self.text, text_color);
        let label_width = galley.as_ref().map_or(0.0, |g| g.size().x);
        let desired_size = Vec2::new(TOUCH_TARGET + label_width, TOUCH_TARGET);

        let sense = match self.roving {
            Some(false) => Sense::CLICK,
            Some(true) | None => Sense::click(),
        };
        let (auto_id, rect) = ui.allocate_space(desired_size);
        let id = self.id.unwrap_or(auto_id);
        let mut response = ui.interact(rect, id, crate::a11y::sense(self.enabled, sense));

        let was_selected = self.selected.as_ref() == Some(&self.value);
        if response.clicked() && self.enabled {
            if self.toggleable && was_selected {
                *self.selected = None;
            } else {
                *self.selected = Some(self.value.clone());
            }
            response.mark_changed();
        }
        let selected = self.selected.as_ref() == Some(&self.value);
        crate::a11y::toggle(
            &response,
            egui::Role::RadioButton,
            self.enabled,
            &self.text,
            selected,
        );

        // In a group, the arrows move the selection, not egui's focus.
        if self.roving.is_some() && response.has_focus() {
            ui.memory_mut(|m| {
                m.set_focus_lock_filter(
                    id,
                    EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        ..Default::default()
                    },
                );
            });
        }

        let interaction = InteractionState::from_response(&response);
        let active = interaction.hovered || interaction.focused || interaction.pressed;
        let color = radio_color(&theme, selected, self.enabled, active, self.fill_color);
        let layer_color = self.overlay_color.unwrap_or(if selected {
            self.fill_color.unwrap_or(theme.color(ColorRole::Primary))
        } else {
            theme.color(ColorRole::OnSurface)
        });

        let target = Rect::from_min_size(rect.min, Vec2::splat(TOUCH_TARGET));
        let center = target.center();
        let layer_size = self.splash_radius.map_or(STATE_LAYER_SIZE, |r| r * 2.0);
        let layer_rect = Rect::from_center_size(center, Vec2::splat(layer_size));
        let painter = ui.painter();

        crate::paint::paint_state_layer(
            painter,
            layer_rect,
            full_corner(layer_size),
            layer_color,
            interaction,
        );
        if let Some(background) = self.background_color {
            painter.circle_filled(center, ICON_SIZE / 2.0, background);
        }
        let full_dot = self.inner_radius.unwrap_or(DOT_SIZE / 2.0);
        paint_radio_icon(
            painter,
            center,
            color,
            dot_radius(ui, id, selected, full_dot),
        );
        focus_ring(painter, &theme, &response, layer_rect, layer_size);

        if let Some(galley) = galley {
            let pos = Pos2::new(target.max.x, rect.center().y - galley.size().y / 2.0);
            painter.galley(pos, galley, text_color);
        }

        response
    }
}

impl<'a, T: PartialEq + Clone> MaterialRadioGroup<'a, T> {
    /// An empty group selecting into `selected`.
    pub fn new(selected: &'a mut Option<T>) -> Self {
        Self {
            selected,
            options: Vec::new(),
            enabled: true,
            toggleable: false,
            id_salt: None,
        }
    }

    /// Adds an option for `value`, labelled `text`.
    pub fn option(mut self, value: T, text: impl Into<String>) -> Self {
        self.options.push(RadioOption {
            text: text.into(),
            value,
        });
        self
    }

    /// Enables or disables every option.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Whether clicking the selected option clears the selection.
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    /// A stable id for the group, so keyboard focus survives other widgets
    /// appearing before it. Defaults to the next automatic id.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = Some(egui::IdSalt::new(salt));
        self
    }
}

/// Net arrow-key steps this frame: forward minus back. Down is forward;
/// Right is forward left-to-right and back right-to-left.
fn consume_arrow_steps(ui: &Ui, direction: LayoutDirection) -> i64 {
    let (forward, back) = match direction {
        LayoutDirection::Ltr => (Key::ArrowRight, Key::ArrowLeft),
        LayoutDirection::Rtl => (Key::ArrowLeft, Key::ArrowRight),
    };
    ui.input_mut(|i| {
        let count = |i: &mut egui::InputState, key| {
            i64::try_from(i.count_and_consume_key(Modifiers::NONE, key)).unwrap_or(0)
        };
        let down = count(i, Key::ArrowDown);
        let up = count(i, Key::ArrowUp);
        let fwd = count(i, forward);
        let bwd = count(i, back);
        down + fwd - up - bwd
    })
}

impl<T: PartialEq + Clone> Widget for MaterialRadioGroup<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let group_id = match self.id_salt {
            Some(salt) => ui.make_persistent_id(salt),
            None => ui.next_auto_id(),
        };
        let ids: Vec<Id> = (0..self.options.len()).map(|i| group_id.with(i)).collect();

        // Arrow keys on the focused option select and focus the next one,
        // before drawing, so this frame already shows the new selection.
        let mut changed = false;
        let focused = ids.iter().position(|id| ui.memory(|m| m.has_focus(*id)));
        if let (true, Some(focused)) = (self.enabled, focused) {
            let direction = MaterialTheme::of(ui.ctx()).layout_direction;
            let steps = consume_arrow_steps(ui, direction);
            let len = i64::try_from(self.options.len()).unwrap_or(i64::MAX);
            let from = i64::try_from(focused).unwrap_or(0);
            let next = usize::try_from((from + steps).rem_euclid(len.max(1))).unwrap_or(0);
            if steps != 0 {
                if let (Some(option), Some(id)) = (self.options.get(next), ids.get(next)) {
                    *self.selected = Some(option.value.clone());
                    ui.memory_mut(|m| m.request_focus(*id));
                    changed = true;
                }
            }
        }

        let tab_stop = self
            .options
            .iter()
            .position(|o| self.selected.as_ref() == Some(&o.value))
            .unwrap_or(0);

        let mut group_response: Option<Response> = None;
        ui.vertical(|ui| {
            for (i, (option, id)) in self.options.into_iter().zip(ids).enumerate() {
                let mut radio = MaterialRadio::new(self.selected, option.value, option.text)
                    .enabled(self.enabled)
                    .toggleable(self.toggleable);
                radio.id = Some(id);
                radio.roving = Some(i == tab_stop);
                let response = ui.add(radio);
                group_response = Some(match group_response.take() {
                    Some(group) => group.union(response),
                    None => response,
                });
            }
        });

        let mut response = group_response.unwrap_or_else(|| {
            let (_rect, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::hover());
            response
        });
        if changed {
            response.mark_changed();
        }
        response
    }
}

/// Where the radio sits in a [`RadioListTile`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListTileControlAffinity {
    /// Before the text (leading edge).
    Leading,
    /// After the text (trailing edge).
    Trailing,
}

/// A list item with a radio button; the whole tile selects.
///
/// ```rust
/// # use egui_material3::RadioListTile;
/// # egui::__run_test_ui(|ui| {
/// let mut selected = Some(0);
/// ui.add(RadioListTile::new(&mut selected, 0)
///     .title("First Option")
///     .subtitle("Description of first option"));
/// ui.add(RadioListTile::new(&mut selected, 1).title("Second Option"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct RadioListTile<'a, T: PartialEq + Clone> {
    selected: &'a mut Option<T>,
    value: T,
    title: Option<String>,
    subtitle: Option<String>,
    enabled: bool,
    toggleable: bool,
    control_affinity: ListTileControlAffinity,
    dense: bool,
    fill_color: Option<Color32>,
    tile_color: Option<Color32>,
    selected_tile_color: Option<Color32>,
}

impl<'a, T: PartialEq + Clone> RadioListTile<'a, T> {
    /// A tile selecting `value` into `selected`.
    pub fn new(selected: &'a mut Option<T>, value: T) -> Self {
        Self {
            selected,
            value,
            title: None,
            subtitle: None,
            enabled: true,
            toggleable: false,
            control_affinity: ListTileControlAffinity::Leading,
            dense: false,
            fill_color: None,
            tile_color: None,
            selected_tile_color: None,
        }
    }

    /// The label text (`BodyLarge`); also the accessible name.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Supporting text under the label (`BodyMedium`). Makes the tile
    /// two-line (72 dp).
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Enables or disables the tile.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Whether clicking the selected tile clears the selection.
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    /// Where the radio sits (default leading).
    pub fn control_affinity(mut self, affinity: ListTileControlAffinity) -> Self {
        self.control_affinity = affinity;
        self
    }

    /// 8 dp shorter (48 / 64 dp).
    pub fn dense(mut self, dense: bool) -> Self {
        self.dense = dense;
        self
    }

    /// Overrides the radio's selected colour (default `primary`).
    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Container colour when unselected (default transparent, showing the
    /// surface the list sits on).
    pub fn tile_color(mut self, color: Color32) -> Self {
        self.tile_color = Some(color);
        self
    }

    /// Container colour when selected (default `secondaryContainer`).
    pub fn selected_tile_color(mut self, color: Color32) -> Self {
        self.selected_tile_color = Some(color);
        self
    }
}

impl<T: PartialEq + Clone> Widget for RadioListTile<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let mut height = if self.subtitle.is_some() {
            LIST_TWO_LINE_HEIGHT
        } else {
            LIST_ONE_LINE_HEIGHT
        };
        if self.dense {
            height -= DENSE_REDUCTION;
        }
        let desired_size = Vec2::new(ui.available_width(), height);
        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            crate::a11y::sense(self.enabled, Sense::click()),
        );

        let was_selected = self.selected.as_ref() == Some(&self.value);
        if response.clicked() && self.enabled {
            if self.toggleable && was_selected {
                *self.selected = None;
            } else {
                *self.selected = Some(self.value.clone());
            }
            response.mark_changed();
        }
        let selected = self.selected.as_ref() == Some(&self.value);
        crate::a11y::toggle(
            &response,
            egui::Role::RadioButton,
            self.enabled,
            self.title.as_deref().unwrap_or_default(),
            selected,
        );

        let c = |role| theme.color(role);
        let dim = |color: Color32| color.gamma_multiply(DISABLED_CONTENT_OPACITY);
        let (container, label, supporting) = match (self.enabled, selected) {
            (true, true) => (
                self.selected_tile_color
                    .unwrap_or(c(ColorRole::SecondaryContainer)),
                c(ColorRole::OnSecondaryContainer),
                c(ColorRole::OnSecondaryContainer),
            ),
            (true, false) => (
                self.tile_color.unwrap_or(Color32::TRANSPARENT),
                c(ColorRole::OnSurface),
                c(ColorRole::OnSurfaceVariant),
            ),
            (false, true) => (
                self.selected_tile_color.unwrap_or(
                    c(ColorRole::OnSurface)
                        .gamma_multiply(LIST_SELECTED_DISABLED_CONTAINER_OPACITY),
                ),
                dim(c(ColorRole::OnSurface)),
                dim(c(ColorRole::OnSurface)),
            ),
            (false, false) => (
                self.tile_color.unwrap_or(Color32::TRANSPARENT),
                dim(c(ColorRole::OnSurface)),
                dim(c(ColorRole::OnSurface)),
            ),
        };
        let corner = if selected {
            CornerRadius::same(LIST_SELECTED_CORNER)
        } else {
            CornerRadius::ZERO
        };

        let painter = ui.painter();
        if container.a() > 0 {
            painter.rect_filled(rect, corner, container);
        }
        let interaction = InteractionState::from_response(&response);
        crate::paint::paint_state_layer(painter, rect, corner, label, interaction);

        let (radio_x, text_left, text_right) = match self.control_affinity {
            ListTileControlAffinity::Leading => {
                let radio_x = rect.min.x + LIST_EDGE_SPACE + ICON_SIZE / 2.0;
                (
                    radio_x,
                    radio_x + ICON_SIZE / 2.0 + LIST_BETWEEN_SPACE,
                    rect.max.x - LIST_EDGE_SPACE,
                )
            }
            ListTileControlAffinity::Trailing => {
                let radio_x = rect.max.x - LIST_EDGE_SPACE - ICON_SIZE / 2.0;
                (
                    radio_x,
                    rect.min.x + LIST_EDGE_SPACE,
                    radio_x - ICON_SIZE / 2.0 - LIST_BETWEEN_SPACE,
                )
            }
        };
        let active = interaction.hovered || interaction.focused || interaction.pressed;
        let radio = radio_color(&theme, selected, self.enabled, active, self.fill_color);
        let dot = dot_radius(ui, response.id, selected, DOT_SIZE / 2.0);
        paint_radio_icon(painter, Pos2::new(radio_x, rect.center().y), radio, dot);

        let text_width = (text_right - text_left).max(0.0);
        let layout = |role: TypeRole, text: &str, color: Color32| {
            let mut job = role.style().layout_job(ui.ctx(), text, color);
            job.wrap = TextWrapping::truncate_at_width(text_width);
            painter.layout_job(job)
        };
        let title = self
            .title
            .as_deref()
            .map(|t| layout(TypeRole::BodyLarge, t, label));
        let subtitle = self
            .subtitle
            .as_deref()
            .map(|t| layout(TypeRole::BodyMedium, t, supporting));
        let block_height = title.as_ref().map_or(0.0, |g| g.size().y)
            + subtitle.as_ref().map_or(0.0, |g| g.size().y);
        let mut y = rect.center().y - block_height / 2.0;
        for galley in [title, subtitle].into_iter().flatten() {
            let galley_height = galley.size().y;
            painter.galley(Pos2::new(text_left, y), galley, label);
            y += galley_height;
        }

        if response.has_focus() {
            // Inside the tile, so neighbouring tiles do not cover it.
            let inset = crate::paint::FOCUS_RING_OFFSET + crate::paint::FOCUS_RING_WIDTH;
            crate::paint::paint_focus_ring(
                painter,
                rect.shrink(inset),
                corner,
                c(ColorRole::Secondary),
            );
        }

        response
    }
}

/// Shorthand for [`MaterialRadio::new`].
pub fn radio<'a, T: PartialEq + Clone>(
    selected: &'a mut Option<T>,
    value: T,
    text: impl Into<String>,
) -> MaterialRadio<'a, T> {
    MaterialRadio::new(selected, value, text)
}

/// Shorthand for [`MaterialRadioGroup::new`].
pub fn radio_group<'a, T: PartialEq + Clone>(
    selected: &'a mut Option<T>,
) -> MaterialRadioGroup<'a, T> {
    MaterialRadioGroup::new(selected)
}

/// Shorthand for [`RadioListTile::new`].
pub fn radio_list_tile<'a, T: PartialEq + Clone>(
    selected: &'a mut Option<T>,
    value: T,
) -> RadioListTile<'a, T> {
    RadioListTile::new(selected, value)
}
