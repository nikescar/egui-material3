// ============================================================================
// Module:       time_picker::dial
// Description:  The clock dial: angle and ring math, drag and keyboard
//               input, and painting.
//
// Dependencies: egui; super; crate::{date_picker::common, paint, tokens}
// ============================================================================

//! The clock dial (Compose's `ClockFace`): twelve labels on the outer ring
//! (and, for 24-hour hours, 12–23 on the inner ring), the selector handle
//! on a track from the centre. A press or drag picks the value under the
//! pointer; releasing after picking an hour moves on to minutes.

use std::f32::consts::TAU;

use egui::{EventFilter, Id, Key, Modifiers, Pos2, Sense, Stroke, Ui, Vec2};

use super::tokens as t;
use super::{Selection, State, Time, TimeFormat, TimePickerStrings};
use crate::date_picker::common;
use crate::MaterialTheme;

/// The clockwise angle from 12 o'clock of `offset` (y down), 0..TAU.
fn angle_of(offset: Vec2) -> f32 {
    offset.x.atan2(-offset.y).rem_euclid(TAU)
}

/// The position on `ring` steps around the dial: step `i` of `steps`, at
/// `radius` from the centre (y down).
fn position(i: u8, steps: u8, radius: f32) -> Vec2 {
    let angle = f32::from(i) / f32::from(steps.max(1)) * TAU;
    Vec2::new(angle.sin(), -angle.cos()) * radius
}

/// The nearest of `steps` positions to `offset`.
fn step_of(offset: Vec2, steps: u8) -> u8 {
    let steps_f = f32::from(steps.max(1));
    let step = (angle_of(offset) / TAU * steps_f).round() as u32;
    (step % u32::from(steps.max(1))) as u8
}

/// The value the pointer at `offset` from the centre picks: an hour
/// (0..24) or a minute (0..60). In 12-hour mode the hour keeps `time`'s
/// period; in 24-hour mode the inner ring holds 12–23.
pub(super) fn value_at(offset: Vec2, selection: Selection, format: TimeFormat, time: Time) -> u8 {
    match selection {
        Selection::Minute => step_of(offset, 60),
        Selection::Hour => {
            let step = step_of(offset, 12);
            match format {
                TimeFormat::H12 => step + if time.is_pm() { 12 } else { 0 },
                TimeFormat::H24 => {
                    let inner = offset.length() < (t::OUTER_RADIUS + t::INNER_RADIUS) / 2.0;
                    if inner {
                        step + 12
                    } else {
                        step
                    }
                }
            }
        }
    }
}

/// Where the handle sits for `time`, as an offset from the centre.
pub(super) fn handle_offset(selection: Selection, format: TimeFormat, time: Time) -> Vec2 {
    match selection {
        Selection::Minute => position(time.minute(), 60, t::OUTER_RADIUS),
        Selection::Hour => {
            let radius = match format {
                TimeFormat::H24 if time.hour() >= 12 => t::INNER_RADIUS,
                TimeFormat::H12 | TimeFormat::H24 => t::OUTER_RADIUS,
            };
            position(time.hour() % 12, 12, radius)
        }
    }
}

