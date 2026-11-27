// ============================================================================
// Module:       time_picker::tokens
// Description:  Time picker and time input token values from Compose.
//
// Dependencies: crate::tokens
// ============================================================================

//! Time picker tokens.
//!
//! Values from Compose's `TimePickerTokens.kt` and `TimeInputTokens.kt`
//! (baseline set, `androidx-main`), plus the layout constants Compose keeps
//! private in `TimePicker.kt` (`OuterCircleToSizeRatio` = 101 dp / dial,
//! `InnerCircleToSizeRatio` = 69 dp / dial, `ClockDisplayBottomMargin`,
//! `ClockFaceBottomMargin`, `DisplaySeparatorWidth`, `SupportLabelTop`,
//! `PeriodTogglePaddingOld`). The M3 Expressive "vibrant" sizes are not
//! implemented.

use crate::tokens::{ColorRole, Elevation, Shape, TypeRole};

/// `TimePickerTokens.ContainerColor`.
pub const CONTAINER_COLOR: ColorRole = ColorRole::SurfaceContainerHigh;
/// `TimePickerTokens.ContainerElevation`.
pub const CONTAINER_ELEVATION: Elevation = Elevation::Level3;
/// `TimePickerTokens.ContainerShape`.
pub const CONTAINER_SHAPE: Shape = Shape::EXTRA_LARGE;
/// `TimePickerTokens.HeadlineColor`.
pub const HEADLINE_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// `TimePickerTokens.HeadlineFont`.
pub const HEADLINE_FONT: TypeRole = TypeRole::LabelMedium;

/// `TimePickerTokens.ClockDialColor`.
pub const DIAL_COLOR: ColorRole = ColorRole::SurfaceContainerHighest;
/// `TimePickerTokens.ClockDialContainerSize`.
pub const DIAL_SIZE: f32 = 256.0;
/// `TimePickerTokens.ClockDialLabelTextFont`.
pub const DIAL_LABEL_FONT: TypeRole = TypeRole::BodyLarge;
/// `TimePickerTokens.ClockDialSelectedLabelTextColor`.
pub const DIAL_SELECTED_LABEL_COLOR: ColorRole = ColorRole::OnPrimary;
/// `TimePickerTokens.ClockDialUnselectedLabelTextColor`.
pub const DIAL_UNSELECTED_LABEL_COLOR: ColorRole = ColorRole::OnSurface;
/// `TimePickerTokens.ClockDialSelectorCenterContainerColor`.
pub const SELECTOR_CENTER_COLOR: ColorRole = ColorRole::Primary;
/// `TimePickerTokens.ClockDialSelectorCenterContainerSize`.
pub const SELECTOR_CENTER_SIZE: f32 = 8.0;
/// `TimePickerTokens.ClockDialSelectorHandleContainerColor`.
pub const SELECTOR_HANDLE_COLOR: ColorRole = ColorRole::Primary;
/// `TimePickerTokens.ClockDialSelectorHandleContainerSize`.
pub const SELECTOR_HANDLE_SIZE: f32 = 48.0;
/// `TimePickerTokens.ClockDialSelectorTrackContainerColor`.
pub const SELECTOR_TRACK_COLOR: ColorRole = ColorRole::Primary;
/// `TimePickerTokens.ClockDialSelectorTrackContainerWidth`.
pub const SELECTOR_TRACK_WIDTH: f32 = 2.0;
/// Radius of the outer ring of labels: `OuterCircleToSizeRatio` (101 dp).
pub const OUTER_RADIUS: f32 = 101.0;
/// Radius of the inner (13–23 h) ring: `InnerCircleToSizeRatio` (69 dp).
pub const INNER_RADIUS: f32 = 69.0;

/// `TimePickerTokens.TimeSelectorContainerWidth`.
pub const SELECTOR_WIDTH: f32 = 96.0;
/// `TimePickerTokens.TimeSelector24HVerticalContainerWidth`.
pub const SELECTOR_24H_VERTICAL_WIDTH: f32 = 114.0;
/// `TimePickerTokens.TimeSelectorContainerHeight`.
pub const SELECTOR_HEIGHT: f32 = 80.0;
/// `TimePickerTokens.TimeSelectorContainerShape`.
pub const SELECTOR_SHAPE: Shape = Shape::SMALL;
/// `TimePickerTokens.TimeSelectorLabelTextFont`.
pub const SELECTOR_LABEL_FONT: TypeRole = TypeRole::DisplayLarge;
/// `TimePickerTokens.TimeSelectorSelectedContainerColor`.
pub const SELECTOR_SELECTED_CONTAINER_COLOR: ColorRole = ColorRole::PrimaryContainer;
/// `TimePickerTokens.TimeSelectorSelectedLabelTextColor`.
pub const SELECTOR_SELECTED_LABEL_COLOR: ColorRole = ColorRole::OnPrimaryContainer;
/// `TimePickerTokens.TimeSelectorUnselectedContainerColor`.
pub const SELECTOR_UNSELECTED_CONTAINER_COLOR: ColorRole = ColorRole::SurfaceContainerHighest;
/// `TimePickerTokens.TimeSelectorUnselectedLabelTextColor`.
pub const SELECTOR_UNSELECTED_LABEL_COLOR: ColorRole = ColorRole::OnSurface;
/// `TimePickerTokens.TimeSelectorSeparatorColor`.
pub const SEPARATOR_COLOR: ColorRole = ColorRole::OnSurface;
/// `TimePickerTokens.TimeSelectorSeparatorFont`.
pub const SEPARATOR_FONT: TypeRole = TypeRole::DisplayLarge;
/// `DisplaySeparatorWidth` in `TimePicker.kt`.
pub const SEPARATOR_WIDTH: f32 = 24.0;

