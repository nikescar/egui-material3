//! Material Design 3 Tree View Components
//!
//! # M3 Color Role Usage
//!
//! - **onSurface**: Node text
//! - **onSurfaceVariant**: Expand/collapse icons
//! - **secondaryContainer** / **onSecondaryContainer**: selected node
//! - **State layers**: content colour at the `StateLayerTokens` opacities
//!
//! M3 has no tree component; rows follow the list tokens where they apply
//! (`ListTokens.ItemLabelTextFont` `BodyLarge`, `ItemLeadingIconSize` 24 dp,
//! `ItemSelectedContainerColor` `secondaryContainer`).
//!
//! Every row is one AccessKit `TreeItem` node (with its selected state)
//! inside a `Tree`, and is a Tab stop. Keyboard: Up/Down move between
//! visible rows, Right expands a row (or moves to its first child), Left
//! collapses it (or moves to its parent), Space or Enter selects. The
//! widget's response is the union of its rows: `clicked()` when a row was
//! clicked, `changed()` when selection or expansion changed.

use crate::material_symbol::material_symbol_text;
use crate::material_theme::MaterialTheme;
use crate::tokens::{ColorRole, InteractionState, LayoutDirection, TypeRole};
use egui::{EventFilter, Id, Key, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};
use std::collections::HashMap;

/// A tree view item that can contain child items
#[derive(Clone, Debug)]
pub struct TreeViewItem {
    /// Unique identifier for this item
    pub id: String,
    /// Display label for the item
    pub label: String,
    /// Optional icon (Material Symbol name)
    pub icon: Option<String>,
    /// Child items
    pub children: Vec<TreeViewItem>,
    /// Whether this item is selectable
    pub selectable: bool,
    /// Whether this item can toggle (show/hide children)
    pub toggleable: bool,
}

impl TreeViewItem {
    /// Create a new tree view item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            children: Vec::new(),
            selectable: true,
            toggleable: true,
        }
    }

    /// Set the icon for this item
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add a child item
    pub fn child(mut self, child: TreeViewItem) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple children
    pub fn children(mut self, children: Vec<TreeViewItem>) -> Self {
        self.children = children;
        self
    }

    /// Set whether this item is selectable
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Set whether this item is toggleable
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    /// Check if this item has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// State management for tree view
#[derive(Clone, Debug, Default)]
pub struct TreeViewState {
    /// Map of item ID to whether it's expanded
    pub expanded: HashMap<String, bool>,
    /// Map of item ID to whether it's selected
    pub selected: HashMap<String, bool>,
}

impl TreeViewState {
    /// Create a new tree view state
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if an item is expanded
    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded.get(id).copied().unwrap_or(false)
    }

    /// Toggle the expanded state of an item
    pub fn toggle_expanded(&mut self, id: &str) {
        let current = self.is_expanded(id);
        self.expanded.insert(id.to_string(), !current);
    }

    /// Set the expanded state of an item
    pub fn set_expanded(&mut self, id: &str, expanded: bool) {
        self.expanded.insert(id.to_string(), expanded);
    }

    /// Check if an item is selected
    pub fn is_selected(&self, id: &str) -> bool {
        self.selected.get(id).copied().unwrap_or(false)
    }

    /// Toggle the selected state of an item
    pub fn toggle_selected(&mut self, id: &str) {
        let current = self.is_selected(id);
        self.selected.insert(id.to_string(), !current);
    }

    /// Set the selected state of an item
    pub fn set_selected(&mut self, id: &str, selected: bool) {
        self.selected.insert(id.to_string(), selected);
    }

    /// Clear all selections
    pub fn clear_selections(&mut self) {
        self.selected.clear();
    }

    /// Expand all items
    pub fn expand_all(&mut self, items: &[TreeViewItem]) {
        fn expand_recursive(state: &mut TreeViewState, items: &[TreeViewItem]) {
            for item in items {
                if item.has_children() {
                    state.set_expanded(&item.id, true);
                    expand_recursive(state, &item.children);
                }
            }
        }
        expand_recursive(self, items);
    }

    /// Collapse all items
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }
}

/// Material Design tree view component
///
/// A hierarchical tree view component that supports expand/collapse,
/// selection, and icons following Material Design guidelines.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{TreeViewItem, TreeViewState, MaterialTreeView};
///
/// let mut state = TreeViewState::new();
/// let items = vec![
///     TreeViewItem::new("1", "Root Item")
///         .icon("folder")
///         .child(TreeViewItem::new("1.1", "Child 1"))
///         .child(TreeViewItem::new("1.2", "Child 2")),
/// ];
///
/// ui.add(MaterialTreeView::new(&items, &mut state));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialTreeView<'a> {
    items: &'a [TreeViewItem],
    state: &'a mut TreeViewState,
    indent_width: f32,
    item_height: f32,
    id_salt: Option<egui::IdSalt>,
}

/// A row on screen: an item and its depth, with its parent's row.
struct Row<'i> {
    item: &'i TreeViewItem,
    depth: usize,
    parent: Option<usize>,
}

