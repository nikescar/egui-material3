#![doc(hidden)]

use eframe::egui::{self, TextureHandle, TextureId, Window};
use egui_material3::{
    svg_emoji::SvgCollection,
    MaterialButton, assist_chip, filter_chip, input_chip, suggestion_chip,
    fab_primary, fab_secondary,
    icon_button_standard, icon_button_filled, icon_button_filled_tonal, icon_button_outlined,
};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CollectionKind {
    Solar,
    Noto,
    Twemoji,
}

impl CollectionKind {
    fn label(self) -> &'static str {
        match self {
            Self::Solar => "Solar Icons",
            Self::Noto => "Noto Emoji",
            Self::Twemoji => "Twemoji",
        }
    }

    fn load(self) -> Vec<(&'static str, &'static str)> {
        let col = match self {
            Self::Solar => SvgCollection::solar_icons(),
            Self::Noto => SvgCollection::noto_emoji(),
            Self::Twemoji => SvgCollection::twemoji(),
        };
        col.icons.into_iter().map(|i| (i.name, i.svg_data)).collect()
    }
}

#[doc(hidden)]
pub struct SvgEmojiWindow {
    pub open: bool,
    active_kind: CollectionKind,
    filter: String,
    /// Texture cache: texture_key(svg_data, size) -> TextureHandle
    svg_textures: HashMap<u64, TextureHandle>,
    /// Cached collection for the active kind
    cached_kind: Option<CollectionKind>,
    /// All icons as static string pairs (name, svg_data)
    all_icons: Vec<(&'static str, &'static str)>,
    /// Demo state for filter chip
    filter_selected_1: bool,
}

impl Default for SvgEmojiWindow {
    fn default() -> Self {
        Self {
            open: false,
            active_kind: CollectionKind::Solar,
            filter: String::new(),
            svg_textures: HashMap::new(),
            cached_kind: None,
            all_icons: Vec::new(),
            filter_selected_1: false,
        }
    }
}

