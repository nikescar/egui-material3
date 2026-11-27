// ============================================================================
// Module:       color::hct
// Description:  HCT (hue, chroma, tone): the colour space dynamic colour uses.
//
// Dependencies: egui; super::{cam16, hct_solver, utils}
// ============================================================================

//! HCT: CAM16 hue and chroma with L* as tone.
//!
//! Tone is what contrast is measured in (two tones 40 apart are at least
//! 3:1, 50 apart at least 4.5:1), which is why Material's scheme rules are
//! written in it. Ported from Material Color Utilities (`hct/hct.ts`;
//! Apache-2.0, Copyright 2021 Google LLC).

use egui::Color32;

use super::cam16::{Cam16, ViewingConditions};
use super::{hct_solver, utils};

/// A colour as hue (0..360), chroma (0..~150) and tone (0..100).
///
/// Constructed from a colour, it is exact. Constructed from three values
/// ([`Hct::from`]), the chroma is the closest sRGB allows at that hue and
/// tone, so [`Hct::chroma`] may be lower than requested.
///
/// ```
/// use egui_material3::color::Hct;
/// let purple = Hct::from_argb(0xFF67_50A4);
/// assert_eq!(Hct::from(purple.hue(), purple.chroma(), purple.tone()).to_argb(), 0xFF67_50A4);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hct {
    hue: f64,
    chroma: f64,
    tone: f64,
    argb: u32,
}

impl Hct {
    /// The sRGB colour nearest the requested hue, chroma and tone.
    pub fn from(hue: f64, chroma: f64, tone: f64) -> Self {
        Self::from_argb(hct_solver::solve_to_argb(hue, chroma, tone))
    }

    /// The HCT of a packed `0xAARRGGBB` colour (alpha is ignored).
    pub fn from_argb(argb: u32) -> Self {
        let cam = Cam16::from_argb(argb);
        Self {
            hue: cam.hue,
            chroma: cam.chroma,
            tone: utils::lstar_from_argb(argb),
            argb,
        }
    }

    /// The HCT of an egui colour (alpha is ignored).
    pub fn from_color32(color: Color32) -> Self {
        Self::from_argb(utils::argb_from_rgb(color.r(), color.g(), color.b()))
    }

    /// Hue in degrees, `0.0..360.0`.
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Chroma: 0 is grey.
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Tone (L*): 0 is black, 100 is white.
    pub fn tone(&self) -> f64 {
        self.tone
    }

    /// The colour as packed opaque `0xFFRRGGBB`.
    pub fn to_argb(&self) -> u32 {
        self.argb
    }

    /// The colour as an opaque egui colour.
    pub fn to_color32(&self) -> Color32 {
        argb_to_color32(self.argb)
    }

    /// This colour with another hue (chroma and tone kept where sRGB allows).
    pub fn with_hue(&self, hue: f64) -> Self {
        Self::from(hue, self.chroma, self.tone)
    }

    /// This colour with another chroma.
    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self::from(self.hue, chroma, self.tone)
    }

    /// This colour with another tone.
    pub fn with_tone(&self, tone: f64) -> Self {
        Self::from(self.hue, self.chroma, tone)
    }

    /// Whether `hue` is in MCU's blue band (250..270).
    pub fn is_blue(hue: f64) -> bool {
        (250.0..270.0).contains(&hue)
    }

    /// Whether `hue` is in MCU's yellow band (105..125).
    pub fn is_yellow(hue: f64) -> bool {
        (105.0..125.0).contains(&hue)
    }

    /// Whether `hue` is in MCU's cyan band (170..207).
    pub fn is_cyan(hue: f64) -> bool {
        (170.0..207.0).contains(&hue)
    }

    /// This colour as it appears in `vc`, re-expressed in the default
    /// viewing conditions (MCU's `inViewingConditions`).
    pub fn in_viewing_conditions(&self, vc: &ViewingConditions) -> Self {
        let cam = Cam16::from_argb(self.argb);
        let [x, y, z] = cam.xyz_in(vc);
        let recast = Cam16::from_xyz_in(x, y, z, &ViewingConditions::default());
        Self::from(recast.hue, recast.chroma, utils::lstar_from_y(y))
    }
}

/// A packed `0xAARRGGBB` colour as an opaque egui colour.
pub fn argb_to_color32(argb: u32) -> Color32 {
    let [_, r, g, b] = argb.to_be_bytes();
    Color32::from_rgb(r, g, b)
}

/// An egui colour as packed opaque `0xFFRRGGBB` (alpha is dropped).
pub fn color32_to_argb(color: Color32) -> u32 {
    utils::argb_from_rgb(color.r(), color.g(), color.b())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_8_bit_grey_and_a_colour_lattice_round_trip() {
        // MCU's `hct_test` "HCT preserves original color": every colour
        // survives argb -> HCT -> argb through the solver.
        for r in (0..=255u8).step_by(15) {
            for g in (0..=255u8).step_by(15) {
                for b in (0..=255u8).step_by(15) {
                    let argb = utils::argb_from_rgb(r, g, b);
                    let hct = Hct::from_argb(argb);
                    let back = Hct::from(hct.hue(), hct.chroma(), hct.tone()).to_argb();
                    assert_eq!(back, argb, "{argb:08X} came back as {back:08X}");
                }
            }
        }
    }

    #[test]
    fn requested_values_are_honoured_in_gamut() {
        // MCU's `hct_test` "HCT returns a sufficiently close color".
        for hue in (15..360).step_by(30) {
            for chroma in [0.0, 10.0, 30.0, 50.0] {
                for tone in [20.0, 40.0, 60.0, 80.0] {
                    let hue = f64::from(hue);
                    let hct = Hct::from(hue, chroma, tone);
                    if chroma > 0.0 {
                        assert!(
                            utils::difference_degrees(hct.hue(), hue) <= 4.0,
                            "hue {hue} c {chroma} t {tone}: got hue {}",
                            hct.hue()
                        );
                    }
                    assert!(
                        hct.chroma() <= chroma + 2.5,
                        "hue {hue} c {chroma} t {tone}: chroma {} overshoots",
                        hct.chroma()
                    );
                    assert!(
                        (hct.tone() - tone).abs() <= 0.5,
                        "hue {hue} c {chroma} t {tone}: tone {}",
                        hct.tone()
                    );
                }
            }
        }
    }
}
