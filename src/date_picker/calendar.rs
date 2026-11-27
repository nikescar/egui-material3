// ============================================================================
// Module:       date_picker::calendar
// Description:  The picker's header, month/year row, month grid with
//               keyboard navigation, and year list.
//
// Dependencies: egui; super; crate::{paint, tokens, MaterialTheme}
// ============================================================================

//! Calendar mode: the header (title, headline, mode toggle), the month and
//! year row, the weekday labels, the 7×6 day grid and the year list.

use egui::{CornerRadius, EventFilter, Id, Key, Modifiers, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use super::common::{self, Glyph};
use super::tokens as t;
use super::{Config, Date, DisplayMode, State, Value};
use crate::tokens::{InteractionState, LayoutDirection};
use crate::MaterialTheme;

/// The title, headline and mode toggle, then a divider.
pub(super) fn header(
    ui: &mut Ui,
    id: Id,
    config: &Config<'_>,
    state: &mut State,
    value: Value,
    mode: DisplayMode,
    width: f32,
) {
    let theme = MaterialTheme::of(ui.ctx());
    let strings = &config.strings;
    let (height, title, headline, headline_font) = match value {
        Value::Single(date) => (
            t::HEADER_HEIGHT,
            match mode {
                DisplayMode::Picker => &strings.title_select_date,
                DisplayMode::Input => &strings.title_enter_date,
            },
            date.map_or_else(|| strings.headline_no_date.clone(), |d| strings.headline(d)),
            t::HEADER_HEADLINE_FONT,
        ),
        Value::Range(range) => {
            let start = range
                .start
                .map_or_else(|| strings.start_date.clone(), |d| strings.headline(d));
            let end = range
                .end
                .map_or_else(|| strings.end_date.clone(), |d| strings.headline(d));
            (
                t::RANGE_HEADER_HEIGHT,
                match mode {
                    DisplayMode::Picker => &strings.title_select_dates,
                    DisplayMode::Input => &strings.title_enter_dates,
                },
                format!("{start} – {end}"),
                t::RANGE_HEADER_HEADLINE_FONT,
            )
        }
    };
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let color = theme.color(t::HEADER_SUPPORTING_TEXT_COLOR);
    let title = common::galley(ui, t::HEADER_SUPPORTING_TEXT_FONT, title, color);
    let title_y = rect.min.y + t::TITLE_TOP_PADDING + title.size().y / 2.0;
    common::paint_left(
        ui.painter(),
        title,
        rect.min.x + t::HEADER_START_PADDING,
        title_y,
    );
    let headline_color = theme.color(t::HEADER_HEADLINE_COLOR);
    let headline_galley = common::galley(ui, headline_font, &headline, headline_color);
    let headline_y = rect.max.y - t::HEADLINE_BOTTOM_PADDING - headline_galley.size().y / 2.0;
    // The headline is announced as a label.
    let headline_rect = Rect::from_min_size(
        Pos2::new(
            rect.min.x + t::HEADER_START_PADDING,
            headline_y - headline_galley.size().y / 2.0,
        ),
        headline_galley.size(),
    );
    let headline_response = ui.interact(headline_rect, id.with("headline"), Sense::hover());
    crate::a11y::describe(&headline_response, egui::Role::Label, true, &headline, None);
    common::paint_left(
        ui.painter(),
        headline_galley,
        rect.min.x + t::HEADER_START_PADDING,
        headline_y,
    );

    if config.show_mode_toggle {
        let (glyph, label, next) = match mode {
            DisplayMode::Picker => (Glyph::Edit, &strings.switch_to_input, DisplayMode::Input),
            DisplayMode::Input => (
                Glyph::Calendar,
                &strings.switch_to_calendar,
                DisplayMode::Picker,
            ),
        };
        let button = Rect::from_min_size(
            Pos2::new(
                rect.max.x - t::HEADER_END_PADDING - t::ICON_BUTTON_SIZE,
                rect.max.y - t::HEADLINE_BOTTOM_PADDING / 2.0 - t::ICON_BUTTON_SIZE,
            ),
            Vec2::splat(t::ICON_BUTTON_SIZE),
        );
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(button));
        let toggle = common::icon_button(
            &mut child,
            id.with("mode_toggle"),
            glyph,
            label,
            true,
            theme.color(t::HEADER_SUPPORTING_TEXT_COLOR),
        );
        if toggle.clicked() {
            state.mode = Some(next);
            state.synced = None;
            state.years_open = false;
        }
    }

    let (divider, _) = ui.allocate_exact_size(Vec2::new(width, 1.0), Sense::hover());
    ui.painter()
        .rect_filled(divider, 0.0, theme.color(t::DIVIDER_COLOR));
}

