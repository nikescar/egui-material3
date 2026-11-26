//! Rendering implementation for MaterialButton
//!
//! A button is laid out in two steps: [`MaterialButton::prepare`] builds the
//! galleys and measures the content, and [`PreparedButton::show`] allocates
//! the space and paints. Button groups use the gap between the two to
//! measure every item before choosing widths.

use std::sync::Arc;

use super::{tokens::ButtonMetrics, types::*, MaterialButton};
use crate::material_symbol::material_symbol_text;
use crate::paint::{paint_elevation, paint_focus_ring_if_focused, state_layer_over};
use crate::tokens::{
    animate_value_with_spring, ColorRole, InteractionState, LayoutDirection, SpringSpeed,
};
use crate::MaterialTheme;
use egui::{
    ecolor::Color32,
    epaint::{CornerRadius, Stroke, StrokeKind},
    Galley, Image, Rect, Response, Role, Sense, TextStyle, TextWrapMode, TextureHandle, Ui, Vec2,
    Widget, WidgetText,
};

/// One piece of button content, laid out left to right.
enum Content<'a> {
    /// An icon glyph or label text, painted in the content colour.
    Galley(Arc<Galley>),
    /// An SVG icon, painted with its own colours.
    Texture(TextureHandle, f32),
    /// A caller-supplied image.
    Image(Image<'a>, Vec2),
    /// The deprecated shortcut text, painted in the weak text colour.
    Shortcut(Arc<Galley>),
}

impl Content<'_> {
    fn size(&self) -> Vec2 {
        match self {
            Content::Galley(g) | Content::Shortcut(g) => g.size(),
            Content::Texture(_, side) => Vec2::splat(*side),
            Content::Image(_, size) => *size,
        }
    }
}

/// A measured button, ready to allocate and paint.
pub(crate) struct PreparedButton<'a> {
    variant: MaterialButtonVariant,
    metrics: ButtonMetrics,
    shape: ButtonShape,
    content: Vec<Content<'a>>,
    label: String,
    padding: (f32, f32),
    gaps: Vec<f32>,
    desired: Vec2,
    exact_width: Option<f32>,
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    frame: bool,
    sense: Sense,
    corner_radius: Option<CornerRadius>,
    segment: Option<SegmentCorners>,
    toggle: ToggleState<'a>,
    role: Role,
    image_tint_follows_text_color: bool,
    elevation: Option<egui::epaint::Shadow>,
    enabled: bool,
    text_color: Option<Color32>,
}

fn svg_texture(ui: &Ui, svg: Option<String>, prefix: &str) -> Option<TextureHandle> {
    let svg = svg?;
    // A malformed SVG shows no icon rather than failing the whole button.
    crate::image_utils::create_texture_from_svg(ui.ctx(), &svg, &format!("{prefix}_{}", svg.len()))
        .ok()
}

fn icon_galley(ui: &Ui, name: Option<String>, size: f32) -> Option<Arc<Galley>> {
    let name = name?;
    let text: WidgetText = material_symbol_text(&name).into();
    Some(text.into_galley(
        ui,
        Some(TextWrapMode::Extend),
        f32::INFINITY,
        egui::FontId::proportional(size),
    ))
}

impl<'a> MaterialButton<'a> {
    /// Builds the galleys and measures the button.
    pub(crate) fn prepare(self, ui: &Ui) -> PreparedButton<'a> {
        let MaterialButton {
            image,
            text,
            shortcut_text,
            wrap_mode,
            variant,
            size,
            shape,
            fill,
            stroke,
            sense,
            frame,
            min_size,
            width,
            padding: padding_override,
            corner_radius,
            segment,
            toggle,
            role,
            image_tint_follows_text_color,
            elevation,
            disabled,
            leading_icon,
            trailing_icon,
            leading_svg,
            trailing_svg,
            text_color,
            label,
        } = self;
        let metrics = ButtonMetrics::of(size);
        let icon_size = metrics.icon_size;
        let frame = frame.unwrap_or(true);

        let leading_texture = svg_texture(ui, leading_svg, "btn_lead");
        let trailing_texture = svg_texture(ui, trailing_svg, "btn_trail");
        let leading_glyph = if leading_texture.is_none() {
            icon_galley(ui, leading_icon, icon_size)
        } else {
            None
        };
        let trailing_glyph = if trailing_texture.is_none() {
            icon_galley(ui, trailing_icon, icon_size)
        } else {
            None
        };

        let has_leading = leading_texture.is_some() || leading_glyph.is_some() || image.is_some();
        let has_trailing = trailing_texture.is_some() || trailing_glyph.is_some();
        // Baseline padding (`ButtonWithIconContentPadding`): 16 dp beside an
        // icon, 24 dp otherwise; text buttons use 12 dp, 16 dp after an
        // icon. Sized buttons use their symmetric Leading/TrailingSpace.
        let padding = if let Some(padding) = padding_override {
            padding
        } else if metrics.icon_padding_shrinks {
            if variant == MaterialButtonVariant::Text {
                (
                    12.0,
                    if has_leading && !has_trailing {
                        16.0
                    } else {
                        12.0
                    },
                )
            } else {
                (
                    if has_leading {
                        16.0
                    } else {
                        metrics.leading_space
                    },
                    if has_trailing {
                        16.0
                    } else {
                        metrics.trailing_space
                    },
                )
            }
        } else {
            (metrics.leading_space, metrics.trailing_space)
        };

