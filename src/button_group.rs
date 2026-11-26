//! Material 3 Expressive button groups.
//!
//! A [`ButtonGroup`] lays out a row of [`MaterialButton`]s:
//!
//! - **Standard** ([`ButtonGroup::standard`]): 12 dp apart
//!   (`StandardButtonGroupSmallTokens.BetweenSpace`). A pressed button
//!   grows by 15% of its width (`PressedItemWidthMultiplierPercent`) and its
//!   neighbours give up the same space, on the theme's fast spatial spring.
//! - **Connected** ([`ButtonGroup::connected`]): 2 dp apart
//!   (`ConnectedButtonGroupSmallTokens.BetweenSpace`), with fully rounded
//!   outer ends and 8 dp inner corners (`InnerCornerCornerSize`) that
//!   tighten to 4 dp while pressed (`PressedInnerCornerCornerSize`) and
//!   become fully round on a selected item (`SelectedInnerCornerCornerSizePercent`).
//!
//! Either kind can make its buttons a single-select set
//! ([`ButtonGroup::single_select`], reported to screen readers as radio
//! buttons) or a multi-select set ([`ButtonGroup::multi_select`], toggle
//! buttons). With the keyboard, Tab enters the group and the arrow keys
//! move between its buttons, wrapping at the ends.

use egui::{AsIdSalt, Event, EventFilter, IdSalt, Key, Response, Role, Ui, Widget};

use crate::button::{ButtonSize, MaterialButton, SegmentCorners, SideCorners};
use crate::tokens::{animate_value_with_spring, LayoutDirection, SpringSpeed};
use crate::MaterialTheme;

/// `StandardButtonGroupSmallTokens.BetweenSpace`: 12 dp.
pub const STANDARD_GAP: f32 = 12.0;
/// `ConnectedButtonGroupSmallTokens.BetweenSpace`: 2 dp.
pub const CONNECTED_GAP: f32 = 2.0;
/// `StandardButtonGroupSmallTokens.PressedItemWidthMultiplierPercent`: 15%.
pub const PRESSED_GROWTH: f32 = 0.15;
/// `ConnectedButtonGroupSmallTokens.InnerCornerCornerSize`: 8 dp.
pub const CONNECTED_INNER_CORNER: f32 = 8.0;
/// `ConnectedButtonGroupSmallTokens.PressedInnerCornerCornerSize`: 4 dp.
pub const CONNECTED_PRESSED_INNER_CORNER: f32 = 4.0;

/// Which kind of group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GroupKind {
    Standard,
    Connected,
}

/// What clicking a button in the group selects.
enum Selection<'a> {
    None,
    Single(&'a mut Option<usize>),
    Multi(&'a mut [bool]),
}

/// A row of buttons with M3 group spacing, shapes and selection.
///
/// ```no_run
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{ButtonGroup, MaterialButton};
/// let mut view = Some(0);
/// ButtonGroup::connected()
///     .single_select(&mut view)
///     .button(MaterialButton::outlined("Day"))
///     .button(MaterialButton::outlined("Week"))
///     .button(MaterialButton::outlined("Month"))
///     .show(ui);
/// # });
/// ```
#[must_use = "You should call `.show(ui)` or `ui.add(group)`"]
pub struct ButtonGroup<'a> {
    kind: GroupKind,
    items: Vec<MaterialButton<'a>>,
    selection: Selection<'a>,
    size: Option<ButtonSize>,
    id_salt: Option<IdSalt>,
}

/// What a [`ButtonGroup`] reports back.
pub struct ButtonGroupResponse {
    /// The whole group; `changed()` when the selection changed.
    pub response: Response,
    /// One response per button, in order.
    pub items: Vec<Response>,
    /// The button clicked this frame, if any.
    pub clicked: Option<usize>,
}

impl<'a> ButtonGroup<'a> {
    fn new(kind: GroupKind) -> Self {
        Self {
            kind,
            items: Vec::new(),
            selection: Selection::None,
            size: None,
            id_salt: None,
        }
    }

    /// A standard group: separate buttons, 12 dp apart, pressed one grows.
    pub fn standard() -> Self {
        Self::new(GroupKind::Standard)
    }

    /// A connected group: segments 2 dp apart with shared inner corners.
    pub fn connected() -> Self {
        Self::new(GroupKind::Connected)
    }

