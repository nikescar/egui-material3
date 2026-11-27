//! The Material 3 type scale.
//!
//! Values from Compose's `TypeScaleTokens.kt` (size, line height, tracking,
//! weight) and `TypefaceTokens.kt` (weights: Regular 400, Medium 500,
//! Bold 700). Both the baseline and the emphasized styles are here; the
//! emphasized ones are Compose's `*Emphasized*` tokens. Units are sp, used
//! as egui points.
//!
//! Weights are separate font families named by [`weight_family_name`]
//! (`"plain-400"`, `"plain-500"`, `"plain-700"`), registered with the
//! bundled Roboto faces by [`crate::fonts`] (and by
//! [`load_fonts`](crate::theme::load_fonts)). [`font_family`] falls back to
//! `Proportional` when a weight is not registered, so a missing font never
//! panics.
//!
//! [`text_styles`] maps the scale into egui's [`TextStyle`]s: the five
//! built-in ones (`Small`, `Body`, `Button`, `Heading`, `Monospace`) and one
//! `TextStyle::Name` per role ([`TypeRole::text_style`]).
//! [`apply_theme`](crate::theme::apply_theme) installs them. A `TextStyle`
//! carries only a font, so line height and tracking need
//! [`TypeStyle::layout_job`].

use std::collections::BTreeMap;

use egui::{text::LayoutJob, Color32, FontDefinitions, FontFamily, FontId, TextFormat, TextStyle};

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

    /// The role's name: `"DisplayLarge"`, `"LabelLarge"`, ...
    pub const fn name(self) -> &'static str {
        match self {
            TypeRole::DisplayLarge => "DisplayLarge",
            TypeRole::DisplayMedium => "DisplayMedium",
            TypeRole::DisplaySmall => "DisplaySmall",
            TypeRole::HeadlineLarge => "HeadlineLarge",
            TypeRole::HeadlineMedium => "HeadlineMedium",
            TypeRole::HeadlineSmall => "HeadlineSmall",
            TypeRole::TitleLarge => "TitleLarge",
            TypeRole::TitleMedium => "TitleMedium",
            TypeRole::TitleSmall => "TitleSmall",
            TypeRole::BodyLarge => "BodyLarge",
            TypeRole::BodyMedium => "BodyMedium",
            TypeRole::BodySmall => "BodySmall",
            TypeRole::LabelLarge => "LabelLarge",
            TypeRole::LabelMedium => "LabelMedium",
            TypeRole::LabelSmall => "LabelSmall",
        }
    }

    /// The egui text style for this role, `TextStyle::Name(self.name())`,
    /// which [`text_styles`] defines.
    pub fn text_style(self) -> TextStyle {
        TextStyle::Name(self.name().into())
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

/// The role behind each of egui's built-in text styles: `Small` is Body
/// Small, `Body` is Body Medium, `Button` is Label Large, `Heading` is
/// Title Large and `Monospace` uses Body Medium's size.
pub const BUILTIN_TEXT_STYLES: [(TextStyle, TypeRole); 5] = [
    (TextStyle::Small, TypeRole::BodySmall),
    (TextStyle::Body, TypeRole::BodyMedium),
    (TextStyle::Button, TypeRole::LabelLarge),
    (TextStyle::Heading, TypeRole::TitleLarge),
    (TextStyle::Monospace, TypeRole::BodyMedium),
];

/// The type scale as egui text styles, using the weight families that
/// `has_family` reports registered (and `Proportional` for the rest).
pub fn text_styles_with(has_family: impl Fn(&FontFamily) -> bool) -> BTreeMap<TextStyle, FontId> {
    let family = |weight: u16| {
        let named = FontFamily::Name(weight_family_name(weight).into());
        if has_family(&named) {
            named
        } else {
            FontFamily::Proportional
        }
    };
    let mut styles = BTreeMap::new();
    for (text_style, role) in BUILTIN_TEXT_STYLES {
        let style = role.style();
        let font_family = if text_style == TextStyle::Monospace {
            FontFamily::Monospace
        } else {
            family(style.weight)
        };
        styles.insert(text_style, FontId::new(style.size, font_family));
    }
    for role in TypeRole::ALL {
        let style = role.style();
        styles.insert(
            role.text_style(),
            FontId::new(style.size, family(style.weight)),
        );
    }
    styles
}

fn families_id() -> egui::Id {
    egui::Id::unique("egui_material3::typography::registered_families")
}

/// Records the font families `defs` defines, for [`text_styles`]. Called
/// by [`crate::fonts::install`] and [`load_fonts`](crate::theme::load_fonts)
/// with the definitions they hand to [`egui::Context::set_fonts`], which
/// only takes effect later (and before the first frame `ctx.fonts` cannot
/// be read at all).
pub fn record_font_families(ctx: &egui::Context, defs: &FontDefinitions) {
    let families: Vec<FontFamily> = defs.families.keys().cloned().collect();
    ctx.data_mut(|d| d.insert_temp(families_id(), std::sync::Arc::new(families)));
}

/// The type scale as egui text styles, for the families last recorded
/// with [`record_font_families`]. Without a record every weight uses
/// `Proportional`.
pub fn text_styles(ctx: &egui::Context) -> BTreeMap<TextStyle, FontId> {
    let families = ctx
        .data(|d| d.get_temp::<std::sync::Arc<Vec<FontFamily>>>(families_id()))
        .unwrap_or_default();
    text_styles_with(|family| families.contains(family))
}

/// Replaces the text styles of every style in `ctx` (light and dark) with
/// [`text_styles`].
pub fn apply_text_styles(ctx: &egui::Context) {
    let styles = text_styles(ctx);
    ctx.all_styles_mut(|style| {
        if style.text_styles != styles {
            style.text_styles = styles.clone();
        }
    });
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
