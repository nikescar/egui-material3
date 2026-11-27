// ============================================================================
// Module:       time_picker::parts
// Description:  The hour and minute selectors, the AM/PM period selector
//               and the time input fields.
//
// Dependencies: egui; super; crate::{date_picker::common, paint, tokens}
// ============================================================================

//! The clock display above (or beside) the dial, the period selector, and
//! the text fields of the input mode.

use egui::{
    CornerRadius, Frame, Id, Margin, Pos2, Rect, Role, Sense, Stroke, StrokeKind, TextEdit, Ui,
    Vec2,
};

use super::tokens as t;
use super::{Selection, State, Time, TimeFormat, TimePickerStrings};
use crate::date_picker::common;
use crate::tokens::InteractionState;
use crate::MaterialTheme;

/// The hour or minute as shown: two digits, 1–12 for 12-hour hours.
pub(super) fn display_value(time: Time, selection: Selection, format: TimeFormat) -> String {
    match (selection, format) {
        (Selection::Minute, TimeFormat::H12 | TimeFormat::H24) => format!("{:02}", time.minute()),
        (Selection::Hour, TimeFormat::H24) => format!("{:02}", time.hour()),
        (Selection::Hour, TimeFormat::H12) => {
            let h = time.hour() % 12;
            format!("{:02}", if h == 0 { 12 } else { h })
        }
    }
}

/// One time selector box (hour or minute). Returns true when clicked.
#[expect(
    clippy::too_many_arguments,
    reason = "a private helper; its callers pass the picker's pieces through"
)]
pub(super) fn time_selector(
    ui: &mut Ui,
    id: Id,
    which: Selection,
    state: &State,
    time: Time,
    format: TimeFormat,
    width: f32,
    strings: &TimePickerStrings,
) -> bool {
    let theme = MaterialTheme::of(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, t::SELECTOR_HEIGHT), Sense::hover());
    let response = ui.interact(rect, id, Sense::click());
    let selected = state.selection == which;
    let text = display_value(time, which, format);
    let name = match which {
        Selection::Hour => &strings.hour,
        Selection::Minute => &strings.minute,
    };
    crate::a11y::toggle(
        &response,
        Role::RadioButton,
        true,
        &format!("{name} {text}"),
        selected,
    );
    let radius = common::radius(&theme, t::SELECTOR_SHAPE, rect);
    let (container, content) = if selected {
        (
            theme.color(t::SELECTOR_SELECTED_CONTAINER_COLOR),
            theme.color(t::SELECTOR_SELECTED_LABEL_COLOR),
        )
    } else {
        (
            theme.color(t::SELECTOR_UNSELECTED_CONTAINER_COLOR),
            theme.color(t::SELECTOR_UNSELECTED_LABEL_COLOR),
        )
    };
    let painter = ui.painter();
    painter.rect_filled(rect, radius, container);
    crate::paint::paint_state_layer(
        painter,
        rect,
        radius,
        content,
        InteractionState::from_response(&response),
    );
    let g = common::galley(ui, t::SELECTOR_LABEL_FONT, &text, content);
    common::paint_centered(ui.painter(), g, rect.center());
    crate::paint::paint_focus_ring_if_focused(ui.painter(), &response, radius);
    response.clicked()
}

/// The `:` between the hour and the minute.
pub(super) fn separator(ui: &mut Ui, height: f32) {
    let theme = MaterialTheme::of(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(t::SEPARATOR_WIDTH, height), Sense::hover());
    let g = common::galley(ui, t::SEPARATOR_FONT, ":", theme.color(t::SEPARATOR_COLOR));
    common::paint_centered(ui.painter(), g, rect.center());
}

