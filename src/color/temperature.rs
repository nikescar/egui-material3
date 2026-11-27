// ============================================================================
// Module:       color::temperature
// Description:  Colour temperature: analogous and complementary colours.
//
// Dependencies: super::{hct, utils}
// ============================================================================

//! Warm/cool theory on HCT, used by the Content and Fidelity variants to
//! pick their tertiary colour.
//!
//! Ported from Material Color Utilities (`temperature/temperature_cache.ts`;
//! Apache-2.0, Copyright 2023 Google LLC), after Ou, Woodcock and Wright's
//! warm-cool model.

use std::f64::consts::PI;

use super::hct::Hct;
use super::utils;

/// Temperature analysis of one input colour.
#[derive(Clone, Debug)]
pub struct TemperatureCache {
    input: Hct,
    input_temp: f64,
    /// The input's chroma and tone at every whole hue 0..=360, with each
    /// one's raw temperature.
    by_hue: Vec<(Hct, f64)>,
    coldest: (Hct, f64),
    warmest: (Hct, f64),
}

impl TemperatureCache {
    /// Analyses `input`.
    pub fn new(input: Hct) -> Self {
        let by_hue: Vec<(Hct, f64)> = (0..=360)
            .map(|hue| {
                let hct = Hct::from(f64::from(hue), input.chroma(), input.tone());
                (hct, raw_temperature(hct))
            })
            .collect();
        let input_temp = raw_temperature(input);
        // MCU sorts hues-plus-input by temperature (stably) and takes the
        // ends; a stable sort keeps the first of equal minima and the last
        // of equal maxima.
        let mut all: Vec<(Hct, f64)> = by_hue.clone();
        all.push((input, input_temp));
        all.sort_by(|a, b| a.1.total_cmp(&b.1));
        let coldest = all.first().copied().unwrap_or((input, input_temp));
        let warmest = all.last().copied().unwrap_or((input, input_temp));
        Self {
            input,
            input_temp,
            by_hue,
            coldest,
            warmest,
        }
    }

    /// The warmest colour at the input's chroma and tone.
    pub fn warmest(&self) -> Hct {
        self.warmest.0
    }

    /// The coldest colour at the input's chroma and tone.
    pub fn coldest(&self) -> Hct {
        self.coldest.0
    }

    fn at_hue(&self, hue: i32) -> (Hct, f64) {
        let index = usize::try_from(utils::sanitize_degrees_int(hue)).unwrap_or(0);
        self.by_hue
            .get(index)
            .copied()
            .unwrap_or((self.input, self.input_temp))
    }

    /// Hue `hue` rounded to a whole degree (MCU indexes by `Math.round`).
    fn rounded(hue: f64) -> i32 {
        // A hue is within 0..=360, so this fits.
        utils::js_round(hue) as i32
    }

    /// `temp` relative to the coldest (0) and warmest (1) colours.
    fn relative(&self, temp: f64) -> f64 {
        let range = self.warmest.1 - self.coldest.1;
        if range == 0.0 {
            return 0.5;
        }
        (temp - self.coldest.1) / range
    }

    /// The input's temperature relative to the coldest and warmest colours.
    pub fn input_relative_temperature(&self) -> f64 {
        self.relative(self.input_temp)
    }

    /// `count` colours analogous to the input, spread evenly in temperature
    /// over `divisions` steps around the hue circle; the input is in the
    /// middle (MCU's `analogous`).
    pub fn analogous(&self, count: usize, divisions: usize) -> Vec<Hct> {
        let start_hue = Self::rounded(self.input.hue());
        let (start_hct, start_temp) = self.at_hue(start_hue);
        let mut last_temp = self.relative(start_temp);
        let mut all_colors = vec![start_hct];

        let mut absolute_total_temp_delta = 0.0;
        for i in 0..360 {
            let (_, temp) = self.at_hue(start_hue + i);
            let temp = self.relative(temp);
            absolute_total_temp_delta += (temp - last_temp).abs();
            last_temp = temp;
        }

        let divisions_f = divisions as f64;
        let mut hue_addend = 1;
        let temp_step = absolute_total_temp_delta / divisions_f;
        let mut total_temp_delta = 0.0;
        last_temp = self.relative(start_temp);
        while all_colors.len() < divisions {
            let (hct, temp) = self.at_hue(start_hue + hue_addend);
            let temp = self.relative(temp);
            total_temp_delta += (temp - last_temp).abs();
            let desired = all_colors.len() as f64 * temp_step;
            let mut index_satisfied = total_temp_delta >= desired;
            let mut index_addend = 1.0;
            while index_satisfied && all_colors.len() < divisions {
                all_colors.push(hct);
                let desired = (all_colors.len() as f64 + index_addend) * temp_step;
                index_satisfied = total_temp_delta >= desired;
                index_addend += 1.0;
            }
            last_temp = temp;
            hue_addend += 1;
            if hue_addend > 360 {
                while all_colors.len() < divisions {
                    all_colors.push(hct);
                }
                break;
            }
        }

        let len = all_colors.len();
        let pick = |i: usize| {
            all_colors
                .get(i % len.max(1))
                .copied()
                .unwrap_or(self.input)
        };
        let mut answers = vec![self.input];
        let increase_hue_count = count.saturating_sub(1) / 2;
        for i in 1..=increase_hue_count {
            // MCU walks backwards from the end: index `-i` wraps to `len - i`.
            let index = len.max(1) - i % len.max(1);
            answers.insert(0, pick(index));
        }
        let decrease_hue_count = count.saturating_sub(increase_hue_count + 1);
        for i in 1..=decrease_hue_count {
            answers.push(pick(i));
        }
        answers
    }

