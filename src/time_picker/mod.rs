// ============================================================================
// Module:       time_picker
// Description:  Material 3 time pickers: dial and input, 12 and 24 hour,
//               vertical and horizontal layouts, and the modal dialog.
//
// Dependencies: egui; crate::{date_picker::common, tokens, paint, a11y,
//               MaterialTheme}
// ============================================================================

//! Material 3 time pickers.
//!
//! - [`TimePicker`]: the clock display (hour and minute selectors, AM/PM)
//!   and the dial (Compose's `TimePicker`), in a vertical or horizontal
//!   [`TimePickerLayout`].
//! - [`TimeInput`]: hour and minute text fields (Compose's `TimeInput`).
//! - [`TimePickerDialog`]: either one in a modal dialog with a toggle
//!   between them; writes the time back on confirm.
//!
//! [`TimeFormat::H12`] shows 1–12 with an AM/PM selector;
//! [`TimeFormat::H24`] puts 12–23 on an inner ring of the dial. A press or
//! drag on the dial picks the value under the pointer; releasing after an
//! hour moves on to minutes. With the dial focused, the arrow keys step
//! the value.
//!
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_material3::time_picker::{Time, TimeFormat, TimePicker};
//! let mut alarm = Time::new(7, 30).unwrap_or_default();
//! ui.add(TimePicker::new(&mut alarm).format(TimeFormat::H24));
//! # });
//! ```

mod dial;
mod parts;
pub mod tokens;

use egui::{Id, IdSalt, Response, Sense, Ui, Vec2, Widget};

use crate::date_picker::common::{self, Glyph};
pub use crate::date_picker::DialogAction;
use crate::MaterialTheme;
use tokens as t;

/// A time of day: hour 0..24 and minute 0..60.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    /// The time, or `None` when the hour is not 0..24 or the minute 0..60.
    pub const fn new(hour: u8, minute: u8) -> Option<Time> {
        if hour < 24 && minute < 60 {
            Some(Time { hour, minute })
        } else {
            None
        }
    }

    /// The hour, 0..24.
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// The minute, 0..60.
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// Whether the time is noon or later.
    pub const fn is_pm(self) -> bool {
        self.hour >= 12
    }

    /// The same time in the morning (`pm` false) or afternoon.
    pub const fn with_pm(self, pm: bool) -> Time {
        let hour = self.hour % 12 + if pm { 12 } else { 0 };
        Time { hour, ..self }
    }

    fn with(self, selection: Selection, value: u8) -> Time {
        match selection {
            Selection::Hour => Time {
                hour: value % 24,
                ..self
            },
            Selection::Minute => Time {
                minute: value % 60,
                ..self
            },
        }
    }
}

impl std::fmt::Display for Time {
    /// `HH:MM`, 24-hour.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

/// 12-hour (with AM/PM) or 24-hour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimeFormat {
    /// 1–12 with an AM/PM selector.
    #[default]
    H12,
    /// 0–23.
    H24,
}

/// Where the clock display sits relative to the dial.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimePickerLayout {
    /// Display above the dial (portrait).
    #[default]
    Vertical,
    /// Display beside the dial (landscape).
    Horizontal,
}

/// Dial or text input.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimePickerMode {
    /// The clock dial.
    #[default]
    Dial,
    /// Hour and minute fields.
    Input,
}

/// Which value the dial edits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum Selection {
    #[default]
    Hour,
    Minute,
}

/// The text a time picker shows and announces. English by default.
#[derive(Clone, Debug, PartialEq)]
pub struct TimePickerStrings {
    /// The dialog title in dial mode.
    pub title_select_time: String,
    /// The dialog title in input mode.
    pub title_enter_time: String,
    /// The hour selector, dial and field.
    pub hour: String,
    /// The minute selector, dial and field.
    pub minute: String,
    /// The morning period.
    pub am: String,
    /// The afternoon period.
    pub pm: String,
    /// The mode toggle in dial mode.
    pub switch_to_input: String,
    /// The mode toggle in input mode.
    pub switch_to_dial: String,
    /// The dialog's confirm button.
    pub confirm: String,
    /// The dialog's dismiss button.
    pub dismiss: String,
}

impl Default for TimePickerStrings {
    /// Compose's English strings.
    fn default() -> Self {
        Self {
            title_select_time: "Select time".to_owned(),
            title_enter_time: "Enter time".to_owned(),
            hour: "Hour".to_owned(),
            minute: "Minute".to_owned(),
            am: "AM".to_owned(),
            pm: "PM".to_owned(),
            switch_to_input: "Switch to text input mode for the time input.".to_owned(),
            switch_to_dial: "Switch to clock mode for the time input.".to_owned(),
            confirm: "OK".to_owned(),
            dismiss: "Cancel".to_owned(),
        }
    }
}

