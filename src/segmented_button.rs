// ============================================================================
// Module:       segmented_button
// Description:  Material 3 segmented buttons, single and multi select.
//
// Dependencies: egui; crate::{a11y, material_symbol, material_theme, paint, tokens}
// ============================================================================

//! Material 3 segmented buttons.
//!
//! A row of two to five connected segments for picking one option
//! ([`SegmentedButton::single`]) or several ([`SegmentedButton::multi`]).
//! A selected segment shows a check mark before its label (in place of its
//! icon, if it has one).
//!
//! Values from Compose's `OutlinedSegmentedButtonTokens.kt`
//! (`ContainerHeight` 40 dp, `OutlineWidth` 1 dp, `IconSize` 18 dp,
//! `Shape` = `CornerFull`, `LabelTextFont` = `LabelLarge`,
//! `SelectedContainerColor` = `SecondaryContainer`, selected content
//! `OnSecondaryContainer`, unselected content `OnSurface`, `OutlineColor` =
//! `Outline`, disabled content 0.38 and outline 0.12 of `OnSurface`) and
//! `SegmentedButton.kt` (`IconSpacing` 8 dp, content padding 12 dp,
//! `ButtonDefaults.MinWidth` 58 dp, segments overlapping by the outline
//! width so neighbours share one border).
//!
//! Keyboard: Tab reaches the group, Space or Enter activates the focused
//! segment, and the arrow keys (plus Home and End) move between segments.
//! In a single-select group the arrows also move the selection, and only
//! the selected segment is a Tab stop, like a radio group. AccessKit sees a
//! radio group of radio buttons (single) or a group of checkboxes (multi).
//!
//! ```rust
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::{Segment, SegmentedButton};
//!
//! let mut view = 0_usize;
//! ui.add(
//!     SegmentedButton::single(&mut view)
//!         .segment(Segment::new("Day").icon("calendar_view_day"))
//!         .segment(Segment::new("Week"))
//!         .segment(Segment::new("Month")),
//! );
//!
//! let mut sizes = [false, true, false];
//! ui.add(
//!     SegmentedButton::multi(&mut sizes)
//!         .label("Sizes")
//!         .segments(["S", "M", "L"]),
//! );
//! # });
//! ```

use egui::{
    epaint::PathStroke, Color32, CornerRadius, EventFilter, Key, Pos2, Rect, Response, Role, Sense,
    Stroke, StrokeKind, Ui, Vec2, Widget,
};

use crate::material_symbol::material_symbol_text;
use crate::material_theme::MaterialTheme;
use crate::tokens::state::{DISABLED_CONTENT_OPACITY, PRESSED_OPACITY};
use crate::tokens::{ColorRole, InteractionState, LayoutDirection, TypeRole};

/// `OutlinedSegmentedButtonTokens.ContainerHeight`: 40 dp.
pub const SEGMENTED_BUTTON_HEIGHT: f32 = 40.0;
/// `OutlinedSegmentedButtonTokens.OutlineWidth`: 1 dp.
pub const SEGMENTED_BUTTON_OUTLINE_WIDTH: f32 = 1.0;
/// `OutlinedSegmentedButtonTokens.IconSize`: 18 dp.
pub const SEGMENTED_BUTTON_ICON_SIZE: f32 = 18.0;
/// `SegmentedButton.kt` `IconSpacing`: 8 dp between icon and label.
const ICON_SPACING: f32 = 8.0;
/// `SegmentedButtonDefaults.ContentPadding`: 12 dp start and end.
const CONTENT_PADDING: f32 = 12.0;
/// `ButtonDefaults.MinWidth`: 58 dp, the narrowest segment.
const MIN_SEGMENT_WIDTH: f32 = 58.0;
/// `OutlinedSegmentedButtonTokens.DisabledOutlineOpacity`.
const DISABLED_OUTLINE_OPACITY: f32 = 0.12;

/// One segment of a [`SegmentedButton`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Segment {
    label: String,
    icon: Option<String>,
    enabled: bool,
    accessible_label: Option<String>,
}

