// ============================================================================
// Module:       date_picker::date
// Description:  Locale-agnostic calendar dates, weekdays, ranges and the
//               typed date format the input mode parses.
//
// Dependencies: std
// ============================================================================

//! Proleptic Gregorian calendar dates with no time zone and no locale.
//!
//! [`Date`] is always valid: the constructors reject a day that does not
//! exist, and the arithmetic clamps to [`Date::MIN_YEAR`]..=[`Date::MAX_YEAR`]
//! instead of overflowing. Day counting uses Howard Hinnant's
//! `days_from_civil` / `civil_from_days` algorithms.

use std::fmt;

/// A day of the week.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Weekday {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

impl Weekday {
    /// Every weekday, Monday first.
    pub const ALL: [Weekday; 7] = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday,
    ];

    /// 0 for Monday through 6 for Sunday.
    pub const fn index_from_monday(self) -> u8 {
        match self {
            Weekday::Monday => 0,
            Weekday::Tuesday => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday => 3,
            Weekday::Friday => 4,
            Weekday::Saturday => 5,
            Weekday::Sunday => 6,
        }
    }

    /// The weekday `index` days after Monday, wrapping every 7.
    pub const fn from_index_from_monday(index: u8) -> Weekday {
        match index % 7 {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            _ => Weekday::Sunday,
        }
    }

    /// The weekday `n` days later.
    pub const fn add(self, n: u8) -> Weekday {
        Weekday::from_index_from_monday(self.index_from_monday() % 7 + n % 7)
    }

    /// How many days after `first` this weekday falls, 0..=6: the column
    /// of this weekday in a week that starts on `first`.
    pub const fn days_since(self, first: Weekday) -> u8 {
        (self.index_from_monday() + 7 - first.index_from_monday()) % 7
    }
}

/// Whether `year` is a Gregorian leap year.
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// The number of days in `month` (1..=12) of `year`, or 0 for a month
/// outside 1..=12.
pub const fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// A calendar date: year, month (1..=12) and day (1..=31).
///
/// Ordered chronologically.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Date {
    /// The earliest year a [`Date`] can hold.
    pub const MIN_YEAR: i32 = -9999;
    /// The latest year a [`Date`] can hold.
    pub const MAX_YEAR: i32 = 9999;

    /// The date, or `None` when it does not exist (February 30th, month 13,
    /// a year outside [`Date::MIN_YEAR`]..=[`Date::MAX_YEAR`]).
    pub const fn new(year: i32, month: u8, day: u8) -> Option<Date> {
        if year < Self::MIN_YEAR || year > Self::MAX_YEAR {
            return None;
        }
        if day == 0 || day > days_in_month(year, month) {
            return None;
        }
        Some(Date { year, month, day })
    }

    /// The year.
    pub const fn year(self) -> i32 {
        self.year
    }

    /// The month, 1..=12.
    pub const fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, 1..=31.
    pub const fn day(self) -> u8 {
        self.day
    }

    /// Days since 1970-01-01 (negative before it).
    pub const fn days_since_epoch(self) -> i64 {
        // Hinnant, days_from_civil.
        let y = self.year as i64 - if self.month <= 2 { 1 } else { 0 };
        let era = (if y >= 0 { y } else { y - 399 }) / 400;
        let yoe = y - era * 400;
        let m = self.month as i64;
        let mp = if m > 2 { m - 3 } else { m + 9 };
        let doy = (153 * mp + 2) / 5 + self.day as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    /// The date `days` days after 1970-01-01, clamped to the representable
    /// years.
    pub const fn from_days_since_epoch(days: i64) -> Date {
        const MIN_DAYS: i64 = Date::min().days_since_epoch();
        const MAX_DAYS: i64 = Date::max().days_since_epoch();
        let days = if days < MIN_DAYS {
            MIN_DAYS
        } else if days > MAX_DAYS {
            MAX_DAYS
        } else {
            days
        };
        // Hinnant, civil_from_days.
        let z = days + 719_468;
        let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
        let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u8;
        let year = (yoe + era * 400 + if month <= 2 { 1 } else { 0 }) as i32;
        Date { year, month, day }
    }

    /// January 1st of [`Date::MIN_YEAR`].
    pub const fn min() -> Date {
        Date {
            year: Self::MIN_YEAR,
            month: 1,
            day: 1,
        }
    }

    /// December 31st of [`Date::MAX_YEAR`].
    pub const fn max() -> Date {
        Date {
            year: Self::MAX_YEAR,
            month: 12,
            day: 31,
        }
    }

    /// The day of the week.
    pub const fn weekday(self) -> Weekday {
        // 1970-01-01 was a Thursday (index 3 from Monday).
        let index = (self.days_since_epoch() + 3).rem_euclid(7);
        Weekday::from_index_from_monday(index as u8)
    }

    /// The date `days` days later (earlier when negative), clamped.
    pub const fn add_days(self, days: i64) -> Date {
        Date::from_days_since_epoch(self.days_since_epoch().saturating_add(days))
    }

    /// The same day `months` months later (earlier when negative), with the
    /// day clamped to the length of the target month; clamped to the
    /// representable years.
    pub const fn add_months(self, months: i32) -> Date {
        let index = self.year as i64 * 12 + (self.month as i64 - 1) + months as i64;
        let year = index.div_euclid(12);
        if year < Self::MIN_YEAR as i64 {
            return Date::min();
        }
        if year > Self::MAX_YEAR as i64 {
            return Date::max();
        }
        let year = year as i32;
        let month = (index.rem_euclid(12) + 1) as u8;
        let last = days_in_month(year, month);
        let day = if self.day > last { last } else { self.day };
        Date { year, month, day }
    }

    /// The first day of this date's month.
    pub const fn first_of_month(self) -> Date {
        Date {
            year: self.year,
            month: self.month,
            day: 1,
        }
    }

    /// Whether `self` and `other` fall in the same month of the same year.
    pub const fn same_month(self, other: Date) -> bool {
        self.year == other.year && self.month == other.month
    }

    /// Today in UTC, from the system clock. Local time zones need a
    /// calendar library; pass the local date to the pickers' `today`
    /// setters when the difference matters.
    pub fn today_utc() -> Date {
        let secs = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(elapsed) => i64::try_from(elapsed.as_secs()).unwrap_or(i64::MAX),
            Err(before) => i64::try_from(before.duration().as_secs())
                .map(|s| -s)
                .unwrap_or(i64::MIN),
        };
        Date::from_days_since_epoch(secs.div_euclid(86_400))
    }
}

