//! Material Design 3 icons.
//!
//! [`MaterialIcon`] draws a Material Symbols icon by name (`icon("home")`),
//! outlined or filled, at a weight and size, tinted `onSurface` by default
//! (Compose's `LocalContentColor` default; pass `.color(..)` for primary or
//! `onSurfaceVariant` icons).
//!
//! [`Icon`] is the one icon input type for widgets: a Material Symbol, an
//! SVG document, or an already loaded texture, all drawn by [`paint_icon`]
//! with a tint. Monochrome SVGs take the tint; full-colour ones keep their
//! colours.
//!
//! Directional icons (`arrow_back`, `chevron_left`, `send`, ...) mirror
//! when the theme's layout direction is right-to-left, as Compose's
//! `Icons.AutoMirrored` do. A name ending in `_rtl` draws the base icon
//! mirrored (`"drive_file_move_rtl"`): Material Icons had such glyphs and
//! Material Symbols dropped them.

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use egui::epaint::text::VariationCoords;
use egui::epaint::{Mesh, Vertex};
use egui::load::SizedTexture;
use egui::text::LayoutJob;
use egui::{
    self, Color32, ColorImage, FontId, Painter, Pos2, Rect, Response, Sense, TextFormat,
    TextureHandle, TextureOptions, Ui, Vec2, Widget,
};

use crate::fonts::{resolve_symbol_family, SymbolStyle, SYMBOL_WEIGHT};
use crate::material_symbol::get_material_symbol;
use crate::tokens::{ColorRole, LayoutDirection};
use crate::MaterialTheme;

/// Icons that point along the reading direction and so mirror in RTL
/// layouts (Compose's `Icons.AutoMirrored` set, by Material Symbols name).
pub const AUTO_MIRRORED: [&str; 34] = [
    "arrow_back",
    "arrow_back_ios",
    "arrow_back_ios_new",
    "arrow_forward",
    "arrow_forward_ios",
    "arrow_left",
    "arrow_left_alt",
    "arrow_right",
    "arrow_right_alt",
    "chevron_left",
    "chevron_right",
    "exit_to_app",
    "first_page",
    "format_indent_decrease",
    "format_indent_increase",
    "forward",
    "help",
    "keyboard_arrow_left",
    "keyboard_arrow_right",
    "keyboard_tab",
    "last_page",
    "list",
    "login",
    "logout",
    "navigate_before",
    "navigate_next",
    "open_in_new",
    "read_more",
    "redo",
    "reply",
    "reply_all",
    "send",
    "undo",
    "trending_flat",
];

/// What to draw as an icon.
#[derive(Clone, Debug, PartialEq)]
pub enum Icon {
    /// A Material Symbols icon: its name (`"home"`), or the icon's
    /// codepoint as a one-character string.
    Symbol(String),
    /// An SVG document. A monochrome SVG is drawn in the tint colour; a
    /// multi-colour one keeps its colours.
    Svg(Arc<[u8]>),
    /// A texture already loaded into egui, drawn multiplied by the tint.
    Texture(SizedTexture),
}

impl Icon {
    /// An SVG icon from the document's bytes.
    pub fn svg(bytes: impl Into<Arc<[u8]>>) -> Self {
        Icon::Svg(bytes.into())
    }
}

impl From<&str> for Icon {
    fn from(name: &str) -> Self {
        Icon::Symbol(name.to_owned())
    }
}

impl From<String> for Icon {
    fn from(name: String) -> Self {
        Icon::Symbol(name)
    }
}

impl From<char> for Icon {
    fn from(codepoint: char) -> Self {
        Icon::Symbol(codepoint.to_string())
    }
}

impl From<SizedTexture> for Icon {
    fn from(texture: SizedTexture) -> Self {
        Icon::Texture(texture)
    }
}

/// How a Material Symbol is drawn: the font axes and mirroring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SymbolOptions {
    /// FILL 1 (filled) instead of FILL 0 (outlined).
    pub filled: bool,
    /// The wght axis, 100..=700. Exact with a variable font registered
    /// (see [`crate::fonts::add_variable_symbols`]), otherwise the nearest
    /// registered face.
    pub weight: u16,
    /// The GRAD axis, -25..=200 (variable font only).
    pub grade: f32,
    /// Outlined, Rounded or Sharp.
    pub style: SymbolStyle,
    /// Draw mirrored left-to-right.
    pub mirrored: bool,
}

impl Default for SymbolOptions {
    fn default() -> Self {
        Self {
            filled: false,
            weight: SYMBOL_WEIGHT,
            grade: 0.0,
            style: SymbolStyle::Outlined,
            mirrored: false,
        }
    }
}