impl Segment {
    /// A segment showing `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            enabled: true,
            accessible_label: None,
        }
    }

    /// A segment showing only an icon (a Material Symbols name). Screen
    /// readers announce `accessible_label`.
    pub fn icon_only(icon: impl Into<String>, accessible_label: impl Into<String>) -> Self {
        Self {
            label: String::new(),
            icon: Some(icon.into()),
            enabled: true,
            accessible_label: Some(accessible_label.into()),
        }
    }

    /// An icon (a Material Symbols name) before the label. A selected
    /// segment shows the check mark in its place.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Whether this segment can be clicked. Default `true`.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// The name screen readers announce, when it should differ from the
    /// visible label.
    pub fn accessible_label(mut self, label: impl Into<String>) -> Self {
        self.accessible_label = Some(label.into());
        self
    }

    fn a11y_label(&self) -> &str {
        match &self.accessible_label {
            Some(label) => label,
            None => &self.label,
        }
    }
}

impl<S: Into<String>> From<S> for Segment {
    fn from(label: S) -> Self {
        Segment::new(label)
    }
}

enum Selection<'a> {
    Single(&'a mut usize),
    SingleOptional(&'a mut Option<usize>),
    Multi(&'a mut [bool]),
}

impl Selection<'_> {
    fn is_single(&self) -> bool {
        match self {
            Selection::Single(_) | Selection::SingleOptional(_) => true,
            Selection::Multi(_) => false,
        }
    }

    fn is_selected(&self, index: usize) -> bool {
        match self {
            Selection::Single(selected) => **selected == index,
            Selection::SingleOptional(selected) => **selected == Some(index),
            Selection::Multi(flags) => flags.get(index).copied().unwrap_or(false),
        }
    }

    fn selected_index(&self) -> Option<usize> {
        match self {
            Selection::Single(selected) => Some(**selected),
            Selection::SingleOptional(selected) => **selected,
            Selection::Multi(_) => None,
        }
    }

    /// Activates `index`: selects it (single) or toggles it (multi).
    /// Returns whether the selection changed.
    fn activate(&mut self, index: usize) -> bool {
        match self {
            Selection::Single(selected) => {
                let changed = **selected != index;
                **selected = index;
                changed
            }
            Selection::SingleOptional(selected) => {
                let changed = **selected != Some(index);
                **selected = Some(index);
                changed
            }
            Selection::Multi(flags) => match flags.get_mut(index) {
                Some(flag) => {
                    *flag = !*flag;
                    true
                }
                None => false,
            },
        }
    }

    /// Whether `index` has state to change (a multi-select slice may be
    /// shorter than the segment list).
    fn can_hold(&self, index: usize) -> bool {
        match self {
            Selection::Single(_) | Selection::SingleOptional(_) => true,
            Selection::Multi(flags) => index < flags.len(),
        }
    }
}

/// A Material 3 segmented button: connected segments for picking one or
/// several options. See the [module docs](self).
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct SegmentedButton<'a> {
    selection: Selection<'a>,
    segments: Vec<Segment>,
    enabled: bool,
    show_check: bool,
    full_width: bool,
    label: Option<String>,
    id_salt: Option<egui::IdSalt>,
}

impl<'a> SegmentedButton<'a> {
    /// A single-select group: exactly one segment is selected, the one at
    /// index `*selected`.
    pub fn single(selected: &'a mut usize) -> Self {
        Self::with_selection(Selection::Single(selected))
    }

    /// A single-select group that may start with nothing selected. Once
    /// the user picks a segment, one stays selected.
    pub fn single_optional(selected: &'a mut Option<usize>) -> Self {
        Self::with_selection(Selection::SingleOptional(selected))
    }

    /// A multi-select group: segment `i` is selected when `selected[i]` is
    /// `true`. Segments past the end of the slice cannot be selected.
    pub fn multi(selected: &'a mut [bool]) -> Self {
        Self::with_selection(Selection::Multi(selected))
    }

    fn with_selection(selection: Selection<'a>) -> Self {
        Self {
            selection,
            segments: Vec::new(),
            enabled: true,
            show_check: true,
            full_width: false,
            label: None,
            id_salt: None,
        }
    }

    /// Adds a segment (a [`Segment`] or just its label).
    pub fn segment(mut self, segment: impl Into<Segment>) -> Self {
        self.segments.push(segment.into());
        self
    }