impl fmt::Display for Date {
    /// ISO 8601: `YYYY-MM-DD`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// A date range being picked: the start comes first, then the end.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DateRange {
    /// The first day of the range, once picked.
    pub start: Option<Date>,
    /// The last day of the range, once picked. Never before `start`.
    pub end: Option<Date>,
}

impl DateRange {
    /// A complete range, with `start` and `end` in either order.
    pub fn new(a: Date, b: Date) -> Self {
        Self {
            start: Some(a.min(b)),
            end: Some(a.max(b)),
        }
    }

    /// `(start, end)` when both ends are picked.
    pub fn get(&self) -> Option<(Date, Date)> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some((start, end)),
            (Some(_) | None, None) | (None, Some(_)) => None,
        }
    }

    /// Records a tap on `date`, the way Compose's `DateRangePicker` does: a
    /// date on or after a lone start completes the range, anything else
    /// starts a new one.
    pub fn select(&mut self, date: Date) {
        match (self.start, self.end) {
            (Some(start), None) if date >= start => self.end = Some(date),
            (Some(_) | None, Some(_) | None) => {
                self.start = Some(date);
                self.end = None;
            }
        }
    }

    /// Whether `date` lies inside the complete range (ends included).
    pub fn contains(&self, date: Date) -> bool {
        self.get()
            .is_some_and(|(start, end)| start <= date && date <= end)
    }
}

/// The order of the fields in a typed date.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DateOrder {
    /// Year, month, day (ISO 8601).
    #[default]
    YearMonthDay,
    /// Month, day, year.
    MonthDayYear,
    /// Day, month, year.
    DayMonthYear,
}

/// The pattern the input mode accepts and shows: a field order and a
/// separator, always with a four-digit year.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DateFormat {
    /// The field order.
    pub order: DateOrder,
    /// The character between fields.
    pub separator: char,
}

impl Default for DateFormat {
    /// `YYYY-MM-DD`.
    fn default() -> Self {
        Self {
            order: DateOrder::YearMonthDay,
            separator: '-',
        }
    }
}

