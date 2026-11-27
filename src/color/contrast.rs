// ============================================================================
// Module:       color::contrast
// Description:  WCAG contrast ratios between tones, and tones that reach one.
//
// Dependencies: super::utils
// ============================================================================

//! Contrast ratios in tone (L*) terms.
//!
//! Ported from Material Color Utilities (`contrast/contrast.ts`;
//! Apache-2.0, Copyright 2022 Google LLC). Ratios are WCAG 2.1 (1..21).

use super::utils;

/// The WCAG contrast ratio of two tones (each clamped to 0..100).
///
/// ```
/// use egui_material3::color::contrast::ratio_of_tones;
/// assert!((ratio_of_tones(0.0, 100.0) - 21.0).abs() < 1e-9);
/// ```
pub fn ratio_of_tones(tone_a: f64, tone_b: f64) -> f64 {
    let a = utils::clamp_double(0.0, 100.0, tone_a);
    let b = utils::clamp_double(0.0, 100.0, tone_b);
    ratio_of_ys(utils::y_from_lstar(a), utils::y_from_lstar(b))
}

/// The WCAG contrast ratio of two relative luminances (0..100).
pub fn ratio_of_ys(y1: f64, y2: f64) -> f64 {
    let lighter = if y1 > y2 { y1 } else { y2 };
    let darker = if lighter == y2 { y1 } else { y2 };
    (lighter + 5.0) / (darker + 5.0)
}

/// The tone lighter than `tone` with at least `ratio` contrast, or `None`
/// if there is none.
pub fn lighter(tone: f64, ratio: f64) -> Option<f64> {
    if !(0.0..=100.0).contains(&tone) {
        return None;
    }
    let dark_y = utils::y_from_lstar(tone);
    let light_y = ratio * (dark_y + 5.0) - 5.0;
    let real_contrast = ratio_of_ys(light_y, dark_y);
    let delta = (real_contrast - ratio).abs();
    if real_contrast < ratio && delta > 0.04 {
        return None;
    }
    // MCU adds 0.4 so the result still passes after rounding to 8 bits.
    let value = utils::lstar_from_y(light_y) + 0.4;
    (0.0..=100.0).contains(&value).then_some(value)
}

/// The tone darker than `tone` with at least `ratio` contrast, or `None`
/// if there is none.
pub fn darker(tone: f64, ratio: f64) -> Option<f64> {
    if !(0.0..=100.0).contains(&tone) {
        return None;
    }
    let light_y = utils::y_from_lstar(tone);
    let dark_y = ((light_y + 5.0) / ratio) - 5.0;
    let real_contrast = ratio_of_ys(light_y, dark_y);
    let delta = (real_contrast - ratio).abs();
    if real_contrast < ratio && delta > 0.04 {
        return None;
    }
    let value = utils::lstar_from_y(dark_y) - 0.4;
    (0.0..=100.0).contains(&value).then_some(value)
}

/// [`lighter`], or 100 when no tone is light enough.
pub fn lighter_unsafe(tone: f64, ratio: f64) -> f64 {
    lighter(tone, ratio).unwrap_or(100.0)
}

/// [`darker`], or 0 when no tone is dark enough.
pub fn darker_unsafe(tone: f64, ratio: f64) -> f64 {
    darker(tone, ratio).unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // MCU's `contrast_test`.
    #[test]
    fn ratio_of_tones_matches_mcu() {
        assert!(
            (ratio_of_tones(100.0, 0.0) - 21.0).abs() < 0.001,
            "white on black"
        );
        assert!(
            (ratio_of_tones(0.0, 100.0) - 21.0).abs() < 0.001,
            "is symmetric"
        );
        assert!(
            (ratio_of_tones(50.0, 50.0) - 1.0).abs() < 0.001,
            "equal tones"
        );
    }

    #[test]
    fn lighter_and_darker_match_mcu() {
        // The answer clears the ratio, by no more than MCU's 0.4 tone margin.
        let l = lighter(10.0, 7.0).unwrap_or(-1.0);
        let r = ratio_of_tones(10.0, l);
        assert!(
            r >= 7.0 && ratio_of_tones(10.0, l - 0.5) < 7.0,
            "lighter(10, 7) = {l} at {r}:1"
        );
        let d = darker(90.0, 7.0).unwrap_or(-1.0);
        let r = ratio_of_tones(90.0, d);
        assert!(
            r >= 7.0 && ratio_of_tones(90.0, d + 0.5) < 7.0,
            "darker(90, 7) = {d} at {r}:1"
        );
        // MCU's `contrast_test`.
        assert!(
            (ratio_of_tones(-10.0, 110.0) - 21.0).abs() < 0.001,
            "out-of-range tones clamp"
        );
        assert_eq!(lighter(90.0, 10.0), None, "impossible ratio");
        assert_eq!(lighter(110.0, 2.0), None, "tone above range");
        assert_eq!(lighter(-10.0, 2.0), None, "tone below range");
        assert_eq!(
            lighter_unsafe(100.0, 2.0),
            100.0,
            "unsafe returns the max tone"
        );
        assert_eq!(darker(10.0, 20.0), None, "impossible ratio");
        assert_eq!(darker(110.0, 2.0), None, "tone above range");
        assert_eq!(darker(-10.0, 2.0), None, "tone below range");
        assert_eq!(darker_unsafe(0.0, 2.0), 0.0, "unsafe returns the min tone");
    }
}
