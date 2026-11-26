//! Material 3 elevation levels.
//!
//! Level heights from Compose's `ElevationTokens.kt` (`Level0`..`Level5`:
//! 0, 1, 3, 6, 8, 12 dp). Compose does not tokenise shadow geometry:
//! Android renders elevation natively from a key (spot) light above the
//! screen and an ambient light, with the framework's default
//! `spotShadowAlpha` 0.19 and `ambientShadowAlpha` 0.039. The two
//! [`egui::epaint::Shadow`]s here model that: the key shadow falls
//! further and blurs more the higher the surface, the ambient one is an
//! even halo. Colour is the `shadow` role. M3 no longer tints elevated
//! surfaces; pick a surface-container role for the fill instead.

use egui::{epaint::Shadow, Color32};

/// An elevation level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Elevation {
    /// `ElevationTokens.Level0`: 0 dp.
    #[default]
    Level0,
    /// `ElevationTokens.Level1`: 1 dp.
    Level1,
    /// `ElevationTokens.Level2`: 3 dp.
    Level2,
    /// `ElevationTokens.Level3`: 6 dp.
    Level3,
    /// `ElevationTokens.Level4`: 8 dp.
    Level4,
    /// `ElevationTokens.Level5`: 12 dp.
    Level5,
}

/// Opacity of the key (spot) shadow: Android's `spotShadowAlpha`.
pub const KEY_SHADOW_ALPHA: f32 = 0.19;
/// Opacity of the ambient shadow: Android's `ambientShadowAlpha`.
pub const AMBIENT_SHADOW_ALPHA: f32 = 0.039;

impl Elevation {
    /// Every level, lowest first.
    pub const ALL: [Elevation; 6] = [
        Elevation::Level0,
        Elevation::Level1,
        Elevation::Level2,
        Elevation::Level3,
        Elevation::Level4,
        Elevation::Level5,
    ];

    /// The height in dp (egui points).
    pub const fn dp(self) -> f32 {
        match self {
            Elevation::Level0 => 0.0,
            Elevation::Level1 => 1.0,
            Elevation::Level2 => 3.0,
            Elevation::Level3 => 6.0,
            Elevation::Level4 => 8.0,
            Elevation::Level5 => 12.0,
        }
    }

    /// The key and ambient shadows for this level in `shadow_color`
    /// (normally the `shadow` role). Level 0 has none.
    pub fn shadows(self, shadow_color: Color32) -> [Shadow; 2] {
        let dp = self.dp();
        if dp <= 0.0 {
            return [Shadow::NONE, Shadow::NONE];
        }
        // Elevations are at most 12 dp, so these fit i8/u8 comfortably.
        let to_u8 = |v: f32| v.round().clamp(0.0, 255.0) as u8;
        let key = Shadow {
            offset: [0, (dp.round().clamp(0.0, 127.0)) as i8],
            blur: to_u8(dp * 2.0),
            spread: 0,
            color: shadow_color.gamma_multiply(KEY_SHADOW_ALPHA),
        };
        let ambient = Shadow {
            offset: [0, 0],
            blur: to_u8(dp * 2.0),
            spread: to_u8(dp / 4.0),
            color: shadow_color.gamma_multiply(AMBIENT_SHADOW_ALPHA),
        };
        [ambient, key]
    }
}