/// Why typed text is not an acceptable date.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DateParseError {
    /// The text does not follow the pattern, or names a day that does not
    /// exist.
    Pattern,
    /// The year is outside the picker's year range.
    YearRange,
    /// The date exists but the picker does not allow selecting it.
    NotAllowed,
}

impl DateFormat {
    /// The pattern as shown to the user, such as `YYYY-MM-DD`.
    pub fn pattern(&self) -> String {
        let s = self.separator;
        match self.order {
            DateOrder::YearMonthDay => format!("YYYY{s}MM{s}DD"),
            DateOrder::MonthDayYear => format!("MM{s}DD{s}YYYY"),
            DateOrder::DayMonthYear => format!("DD{s}MM{s}YYYY"),
        }
    }

    /// `date` in this format, zero-padded.
    pub fn format(&self, date: Date) -> String {
        let s = self.separator;
        let (y, m, d) = (date.year(), date.month(), date.day());
        match self.order {
            DateOrder::YearMonthDay => format!("{y:04}{s}{m:02}{s}{d:02}"),
            DateOrder::MonthDayYear => format!("{m:02}{s}{d:02}{s}{y:04}"),
            DateOrder::DayMonthYear => format!("{d:02}{s}{m:02}{s}{y:04}"),
        }
    }

    /// Parses `text` (surrounding whitespace ignored): exactly three fields
    /// of ASCII digits, a four-digit year and a one- or two-digit month and
    /// day, naming a day that exists.
    pub fn parse(&self, text: &str) -> Result<Date, DateParseError> {
        let mut fields = text.trim().split(self.separator);
        let (Some(a), Some(b), Some(c), None) =
            (fields.next(), fields.next(), fields.next(), fields.next())
        else {
            return Err(DateParseError::Pattern);
        };
        let (year, month, day) = match self.order {
            DateOrder::YearMonthDay => (a, b, c),
            DateOrder::MonthDayYear => (c, a, b),
            DateOrder::DayMonthYear => (c, b, a),
        };
        let year = number(year, 4, 4).ok_or(DateParseError::Pattern)?;
        let month = number(month, 1, 2).ok_or(DateParseError::Pattern)?;
        let day = number(day, 1, 2).ok_or(DateParseError::Pattern)?;
        let (Ok(year), Ok(month), Ok(day)) =
            (i32::try_from(year), u8::try_from(month), u8::try_from(day))
        else {
            return Err(DateParseError::Pattern);
        };
        Date::new(year, month, day).ok_or(DateParseError::Pattern)
    }
}

