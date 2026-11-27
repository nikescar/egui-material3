// ============================================================================
// Module:       date_picker
// Description:  Material 3 date pickers: docked, modal and modal input,
//               single date and date range.
//
// Dependencies: egui; crate::{tokens, paint, a11y, MaterialTheme}
// ============================================================================

//! Material 3 date pickers.
//!
//! - [`DatePicker`]: the picker itself, as a widget you can place anywhere
//!   (Compose's `DatePicker` / `DateRangePicker`). It has a calendar mode
//!   (month grid with a year list) and an input mode (typed dates with
//!   validation), and a toggle between them.
//! - [`DatePickerDialog`]: the modal date picker. Edits a copy and writes
//!   it back on confirm. Start it in [`DisplayMode::Input`] for the modal
//!   input variant.
//! - [`DockedDatePicker`]: an outlined date field with a calendar that drops
//!   down below it.
//!
//! Every variant picks a single date (`Option<Date>`) or a range
//! ([`DateRange`], whose [`DateRange::get`] returns `(start, end)`).
//!
//! Locale-agnostic: dates are proleptic Gregorian, the strings are English
//! defaults you can replace through [`DatePickerStrings`], the typed format
//! is a [`DateFormat`], and the first day of the week is a parameter.
//!
//! Keyboard: Tab into the month grid, arrow keys move the day focus (across
//! months), Home/End go to the start/end of the week, Page Up/Down change
//! the month, Enter or Space selects. Every day, year and button reports a
//! role, a label and its selected state to AccessKit.
//!
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::date_picker::{Date, DatePicker, DateRange};
//! let mut date: Option<Date> = Date::new(2024, 3, 5);
//! ui.add(DatePicker::single(&mut date).id_salt("birthday"));
//!
//! let mut stay = DateRange::default();
//! ui.add(DatePicker::range(&mut stay).id_salt("stay"));
//! if let Some((start, end)) = stay.get() {
//!     ui.label(format!("{start} to {end}"));
//! }
//! # });
//! ```

mod calendar;
pub(crate) mod common;
mod date;
mod input;
pub mod tokens;

use std::ops::RangeInclusive;

use egui::{Id, IdSalt, Rect, Response, Sense, Ui, Vec2, Widget};

pub use common::DialogAction;
pub use date::{
    days_in_month, is_leap_year, Date, DateFormat, DateOrder, DateParseError, DateRange, Weekday,
};

use crate::MaterialTheme;
use tokens as t;

/// Whether the picker shows the calendar or the text input.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DisplayMode {
    /// The month grid and year list (Compose `DisplayMode.Picker`).
    #[default]
    Picker,
    /// Typed dates (Compose `DisplayMode.Input`).
    Input,
}

/// The text a date picker shows and announces. English by default; replace
/// any field to localise.
#[derive(Clone, Debug, PartialEq)]
pub struct DatePickerStrings {
    /// Month names, January first.
    pub months: [String; 12],
    /// Abbreviated month names for the headline, January first.
    pub months_short: [String; 12],
    /// Weekday names, Monday first (announced for each day).
    pub weekdays: [String; 7],
    /// One-letter weekday column headers, Monday first.
    pub weekdays_narrow: [String; 7],
    /// Title of the single-date picker in calendar mode.
    pub title_select_date: String,
    /// Title of the range picker in calendar mode.
    pub title_select_dates: String,
    /// Title of the single-date picker in input mode.
    pub title_enter_date: String,
    /// Title of the range picker in input mode.
    pub title_enter_dates: String,
    /// Headline when no date is selected.
    pub headline_no_date: String,
    /// Placeholder for a missing range start in the headline, and the
    /// start field's label.
    pub start_date: String,
    /// Placeholder for a missing range end in the headline, and the end
    /// field's label.
    pub end_date: String,
    /// Label of the single input field.
    pub date: String,
    /// Suffix announced for today's date.
    pub today: String,
    /// The previous-month button.
    pub previous_month: String,
    /// The next-month button.
    pub next_month: String,
    /// The month/year button while the month grid shows.
    pub switch_to_year: String,
    /// The month/year button while the year list shows.
    pub switch_to_day: String,
    /// The mode toggle in calendar mode.
    pub switch_to_input: String,
    /// The mode toggle in input mode.
    pub switch_to_calendar: String,
    /// The docked field's calendar button.
    pub open_calendar: String,
    /// Error for text that does not match; `{pattern}` is replaced.
    pub error_pattern: String,
    /// Error for a year outside the range; `{start}` and `{end}` are replaced.
    pub error_year_range: String,
    /// Error for a date the picker does not allow.
    pub error_not_allowed: String,
    /// Error for an end date before the start date.
    pub error_range: String,
    /// The dialog's confirm button.
    pub confirm: String,
    /// The dialog's dismiss button.
    pub dismiss: String,
}