/// The AM/PM selector, stacked (`vertical`) or side by side. Returns the
/// new time when a period was picked.
pub(super) fn period_selector(
    ui: &mut Ui,
    id: Id,
    time: Time,
    size: Vec2,
    vertical: bool,
    strings: &TimePickerStrings,
) -> Option<Time> {
    let theme = MaterialTheme::of(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let radius = common::radius(&theme, t::PERIOD_SHAPE, rect);
    let (first, second) = if vertical {
        let mid = rect.center().y;
        (
            Rect::from_min_max(rect.min, Pos2::new(rect.max.x, mid)),
            Rect::from_min_max(Pos2::new(rect.min.x, mid), rect.max),
        )
    } else {
        let mid = rect.center().x;
        (
            Rect::from_min_max(rect.min, Pos2::new(mid, rect.max.y)),
            Rect::from_min_max(Pos2::new(mid, rect.min.y), rect.max),
        )
    };
    // The halves' outer corners follow the container; the inner ones are square.
    let (first_radius, second_radius) = if vertical {
        (
            CornerRadius {
                nw: radius.nw,
                ne: radius.ne,
                sw: 0,
                se: 0,
            },
            CornerRadius {
                nw: 0,
                ne: 0,
                sw: radius.sw,
                se: radius.se,
            },
        )
    } else {
        (
            CornerRadius {
                nw: radius.nw,
                sw: radius.sw,
                ne: 0,
                se: 0,
            },
            CornerRadius {
                nw: 0,
                sw: 0,
                ne: radius.ne,
                se: radius.se,
            },
        )
    };
    let mut picked = None;
    for (half, half_radius, pm, label) in [
        (first, first_radius, false, &strings.am),
        (second, second_radius, true, &strings.pm),
    ] {
        let response = ui.interact(half, id.with(pm), Sense::click());
        let selected = time.is_pm() == pm;
        crate::a11y::toggle(&response, Role::RadioButton, true, label, selected);
        let content = if selected {
            ui.painter().rect_filled(
                half,
                half_radius,
                theme.color(t::PERIOD_SELECTED_CONTAINER_COLOR),
            );
            theme.color(t::PERIOD_SELECTED_LABEL_COLOR)
        } else {
            theme.color(t::PERIOD_UNSELECTED_LABEL_COLOR)
        };
        crate::paint::paint_state_layer(
            ui.painter(),
            half,
            half_radius,
            content,
            InteractionState::from_response(&response),
        );
        let g = common::galley(ui, t::PERIOD_LABEL_FONT, label, content);
        common::paint_centered(ui.painter(), g, half.center());
        crate::paint::paint_focus_ring_if_focused(ui.painter(), &response, half_radius);
        if response.clicked() && !selected {
            picked = Some(time.with_pm(pm));
        }
    }
    let outline = Stroke::new(
        t::PERIOD_OUTLINE_WIDTH,
        theme.color(t::PERIOD_OUTLINE_COLOR),
    );
    ui.painter()
        .rect_stroke(rect, radius, outline, StrokeKind::Inside);
    let divider = if vertical {
        [first.left_bottom(), first.right_bottom()]
    } else {
        [first.right_top(), first.right_bottom()]
    };
    ui.painter().line_segment(divider, outline);
    picked
}

/// The hour or minute input field with its supporting label. Returns the
/// value typed when it is valid and changed this frame.
pub(super) fn time_field(
    ui: &mut Ui,
    id: Id,
    text: &mut String,
    which: Selection,
    format: TimeFormat,
    strings: &TimePickerStrings,
) -> Option<u8> {
    let theme = MaterialTheme::of(ui.ctx());
    let support = t::FIELD_SUPPORTING_FONT.style().line_height + t::SUPPORT_LABEL_TOP;
    let (outer, _) = ui.allocate_exact_size(
        Vec2::new(t::FIELD_WIDTH, t::FIELD_HEIGHT + support),
        Sense::hover(),
    );
    let rect = Rect::from_min_size(outer.min, Vec2::new(t::FIELD_WIDTH, t::FIELD_HEIGHT));
    let radius = common::radius(&theme, t::FIELD_SHAPE, rect);
    let background = ui.painter().add(egui::Shape::Noop);

    let style = t::FIELD_LABEL_FONT.style();
    let edit_rect = Rect::from_center_size(
        rect.center(),
        Vec2::new(rect.width() - 8.0, style.line_height),
    );
    let focused_before = ui.memory(|m| m.has_focus(id));
    let color = if focused_before {
        theme.color(t::FIELD_FOCUS_LABEL_COLOR)
    } else {
        theme.color(t::FIELD_LABEL_COLOR)
    };
    let response = ui.put(
        edit_rect,
        TextEdit::singleline(text)
            .id(id)
            .frame(Frame::NONE)
            .margin(Margin::ZERO)
            .font(style.font_id(ui.ctx()))
            .text_color(color)
            .char_limit(2)
            .align(egui::Align2::CENTER_CENTER)
            .desired_width(edit_rect.width()),
    );
    if response.changed() {
        text.retain(|c| c.is_ascii_digit());
    }
    let (name, max, min) = match (which, format) {
        (Selection::Hour, TimeFormat::H12) => (&strings.hour, 12u8, 1u8),
        (Selection::Hour, TimeFormat::H24) => (&strings.hour, 23, 0),
        (Selection::Minute, TimeFormat::H12 | TimeFormat::H24) => (&strings.minute, 59, 0),
    };
    let parsed = text.parse::<u8>().ok().filter(|v| (min..=max).contains(v));
    let invalid = parsed.is_none() && !text.is_empty();
    response.ctx.accesskit_node_builder(response.id, |node| {
        node.set_label(name.clone());
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        }
    });

    let focused = response.has_focus();
    let fill = if focused {
        theme.color(t::FIELD_FOCUS_CONTAINER_COLOR)
    } else {
        theme.color(t::FIELD_CONTAINER_COLOR)
    };
    ui.painter()
        .set(background, egui::Shape::rect_filled(rect, radius, fill));
    if invalid {
        ui.painter().rect_stroke(
            rect,
            radius,
            Stroke::new(
                t::FIELD_FOCUS_OUTLINE_WIDTH,
                theme.color(crate::tokens::ColorRole::Error),
            ),
            StrokeKind::Inside,
        );
    } else if focused {
        ui.painter().rect_stroke(
            rect,
            radius,
            Stroke::new(
                t::FIELD_FOCUS_OUTLINE_WIDTH,
                theme.color(t::FIELD_FOCUS_OUTLINE_COLOR),
            ),
            StrokeKind::Inside,
        );
    }
    let g = common::galley(
        ui,
        t::FIELD_SUPPORTING_FONT,
        name,
        theme.color(t::FIELD_SUPPORTING_COLOR),
    );
    common::paint_left(
        ui.painter(),
        g,
        rect.min.x,
        rect.max.y + t::SUPPORT_LABEL_TOP + support / 2.0 - t::SUPPORT_LABEL_TOP / 2.0,
    );
    parsed.filter(|_| response.changed())
}