/// The month/year row and either the month grid or the year list.
pub(super) fn body(
    ui: &mut Ui,
    id: Id,
    config: &Config<'_>,
    state: &mut State,
    value: &mut Value,
    width: f32,
) {
    month_year_row(ui, id, config, state, width);
    if state.years_open {
        year_list(ui, id, config, state, width);
    } else {
        weekdays(ui, config, width);
        month_grid(ui, id, config, state, value, width);
    }
}

fn displayed(config: &Config<'_>, state: &State) -> Date {
    state
        .displayed
        .unwrap_or_else(|| config.clamp_month(config.today()))
}

fn month_year_row(ui: &mut Ui, id: Id, config: &Config<'_>, state: &mut State, width: f32) {
    let theme = MaterialTheme::of(ui.ctx());
    let shown = displayed(config, state);
    let color = theme.color(t::NAVIGATION_CONTENT_COLOR);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, t::MONTH_YEAR_HEIGHT), Sense::hover());
    let inner = rect.shrink2(Vec2::new(t::HORIZONTAL_PADDING, 0.0));

    // The month/year menu button: label and a drop arrow.
    let text = format!("{} {}", config.strings.month(shown.month()), shown.year());
    let label = common::galley(ui, t::MONTH_YEAR_FONT, &text, color);
    let button_width = 12.0 + label.size().x + 4.0 + t::ICON_SIZE + 8.0;
    let button = Rect::from_center_size(
        Pos2::new(inner.min.x + button_width / 2.0, inner.center().y),
        Vec2::new(button_width, 40.0),
    );
    let response = ui.interact(button, id.with("year_toggle"), Sense::click());
    let announce = if state.years_open {
        &config.strings.switch_to_day
    } else {
        &config.strings.switch_to_year
    };
    crate::a11y::describe(
        &response,
        egui::Role::Button,
        true,
        &format!("{text}, {announce}"),
        Some(state.years_open),
    );
    let painter = ui.painter();
    let pill = CornerRadius::same(20);
    crate::paint::paint_state_layer(
        painter,
        button,
        pill,
        color,
        InteractionState::from_response(&response),
    );
    common::paint_left(painter, label, button.min.x + 12.0, button.center().y);
    let glyph = if state.years_open {
        Glyph::DropUp
    } else {
        Glyph::DropDown
    };
    common::paint_glyph(
        painter,
        glyph,
        Pos2::new(button.max.x - 8.0 - t::ICON_SIZE / 2.0, button.center().y),
        color,
    );
    crate::paint::paint_focus_ring_if_focused(painter, &response, pill);
    if response.clicked() {
        state.years_open = !state.years_open;
        state.scroll_to_year = state.years_open;
    }

    // Previous / next month, hidden while the year list shows.
    if !state.years_open {
        let size = Vec2::splat(t::ICON_BUTTON_SIZE);
        let next_rect = Rect::from_min_size(
            Pos2::new(inner.max.x - size.x, inner.center().y - size.y / 2.0),
            size,
        );
        let prev_rect = next_rect.translate(Vec2::new(-size.x, 0.0));
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let (prev_glyph, next_glyph) = if rtl {
            (Glyph::ChevronRight, Glyph::ChevronLeft)
        } else {
            (Glyph::ChevronLeft, Glyph::ChevronRight)
        };
        let can_prev = config.first_month().is_none_or(|first| shown > first);
        let can_next = config.last_month().is_none_or(|last| shown < last);
        let mut prev_ui = ui.new_child(egui::UiBuilder::new().max_rect(prev_rect));
        let prev = common::icon_button(
            &mut prev_ui,
            id.with("prev_month"),
            prev_glyph,
            &config.strings.previous_month,
            can_prev,
            color,
        );
        let mut next_ui = ui.new_child(egui::UiBuilder::new().max_rect(next_rect));
        let next = common::icon_button(
            &mut next_ui,
            id.with("next_month"),
            next_glyph,
            &config.strings.next_month,
            can_next,
            color,
        );
        if prev.clicked() {
            state.displayed = Some(config.clamp_month(shown.add_months(-1)));
        }
        if next.clicked() {
            state.displayed = Some(config.clamp_month(shown.add_months(1)));
        }
    }
}

