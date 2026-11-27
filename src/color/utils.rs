// ============================================================================
// Module:       color::utils
// Description:  sRGB, linear RGB, XYZ, L*a*b* and L* conversions; angle maths.
//
// Dependencies: none
// ============================================================================

//! Colour-space and maths helpers shared by the dynamic colour port.
//!
//! Ported from Material Color Utilities (`utils/color_utils.ts`,
//! `utils/math_utils.ts`; Apache-2.0, Copyright 2021 Google LLC). Colours
//! are packed `0xAARRGGBB` integers, as in MCU, so results can be compared
//! hex for hex.

/// sRGB (linear, 0..100) to XYZ (D65).
const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.41233895, 0.35762064, 0.18051042],
    [0.2126, 0.7152, 0.0722],
    [0.01932141, 0.11916382, 0.95034478],
];

/// XYZ (D65) to linear sRGB (0..100).
const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [
        3.2413774792388685,
        -1.5376652402851851,
        -0.49885366846268053,
    ],
    [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
    [
        0.05562093689691305,
        -0.20395524564742123,
        1.0571799111220335,
    ],
];

/// The D65 white point in XYZ, Y = 100.
pub(crate) const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

/// JavaScript's `Math.round`: halves round towards positive infinity.
///
/// MCU's reference implementation is TypeScript/Java, where `round(-0.5)`
/// is `-0`; Rust's `f64::round` rounds halves away from zero. Using this
/// keeps every threshold comparison identical to MCU's.
#[inline]
pub(crate) fn js_round(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// -1, 0 or 1.
#[inline]
pub(crate) fn signum(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else if x == 0.0 {
        0.0
    } else {
        1.0
    }
}

/// Linear interpolation: `start` at 0, `stop` at 1.
#[inline]
pub(crate) fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * stop
}

/// Clamps `input` to `min..=max`, MCU style (no panic when `min > max`:
/// `min` wins, as in MCU's `clampDouble`).
#[inline]
pub(crate) fn clamp_double(min: f64, max: f64, input: f64) -> f64 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

/// Wraps an integer angle into `0..360`.
#[inline]
pub(crate) fn sanitize_degrees_int(degrees: i32) -> i32 {
    degrees.rem_euclid(360)
}

/// Wraps an angle into `0.0..360.0`.
#[inline]
pub(crate) fn sanitize_degrees_double(degrees: f64) -> f64 {
    let d = degrees % 360.0;
    if d < 0.0 {
        d + 360.0
    } else {
        d
    }
}

/// 1 if the shortest way from `from` to `to` is increasing, else -1.
#[inline]
pub(crate) fn rotation_direction(from: f64, to: f64) -> f64 {
    let increasing = sanitize_degrees_double(to - from);
    if increasing <= 180.0 {
        1.0
    } else {
        -1.0
    }
}

/// The distance between two hue angles, `0.0..=180.0`.
#[inline]
pub(crate) fn difference_degrees(a: f64, b: f64) -> f64 {
    180.0 - ((a - b).abs() - 180.0).abs()
}

/// `matrix * row` for a 3x3 matrix.
#[inline]
pub(crate) fn matrix_multiply(row: [f64; 3], matrix: &[[f64; 3]; 3]) -> [f64; 3] {
    let [r0, r1, r2] = row;
    let [m0, m1, m2] = matrix;
    [
        r0 * m0[0] + r1 * m0[1] + r2 * m0[2],
        r0 * m1[0] + r1 * m1[1] + r2 * m1[2],
        r0 * m2[0] + r1 * m2[1] + r2 * m2[2],
    ]
}

/// Packs opaque `0xFFRRGGBB`.
#[inline]
pub(crate) fn argb_from_rgb(red: u8, green: u8, blue: u8) -> u32 {
    0xFF00_0000 | (u32::from(red) << 16) | (u32::from(green) << 8) | u32::from(blue)
}

/// The red channel.
#[inline]
pub(crate) fn red_from_argb(argb: u32) -> u8 {
    let [_, r, _, _] = argb.to_be_bytes();
    r
}

/// The green channel.
#[inline]
pub(crate) fn green_from_argb(argb: u32) -> u8 {
    let [_, _, g, _] = argb.to_be_bytes();
    g
}

/// The blue channel.
#[inline]
pub(crate) fn blue_from_argb(argb: u32) -> u8 {
    let [_, _, _, b] = argb.to_be_bytes();
    b
}

