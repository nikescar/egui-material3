use crate::theme::get_global_color;
use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Material Design select/dropdown component.
///
/// Select components allow users to choose one option from a list.
/// They display the currently selected option in a text field-style input
/// and show all options in a dropdown menu when activated.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selected = Some(1);
/// 
/// ui.add(MaterialSelect::new(&mut selected)
///     .placeholder("Choose an option")
///     .option(0, "Option 1")
///     .option(1, "Option 2")
///     .option(2, "Option 3")
///     .helper_text("Select your preferred option"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialSelect<'a> {
    /// Reference to the currently selected option
    selected: &'a mut Option<usize>,
    /// List of available options
    options: Vec<SelectOption>,
    /// Placeholder text when no option is selected
    placeholder: String,
    /// Whether the select is enabled for interaction
    enabled: bool,
    /// Fixed width of the select component
    width: Option<f32>,
    /// Error message to display below the select
    error_text: Option<String>,
    /// Helper text to display below the select
    helper_text: Option<String>,
    /// Icon to show at the start of the select field
    leading_icon: Option<String>,
    /// Icon to show at the end of the select field (overrides default dropdown arrow)
    trailing_icon: Option<String>,
    /// Whether to keep the dropdown open after selecting an option
    keep_open_on_select: bool,
}

/// Individual option in a select component.
pub struct SelectOption {
    /// Unique identifier for this option
    value: usize,
    /// Display text for this option
    text: String,
}

impl<'a> MaterialSelect<'a> {
    /// Create a new select component.
    ///
    /// # Arguments
    /// * `selected` - Mutable reference to the currently selected option value
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// let select = MaterialSelect::new(&mut selection);
    /// # });
    /// ```
    pub fn new(selected: &'a mut Option<usize>) -> Self {
        Self {
            selected,
            options: Vec::new(),
            placeholder: "Select an option".to_string(),
            enabled: true,
            width: None,
            error_text: None,
            helper_text: None,
            leading_icon: None,
            trailing_icon: None,
            keep_open_on_select: false,
        }
    }

    /// Add an option to the select component.
    ///
    /// # Arguments
    /// * `value` - Unique identifier for this option
    /// * `text` - Display text for this option
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .option(1, "First Option")
    ///     .option(2, "Second Option"));
    /// # });
    /// ```
    pub fn option(mut self, value: usize, text: impl Into<String>) -> Self {
        self.options.push(SelectOption {
            value,
            text: text.into(),
        });
        self
    }

    /// Set placeholder text shown when no option is selected.
    ///
    /// # Arguments
    /// * `placeholder` - The placeholder text to display
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .placeholder("Choose your option"));
    /// # });
    /// ```
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Enable or disable the select component.
    ///
    /// # Arguments
    /// * `enabled` - Whether the select should be enabled (true) or disabled (false)
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .enabled(false)); // Disabled select
    /// # });
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set a fixed width for the select component.
    ///
    /// # Arguments
    /// * `width` - The width in pixels
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .width(300.0)); // Fixed width of 300 pixels
    /// # });
    /// ```
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set error text to display below the select component.
    ///
    /// # Arguments
    /// * `text` - The error message text
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .error_text("This field is required")); // Error message
    /// # });
    /// ```
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    /// Set helper text to display below the select component.
    ///
    /// # Arguments
    /// * `text` - The helper message text
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .helper_text("Select an option from the list")); // Helper text
    /// # });
    /// ```
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Set an icon to display at the start of the select field.
    ///
    /// # Arguments
    /// * `icon` - The icon identifier (e.g., "home", "settings")
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .leading_icon("settings")); // Gear icon on the left
    /// # });
    /// ```
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set an icon to display at the end of the select field (overrides default dropdown arrow).
    ///
    /// # Arguments
    /// * `icon` - The icon identifier (e.g., "check", "close")
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .trailing_icon("check")); // Check icon on the right
    /// # });
    /// ```
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set whether to keep the dropdown open after selecting an option.
    ///
    /// # Arguments
    /// * `keep_open` - If true, the dropdown remains open after selection;
    ///                 if false, it closes (default behavior)
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut selection = None;
    /// ui.add(MaterialSelect::new(&mut selection)
    ///     .keep_open_on_select(true)); // Dropdown stays open after selection
    /// # });
    /// ```
    pub fn keep_open_on_select(mut self, keep_open: bool) -> Self {
        self.keep_open_on_select = keep_open;
        self
    }
}