impl Default for DatePickerStrings {
    /// Compose's English strings (`strings.xml` in material3).
    fn default() -> Self {
        let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
        let months = s(&[
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ]);
        let months_short = s(&[
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]);
        let weekdays = s(&[
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ]);
        let narrow = s(&["M", "T", "W", "T", "F", "S", "S"]);
        Self {
            months: array12(months),
            months_short: array12(months_short),
            weekdays: array7(weekdays),
            weekdays_narrow: array7(narrow),
            title_select_date: "Select date".to_owned(),
            title_select_dates: "Select dates".to_owned(),
            title_enter_date: "Enter date".to_owned(),
            title_enter_dates: "Enter dates".to_owned(),
            headline_no_date: "Selected date".to_owned(),
            start_date: "Start date".to_owned(),
            end_date: "End date".to_owned(),
            date: "Date".to_owned(),
            today: "Today".to_owned(),
            previous_month: "Change to previous month".to_owned(),
            next_month: "Change to next month".to_owned(),
            switch_to_year: "Switch to selecting a year".to_owned(),
            switch_to_day: "Switch to selecting a day".to_owned(),
            switch_to_input: "Switch to text input mode".to_owned(),
            switch_to_calendar: "Switch to calendar input mode".to_owned(),
            open_calendar: "Open calendar".to_owned(),
            error_pattern: "Date does not match expected pattern: {pattern}".to_owned(),
            error_year_range: "Date out of expected year range {start} - {end}".to_owned(),
            error_not_allowed: "Date not allowed".to_owned(),
            error_range: "Invalid date range input".to_owned(),
            confirm: "OK".to_owned(),
            dismiss: "Cancel".to_owned(),
        }
    }
}

fn array12(v: Vec<String>) -> [String; 12] {
    let mut out: [String; 12] = Default::default();
    for (slot, value) in out.iter_mut().zip(v) {
        *slot = value;
    }
    out
}

fn array7(v: Vec<String>) -> [String; 7] {
    let mut out: [String; 7] = Default::default();
    for (slot, value) in out.iter_mut().zip(v) {
        *slot = value;
    }
    out
}

impl DatePickerStrings {
    /// The name of `month` (1..=12).
    pub fn month(&self, month: u8) -> &str {
        self.months
            .get(usize::from(month).wrapping_sub(1))
            .map_or("", String::as_str)
    }

    fn month_short(&self, month: u8) -> &str {
        self.months_short
            .get(usize::from(month).wrapping_sub(1))
            .map_or("", String::as_str)
    }

    fn weekday(&self, day: Weekday) -> &str {
        self.weekdays
            .get(usize::from(day.index_from_monday()))
            .map_or("", String::as_str)
    }

    fn weekday_narrow(&self, day: Weekday) -> &str {
        self.weekdays_narrow
            .get(usize::from(day.index_from_monday()))
            .map_or("", String::as_str)
    }

    /// The headline form of `date`: `Mar 5, 2024`.
    pub fn headline(&self, date: Date) -> String {
        format!(
            "{} {}, {}",
            self.month_short(date.month()),
            date.day(),
            date.year()
        )
    }

    /// The announced form of `date`: `Tuesday, March 5, 2024`.
    pub fn spoken(&self, date: Date) -> String {
        format!(
            "{}, {} {}, {}",
            self.weekday(date.weekday()),
            self.month(date.month()),
            date.day(),
            date.year()
        )
    }
}

/// The value a picker edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Value {
    Single(Option<Date>),
    Range(DateRange),
}