fn visible_rows<'i>(
    items: &'i [TreeViewItem],
    state: &TreeViewState,
    depth: usize,
    parent: Option<usize>,
    rows: &mut Vec<Row<'i>>,
) {
    for item in items {
        let index = rows.len();
        rows.push(Row {
            item,
            depth,
            parent,
        });
        if item.has_children() && state.is_expanded(&item.id) {
            visible_rows(&item.children, state, depth + 1, Some(index), rows);
        }
    }
}

/// Chevron hit target and icon size.
const CHEVRON_SIZE: f32 = 24.0;
/// `ListTokens.ItemLeadingIconSize`.
const ICON_SIZE: f32 = 24.0;
/// Gap between chevron, icon and label (`ListTokens.ItemBetweenSpace`).
const GAP: f32 = 12.0;

impl<'a> MaterialTreeView<'a> {
    /// Create a new tree view
    pub fn new(items: &'a [TreeViewItem], state: &'a mut TreeViewState) -> Self {
        Self {
            items,
            state,
            indent_width: 24.0,
            item_height: 40.0,
            id_salt: None,
        }
    }

    /// Set the indent width for child items
    pub fn indent_width(mut self, width: f32) -> Self {
        self.indent_width = width;
        self
    }

    /// Set the height of each item
    pub fn item_height(mut self, height: f32) -> Self {
        self.item_height = height;
        self
    }

    /// Salt for the row ids, for two trees whose automatic ids would
    /// otherwise collide.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = Some(egui::IdSalt::new(salt));
        self
    }
}