    /// The colour complementary in temperature to the input (MCU's
    /// `complement`).
    pub fn complement(&self) -> Hct {
        let coldest_hue = self.coldest.0.hue();
        let coldest_temp = self.coldest.1;
        let warmest_hue = self.warmest.0.hue();
        let warmest_temp = self.warmest.1;
        let range = warmest_temp - coldest_temp;
        let start_is_coldest_to_warmest = is_between(self.input.hue(), coldest_hue, warmest_hue);
        let (start_hue, end_hue) = if start_is_coldest_to_warmest {
            (warmest_hue, coldest_hue)
        } else {
            (coldest_hue, warmest_hue)
        };
        let direction_of_rotation = 1.0;
        let mut smallest_error = 1000.0;
        let mut answer = self.at_hue(Self::rounded(self.input.hue())).0;
        let complement_relative_temp = 1.0 - self.input_relative_temperature();
        for hue_addend in 0..=360 {
            let hue = utils::sanitize_degrees_double(
                start_hue + direction_of_rotation * f64::from(hue_addend),
            );
            if !is_between(hue, start_hue, end_hue) {
                continue;
            }
            let (possible, temp) = self.at_hue(Self::rounded(hue));
            let relative_temp = (temp - coldest_temp) / range;
            let error = (complement_relative_temp - relative_temp).abs();
            if error < smallest_error {
                smallest_error = error;
                answer = possible;
            }
        }
        answer
    }
}

fn is_between(angle: f64, a: f64, b: f64) -> bool {
    if a < b {
        a <= angle && angle <= b
    } else {
        a <= angle || angle <= b
    }
}

/// Raw temperature of a colour: below 0 is cool, above is warm (MCU's
/// `rawTemperature`, from L*a*b* hue and chroma).
pub fn raw_temperature(color: Hct) -> f64 {
    let [_, a, b] = utils::lab_from_argb(color.to_argb());
    let hue = utils::sanitize_degrees_double(b.atan2(a) * 180.0 / PI);
    let chroma = (a * a + b * b).sqrt();
    -0.5 + 0.02
        * chroma.powf(1.07)
        * (utils::sanitize_degrees_double(hue - 50.0) * PI / 180.0).cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    // MCU's `temperature_cache_test`.
    #[test]
    fn raw_temperatures_match_mcu() {
        let cases: [(u32, f64); 4] = [
            (0xFF00_00FF, -1.393),
            (0xFFFF_0000, 2.351),
            (0xFF00_FF00, -0.267),
            (0xFFFF_FFFF, -0.5),
        ];
        for (argb, expected) in cases {
            let t = raw_temperature(Hct::from_argb(argb));
            assert!((t - expected).abs() < 0.001, "{argb:08X}: {t}");
        }
    }

    #[test]
    fn complement_of_blue_matches_mcu() {
        let c = TemperatureCache::new(Hct::from_argb(0xFF00_00FF)).complement();
        assert_eq!(c.to_argb(), 0xFF9D_0002, "complement {:08X}", c.to_argb());
    }

    #[test]
    fn analogous_of_blue_matches_mcu() {
        let a = TemperatureCache::new(Hct::from_argb(0xFF00_00FF)).analogous(5, 12);
        let got: Vec<u32> = a.iter().map(Hct::to_argb).collect();
        assert_eq!(
            got,
            vec![
                0xFF00_590C,
                0xFF00_564E,
                0xFF00_00FF,
                0xFF67_00CC,
                0xFF81_009F
            ],
            "analogous of blue"
        );
    }
}
