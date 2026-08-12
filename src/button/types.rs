//! Type definitions for Material Design 3 Button Components

use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Shadow, Stroke},
    Image, Sense, TextWrapMode, Vec2, WidgetText,
};

/// Material Design button variants following Material Design 3 specifications
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialButtonVariant {
    /// Filled button - High emphasis, filled background with primary color
    Filled,
    /// Outlined button - Medium emphasis, transparent background with outline
    Outlined,
    /// Text button - Low emphasis, transparent background, no outline  
    Text,
    /// Elevated button - Medium emphasis, filled background with shadow elevation
    Elevated,
    /// Filled tonal button - Medium emphasis, filled background with secondary container color
    FilledTonal,
}

/// Material Design button widget implementing Material Design 3 button specifications
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialButton<'a> {
    pub(crate) image: Option<Image<'a>>,
    pub(crate) text: Option<WidgetText>,
    pub(crate) shortcut_text: WidgetText,
    pub(crate) wrap_mode: Option<TextWrapMode>,
    pub(crate) variant: MaterialButtonVariant,
    pub(crate) fill: Option<Color32>,
    pub(crate) stroke: Option<Stroke>,
    pub(crate) sense: Sense,
    pub(crate) small: bool,
    pub(crate) frame: Option<bool>,
    pub(crate) min_size: Vec2,
    pub(crate) corner_radius: Option<CornerRadius>,
    pub(crate) selected: bool,
    pub(crate) image_tint_follows_text_color: bool,
    pub(crate) elevation: Option<Shadow>,
    pub(crate) disabled: bool,
    pub(crate) leading_icon: Option<String>,
    pub(crate) trailing_icon: Option<String>,
    pub(crate) leading_svg: Option<String>,
    pub(crate) trailing_svg: Option<String>,
    pub(crate) text_color: Option<Color32>,
}