/// Draws the dial and handles its input. Returns whether `time` changed.
pub(super) fn dial(
    ui: &mut Ui,
    id: Id,
    state: &mut State,
    time: &mut Time,
    format: TimeFormat,
    strings: &TimePickerStrings,
) -> bool {
    let theme = MaterialTheme::of(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(t::DIAL_SIZE), Sense::hover());
    let response = ui.interact(rect, id, Sense::click_and_drag());
    let center = rect.center();
    let selection = state.selection;
    let before = *time;

    let (label, value, max) = match selection {
        Selection::Hour => (&strings.hour, time.hour(), 23u8),
        Selection::Minute => (&strings.minute, time.minute(), 59u8),
    };
    crate::a11y::slider(&response, true, label, f64::from(value));

    let pressing = response.dragged() || response.clicked() || response.is_pointer_button_down_on();
    if let (true, Some(pointer)) = (pressing, response.interact_pointer_pos()) {
        let v = value_at(pointer - center, selection, format, *time);
        *time = time.with(selection, v);
    }
    if (response.drag_stopped() || response.clicked()) && selection == Selection::Hour {
        state.selection = Selection::Minute;
    }

    // Keyboard: arrows step the value while the dial has focus.
    if response.has_focus() {
        ui.memory_mut(|m| {
            m.set_focus_lock_filter(
                id,
                EventFilter {
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    tab: false,
                    escape: false,
                },
            );
        });
        let up = ui.input_mut(|i| {
            i.consume_key(Modifiers::NONE, Key::ArrowUp)
                || i.consume_key(Modifiers::NONE, Key::ArrowRight)
        });
        let down = ui.input_mut(|i| {
            i.consume_key(Modifiers::NONE, Key::ArrowDown)
                || i.consume_key(Modifiers::NONE, Key::ArrowLeft)
        });
        let step = |v: u8, delta: i16| -> u8 {
            let span = i16::from(max) + 1;
            (i16::from(v) + delta).rem_euclid(span).clamp(0, 255) as u8
        };
        if up {
            *time = time.with(selection, step(value, 1));
        } else if down {
            *time = time.with(selection, step(value, -1));
        }
    }

    if ui.is_rect_visible(rect) {
        paint(ui, rect.center(), selection, format, *time, &theme);
        crate::paint::paint_focus_ring_if_focused(
            ui.painter(),
            &response,
            egui::CornerRadius::same(128),
        );
    }
    *time != before
}

