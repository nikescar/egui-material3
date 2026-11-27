// ============================================================================
// Module:       date_picker::tokens
// Description:  Date picker and date input token values from Compose.
//
// Dependencies: crate::tokens
// ============================================================================

//! Date picker tokens.
//!
//! Values from Compose's `DatePickerModalTokens.kt`,
//! `DateInputModalTokens.kt` and `OutlinedTextFieldTokens.kt` (baseline set),
//! plus the layout constants Compose keeps private in `DatePicker.kt`
//! (`RecommendedSizeForAccessibility`, `MonthYearHeight`,
//! `DatePickerHorizontalPadding`, `YearsVerticalPadding`, the title and
//! headline paddings). Compose has no docked token file; the docked picker
//! uses the modal tokens without the header, and the M3 docked container
//! shape `ShapeTokens.CornerLarge`. `DateInputModalTokens.ContainerColor`
//! is `Surface` (with a tint layer, an older scheme); Compose's
//! `DatePickerDialog` draws the input mode on
//! `DatePickerModalTokens.ContainerColor`, and so does this crate.

use crate::tokens::{ColorRole, Elevation, Shape, TypeRole};

/// `DatePickerModalTokens.ContainerColor`.
pub const CONTAINER_COLOR: ColorRole = ColorRole::SurfaceContainerHigh;
/// `DatePickerModalTokens.ContainerElevation`.
pub const CONTAINER_ELEVATION: Elevation = Elevation::Level3;
/// `DatePickerModalTokens.ContainerShape`.
pub const CONTAINER_SHAPE: Shape = Shape::EXTRA_LARGE;
/// `DatePickerModalTokens.ContainerWidth`.
pub const CONTAINER_WIDTH: f32 = 360.0;
/// The docked container shape (M3 docked date picker: `CornerLarge`).
pub const DOCKED_CONTAINER_SHAPE: Shape = Shape::LARGE;

/// `DatePickerModalTokens.HeaderContainerHeight`.
pub const HEADER_HEIGHT: f32 = 120.0;
/// `DatePickerModalTokens.RangeSelectionHeaderContainerHeight`.
pub const RANGE_HEADER_HEIGHT: f32 = 128.0;
/// `DatePickerModalTokens.HeaderHeadlineColor`.
pub const HEADER_HEADLINE_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// `DatePickerModalTokens.HeaderHeadlineFont`.
pub const HEADER_HEADLINE_FONT: TypeRole = TypeRole::HeadlineLarge;
/// `DatePickerModalTokens.RangeSelectionHeaderHeadlineFont`.
pub const RANGE_HEADER_HEADLINE_FONT: TypeRole = TypeRole::TitleLarge;
/// `DatePickerModalTokens.HeaderSupportingTextColor`.
pub const HEADER_SUPPORTING_TEXT_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// `DatePickerModalTokens.HeaderSupportingTextFont`.
pub const HEADER_SUPPORTING_TEXT_FONT: TypeRole = TypeRole::LabelLarge;
/// `DatePickerDefaults` divider colour (`DividerTokens.Color`).
pub const DIVIDER_COLOR: ColorRole = ColorRole::OutlineVariant;

/// `DatePickerModalTokens.DateContainerWidth` / `DateContainerHeight`.
pub const DATE_CONTAINER_SIZE: f32 = 40.0;
/// `DatePickerModalTokens.DateLabelTextFont`.
pub const DATE_LABEL_FONT: TypeRole = TypeRole::BodyLarge;
/// `DatePickerModalTokens.DateSelectedContainerColor`.
pub const DATE_SELECTED_CONTAINER_COLOR: ColorRole = ColorRole::Primary;
/// `DatePickerModalTokens.DateSelectedLabelTextColor`.
pub const DATE_SELECTED_LABEL_COLOR: ColorRole = ColorRole::OnPrimary;
/// `DatePickerModalTokens.DateTodayContainerOutlineColor`.
pub const DATE_TODAY_OUTLINE_COLOR: ColorRole = ColorRole::Primary;
/// `DatePickerModalTokens.DateTodayContainerOutlineWidth`.
pub const DATE_TODAY_OUTLINE_WIDTH: f32 = 1.0;
/// `DatePickerModalTokens.DateTodayLabelTextColor`.
pub const DATE_TODAY_LABEL_COLOR: ColorRole = ColorRole::Primary;
/// `DatePickerModalTokens.DateUnselectedLabelTextColor`.
pub const DATE_UNSELECTED_LABEL_COLOR: ColorRole = ColorRole::OnSurface;