/// `TimePickerTokens.PeriodSelectorContainerShape`.
pub const PERIOD_SHAPE: Shape = Shape::SMALL;
/// `TimePickerTokens.PeriodSelectorVerticalContainerWidth`.
pub const PERIOD_VERTICAL_WIDTH: f32 = 52.0;
/// `TimePickerTokens.PeriodSelectorVerticalContainerHeight`.
pub const PERIOD_VERTICAL_HEIGHT: f32 = 80.0;
/// `TimePickerTokens.PeriodSelectorHorizontalContainerWidth`.
pub const PERIOD_HORIZONTAL_WIDTH: f32 = 216.0;
/// `TimePickerTokens.PeriodSelectorHorizontalContainerHeight`.
pub const PERIOD_HORIZONTAL_HEIGHT: f32 = 38.0;
/// `TimePickerTokens.PeriodSelectorLabelTextFont`.
pub const PERIOD_LABEL_FONT: TypeRole = TypeRole::TitleMedium;
/// `TimePickerTokens.PeriodSelectorOutlineColor`.
pub const PERIOD_OUTLINE_COLOR: ColorRole = ColorRole::Outline;
/// `TimePickerTokens.PeriodSelectorOutlineWidth`.
pub const PERIOD_OUTLINE_WIDTH: f32 = 1.0;
/// `TimePickerTokens.PeriodSelectorSelectedContainerColor`.
pub const PERIOD_SELECTED_CONTAINER_COLOR: ColorRole = ColorRole::TertiaryContainer;
/// `TimePickerTokens.PeriodSelectorSelectedLabelTextColor`.
pub const PERIOD_SELECTED_LABEL_COLOR: ColorRole = ColorRole::OnTertiaryContainer;
/// `TimePickerTokens.PeriodSelectorUnselectedLabelTextColor`.
pub const PERIOD_UNSELECTED_LABEL_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// `PeriodTogglePaddingOld` in `TimePicker.kt`: gap before the period
/// selector.
pub const PERIOD_GAP: f32 = 12.0;

/// `ClockDisplayBottomMargin` in `TimePicker.kt` (also the horizontal
/// layout's gap between display and dial).
pub const DISPLAY_BOTTOM_MARGIN: f32 = 36.0;
/// `ClockFaceBottomMargin` in `TimePicker.kt`.
pub const DIAL_BOTTOM_MARGIN: f32 = 24.0;

/// `TimeInputTokens.TimeFieldContainerWidth`.
pub const FIELD_WIDTH: f32 = 96.0;
/// `TimeInputTokens.TimeFieldContainerHeight`.
pub const FIELD_HEIGHT: f32 = 72.0;
/// `TimeInputTokens.TimeFieldContainerShape`.
pub const FIELD_SHAPE: Shape = Shape::SMALL;
/// `TimeInputTokens.TimeFieldContainerColor`.
pub const FIELD_CONTAINER_COLOR: ColorRole = ColorRole::SurfaceContainerHighest;
/// `TimeInputTokens.TimeFieldFocusContainerColor`.
pub const FIELD_FOCUS_CONTAINER_COLOR: ColorRole = ColorRole::PrimaryContainer;
/// `TimeInputTokens.TimeFieldFocusLabelTextColor`.
pub const FIELD_FOCUS_LABEL_COLOR: ColorRole = ColorRole::OnPrimaryContainer;
/// `TimeInputTokens.TimeFieldFocusOutlineColor`.
pub const FIELD_FOCUS_OUTLINE_COLOR: ColorRole = ColorRole::Primary;
/// `TimeInputTokens.TimeFieldFocusOutlineWidth`.
pub const FIELD_FOCUS_OUTLINE_WIDTH: f32 = 2.0;
/// `TimeInputTokens.TimeFieldLabelTextColor`.
pub const FIELD_LABEL_COLOR: ColorRole = ColorRole::OnSurface;
/// `TimeInputTokens.TimeFieldLabelTextFont`.
pub const FIELD_LABEL_FONT: TypeRole = TypeRole::DisplayMedium;
/// `TimeInputTokens.TimeFieldSeparatorFont`.
pub const FIELD_SEPARATOR_FONT: TypeRole = TypeRole::DisplayLarge;
/// `TimeInputTokens.TimeFieldSupportingTextColor`.
pub const FIELD_SUPPORTING_COLOR: ColorRole = ColorRole::OnSurfaceVariant;
/// `TimeInputTokens.TimeFieldSupportingTextFont`.
pub const FIELD_SUPPORTING_FONT: TypeRole = TypeRole::BodySmall;
/// `SupportLabelTop` in `TimePicker.kt`.
pub const SUPPORT_LABEL_TOP: f32 = 7.0;
/// `TimeInputTokens.PeriodSelectorContainerHeight`.
pub const INPUT_PERIOD_HEIGHT: f32 = 72.0;
/// `TimeInputTokens.PeriodSelectorContainerWidth`.
pub const INPUT_PERIOD_WIDTH: f32 = 52.0;

/// The dialog's content padding (`contentPadding` in `TimePickerDialog.kt`).
pub const DIALOG_PADDING: f32 = 24.0;