impl Value {
    fn anchor(self) -> Option<Date> {
        match self {
            Value::Single(date) => date,
            Value::Range(range) => range.start,
        }
    }
}

/// Where a picker's value lives.
enum Target<'a> {
    Single(&'a mut Option<Date>),
    Range(&'a mut DateRange),
}

impl Target<'_> {
    fn get(&self) -> Value {
        match self {
            Target::Single(date) => Value::Single(**date),
            Target::Range(range) => Value::Range(**range),
        }
    }

    fn set(&mut self, value: Value) {
        match (self, value) {
            (Target::Single(date), Value::Single(v)) => **date = v,
            (Target::Range(range), Value::Range(v)) => **range = v,
            (Target::Single(_), Value::Range(_)) | (Target::Range(_), Value::Single(_)) => {}
        }
    }
}

/// A predicate deciding which dates can be selected.
pub type SelectableDates<'a> = Box<dyn Fn(Date) -> bool + 'a>;

/// Everything about a picker except its value.
pub(crate) struct Config<'a> {
    id_salt: IdSalt,
    first_day_of_week: Weekday,
    year_range: RangeInclusive<i32>,
    today: Option<Date>,
    selectable: Option<SelectableDates<'a>>,
    initial_mode: DisplayMode,
    show_mode_toggle: bool,
    show_header: bool,
    paint_container: bool,
    docked: bool,
    strings: DatePickerStrings,
    format: DateFormat,
}

impl Config<'_> {
    fn new(id_salt: IdSalt) -> Self {
        Self {
            id_salt,
            first_day_of_week: Weekday::Sunday,
            year_range: 1900..=2100,
            today: None,
            selectable: None,
            initial_mode: DisplayMode::Picker,
            show_mode_toggle: true,
            show_header: true,
            paint_container: true,
            docked: false,
            strings: DatePickerStrings::default(),
            format: DateFormat::default(),
        }
    }

    fn today(&self) -> Date {
        self.today.unwrap_or_else(Date::today_utc)
    }

    fn in_year_range(&self, date: Date) -> bool {
        self.year_range.contains(&date.year())
    }

    /// Whether `date` may be picked: inside the year range and allowed by
    /// the predicate.
    fn allows(&self, date: Date) -> bool {
        self.in_year_range(date) && self.selectable.as_ref().is_none_or(|f| f(date))
    }

    fn first_month(&self) -> Option<Date> {
        Date::new(*self.year_range.start(), 1, 1)
    }

    fn last_month(&self) -> Option<Date> {
        Date::new(*self.year_range.end(), 12, 1)
    }

    /// `date`'s month, moved into the year range.
    fn clamp_month(&self, date: Date) -> Date {
        let month = date.first_of_month();
        match (self.first_month(), self.last_month()) {
            (Some(first), _) if month < first => first,
            (_, Some(last)) if month > last => last,
            (Some(_) | None, Some(_) | None) => month,
        }
    }
}

/// Per-picker UI state, kept in egui memory between frames.
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    /// First day of the month on show; `None` until the first frame.
    displayed: Option<Date>,
    mode: Option<DisplayMode>,
    years_open: bool,
    scroll_to_year: bool,
    /// The day to give keyboard focus to when its cell is drawn.
    focus_request: Option<Date>,
    /// The day cell that had focus when last drawn.
    focused: Option<Date>,
    start_text: String,
    end_text: String,
    /// The value the input texts were last filled from.
    synced: Option<Value>,
}

impl State {
    fn load(ctx: &egui::Context, id: Id) -> Self {
        ctx.data(|d| d.get_temp::<State>(id)).unwrap_or_default()
    }

    fn store(self, ctx: &egui::Context, id: Id) {
        ctx.data_mut(|d| d.insert_temp(id, self));
    }
}

/// A date picker: calendar and input modes, single date or range.
///
/// See the [module docs](self) for an example. The value is written as the
/// user picks; [`Response::changed`] reports when it did.
pub struct DatePicker<'a> {
    target: Target<'a>,
    config: Config<'a>,
}