        let mut content = Vec::new();
        if let Some(texture) = leading_texture {
            content.push(Content::Texture(texture, icon_size));
        }
        if let Some(glyph) = leading_glyph {
            content.push(Content::Galley(glyph));
        }
        if let Some(image) = image {
            let room = if text.is_some() {
                Vec2::splat(icon_size)
            } else {
                ui.available_size() - Vec2::new(padding.0 + padding.1, 0.0)
            };
            let size = image.load_and_calc_size(ui, room).unwrap_or(room);
            content.push(Content::Image(image, size));
        }

        let fixed_width: f32 = content.iter().map(|c| c.size().x).sum::<f32>()
            + trailing_glyph.as_ref().map_or(0.0, |g| g.size().x)
            + trailing_texture.as_ref().map_or(0.0, |_| icon_size)
            + metrics.icon_label_space * 3.0;
        let outer_width = width.unwrap_or_else(|| ui.available_width());
        let wrap_width = (outer_width - padding.0 - padding.1 - fixed_width).max(0.0);

        let mut text_label = String::new();
        if let Some(text) = text {
            let style = metrics.label.style();
            let text = match text {
                WidgetText::Text(s) => WidgetText::LayoutJob(Arc::new(style.layout_job(
                    ui.ctx(),
                    s,
                    Color32::PLACEHOLDER,
                ))),
                other => other,
            };
            let galley = text.into_galley(ui, wrap_mode, wrap_width, style.font_id(ui.ctx()));
            text_label = galley.text().to_owned();
            content.push(Content::Galley(galley));
        }
        if let Some(glyph) = trailing_glyph {
            content.push(Content::Galley(glyph));
        }
        if let Some(texture) = trailing_texture {
            content.push(Content::Texture(texture, icon_size));
        }

        let mut gaps = vec![metrics.icon_label_space; content.len().saturating_sub(1)];
        if !shortcut_text.is_empty() {
            let galley = shortcut_text.into_galley(
                ui,
                Some(TextWrapMode::Extend),
                f32::INFINITY,
                TextStyle::Body,
            );
            if !content.is_empty() {
                gaps.push(ui.spacing().item_spacing.x);
            }
            content.push(Content::Shortcut(galley));
        }

        let content_width: f32 =
            content.iter().map(|c| c.size().x).sum::<f32>() + gaps.iter().sum::<f32>();
        let content_height = content.iter().map(|c| c.size().y).fold(0.0_f32, f32::max);
        let mut desired = Vec2::new(
            padding.0 + content_width + padding.1,
            metrics.height.max(content_height + 8.0),
        );
        desired = desired.max(min_size);
        if let Some(width) = width {
            desired.x = width;
        }

        PreparedButton {
            variant,
            metrics,
            shape,
            content,
            label: label.unwrap_or(text_label),
            padding,
            gaps,
            desired,
            exact_width: width,
            fill,
            stroke,
            frame,
            sense,
            corner_radius,
            segment,
            toggle,
            role,
            image_tint_follows_text_color,
            elevation,
            enabled: !disabled,
            text_color,
        }
    }
}