    /// Adds several segments.
    pub fn segments<I, S>(mut self, segments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Segment>,
    {
        self.segments.extend(segments.into_iter().map(Into::into));
        self
    }

    /// Whether the whole group can be used. Default `true`.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Whether a selected segment shows the check mark. Default `true`.
    pub fn show_check(mut self, show: bool) -> Self {
        self.show_check = show;
        self
    }

    /// Fill the available width, splitting it evenly between segments.
    /// Otherwise every segment is as wide as the widest one needs.
    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    /// The group's name for screen readers ("View", "Sizes").
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Salt for the widget ids, for two groups whose automatic ids would
    /// otherwise collide.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = Some(egui::IdSalt::new(salt));
        self
    }
}

/// The colours of one segment in one state.
struct SegmentColors {
    container: Color32,
    content: Color32,
    outline: Color32,
}

fn segment_colors(theme: &MaterialTheme, selected: bool, enabled: bool) -> SegmentColors {
    let on_surface = theme.color(ColorRole::OnSurface);
    let container = if selected {
        theme.color(ColorRole::SecondaryContainer)
    } else {
        Color32::TRANSPARENT
    };
    if !enabled {
        return SegmentColors {
            container,
            content: on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY),
            outline: on_surface.gamma_multiply(DISABLED_OUTLINE_OPACITY),
        };
    }
    SegmentColors {
        container,
        content: if selected {
            theme.color(ColorRole::OnSecondaryContainer)
        } else {
            on_surface
        },
        outline: theme.color(ColorRole::Outline),
    }
}

/// Corner radii of segment `index` of `count`: the outer ends are fully
/// round, inner edges square. `visual_index` is the left-to-right position.
fn segment_corners(visual_index: usize, count: usize, height: f32) -> CornerRadius {
    let full = (height / 2.0).round().clamp(0.0, 255.0) as u8;
    let first = visual_index == 0;
    let last = visual_index + 1 == count;
    CornerRadius {
        nw: if first { full } else { 0 },
        sw: if first { full } else { 0 },
        ne: if last { full } else { 0 },
        se: if last { full } else { 0 },
    }
}

/// Paints a check mark (the Material `check` glyph's geometry) in `rect`.
pub(crate) fn paint_check(painter: &egui::Painter, rect: Rect, color: Color32) {
    // The `check` symbol in its 24-unit grid: (4.5, 12.5) -> (9, 17) -> (19.5, 6.5).
    let point = |x: f32, y: f32| {
        Pos2::new(
            rect.min.x + rect.width() * x / 24.0,
            rect.min.y + rect.height() * y / 24.0,
        )
    };
    let width = (rect.width() / 9.0).max(1.5);
    painter.add(egui::Shape::line(
        vec![point(4.5, 12.5), point(9.5, 17.5), point(19.5, 6.5)],
        PathStroke::new(width, color),
    ));
}

