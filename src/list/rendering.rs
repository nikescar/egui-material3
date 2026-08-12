//! Rendering implementation for MaterialList

use super::{types::*, MaterialList};
use crate::{get_global_color, material_symbol::material_symbol_text};
use egui::{ecolor::Color32, epaint::Stroke, pos2, Align, FontFamily, FontId, Pos2, Rect, Response, Sense, Ui, Vec2, Widget, WidgetText};


impl<'a> Widget for MaterialList<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Material Design 3 Color Roles
        // Surface & Outline Roles - for backgrounds and low-emphasis areas
        let surface_container_lowest = get_global_color("surfaceContainerLowest");
        let on_surface = get_global_color("onSurface"); // Content on surface
        let on_surface_variant = get_global_color("onSurfaceVariant"); // Lower emphasis content
        let outline_variant = get_global_color("outlineVariant"); // Borders and dividers

        // Accent Color Roles - for selection states
        let primary_container = get_global_color("primaryContainer"); // Selected background
        let on_primary_container = get_global_color("onPrimaryContainer"); // Content on selected

        // Calculate total height and max width
        let mut total_height = 0.0;
        let mut max_content_width = 200.0;

        for item in &self.items {
            // Calculate item height based on configuration
            let visual_density = item.visual_density.unwrap_or_default();
            let density_adjustment = visual_density.base_size_adjustment().y;
            let is_dense = item.dense.unwrap_or(false);

            let base_height = if item.is_three_line.unwrap_or(false)
                || (item.overline_text.is_some() && item.secondary_text.is_some())
            {
                if is_dense {
                    76.0
                } else {
                    88.0
                }
            } else if item.secondary_text.is_some() || item.overline_text.is_some() {
                if is_dense {
                    64.0
                } else {
                    72.0
                }
            } else {
                if is_dense {
                    48.0
                } else {
                    56.0
                }
            };

            let item_height = item
                .min_tile_height
                .unwrap_or(base_height + density_adjustment);
            total_height += item_height;

            // Calculate item width
            let mut item_width = 32.0; // base padding
            if item.leading_icon.is_some() {
                item_width += item.min_leading_width.unwrap_or(40.0);
            }
            let primary_text_width = item.primary_text.len() as f32 * 8.0;
            let secondary_text_width = item
                .secondary_text
                .as_ref()
                .map_or(0.0, |s| s.len() as f32 * 6.0);
            let overline_text_width = item
                .overline_text
                .as_ref()
                .map_or(0.0, |s| s.len() as f32 * 5.5);
            let max_text_width = primary_text_width
                .max(secondary_text_width)
                .max(overline_text_width);
            item_width += max_text_width;
            if let Some(ref trailing_text) = item.trailing_text {
                item_width += trailing_text.len() as f32 * 6.0;
            }
            if item.trailing_icon.is_some() {
                item_width += 40.0;
            }
            item_width += 32.0;

            if item_width > max_content_width {
                max_content_width = item_width;
            }
        }

        if self.dividers && self.items.len() > 1 {
            total_height += (self.items.len() - 1) as f32;
        }

        let list_width = max_content_width.min(ui.available_width());
        let desired_size = Vec2::new(list_width, total_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Draw list background using surfaceContainerLowest (lowest emphasis surface container)
        ui.painter().rect_filled(rect, 8.0, surface_container_lowest);
        // Draw border using outlineVariant (less emphasized outline for structure)
        ui.painter().rect_stroke(
            rect,
            8.0,
            Stroke::new(1.0, outline_variant),
            egui::epaint::StrokeKind::Outside,
        );

        let mut current_y = rect.min.y;
        let mut pending_actions = Vec::new();
        let items_len = self.items.len();

        for (index, item) in self.items.into_iter().enumerate() {
            // Calculate item-specific dimensions
            let visual_density = item.visual_density.unwrap_or_default();
            let density_adjustment = visual_density.base_size_adjustment().y;
            let is_dense = item.dense.unwrap_or(false);

            let base_height = if item.is_three_line.unwrap_or(false)
                || (item.overline_text.is_some() && item.secondary_text.is_some())
            {
                if is_dense {
                    76.0
                } else {
                    88.0
                }
            } else if item.secondary_text.is_some() || item.overline_text.is_some() {
                if is_dense {
                    64.0
                } else {
                    72.0
                }
            } else {
                if is_dense {
                    48.0
                } else {
                    56.0
                }
            };

            let item_height = item
                .min_tile_height
                .unwrap_or(base_height + density_adjustment);

            let item_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, current_y),
                Vec2::new(rect.width(), item_height),
            );

            // Use list's ID (or auto-generate one) to scope item IDs and avoid collisions
            let list_id = self.id.unwrap_or_else(|| ui.id().with("material_list"));
            let unique_id = list_id.with(("item", index));
            let item_response = ui.interact(item_rect, unique_id, Sense::click());

            // Determine background color using M3 color roles
            let bg_color = if item.selected {
                // Selected state: use primaryContainer (less emphasized fill for selected elements)
                item.selected_tile_color.unwrap_or_else(|| {
                    Color32::from_rgba_premultiplied(
                        primary_container.r(),
                        primary_container.g(),
                        primary_container.b(),
                        255,
                    )
                })
            } else {
                // Non-selected state: transparent to show parent surface
                item.tile_color.unwrap_or(Color32::TRANSPARENT)
            };

            // Draw background
            if bg_color != Color32::TRANSPARENT {
                ui.painter().rect_filled(item_rect, 0.0, bg_color);
            }

            // Draw hover state layer (M3 interaction state overlay)
            if item_response.hovered() && item.enabled {
                // Use onSurface with 8% opacity for hover state layer (M3 spec)
                let hover_color = Color32::from_rgba_premultiplied(
                    on_surface.r(),
                    on_surface.g(),
                    on_surface.b(),
                    20, // ~8% opacity (20/255 ≈ 0.078)
                );
                ui.painter().rect_filled(item_rect, 0.0, hover_color);
            }

            // Handle click
            if item_response.clicked() && item.enabled {
                if let Some(action) = item.action {
                    pending_actions.push(action);
                }
            }

            // Calculate colors using M3 color roles
            let icon_color = if item.selected {
                // Selected: use onPrimaryContainer (content on primaryContainer)
                item.selected_color.unwrap_or(on_primary_container)
            } else if item.enabled {
                // Enabled: use onSurfaceVariant (lower emphasis for icons)
                item.icon_color.unwrap_or(on_surface_variant)
            } else {
                // Disabled: use onSurfaceVariant with 38% opacity (M3 disabled state)
                on_surface_variant.linear_multiply(0.38)
            };

            let text_color = if item.selected {
                // Selected: use onPrimaryContainer (content on primaryContainer)
                item.selected_color.unwrap_or(on_primary_container)
            } else if item.enabled {
                // Enabled: use onSurface (standard content color on surface)
                item.text_color.unwrap_or(on_surface)
            } else {
                // Disabled: use onSurface with 38% opacity (M3 disabled state)
                on_surface.linear_multiply(0.38)
            };

            // Layout constants
            let horizontal_title_gap = item.horizontal_title_gap.unwrap_or(16.0)
                + visual_density.horizontal * 2.0;
            let _min_vertical_padding = item.min_vertical_padding.unwrap_or(8.0);
            let min_leading_width = item.min_leading_width.unwrap_or(40.0);
            
            let mut content_x = item_rect.min.x + 16.0;
            let content_y = item_rect.center().y;

            // Draw leading icon
            if let Some(icon_name) = &item.leading_icon {
                let leading_width = min_leading_width;
                let icon_pos = Pos2::new(content_x + leading_width / 2.0, content_y);

                let icon_string = material_symbol_text(icon_name);
                ui.painter().text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    &icon_string,
                    egui::FontId::proportional(20.0),
                    icon_color,
                );
                content_x += leading_width + horizontal_title_gap;
            }

            // Calculate trailing width
            let trailing_icon_width = if item.trailing_icon.is_some() {
                40.0
            } else {
                0.0
            };
            let trailing_text_width = if item.trailing_text.is_some() {
                80.0
            } else {
                0.0
            };
            let _total_trailing_width = trailing_icon_width + trailing_text_width;

            // Draw text content based on configuration
            match (&item.overline_text, &item.secondary_text) {
                (Some(overline), Some(secondary)) => {
                    // Three-line layout
                    let overline_pos = Pos2::new(content_x, content_y - 20.0);
                    let primary_pos = Pos2::new(content_x, content_y);
                    let secondary_pos = Pos2::new(content_x, content_y + 20.0);

                    // Overline: use onSurfaceVariant (lower emphasis supporting text)
                    ui.painter().text(
                        overline_pos,
                        egui::Align2::LEFT_CENTER,
                        overline,
                        egui::FontId::proportional(if is_dense { 10.0 } else { 11.0 }),
                        on_surface_variant,
                    );

                    // Primary text: use calculated text_color (onSurface or onPrimaryContainer)
                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );

                    // Secondary text: use onSurfaceVariant (lower emphasis supporting text)
                    ui.painter().text(
                        secondary_pos,
                        egui::Align2::LEFT_CENTER,
                        secondary,
                        egui::FontId::proportional(if is_dense { 11.0 } else { 12.0 }),
                        on_surface_variant,
                    );
                }
                (Some(overline), None) => {
                    // Two-line layout: overline + primary
                    let overline_pos = Pos2::new(content_x, content_y - 10.0);
                    let primary_pos = Pos2::new(content_x, content_y + 10.0);

                    // Overline: use onSurfaceVariant (lower emphasis supporting text)
                    ui.painter().text(
                        overline_pos,
                        egui::Align2::LEFT_CENTER,
                        overline,
                        egui::FontId::proportional(if is_dense { 10.0 } else { 11.0 }),
                        on_surface_variant,
                    );

                    // Primary text: use calculated text_color (onSurface or onPrimaryContainer)
                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );
                }
                (None, Some(secondary)) => {
                    // Two-line layout: primary + secondary
                    let primary_pos = Pos2::new(content_x, content_y - 10.0);
                    let secondary_pos = Pos2::new(content_x, content_y + 10.0);

                    // Primary text: use calculated text_color (onSurface or onPrimaryContainer)
                    ui.painter().text(
                        primary_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );

                    // Secondary text: use onSurfaceVariant (lower emphasis supporting text)
                    ui.painter().text(
                        secondary_pos,
                        egui::Align2::LEFT_CENTER,
                        secondary,
                        egui::FontId::proportional(if is_dense { 11.0 } else { 12.0 }),
                        on_surface_variant,
                    );
                }
                (None, None) => {
                    // Single-line layout
                    let text_pos = Pos2::new(content_x, content_y);
                    // Primary text: use calculated text_color (onSurface or onPrimaryContainer)
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        &item.primary_text,
                        egui::FontId::proportional(if is_dense { 13.0 } else { 14.0 }),
                        text_color,
                    );
                }
            }

            // Draw trailing text (e.g., badges, counts)
            if let Some(ref trailing_text) = item.trailing_text {
                let trailing_text_pos = Pos2::new(
                    item_rect.max.x - trailing_icon_width - trailing_text_width + 10.0,
                    content_y,
                );

                // Trailing text: use onSurfaceVariant (lower emphasis supporting content)
                ui.painter().text(
                    trailing_text_pos,
                    egui::Align2::LEFT_CENTER,
                    trailing_text,
                    egui::FontId::proportional(12.0),
                    on_surface_variant,
                );
            }

            // Draw trailing icon
            if let Some(icon_name) = &item.trailing_icon {
                let icon_pos = Pos2::new(item_rect.max.x - 28.0, content_y);

                let icon_string = material_symbol_text(icon_name);
                ui.painter().text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    &icon_string,
                    egui::FontId::proportional(20.0),
                    icon_color,
                );
            }

            current_y += item_height;

            // Draw divider between items
            if self.dividers && index < items_len - 1 {
                let divider_y = current_y;
                let divider_start = Pos2::new(rect.min.x + 16.0, divider_y);
                let divider_end = Pos2::new(rect.max.x - 16.0, divider_y);

                // Divider: use outlineVariant (less emphasized outline for structure)
                ui.painter().line_segment(
                    [divider_start, divider_end],
                    Stroke::new(1.0, outline_variant),
                );
                current_y += 1.0;
            }
        }

        // Execute pending actions
        for action in pending_actions {
            action();
        }

        response
    }
}

pub fn list_item(primary_text: impl Into<String>) -> ListItem<'static> {
    ListItem::new(primary_text)
}

pub fn list() -> MaterialList<'static> {
    MaterialList::new()
}
