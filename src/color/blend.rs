// ============================================================================
// Module:       color::blend
// Description:  Harmonising and blending colours in HCT and CAM16-UCS.
//
// Dependencies: super::{cam16, hct, utils}
// ============================================================================

//! Colour blending: shifting a brand or semantic colour's hue towards the
//! scheme's source so it sits with the theme ([`harmonize`]).
//!
//! Ported from Material Color Utilities (`blend/blend.ts`; Apache-2.0,
//! Copyright 2021 Google LLC).

use super::cam16::Cam16;
use super::hct::Hct;
use super::utils;

/// Rotates `design_color`'s hue towards `source_color`'s by half their
/// difference, at most 15°, keeping its chroma and tone (MCU's
/// `Blend.harmonize`).
///
/// ```
/// use egui_material3::color::blend::harmonize;
/// // Red harmonised towards blue moves its hue, keeps its tone.
/// let red = 0xFFFF_0000;
/// let harmonized = harmonize(red, 0xFF00_00FF);
/// assert_ne!(harmonized, red);
/// ```
pub fn harmonize(design_color: u32, source_color: u32) -> u32 {
    let from = Hct::from_argb(design_color);
    let to = Hct::from_argb(source_color);
    let difference = utils::difference_degrees(from.hue(), to.hue());
    let rotation = f64::min(difference * 0.5, 15.0);
    let output_hue = utils::sanitize_degrees_double(
        from.hue() + rotation * utils::rotation_direction(from.hue(), to.hue()),
    );
    Hct::from(output_hue, from.chroma(), from.tone()).to_argb()
}

/// Blends `from`'s hue towards `to`'s by `amount` (0..1) in CAM16-UCS,
/// keeping `from`'s chroma and tone (MCU's `Blend.hctHue`).
pub fn hct_hue(from: u32, to: u32, amount: f64) -> u32 {
    let ucs = cam16_ucs(from, to, amount);
    let ucs_cam = Cam16::from_argb(ucs);
    let from_cam = Cam16::from_argb(from);
    Hct::from(ucs_cam.hue, from_cam.chroma, utils::lstar_from_argb(from)).to_argb()
}

/// Blends `from` towards `to` by `amount` (0..1) in CAM16-UCS (MCU's
/// `Blend.cam16Ucs`).
pub fn cam16_ucs(from: u32, to: u32, amount: f64) -> u32 {
    let from_cam = Cam16::from_argb(from);
    let to_cam = Cam16::from_argb(to);
    let jstar = from_cam.jstar + (to_cam.jstar - from_cam.jstar) * amount;
    let astar = from_cam.astar + (to_cam.astar - from_cam.astar) * amount;
    let bstar = from_cam.bstar + (to_cam.bstar - from_cam.bstar) * amount;
    Cam16::from_ucs(jstar, astar, bstar).to_argb()
}

#[cfg(test)]
mod tests {
    use super::*;

    // MCU's `blend_test` "harmonize".
    #[test]
    fn harmonize_matches_mcu() {
        let cases: [(u32, u32, u32); 6] = [
            (0xFFFF_0000, 0xFF00_00FF, 0xFFFB_0057),
            (0xFFFF_0000, 0xFF00_FF00, 0xFFD8_5600),
            (0xFFFF_0000, 0xFFFF_FF00, 0xFFD8_5600),
            (0xFF00_00FF, 0xFF00_FF00, 0xFF00_47A3),
            (0xFF00_00FF, 0xFFFF_0000, 0xFF57_00DC),
            (0xFF00_00FF, 0xFFFF_FF00, 0xFF00_47A3),
        ];
        for (design, source, expected) in cases {
            let got = harmonize(design, source);
            assert_eq!(
                got, expected,
                "{design:08X} towards {source:08X}: {got:08X}"
            );
        }
    }
}