impl<'a> DatePicker<'a> {
    /// A picker for one date.
    pub fn single(date: &'a mut Option<Date>) -> Self {
        Self {
            target: Target::Single(date),
            config: Config::new(IdSalt::new("material_date_picker")),
        }
    }

    /// A picker for a range of dates.
    pub fn range(range: &'a mut DateRange) -> Self {
        Self {
            target: Target::Range(range),
            config: Config::new(IdSalt::new("material_date_range_picker")),
        }
    }

    /// Distinguishes this picker's state from other pickers'.
    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.config.id_salt = IdSalt::new(salt);
        self
    }

    /// The weekday the grid's first column shows. Default Sunday.
    pub fn first_day_of_week(mut self, day: Weekday) -> Self {
        self.config.first_day_of_week = day;
        self
    }

    /// The years that can be shown and picked. Default 1900..=2100
    /// (`DatePickerDefaults.YearRange`).
    pub fn year_range(mut self, years: RangeInclusive<i32>) -> Self {
        let (start, end) = (*years.start(), *years.end());
        let start = start.clamp(Date::MIN_YEAR, Date::MAX_YEAR);
        let end = end.clamp(start, Date::MAX_YEAR);
        self.config.year_range = start..=end;
        self
    }

    /// The date drawn as today. Default: today in UTC.
    pub fn today(mut self, today: Date) -> Self {
        self.config.today = Some(today);
        self
    }

    /// Only dates for which `selectable` returns true can be picked.
    pub fn selectable_dates(mut self, selectable: impl Fn(Date) -> bool + 'a) -> Self {
        self.config.selectable = Some(Box::new(selectable));
        self
    }

    /// The mode the picker opens in. Default [`DisplayMode::Picker`].
    pub fn display_mode(mut self, mode: DisplayMode) -> Self {
        self.config.initial_mode = mode;
        self
    }

    /// Whether to show the calendar/input toggle. Default true.
    pub fn show_mode_toggle(mut self, show: bool) -> Self {
        self.config.show_mode_toggle = show;
        self
    }

    /// Whether to show the title and headline. Default true.
    pub fn show_header(mut self, show: bool) -> Self {
        self.config.show_header = show;
        self
    }

    /// The strings shown and announced.
    pub fn strings(mut self, strings: DatePickerStrings) -> Self {
        self.config.strings = strings;
        self
    }

    /// The typed format of the input mode. Default `YYYY-MM-DD`.
    pub fn date_format(mut self, format: DateFormat) -> Self {
        self.config.format = format;
        self
    }

    fn id(&self, ui: &Ui) -> Id {
        ui.make_persistent_id(self.config.id_salt)
    }
}

impl Widget for DatePicker<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let id = self.id(ui);
        let mut value = self.target.get();
        let mut response = show_picker(ui, id, &self.config, &mut value);
        if value != self.target.get() {
            self.target.set(value);
            response.mark_changed();
        }
        response
    }
}

/// Draws a picker for `value` with `config`; the outer response.
pub(crate) fn show_picker(ui: &mut Ui, id: Id, config: &Config<'_>, value: &mut Value) -> Response {
    let theme = MaterialTheme::of(ui.ctx());
    let mut state = State::load(ui.ctx(), id);
    let mode = *state.mode.get_or_insert(config.initial_mode);
    let displayed = match state.displayed {
        Some(d) => d,
        None => config.clamp_month(value.anchor().unwrap_or_else(|| config.today())),
    };
    state.displayed = Some(displayed);

    let width = match mode {
        DisplayMode::Picker => t::CONTAINER_WIDTH,
        DisplayMode::Input => t::INPUT_CONTAINER_WIDTH,
    };
    let surface = theme.color(t::CONTAINER_COLOR);

    let inner = ui.allocate_ui_with_layout(
        Vec2::new(width, 0.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(width);
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            // Reserve the container's background under everything drawn next.
            let background = ui.painter().add(egui::Shape::Noop);
            let top = ui.cursor().min;
            if config.show_header {
                calendar::header(ui, id, config, &mut state, *value, mode, width);
            }
            match mode {
                DisplayMode::Picker => calendar::body(ui, id, config, &mut state, value, width),
                DisplayMode::Input => {
                    input::body(ui, id, config, &mut state, value, width, surface);
                }
            }
            let rect = Rect::from_min_max(top, egui::pos2(top.x + width, ui.cursor().min.y));
            if config.paint_container {
                let shape = if config.docked {
                    t::DOCKED_CONTAINER_SHAPE
                } else {
                    t::CONTAINER_SHAPE
                };
                ui.painter().set(
                    background,
                    egui::Shape::rect_filled(rect, common::radius(&theme, shape, rect), surface),
                );
            }
            rect
        },
    );
    state.store(ui.ctx(), id);
    ui.interact(inner.inner, id.with("container"), Sense::hover())
}

