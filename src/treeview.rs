use crate::theme::get_global_color;
use crate::material_symbol::material_symbol_text;
use egui::{
    ecolor::Color32, pos2, FontId, Rect, Response, Sense, Ui, Vec2, Widget, Id,
};
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
}

impl<'a> MaterialTreeView<'a> {
    /// Create a new tree view
    pub fn new(items: &'a [TreeViewItem], state: &'a mut TreeViewState) -> Self {
        Self {
            items,
            state,
            indent_width: 24.0,
            item_height: 40.0,
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

    /// Render a single tree item and its children
    fn render_item(
        &mut self,
        ui: &mut Ui,
        item: &TreeViewItem,
        depth: usize,
    ) -> Response {
        let indent = depth as f32 * self.indent_width;
        let is_expanded = self.state.is_expanded(&item.id);
        let is_selected = self.state.is_selected(&item.id);

        // Get Material Design colors
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let surface_variant = get_global_color("surfaceVariant");
        let primary = get_global_color("primary");

        // Calculate item width
        let available_width = ui.available_width();

        ui.horizontal(|ui| {
            ui.add_space(indent);

            // Toggle button (chevron icon) if item has children
            if item.has_children() && item.toggleable {
                let chevron_icon = if is_expanded {
                    material_symbol_text("expand_more")
                } else {
                    material_symbol_text("chevron_right")
                };

                let chevron_button = egui::Button::new(chevron_icon)
                    .frame(false)
                    .min_size(Vec2::new(24.0, 24.0));

                if ui.add(chevron_button).clicked() {
                    self.state.toggle_expanded(&item.id);
                }
            } else {
                // Empty space for alignment
                ui.add_space(24.0);
            }

            // Icon if present
            if let Some(icon_name) = &item.icon {
                let icon_text = material_symbol_text(icon_name);
                ui.label(egui::RichText::new(icon_text).size(20.0).color(on_surface_variant));
                ui.add_space(8.0);
            }

            // Label
            let label_color = if is_selected { primary } else { on_surface };
            let label_response = ui.selectable_label(is_selected,
                egui::RichText::new(&item.label).color(label_color));

            if label_response.clicked() && item.selectable {
                self.state.toggle_selected(&item.id);
            }
        });

        // Render children if expanded
        let mut child_response = ui.allocate_response(Vec2::ZERO, Sense::hover());
        if is_expanded && item.has_children() {
            for child in &item.children {
                let response = self.render_item(ui, child, depth + 1);
                child_response = child_response.union(response);
            }
        }

        child_response
    }
}

impl<'a> Widget for MaterialTreeView<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let mut response = ui.allocate_response(Vec2::ZERO, Sense::hover());

        for item in self.items {
            let item_response = self.render_item(ui, item, 0);
            response = response.union(item_response);
        }

        response
    }
}

/// Convenience function to create a tree view
pub fn tree_view<'a>(items: &'a [TreeViewItem], state: &'a mut TreeViewState) -> MaterialTreeView<'a> {
    MaterialTreeView::new(items, state)
}
