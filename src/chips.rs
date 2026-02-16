use crate::{get_global_color, image_utils};
use eframe::egui::{
    self, Color32, Pos2, Rect, Response, Sense, Stroke, TextureHandle, Ui, Vec2, Widget,
};

/// Material Design chip variants following Material Design 3 specifications
#[derive(Clone, Copy, PartialEq)]
pub enum ChipVariant {
    /// Assist chips help users take actions or get information about their current context
    Assist,
    /// Filter chips let users refine content by selecting or deselecting options
    Filter,
    /// Input chips represent discrete pieces of information entered by a user
    Input,
    /// Suggestion chips help users discover relevant, actionable content
    Suggestion,
}

/// Types of icons that can be displayed in chips
#[derive(Clone)]
pub enum IconType {
    /// Material Design icon using icon name or unicode
    MaterialIcon(String),
    /// Custom SVG icon data
    SvgData(String),
    /// PNG image data as bytes
    PngBytes(Vec<u8>),
    /// Pre-loaded egui texture handle
    Texture(TextureHandle),
}

/// Material Design chip component following Material Design 3 specifications
///
/// Chips are compact elements that represent an input, attribute, or action.
/// They allow users to enter information, make selections, filter content, or trigger actions.
///
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// // Assist chip - helps users with contextual actions
/// if ui.add(MaterialChip::assist("Settings")).clicked() {
///     // Open settings
/// }
///
/// // Filter chip - for filtering content
/// let mut filter_active = false;
/// ui.add(MaterialChip::filter("Photos")
///     .selected(&mut filter_active));
///
/// // Input chip - represents entered data
/// ui.add(MaterialChip::input("john@example.com")
///     .removable(true));
///
/// // Suggestion chip - suggests actions or content
/// ui.add(MaterialChip::suggestion("Try this feature"));
/// # });
/// ```
///
/// ## Material Design Spec
/// - Height: 32dp
/// - Corner radius: 8dp
/// - Text: Label Large (14sp/500 weight)
/// - Touch target: Minimum 48x48dp
pub struct MaterialChip<'a> {
    /// Text content displayed on the chip
    text: String,
    /// Which type of chip this is (affects styling and behavior)
    variant: ChipVariant,
    /// Optional mutable reference to selection state (for filter chips)
    selected: Option<&'a mut bool>,
    /// Whether the chip is interactive
    enabled: bool,
    /// Whether the chip is soft-disabled (different visual treatment)
    soft_disabled: bool,
    /// Whether the chip has elevation shadow
    elevated: bool,
    /// Whether the chip can be removed (shows X icon)
    removable: bool,
    /// Optional leading icon to display
    leading_icon: Option<IconType>,
    /// Whether to use avatar-style rounded appearance
    avatar: bool,
    /// Optional action callback when chip is clicked
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialChip<'a> {
    /// Create a new chip with specified text and variant
    ///
    /// ## Parameters
    /// - `text`: Text to display on the chip
    /// - `variant`: Type of chip (Assist, Filter, Input, Suggestion)
    pub fn new(text: impl Into<String>, variant: ChipVariant) -> Self {
        Self {
            text: text.into(),
            variant,
            selected: None,
            enabled: true,
            soft_disabled: false,
            elevated: false,
            removable: false,
            leading_icon: None,
            avatar: false, // regular chips are more rectangular by default
            action: None,
        }
    }

    /// Create an assist chip for contextual actions
    ///
    /// Assist chips help users take actions or get information about their current context.
    /// They should appear dynamically and contextually in the UI.
    ///
    /// ## Material Design Usage
    /// - Display contextually relevant actions
    /// - Usually triggered by user actions or context changes  
    /// - Should not be persistent in the interface
    pub fn assist(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Assist)
    }

    /// Create a filter chip for content filtering
    ///
    /// Filter chips are used for filtering content and are typically displayed in a set.
    /// They can be selected/deselected to refine displayed content.
    ///
    /// ## Parameters
    /// - `text`: Label for the filter option
    /// - `selected`: Mutable reference to selection state
    ///
    /// ## Material Design Usage
    /// - Group related filter options together
    /// - Allow multiple selections for broad filtering
    /// - Provide clear visual feedback for selected state
    pub fn filter(text: impl Into<String>, selected: &'a mut bool) -> Self {
        let mut chip = Self::new(text, ChipVariant::Filter);
        chip.selected = Some(selected);
        chip
    }

    /// Create an input chip representing user-entered data
    ///
    /// Input chips represent discrete pieces of information entered by a user,
    /// such as tags, contacts, or other structured data.
    ///
    /// ## Material Design Usage
    /// - Represent complex entities in a compact form
    /// - Often removable to allow editing of input data
    /// - Used in forms and data entry interfaces
    pub fn input(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Input)
    }

    /// Create a suggestion chip that provides actionable content suggestions
    ///
    /// Suggestion chips are used to help users discover relevant actions or content.
    /// They can be used in conjunction with dynamic features like autocomplete or
    /// content recommendations.
    pub fn suggestion(text: impl Into<String>) -> Self {
        Self::new(text, ChipVariant::Suggestion)
    }

    /// Set whether the chip should have elevation (shadow) effect
    ///
    /// Elevated chips have a surface-container-high background and a shadow
    /// to indicate elevation. This is typically used for assist and suggestion chips.
    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    /// Enable or disable the chip
    ///
    /// Disabled chips have a different visual treatment and do not respond to
    /// user interactions. Soft-disabled chips are still visible but appear
    /// with reduced opacity.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        if enabled {
            self.soft_disabled = false; // if enabled, can't be soft disabled
        }
        self
    }

    /// Set the chip as soft-disabled
    ///
    /// Soft-disabled chips have a different visual treatment (e.g., lighter opacity)
    /// compared to hard-disabled chips. They are still interactive but indicate
    /// that the action is unavailable.
    pub fn soft_disabled(mut self, soft_disabled: bool) -> Self {
        self.soft_disabled = soft_disabled;
        if soft_disabled {
            self.enabled = false; // soft disabled means not enabled
        }
        self
    }

    /// Set whether the chip can be removed
    ///
    /// Removable chips show an X icon that allows users to remove the chip
    /// from the UI. This is useful for input and filter chips.
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    /// Set a leading icon for the chip using a Material icon name
    ///
    /// The icon will be displayed on the left side of the chip's text.
    /// This is commonly used for assist and filter chips.
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(IconType::MaterialIcon(icon.into()));
        self
    }

    /// Set a leading icon for the chip using SVG data
    ///
    /// The SVG data will be converted to a texture and displayed on the left
    /// side of the chip's text. This allows for custom icons with scalable
    /// vector graphics.
    pub fn leading_icon_svg(mut self, svg_data: impl Into<String>) -> Self {
        self.leading_icon = Some(IconType::SvgData(svg_data.into()));
        self
    }

    /// Set a leading icon for the chip using PNG image data
    ///
    /// The PNG image data will be converted to a texture and displayed on the left
    /// side of the chip's text. This is useful for using raster images as icons.
    pub fn leading_icon_png(mut self, png_bytes: Vec<u8>) -> Self {
        self.leading_icon = Some(IconType::PngBytes(png_bytes));
        self
    }

    /// Set a pre-loaded texture as the leading icon for the chip
    ///
    /// This allows using any texture as an icon, without the need to convert
    /// from image data. The texture should be created and managed externally.
    pub fn leading_icon_texture(mut self, texture: TextureHandle) -> Self {
        self.leading_icon = Some(IconType::Texture(texture));
        self
    }

    /// Set whether to use avatar-style rounded appearance for the chip
    ///
    /// Avatar-style chips have a more pronounced roundness, making them suitable
    /// for representing users or profile-related content. Regular chips are more
    /// rectangular.
    pub fn avatar(mut self, avatar: bool) -> Self {
        self.avatar = avatar;
        self
    }

    /// Set a callback function to be called when the chip is clicked
    ///
    /// This allows defining custom actions for each chip, such as navigating to
    /// a different view, opening a dialog, or triggering any other behavior.
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