/// Linear RGB (each 0..100) to ARGB.
pub(crate) fn argb_from_linrgb(linrgb: [f64; 3]) -> u32 {
    let [r, g, b] = linrgb;
    argb_from_rgb(delinearized(r), delinearized(g), delinearized(b))
}

/// XYZ to ARGB.
pub(crate) fn argb_from_xyz(x: f64, y: f64, z: f64) -> u32 {
    let [m0, m1, m2] = XYZ_TO_SRGB;
    let linear_r = m0[0] * x + m0[1] * y + m0[2] * z;
    let linear_g = m1[0] * x + m1[1] * y + m1[2] * z;
    let linear_b = m2[0] * x + m2[1] * y + m2[2] * z;
    argb_from_rgb(
        delinearized(linear_r),
        delinearized(linear_g),
        delinearized(linear_b),
    )
}

/// ARGB to XYZ.
pub(crate) fn xyz_from_argb(argb: u32) -> [f64; 3] {
    let r = linearized(red_from_argb(argb));
    let g = linearized(green_from_argb(argb));
    let b = linearized(blue_from_argb(argb));
    matrix_multiply([r, g, b], &SRGB_TO_XYZ)
}

/// ARGB to L*a*b*.
pub(crate) fn lab_from_argb(argb: u32) -> [f64; 3] {
    let [x, y, z] = xyz_from_argb(argb);
    let [wx, wy, wz] = WHITE_POINT_D65;
    let fx = lab_f(x / wx);
    let fy = lab_f(y / wy);
    let fz = lab_f(z / wz);
    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    [l, a, b]
}

/// The grey with lightness `lstar`.
pub(crate) fn argb_from_lstar(lstar: f64) -> u32 {
    let y = y_from_lstar(lstar);
    let component = delinearized(y);
    argb_from_rgb(component, component, component)
}

/// The L* (CIE lightness, MCU's "tone") of an ARGB colour.
pub(crate) fn lstar_from_argb(argb: u32) -> f64 {
    let [_, y, _] = xyz_from_argb(argb);
    116.0 * lab_f(y / 100.0) - 16.0
}

/// Relative luminance Y (0..100) of lightness `lstar`.
pub(crate) fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

/// Lightness L* of relative luminance `y` (0..100).
pub(crate) fn lstar_from_y(y: f64) -> f64 {
    lab_f(y / 100.0) * 116.0 - 16.0
}

/// An 8-bit sRGB channel to linear 0..100.
pub(crate) fn linearized(component: u8) -> f64 {
    let normalized = f64::from(component) / 255.0;
    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/// A linear 0..100 channel to 8-bit sRGB, rounded and clamped.
pub(crate) fn delinearized(component: f64) -> u8 {
    let normalized = component / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };
    let rounded = clamp_double(0.0, 255.0, js_round(delinearized * 255.0));
    // In 0..=255 after the clamp (NaN, which a cast maps to 0, cannot reach
    // here from finite inputs), so the cast is exact.
    rounded as u8
}

fn lab_f(t: f64) -> f64 {
    const E: f64 = 216.0 / 24389.0;
    const KAPPA: f64 = 24389.0 / 27.0;
    if t > E {
        t.powf(1.0 / 3.0)
    } else {
        (KAPPA * t + 16.0) / 116.0
    }
}

fn lab_invf(ft: f64) -> f64 {
    const E: f64 = 216.0 / 24389.0;
    const KAPPA: f64 = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;
    if ft3 > E {
        ft3
    } else {
        (116.0 * ft - 16.0) / KAPPA
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lstar_round_trips_through_y() {
        for i in 0..=100 {
            let lstar = f64::from(i);
            let back = lstar_from_y(y_from_lstar(lstar));
            assert!(
                (back - lstar).abs() < 1e-8,
                "L* {lstar} came back as {back}"
            );
        }
    }

    #[test]
    fn channels_round_trip_through_linear() {
        for c in 0..=255u8 {
            assert_eq!(delinearized(linearized(c)), c, "channel {c}");
        }
    }

    #[test]
    fn js_round_rounds_halves_up() {
        assert_eq!(js_round(-0.5), 0.0, "JS rounds -0.5 to -0");
        assert_eq!(js_round(0.5), 1.0, "JS rounds 0.5 to 1");
        assert_eq!(js_round(59.5), 60.0, "JS rounds 59.5 to 60");
    }
}
