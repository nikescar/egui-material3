//! The Material 3 type scale.
//!
//! Values from Compose's `TypeScaleTokens.kt` (size, line height, tracking,
//! weight) and `TypefaceTokens.kt` (weights: Regular 400, Medium 500,
//! Bold 700). Both the baseline and the emphasized styles are here; the
//! emphasized ones are Compose's `*Emphasized*` tokens. Units are sp, used
//! as egui points.
//!
//! Weights are separate font families named by [`weight_family_name`]
//! (`"plain-400"`, `"plain-500"`, `"plain-700"`), registered by
//! [`load_fonts`](crate::theme::load_fonts). [`font_family`] falls back to
//! `Proportional` when a weight is not registered, so a missing font never
//! panics.

use egui::{text::LayoutJob, Color32, FontFamily, FontId, TextFormat};

/// A role in the Material 3 type scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypeRole {
    /// Display Large: 57/64.
    DisplayLarge,
    /// Display Medium: 45/52.
    DisplayMedium,
    /// Display Small: 36/44.
    DisplaySmall,
    /// Headline Large: 32/40.
    HeadlineLarge,
    /// Headline Medium: 28/36.
    HeadlineMedium,
    /// Headline Small: 24/32.
    HeadlineSmall,
    /// Title Large: 22/28.
    TitleLarge,
    /// Title Medium: 16/24, Medium.
    TitleMedium,
    /// Title Small: 14/20, Medium.
    TitleSmall,
    /// Body Large: 16/24.
    BodyLarge,
    /// Body Medium: 14/20.
    BodyMedium,
    /// Body Small: 12/16.
    BodySmall,
    /// Label Large: 14/20, Medium (buttons, tabs).
    LabelLarge,
    /// Label Medium: 12/16, Medium.
    LabelMedium,
    /// Label Small: 11/16, Medium.
    LabelSmall,
}

/// The weight `TypefaceTokens.WeightRegular`.
pub const WEIGHT_REGULAR: u16 = 400;
/// The weight `TypefaceTokens.WeightMedium`.
pub const WEIGHT_MEDIUM: u16 = 500;
/// The weight `TypefaceTokens.WeightBold`.
pub const WEIGHT_BOLD: u16 = 700;

/// The weights the crate registers a font family for.
pub const WEIGHTS: [u16; 3] = [WEIGHT_REGULAR, WEIGHT_MEDIUM, WEIGHT_BOLD];

/// One text style: size, line height and tracking in points, and a weight.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeStyle {
    /// Font size.
    pub size: f32,
    /// Line height (the row height of one line of text).
    pub line_height: f32,
    /// Font weight, 100..=900.
    pub weight: u16,
    /// Letter spacing added between glyphs.
    pub tracking: f32,
}

const fn ts(size: f32, line_height: f32, tracking: f32, weight: u16) -> TypeStyle {
    TypeStyle {
        size,
        line_height,
        weight,
        tracking,
    }
}

impl TypeRole {
    /// Every role, largest first.
    pub const ALL: [TypeRole; 15] = [
        TypeRole::DisplayLarge,
        TypeRole::DisplayMedium,
        TypeRole::DisplaySmall,
        TypeRole::HeadlineLarge,
        TypeRole::HeadlineMedium,
        TypeRole::HeadlineSmall,
        TypeRole::TitleLarge,
        TypeRole::TitleMedium,
        TypeRole::TitleSmall,
        TypeRole::BodyLarge,
        TypeRole::BodyMedium,
        TypeRole::BodySmall,
        TypeRole::LabelLarge,
        TypeRole::LabelMedium,
        TypeRole::LabelSmall,
    ];

