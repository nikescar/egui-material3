// ============================================================================
// Module:       color::cam16
// Description:  CAM16 colour appearance model and its viewing conditions.
//
// Dependencies: super::utils
// ============================================================================

//! CAM16: the colour appearance model HCT's hue and chroma come from.
//!
//! Ported from Material Color Utilities (`hct/cam16.ts`,
//! `hct/viewing_conditions.ts`; Apache-2.0, Copyright 2021 Google LLC).

use std::f64::consts::PI;
use std::sync::LazyLock;

use super::utils;

/// The environment a colour is seen in. Everything in this crate uses
/// [`ViewingConditions::default`] (MCU's `ViewingConditions.DEFAULT`):
/// D65 white, a mid-grey background, average surround.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewingConditions {
    pub(crate) n: f64,
    pub(crate) aw: f64,
    pub(crate) nbb: f64,
    pub(crate) ncb: f64,
    pub(crate) c: f64,
    pub(crate) nc: f64,
    pub(crate) rgb_d: [f64; 3],
    pub(crate) fl: f64,
    pub(crate) fl_root: f64,
    pub(crate) z: f64,
}

static DEFAULT_VIEWING_CONDITIONS: LazyLock<ViewingConditions> = LazyLock::new(|| {
    ViewingConditions::make(
        utils::WHITE_POINT_D65,
        (200.0 / PI) * utils::y_from_lstar(50.0) / 100.0,
        50.0,
        2.0,
        false,
    )
});

impl Default for ViewingConditions {
    fn default() -> Self {
        *DEFAULT_VIEWING_CONDITIONS
    }
}

impl ViewingConditions {
    /// Viewing conditions from their physical parameters (MCU's
    /// `ViewingConditions.make`).
    ///
    /// * `white_point`: XYZ of the adopted white, Y = 100.
    /// * `adapting_luminance`: in cd/m².
    /// * `background_lstar`: L* of the background.
    /// * `surround`: 0 (dark) to 2 (average).
    /// * `discounting_illuminant`: whether the eye fully adapts to the white.
    pub fn make(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
    ) -> Self {
        let [wx, wy, wz] = white_point;
        let r_w = wx * 0.401288 + wy * 0.650173 + wz * -0.051461;
        let g_w = wx * -0.250268 + wy * 1.204414 + wz * 0.045854;
        let b_w = wx * -0.002079 + wy * 0.048952 + wz * 0.953127;
        let f = 0.8 + surround / 10.0;
        let c = if f >= 0.9 {
            utils::lerp(0.59, 0.69, (f - 0.9) * 10.0)
        } else {
            utils::lerp(0.525, 0.59, (f - 0.8) * 10.0)
        };
        let d = if discounting_illuminant {
            1.0
        } else {
            f * (1.0 - (1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp())
        };
        let d = utils::clamp_double(0.0, 1.0, d);
        let nc = f;
        let rgb_d = [
            d * (100.0 / r_w) + 1.0 - d,
            d * (100.0 / g_w) + 1.0 - d,
            d * (100.0 / b_w) + 1.0 - d,
        ];
        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4f = 1.0 - k4;
        let fl = k4 * adapting_luminance + 0.1 * k4f * k4f * (5.0 * adapting_luminance).cbrt();
        let n = utils::y_from_lstar(background_lstar) / wy;
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 / n.powf(0.2);
        let ncb = nbb;
        let [d_r, d_g, d_b] = rgb_d;
        let factor = |d: f64, w: f64| (fl * d * w / 100.0).powf(0.42);
        let adapt = |f: f64| (400.0 * f) / (f + 27.13);
        let r_a = adapt(factor(d_r, r_w));
        let g_a = adapt(factor(d_g, g_w));
        let b_a = adapt(factor(d_b, b_w));
        let aw = (2.0 * r_a + g_a + 0.05 * b_a) * nbb;
        Self {
            n,
            aw,
            nbb,
            ncb,
            c,
            nc,
            rgb_d,
            fl,
            fl_root: fl.powf(0.25),
            z,
        }
    }
}

/// A colour in CAM16 (and its CAM16-UCS coordinates).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cam16 {
    /// Hue angle in degrees, `0.0..360.0`.
    pub hue: f64,
    /// Chroma.
    pub chroma: f64,
    /// Lightness J.
    pub j: f64,
    /// Brightness Q.
    pub q: f64,
    /// Colourfulness M.
    pub m: f64,
    /// Saturation s.
    pub s: f64,
    /// CAM16-UCS J*.
    pub jstar: f64,
    /// CAM16-UCS a*.
    pub astar: f64,
    /// CAM16-UCS b*.
    pub bstar: f64,
}