/// Removes a picker's remembered UI state, so it opens fresh next time.
fn forget(ctx: &egui::Context, id: Id) {
    ctx.data_mut(|d| d.remove::<State>(id));
}

/// The modal date picker: [`DatePicker`] in a dialog with confirm and
/// dismiss buttons, over a scrim.
///
/// Call [`DatePickerDialog::show`] every frame while your "open" flag is
/// set, and clear the flag when it returns [`DialogAction::Confirmed`] or
/// [`DialogAction::Dismissed`]. The value is written only on confirm.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::date_picker::{Date, DatePicker, DatePickerDialog, DialogAction};
/// # let mut open = true;
/// let mut date: Option<Date> = None;
/// if open {
///     match DatePickerDialog::new(DatePicker::single(&mut date).id_salt("due")).show(ui.ctx()) {
///         DialogAction::Open => {}
///         DialogAction::Confirmed | DialogAction::Dismissed => open = false,
///     }
/// }
/// # });
/// ```
pub struct DatePickerDialog<'a> {
    picker: DatePicker<'a>,
}

impl<'a> DatePickerDialog<'a> {
    /// A dialog around `picker`.
    pub fn new(picker: DatePicker<'a>) -> Self {
        Self { picker }
    }

    /// Shows the dialog for this frame.
    pub fn show(self, ctx: &egui::Context) -> DialogAction {
        let DatePicker {
            mut target,
            mut config,
        } = self.picker;
        let id = Id::unique(("material_date_picker_dialog", config.id_salt));
        let pending_id = id.with("pending");
        let mut pending = ctx
            .data(|d| d.get_temp::<Value>(pending_id))
            .unwrap_or_else(|| target.get());
        config.paint_container = false;
        let theme = MaterialTheme::of(ctx);

        let modal = egui::Modal::new(id)
            .frame(common::dialog_frame(
                &theme,
                t::CONTAINER_COLOR,
                t::CONTAINER_SHAPE,
            ))
            .backdrop_color(common::scrim_color(&theme))
            .show(ctx, |ui| {
                let response = show_picker(ui, id.with("picker"), &config, &mut pending);
                let complete = match pending {
                    Value::Single(_) => true,
                    Value::Range(range) => range.get().is_some() || range.start.is_none(),
                };
                common::dialog_buttons(
                    ui,
                    response.rect.width(),
                    &config.strings.dismiss,
                    &config.strings.confirm,
                    complete,
                )
            });
        let (dismissed, confirmed) = modal.inner;
        let action = if confirmed {
            target.set(pending);
            DialogAction::Confirmed
        } else if dismissed || modal.should_close() {
            DialogAction::Dismissed
        } else {
            DialogAction::Open
        };
        match action {
            DialogAction::Open => {
                ctx.data_mut(|d| d.insert_temp(pending_id, pending));
            }
            DialogAction::Confirmed | DialogAction::Dismissed => {
                ctx.data_mut(|d| d.remove::<Value>(pending_id));
                forget(ctx, id.with("picker"));
            }
        }
        action
    }
}

/// The docked date picker: an outlined date field whose calendar button
/// drops a calendar down below it. Typing a valid date or picking one in
/// the calendar sets the value.
pub struct DockedDatePicker<'a> {
    date: &'a mut Option<Date>,
    config: Config<'a>,
    width: f32,
    label: Option<String>,
}