    /// The baseline style (`TypeScaleTokens.<Role>*`).
    pub const fn style(self) -> TypeStyle {
        match self {
            TypeRole::DisplayLarge => ts(57.0, 64.0, -0.25, WEIGHT_REGULAR),
            TypeRole::DisplayMedium => ts(45.0, 52.0, 0.0, WEIGHT_REGULAR),
            TypeRole::DisplaySmall => ts(36.0, 44.0, 0.0, WEIGHT_REGULAR),
            TypeRole::HeadlineLarge => ts(32.0, 40.0, 0.0, WEIGHT_REGULAR),
            TypeRole::HeadlineMedium => ts(28.0, 36.0, 0.0, WEIGHT_REGULAR),
            TypeRole::HeadlineSmall => ts(24.0, 32.0, 0.0, WEIGHT_REGULAR),
            TypeRole::TitleLarge => ts(22.0, 28.0, 0.0, WEIGHT_REGULAR),
            TypeRole::TitleMedium => ts(16.0, 24.0, 0.15, WEIGHT_MEDIUM),
            TypeRole::TitleSmall => ts(14.0, 20.0, 0.1, WEIGHT_MEDIUM),
            TypeRole::BodyLarge => ts(16.0, 24.0, 0.5, WEIGHT_REGULAR),
            TypeRole::BodyMedium => ts(14.0, 20.0, 0.25, WEIGHT_REGULAR),
            TypeRole::BodySmall => ts(12.0, 16.0, 0.4, WEIGHT_REGULAR),
            TypeRole::LabelLarge => ts(14.0, 20.0, 0.1, WEIGHT_MEDIUM),
            TypeRole::LabelMedium => ts(12.0, 16.0, 0.5, WEIGHT_MEDIUM),
            TypeRole::LabelSmall => ts(11.0, 16.0, 0.5, WEIGHT_MEDIUM),
        }
    }

    /// The emphasized style (`TypeScaleTokens.<Role>Emphasized*`): same
    /// metrics, heavier weight (Medium for display, headline, title large
    /// and body; Bold for title medium/small and labels).
    pub const fn emphasized(self) -> TypeStyle {
        let base = self.style();
        let weight = match self {
            TypeRole::TitleMedium
            | TypeRole::TitleSmall
            | TypeRole::LabelLarge
            | TypeRole::LabelMedium
            | TypeRole::LabelSmall => WEIGHT_BOLD,
            TypeRole::DisplayLarge
            | TypeRole::DisplayMedium
            | TypeRole::DisplaySmall
            | TypeRole::HeadlineLarge
            | TypeRole::HeadlineMedium
            | TypeRole::HeadlineSmall
            | TypeRole::TitleLarge
            | TypeRole::BodyLarge
            | TypeRole::BodyMedium
            | TypeRole::BodySmall => WEIGHT_MEDIUM,
        };
        ts(base.size, base.line_height, base.tracking, weight)
    }
}

/// The family name a weight is registered under: `"plain-<weight>"`.
pub fn weight_family_name(weight: u16) -> String {
    format!("plain-{weight}")
}

/// The registered family for `weight`, or `Proportional` when the context
/// has no font for it.
pub fn font_family(ctx: &egui::Context, weight: u16) -> FontFamily {
    let family = FontFamily::Name(weight_family_name(weight).into());
    let registered = ctx.fonts(|f| f.definitions().families.contains_key(&family));
    if registered {
        family
    } else {
        FontFamily::Proportional
    }
}

impl TypeStyle {
    /// The font for this style.
    pub fn font_id(&self, ctx: &egui::Context) -> FontId {
        FontId::new(self.size, font_family(ctx, self.weight))
    }

    /// A [`TextFormat`] with this style's font, line height and tracking.
    pub fn text_format(&self, ctx: &egui::Context, color: Color32) -> TextFormat {
        TextFormat {
            font_id: self.font_id(ctx),
            line_height: Some(self.line_height),
            extra_letter_spacing: self.tracking,
            color,
            ..Default::default()
        }
    }

    /// A single-section [`LayoutJob`] for `text` in this style.
    pub fn layout_job(
        &self,
        ctx: &egui::Context,
        text: impl Into<String>,
        color: Color32,
    ) -> LayoutJob {
        LayoutJob::single_section(text.into(), self.text_format(ctx, color))
    }
}