impl Cam16 {
    /// Perceptual distance to `other` in CAM16-UCS.
    pub fn distance(&self, other: &Cam16) -> f64 {
        let dj = self.jstar - other.jstar;
        let da = self.astar - other.astar;
        let db = self.bstar - other.bstar;
        let de_prime = (dj * dj + da * da + db * db).sqrt();
        1.41 * de_prime.powf(0.63)
    }

    /// The CAM16 of an ARGB colour in the default viewing conditions.
    pub fn from_argb(argb: u32) -> Self {
        Self::from_argb_in(argb, &ViewingConditions::default())
    }

    /// The CAM16 of an ARGB colour in `vc`.
    pub fn from_argb_in(argb: u32, vc: &ViewingConditions) -> Self {
        let red_l = utils::linearized(utils::red_from_argb(argb));
        let green_l = utils::linearized(utils::green_from_argb(argb));
        let blue_l = utils::linearized(utils::blue_from_argb(argb));
        let x = 0.41233895 * red_l + 0.35762064 * green_l + 0.18051042 * blue_l;
        let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
        let z = 0.01932141 * red_l + 0.11916382 * green_l + 0.95034478 * blue_l;
        Self::from_xyz_in(x, y, z, vc)
    }

    /// The CAM16 of an XYZ colour in `vc`.
    pub fn from_xyz_in(x: f64, y: f64, z: f64, vc: &ViewingConditions) -> Self {
        let r_c = 0.401288 * x + 0.650173 * y - 0.051461 * z;
        let g_c = -0.250268 * x + 1.204414 * y + 0.045854 * z;
        let b_c = -0.002079 * x + 0.048952 * y + 0.953127 * z;
        let [d_r, d_g, d_b] = vc.rgb_d;
        let r_d = d_r * r_c;
        let g_d = d_g * g_c;
        let b_d = d_b * b_c;
        let adapt = |d: f64| {
            let af = (vc.fl * d.abs() / 100.0).powf(0.42);
            utils::signum(d) * 400.0 * af / (af + 27.13)
        };
        let r_a = adapt(r_d);
        let g_a = adapt(g_d);
        let b_a = adapt(b_d);
        let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
        let b = (r_a + g_a - 2.0 * b_a) / 9.0;
        let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
        let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;
        let atan_degrees = b.atan2(a) * 180.0 / PI;
        let hue = utils::sanitize_degrees_double(atan_degrees);
        let hue_radians = hue * PI / 180.0;
        let ac = p2 * vc.nbb;
        let j = 100.0 * (ac / vc.aw).powf(vc.c * vc.z);
        let q = (4.0 / vc.c) * (j / 100.0).sqrt() * (vc.aw + 4.0) * vc.fl_root;
        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = 0.25 * ((hue_prime * PI / 180.0 + 2.0).cos() + 3.8);
        let p1 = (50000.0 / 13.0) * e_hue * vc.nc * vc.ncb;
        let t = (p1 * (a * a + b * b).sqrt()) / (u + 0.305);
        let alpha = t.powf(0.9) * (1.64 - 0.29_f64.powf(vc.n)).powf(0.73);
        let c = alpha * (j / 100.0).sqrt();
        let m = c * vc.fl_root;
        let s = 50.0 * ((alpha * vc.c) / (vc.aw + 4.0)).sqrt();
        let jstar = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
        let mstar = (1.0 / 0.0228) * (1.0 + 0.0228 * m).ln();
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();
        Self {
            hue,
            chroma: c,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }

    /// A CAM16 colour from lightness J, chroma C and hue h, in the default
    /// viewing conditions.
    pub fn from_jch(j: f64, c: f64, h: f64) -> Self {
        Self::from_jch_in(j, c, h, &ViewingConditions::default())
    }

    /// A CAM16 colour from J, C and h in `vc`.
    pub fn from_jch_in(j: f64, c: f64, h: f64, vc: &ViewingConditions) -> Self {
        let q = (4.0 / vc.c) * (j / 100.0).sqrt() * (vc.aw + 4.0) * vc.fl_root;
        let m = c * vc.fl_root;
        let alpha = c / (j / 100.0).sqrt();
        let s = 50.0 * ((alpha * vc.c) / (vc.aw + 4.0)).sqrt();
        let hue_radians = h * PI / 180.0;
        let jstar = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
        let mstar = (1.0 / 0.0228) * (1.0 + 0.0228 * m).ln();
        Self {
            hue: h,
            chroma: c,
            j,
            q,
            m,
            s,
            jstar,
            astar: mstar * hue_radians.cos(),
            bstar: mstar * hue_radians.sin(),
        }
    }

    /// A CAM16 colour from CAM16-UCS coordinates, in the default viewing
    /// conditions.
    pub fn from_ucs(jstar: f64, astar: f64, bstar: f64) -> Self {
        let vc = ViewingConditions::default();
        let m = (astar * astar + bstar * bstar).sqrt();
        let big_m = ((m * 0.0228).exp() - 1.0) / 0.0228;
        let c = big_m / vc.fl_root;
        let mut h = bstar.atan2(astar) * (180.0 / PI);
        if h < 0.0 {
            h += 360.0;
        }
        let j = jstar / (1.0 - (jstar - 100.0) * 0.007);
        Self::from_jch_in(j, c, h, &vc)
    }

    /// The ARGB colour in the default viewing conditions.
    pub fn to_argb(&self) -> u32 {
        let [x, y, z] = self.xyz_in(&ViewingConditions::default());
        utils::argb_from_xyz(x, y, z)
    }

    /// The XYZ of this colour seen in `vc`.
    pub fn xyz_in(&self, vc: &ViewingConditions) -> [f64; 3] {
        let alpha = if self.chroma == 0.0 || self.j == 0.0 {
            0.0
        } else {
            self.chroma / (self.j / 100.0).sqrt()
        };
        let t = (alpha / (1.64 - 0.29_f64.powf(vc.n)).powf(0.73)).powf(1.0 / 0.9);
        let h_rad = self.hue * PI / 180.0;
        let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let ac = vc.aw * (self.j / 100.0).powf(1.0 / vc.c / vc.z);
        let p1 = e_hue * (50000.0 / 13.0) * vc.nc * vc.ncb;
        let p2 = ac / vc.nbb;
        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();
        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;
        let unadapt = |adapted: f64| {
            let base = f64::max(0.0, (27.13 * adapted.abs()) / (400.0 - adapted.abs()));
            utils::signum(adapted) * (100.0 / vc.fl) * base.powf(1.0 / 0.42)
        };
        let [d_r, d_g, d_b] = vc.rgb_d;
        let r_f = unadapt(r_a) / d_r;
        let g_f = unadapt(g_a) / d_g;
        let b_f = unadapt(b_a) / d_b;
        [
            1.86206786 * r_f - 1.01125463 * g_f + 0.14918677 * b_f,
            0.38752654 * r_f + 0.62144744 * g_f - 0.00897398 * b_f,
            -0.01584150 * r_f - 0.03412294 * g_f + 1.04996444 * b_f,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values from MCU's `cam16_test` / `hct_test` (red, green, blue,
    // white, black), within the tolerance MCU's own tests use.
    #[test]
    fn cam16_of_primaries_matches_mcu() {
        let red = Cam16::from_argb(0xFFFF_0000);
        assert!((red.hue - 27.408).abs() < 0.01, "red hue {}", red.hue);
        assert!(
            (red.chroma - 113.357).abs() < 0.01,
            "red chroma {}",
            red.chroma
        );
        assert!((red.j - 46.445).abs() < 0.01, "red J {}", red.j);
        assert!((red.m - 89.494).abs() < 0.01, "red M {}", red.m);
        assert!((red.s - 91.889).abs() < 0.01, "red s {}", red.s);
        assert!((red.q - 105.988).abs() < 0.01, "red Q {}", red.q);

        let blue = Cam16::from_argb(0xFF00_00FF);
        assert!((blue.hue - 282.788).abs() < 0.01, "blue hue {}", blue.hue);
        assert!(
            (blue.chroma - 87.230).abs() < 0.01,
            "blue chroma {}",
            blue.chroma
        );

        let white = Cam16::from_argb(0xFFFF_FFFF);
        assert!((white.j - 100.0).abs() < 0.01, "white J {}", white.j);
        assert!(
            (white.chroma - 2.869).abs() < 0.01,
            "white chroma {}",
            white.chroma
        );
    }

    #[test]
    fn cam16_round_trips() {
        for argb in [
            0xFFFF_0000_u32,
            0xFF00_FF00,
            0xFF00_00FF,
            0xFF67_50A4,
            0xFF12_3456,
        ] {
            let cam = Cam16::from_argb(argb);
            assert_eq!(cam.to_argb(), argb, "{argb:08X} round trip");
        }
    }
}