impl<'a> DockedDatePicker<'a> {
    /// A docked picker for `date`.
    pub fn new(date: &'a mut Option<Date>) -> Self {
        let mut config = Config::new(IdSalt::new("material_docked_date_picker"));
        config.show_header = false;
        config.show_mode_toggle = false;
        config.docked = true;
        config.paint_container = false;
        Self {
            date,
            config,
            width: t::CONTAINER_WIDTH,
            label: None,
        }
    }

    /// Distinguishes this picker's state from other pickers'.
    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.config.id_salt = IdSalt::new(salt);
        self
    }

    /// The field's label. Default [`DatePickerStrings::date`].
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// The field's width. Default 360.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(t::ICON_BUTTON_SIZE * 2.0);
        self
    }

    /// The weekday the grid's first column shows. Default Sunday.
    pub fn first_day_of_week(mut self, day: Weekday) -> Self {
        self.config.first_day_of_week = day;
        self
    }

    /// The years that can be shown and picked. Default 1900..=2100.
    pub fn year_range(mut self, years: RangeInclusive<i32>) -> Self {
        let (start, end) = (*years.start(), *years.end());
        let start = start.clamp(Date::MIN_YEAR, Date::MAX_YEAR);
        let end = end.clamp(start, Date::MAX_YEAR);
        self.config.year_range = start..=end;
        self
    }

    /// The date drawn as today. Default: today in UTC.
    pub fn today(mut self, today: Date) -> Self {
        self.config.today = Some(today);
        self
    }

    /// Only dates for which `selectable` returns true can be picked.
    pub fn selectable_dates(mut self, selectable: impl Fn(Date) -> bool + 'a) -> Self {
        self.config.selectable = Some(Box::new(selectable));
        self
    }

    /// The strings shown and announced.
    pub fn strings(mut self, strings: DatePickerStrings) -> Self {
        self.config.strings = strings;
        self
    }

    /// The typed format. Default `YYYY-MM-DD`.
    pub fn date_format(mut self, format: DateFormat) -> Self {
        self.config.format = format;
        self
    }
}

impl Widget for DockedDatePicker<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let DockedDatePicker {
            date,
            config,
            width,
            label,
        } = self;
        let theme = MaterialTheme::of(ui.ctx());
        let id = ui.make_persistent_id(config.id_salt);
        let open_id = id.with("open");
        let mut open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(open_id))
            .unwrap_or(false);
        let mut value = Value::Single(*date);
        let before = value;

        // The field: typed input with validation.
        let mut state = State::load(ui.ctx(), id.with("field"));
        let mut field_value = value;
        let label = label.unwrap_or_else(|| config.strings.date.clone());
        let surface = ui.visuals().panel_fill;
        let field = input::docked_field(
            ui,
            id.with("field"),
            &config,
            &mut state,
            &mut field_value,
            width,
            surface,
            &label,
        );
        state.store(ui.ctx(), id.with("field"));
        if field_value != value {
            value = field_value;
        }
        if field.trailing_clicked {
            open = !open;
        }

        if open {
            let picker_id = id.with("picker");
            let pos =
                field.response.rect.left_bottom() + Vec2::new(-t::FIELD_HORIZONTAL_PADDING, 12.0);
            let area = egui::Area::new(id.with("popup"))
                .order(egui::Order::Foreground)
                .fixed_pos(pos)
                .show(ui.ctx(), |ui| {
                    common::dialog_frame(&theme, t::CONTAINER_COLOR, t::DOCKED_CONTAINER_SHAPE)
                        .show(ui, |ui| show_picker(ui, picker_id, &config, &mut value))
                });
            let picked = value != before && value != field_value;
            let clicked_outside = ui.input(|i| i.pointer.any_click())
                && !area.response.contains_pointer()
                && !field
                    .response
                    .rect
                    .expand(t::ICON_BUTTON_SIZE)
                    .contains(ui.input(|i| i.pointer.interact_pos()).unwrap_or_default());
            let escape = ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
            if picked || clicked_outside || escape {
                open = false;
                forget(ui.ctx(), picker_id);
            }
        }
        ui.ctx().data_mut(|d| d.insert_temp(open_id, open));

        let mut response = field.response;
        if value != before {
            if let Value::Single(v) = value {
                *date = v;
            }
            response.mark_changed();
        }
        response
    }
}