impl<'a> Widget for MaterialTreeView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = MaterialTheme::of(ui.ctx());
        let direction = theme.layout_direction;
        let base_id = match self.id_salt {
            Some(salt) => ui.make_persistent_id(salt),
            None => ui.next_auto_id(),
        };
        let row_id = |item: &TreeViewItem| base_id.with(("tree_item", item.id.as_str()));
        let mut changed = false;

        // Keyboard, before layout, so this frame shows the result.
        let mut rows = Vec::new();
        visible_rows(self.items, self.state, 0, None, &mut rows);
        let focused = ui.memory(|m| m.focused());
        let focused_row = rows.iter().position(|r| focused == Some(row_id(r.item)));
        if let (Some(current), Some(focused_id)) = (focused_row, focused) {
            ui.memory_mut(|m| {
                m.set_focus_lock_filter(
                    focused_id,
                    EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        ..Default::default()
                    },
                );
            });
            let none = egui::Modifiers::NONE;
            let (expand_key, collapse_key) = match direction {
                LayoutDirection::Ltr => (Key::ArrowRight, Key::ArrowLeft),
                LayoutDirection::Rtl => (Key::ArrowLeft, Key::ArrowRight),
            };
            let (down, up, expand, collapse) = ui.input_mut(|i| {
                (
                    i.consume_key(none, Key::ArrowDown),
                    i.consume_key(none, Key::ArrowUp),
                    i.consume_key(none, expand_key),
                    i.consume_key(none, collapse_key),
                )
            });
            let mut focus_to: Option<Id> = None;
            if let Some(row) = rows.get(current) {
                let item = row.item;
                let expanded = self.state.is_expanded(&item.id);
                if down {
                    focus_to = rows.get(current + 1).map(|r| row_id(r.item));
                } else if up && current > 0 {
                    focus_to = rows.get(current - 1).map(|r| row_id(r.item));
                } else if expand && item.has_children() && item.toggleable {
                    if expanded {
                        focus_to = item.children.first().map(row_id);
                    } else {
                        self.state.set_expanded(&item.id, true);
                        changed = true;
                    }
                } else if collapse {
                    if expanded && item.toggleable {
                        self.state.set_expanded(&item.id, false);
                        changed = true;
                    } else {
                        focus_to = row.parent.and_then(|p| rows.get(p)).map(|r| row_id(r.item));
                    }
                }
            }
            if let Some(id) = focus_to {
                ui.memory_mut(|m| m.request_focus(id));
            }
            if changed {
                rows.clear();
                visible_rows(self.items, self.state, 0, None, &mut rows);
            }
        }

        let width = ui.available_width();
        let (rect, mut response) = ui.allocate_exact_size(
            Vec2::new(width, rows.len() as f32 * self.item_height),
            Sense::hover(),
        );
        crate::a11y::describe(&response, egui::Role::Tree, true, "", None);

        let label_style = TypeRole::BodyLarge.style();
        let mut to_toggle: Vec<String> = Vec::new();
        let mut to_select: Vec<String> = Vec::new();
        for (index, row) in rows.iter().enumerate() {
            let item = row.item;
            let row_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.min.y + index as f32 * self.item_height),
                Vec2::new(width, self.item_height),
            );
            let selected = self.state.is_selected(&item.id);
            let expanded = self.state.is_expanded(&item.id);
            let row_response = ui.interact(row_rect, row_id(item), Sense::click());
            crate::a11y::describe(&row_response, egui::Role::TreeItem, true, &item.label, None);
            // A tree item's selection is AccessKit's `selected`, not the
            // `toggled` state egui reports for `WidgetInfo::selected`.
            ui.ctx().accesskit_node_builder(row_response.id, |node| {
                node.set_selected(selected);
                if item.has_children() {
                    node.set_expanded(expanded);
                }
            });

            let indent = row.depth as f32 * self.indent_width;
            let chevron_rect = mirror(
                Rect::from_min_size(
                    Pos2::new(
                        row_rect.min.x + indent,
                        row_rect.center().y - CHEVRON_SIZE / 2.0,
                    ),
                    Vec2::splat(CHEVRON_SIZE),
                ),
                row_rect,
                direction,
            );
            let has_chevron = item.has_children() && item.toggleable;
            let chevron_clicked = has_chevron
                && row_response.clicked()
                && row_response
                    .interact_pointer_pos()
                    .is_some_and(|p| chevron_rect.contains(p));
            if chevron_clicked {
                to_toggle.push(item.id.clone());
            } else if row_response.clicked() {
                if item.selectable {
                    to_select.push(item.id.clone());
                } else if has_chevron {
                    to_toggle.push(item.id.clone());
                }
            }

            // Painting.
            let painter = ui.painter();
            let (content, icon_color) = if selected {
                let c = theme.color(ColorRole::OnSecondaryContainer);
                (c, c)
            } else {
                (
                    theme.color(ColorRole::OnSurface),
                    theme.color(ColorRole::OnSurfaceVariant),
                )
            };
            let content_left = row_rect.min.x + indent;
            let fill_rect = mirror(
                Rect::from_min_max(Pos2::new(content_left, row_rect.min.y), row_rect.max),
                row_rect,
                direction,
            );
            let corner = (self.item_height / 2.0).round().clamp(0.0, 255.0) as u8;
            if selected {
                painter.rect_filled(
                    fill_rect,
                    corner,
                    theme.color(ColorRole::SecondaryContainer),
                );
            }
            let layer = InteractionState::from_response(&row_response).opacity();
            if layer > 0.0 {
                painter.rect_filled(fill_rect, corner, content.gamma_multiply(layer));
            }
            if row_response.has_focus() {
                let ring = fill_rect
                    .shrink(crate::paint::FOCUS_RING_OFFSET + crate::paint::FOCUS_RING_WIDTH);
                crate::paint::paint_focus_ring(
                    painter,
                    ring,
                    corner,
                    theme.color(ColorRole::Secondary),
                );
            }
            if has_chevron {
                let name = match (expanded, direction) {
                    (true, LayoutDirection::Ltr | LayoutDirection::Rtl) => "expand_more",
                    (false, LayoutDirection::Ltr) => "chevron_right",
                    (false, LayoutDirection::Rtl) => "chevron_left",
                };
                painter.text(
                    chevron_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    material_symbol_text(name),
                    egui::FontId::proportional(CHEVRON_SIZE),
                    icon_color,
                );
            }
            let mut x = content_left + CHEVRON_SIZE + GAP / 2.0;
            if let Some(icon) = &item.icon {
                let slot = mirror(
                    Rect::from_min_size(
                        Pos2::new(x, row_rect.center().y - ICON_SIZE / 2.0),
                        Vec2::splat(ICON_SIZE),
                    ),
                    row_rect,
                    direction,
                );
                painter.text(
                    slot.center(),
                    egui::Align2::CENTER_CENTER,
                    material_symbol_text(icon),
                    egui::FontId::proportional(ICON_SIZE),
                    icon_color,
                );
                x += ICON_SIZE + GAP;
            }
            let galley =
                painter.layout_job(label_style.layout_job(ui.ctx(), item.label.clone(), content));
            let size = galley.size();
            let text_rect = mirror(
                Rect::from_min_size(Pos2::new(x, row_rect.center().y - size.y / 2.0), size),
                row_rect,
                direction,
            );
            painter
                .with_clip_rect(row_rect.intersect(painter.clip_rect()))
                .galley(text_rect.min, galley, content);

            response = response.union(row_response);
        }

        for id in to_toggle {
            self.state.toggle_expanded(&id);
            changed = true;
        }
        for id in to_select {
            self.state.toggle_selected(&id);
            changed = true;
        }
        if changed {
            response.mark_changed();
        }
        response
    }
}

/// Mirrors `rect` within `row` for right-to-left layouts.
fn mirror(rect: Rect, row: Rect, direction: LayoutDirection) -> Rect {
    match direction {
        LayoutDirection::Ltr => rect,
        LayoutDirection::Rtl => {
            let min_x = row.min.x + row.max.x - rect.max.x;
            Rect::from_min_size(Pos2::new(min_x, rect.min.y), rect.size())
        }
    }
}

/// Convenience function to create a tree view
pub fn tree_view<'a>(
    items: &'a [TreeViewItem],
    state: &'a mut TreeViewState,
) -> MaterialTreeView<'a> {
    MaterialTreeView::new(items, state)
}
