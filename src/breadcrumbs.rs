//! Material Design 3 Breadcrumbs Components
//!
//! # M3 Color Role Usage
//!
//! - **onSurface**: Current/active breadcrumb text
//! - **onSurfaceVariant**: Inactive breadcrumb text, separators
//! - **primary**: Breadcrumb links (hover)
//! - **State layers**: onSurface @ 8% (hover), 12% (press)
//!
//! The current (active) breadcrumb is plain `LabelLarge` text in
//! `onSurface`: it is the page you are on, so it is not a button, but it
//! is not disabled either (it reads at full contrast and is reported
//! enabled to AccessKit, as a label).

use crate::material_theme::MaterialTheme;
use crate::tokens::{ColorRole, TypeRole};
use crate::MaterialButton;
use egui::{Response, RichText, Sense, Ui, Widget};

/// Material Design breadcrumbs component.
///
/// Breadcrumbs allow users to keep track and maintain awareness of their locations
/// within the app or website. They function best for large sites and apps with
/// hierarchically arranged pages.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::breadcrumbs;
///
/// ui.add(breadcrumbs()
///     .item("Home")
///     .item("Products")
///     .item("Electronics")
///     .active_item("Laptops"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialBreadcrumbs {
    /// List of breadcrumb items (text, is_active, on_click)
    items: Vec<BreadcrumbItem>,
    /// Custom separator between items (default: "/")
    separator: String,
    /// Font size for breadcrumb text
    font_size: f32,
    /// Spacing between items and separators
    spacing: f32,
    /// Whether to show the separator
    show_separator: bool,
}

/// Represents a single breadcrumb item
struct BreadcrumbItem {
    text: String,
    is_active: bool,
    clickable: bool,
    callback: Option<Box<dyn FnMut()>>,
}

impl Default for MaterialBreadcrumbs {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            separator: "/".to_string(),
            font_size: 14.0,
            spacing: 8.0,
            show_separator: true,
        }
    }
}

impl MaterialBreadcrumbs {
    /// Create a new breadcrumbs component
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a clickable breadcrumb item
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::breadcrumbs;
    ///
    /// ui.add(breadcrumbs()
    ///     .item("Home")
    ///     .item("Products"));
    /// # });
    /// ```
    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem {
            text: text.into(),
            is_active: false,
            clickable: true,
            callback: None,
        });
        self
    }

    /// Add a clickable breadcrumb item with a callback
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::breadcrumbs;
    ///
    /// ui.add(breadcrumbs()
    ///     .item_with_callback("Home", || println!("Home clicked")));
    /// # });
    /// ```
    pub fn item_with_callback<F>(mut self, text: impl Into<String>, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.items.push(BreadcrumbItem {
            text: text.into(),
            is_active: false,
            clickable: true,
            callback: Some(Box::new(callback)),
        });
        self
    }

    /// Add the active (current) breadcrumb item
    ///
    /// This item is typically the last one and represents the current page.
    /// It is not clickable and has distinct styling.
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::breadcrumbs;
    ///
    /// ui.add(breadcrumbs()
    ///     .item("Home")
    ///     .active_item("Current Page"));
    /// # });
    /// ```
    pub fn active_item(mut self, text: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem {
            text: text.into(),
            is_active: true,
            clickable: false,
            callback: None,
        });
        self
    }

    /// Set a custom separator between breadcrumb items
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::breadcrumbs;
    ///
    /// ui.add(breadcrumbs()
    ///     .separator(">")
    ///     .item("Home")
    ///     .item("Products"));
    /// # });
    /// ```
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Set the font size for breadcrumb text
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the spacing between items and separators
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Hide the separator between items
    pub fn hide_separator(mut self) -> Self {
        self.show_separator = false;
        self
    }

    /// Add multiple items at once
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// use egui_material3::breadcrumbs;
    ///
    /// ui.add(breadcrumbs()
    ///     .items(vec!["Home", "Products", "Electronics"])
    ///     .active_item("Laptops"));
    /// # });
    /// ```
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for text in items {
            self.items.push(BreadcrumbItem {
                text: text.into(),
                is_active: false,
                clickable: true,
                callback: None,
            });
        }
        self
    }
}

impl Widget for MaterialBreadcrumbs {
    fn ui(mut self, ui: &mut Ui) -> Response {
        // Material Design colors
        let theme = MaterialTheme::of(ui.ctx());
        let on_surface_variant = theme.color(ColorRole::OnSurfaceVariant);
        let on_surface = theme.color(ColorRole::OnSurface);

        // Use horizontal layout
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = self.spacing;

            let items_count = self.items.len();

            for (index, item) in self.items.iter_mut().enumerate() {
                let is_last = index == items_count - 1;

                // Render the breadcrumb item as a small button
                if item.clickable && !item.is_active {
                    // Clickable breadcrumb item - use text button
                    let button = MaterialButton::text(&item.text).small();

                    let response = ui.add(button);

                    // Handle click
                    if response.clicked() {
                        if let Some(callback) = &mut item.callback {
                            callback();
                        }
                    }
                } else {
                    // The current page: full-contrast text, not a disabled
                    // button.
                    let galley = ui
                        .painter()
                        .layout_job(TypeRole::LabelLarge.style().layout_job(
                            ui.ctx(),
                            item.text.clone(),
                            on_surface,
                        ));
                    // Same height as the small text buttons beside it.
                    let height = ui.spacing().interact_size.y.max(galley.size().y).max(32.0);
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(galley.size().x + 2.0 * 12.0, height),
                        Sense::hover(),
                    );
                    let at = rect.center() - galley.size() / 2.0;
                    ui.painter().galley(at, galley, on_surface);
                    crate::a11y::describe(&response, egui::Role::Label, true, &item.text, None);
                }

                // Add separator if not the last item and separator is enabled
                if !is_last && self.show_separator {
                    let separator_text = RichText::new(&self.separator)
                        .color(on_surface_variant)
                        .size(self.font_size);
                    ui.label(separator_text);
                }
            }
        })
        .response
    }
}

/// Convenience function to create a breadcrumbs component
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::breadcrumbs;
///
/// ui.add(breadcrumbs()
///     .item("Home")
///     .item("Products")
///     .active_item("Electronics"));
/// # });
/// ```
pub fn breadcrumbs() -> MaterialBreadcrumbs {
    MaterialBreadcrumbs::new()
}