/// Per-picker UI state, kept in egui memory.
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    selection: Selection,
    hour_text: String,
    minute_text: String,
    synced: Option<(Time, TimeFormat)>,
}

impl State {
    fn load(ctx: &egui::Context, id: Id) -> Self {
        ctx.data(|d| d.get_temp::<State>(id)).unwrap_or_default()
    }

    fn store(self, ctx: &egui::Context, id: Id) {
        ctx.data_mut(|d| d.insert_temp(id, self));
    }
}

/// Settings shared by the dial and input pickers.
#[derive(Clone, Debug)]
struct Config {
    id_salt: IdSalt,
    format: TimeFormat,
    layout: TimePickerLayout,
    strings: TimePickerStrings,
}

impl Config {
    fn new(salt: &str) -> Self {
        Self {
            id_salt: IdSalt::new(salt),
            format: TimeFormat::H12,
            layout: TimePickerLayout::Vertical,
            strings: TimePickerStrings::default(),
        }
    }
}

/// The time picker with the clock dial.
pub struct TimePicker<'a> {
    time: &'a mut Time,
    config: Config,
}

impl<'a> TimePicker<'a> {
    /// A dial picker for `time`.
    pub fn new(time: &'a mut Time) -> Self {
        Self {
            time,
            config: Config::new("material_time_picker"),
        }
    }

    /// Distinguishes this picker's state from other pickers'.
    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.config.id_salt = IdSalt::new(salt);
        self
    }

    /// 12- or 24-hour. Default 12-hour.
    pub fn format(mut self, format: TimeFormat) -> Self {
        self.config.format = format;
        self
    }

    /// Display above or beside the dial. Default vertical.
    pub fn layout(mut self, layout: TimePickerLayout) -> Self {
        self.config.layout = layout;
        self
    }

    /// The strings shown and announced.
    pub fn strings(mut self, strings: TimePickerStrings) -> Self {
        self.config.strings = strings;
        self
    }
}

impl Widget for TimePicker<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id(self.config.id_salt);
        show(ui, id, &self.config, TimePickerMode::Dial, self.time)
    }
}

/// The time picker with hour and minute text fields.
pub struct TimeInput<'a> {
    time: &'a mut Time,
    config: Config,
}

impl<'a> TimeInput<'a> {
    /// An input picker for `time`.
    pub fn new(time: &'a mut Time) -> Self {
        Self {
            time,
            config: Config::new("material_time_input"),
        }
    }

    /// Distinguishes this picker's state from other pickers'.
    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.config.id_salt = IdSalt::new(salt);
        self
    }

    /// 12- or 24-hour. Default 12-hour.
    pub fn format(mut self, format: TimeFormat) -> Self {
        self.config.format = format;
        self
    }

    /// The strings shown and announced.
    pub fn strings(mut self, strings: TimePickerStrings) -> Self {
        self.config.strings = strings;
        self
    }
}

impl Widget for TimeInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id(self.config.id_salt);
        show(ui, id, &self.config, TimePickerMode::Input, self.time)
    }
}

/// Draws the dial or input picker; the outer response, changed when the
/// time did.
fn show(ui: &mut Ui, id: Id, config: &Config, mode: TimePickerMode, time: &mut Time) -> Response {
    let mut state = State::load(ui.ctx(), id);
    let before = *time;
    let inner = ui.scope(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        match mode {
            TimePickerMode::Dial => dial_layout(ui, id, config, &mut state, time),
            TimePickerMode::Input => input_layout(ui, id, config, &mut state, time),
        }
    });
    state.store(ui.ctx(), id);
    let mut response = ui.interact(inner.response.rect, id.with("container"), Sense::hover());
    if *time != before {
        response.mark_changed();
    }
    response
}

/// The clock display: hour, `:`, minute.
fn clock_display(ui: &mut Ui, id: Id, config: &Config, state: &mut State, time: Time) {
    let width = match (config.format, config.layout) {
        (TimeFormat::H24, TimePickerLayout::Vertical) => t::SELECTOR_24H_VERTICAL_WIDTH,
        (TimeFormat::H12, TimePickerLayout::Vertical | TimePickerLayout::Horizontal)
        | (TimeFormat::H24, TimePickerLayout::Horizontal) => t::SELECTOR_WIDTH,
    };
    ui.horizontal(|ui| {
        for (i, which) in [Selection::Hour, Selection::Minute].into_iter().enumerate() {
            if i == 1 {
                parts::separator(ui, t::SELECTOR_HEIGHT);
            }
            if parts::time_selector(
                ui,
                id.with(("selector", i)),
                which,
                state,
                time,
                config.format,
                width,
                &config.strings,
            ) {
                state.selection = which;
            }
        }
    });
}