fn weekdays(ui: &mut Ui, config: &Config<'_>, width: f32) {
    let theme = MaterialTheme::of(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, t::CELL_SIZE), Sense::hover());
    let color = theme.color(t::WEEKDAYS_LABEL_COLOR);
    for column in 0..7u8 {
        let day = config.first_day_of_week.add(column);
        let center = cell_center(&theme, rect, column, 0);
        let g = common::galley(
            ui,
            t::WEEKDAYS_LABEL_FONT,
            config.strings.weekday_narrow(day),
            color,
        );
        common::paint_centered(ui.painter(), g, center);
    }
}

/// The centre of the cell at `column`, `row` of a grid whose top-left
/// starts at `rect.min` plus the horizontal padding, mirrored in RTL.
fn cell_center(theme: &MaterialTheme, rect: Rect, column: u8, row: u8) -> Pos2 {
    let column = match theme.layout_direction {
        LayoutDirection::Ltr => column,
        LayoutDirection::Rtl => 6 - column.min(6),
    };
    Pos2::new(
        rect.min.x + t::HORIZONTAL_PADDING + (f32::from(column) + 0.5) * t::CELL_SIZE,
        rect.min.y + (f32::from(row) + 0.5) * t::CELL_SIZE,
    )
}

/// The id of a day's cell.
pub(super) fn day_id(grid: Id, date: Date) -> Id {
    grid.with(date.days_since_epoch())
}

/// Reads one grid-navigation key for the focused day, if any, and returns
/// where focus should go.
fn navigation_target(ui: &Ui, config: &Config<'_>, focused: Date) -> Option<Date> {
    let theme = MaterialTheme::of(ui.ctx());
    let forward = match theme.layout_direction {
        LayoutDirection::Ltr => 1,
        LayoutDirection::Rtl => -1,
    };
    let column = i64::from(focused.weekday().days_since(config.first_day_of_week));
    let steps: [(Key, i64); 6] = [
        (Key::ArrowRight, forward),
        (Key::ArrowLeft, -forward),
        (Key::ArrowDown, 7),
        (Key::ArrowUp, -7),
        (Key::Home, -column),
        (Key::End, 6 - column),
    ];
    for (key, days) in steps {
        if ui.input_mut(|i| i.consume_key(Modifiers::NONE, key)) {
            return Some(focused.add_days(days));
        }
    }
    if ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::PageDown)) {
        return Some(focused.add_months(1));
    }
    if ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::PageUp)) {
        return Some(focused.add_months(-1));
    }
    None
}

