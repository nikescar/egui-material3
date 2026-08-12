//! Rendering implementation for MaterialButton

use super::{types::*, MaterialButton};
use crate::{get_global_color, material_symbol::material_symbol_text};
use egui::{
    ecolor::Color32,
    emath::NumExt,
    epaint::{CornerRadius, Shadow, Stroke},
    Align, Rect, Response, TextStyle, TextWrapMode, Ui, Vec2, Widget, WidgetInfo, WidgetText,
    WidgetType,
};

impl Widget for MaterialButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialButton {
            variant,
            text,
            image,
            shortcut_text,
            wrap_mode,
            fill,
            stroke,
            sense,
            small,
            frame,
            min_size,
            corner_radius,
            selected,
            image_tint_follows_text_color,
            elevation,
            disabled,
            leading_icon,
            trailing_icon,
            leading_svg,
            trailing_svg,
            text_color: custom_text_color,
        } = self;

        // M3 Color Roles - Button Variants
        let primary = get_global_color("primary"); // Filled button background
        let on_primary = get_global_color("onPrimary"); // Content on primary background
        let secondary_container = get_global_color("secondaryContainer"); // Tonal button background
        let on_secondary_container = get_global_color("onSecondaryContainer"); // Content on tonal background
        let surface = get_global_color("surface"); // Elevated button background, disabled button background
        let on_surface = get_global_color("onSurface"); // Content on surface, disabled content @ 38%
        let outline = get_global_color("outline"); // Outlined button border

        // Material Design button defaults based on variant
        let (default_fill, default_stroke, default_corner_radius, _has_elevation) = match variant {
            MaterialButtonVariant::Filled => (
                Some(primary), // Use primary for high-emphasis filled button background
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false,
            ),
            MaterialButtonVariant::Outlined => (
                Some(Color32::TRANSPARENT),      // Transparent to show parent surface
                Some(Stroke::new(1.0, outline)), // Use outline for medium-emphasis border
                CornerRadius::from(20),
                false,
            ),
            MaterialButtonVariant::Text => (
                Some(Color32::TRANSPARENT), // Transparent to show parent surface
                Some(Stroke::NONE),         // No border for low-emphasis text button
                CornerRadius::from(20),
                false,
            ),
            MaterialButtonVariant::Elevated => (
                Some(surface), // Use surface for elevated container background
                Some(Stroke::NONE),
                CornerRadius::from(20),
                true,
            ),
            MaterialButtonVariant::FilledTonal => (
                Some(secondary_container), // Use secondaryContainer for toned-down emphasis
                Some(Stroke::NONE),
                CornerRadius::from(20),
                false,
            ),
        };

        let frame = frame.unwrap_or(!matches!(variant, MaterialButtonVariant::Text));

        // Load SVG textures early if provided (takes precedence over font icons)
        let leading_svg_texture = leading_svg.and_then(|svg_data| {
            crate::image_utils::create_texture_from_svg(
                ui.ctx(),
                &svg_data,
                &format!("btn_lead_{}", svg_data.len()),
            )
            .ok()
        });
        let trailing_svg_texture = trailing_svg.and_then(|svg_data| {
            crate::image_utils::create_texture_from_svg(
                ui.ctx(),
                &svg_data,
                &format!("btn_trail_{}", svg_data.len()),
            )
            .ok()
        });

        // Build icon galleys early (only if no SVG provided)
        let leading_icon_galley = if leading_svg_texture.is_none() {
            leading_icon.map(|name| {
                let icon_str: WidgetText = material_symbol_text(&name).into();
                icon_str.into_galley(
                    ui,
                    Some(TextWrapMode::Extend),
                    f32::INFINITY,
                    TextStyle::Body,
                )
            })
        } else {
            None
        };
        let trailing_icon_galley = if trailing_svg_texture.is_none() {
            trailing_icon.map(|name| {
                let icon_str: WidgetText = material_symbol_text(&name).into();
                icon_str.into_galley(
                    ui,
                    Some(TextWrapMode::Extend),
                    f32::INFINITY,
                    TextStyle::Body,
                )
            })
        } else {
            None
        };

        // Material Design button padding
        // With leading icon: 16px left, 24px right
        // With trailing icon: 24px left, 16px right
        // With both icons: 16px left, 16px right
        // No icons: 24px left, 24px right
        // For small buttons: 4px (with icon) or 6px (without icon)
        let has_leading =
            leading_icon_galley.is_some() || leading_svg_texture.is_some() || image.is_some();
        let has_trailing = trailing_icon_galley.is_some() || trailing_svg_texture.is_some();
        let padding_multiplier = if small { 0.25 } else { 1.0 };
        let padding_left = if has_leading { 16.0 } else { 24.0 } * padding_multiplier;
        let padding_right = if has_trailing { 16.0 } else { 24.0 } * padding_multiplier;
        let button_padding_left;
        let button_padding_right;
        let button_padding_y;
        if frame || variant == MaterialButtonVariant::Text {
            button_padding_left = padding_left;
            button_padding_right = padding_right;
            button_padding_y = if small { 4.0 } else { 10.0 };
        } else {
            button_padding_left = 0.0;
            button_padding_right = 0.0;
            button_padding_y = 0.0;
        }

        // Material Design minimum button height
        let min_button_height = if small { 32.0 } else { 40.0 };
        let icon_spacing = if small { 4.0 } else { 8.0 }; // Material Design icon-to-text gap
        let svg_icon_size = 18.0; // Size for SVG icons

        // Resolve the variant-based text color (used for text and icons)
        let resolved_text_color = if disabled {
            // Disabled state: use onSurface @ 38% opacity (M3 spec)
            on_surface.linear_multiply(0.38)
        } else if let Some(custom) = custom_text_color {
            custom
        } else {
            match variant {
                MaterialButtonVariant::Filled => on_primary, // Use onPrimary for content on primary background
                MaterialButtonVariant::Outlined => on_surface, // Use onSurface for content on transparent surface
                MaterialButtonVariant::Text => on_surface, // Use onSurface for content on transparent surface
                MaterialButtonVariant::Elevated => on_surface, // Use onSurface for content on elevated surface
                MaterialButtonVariant::FilledTonal => on_secondary_container, // Use onSecondaryContainer for content on tinted background
            }
        };

        let space_available_for_image = if let Some(_text) = &text {
            let font_height = ui.text_style_height(&TextStyle::Body);
            Vec2::splat(font_height)
        } else {
            let total_h_padding = button_padding_left + button_padding_right;
            ui.available_size() - Vec2::new(total_h_padding, 2.0 * button_padding_y)
        };

        let image_size = if let Some(image) = &image {
            image
                .load_and_calc_size(ui, space_available_for_image)
                .unwrap_or(space_available_for_image)
        } else {
            Vec2::ZERO
        };

        let gap_before_shortcut_text = ui.spacing().item_spacing.x;

        let mut text_wrap_width = ui.available_width() - button_padding_left - button_padding_right;
        if image.is_some() {
            text_wrap_width -= image_size.x + icon_spacing;
        }
        if let Some(galley) = &leading_icon_galley {
            text_wrap_width -= galley.size().x + icon_spacing;
        }
        if leading_svg_texture.is_some() {
            text_wrap_width -= svg_icon_size + icon_spacing;
        }
        if let Some(galley) = &trailing_icon_galley {
            text_wrap_width -= galley.size().x + icon_spacing;
        }
        if trailing_svg_texture.is_some() {
            text_wrap_width -= svg_icon_size + icon_spacing;
        }

        // Note: we don't wrap the shortcut text
        let shortcut_galley = (!shortcut_text.is_empty()).then(|| {
            shortcut_text.into_galley(
                ui,
                Some(TextWrapMode::Extend),
                f32::INFINITY,
                TextStyle::Body,
            )
        });

        if let Some(shortcut_galley) = &shortcut_galley {
            text_wrap_width -= gap_before_shortcut_text + shortcut_galley.size().x;
        }

        let galley =
            text.map(|text| text.into_galley(ui, wrap_mode, text_wrap_width, TextStyle::Body));

        let mut desired_size = Vec2::ZERO;

        // Leading icon (font or SVG)
        if let Some(lg) = &leading_icon_galley {
            desired_size.x += lg.size().x;
            desired_size.y = desired_size.y.max(lg.size().y);
        }
        if leading_svg_texture.is_some() {
            desired_size.x += svg_icon_size;
            desired_size.y = desired_size.y.max(svg_icon_size);
        }

        // Image
        if image.is_some() {
            if leading_icon_galley.is_some() || leading_svg_texture.is_some() {
                desired_size.x += icon_spacing;
            }
            desired_size.x += image_size.x;
            desired_size.y = desired_size.y.max(image_size.y);
        }

        // Gap between leading content and text
        if (leading_icon_galley.is_some() || leading_svg_texture.is_some() || image.is_some())
            && galley.is_some()
        {
            desired_size.x += icon_spacing;
        }

        if let Some(galley) = &galley {
            desired_size.x += galley.size().x;
            desired_size.y = desired_size.y.max(galley.size().y);
        }

        // Trailing icon (font or SVG)
        if let Some(tg) = &trailing_icon_galley {
            if galley.is_some()
                || image.is_some()
                || leading_icon_galley.is_some()
                || leading_svg_texture.is_some()
            {
                desired_size.x += icon_spacing;
            }
            desired_size.x += tg.size().x;
            desired_size.y = desired_size.y.max(tg.size().y);
        }
        if trailing_svg_texture.is_some() {
            if galley.is_some()
                || image.is_some()
                || leading_icon_galley.is_some()
                || leading_svg_texture.is_some()
            {
                desired_size.x += icon_spacing;
            }
            desired_size.x += svg_icon_size;
            desired_size.y = desired_size.y.max(svg_icon_size);
        }

        if let Some(shortcut_galley) = &shortcut_galley {
            desired_size.x += gap_before_shortcut_text + shortcut_galley.size().x;
            desired_size.y = desired_size.y.max(shortcut_galley.size().y);
        }

        desired_size.x += button_padding_left + button_padding_right;
        desired_size.y += 2.0 * button_padding_y;
        if !small {
            desired_size.y = desired_size.y.at_least(min_button_height);
        }
        desired_size = desired_size.at_least(min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, sense);
        response.widget_info(|| {
            if let Some(galley) = &galley {
                WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), galley.text())
            } else {
                WidgetInfo::new(WidgetType::Button)
            }
        });

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            let (frame_expansion, _frame_cr, frame_fill, frame_stroke) = if selected {
                let selection = ui.visuals().selection;
                (
                    Vec2::ZERO,
                    CornerRadius::ZERO,
                    selection.bg_fill,
                    selection.stroke,
                )
            } else if frame {
                let expansion = Vec2::splat(visuals.expansion);
                (
                    expansion,
                    visuals.corner_radius,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                )
            } else {
                Default::default()
            };
            let frame_cr = corner_radius.unwrap_or(default_corner_radius);
            let mut frame_fill = fill.unwrap_or(default_fill.unwrap_or(frame_fill));
            let mut frame_stroke = stroke.unwrap_or(default_stroke.unwrap_or(frame_stroke));

            // Apply disabled styling (M3 spec: 38% opacity content, 12% opacity outline)
            if disabled {
                frame_fill = surface; // Use surface for disabled button background
                frame_stroke.color = on_surface.linear_multiply(0.12); // 12% opacity for disabled outline
                frame_stroke.width = if matches!(variant, MaterialButtonVariant::Outlined) {
                    1.0 // Keep 1dp border for outlined variant
                } else {
                    0.0 // No border for other variants
                };
            }

            // M3 state layers: interactive overlay on hover/press
            if !disabled {
                let state_layer_color = resolved_text_color;
                if response.is_pointer_button_down_on() {
                    // Pressed state: 12% opacity overlay (M3 interaction state)
                    frame_fill = blend_overlay(frame_fill, state_layer_color, 0.12);
                } else if response.hovered() {
                    // Hover state: 8% opacity overlay (M3 interaction state)
                    frame_fill = blend_overlay(frame_fill, state_layer_color, 0.08);
                }
            }

            // Draw elevation shadow if present
            if let Some(shadow) = &elevation {
                // Hover elevation boost for elevated buttons
                let shadow = if !disabled && response.hovered() {
                    Shadow {
                        offset: [shadow.offset[0], shadow.offset[1] + 2],
                        blur: shadow.blur + 4,
                        spread: shadow.spread,
                        color: shadow.color,
                    }
                } else {
                    *shadow
                };
                let shadow_offset = Vec2::new(shadow.offset[0] as f32, shadow.offset[1] as f32);
                let shadow_rect = rect.expand2(frame_expansion).translate(shadow_offset);
                ui.painter()
                    .rect_filled(shadow_rect, frame_cr, shadow.color);
            }

            ui.painter().rect(
                rect.expand2(frame_expansion),
                frame_cr,
                frame_fill,
                frame_stroke,
                egui::epaint::StrokeKind::Outside,
            );

            let mut cursor_x = rect.min.x + button_padding_left;
            let content_rect_y_min = rect.min.y + button_padding_y;
            let content_rect_y_max = rect.max.y - button_padding_y;
            let content_height = content_rect_y_max - content_rect_y_min;

            // Draw leading icon (font icon)
            if let Some(leading_galley) = &leading_icon_galley {
                let icon_y = content_rect_y_min + (content_height - leading_galley.size().y) / 2.0;
                let icon_pos = egui::pos2(cursor_x, icon_y);
                ui.painter()
                    .galley(icon_pos, leading_galley.clone(), resolved_text_color);
                cursor_x += leading_galley.size().x + icon_spacing;
            }

            // Draw leading icon (SVG texture)
            if let Some(texture) = &leading_svg_texture {
                let icon_y = content_rect_y_min + (content_height - svg_icon_size) / 2.0;
                let icon_rect =
                    Rect::from_min_size(egui::pos2(cursor_x, icon_y), Vec2::splat(svg_icon_size));
                ui.painter().image(
                    texture.id(),
                    icon_rect,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE, // Use WHITE to preserve original SVG colors (e.g., emoji)
                );
                cursor_x += svg_icon_size;
                // Add spacing only if there's content after the icon
                if image.is_some()
                    || galley.is_some()
                    || trailing_icon_galley.is_some()
                    || trailing_svg_texture.is_some()
                    || shortcut_galley.is_some()
                {
                    cursor_x += icon_spacing;
                }
            }

            // Draw image
            if let Some(image) = &image {
                let mut image_pos = ui
                    .layout()
                    .align_size_within_rect(
                        image_size,
                        Rect::from_min_max(
                            egui::pos2(cursor_x, content_rect_y_min),
                            egui::pos2(rect.max.x - button_padding_right, content_rect_y_max),
                        ),
                    )
                    .min;
                if galley.is_some() || shortcut_galley.is_some() || trailing_icon_galley.is_some() {
                    image_pos.x = cursor_x;
                }
                let image_rect = Rect::from_min_size(image_pos, image_size);
                cursor_x += image_size.x + icon_spacing;
                let mut image_widget = image.clone();
                if image_tint_follows_text_color {
                    image_widget = image_widget.tint(visuals.text_color());
                }
                image_widget.paint_at(ui, image_rect);
            }

            // Draw main text
            let has_text = galley.is_some();
            if let Some(galley) = galley {
                let text_y = content_rect_y_min
                    + (content_height - galley.size().y) / 2.0
                    + if small { 1.0 } else { 0.0 };
                let mut text_pos = egui::pos2(cursor_x, text_y);
                // Center text if no leading/trailing elements
                if leading_icon_galley.is_none()
                    && leading_svg_texture.is_none()
                    && image.is_none()
                    && trailing_icon_galley.is_none()
                    && trailing_svg_texture.is_none()
                    && shortcut_galley.is_none()
                {
                    text_pos = ui
                        .layout()
                        .align_size_within_rect(
                            galley.size(),
                            Rect::from_min_max(
                                egui::pos2(rect.min.x + button_padding_left, content_rect_y_min),
                                egui::pos2(rect.max.x - button_padding_right, content_rect_y_max),
                            ),
                        )
                        .min;
                }

                cursor_x = text_pos.x + galley.size().x;
                ui.painter().galley(text_pos, galley, resolved_text_color);
            }

            // Draw trailing icon (font icon)
            if let Some(trailing_galley) = &trailing_icon_galley {
                cursor_x += icon_spacing;
                let icon_y = content_rect_y_min + (content_height - trailing_galley.size().y) / 2.0;
                let icon_pos = egui::pos2(cursor_x, icon_y);
                ui.painter()
                    .galley(icon_pos, trailing_galley.clone(), resolved_text_color);
            }

            // Draw trailing icon (SVG texture)
            if let Some(texture) = &trailing_svg_texture {
                // Add spacing before the icon if there's content before it
                if has_text
                    || image.is_some()
                    || leading_icon_galley.is_some()
                    || leading_svg_texture.is_some()
                {
                    cursor_x += icon_spacing;
                }
                let icon_y = content_rect_y_min + (content_height - svg_icon_size) / 2.0;
                let icon_rect =
                    Rect::from_min_size(egui::pos2(cursor_x, icon_y), Vec2::splat(svg_icon_size));
                ui.painter().image(
                    texture.id(),
                    icon_rect,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE, // Use WHITE to preserve original SVG colors (e.g., emoji)
                );
            }

            // Draw shortcut text
            if let Some(shortcut_galley) = shortcut_galley {
                let layout = if ui.layout().is_horizontal() {
                    ui.layout().with_main_align(Align::Max)
                } else {
                    ui.layout().with_cross_align(Align::Max)
                };
                let shortcut_text_pos = layout
                    .align_size_within_rect(
                        shortcut_galley.size(),
                        Rect::from_min_max(
                            egui::pos2(rect.min.x + button_padding_left, content_rect_y_min),
                            egui::pos2(rect.max.x - button_padding_right, content_rect_y_max),
                        ),
                    )
                    .min;
                ui.painter().galley(
                    shortcut_text_pos,
                    shortcut_galley,
                    ui.visuals().weak_text_color(),
                );
            }
        }

        if let Some(cursor) = ui.visuals().interact_cursor {
            if response.hovered() {
                ui.ctx().set_cursor_icon(cursor);
            }
        }

        response
    }
}

/// Blend an overlay color on top of a base color with given opacity.
fn blend_overlay(base: Color32, overlay: Color32, opacity: f32) -> Color32 {
    let alpha = (opacity * 255.0) as u8;
    let overlay_with_alpha =
        Color32::from_rgba_unmultiplied(overlay.r(), overlay.g(), overlay.b(), alpha);
    // Simple alpha blending
    let inv_alpha = 255 - alpha;
    Color32::from_rgba_unmultiplied(
        ((base.r() as u16 * inv_alpha as u16 + overlay_with_alpha.r() as u16 * alpha as u16) / 255)
            as u8,
        ((base.g() as u16 * inv_alpha as u16 + overlay_with_alpha.g() as u16 * alpha as u16) / 255)
            as u8,
        ((base.b() as u16 * inv_alpha as u16 + overlay_with_alpha.b() as u16 * alpha as u16) / 255)
            as u8,
        base.a(),
    )
}
