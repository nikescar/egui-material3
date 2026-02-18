use crate::get_global_color;
use egui::{self, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget};

/// Material Design switch component following Material Design 3 specifications
///
/// Switches toggle the state of a single item on or off. They are the preferred way to
/// adjust settings on mobile and should be used instead of checkboxes in most cases.
///
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut wifi_enabled = false;
/// let mut bluetooth_enabled = true;
///
/// // Basic switch
/// ui.add(MaterialSwitch::new(&mut wifi_enabled));
///
/// // Switch with label text
/// ui.add(MaterialSwitch::new(&mut bluetooth_enabled)
///     .text("Enable Bluetooth"));
///
/// // Disabled switch
/// let mut disabled_option = false;
/// ui.add(MaterialSwitch::new(&mut disabled_option)
///     .text("Unavailable option")
///     .enabled(false));
/// # });
/// ```
///
/// ## Material Design Spec (Material 3)
/// - Track width: 52dp, height: 32dp
/// - Thumb diameter: 24dp when on, 16dp when off, 28dp when pressed
/// - Corner radius: 16dp (fully rounded)
/// - Touch target: 48x48dp minimum
/// - Colors: Primary when on, surfaceContainerHighest when off
/// - Track outline: 2dp when off, transparent when on
/// - Icons: 16dp, displayed on thumb
/// - Animation: 300ms cubic-bezier transition
pub struct MaterialSwitch<'a> {
    /// Mutable reference to the switch state (on/off)
    selected: &'a mut bool,
    /// Optional text label displayed next to the switch
    text: Option<String>,
    /// Whether the switch is interactive (enabled/disabled)
    enabled: bool,
    /// Optional icon displayed on thumb when selected
    selected_icon: Option<char>,
    /// Optional icon displayed on thumb when unselected
    unselected_icon: Option<char>,
    /// Whether to show track outline (Material 3: true, Material 2: false)
    show_track_outline: bool,
}

impl<'a> MaterialSwitch<'a> {
    /// Create a new Material Design switch
    ///
    /// ## Parameters
    /// - `selected`: Mutable reference to boolean state representing on/off
    ///
    /// ## Returns
    /// A new MaterialSwitch instance with default settings
    pub fn new(selected: &'a mut bool) -> Self {
        Self {
            selected,
            text: None,
            enabled: true,
            selected_icon: None,
            unselected_icon: None,
            show_track_outline: true, // Material 3 default
        }
    }

    /// Set optional text label for the switch
    ///
    /// The text will be displayed to the right of the switch component.
    ///
    /// ## Parameters
    /// - `text`: Label text to display next to the switch
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set whether the switch is enabled or disabled
    ///
    /// Disabled switches cannot be interacted with and are visually dimmed.
    ///
    /// ## Parameters
    /// - `enabled`: True for interactive, false for disabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set an icon to display on the thumb when the switch is selected (on)
    ///
    /// ## Parameters
    /// - `icon`: Character representing a Material icon to display on selected thumb
    pub fn selected_icon(mut self, icon: char) -> Self {
        self.selected_icon = Some(icon);
        self
    }

    /// Set an icon to display on the thumb when the switch is unselected (off)
    ///
    /// ## Parameters
    /// - `icon`: Character representing a Material icon to display on unselected thumb
    pub fn unselected_icon(mut self, icon: char) -> Self {
        self.unselected_icon = Some(icon);
        self
    }

    /// Set icons for both selected and unselected states
    ///
    /// ## Parameters
    /// - `selected`: Character representing a Material icon to display when on
    /// - `unselected`: Character representing a Material icon to display when off
    pub fn with_icons(mut self, selected: char, unselected: char) -> Self {
        self.selected_icon = Some(selected);
        self.unselected_icon = Some(unselected);
        self
    }

    /// Set whether to show track outline (Material 3 style)
    ///
    /// ## Parameters
    /// - `show`: True to show outline when off, false for no outline
    pub fn show_track_outline(mut self, show: bool) -> Self {
        self.show_track_outline = show;
        self
    }
}