fn month_grid(
    ui: &mut Ui,
    id: Id,
    config: &Config<'_>,
    state: &mut State,
    value: &mut Value,
    width: f32,
) {
    let theme = MaterialTheme::of(ui.ctx());
    let grid = id.with("grid");

    // Keyboard first, so the month can change before the cells are drawn.
    let focused = state
        .focused
        .filter(|&f| ui.memory(|m| m.has_focus(day_id(grid, f))));
    if let Some(target) = focused.and_then(|f| navigation_target(ui, config, f)) {
        if config.in_year_range(target) {
            state.displayed = Some(target.first_of_month());
            state.focus_request = Some(target);
            ui.ctx().request_repaint();
        }
    }
    state.focused = None;

    let shown = displayed(config, state);
    let today = config.today();
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(width, t::CELL_SIZE * t::MAX_CALENDAR_ROWS as f32),
        Sense::hover(),
    );
    let lead = shown.weekday().days_since(config.first_day_of_week);
    let days = super::days_in_month(shown.year(), shown.month());
    let range = match *value {
        Value::Range(range) => Some(range),
        Value::Single(_) => None,
    };
    let mut clicked = None;
    for day in 1..=days {
        let Some(date) = Date::new(shown.year(), shown.month(), day) else {
            continue;
        };
        let index = lead + day - 1;
        let center = cell_center(&theme, rect, index % 7, index / 7);
        let cell = Rect::from_center_size(center, Vec2::splat(t::CELL_SIZE));
        let circle = Rect::from_center_size(center, Vec2::splat(t::DATE_CONTAINER_SIZE));
        let enabled = config.allows(date);
        let is_today = date == today;
        let (selected, in_range, band) = match (*value, range) {
            (Value::Single(selection), _) => (selection == Some(date), false, None),
            (Value::Range(_), Some(r)) => {
                let endpoint = r.start == Some(date) || r.end == Some(date);
                let band = r.get().and_then(|(start, end)| {
                    if start == end || date < start || date > end {
                        return None;
                    }
                    let (left_half, right_half) = (
                        Rect::from_min_max(
                            Pos2::new(cell.min.x, circle.min.y),
                            Pos2::new(center.x, circle.max.y),
                        ),
                        Rect::from_min_max(
                            Pos2::new(center.x, circle.min.y),
                            Pos2::new(cell.max.x, circle.max.y),
                        ),
                    );
                    let (before_half, after_half) = match theme.layout_direction {
                        LayoutDirection::Ltr => (left_half, right_half),
                        LayoutDirection::Rtl => (right_half, left_half),
                    };
                    Some(if date == start {
                        after_half
                    } else if date == end {
                        before_half
                    } else {
                        Rect::from_min_max(
                            Pos2::new(cell.min.x, circle.min.y),
                            Pos2::new(cell.max.x, circle.max.y),
                        )
                    })
                });
                (endpoint, r.contains(date) && !endpoint, band)
            }
            (Value::Range(_), None) => (false, false, None),
        };

        let mut label = config.strings.spoken(date);
        if is_today {
            label = format!("{label}, {}", config.strings.today);
        }
        let cell_id = day_id(grid, date);
        let response = common::selectable_cell(ui, cell, cell_id, enabled, &label, selected);
        if state.focus_request == Some(date) {
            response.request_focus();
            state.focus_request = None;
        }
        if response.has_focus() {
            state.focused = Some(date);
            ui.memory_mut(|m| {
                m.set_focus_lock_filter(
                    cell_id,
                    EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        tab: false,
                        escape: false,
                    },
                );
            });
        }
        if response.clicked() {
            clicked = Some(date);
        }

        if !ui.is_rect_visible(cell) {
            continue;
        }
        let painter = ui.painter();
        if let Some(band) = band {
            painter.rect_filled(band, 0.0, theme.color(t::RANGE_ACTIVE_INDICATOR_COLOR));
        }
        let round = CornerRadius::same(20);
        let content = if selected {
            painter.rect_filled(circle, round, theme.color(t::DATE_SELECTED_CONTAINER_COLOR));
            theme.color(t::DATE_SELECTED_LABEL_COLOR)
        } else if in_range {
            theme.color(t::DATE_IN_RANGE_LABEL_COLOR)
        } else if is_today {
            painter.rect_stroke(
                circle,
                round,
                Stroke::new(
                    t::DATE_TODAY_OUTLINE_WIDTH,
                    theme.color(t::DATE_TODAY_OUTLINE_COLOR),
                ),
                egui::StrokeKind::Inside,
            );
            theme.color(t::DATE_TODAY_LABEL_COLOR)
        } else {
            theme.color(t::DATE_UNSELECTED_LABEL_COLOR)
        };
        let content = if enabled {
            content
        } else {
            content.gamma_multiply(crate::tokens::state::DISABLED_CONTENT_OPACITY)
        };
        crate::paint::paint_state_layer(
            painter,
            circle,
            round,
            content,
            InteractionState::from_response(&response),
        );
        let g = common::galley(ui, t::DATE_LABEL_FONT, &day.to_string(), content);
        common::paint_centered(ui.painter(), g, center);
        crate::paint::paint_focus_ring_if_focused(ui.painter(), &response, round);
    }

    if let Some(date) = clicked {
        match value {
            Value::Single(selection) => *selection = Some(date),
            Value::Range(range) => range.select(date),
        }
        state.focused = Some(date);
    }
}