/// `DatePickerModalTokens.RangeSelectionActiveIndicatorContainerColor`.
pub const RANGE_ACTIVE_INDICATOR_COLOR: ColorRole = ColorRole::SecondaryContainer;
/// `DatePickerModalTokens.RangeSelectionActiveIndicatorContainerHeight`.
pub const RANGE_ACTIVE_INDICATOR_HEIGHT: f32 = 40.0;
/// `DatePickerModalTokens.SelectionDateInRangeRangeLabelTextColor`.
pub const DATE_IN_RANGE_LABEL_COLOR: ColorRole = ColorRole::OnSecondaryContainer;

/// `DatePickerModalTokens.WeekdaysLabelTextColor`.
pub const WEEKDAYS_LABEL_COLOR: ColorRole = ColorRole::OnSurface;
/// `DatePickerModalTokens.WeekdaysLabelTextFont`.
pub const WEEKDAYS_LABEL_FONT: TypeRole = TypeRole::BodyLarge;

/// `DatePickerModalTokens.YearContainerWidth`.
pub const YEAR_CONTAINER_WIDTH: f32 = 72.0;
/// `DatePickerModalTokens.YearContainerHeight`.
pub const YEAR_CONTAINER_HEIGHT: f32 = 36.0;
/// `DatePickerModalTokens.YearLabelTextFont`.
pub const YEAR_LABEL_FONT: TypeRole = TypeRole::BodyLarge;
/// `DatePickerModalTokens.YearSelectedContainerColor`.
pub const YEAR_SELECTED_CONTAINER_COLOR: ColorRole = ColorRole::Primary;
/// `DatePickerModalTokens.YearSelectedLabelTextColor`.
pub const YEAR_SELECTED_LABEL_COLOR: ColorRole = ColorRole::OnPrimary;
/// `DatePickerModalTokens.YearUnselectedLabelTextColor`.
pub const YEAR_UNSELECTED_LABEL_COLOR: ColorRole = ColorRole::OnSurfaceVariant;

/// The month/year menu button and navigation icons: `DatePickerDefaults`
/// `subheadContentColor` / `navigationContentColor` (`OnSurfaceVariant`),
/// label in `LabelLarge`.
pub const NAVIGATION_CONTENT_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// The month/year menu button label font.
pub const MONTH_YEAR_FONT: TypeRole = TypeRole::LabelLarge;

/// `RecommendedSizeForAccessibility` in `DatePicker.kt`: one grid cell and
/// the weekday row height.
pub const CELL_SIZE: f32 = 48.0;
/// `MonthYearHeight` in `DatePicker.kt`.
pub const MONTH_YEAR_HEIGHT: f32 = 56.0;
/// `DatePickerHorizontalPadding` in `DatePicker.kt`.
pub const HORIZONTAL_PADDING: f32 = 12.0;
/// `YearsVerticalPadding` in `DatePicker.kt`.
pub const YEARS_VERTICAL_PADDING: f32 = 16.0;
/// Rows in a month grid (`MaxCalendarRows`).
pub const MAX_CALENDAR_ROWS: usize = 6;
/// Years per row in the year list (`YearsInRow`).
pub const YEARS_IN_ROW: usize = 3;
/// `DatePickerTitlePadding` / `DatePickerHeadlinePadding` start inset.
pub const HEADER_START_PADDING: f32 = 24.0;
/// `DatePickerTitlePadding` / `DatePickerHeadlinePadding` end inset.
pub const HEADER_END_PADDING: f32 = 12.0;
/// `DatePickerTitlePadding` top inset.
pub const TITLE_TOP_PADDING: f32 = 16.0;
/// `DatePickerHeadlinePadding` bottom inset.
pub const HEADLINE_BOTTOM_PADDING: f32 = 12.0;
/// Icon size of the navigation and mode-toggle icon buttons
/// (`IconButtonTokens.IconSize`).
pub const ICON_SIZE: f32 = 24.0;
/// Touch target of the icon buttons (`IconButtonTokens.StateLayerSize`
/// within the 48 minimum).
pub const ICON_BUTTON_SIZE: f32 = 48.0;