    /// Adds a button.
    #[inline]
    pub fn button(mut self, button: MaterialButton<'a>) -> Self {
        self.items.push(button);
        self
    }

    /// Sets every button to `size`.
    #[inline]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Makes the buttons a single-select set: clicking one selects it.
    #[inline]
    pub fn single_select(mut self, selected: &'a mut Option<usize>) -> Self {
        self.selection = Selection::Single(selected);
        self
    }

    /// Makes the buttons toggle buttons: clicking one flips its flag.
    /// Buttons past the end of `selected` are plain push buttons.
    #[inline]
    pub fn multi_select(mut self, selected: &'a mut [bool]) -> Self {
        self.selection = Selection::Multi(selected);
        self
    }

    /// Distinguishes two groups with the same buttons in one `Ui`.
    #[inline]
    pub fn id_salt(mut self, salt: impl AsIdSalt) -> Self {
        self.id_salt = Some(IdSalt::new(salt));
        self
    }

    /// Lays out and paints the group.
    pub fn show(self, ui: &mut Ui) -> ButtonGroupResponse {
        let ButtonGroup {
            kind,
            items,
            mut selection,
            size,
            id_salt,
        } = self;
        let id = ui.make_persistent_id(id_salt.unwrap_or_else(|| IdSalt::new("m3_button_group")));
        let theme = MaterialTheme::of(ui.ctx());
        let spring = theme.motion.spatial(SpringSpeed::Fast);
        let pressed_before = ui.data(|d| d.get_temp::<Option<usize>>(id.with("pressed")));
        let count = items.len();

        let selected_at = |selection: &Selection<'_>, i: usize| match selection {
            Selection::None => None,
            Selection::Single(selected) => Some(**selected == Some(i)),
            Selection::Multi(flags) => flags.get(i).copied(),
        };

        let inner = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = match kind {
                GroupKind::Standard => STANDARD_GAP,
                GroupKind::Connected => CONNECTED_GAP,
            };
            let mut prepared = Vec::with_capacity(count);
            for (i, mut button) in items.into_iter().enumerate() {
                if let Some(size) = size {
                    button = button.size(size);
                }
                if let Some(selected) = selected_at(&selection, i) {
                    button = button.selected(selected);
                    if matches!(selection, Selection::Single(_)) {
                        button = button.role(Role::RadioButton);
                    }
                }
                if kind == GroupKind::Connected {
                    button = button.segment(connected_segment(i, count));
                }
                prepared.push(button.prepare(ui));
            }

            if kind == GroupKind::Standard {
                let natural: Vec<f32> = prepared.iter().map(|p| p.desired_width()).collect();
                let growth: Vec<f32> = (0..count)
                    .map(|i| {
                        let target = if pressed_before.flatten() == Some(i) {
                            1.0
                        } else {
                            0.0
                        };
                        let t = animate_value_with_spring(
                            ui.ctx(),
                            id.with(("grow", i)),
                            target,
                            spring,
                        );
                        t * PRESSED_GROWTH * natural.get(i).copied().unwrap_or(0.0)
                    })
                    .collect();
                for (i, delta) in width_deltas(&growth).into_iter().enumerate() {
                    if delta.abs() > 0.01 {
                        if let (Some(button), Some(width)) = (prepared.get_mut(i), natural.get(i)) {
                            button.set_width((width + delta).max(0.0));
                        }
                    }
                }
            }

