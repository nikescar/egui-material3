// ============================================================================
// Module:       color::tonal_palette
// Description:  A hue and chroma, from which every tone 0..100 is derived.
//
// Dependencies: egui; super::hct
// ============================================================================

//! [`TonalPalette`]: one hue and chroma at every tone.
//!
//! Ported from Material Color Utilities (`palettes/tonal_palette.ts`;
//! Apache-2.0, Copyright 2021 Google LLC), including the key-colour search
//! (the tone nearest 50 that reaches the requested chroma).

use egui::Color32;

use super::hct::{argb_to_color32, Hct};

/// A palette of tones sharing one hue and chroma.
///
/// ```
/// use egui_material3::color::TonalPalette;
/// let palette = TonalPalette::from_argb(0xFF67_50A4);
/// assert_eq!(palette.tone(100.0), 0xFFFF_FFFF);
/// assert_eq!(palette.tone(0.0), 0xFF00_0000);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TonalPalette {
    hue: f64,
    chroma: f64,
    key_color: Hct,
}

impl TonalPalette {
    /// The palette of a colour's hue and chroma; the colour is the key colour.
    pub fn from_argb(argb: u32) -> Self {
        Self::from_hct(Hct::from_argb(argb))
    }

    /// The palette of an HCT colour's hue and chroma.
    pub fn from_hct(hct: Hct) -> Self {
        Self {
            hue: hct.hue(),
            chroma: hct.chroma(),
            key_color: hct,
        }
    }

    /// The palette of `hue` at `chroma`. The key colour is the tone closest
    /// to 50 at which sRGB reaches `chroma`.
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
        Self {
            hue,
            chroma,
            key_color: key_color(hue, chroma),
        }
    }

    /// The hue.
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// The requested chroma (tones far from 50 may have less).
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// The key colour.
    pub fn key_color(&self) -> Hct {
        self.key_color
    }

    /// The colour at `tone` (0..100) as packed `0xFFRRGGBB`.
    pub fn tone(&self, tone: f64) -> u32 {
        if tone == 99.0 && Hct::is_yellow(self.hue) {
            // MCU averages tones 98 and 100 here: at 99 the yellow band's
            // solver result is visibly greener than its neighbours.
            return average_argb(self.tone(98.0), self.tone(100.0));
        }
        Hct::from(self.hue, self.chroma, tone).to_argb()
    }

    /// The colour at `tone` as HCT.
    pub fn hct(&self, tone: f64) -> Hct {
        Hct::from_argb(self.tone(tone))
    }

    /// The colour at `tone` as an egui colour.
    pub fn color32(&self, tone: f64) -> Color32 {
        argb_to_color32(self.tone(tone))
    }
}

fn average_argb(a: u32, b: u32) -> u32 {
    let [_, r1, g1, b1] = a.to_be_bytes();
    let [_, r2, g2, b2] = b.to_be_bytes();
    // Rounding the half up is JavaScript's `Math.round((x + y) / 2)` for
    // non-negative integers, and stays within u8.
    let avg = |x: u8, y: u8| (u16::from(x) + u16::from(y)).div_ceil(2).to_be_bytes();
    let [_, r] = avg(r1, r2);
    let [_, g] = avg(g1, g2);
    let [_, bl] = avg(b1, b2);
    u32::from_be_bytes([0xFF, r, g, bl])
}