fn year_list(ui: &mut Ui, id: Id, config: &Config<'_>, state: &mut State, width: f32) {
    let theme = MaterialTheme::of(ui.ctx());
    let shown = displayed(config, state);
    let today = config.today();
    let height = t::CELL_SIZE * (t::MAX_CALENDAR_ROWS as f32 + 1.0);
    let column_width = (width - 2.0 * t::HORIZONTAL_PADDING) / t::YEARS_IN_ROW as f32;
    let row_height = t::YEAR_CONTAINER_HEIGHT + t::YEARS_VERTICAL_PADDING;
    let years: Vec<i32> = config.year_range.clone().collect();
    let rows = years.len().div_ceil(t::YEARS_IN_ROW);
    let (outer, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(outer));
    let mut picked = None;
    // The selected year may lie outside show_rows' visible window, so the
    // first frame scrolls to it by offset.
    let mut scroll = egui::ScrollArea::vertical();
    let selected_index = years.iter().position(|&y| y == shown.year());
    if let (true, Some(index)) = (state.scroll_to_year, selected_index) {
        let row = index / t::YEARS_IN_ROW;
        let offset = (row as f32 * row_height - height / 2.0 + row_height / 2.0).max(0.0);
        scroll = scroll.vertical_scroll_offset(offset);
        state.scroll_to_year = false;
    }
    scroll
        .id_salt(id.with("years_scroll"))
        .max_height(height)
        .auto_shrink([false, false])
        .show_rows(&mut child, row_height, rows, |ui, visible| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            for row in visible {
                let (row_rect, _) =
                    ui.allocate_exact_size(Vec2::new(width, row_height), Sense::hover());
                for column in 0..t::YEARS_IN_ROW {
                    let Some(&year) = years.get(row * t::YEARS_IN_ROW + column) else {
                        continue;
                    };
                    let column = match theme.layout_direction {
                        LayoutDirection::Ltr => column,
                        LayoutDirection::Rtl => t::YEARS_IN_ROW - 1 - column,
                    };
                    let center = Pos2::new(
                        row_rect.min.x
                            + t::HORIZONTAL_PADDING
                            + (column as f32 + 0.5) * column_width,
                        row_rect.center().y,
                    );
                    let pill = Rect::from_center_size(
                        center,
                        Vec2::new(t::YEAR_CONTAINER_WIDTH, t::YEAR_CONTAINER_HEIGHT),
                    );
                    let selected = year == shown.year();
                    let response = common::selectable_cell(
                        ui,
                        pill,
                        id.with(("year", year)),
                        true,
                        &year.to_string(),
                        selected,
                    );
                    if response.clicked() {
                        picked = Some(year);
                    }
                    let painter = ui.painter();
                    let round = CornerRadius::same(18);
                    let content = if selected {
                        painter.rect_filled(
                            pill,
                            round,
                            theme.color(t::YEAR_SELECTED_CONTAINER_COLOR),
                        );
                        theme.color(t::YEAR_SELECTED_LABEL_COLOR)
                    } else if year == today.year() {
                        painter.rect_stroke(
                            pill,
                            round,
                            Stroke::new(
                                t::DATE_TODAY_OUTLINE_WIDTH,
                                theme.color(t::DATE_TODAY_OUTLINE_COLOR),
                            ),
                            egui::StrokeKind::Inside,
                        );
                        theme.color(t::DATE_TODAY_LABEL_COLOR)
                    } else {
                        theme.color(t::YEAR_UNSELECTED_LABEL_COLOR)
                    };
                    crate::paint::paint_state_layer(
                        painter,
                        pill,
                        round,
                        content,
                        InteractionState::from_response(&response),
                    );
                    let g = common::galley(ui, t::YEAR_LABEL_FONT, &year.to_string(), content);
                    common::paint_centered(ui.painter(), g, center);
                    crate::paint::paint_focus_ring_if_focused(ui.painter(), &response, round);
                }
            }
        });
    if let Some(year) = picked {
        let month = shown.month();
        if let Some(date) = Date::new(year, month, 1) {
            state.displayed = Some(config.clamp_month(date));
        }
        state.years_open = false;
    }
}