/// The codepoint for a symbol name, and whether the name asks for the
/// mirrored glyph (a trailing `_rtl`). A one-character string that is not
/// a name is taken as the codepoint itself.
pub fn resolve_symbol(name: &str) -> Option<(char, bool)> {
    if let Some(c) = get_material_symbol(name) {
        return Some((c, false));
    }
    if let Some(c) = name.strip_suffix("_rtl").and_then(get_material_symbol) {
        return Some((c, true));
    }
    let mut chars = name.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some((c, false)),
        (Some(_), Some(_)) | (None, _) => None,
    }
}

/// The galley of one Material Symbols glyph at `size` points.
fn symbol_galley(
    painter: &Painter,
    glyph: char,
    size: f32,
    options: &SymbolOptions,
    tint: Color32,
) -> Arc<egui::Galley> {
    let family = painter.ctx().fonts(|f| {
        resolve_symbol_family(
            f.definitions(),
            options.style,
            options.filled,
            options.weight,
        )
    });
    // The axes only take effect on a variable font; static faces ignore them.
    let coords = VariationCoords::new([
        ("FILL", f32::from(u8::from(options.filled))),
        ("wght", f32::from(options.weight)),
        ("GRAD", options.grade),
        ("opsz", size.clamp(20.0, 48.0)),
    ]);
    let format = TextFormat {
        font_id: FontId::new(size, family),
        color: tint,
        coords,
        ..Default::default()
    };
    painter.layout_job(LayoutJob::single_section(glyph.to_string(), format))
}

/// Paints the glyph `glyph` centred in `rect`.
pub fn paint_symbol(
    painter: &Painter,
    rect: Rect,
    glyph: char,
    options: &SymbolOptions,
    tint: Color32,
) {
    let size = rect.width().min(rect.height());
    let galley = symbol_galley(painter, glyph, size, options, tint);
    let pos = rect.center() - galley.size() / 2.0;
    if options.mirrored {
        paint_mirrored(painter, pos, &galley, rect.center().x);
    } else {
        painter.galley(pos, galley, tint);
    }
}

/// Paints `galley` at `pos` flipped horizontally about `axis_x`.
fn paint_mirrored(painter: &Painter, pos: Pos2, galley: &egui::Galley, axis_x: f32) {
    let [tex_w, tex_h] = painter.ctx().fonts(|f| f.font_image_size());
    let uv_scale = Vec2::new(1.0 / tex_w.max(1) as f32, 1.0 / tex_h.max(1) as f32);
    let mut mesh = Mesh::default();
    for row in &galley.rows {
        let offset = pos.to_vec2() + row.pos.to_vec2();
        let base = mesh.vertices.len() as u32;
        mesh.vertices
            .extend(row.visuals.mesh.vertices.iter().map(|v| {
                let p = v.pos + offset;
                Vertex {
                    pos: Pos2::new(2.0 * axis_x - p.x, p.y),
                    uv: Pos2::new(v.uv.x * uv_scale.x, v.uv.y * uv_scale.y),
                    color: v.color,
                }
            }));
        mesh.indices
            .extend(row.visuals.mesh.indices.iter().map(|i| base + i));
    }
    painter.add(mesh);
}

/// A rasterised SVG, cached per document and pixel size.
#[derive(Clone)]
struct SvgTexture {
    texture: TextureHandle,
    /// Every visible pixel had one colour, so the image was stored white
    /// and takes the tint.
    tintable: bool,
}

fn rasterize_svg(bytes: &[u8], width: u32, height: u32) -> Option<(ColorImage, bool)> {
    use resvg::{tiny_skia, usvg};
    let tree = usvg::Tree::from_data(bytes, &usvg::Options::default()).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    let size = tree.size();
    let scale = (width as f32 / size.width()).min(height as f32 / size.height());
    let dx = (width as f32 - size.width() * scale) / 2.0;
    let dy = (height as f32 - size.height() * scale) / 2.0;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_row(scale, 0.0, 0.0, scale, dx, dy),
        &mut pixmap.as_mut(),
    );
    let pixels: Vec<tiny_skia::ColorU8> = pixmap.pixels().iter().map(|p| p.demultiply()).collect();
    let mut visible = pixels.iter().filter(|p| p.alpha() > 0);
    let first = visible.next().copied();
    let tintable = first.is_some_and(|first| {
        visible.all(|p| {
            p.red().abs_diff(first.red()) <= 8
                && p.green().abs_diff(first.green()) <= 8
                && p.blue().abs_diff(first.blue()) <= 8
        })
    });
    let size = [width as usize, height as usize];
    let rgba: Vec<u8> = pixels
        .iter()
        .flat_map(|p| {
            if tintable {
                [255, 255, 255, p.alpha()]
            } else {
                [p.red(), p.green(), p.blue(), p.alpha()]
            }
        })
        .collect();
    Some((ColorImage::from_rgba_unmultiplied(size, &rgba), tintable))
}