/// Resolved chip colors for rendering
struct ChipColors {
    bg: Color32,
    border: Color32,
    text: Color32,
    icon: Color32,
    delete_icon: Color32,
    state_layer: Color32,
}

/// Resolve chip colors per Material Design 3 spec (_ChipDefaultsM3)
fn resolve_chip_colors(
    variant: ChipVariant,
    is_selected: bool,
    enabled: bool,
    soft_disabled: bool,
    elevated: bool,
    is_hovered: bool,
    is_pressed: bool,
) -> ChipColors {
    let on_surface = get_global_color("onSurface");
    let on_surface_variant = get_global_color("onSurfaceVariant");
    let outline_variant = get_global_color("outlineVariant");
    let surface_container_low = get_global_color("surfaceContainerLow");
    let secondary_container = get_global_color("secondaryContainer");
    let on_secondary_container = get_global_color("onSecondaryContainer");
    let primary = get_global_color("primary");

    // Disabled states (shared across all variants per M3 spec)
    if !enabled {
        let (bg, border, text) = if soft_disabled {
            (
                on_surface.gamma_multiply(0.12),
                Color32::TRANSPARENT,
                on_surface.gamma_multiply(0.60),
            )
        } else {
            (
                on_surface.gamma_multiply(0.12),
                on_surface.gamma_multiply(0.12),
                on_surface.gamma_multiply(0.38),
            )
        };
        return ChipColors {
            bg,
            border,
            text,
            icon: text,
            delete_icon: text,
            state_layer: Color32::TRANSPARENT,
        };
    }

    // State layer (shared logic for all enabled variants)
    let state_layer_base = if is_selected { on_secondary_container } else { on_surface_variant };
    let state_layer = if is_pressed {
        state_layer_base.gamma_multiply(0.12)
    } else if is_hovered {
        state_layer_base.gamma_multiply(0.08)
    } else {
        Color32::TRANSPARENT
    };

    // Selected filter chip
    if variant == ChipVariant::Filter && is_selected {
        return ChipColors {
            bg: secondary_container,
            border: Color32::TRANSPARENT,
            text: on_secondary_container,
            icon: primary,
            delete_icon: on_secondary_container,
            state_layer,
        };
    }

    // Elevated (unselected)
    if elevated {
        return ChipColors {
            bg: surface_container_low,
            border: Color32::TRANSPARENT,
            text: on_surface_variant,
            icon: primary,
            delete_icon: on_surface_variant,
            state_layer,
        };
    }

    // Default (flat, unselected)
    ChipColors {
        bg: Color32::TRANSPARENT,
        border: outline_variant,
        text: on_surface_variant,
        icon: primary,
        delete_icon: on_surface_variant,
        state_layer,
    }
}