impl Widget for SegmentedButton<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let ctx = ui.ctx().clone();
        let count = self.segments.len();
        let base_id = match self.id_salt {
            Some(salt) => ui.make_persistent_id(salt),
            None => ui.next_auto_id(),
        };
        let segment_id = |index: usize| base_id.with(("segment", index));
        let segment_enabled = |segments: &[Segment], index: usize, enabled: bool| {
            enabled && segments.get(index).is_some_and(|s| s.enabled)
        };

        // Keyboard, before painting, so the frame shows its result.
        let mut changed = false;
        let focused = ctx.memory(|m| m.focused());
        let focused_index = (0..count).find(|&i| focused == Some(segment_id(i)));
        if let Some(current) = focused_index {
            ctx.memory_mut(|m| {
                m.set_focus_lock_filter(
                    segment_id(current),
                    EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        ..Default::default()
                    },
                );
            });
            let (next_key, prev_key) = match theme.layout_direction {
                LayoutDirection::Ltr => (Key::ArrowRight, Key::ArrowLeft),
                LayoutDirection::Rtl => (Key::ArrowLeft, Key::ArrowRight),
            };
            let none = egui::Modifiers::NONE;
            let (forward, backward, home, end) = ui.input_mut(|i| {
                (
                    i.consume_key(none, next_key) || i.consume_key(none, Key::ArrowDown),
                    i.consume_key(none, prev_key) || i.consume_key(none, Key::ArrowUp),
                    i.consume_key(none, Key::Home),
                    i.consume_key(none, Key::End),
                )
            });
            let usable = |i: usize| {
                segment_enabled(&self.segments, i, self.enabled) && self.selection.can_hold(i)
            };
            let target = if home {
                (0..count).find(|&i| usable(i))
            } else if end {
                (0..count).rev().find(|&i| usable(i))
            } else if forward {
                (1..count)
                    .map(|step| (current + step) % count)
                    .find(|&i| usable(i))
            } else if backward {
                (1..count)
                    .map(|step| (current + count - step) % count)
                    .find(|&i| usable(i))
            } else {
                None
            };
            if let Some(target) = target {
                ctx.memory_mut(|m| m.request_focus(segment_id(target)));
                if self.selection.is_single() {
                    changed |= self.selection.activate(target);
                }
            }
        }

        // Measure.
        let label_style = TypeRole::LabelLarge.style();
        let galleys: Vec<_> = self
            .segments
            .iter()
            .map(|segment| {
                (!segment.label.is_empty()).then(|| {
                    ui.painter().layout_job(label_style.layout_job(
                        &ctx,
                        segment.label.clone(),
                        Color32::PLACEHOLDER,
                    ))
                })
            })
            .collect();
        let lead_width = SEGMENTED_BUTTON_ICON_SIZE + ICON_SPACING;
        let natural_width = self
            .segments
            .iter()
            .zip(&galleys)
            .map(|(segment, galley)| {
                let content = match galley {
                    Some(galley) => {
                        let lead = if segment.icon.is_some() || self.show_check {
                            lead_width
                        } else {
                            0.0
                        };
                        lead + galley.size().x
                    }
                    None => SEGMENTED_BUTTON_ICON_SIZE,
                };
                (content + 2.0 * CONTENT_PADDING).max(MIN_SEGMENT_WIDTH)
            })
            .fold(MIN_SEGMENT_WIDTH, f32::max);
        let overlap = SEGMENTED_BUTTON_OUTLINE_WIDTH;
        let n = count.max(1) as f32;
        let total_width = if self.full_width {
            ui.available_width()
        } else {
            natural_width * n - overlap * (n - 1.0)
        };
        let segment_width = (total_width + overlap * (n - 1.0)) / n;

        let (rect, mut response) = ui.allocate_exact_size(
            Vec2::new(total_width, SEGMENTED_BUTTON_HEIGHT),
            Sense::hover(),
        );
        let group_role = if self.selection.is_single() {
            Role::RadioGroup
        } else {
            Role::Group
        };
        let group_label = self.label.clone().unwrap_or_default();
        crate::a11y::describe(&response, group_role, self.enabled, &group_label, None);

        let selected_index = self.selection.selected_index();
        let mut clicked_segment = None;
        let mut focused_rect = None;
        for (index, (segment, galley)) in self.segments.iter().zip(galleys).enumerate() {
            let visual_index = match theme.layout_direction {
                LayoutDirection::Ltr => index,
                LayoutDirection::Rtl => count - 1 - index,
            };
            let segment_rect = Rect::from_min_size(
                Pos2::new(
                    rect.min.x + visual_index as f32 * (segment_width - overlap),
                    rect.min.y,
                ),
                Vec2::new(segment_width, SEGMENTED_BUTTON_HEIGHT),
            );
            let enabled = segment_enabled(&self.segments, index, self.enabled)
                && self.selection.can_hold(index);
            let selected = self.selection.is_selected(index);
            // In a single-select group only the selected segment (or the
            // first, when none is) is a Tab stop; arrows reach the rest.
            let tab_stop = match (self.selection.is_single(), selected_index) {
                (true, Some(sel)) if sel < count => sel == index,
                (true, _) => index == 0,
                (false, _) => true,
            };
            let sense = if tab_stop || focused_index == Some(index) {
                Sense::click()
            } else {
                Sense::CLICK
            };
            let seg_response = ui.interact(
                segment_rect,
                segment_id(index),
                crate::a11y::sense(enabled, sense),
            );
            let role = if self.selection.is_single() {
                Role::RadioButton
            } else {
                Role::CheckBox
            };
            crate::a11y::toggle(&seg_response, role, enabled, segment.a11y_label(), selected);
            if seg_response.clicked() {
                clicked_segment = Some(index);
                ctx.memory_mut(|m| m.request_focus(segment_id(index)));
            }

            let corners = segment_corners(visual_index, count, SEGMENTED_BUTTON_HEIGHT);
            let colors = segment_colors(&theme, selected, enabled);
            let painter = ui.painter();
            if colors.container != Color32::TRANSPARENT {
                painter.rect_filled(segment_rect, corners, colors.container);
            }
            let mut state = InteractionState::from_response(&seg_response);
            // A keyboard-activated segment flashes its pressed layer.
            if seg_response.clicked() && !seg_response.hovered() {
                state.pressed = true;
            }
            let opacity = if state.pressed {
                PRESSED_OPACITY
            } else {
                state.opacity()
            };
            if opacity > 0.0 {
                painter.rect_filled(
                    segment_rect,
                    corners,
                    colors.content.gamma_multiply(opacity),
                );
            }
            painter.rect_stroke(
                segment_rect,
                corners,
                Stroke::new(SEGMENTED_BUTTON_OUTLINE_WIDTH, colors.outline),
                StrokeKind::Inside,
            );
            if seg_response.has_focus() {
                focused_rect = Some((segment_rect, corners));
            }

            // Content, centred: [check or icon] [label].
            let show_lead_check = selected && self.show_check;
            let lead_icon = segment.icon.as_deref();
            let center = segment_rect.center();
            let icon_rect_at = |x: f32| {
                Rect::from_min_size(
                    Pos2::new(x, center.y - SEGMENTED_BUTTON_ICON_SIZE / 2.0),
                    Vec2::splat(SEGMENTED_BUTTON_ICON_SIZE),
                )
            };
            let paint_lead = |x: f32| {
                let icon_rect = icon_rect_at(x);
                if show_lead_check {
                    paint_check(painter, icon_rect, colors.content);
                } else if let Some(icon) = lead_icon {
                    painter.text(
                        icon_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        material_symbol_text(icon),
                        egui::FontId::proportional(SEGMENTED_BUTTON_ICON_SIZE),
                        colors.content,
                    );
                }
            };
            let has_lead = show_lead_check || lead_icon.is_some();
            match galley {
                Some(galley) => {
                    let content_width = galley.size().x + if has_lead { lead_width } else { 0.0 };
                    let max_text = (segment_rect.width()
                        - 2.0 * CONTENT_PADDING
                        - if has_lead { lead_width } else { 0.0 })
                    .max(0.0);
                    let mut x = center.x - content_width.min(segment_rect.width()) / 2.0;
                    x = x.max(segment_rect.min.x + CONTENT_PADDING);
                    if has_lead {
                        paint_lead(x);
                        x += lead_width;
                    }
                    let text_rect = Rect::from_min_size(
                        Pos2::new(x, center.y - galley.size().y / 2.0),
                        Vec2::new(max_text, galley.size().y),
                    );
                    painter
                        .with_clip_rect(text_rect.intersect(segment_rect))
                        .galley(text_rect.min, galley, colors.content);
                }
                None => paint_lead(center.x - SEGMENTED_BUTTON_ICON_SIZE / 2.0),
            }
        }
        if let Some((focus_rect, corners)) = focused_rect {
            crate::paint::paint_focus_ring(
                ui.painter(),
                focus_rect,
                corners,
                theme.color(ColorRole::Secondary),
            );
        }

        if let Some(index) = clicked_segment {
            changed |= self.selection.activate(index);
        }
        if changed {
            response.mark_changed();
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outer_corners_are_round_and_inner_square() {
        let first = segment_corners(0, 3, 40.0);
        let middle = segment_corners(1, 3, 40.0);
        let last = segment_corners(2, 3, 40.0);
        assert_eq!((first.nw, first.ne), (20, 0), "first: round start only");
        assert_eq!(middle, CornerRadius::ZERO, "middle: square");
        assert_eq!((last.nw, last.se), (0, 20), "last: round end only");
        let only = segment_corners(0, 1, 40.0);
        assert_eq!(only, CornerRadius::same(20), "a lone segment is a pill");
    }

    #[test]
    fn multi_selection_ignores_segments_past_the_slice() {
        let mut flags = [false];
        let mut selection = Selection::Multi(&mut flags);
        assert!(selection.activate(0), "toggling a held flag changes it");
        assert!(
            !selection.activate(3),
            "a segment past the slice changes nothing"
        );
        assert!(selection.is_selected(0), "flag 0 is now set");
        assert!(!selection.is_selected(3), "flag 3 does not exist");
    }

    #[test]
    fn single_selection_reports_change_only_on_a_new_index() {
        let mut selected = 1;
        let mut selection = Selection::Single(&mut selected);
        assert!(!selection.activate(1), "re-selecting is not a change");
        assert!(selection.activate(2), "selecting another is a change");
    }
}