fn svg_texture(ctx: &egui::Context, bytes: &[u8], rect: Rect) -> Option<SvgTexture> {
    let ppp = ctx.pixels_per_point();
    let width = (rect.width() * ppp).round().clamp(1.0, 1024.0) as u32;
    let height = (rect.height() * ppp).round().clamp(1.0, 1024.0) as u32;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut hasher);
    let id = egui::Id::unique(("egui_material3::svg_icon", hasher.finish(), width, height));
    if let Some(cached) = ctx.data(|d| d.get_temp::<Option<SvgTexture>>(id)) {
        return cached;
    }
    let texture = rasterize_svg(bytes, width, height).map(|(image, tintable)| SvgTexture {
        texture: ctx.load_texture(format!("svg-icon-{id:?}"), image, TextureOptions::LINEAR),
        tintable,
    });
    ctx.data_mut(|d| d.insert_temp(id, texture.clone()));
    texture
}

const FULL_UV: Rect = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));

/// Paints `icon` into `rect` in `tint`: a symbol glyph (with `options`), an
/// SVG (tinted when monochrome) or a texture (multiplied by `tint`, fitted
/// to `rect` keeping its aspect ratio). An unknown symbol name paints
/// nothing.
pub fn paint_icon(
    painter: &Painter,
    rect: Rect,
    icon: &Icon,
    options: &SymbolOptions,
    tint: Color32,
) {
    match icon {
        Icon::Symbol(name) => {
            if let Some((glyph, rtl_variant)) = resolve_symbol(name) {
                let options = SymbolOptions {
                    mirrored: options.mirrored != rtl_variant,
                    ..*options
                };
                paint_symbol(painter, rect, glyph, &options, tint);
            }
        }
        Icon::Svg(bytes) => {
            if let Some(svg) = svg_texture(painter.ctx(), bytes, rect) {
                let tint = if svg.tintable { tint } else { Color32::WHITE };
                painter.image(svg.texture.id(), rect, FULL_UV, tint);
            }
        }
        Icon::Texture(texture) => {
            let fitted = if texture.size.x > 0.0 && texture.size.y > 0.0 {
                let scale = (rect.width() / texture.size.x).min(rect.height() / texture.size.y);
                Rect::from_center_size(rect.center(), texture.size * scale)
            } else {
                rect
            };
            painter.image(texture.id, fitted, FULL_UV, tint);
        }
    }
}

/// A Material Symbols icon (or any [`Icon`]) as a widget.
#[must_use = "add it with `ui.add(...)`"]
pub struct MaterialIcon {
    icon: Icon,
    size: f32,
    color: Option<Color32>,
    options: SymbolOptions,
    auto_mirror: Option<bool>,
}

impl MaterialIcon {
    /// An icon from a symbol name (`"home"`), a codepoint, or any [`Icon`].
    pub fn new(icon: impl Into<Icon>) -> Self {
        Self {
            icon: icon.into(),
            size: 24.0,
            color: None,
            options: SymbolOptions::default(),
            auto_mirror: None,
        }
    }

    /// The icon's size in points (default 24). Also the optical size (opsz)
    /// with a variable font.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// The colour (default: the theme's `onSurface`).
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// The filled face (FILL 1), as selected states use.
    pub fn filled(mut self, filled: bool) -> Self {
        self.options.filled = filled;
        self
    }

    /// The weight, 100..=700 (default 400).
    pub fn weight(mut self, weight: u16) -> Self {
        self.options.weight = weight;
        self
    }

    /// The grade, -25..=200 (default 0).
    pub fn grade(mut self, grade: f32) -> Self {
        self.options.grade = grade;
        self
    }

    /// Outlined (default), Rounded or Sharp.
    pub fn style(mut self, style: SymbolStyle) -> Self {
        self.options.style = style;
        self
    }

    /// Always draw mirrored.
    pub fn mirrored(mut self, mirrored: bool) -> Self {
        self.options.mirrored = mirrored;
        self
    }

    /// Whether to mirror in right-to-left layouts. Defaults to true for
    /// the directional icons in [`AUTO_MIRRORED`].
    pub fn auto_mirror(mut self, auto_mirror: bool) -> Self {
        self.auto_mirror = Some(auto_mirror);
        self
    }
}

impl Widget for MaterialIcon {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());
        if !ui.is_rect_visible(rect) {
            return response;
        }
        let theme = MaterialTheme::of(ui.ctx());
        // A disabled Ui's painter already fades what it paints.
        let tint = self
            .color
            .unwrap_or_else(|| theme.color(ColorRole::OnSurface));
        let auto_mirror = self.auto_mirror.unwrap_or_else(|| match &self.icon {
            Icon::Symbol(name) => AUTO_MIRRORED.contains(&name.as_str()),
            Icon::Svg(_) | Icon::Texture(_) => false,
        });
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let options = SymbolOptions {
            mirrored: self.options.mirrored != (auto_mirror && rtl),
            ..self.options
        };
        paint_icon(ui.painter(), rect, &self.icon, &options, tint);
        response
    }
}

/// [`MaterialIcon::new`].
pub fn icon(icon: impl Into<Icon>) -> MaterialIcon {
    MaterialIcon::new(icon)
}