/// `field` as a number when it is `min..=max` ASCII digits.
fn number(field: &str, min: usize, max: usize) -> Option<u32> {
    let len = field.chars().count();
    if len < min || len > max || !field.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    field.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u8, day: u8) -> Result<Date, String> {
        Date::new(y, m, day).ok_or_else(|| format!("{y}-{m}-{day} should exist"))
    }

    #[test]
    fn epoch_round_trip_over_four_centuries() -> Result<(), String> {
        let start = d(1900, 1, 1)?.days_since_epoch();
        for days in start..start + 146_097 {
            let date = Date::from_days_since_epoch(days);
            assert_eq!(date.days_since_epoch(), days, "round trip of {date}");
            assert!(
                Date::new(date.year(), date.month(), date.day()).is_some(),
                "{date} is valid"
            );
        }
        Ok(())
    }

    #[test]
    fn known_weekdays() -> Result<(), String> {
        assert_eq!(d(1970, 1, 1)?.weekday(), Weekday::Thursday, "epoch");
        assert_eq!(d(2000, 2, 29)?.weekday(), Weekday::Tuesday, "leap day 2000");
        assert_eq!(d(2024, 3, 5)?.weekday(), Weekday::Tuesday, "2024-03-05");
        assert_eq!(d(1900, 1, 1)?.weekday(), Weekday::Monday, "1900-01-01");
        Ok(())
    }

    #[test]
    fn rejects_days_that_do_not_exist() {
        assert!(Date::new(2023, 2, 29).is_none(), "2023 is not a leap year");
        assert!(Date::new(1900, 2, 29).is_none(), "1900 is not a leap year");
        assert!(Date::new(2000, 2, 29).is_some(), "2000 is a leap year");
        assert!(Date::new(2024, 13, 1).is_none(), "month 13");
        assert!(Date::new(2024, 4, 31).is_none(), "April 31st");
        assert!(Date::new(2024, 1, 0).is_none(), "day 0");
        assert!(Date::new(10_000, 1, 1).is_none(), "year beyond MAX_YEAR");
    }

    #[test]
    fn month_arithmetic_clamps_the_day() -> Result<(), String> {
        assert_eq!(d(2024, 1, 31)?.add_months(1), d(2024, 2, 29)?, "Jan 31 + 1");
        assert_eq!(
            d(2024, 3, 31)?.add_months(-1),
            d(2024, 2, 29)?,
            "Mar 31 - 1"
        );
        assert_eq!(d(2024, 12, 15)?.add_months(1), d(2025, 1, 15)?, "year wrap");
        assert_eq!(d(2024, 1, 15)?.add_months(-13), d(2022, 12, 15)?, "back 13");
        assert_eq!(Date::max().add_months(1), Date::max(), "clamped at max");
        assert_eq!(Date::min().add_days(-1), Date::min(), "clamped at min");
        assert_eq!(Date::max().add_days(i64::MAX), Date::max(), "no overflow");
        Ok(())
    }

    #[test]
    fn weekday_columns() {
        assert_eq!(Weekday::Sunday.days_since(Weekday::Sunday), 0, "Sun/Sun");
        assert_eq!(Weekday::Monday.days_since(Weekday::Sunday), 1, "Mon/Sun");
        assert_eq!(Weekday::Sunday.days_since(Weekday::Monday), 6, "Sun/Mon");
        assert_eq!(Weekday::Saturday.add(2), Weekday::Monday, "wraps");
    }

    #[test]
    fn range_selection_follows_compose() -> Result<(), String> {
        let mut range = DateRange::default();
        range.select(d(2024, 3, 10)?);
        assert_eq!(range.get(), None, "only a start");
        range.select(d(2024, 3, 5)?);
        assert_eq!(range.start, Some(d(2024, 3, 5)?), "earlier date restarts");
        range.select(d(2024, 3, 12)?);
        assert_eq!(
            range.get(),
            Some((d(2024, 3, 5)?, d(2024, 3, 12)?)),
            "complete"
        );
        assert!(range.contains(d(2024, 3, 8)?), "inside");
        range.select(d(2024, 3, 20)?);
        assert_eq!(range.get(), None, "a tap after a full range restarts it");
        Ok(())
    }

    #[test]
    fn parse_and_format() -> Result<(), String> {
        let iso = DateFormat::default();
        assert_eq!(iso.pattern(), "YYYY-MM-DD", "pattern");
        assert_eq!(iso.parse(" 2024-03-05 "), Ok(d(2024, 3, 5)?), "iso");
        assert_eq!(iso.parse("2024-3-5"), Ok(d(2024, 3, 5)?), "short fields");
        assert_eq!(iso.format(d(987, 1, 2)?), "0987-01-02", "padded");
        let us = DateFormat {
            order: DateOrder::MonthDayYear,
            separator: '/',
        };
        assert_eq!(us.parse("03/05/2024"), Ok(d(2024, 3, 5)?), "m/d/y");
        assert_eq!(us.format(d(2024, 3, 5)?), "03/05/2024", "m/d/y format");
        for bad in [
            "",
            "2024",
            "2024-03",
            "2024-03-05-01",
            "24-03-05",
            "2024-02-30",
            "2024-+3-05",
            "2024-03-0x",
            "２０２４-03-05",
            "2024--05",
        ] {
            assert_eq!(iso.parse(bad), Err(DateParseError::Pattern), "{bad:?}");
        }
        Ok(())
    }

    #[test]
    fn parse_never_panics_on_odd_input() {
        // A small deterministic fuzz: every string over a mixed alphabet up
        // to length 6 (multi-byte characters included).
        let alphabet = ['0', '9', '-', '/', ' ', 'é', '𝟘', '\u{0}'];
        let iso = DateFormat::default();
        let mut stack = vec![String::new()];
        while let Some(s) = stack.pop() {
            if let Ok(date) = iso.parse(&s) {
                assert!(
                    Date::new(date.year(), date.month(), date.day()).is_some(),
                    "{s:?} parsed to a valid date"
                );
            }
            if s.chars().count() < 6 {
                for c in alphabet {
                    let mut next = s.clone();
                    next.push(c);
                    stack.push(next);
                }
            }
        }
    }
}