impl PreparedButton<'_> {
    /// The width the button asks for.
    pub(crate) fn desired_width(&self) -> f32 {
        self.desired.x
    }

    /// Forces the allocated width.
    pub(crate) fn set_width(&mut self, width: f32) {
        self.desired.x = width;
        self.exact_width = Some(width);
    }

    /// Allocates the button and paints it.
    pub(crate) fn show(self, ui: &mut Ui) -> Response {
        let PreparedButton {
            variant,
            metrics,
            shape,
            content,
            label,
            padding,
            gaps,
            desired,
            exact_width,
            fill,
            stroke,
            frame,
            sense,
            corner_radius,
            segment,
            mut toggle,
            role,
            image_tint_follows_text_color,
            elevation,
            enabled,
            text_color,
        } = self;
        let enabled = enabled && ui.is_enabled();
        let sense = crate::a11y::sense(enabled, sense);
        let (rect, mut response) = if exact_width.is_some() {
            ui.allocate_exact_size(desired, sense)
        } else {
            ui.allocate_at_least(desired, sense)
        };

        if response.clicked() {
            if let ToggleState::Bound(selected) = &mut toggle {
                **selected = !**selected;
                response.mark_changed();
            }
        }
        let selected = toggle.selected();
        crate::a11y::describe(&response, role, enabled, &label, selected);

        if !ui.is_rect_visible(rect) {
            return response;
        }

        let theme = MaterialTheme::of(ui.ctx());
        let state = InteractionState::from_response(&response);
        let colors = variant.colors(&theme, selected, enabled);
        let content_color = match text_color {
            Some(color) if enabled => color,
            Some(_) | None => colors.content,
        };

        let corner_radius = corner_radius.unwrap_or_else(|| {
            animated_corners(
                ui,
                &response,
                &metrics,
                shape,
                segment,
                selected.unwrap_or(false),
            )
        });

        let container = if frame {
            fill.unwrap_or(colors.container)
        } else {
            Color32::TRANSPARENT
        };
        let container = if enabled {
            state_layer_over(container, content_color, state.opacity())
        } else {
            container
        };
        let outline = match (stroke, colors.outline) {
            (Some(stroke), _) => stroke,
            (None, Some(color)) if frame => Stroke::new(metrics.outline_width, color),
            (None, Some(_) | None) => Stroke::NONE,
        };

        let painter = ui.painter();
        if frame {
            match elevation {
                Some(shadow) => {
                    painter.add(shadow.as_shape(rect, corner_radius));
                }
                None => paint_elevation(
                    painter,
                    rect,
                    corner_radius,
                    variant.elevation(state, enabled),
                    theme.color(ColorRole::Shadow),
                ),
            }
        }
        painter.rect(rect, corner_radius, container, outline, StrokeKind::Inside);
        paint_focus_ring_if_focused(painter, &response, corner_radius);

        // Content, centred between the paddings.
        let content_width: f32 =
            content.iter().map(|c| c.size().x).sum::<f32>() + gaps.iter().sum::<f32>();
        let inner_left = rect.min.x + padding.0;
        let inner_width = (rect.width() - padding.0 - padding.1).max(0.0);
        let mut x = inner_left + ((inner_width - content_width) / 2.0).max(0.0);
        let weak = ui.visuals().weak_text_color();
        for (i, item) in content.into_iter().enumerate() {
            if i > 0 {
                x += gaps.get(i - 1).copied().unwrap_or(0.0);
            }
            let size = item.size();
            let item_rect =
                Rect::from_min_size(egui::pos2(x, rect.center().y - size.y / 2.0), size);
            match item {
                Content::Galley(galley) => {
                    ui.painter().galley(item_rect.min, galley, content_color);
                }
                Content::Shortcut(galley) => {
                    ui.painter().galley(item_rect.min, galley, weak);
                }
                Content::Texture(texture, _) => {
                    ui.painter().image(
                        texture.id(),
                        item_rect,
                        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        // White keeps the SVG's own colours (emoji, logos).
                        Color32::WHITE,
                    );
                }
                Content::Image(image, _) => {
                    let image = if image_tint_follows_text_color {
                        image.tint(content_color)
                    } else {
                        image
                    };
                    image.paint_at(ui, item_rect);
                }
            }
            x += size.x;
        }

        if let Some(cursor) = ui.visuals().interact_cursor {
            if response.hovered() {
                ui.ctx().set_cursor_icon(cursor);
            }
        }
        response
    }
}

/// The corner radii for this frame, springing towards the radius the
/// button's state calls for (pressed, selected, hovered segment, resting).
fn animated_corners(
    ui: &Ui,
    response: &Response,
    metrics: &ButtonMetrics,
    shape: ButtonShape,
    segment: Option<SegmentCorners>,
    selected: bool,
) -> CornerRadius {
    let theme = MaterialTheme::of(ui.ctx());
    let rect = response.rect;
    let state = InteractionState::from_response(response);
    let direction = theme.layout_direction;
    let spring = theme.motion.spatial(SpringSpeed::Fast);
    let animate = |salt: &str, target: f32| {
        animate_value_with_spring(ui.ctx(), response.id.with(salt), target, spring)
    };
    let to_u8 = |r: f32| r.round().clamp(0.0, 255.0) as u8;
    let half = rect.height().min(rect.width()) / 2.0;
    match segment {
        Some(segment) => {
            let height = rect.height();
            let selected = selected || segment.selected;
            let side = |corners: SideCorners| {
                corners
                    .radius(height, state.hovered, state.pressed, selected)
                    .min(half)
            };
            let start = to_u8(animate("m3_corner_start", side(segment.start)));
            let end = to_u8(animate("m3_corner_end", side(segment.end)));
            let (left, right) = match direction {
                LayoutDirection::Ltr => (start, end),
                LayoutDirection::Rtl => (end, start),
            };
            CornerRadius {
                nw: left,
                sw: left,
                ne: right,
                se: right,
            }
        }
        None => {
            let corner = if state.pressed {
                metrics.pressed_shape
            } else if selected {
                metrics.selected_shape(shape)
            } else {
                metrics.shape(shape)
            };
            CornerRadius::same(to_u8(animate("m3_corner", corner.radius(rect))))
        }
    }
}

impl Widget for MaterialButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.prepare(ui).show(ui)
    }
}
