//! The bundled fonts: Roboto for text and Material Symbols for icons.
//!
//! Material 3's Plain and Brand typefaces are both Roboto
//! (`TypefaceTokens.Plain`/`Brand`). The crate bundles static Roboto
//! instances at the three weights the type scale uses (400, 500, 700) and
//! registers each as the `plain-<weight>` family that
//! [`TypeStyle::font_id`](crate::tokens::TypeStyle::font_id) asks for, so
//! Title Medium and Label Large render in a real Medium face.
//!
//! Icons come from Material Symbols. The bundled faces are static
//! instances of the variable font (FILL 0 and FILL 1 at wght 400, GRAD 0,
//! opsz 24), registered as one family per style and fill, named by
//! [`symbol_family_name`]. Outlined is always bundled; Rounded and Sharp
//! are behind the `symbols_rounded` and `symbols_sharp` features. Both
//! the text and icon fonts are cut by `tools/gen_fonts.py`.
//!
//! egui 0.37 can render a variable font at any axis position, so an app
//! that ships the full variable font can register it with
//! [`add_variable_symbols`]; [`MaterialIcon`](crate::icon::MaterialIcon)
//! then gets exact weight, grade and optical size too. With only the
//! static instances, weights other than 400 use the nearest registered
//! face (see [`add_symbol_font`] to add one).
//!
//! Licences: Roboto is under the SIL Open Font License 1.1
//! (`resources/fonts/OFL-Roboto.txt`); Material Symbols is under the
//! Apache License 2.0 (`resources/fonts/LICENSE-MaterialSymbols.txt`).
//!
//! Most apps call [`install`] once at start-up. Apps that build their own
//! [`FontDefinitions`] call [`add_material_fonts`] on them instead.

use egui::epaint::text::{FontTweak, VariationCoords};
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

use crate::tokens::typography::{weight_family_name, WEIGHTS, WEIGHT_REGULAR};

/// The Roboto faces: weight, font name and the TTF bytes.
pub const ROBOTO: [(u16, &str, &[u8]); 3] = [
    (
        400,
        "Roboto-Regular",
        include_bytes!("../resources/fonts/Roboto-Regular.ttf"),
    ),
    (
        500,
        "Roboto-Medium",
        include_bytes!("../resources/fonts/Roboto-Medium.ttf"),
    ),
    (
        700,
        "Roboto-Bold",
        include_bytes!("../resources/fonts/Roboto-Bold.ttf"),
    ),
];

/// The weight of the bundled Material Symbols instances.
pub const SYMBOL_WEIGHT: u16 = 400;

/// A Material Symbols style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SymbolStyle {
    /// Outlined: the default, always bundled.
    #[default]
    Outlined,
    /// Rounded (bundled with the `symbols_rounded` feature).
    Rounded,
    /// Sharp (bundled with the `symbols_sharp` feature).
    Sharp,
}

impl SymbolStyle {
    /// Every style.
    pub const ALL: [SymbolStyle; 3] = [
        SymbolStyle::Outlined,
        SymbolStyle::Rounded,
        SymbolStyle::Sharp,
    ];

    /// The style's name, as in the font's file name.
    pub const fn name(self) -> &'static str {
        match self {
            SymbolStyle::Outlined => "Outlined",
            SymbolStyle::Rounded => "Rounded",
            SymbolStyle::Sharp => "Sharp",
        }
    }

    /// The bundled FILL 0 and FILL 1 faces of this style, if this build
    /// bundles it.
    pub fn bundled(self) -> Option<[&'static [u8]; 2]> {
        match self {
            SymbolStyle::Outlined => Some([
                include_bytes!("../resources/fonts/MaterialSymbolsOutlined-Fill0.ttf"),
                include_bytes!("../resources/fonts/MaterialSymbolsOutlined-Fill1.ttf"),
            ]),
            SymbolStyle::Rounded => rounded_faces(),
            SymbolStyle::Sharp => sharp_faces(),
        }
    }
}

#[cfg(feature = "symbols_rounded")]
fn rounded_faces() -> Option<[&'static [u8]; 2]> {
    Some([
        include_bytes!("../resources/fonts/MaterialSymbolsRounded-Fill0.ttf"),
        include_bytes!("../resources/fonts/MaterialSymbolsRounded-Fill1.ttf"),
    ])
}

#[cfg(not(feature = "symbols_rounded"))]
fn rounded_faces() -> Option<[&'static [u8]; 2]> {
    None
}

#[cfg(feature = "symbols_sharp")]
fn sharp_faces() -> Option<[&'static [u8]; 2]> {
    Some([
        include_bytes!("../resources/fonts/MaterialSymbolsSharp-Fill0.ttf"),
        include_bytes!("../resources/fonts/MaterialSymbolsSharp-Fill1.ttf"),
    ])
}

#[cfg(not(feature = "symbols_sharp"))]
fn sharp_faces() -> Option<[&'static [u8]; 2]> {
    None
}

/// The family name of a Material Symbols face:
/// `"MaterialSymbols-<Style>-Fill<0|1>"` at weight 400, with `-W<weight>`
/// appended for other weights.
pub fn symbol_family_name(style: SymbolStyle, filled: bool, weight: u16) -> String {
    let fill = u8::from(filled);
    let style = style.name();
    if weight == SYMBOL_WEIGHT {
        format!("MaterialSymbols-{style}-Fill{fill}")
    } else {
        format!("MaterialSymbols-{style}-Fill{fill}-W{weight}")
    }
}