impl SvgEmojiWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("SVG Emoji / Icon Browser")
            .open(&mut open)
            .default_size([960.0, 700.0])
            .show(ctx, |ui| {
                self.ui(ui);
            });
        self.open = open;
    }

    fn ensure_collection(&mut self) {
        if self.cached_kind != Some(self.active_kind) {
            self.all_icons = self.active_kind.load();
            self.cached_kind = Some(self.active_kind);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        self.ensure_collection();

        // ── Top bar ──────────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            for kind in [CollectionKind::Solar, CollectionKind::Noto, CollectionKind::Twemoji] {
                let selected = self.active_kind == kind;
                if ui.selectable_label(selected, kind.label()).clicked() && !selected {
                    self.active_kind = kind;
                    self.filter.clear();
                    self.ensure_collection();
                }
            }
            ui.add_space(16.0);
            ui.label("🔍");
            ui.add(
                egui::TextEdit::singleline(&mut self.filter)
                    .hint_text("Search by name…")
                    .desired_width(200.0),
            );
            if !self.filter.is_empty() && ui.small_button("✕").clicked() {
                self.filter.clear();
            }
        });

        ui.separator();

        // ── Build filtered list (owned static refs – releases borrow on self) ─
        let filter_lower = self.filter.to_lowercase();
        let filtered: Vec<(&'static str, &'static str)> = self
            .all_icons
            .iter()
            .filter(|(name, _)| {
                filter_lower.is_empty() || name.to_lowercase().contains(&filter_lower)
            })
            .copied()
            .collect();

        let total = self.all_icons.len();
        ui.label(format!(
            "{} / {} icons  —  hover to preview on controls",
            filtered.len(),
            total,
        ));
        ui.separator();

        // ── Virtual-scroll grid ───────────────────────────────────────────────
        const CELL_W: f32 = 72.0;
        const CELL_H: f32 = 84.0;
        const ICON_SIZE: u32 = 40;

        let available_width = ui.available_width();
        let cols = ((available_width / CELL_W) as usize).max(1);
        let total_rows = (filtered.len() + cols - 1) / cols;

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show_rows(ui, CELL_H, total_rows, |ui, row_range| {
                for row in row_range {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

                        for col in 0..cols {
                            let idx = row * cols + col;
                            if let Some(&(name, svg_data)) = filtered.get(idx) {
                                let (resp, painter) = ui.allocate_painter(
                                    egui::vec2(CELL_W, CELL_H),
                                    egui::Sense::hover(),
                                );

                                // Hover background
                                if resp.hovered() {
                                    painter.rect_filled(
                                        resp.rect,
                                        4.0,
                                        ui.visuals().widgets.hovered.bg_fill,
                                    );
                                }

                                // SVG icon – get TextureId (Copy) so borrow ends here
                                let tid = self.get_or_load_id(
                                    painter.ctx(),
                                    svg_data,
                                    ICON_SIZE,
                                );
                                if let Some(tid) = tid {
                                    let icon_rect = egui::Rect::from_center_size(
                                        resp.rect.center_top()
                                            + egui::vec2(0.0, ICON_SIZE as f32 / 2.0 + 6.0),
                                        egui::vec2(ICON_SIZE as f32, ICON_SIZE as f32),
                                    );
                                    painter.image(
                                        tid,
                                        icon_rect,
                                        egui::Rect::from_min_max(
                                            egui::pos2(0.0, 0.0),
                                            egui::pos2(1.0, 1.0),
                                        ),
                                        egui::Color32::WHITE,
                                    );
                                }

                                // Truncated name
                                let label = truncate_name(name, 9);
                                painter.text(
                                    resp.rect.center_bottom() - egui::vec2(0.0, 5.0),
                                    egui::Align2::CENTER_BOTTOM,
                                    &label,
                                    egui::FontId::proportional(9.0),
                                    ui.visuals().weak_text_color(),
                                );

                                // Hover popup (self no longer borrowed by tid/label)
                                if resp.hovered() {
                                    resp.on_hover_ui(|ui| {
                                        self.hover_panel(ui, name, svg_data);
                                    });
                                }
                            } else {
                                let _ = ui.allocate_space(egui::vec2(CELL_W, CELL_H));
                            }
                        }
                    });
                }
            });
    }

    /// Tooltip popup shown when hovering an icon cell.
    fn hover_panel(&mut self, ui: &mut egui::Ui, name: &str, svg_data: &str) {
        ui.set_min_width(340.0);

        // ── Large preview ────────────────────────────────────────────────────
        ui.vertical_centered(|ui| {
            if let Some(tid) = self.get_or_load_id(ui.ctx(), svg_data, 64) {
                ui.image((tid, egui::vec2(64.0, 64.0)));
            }
            ui.add_space(4.0);
            ui.strong(name);
        });

        ui.separator();

        // ── Rendered on Material components ──────────────────────────────────
        ui.label("With SVG support (using this icon):");
        ui.add_space(8.0);

        ui.label("Chips:");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                assist_chip("Assist")
                    .leading_icon_svg(svg_data)
                    .on_click(|| {}),
            );
            ui.add(
                filter_chip("Filter", &mut self.filter_selected_1)
                    .leading_icon_svg(svg_data),
            );
            ui.add(
                input_chip("Input")
                    .leading_icon_svg(svg_data)
                    .removable(true)
                    .on_click(|| {}),
            );
            ui.add(
                suggestion_chip("Suggest")
                    .leading_icon_svg(svg_data)
                    .on_click(|| {}),
            );
        });

        ui.add_space(8.0);
        ui.label("Icon Buttons:");
        ui.horizontal_wrapped(|ui| {
            ui.add(icon_button_standard("").svg_data(svg_data));
            ui.add(icon_button_filled("").svg_data(svg_data));
            ui.add(icon_button_filled_tonal("").svg_data(svg_data));
            ui.add(icon_button_outlined("").svg_data(svg_data));
        });

        ui.add_space(8.0);
        ui.label("Buttons (using this SVG icon):");
        ui.horizontal_wrapped(|ui| {
            ui.add(MaterialButton::filled("Filled").leading_svg(svg_data));
            ui.add(MaterialButton::outlined("Outlined").leading_svg(svg_data));
            ui.add(MaterialButton::elevated("Elevated").leading_svg(svg_data).trailing_svg(svg_data));
            ui.add(MaterialButton::text("Text").trailing_svg(svg_data));
        });

        ui.add_space(8.0);
        ui.label("FABs (using this SVG icon):");
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add(fab_primary().svg_data(svg_data).on_click(|| {}));
            ui.add_space(8.0);
            ui.add(fab_secondary().svg_data(svg_data).text("Edit").on_click(|| {}));
        });

    }

    // ── Texture helpers ───────────────────────────────────────────────────────

    fn texture_key(svg_data: &str, size: u32) -> u64 {
        let mut h = DefaultHasher::new();
        svg_data.hash(&mut h);
        // Mix in size so different sizes get different keys
        h.finish() ^ ((size as u64) << 48)
    }

    /// Load (if needed) and return a `TextureId` (Copy – no borrow held on self).
    fn get_or_load_id(&mut self, ctx: &egui::Context, svg_data: &str, size: u32) -> Option<TextureId> {
        let key = Self::texture_key(svg_data, size);
        if !self.svg_textures.contains_key(&key) {
            match Self::render_svg(ctx, svg_data, size, &format!("svgem_{key:x}")) {
                Ok(tex) => {
                    self.svg_textures.insert(key, tex);
                }
                Err(e) => {
                    eprintln!("SVG render error ({size}px): {e}");
                    return None;
                }
            }
        }
        // TextureId is Copy – the borrow of svg_textures ends here
        self.svg_textures.get(&key).map(|t| t.id())
    }

    fn render_svg(
        ctx: &egui::Context,
        svg_data: &str,
        size: u32,
        key: &str,
    ) -> Result<TextureHandle, String> {
        use resvg::usvg;

        let tree = usvg::Tree::from_str(svg_data, &usvg::Options::default())
            .map_err(|e| e.to_string())?;
        let mut pixmap =
            tiny_skia::Pixmap::new(size, size).ok_or_else(|| "pixmap alloc failed".to_string())?;

        let ts = tree.size();
        let scale = (size as f32 / ts.width()).min(size as f32 / ts.height());
        resvg::render(
            &tree,
            tiny_skia::Transform::from_scale(scale, scale),
            &mut pixmap.as_mut(),
        );

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [size as usize, size as usize],
            pixmap.data(),
        );
        Ok(ctx.load_texture(key, color_image, egui::TextureOptions::LINEAR))
    }
}

fn truncate_name(name: &str, max_chars: usize) -> String {
    let mut chars = name.char_indices();
    let end = chars
        .nth(max_chars)
        .map(|(i, _)| i)
        .unwrap_or(name.len());
    if end < name.len() {
        format!("{}…", &name[..end])
    } else {
        name.to_string()
    }
}