fn paint(
    ui: &Ui,
    center: Pos2,
    selection: Selection,
    format: TimeFormat,
    time: Time,
    theme: &MaterialTheme,
) {
    let painter = ui.painter();
    painter.circle_filled(center, t::DIAL_SIZE / 2.0, theme.color(t::DIAL_COLOR));

    // (text, value, radius) for every label on show.
    let mut labels: Vec<(String, u8, f32)> = Vec::new();
    match selection {
        Selection::Minute => {
            for i in 0..12u8 {
                labels.push((format!("{:02}", i * 5), i * 5, t::OUTER_RADIUS));
            }
        }
        Selection::Hour => match format {
            TimeFormat::H12 => {
                for i in 0..12u8 {
                    let shown = if i == 0 { 12 } else { i };
                    let value = i + if time.is_pm() { 12 } else { 0 };
                    labels.push((shown.to_string(), value, t::OUTER_RADIUS));
                }
            }
            TimeFormat::H24 => {
                for i in 0..12u8 {
                    let outer = if i == 0 {
                        "00".to_owned()
                    } else {
                        i.to_string()
                    };
                    labels.push((outer, i, t::OUTER_RADIUS));
                    labels.push(((i + 12).to_string(), i + 12, t::INNER_RADIUS));
                }
            }
        },
    }
    let current = match selection {
        Selection::Hour => time.hour(),
        Selection::Minute => time.minute(),
    };
    let step_of_label = |value: u8, steps: u8| -> u8 { value % steps };
    let unselected = theme.color(t::DIAL_UNSELECTED_LABEL_COLOR);
    for (text, value, radius) in &labels {
        let steps = match selection {
            Selection::Hour => 12,
            Selection::Minute => 60,
        };
        let pos = center + position(step_of_label(*value, steps), steps, *radius);
        let g = common::galley(ui, t::DIAL_LABEL_FONT, text, unselected);
        common::paint_centered(painter, g, pos);
    }

    let handle = center + handle_offset(selection, format, time);
    let primary = theme.color(t::SELECTOR_TRACK_COLOR);
    painter.line_segment(
        [center, handle],
        Stroke::new(t::SELECTOR_TRACK_WIDTH, primary),
    );
    painter.circle_filled(
        center,
        t::SELECTOR_CENTER_SIZE / 2.0,
        theme.color(t::SELECTOR_CENTER_COLOR),
    );
    painter.circle_filled(
        handle,
        t::SELECTOR_HANDLE_SIZE / 2.0,
        theme.color(t::SELECTOR_HANDLE_COLOR),
    );
    // The label under the handle, redrawn on it.
    let selected = theme.color(t::DIAL_SELECTED_LABEL_COLOR);
    match labels.iter().find(|(_, value, _)| *value == current) {
        Some((text, _, _)) => {
            let g = common::galley(ui, t::DIAL_LABEL_FONT, text, selected);
            common::paint_centered(painter, g, handle);
        }
        // Between labels (minutes not on a multiple of five): a dot.
        None => {
            painter.circle_filled(handle, 2.0, selected);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(h: u8, m: u8) -> Result<Time, String> {
        Time::new(h, m).ok_or_else(|| format!("{h}:{m} should be valid"))
    }

    #[test]
    fn angles_run_clockwise_from_twelve() {
        assert!(angle_of(Vec2::new(0.0, -1.0)).abs() < 1e-5, "12 o'clock");
        assert!(
            (angle_of(Vec2::new(1.0, 0.0)) - TAU / 4.0).abs() < 1e-5,
            "3 o'clock"
        );
        assert!(
            (angle_of(Vec2::new(-1.0, 0.0)) - 3.0 * TAU / 4.0).abs() < 1e-5,
            "9 o'clock"
        );
    }

    #[test]
    fn hours_and_rings() -> Result<(), String> {
        let am = time(9, 0)?;
        let pm = time(21, 0)?;
        let three = Vec2::new(t::OUTER_RADIUS, 0.0);
        assert_eq!(
            value_at(three, Selection::Hour, TimeFormat::H12, am),
            3,
            "3 am"
        );
        assert_eq!(
            value_at(three, Selection::Hour, TimeFormat::H12, pm),
            15,
            "3 pm"
        );
        let top = Vec2::new(0.0, -t::OUTER_RADIUS);
        assert_eq!(
            value_at(top, Selection::Hour, TimeFormat::H12, am),
            0,
            "12 am"
        );
        assert_eq!(value_at(top, Selection::Hour, TimeFormat::H24, am), 0, "00");
        let inner_top = Vec2::new(0.0, -t::INNER_RADIUS);
        assert_eq!(
            value_at(inner_top, Selection::Hour, TimeFormat::H24, am),
            12,
            "inner 12"
        );
        let inner_three = Vec2::new(t::INNER_RADIUS, 0.0);
        assert_eq!(
            value_at(inner_three, Selection::Hour, TimeFormat::H24, am),
            15,
            "inner 15"
        );
        Ok(())
    }

    #[test]
    fn minutes_snap_to_the_nearest() -> Result<(), String> {
        let t0 = time(0, 0)?;
        let half = Vec2::new(0.0, t::OUTER_RADIUS);
        assert_eq!(
            value_at(half, Selection::Minute, TimeFormat::H12, t0),
            30,
            "half past"
        );
        let almost = position(59, 60, t::OUTER_RADIUS) * 1.01;
        assert_eq!(
            value_at(almost, Selection::Minute, TimeFormat::H12, t0),
            59,
            "59"
        );
        Ok(())
    }

    #[test]
    fn handle_round_trips_every_value() -> Result<(), String> {
        for format in [TimeFormat::H12, TimeFormat::H24] {
            for hour in 0..24 {
                let tm = time(hour, 0)?;
                let offset = handle_offset(Selection::Hour, format, tm);
                assert_eq!(
                    value_at(offset, Selection::Hour, format, tm),
                    hour,
                    "{format:?} hour {hour}"
                );
            }
        }
        for minute in 0..60 {
            let tm = time(10, minute)?;
            let offset = handle_offset(Selection::Minute, TimeFormat::H24, tm);
            assert_eq!(
                value_at(offset, Selection::Minute, TimeFormat::H24, tm),
                minute,
                "minute {minute}"
            );
        }
        Ok(())
    }
}