/// The [`FontFamily`] for [`symbol_family_name`].
pub fn symbol_family(style: SymbolStyle, filled: bool, weight: u16) -> FontFamily {
    FontFamily::Name(symbol_family_name(style, filled, weight).into())
}

/// The family to draw an icon with in `defs`: the exact style, fill and
/// weight if registered, else that style and fill at weight 400, else
/// Outlined at weight 400, else `Proportional`.
pub fn resolve_symbol_family(
    defs: &FontDefinitions,
    style: SymbolStyle,
    filled: bool,
    weight: u16,
) -> FontFamily {
    [
        symbol_family(style, filled, weight),
        symbol_family(style, filled, SYMBOL_WEIGHT),
        symbol_family(SymbolStyle::Outlined, filled, SYMBOL_WEIGHT),
    ]
    .into_iter()
    .find(|family| defs.families.contains_key(family))
    .unwrap_or(FontFamily::Proportional)
}

fn insert_front(defs: &mut FontDefinitions, family: FontFamily, font: &str) {
    let list = defs.families.entry(family).or_default();
    list.retain(|name| name != font);
    list.insert(0, font.to_owned());
}

fn push_back(defs: &mut FontDefinitions, family: FontFamily, font: &str) {
    let list = defs.families.entry(family).or_default();
    if !list.iter().any(|name| name == font) {
        list.push(font.to_owned());
    }
}

/// Adds Roboto: each weight at the front of its `plain-<weight>` family,
/// and Roboto Regular at the front of `Proportional`.
pub fn add_roboto(defs: &mut FontDefinitions) {
    for (weight, name, bytes) in ROBOTO {
        defs.font_data
            .insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
        insert_front(
            defs,
            FontFamily::Name(weight_family_name(weight).into()),
            name,
        );
        if weight == WEIGHT_REGULAR {
            insert_front(defs, FontFamily::Proportional, name);
        }
    }
}

/// Registers one static Material Symbols face (for instance a weight cut
/// with `tools/gen_fonts.py --weights 300,500,700`) as the family
/// [`symbol_family_name`]`(style, filled, weight)`.
pub fn add_symbol_font(
    defs: &mut FontDefinitions,
    style: SymbolStyle,
    filled: bool,
    weight: u16,
    data: FontData,
) {
    let family = symbol_family_name(style, filled, weight);
    defs.font_data.insert(family.clone(), Arc::new(data));
    defs.families
        .insert(FontFamily::Name(family.clone().into()), vec![family]);
}

/// Registers the full variable Material Symbols font of `style` (the
/// `MaterialSymbols<Style>[FILL,GRAD,opsz,wght].ttf` file) for both fills,
/// replacing the static faces. Icons then render at their exact weight,
/// grade and optical size, because
/// [`MaterialIcon`](crate::icon::MaterialIcon) passes those axes with
/// every glyph.
pub fn add_variable_symbols(defs: &mut FontDefinitions, style: SymbolStyle, data: FontData) {
    for filled in [false, true] {
        let tweak = FontTweak {
            coords: VariationCoords::new([("FILL", f32::from(u8::from(filled)))]),
            ..data.tweak.clone()
        };
        add_symbol_font(
            defs,
            style,
            filled,
            SYMBOL_WEIGHT,
            data.clone().tweak(tweak),
        );
    }
}

/// Adds every bundled Material Symbols style, and Outlined FILL 0 at the
/// end of `Proportional`, so a codepoint in ordinary text (as
/// [`material_symbol_text`](crate::material_symbol::material_symbol_text)
/// returns) still draws as an icon.
pub fn add_symbols(defs: &mut FontDefinitions) {
    for style in SymbolStyle::ALL {
        let Some(faces) = style.bundled() else {
            continue;
        };
        for (filled, bytes) in [(false, faces[0]), (true, faces[1])] {
            add_symbol_font(
                defs,
                style,
                filled,
                SYMBOL_WEIGHT,
                FontData::from_static(bytes),
            );
        }
    }
    let outlined = symbol_family_name(SymbolStyle::Outlined, false, SYMBOL_WEIGHT);
    push_back(defs, FontFamily::Proportional, &outlined);
}

/// Makes sure every `plain-<weight>` family exists and falls back to the
/// `Proportional` chain, so a weight without its own face, or a glyph
/// Roboto lacks, still renders.
pub fn complete_weight_families(defs: &mut FontDefinitions) {
    let proportional = defs
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    for weight in WEIGHTS {
        let family = FontFamily::Name(weight_family_name(weight).into());
        for name in &proportional {
            push_back(defs, family.clone(), name);
        }
    }
}

/// Adds Roboto, the bundled Material Symbols and the weight families to
/// `defs`.
pub fn add_material_fonts(defs: &mut FontDefinitions) {
    add_roboto(defs);
    add_symbols(defs);
    complete_weight_families(defs);
}

/// egui's default fonts with [`add_material_fonts`] applied.
pub fn material_font_definitions() -> FontDefinitions {
    let mut defs = FontDefinitions::default();
    add_material_fonts(&mut defs);
    defs
}

/// Installs [`material_font_definitions`] in `ctx` and maps the type scale
/// into its text styles
/// ([`apply_text_styles`](crate::tokens::typography::apply_text_styles)).
pub fn install(ctx: &egui::Context) {
    let defs = material_font_definitions();
    crate::tokens::typography::record_font_families(ctx, &defs);
    crate::tokens::typography::apply_text_styles(ctx);
    ctx.set_fonts(defs);
}