/// MCU's `KeyColor.create`: a binary search for the tone nearest 50 that
/// still reaches `requested_chroma`, or the tone of maximum chroma.
fn key_color(hue: f64, requested_chroma: f64) -> Hct {
    const PIVOT_TONE: i32 = 50;
    const TONE_STEP: i32 = 1;
    const EPSILON: f64 = 0.01;
    const MAX_CHROMA_VALUE: f64 = 200.0;

    let mut cache: [Option<f64>; 102] = [None; 102];
    let mut max_chroma = |tone: i32| -> f64 {
        let slot = usize::try_from(tone).ok().and_then(|i| cache.get_mut(i));
        if let Some(Some(chroma)) = slot.as_deref() {
            return *chroma;
        }
        let chroma = Hct::from(hue, MAX_CHROMA_VALUE, f64::from(tone)).chroma();
        if let Some(slot) = slot {
            *slot = Some(chroma);
        }
        chroma
    };

    let mut lower_tone = 0;
    let mut upper_tone = 100;
    while lower_tone < upper_tone {
        let mid_tone = (lower_tone + upper_tone) / 2;
        let is_ascending = max_chroma(mid_tone) < max_chroma(mid_tone + TONE_STEP);
        let sufficient_chroma = max_chroma(mid_tone) >= requested_chroma - EPSILON;
        if sufficient_chroma {
            if (lower_tone - PIVOT_TONE).abs() < (upper_tone - PIVOT_TONE).abs() {
                upper_tone = mid_tone;
            } else {
                if lower_tone == mid_tone {
                    return Hct::from(hue, requested_chroma, f64::from(lower_tone));
                }
                lower_tone = mid_tone;
            }
        } else if is_ascending {
            lower_tone = mid_tone + TONE_STEP;
        } else {
            upper_tone = mid_tone;
        }
    }
    Hct::from(hue, requested_chroma, f64::from(lower_tone))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_palette_of_baseline_purple_matches_mcu() {
        // MCU's `scheme_test` "3rd party light/dark scheme": SchemeAndroid reads
        // CorePalette.of(#6750A4) (a1 = hue at max(48, chroma), a2 = hue at 16,
        // a3 = hue + 60 at 24, n1 = hue at 4).
        let seed = Hct::from_argb(0xFF67_50A4);
        let hue = seed.hue();
        let a1 = TonalPalette::from_hue_and_chroma(hue, seed.chroma().max(48.0));
        let a2 = TonalPalette::from_hue_and_chroma(hue, 16.0);
        let a3 = TonalPalette::from_hue_and_chroma((hue + 60.0) % 360.0, 24.0);
        let n1 = TonalPalette::from_hue_and_chroma(hue, 4.0);
        let cases: [(&TonalPalette, f64, u32); 7] = [
            (&a1, 90.0, 0xFFE9_DDFF),
            (&a2, 90.0, 0xFFE8_DEF8),
            (&a3, 90.0, 0xFFFF_D9E3),
            (&n1, 98.0, 0xFFFD_F8FD),
            (&n1, 10.0, 0xFF1C_1B1E),
            (&n1, 20.0, 0xFF31_3033),
            (&n1, 95.0, 0xFFF4_EFF4),
        ];
        for (palette, tone, argb) in cases {
            let got = palette.tone(tone);
            assert_eq!(got, argb, "tone {tone}: {got:08X}, expected {argb:08X}");
        }
    }

    #[test]
    fn core_palette_secondary_of_blue_matches_mcu() {
        // MCU's `palettes_test` "CorePalette ofBlue" a2.
        let a2 = TonalPalette::from_hue_and_chroma(Hct::from_argb(0xFF00_00FF).hue(), 16.0);
        let expected: [(f64, u32); 12] = [
            (100.0, 0xFFFF_FFFF),
            (95.0, 0xFFF1_EFFF),
            (90.0, 0xFFE1_E0F9),
            (80.0, 0xFFC5_C4DD),
            (70.0, 0xFFA9_A9C1),
            (60.0, 0xFF8F_8FA6),
            (50.0, 0xFF75_758B),
            (40.0, 0xFF5C_5D72),
            (30.0, 0xFF44_4559),
            (20.0, 0xFF2E_2F42),
            (10.0, 0xFF19_1A2C),
            (0.0, 0xFF00_0000),
        ];
        for (tone, argb) in expected {
            let got = a2.tone(tone);
            assert_eq!(got, argb, "tone {tone}: {got:08X}, expected {argb:08X}");
        }
    }

    #[test]
    fn blue_tones_match_mcu() {
        // MCU's `palettes_test` "tones of blue".
        let palette = TonalPalette::from_argb(0xFF00_00FF);
        let expected: [(f64, u32); 12] = [
            (100.0, 0xFFFF_FFFF),
            (95.0, 0xFFF1_EFFF),
            (90.0, 0xFFE0_E0FF),
            (80.0, 0xFFBE_C2FF),
            (70.0, 0xFF9D_A3FF),
            (60.0, 0xFF7C_84FF),
            (50.0, 0xFF5A_64FF),
            (40.0, 0xFF34_3DFF),
            (30.0, 0xFF00_00EF),
            (20.0, 0xFF00_01AC),
            (10.0, 0xFF00_006E),
            (0.0, 0xFF00_0000),
        ];
        for (tone, argb) in expected {
            let got = palette.tone(tone);
            assert_eq!(got, argb, "tone {tone}: {got:08X}, expected {argb:08X}");
        }
    }

    #[test]
    fn key_color_has_the_requested_chroma_near_tone_50() {
        // MCU's `palettes_test` "KeyColor with exact chroma".
        let key = TonalPalette::from_hue_and_chroma(50.0, 60.0).key_color();
        assert!((key.hue() - 50.0).abs() < 10.0, "key hue {}", key.hue());
        assert!(
            (key.chroma() - 60.0).abs() < 0.5,
            "key chroma {}",
            key.chroma()
        );
        assert!(
            key.tone() > 0.0 && key.tone() < 100.0,
            "key tone {}",
            key.tone()
        );
    }
}
