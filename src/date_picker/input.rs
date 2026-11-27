// ============================================================================
// Module:       date_picker::input
// Description:  Input mode: typed dates with validation, single and range,
//               and the docked picker's field.
//
// Dependencies: egui; super
// ============================================================================

//! Input mode (Compose's `DateInput` / `DateRangeInput`): outlined fields
//! that parse the typed text with the picker's [`DateFormat`](super::DateFormat)
//! and check it against the year range, the selectable-dates predicate and,
//! for a range, that the end is not before the start.

use egui::{Color32, Id, Ui, Vec2};

use super::common::{FieldOutput, Glyph, OutlinedField};
use super::tokens as t;
use super::{Config, Date, DateParseError, DateRange, State, Value};

/// The error message for `text`, or the date it names. Empty text is no
/// date and no error.
pub(super) fn validate(config: &Config<'_>, text: &str) -> Result<Option<Date>, String> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let error = match config.format.parse(text) {
        Ok(date) if !config.in_year_range(date) => DateParseError::YearRange,
        Ok(date) if !config.allows(date) => DateParseError::NotAllowed,
        Ok(date) => return Ok(Some(date)),
        Err(error) => error,
    };
    let strings = &config.strings;
    Err(match error {
        DateParseError::Pattern => strings
            .error_pattern
            .replace("{pattern}", &config.format.pattern()),
        DateParseError::YearRange => strings
            .error_year_range
            .replace("{start}", &config.year_range.start().to_string())
            .replace("{end}", &config.year_range.end().to_string()),
        DateParseError::NotAllowed => strings.error_not_allowed.clone(),
    })
}

/// Refills the texts from `value` when it changed outside the fields.
fn sync(config: &Config<'_>, state: &mut State, value: Value) {
    if state.synced == Some(value) {
        return;
    }
    let text = |d: Option<Date>| d.map(|d| config.format.format(d)).unwrap_or_default();
    match value {
        Value::Single(date) => state.start_text = text(date),
        Value::Range(range) => {
            state.start_text = text(range.start);
            state.end_text = text(range.end);
        }
    }
    state.synced = Some(value);
}

/// The input-mode body: one field, or start and end fields side by side.
pub(super) fn body(
    ui: &mut Ui,
    id: Id,
    config: &Config<'_>,
    state: &mut State,
    value: &mut Value,
    width: f32,
    surface: Color32,
) {
    sync(config, state, *value);
    ui.add_space(t::INPUT_FIELD_TOP_PADDING);
    let pattern = config.format.pattern();
    let inner_width = width - 2.0 * t::INPUT_FIELD_PADDING;
    let row = |ui: &mut Ui, add: &mut dyn FnMut(&mut Ui)| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);
            ui.add_space(t::INPUT_FIELD_PADDING);
            add(ui);
        });
    };
    match value {
        Value::Single(date) => {
            let result = validate(config, &state.start_text);
            let error = result.as_ref().err().cloned();
            let mut changed = false;
            row(ui, &mut |ui| {
                let out = OutlinedField {
                    id: id.with("input_single"),
                    text: &mut state.start_text,
                    label: &config.strings.date,
                    placeholder: &pattern,
                    supporting: Some(&pattern),
                    error: error.as_deref(),
                    width: inner_width,
                    surface,
                    trailing: None,
                }
                .show(ui);
                changed = out.response.changed();
            });
            if changed {
                *date = validate(config, &state.start_text).unwrap_or(None);
                state.synced = Some(Value::Single(*date));
            }
        }
        Value::Range(range) => {
            let start = validate(config, &state.start_text);
            let end = validate(config, &state.end_text);
            let start_error = start.as_ref().err().cloned();
            let end_error = match (&start, &end) {
                (_, Err(error)) => Some(error.clone()),
                (Ok(Some(s)), Ok(Some(e))) if e < s => Some(config.strings.error_range.clone()),
                (Ok(_) | Err(_), Ok(_)) => None,
            };
            let field_width = (inner_width - 8.0) / 2.0;
            let mut changed = false;
            row(ui, &mut |ui| {
                let a = OutlinedField {
                    id: id.with("input_start"),
                    text: &mut state.start_text,
                    label: &config.strings.start_date,
                    placeholder: &pattern,
                    supporting: Some(&pattern),
                    error: start_error.as_deref(),
                    width: field_width,
                    surface,
                    trailing: None,
                }
                .show(ui);
                let b = OutlinedField {
                    id: id.with("input_end"),
                    text: &mut state.end_text,
                    label: &config.strings.end_date,
                    placeholder: &pattern,
                    supporting: Some(&pattern),
                    error: end_error.as_deref(),
                    width: field_width,
                    surface,
                    trailing: None,
                }
                .show(ui);
                changed = a.response.changed() || b.response.changed();
            });
            if changed {
                let start = validate(config, &state.start_text).unwrap_or(None);
                let end = validate(config, &state.end_text).unwrap_or(None);
                let end = match (start, end) {
                    (Some(s), Some(e)) if e < s => None,
                    (Some(_), e) => e,
                    (None, _) => None,
                };
                *range = DateRange { start, end };
                state.synced = Some(Value::Range(*range));
            }
        }
    }
    ui.add_space(t::INPUT_FIELD_PADDING);
}

/// The docked picker's field: a single-date input with a calendar button.
#[expect(
    clippy::too_many_arguments,
    reason = "a private helper; its callers pass the picker's pieces through"
)]
pub(super) fn docked_field(
    ui: &mut Ui,
    id: Id,
    config: &Config<'_>,
    state: &mut State,
    value: &mut Value,
    width: f32,
    surface: Color32,
    label: &str,
) -> FieldOutput {
    sync(config, state, *value);
    let pattern = config.format.pattern();
    let result = validate(config, &state.start_text);
    let error = result.as_ref().err().cloned();
    let out = OutlinedField {
        id: id.with("input"),
        text: &mut state.start_text,
        label,
        placeholder: &pattern,
        supporting: Some(&pattern),
        error: error.as_deref(),
        width,
        surface,
        trailing: Some((Glyph::Calendar, &config.strings.open_calendar)),
    }
    .show(ui);
    if out.response.changed() {
        let date = validate(config, &state.start_text).unwrap_or(None);
        *value = Value::Single(date);
        state.synced = Some(*value);
    }
    out
}