fn period(ui: &mut Ui, id: Id, config: &Config, time: &mut Time, size: Vec2, vertical: bool) {
    ui.add_space(t::PERIOD_GAP);
    if let Some(new) = parts::period_selector(
        ui,
        id.with("period"),
        *time,
        size,
        vertical,
        &config.strings,
    ) {
        *time = new;
    }
}

fn dial_layout(ui: &mut Ui, id: Id, config: &Config, state: &mut State, time: &mut Time) {
    let twelve = config.format == TimeFormat::H12;
    match config.layout {
        TimePickerLayout::Vertical => {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    clock_display(ui, id, config, state, *time);
                    if twelve {
                        let size = Vec2::new(t::PERIOD_VERTICAL_WIDTH, t::PERIOD_VERTICAL_HEIGHT);
                        period(ui, id, config, time, size, true);
                    }
                });
                ui.add_space(t::DISPLAY_BOTTOM_MARGIN);
                dial::dial(
                    ui,
                    id.with("dial"),
                    state,
                    time,
                    config.format,
                    &config.strings,
                );
                ui.add_space(t::DIAL_BOTTOM_MARGIN);
            });
        }
        TimePickerLayout::Horizontal => {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    clock_display(ui, id, config, state, *time);
                    if twelve {
                        let size =
                            Vec2::new(t::PERIOD_HORIZONTAL_WIDTH, t::PERIOD_HORIZONTAL_HEIGHT);
                        period(ui, id, config, time, size, false);
                    }
                });
                ui.add_space(t::DISPLAY_BOTTOM_MARGIN);
                dial::dial(
                    ui,
                    id.with("dial"),
                    state,
                    time,
                    config.format,
                    &config.strings,
                );
            });
            ui.add_space(t::DIAL_BOTTOM_MARGIN);
        }
    }
}

fn input_layout(ui: &mut Ui, id: Id, config: &Config, state: &mut State, time: &mut Time) {
    if state.synced != Some((*time, config.format)) {
        state.hour_text = parts::display_value(*time, Selection::Hour, config.format);
        state.minute_text = parts::display_value(*time, Selection::Minute, config.format);
        state.synced = Some((*time, config.format));
    }
    ui.horizontal_top(|ui| {
        let hour = parts::time_field(
            ui,
            id.with("hour_field"),
            &mut state.hour_text,
            Selection::Hour,
            config.format,
            &config.strings,
        );
        parts::separator(ui, t::FIELD_HEIGHT);
        let minute = parts::time_field(
            ui,
            id.with("minute_field"),
            &mut state.minute_text,
            Selection::Minute,
            config.format,
            &config.strings,
        );
        if let Some(h) = hour {
            let h = match config.format {
                TimeFormat::H24 => h,
                TimeFormat::H12 => h % 12 + if time.is_pm() { 12 } else { 0 },
            };
            *time = time.with(Selection::Hour, h);
        }
        if let Some(m) = minute {
            *time = time.with(Selection::Minute, m);
        }
        if config.format == TimeFormat::H12 {
            let size = Vec2::new(t::INPUT_PERIOD_WIDTH, t::INPUT_PERIOD_HEIGHT);
            period(ui, id, config, time, size, true);
        }
    });
    // What was typed stays as typed; only changes from outside refill it.
    state.synced = Some((*time, config.format));
}

/// The modal time picker: [`TimePicker`] or [`TimeInput`] in a dialog with
/// a mode toggle and confirm and dismiss buttons. The time is written only
/// on confirm; call [`TimePickerDialog::show`] every frame while open.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::time_picker::{DialogAction, Time, TimePickerDialog};
/// # let mut open = true;
/// let mut time = Time::default();
/// if open {
///     match TimePickerDialog::new(&mut time).show(ui.ctx()) {
///         DialogAction::Open => {}
///         DialogAction::Confirmed | DialogAction::Dismissed => open = false,
///     }
/// }
/// # });
/// ```
pub struct TimePickerDialog<'a> {
    time: &'a mut Time,
    config: Config,
    initial_mode: TimePickerMode,
    show_mode_toggle: bool,
}

impl<'a> TimePickerDialog<'a> {
    /// A dialog editing `time`.
    pub fn new(time: &'a mut Time) -> Self {
        Self {
            time,
            config: Config::new("material_time_picker_dialog"),
            initial_mode: TimePickerMode::Dial,
            show_mode_toggle: true,
        }
    }