impl<'a> Widget for MaterialSelect<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let width = self.width.unwrap_or(200.0);
        let height = 56.0;
        let desired_size = Vec2::new(width, height);

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        // Use persistent state for dropdown open/close with global coordination
        let select_id = egui::Id::new(("select_widget", rect.min.x as i32, rect.min.y as i32, self.placeholder.clone()));
        let mut open = ui.memory(|mem| mem.data.get_temp::<bool>(select_id).unwrap_or(false));

        // Global state to close other select menus
        let global_open_select_id = egui::Id::new("global_open_select");
        let current_open_select = ui.memory(|mem| mem.data.get_temp::<egui::Id>(global_open_select_id));

        if response.clicked() && self.enabled {
            if open {
                // Close this select
                open = false;
                ui.memory_mut(|mem| mem.data.remove::<egui::Id>(global_open_select_id));
            } else {
                // Close any other open select and open this one
                if let Some(other_id) = current_open_select {
                    if other_id != select_id {
                        ui.memory_mut(|mem| mem.data.insert_temp(other_id, false));
                    }
                }
                open = true;
                ui.memory_mut(|mem| mem.data.insert_temp(global_open_select_id, select_id));
            }
            ui.memory_mut(|mem| mem.data.insert_temp(select_id, open));
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
        let surface = get_global_color("surface");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let (bg_color, border_color, text_color) = if !self.enabled {
            (
                get_global_color("surfaceVariant").linear_multiply(0.38),
                get_global_color("outline").linear_multiply(0.38),
                get_global_color("onSurface").linear_multiply(0.38),
            )
        } else if response.hovered() || open {
            (surface, primary_color, on_surface)
        } else {
            (surface, outline, on_surface_variant)
        };

        // Draw select field background
        ui.painter().rect_filled(
            rect,
            4.0,
            bg_color,
        );

        // Draw border
        ui.painter().rect_stroke(
            rect,
            4.0,
            Stroke::new(1.0, border_color),
            egui::epaint::StrokeKind::Outside,
        );

        // Draw selected text or placeholder
        let display_text = if let Some(selected_value) = *self.selected {
            self.options.iter()
                .find(|option| option.value == selected_value)
                .map(|option| option.text.as_str())
                .unwrap_or(&self.placeholder)
        } else {
            &self.placeholder
        };

        // Use consistent font styling for select field
        let select_font = FontId::new(16.0, FontFamily::Proportional);
        let text_pos = Pos2::new(rect.min.x + 16.0, rect.center().y);
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            display_text,
            select_font.clone(),
            text_color,
        );

        // Draw dropdown arrow
        let arrow_center = Pos2::new(rect.max.x - 24.0, rect.center().y);
        let arrow_size = 8.0;
        
        if open {
            // Up arrow
            ui.painter().line_segment([
                Pos2::new(arrow_center.x - arrow_size / 2.0, arrow_center.y + arrow_size / 4.0),
                Pos2::new(arrow_center.x, arrow_center.y - arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
            ui.painter().line_segment([
                Pos2::new(arrow_center.x, arrow_center.y - arrow_size / 4.0),
                Pos2::new(arrow_center.x + arrow_size / 2.0, arrow_center.y + arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
        } else {
            // Down arrow
            ui.painter().line_segment([
                Pos2::new(arrow_center.x - arrow_size / 2.0, arrow_center.y - arrow_size / 4.0),
                Pos2::new(arrow_center.x, arrow_center.y + arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
            ui.painter().line_segment([
                Pos2::new(arrow_center.x, arrow_center.y + arrow_size / 4.0),
                Pos2::new(arrow_center.x + arrow_size / 2.0, arrow_center.y - arrow_size / 4.0),
            ], Stroke::new(2.0, text_color));
        }

        // Show dropdown if open
        if open {
            // Calculate available space below and above
            let available_space_below = ui.max_rect().max.y - rect.max.y - 4.0;
            let available_space_above = rect.min.y - ui.max_rect().min.y - 4.0;
            
            let item_height = 48.0;
            let dropdown_padding = 16.0;
            let max_items_below = ((available_space_below - dropdown_padding) / item_height).floor() as usize;
            let max_items_above = ((available_space_above - dropdown_padding) / item_height).floor() as usize;
            
            // Determine dropdown position and size
            let (dropdown_y, visible_items, scroll_needed) = if max_items_below >= self.options.len() {
                // Fit below
                (rect.max.y + 4.0, self.options.len(), false)
            } else if max_items_above >= self.options.len() {
                // Fit above
                let dropdown_height = self.options.len() as f32 * item_height + dropdown_padding;
                (rect.min.y - 4.0 - dropdown_height, self.options.len(), false)
            } else if max_items_below >= max_items_above {
                // Partial fit below with scroll
                (rect.max.y + 4.0, max_items_below.max(3), true)
            } else {
                // Partial fit above with scroll
                let visible_items = max_items_above.max(3);
                let dropdown_height = visible_items as f32 * item_height + dropdown_padding;
                (rect.min.y - 4.0 - dropdown_height, visible_items, true)
            };

            let dropdown_height = visible_items as f32 * item_height + dropdown_padding;
            let dropdown_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, dropdown_y),
                Vec2::new(width, dropdown_height),
            );

            // Use page background color as specified
            let dropdown_bg_color = ui.visuals().window_fill;

            // Draw dropdown background with proper elevation
            ui.painter().rect_filled(
                dropdown_rect,
                8.0,
                dropdown_bg_color,
            );

            // Draw dropdown border with elevation shadow
            ui.painter().rect_stroke(
                dropdown_rect,
                8.0,
                Stroke::new(1.0, outline),
                egui::epaint::StrokeKind::Outside,
            );

            // Draw subtle elevation shadow
            let shadow_color = Color32::from_rgba_premultiplied(0, 0, 0, 20);
            ui.painter().rect_filled(
                dropdown_rect.translate(Vec2::new(0.0, 2.0)),
                8.0,
                shadow_color,
            );

            // Render options with scrolling support and edge attachment
            if scroll_needed && visible_items < self.options.len() {
                // Use scroll area for overflow with edge attachment
                let scroll_area_rect = Rect::from_min_size(
                    Pos2::new(dropdown_rect.min.x + 8.0, dropdown_rect.min.y + 8.0),
                    Vec2::new(width - 16.0, dropdown_height - 16.0),
                );
                
                ui.scope_builder(egui::UiBuilder::new().max_rect(scroll_area_rect), |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(dropdown_height - 16.0)
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for option in &self.options {
                                // Create custom option layout with proper text styling
                                let option_height = 48.0;
                                let (option_rect, option_response) = ui.allocate_exact_size(
                                    Vec2::new(ui.available_width(), option_height), 
                                    Sense::click()
                                );

                                // Match select field styling
                                let is_selected = *self.selected == Some(option.value);
                                let option_bg_color = if is_selected {
                                    Color32::from_rgba_premultiplied(
                                        on_surface.r(), on_surface.g(), on_surface.b(), 30
                                    )
                                } else if option_response.hovered() {
                                    Color32::from_rgba_premultiplied(
                                        on_surface.r(), on_surface.g(), on_surface.b(), 20
                                    )
                                } else {
                                    Color32::TRANSPARENT
                                };

                                if option_bg_color != Color32::TRANSPARENT {
                                    ui.painter().rect_filled(option_rect, 4.0, option_bg_color);
                                }

                                // Use same font as select field with text wrapping
                                let text_pos = Pos2::new(option_rect.min.x + 16.0, option_rect.center().y);
                                let text_color = if is_selected { 
                                    get_global_color("primary") 
                                } else { 
                                    on_surface 
                                };
                                
                                // Handle text wrapping for long content
                                let available_width = option_rect.width() - 32.0; // Account for padding
                                let galley = ui.fonts(|f| f.layout_job(egui::text::LayoutJob {
                                    text: option.text.clone(),
                                    sections: vec![egui::text::LayoutSection {
                                        leading_space: 0.0,
                                        byte_range: 0..option.text.len(),
                                        format: egui::TextFormat {
                                            font_id: select_font.clone(),
                                            color: text_color,
                                            ..Default::default()
                                        },
                                    }],
                                    wrap: egui::text::TextWrapping {
                                        max_width: available_width,
                                        ..Default::default()
                                    },
                                    break_on_newline: true,
                                    halign: egui::Align::LEFT,
                                    justify: false,
                                    first_row_min_height: 0.0,
                                    round_output_to_gui: true,
                                }));
                                
                                ui.painter().galley(text_pos, galley, text_color);

                                if option_response.clicked() {
                                    *self.selected = Some(option.value);
                                    if !self.keep_open_on_select {
                                        open = false;
                                        ui.memory_mut(|mem| {
                                            mem.data.insert_temp(select_id, open);
                                            mem.data.remove::<egui::Id>(global_open_select_id);
                                        });
                                    }
                                    response.mark_changed();
                                }
                            }
                        });
                });
            } else {
                // Draw options normally without scrolling
                let mut current_y = dropdown_rect.min.y + 8.0;
                let items_to_show = visible_items.min(self.options.len());
                
                for option in self.options.iter().take(items_to_show) {
                    let option_rect = Rect::from_min_size(
                        Pos2::new(dropdown_rect.min.x + 8.0, current_y),
                        Vec2::new(width - 16.0, item_height),
                    );

                    let option_response = ui.interact(
                        option_rect,
                        egui::Id::new(("select_option", option.value, option.text.clone())),
                        Sense::click(),
                    );

                    // Highlight selected option
                    let is_selected = *self.selected == Some(option.value);
                    let option_bg_color = if is_selected {
                        Color32::from_rgba_premultiplied(
                            on_surface.r(), on_surface.g(), on_surface.b(), 30
                        )
                    } else if option_response.hovered() {
                        Color32::from_rgba_premultiplied(
                            on_surface.r(), on_surface.g(), on_surface.b(), 20
                        )
                    } else {
                        Color32::TRANSPARENT
                    };

                    if option_bg_color != Color32::TRANSPARENT {
                        ui.painter().rect_filled(option_rect, 4.0, option_bg_color);
                    }

                    if option_response.clicked() {
                        *self.selected = Some(option.value);
                        if !self.keep_open_on_select {
                            open = false;
                            ui.memory_mut(|mem| {
                                mem.data.insert_temp(select_id, open);
                                mem.data.remove::<egui::Id>(global_open_select_id);
                            });
                        }
                        response.mark_changed();
                    }

                    let text_pos = Pos2::new(option_rect.min.x + 16.0, option_rect.center().y);
                    let text_color = if is_selected { 
                        get_global_color("primary") 
                    } else { 
                        on_surface 
                    };
                    
                    // Handle text wrapping for long content
                    let available_width = option_rect.width() - 32.0; // Account for padding
                    let galley = ui.fonts(|f| f.layout_job(egui::text::LayoutJob {
                        text: option.text.clone(),
                        sections: vec![egui::text::LayoutSection {
                            leading_space: 0.0,
                            byte_range: 0..option.text.len(),
                            format: egui::TextFormat {
                                font_id: select_font.clone(),
                                color: text_color,
                                ..Default::default()
                            },
                        }],
                        wrap: egui::text::TextWrapping {
                            max_width: available_width,
                            ..Default::default()
                        },
                        break_on_newline: true,
                        halign: egui::Align::LEFT,
                        justify: false,
                        first_row_min_height: 0.0,
                        round_output_to_gui: true,
                    }));
                    
                    ui.painter().galley(text_pos, galley, text_color);

                    current_y += item_height;
                }
            }
        }

        response
    }
}

/// Convenience function to create a select component.
///
/// Shorthand for `MaterialSelect::new()`.
///
/// # Arguments
/// * `selected` - Mutable reference to the currently selected option value
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut selection = Some(1);
/// ui.add(select(&mut selection)
///     .option(0, "Option 1")
///     .option(1, "Option 2"));
/// # });
/// ```
pub fn select<'a>(selected: &'a mut Option<usize>) -> MaterialSelect<'a> {
    MaterialSelect::new(selected)
}