            prepared
                .into_iter()
                .map(|button| button.show(ui))
                .collect::<Vec<Response>>()
        });
        let items = inner.inner;
        let mut response = inner.response;

        let pressed_now = items.iter().position(|r| r.is_pointer_button_down_on());
        ui.data_mut(|d| d.insert_temp(id.with("pressed"), pressed_now));

        // Arrow keys move focus within the group, wrapping at the ends.
        if let Some(focused) = items.iter().position(|r| r.has_focus()) {
            let filter = EventFilter {
                horizontal_arrows: true,
                vertical_arrows: true,
                ..Default::default()
            };
            if let Some(r) = items.get(focused) {
                ui.memory_mut(|m| m.set_focus_lock_filter(r.id, filter));
            }
            let rtl = theme.layout_direction == LayoutDirection::Rtl;
            let step = ui.input(|input| {
                input
                    .events
                    .iter()
                    .fold(0_isize, |step, event| match event {
                        Event::Key {
                            key,
                            pressed: true,
                            modifiers,
                            ..
                        } if !modifiers.any() => match key {
                            Key::ArrowDown => step + 1,
                            Key::ArrowUp => step - 1,
                            Key::ArrowRight => step + if rtl { -1 } else { 1 },
                            Key::ArrowLeft => step + if rtl { 1 } else { -1 },
                            _ => step,
                        },
                        _ => step,
                    })
            });
            if step != 0 && count > 0 {
                let next = (focused as isize + step).rem_euclid(count as isize) as usize;
                if let Some(r) = items.get(next) {
                    r.request_focus();
                }
            }
        }

        let clicked = items.iter().position(|r| r.clicked());
        if let Some(i) = clicked {
            let changed = match &mut selection {
                Selection::None => false,
                Selection::Single(selected) => {
                    let changed = **selected != Some(i);
                    **selected = Some(i);
                    changed
                }
                Selection::Multi(flags) => match flags.get_mut(i) {
                    Some(flag) => {
                        *flag = !*flag;
                        true
                    }
                    None => false,
                },
            };
            if changed {
                response.mark_changed();
                ui.ctx().request_repaint();
            }
        }

        ButtonGroupResponse {
            response,
            items,
            clicked,
        }
    }
}

impl Widget for ButtonGroup<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let group = self.show(ui);
        match group.clicked.and_then(|i| group.items.get(i)) {
            // Report the click on the group response too.
            Some(clicked) => group.response.union(clicked.clone()),
            None => group.response,
        }
    }
}

/// The corner rules for item `index` of `count` in a connected group.
pub(crate) fn connected_segment(index: usize, count: usize) -> SegmentCorners {
    let inner = SideCorners::Inner {
        idle: CONNECTED_INNER_CORNER,
        hovered: CONNECTED_INNER_CORNER,
        pressed: CONNECTED_PRESSED_INNER_CORNER,
        selected_full: true,
    };
    SegmentCorners {
        start: if index == 0 {
            SideCorners::Outer
        } else {
            inner
        },
        end: if index + 1 >= count {
            SideCorners::Outer
        } else {
            inner
        },
        selected: false,
    }
}

/// Width changes for a standard group: each item gains its own growth and
/// gives up an equal share of each neighbour's growth, so the row keeps
/// its total width.
fn width_deltas(growth: &[f32]) -> Vec<f32> {
    let n = growth.len();
    let neighbours = |i: usize| usize::from(i > 0) + usize::from(i + 1 < n);
    (0..n)
        .map(|i| {
            let own = growth.get(i).copied().unwrap_or(0.0);
            let mut taken = 0.0;
            for j in [i.checked_sub(1), Some(i + 1)].into_iter().flatten() {
                if let Some(g) = growth.get(j) {
                    taken += g / neighbours(j).max(1) as f32;
                }
            }
            own - taken
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_deltas_keep_the_row_width() {
        let deltas = width_deltas(&[0.0, 15.0, 0.0]);
        assert_eq!(deltas, vec![-7.5, 15.0, -7.5], "middle grows, ends shrink");
        let deltas = width_deltas(&[9.0, 0.0, 0.0]);
        assert_eq!(deltas, vec![9.0, -9.0, 0.0], "first grows, second shrinks");
        let total: f32 = width_deltas(&[3.0, 5.0, 7.0, 2.0]).iter().sum();
        assert!(total.abs() < 1e-4, "total width unchanged: {total}");
    }

    #[test]
    fn connected_inner_corners_follow_tokens() {
        let middle = connected_segment(1, 3);
        assert_eq!(middle.start.radius(40.0, false, false, false), 8.0, "idle");
        assert_eq!(
            middle.start.radius(40.0, false, true, false),
            4.0,
            "pressed"
        );
        assert_eq!(
            middle.start.radius(40.0, false, false, true),
            20.0,
            "selected"
        );
        assert_eq!(
            connected_segment(0, 3).start,
            SideCorners::Outer,
            "first start is outer"
        );
        assert_eq!(
            connected_segment(2, 3).end,
            SideCorners::Outer,
            "last end is outer"
        );
    }
}
