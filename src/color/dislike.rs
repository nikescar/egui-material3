// ============================================================================
// Module:       color::dislike
// Description:  Detects and fixes universally disliked dark yellow-greens.
//
// Dependencies: super::{hct, utils}
// ============================================================================

//! Dark yellow-greens ("bile") are widely disliked; MCU lightens them.
//!
//! Ported from Material Color Utilities (`dislike/dislike_analyzer.ts`;
//! Apache-2.0, Copyright 2023 Google LLC).

use super::hct::Hct;
use super::utils::js_round;

/// Whether `hct` is in the disliked range: hue 90..=111, chroma > 16,
/// tone < 65 (each rounded first, as MCU does).
pub fn is_disliked(hct: Hct) -> bool {
    let hue = js_round(hct.hue());
    let hue_passes = (90.0..=111.0).contains(&hue);
    let chroma_passes = js_round(hct.chroma()) > 16.0;
    let tone_passes = js_round(hct.tone()) < 65.0;
    hue_passes && chroma_passes && tone_passes
}

/// `hct` lightened to tone 70 if it is disliked, else `hct` unchanged.
pub fn fix_if_disliked(hct: Hct) -> Hct {
    if is_disliked(hct) {
        Hct::from(hct.hue(), hct.chroma(), 70.0)
    } else {
        hct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MCU's `dislike_analyzer_test`.
    #[test]
    fn bile_is_disliked_and_fixed() {
        for argb in [
            0xFF95_884B_u32,
            0xFF71_6B40,
            0xFFB0_8E00,
            0xFF4C_4308,
            0xFF46_4521,
        ] {
            let hct = Hct::from_argb(argb);
            assert!(is_disliked(hct), "{argb:08X} should be disliked");
            assert!(
                !is_disliked(fix_if_disliked(hct)),
                "{argb:08X} fixed is still disliked"
            );
        }
    }

    #[test]
    fn liked_colours_are_left_alone() {
        for argb in [0xFF1B_7E22_u32, 0xFF6C_A734, 0xFF67_50A4] {
            let hct = Hct::from_argb(argb);
            assert!(!is_disliked(hct), "{argb:08X} should not be disliked");
            assert_eq!(fix_if_disliked(hct), hct, "{argb:08X} should be unchanged");
        }
    }
}