    /// Distinguishes this dialog's state from other dialogs'.
    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.config.id_salt = IdSalt::new(salt);
        self
    }

    /// 12- or 24-hour. Default 12-hour.
    pub fn format(mut self, format: TimeFormat) -> Self {
        self.config.format = format;
        self
    }

    /// Display above or beside the dial. Default vertical.
    pub fn layout(mut self, layout: TimePickerLayout) -> Self {
        self.config.layout = layout;
        self
    }

    /// The mode the dialog opens in. Default the dial.
    pub fn mode(mut self, mode: TimePickerMode) -> Self {
        self.initial_mode = mode;
        self
    }

    /// Whether to show the dial/input toggle. Default true.
    pub fn show_mode_toggle(mut self, show: bool) -> Self {
        self.show_mode_toggle = show;
        self
    }

    /// The strings shown and announced.
    pub fn strings(mut self, strings: TimePickerStrings) -> Self {
        self.config.strings = strings;
        self
    }

    /// Shows the dialog for this frame.
    pub fn show(self, ctx: &egui::Context) -> DialogAction {
        let id = Id::unique(("material_time_picker_dialog", self.config.id_salt));
        let pending_id = id.with("pending");
        let mode_id = id.with("mode");
        let picker_id = id.with("picker");
        let mut pending = ctx
            .data(|d| d.get_temp::<Time>(pending_id))
            .unwrap_or(*self.time);
        let mut mode = ctx
            .data(|d| d.get_temp::<TimePickerMode>(mode_id))
            .unwrap_or(self.initial_mode);
        let theme = MaterialTheme::of(ctx);
        let config = &self.config;
        let show_toggle = self.show_mode_toggle;
        // The 24 dp content padding (`contentPadding` in `TimePickerDialog.kt`).
        let padding = t::DIALOG_PADDING.round().clamp(0.0, 127.0) as i8;

        let modal = egui::Modal::new(id)
            .frame(
                common::dialog_frame(&theme, t::CONTAINER_COLOR, t::CONTAINER_SHAPE).inner_margin(
                    egui::Margin {
                        left: padding,
                        right: padding,
                        top: padding,
                        bottom: 0,
                    },
                ),
            )
            .backdrop_color(common::scrim_color(&theme))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                let title = match mode {
                    TimePickerMode::Dial => &config.strings.title_select_time,
                    TimePickerMode::Input => &config.strings.title_enter_time,
                };
                let g = common::galley(ui, t::HEADLINE_FONT, title, theme.color(t::HEADLINE_COLOR));
                let (title_rect, _) = ui
                    .allocate_exact_size(Vec2::new(g.size().x, g.size().y + 20.0), Sense::hover());
                ui.painter()
                    .galley(title_rect.min, g, egui::Color32::PLACEHOLDER);
                let content = show(ui, picker_id, config, mode, &mut pending);
                let width = content.rect.width().max(t::DIAL_SIZE);
                let mut toggled = false;
                let row = ui.horizontal(|ui| {
                    let mut used = 0.0;
                    if show_toggle {
                        let (glyph, label) = match mode {
                            TimePickerMode::Dial => {
                                (Glyph::Keyboard, &config.strings.switch_to_input)
                            }
                            TimePickerMode::Input => (Glyph::Clock, &config.strings.switch_to_dial),
                        };
                        let color = theme.color(crate::tokens::ColorRole::OnSurfaceVariant);
                        toggled =
                            common::icon_button(ui, id.with("toggle"), glyph, label, true, color)
                                .clicked();
                        used = crate::date_picker::tokens::ICON_BUTTON_SIZE;
                    }
                    common::dialog_buttons(
                        ui,
                        (width - used).max(0.0),
                        &config.strings.dismiss,
                        &config.strings.confirm,
                        true,
                    )
                });
                (row.inner, toggled)
            });
        let ((dismissed, confirmed), toggled) = modal.inner;
        if toggled {
            mode = match mode {
                TimePickerMode::Dial => TimePickerMode::Input,
                TimePickerMode::Input => TimePickerMode::Dial,
            };
        }
        let action = if confirmed {
            *self.time = pending;
            DialogAction::Confirmed
        } else if dismissed || modal.should_close() {
            DialogAction::Dismissed
        } else {
            DialogAction::Open
        };
        match action {
            DialogAction::Open => ctx.data_mut(|d| {
                d.insert_temp(pending_id, pending);
                d.insert_temp(mode_id, mode);
            }),
            DialogAction::Confirmed | DialogAction::Dismissed => ctx.data_mut(|d| {
                d.remove::<Time>(pending_id);
                d.remove::<TimePickerMode>(mode_id);
                d.remove::<State>(picker_id);
            }),
        }
        action
    }
}