impl<'a> Widget for MaterialChip<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let is_selected = self.selected.as_ref().map_or(false, |s| **s);

        let text_width = ui.fonts(|fonts| {
            fonts
                .layout_no_wrap(
                    self.text.clone(),
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                )
                .rect
                .width()
        });

        let has_leading = self.leading_icon.is_some()
            || (self.variant == ChipVariant::Filter && is_selected);
        let icon_width = if has_leading { 24.0 } else { 0.0 };
        let remove_width = if self.removable { 24.0 } else { 0.0 };
        let padding = 16.0;

        let desired_size = Vec2::new(
            (text_width + icon_width + remove_width + padding).min(ui.available_width()),
            32.0,
        );

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        let is_pressed = response.is_pointer_button_down_on();
        let is_hovered = response.hovered();

        let colors = resolve_chip_colors(
            self.variant,
            is_selected,
            self.enabled,
            self.soft_disabled,
            self.elevated,
            is_hovered,
            is_pressed,
        );

        let corner_radius = 8.0;

        // Draw elevation shadow (before background)
        if self.elevated && self.enabled {
            let shadow_rect = rect.translate(Vec2::new(0.0, 2.0));
            ui.painter().rect_filled(
                shadow_rect,
                corner_radius,
                Color32::from_rgba_unmultiplied(0, 0, 0, 30),
            );
        }

        // Draw chip background
        ui.painter().rect_filled(rect, corner_radius, colors.bg);

        // Draw state layer (hover/pressed overlay)
        if colors.state_layer != Color32::TRANSPARENT {
            ui.painter()
                .rect_filled(rect, corner_radius, colors.state_layer);
        }

        // Draw chip border
        if colors.border != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                corner_radius,
                Stroke::new(1.0, colors.border),
                egui::epaint::StrokeKind::Outside,
            );
        }

        // Layout content
        let mut content_x = rect.min.x + 8.0;

        // Draw leading icon or checkmark
        if let Some(icon) = &self.leading_icon {
            let icon_rect = Rect::from_min_size(
                Pos2::new(content_x, rect.center().y - 10.0),
                Vec2::splat(20.0),
            );

            match icon {
                IconType::MaterialIcon(icon_str) => {
                    ui.painter().text(
                        icon_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        icon_str,
                        egui::FontId::proportional(16.0),
                        colors.icon,
                    );
                }
                IconType::SvgData(svg_data) => {
                    if let Ok(texture) = image_utils::create_texture_from_svg(
                        ui.ctx(),
                        svg_data,
                        &format!("chip_svg_{}", svg_data.len()),
                    ) {
                        ui.painter().image(
                            texture.id(),
                            icon_rect,
                            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                            Color32::WHITE,
                        );
                    }
                }
                IconType::PngBytes(png_bytes) => {
                    if let Ok(texture) = image_utils::create_texture_from_png_bytes(
                        ui.ctx(),
                        png_bytes,
                        &format!("chip_png_{}", png_bytes.len()),
                    ) {
                        ui.painter().image(
                            texture.id(),
                            icon_rect,
                            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                            Color32::WHITE,
                        );
                    }
                }
                IconType::Texture(texture) => {
                    ui.painter().image(
                        texture.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
            }
            content_x += 24.0;
        } else if self.variant == ChipVariant::Filter && is_selected {
            // Draw checkmark for selected filter chips
            let icon_rect = Rect::from_min_size(
                Pos2::new(content_x, rect.center().y - 10.0),
                Vec2::splat(20.0),
            );

            let center = icon_rect.center();
            let checkmark_size = 12.0;

            let start = Pos2::new(center.x - checkmark_size * 0.3, center.y);
            let middle = Pos2::new(
                center.x - checkmark_size * 0.1,
                center.y + checkmark_size * 0.2,
            );
            let end = Pos2::new(
                center.x + checkmark_size * 0.3,
                center.y - checkmark_size * 0.2,
            );

            ui.painter()
                .line_segment([start, middle], Stroke::new(2.0, colors.icon));
            ui.painter()
                .line_segment([middle, end], Stroke::new(2.0, colors.icon));
            content_x += 24.0;
        }

        // Draw text
        let text_pos = Pos2::new(content_x, rect.center().y);
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            &self.text,
            egui::FontId::default(),
            colors.text,
        );

        // Draw remove button for removable chips
        if self.removable {
            let remove_rect = Rect::from_min_size(
                Pos2::new(rect.max.x - 24.0, rect.center().y - 10.0),
                Vec2::splat(20.0),
            );

            let center = remove_rect.center();
            let cross_size = 8.0;
            ui.painter().line_segment(
                [
                    Pos2::new(center.x - cross_size / 2.0, center.y - cross_size / 2.0),
                    Pos2::new(center.x + cross_size / 2.0, center.y + cross_size / 2.0),
                ],
                Stroke::new(1.5, colors.delete_icon),
            );
            ui.painter().line_segment(
                [
                    Pos2::new(center.x + cross_size / 2.0, center.y - cross_size / 2.0),
                    Pos2::new(center.x - cross_size / 2.0, center.y + cross_size / 2.0),
                ],
                Stroke::new(1.5, colors.delete_icon),
            );
        }

        // Handle interactions
        if response.clicked() && self.enabled {
            match self.variant {
                ChipVariant::Filter => {
                    if let Some(selected) = self.selected {
                        *selected = !*selected;
                        response.mark_changed();
                    }
                }
                _ => {
                    if let Some(action) = self.action {
                        action();
                    }
                }
            }
        }

        response
    }
}

pub fn assist_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::assist(text)
}

pub fn filter_chip(text: impl Into<String>, selected: &mut bool) -> MaterialChip<'_> {
    MaterialChip::filter(text, selected)
}

pub fn input_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::input(text)
}

pub fn suggestion_chip(text: impl Into<String>) -> MaterialChip<'static> {
    MaterialChip::suggestion(text)
}