impl<'a> Widget for MaterialSwitch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Material 3 specifications
        let switch_width = 52.0;
        let switch_height = 32.0;
        let track_height = 32.0;

        let desired_size = if let Some(ref text) = self.text {
            let text_width = ui.fonts(|fonts| {
                fonts.glyph_width(&egui::FontId::default(), ' ') * text.len() as f32
            });
            Vec2::new(switch_width + 8.0 + text_width, switch_height)
        } else {
            Vec2::new(switch_width, switch_height)
        };

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() && self.enabled {
            *self.selected = !*self.selected;
            response.mark_changed();
        }

        // Track interaction states
        let is_pressed = response.is_pointer_button_down_on();
        let is_hovered = response.hovered();
        let is_focused = response.has_focus();

        // Material Design 3 colors
        let primary_color = get_global_color("primary");
        let on_primary = get_global_color("onPrimary");
        let primary_container = get_global_color("primaryContainer");
        let on_primary_container = get_global_color("onPrimaryContainer");
        let surface_container_highest = get_global_color("surfaceContainerHighest");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        // Calculate switch area
        let switch_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - switch_height / 2.0),
            Vec2::new(switch_width, switch_height),
        );

        let track_rect =
            Rect::from_center_size(switch_rect.center(), Vec2::new(switch_width, track_height));

        // Material 3: thumb is 16dp when off, 24dp when on, 28dp when pressed
        let has_icon = if *self.selected {
            self.selected_icon.is_some()
        } else {
            self.unselected_icon.is_some()
        };
        
        let base_thumb_size_on = 24.0;
        let base_thumb_size_off = if has_icon { 24.0 } else { 16.0 };
        let pressed_thumb_size = 28.0;
        
        let thumb_size = if is_pressed {
            pressed_thumb_size
        } else if *self.selected {
            base_thumb_size_on
        } else {
            base_thumb_size_off
        };
        
        let thumb_travel = switch_width - base_thumb_size_on - 4.0;
        let thumb_x = if *self.selected {
            switch_rect.min.x + 2.0 + thumb_travel
        } else {
            switch_rect.min.x + 2.0
        };

        let thumb_center = Pos2::new(thumb_x + thumb_size / 2.0, switch_rect.center().y);

        // Material 3 color resolution based on state
        let (track_color, thumb_color, track_outline_color, icon_color) = if !self.enabled {
            // Disabled state
            let disabled_track = if *self.selected {
                on_surface.linear_multiply(0.12)
            } else {
                surface_container_highest.linear_multiply(0.12)
            };
            let disabled_thumb = if *self.selected {
                on_surface.linear_multiply(1.0)
            } else {
                on_surface.linear_multiply(0.38)
            };
            let disabled_outline = on_surface.linear_multiply(0.12);
            let disabled_icon = if *self.selected {
                on_surface.linear_multiply(0.38)
            } else {
                surface_container_highest.linear_multiply(0.38)
            };
            (disabled_track, disabled_thumb, disabled_outline, disabled_icon)
        } else if *self.selected {
            // Selected (on) state
            let track = primary_color;
            let thumb = if is_pressed || is_hovered || is_focused {
                primary_container
            } else {
                on_primary
            };
            let outline = Color32::TRANSPARENT;
            let icon = if is_pressed || is_hovered || is_focused {
                on_primary_container
            } else {
                on_primary_container
            };
            (track, thumb, outline, icon)
        } else {
            // Unselected (off) state
            let track = if is_pressed || is_hovered || is_focused {
                surface_container_highest
            } else {
                surface_container_highest
            };
            let thumb = if is_pressed || is_hovered || is_focused {
                on_surface_variant
            } else {
                outline
            };
            let track_outline = outline;
            let icon = surface_container_highest;
            (track, thumb, track_outline, icon)
        };

        // Draw track
        ui.painter()
            .rect_filled(track_rect, track_height / 2.0, track_color);

        // Draw track outline (Material 3)
        if self.show_track_outline && track_outline_color != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                track_rect,
                track_height / 2.0,
                Stroke::new(2.0, track_outline_color),
                StrokeKind::Outside,
            );
        }

        // Draw state layer (ripple/overlay effect) - Material 3
        if self.enabled {
            let overlay_radius = 20.0; // 40dp diameter / 2
            let overlay_color = if *self.selected {
                if is_pressed {
                    primary_color.linear_multiply(0.1)
                } else if is_hovered {
                    primary_color.linear_multiply(0.08)
                } else if is_focused {
                    primary_color.linear_multiply(0.1)
                } else {
                    Color32::TRANSPARENT
                }
            } else {
                if is_pressed {
                    on_surface.linear_multiply(0.1)
                } else if is_hovered {
                    on_surface.linear_multiply(0.08)
                } else if is_focused {
                    on_surface.linear_multiply(0.1)
                } else {
                    Color32::TRANSPARENT
                }
            };

            if overlay_color != Color32::TRANSPARENT {
                ui.painter()
                    .circle_filled(thumb_center, overlay_radius, overlay_color);
            }
        }

        // Draw thumb
        ui.painter()
            .circle_filled(thumb_center, thumb_size / 2.0, thumb_color);

        // Draw icon on thumb if present
        let current_icon = if *self.selected {
            self.selected_icon
        } else {
            self.unselected_icon
        };

        if let Some(icon) = current_icon {
            let icon_size = 16.0;
            let icon_font = FontId::proportional(icon_size);
            
            ui.painter().text(
                thumb_center,
                egui::Align2::CENTER_CENTER,
                icon.to_string(),
                icon_font,
                icon_color,
            );
        }

        // Draw label text
        if let Some(ref text) = self.text {
            let text_pos = Pos2::new(switch_rect.max.x + 8.0, rect.center().y);

            let text_color = if self.enabled {
                on_surface
            } else {
                on_surface.linear_multiply(0.38)
            };

            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::default(),
                text_color,
            );
        }

        response
    }
}

pub fn switch(selected: &mut bool) -> MaterialSwitch<'_> {
    MaterialSwitch::new(selected)
}