/// `DateInputModalTokens.ContainerWidth`.
pub const INPUT_CONTAINER_WIDTH: f32 = 328.0;
/// `DateInputModalTokens.HeaderContainerHeight`.
pub const INPUT_HEADER_HEIGHT: f32 = 120.0;
/// `InputTextFieldPadding` in `DateInput.kt`: start, end and top.
pub const INPUT_FIELD_PADDING: f32 = 24.0;
/// `InputTextFieldPadding` top inset.
pub const INPUT_FIELD_TOP_PADDING: f32 = 10.0;

/// `OutlinedTextFieldTokens` container height (`TextFieldMinHeight`: 56).
pub const FIELD_HEIGHT: f32 = 56.0;
/// `OutlinedTextFieldTokens.ContainerShape`.
pub const FIELD_SHAPE: Shape = Shape::EXTRA_SMALL;
/// `OutlinedTextFieldTokens.OutlineColor`.
pub const FIELD_OUTLINE_COLOR: ColorRole = ColorRole::Outline;
/// `OutlinedTextFieldTokens.OutlineWidth`.
pub const FIELD_OUTLINE_WIDTH: f32 = 1.0;
/// `OutlinedTextFieldTokens.FocusOutlineColor`.
pub const FIELD_FOCUS_OUTLINE_COLOR: ColorRole = ColorRole::Primary;
/// `OutlinedTextFieldTokens.FocusOutlineWidth`.
pub const FIELD_FOCUS_OUTLINE_WIDTH: f32 = 3.0;
/// `OutlinedTextFieldTokens.ErrorOutlineColor` (and error label and
/// supporting text colours).
pub const FIELD_ERROR_COLOR: ColorRole = ColorRole::Error;
/// `OutlinedTextFieldTokens.InputColor`.
pub const FIELD_INPUT_COLOR: ColorRole = ColorRole::OnSurface;
/// `OutlinedTextFieldTokens.InputFont`.
pub const FIELD_INPUT_FONT: TypeRole = TypeRole::BodyLarge;
/// `OutlinedTextFieldTokens.LabelColor`.
pub const FIELD_LABEL_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// The floating label font (`BodySmall`, Compose's label at its top
/// position).
pub const FIELD_FLOATING_LABEL_FONT: TypeRole = TypeRole::BodySmall;
/// `OutlinedTextFieldTokens.SupportingColor`.
pub const FIELD_SUPPORTING_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// `OutlinedTextFieldTokens.SupportingFont`.
pub const FIELD_SUPPORTING_FONT: TypeRole = TypeRole::BodySmall;
/// `TextFieldPadding` (start and end) in `TextFieldImpl.kt`.
pub const FIELD_HORIZONTAL_PADDING: f32 = 16.0;

/// `DialogButtonsPadding` in `DatePickerDialog.kt`: bottom inset.
pub const BUTTONS_BOTTOM_PADDING: f32 = 8.0;
/// `DialogButtonsPadding` end inset.
pub const BUTTONS_END_PADDING: f32 = 6.0;
/// `DialogButtonsMainAxisSpacing`.
pub const BUTTONS_SPACING: f32 = 8.0;
/// Height of the dialog's button row (text buttons are 40 high).
pub const BUTTONS_ROW_HEIGHT: f32 = 52.0;
