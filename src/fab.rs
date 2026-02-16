use crate::get_global_color;
use crate::icon::MaterialIcon;
use crate::icons::icon_text;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};

/// Material Design FAB (Floating Action Button) variants
#[derive(Clone, Copy, PartialEq)]
pub enum FabVariant {
    /// Surface FAB - uses surface colors for neutral actions
    Surface,
    /// Primary FAB - uses primary colors for main actions (most common)
    Primary,
    /// Secondary FAB - uses secondary colors for secondary actions
    Secondary,
    /// Tertiary FAB - uses tertiary colors for alternative actions
    Tertiary,
    /// Branded FAB - uses custom brand colors
    Branded,
}

/// Material Design FAB sizes following Material Design 3 specifications
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FabSize {
    /// Small FAB - 40x40dp, used in compact layouts
    Small,
    /// Regular FAB - 56x56dp, the standard size
    Regular,
    /// Large FAB - 96x96dp, used for prominent actions
    Large,
    /// Extended FAB - Variable width with text, at least 80dp wide
    Extended,
}

/// Material Design Floating Action Button (FAB) component
///
/// FABs help users take primary actions within an app. They appear in front of all screen content,
/// typically as a circular button with an icon in the center.
///
/// ## Usage Examples
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// // Primary FAB with add icon
/// ui.add(MaterialFab::primary()
///     .icon("add")
///     .action(|| println!("Add clicked")));
///
/// // Extended FAB with text
/// ui.add(MaterialFab::primary()
///     .size(FabSize::Extended)
///     .icon("edit")
///     .text("Compose")
///     .action(|| println!("Compose clicked")));
///
/// // Large FAB for prominent action
/// ui.add(MaterialFab::primary()
///     .size(FabSize::Large)
///     .icon("camera")
///     .action(|| println!("Camera clicked")));
/// # });
/// ```
///
/// ## Material Design Spec
/// - Elevation: 6dp (raised above content)
/// - Corner radius: 50% (fully rounded)
/// - Sizes: Small (40dp), Regular (56dp), Large (96dp), Extended (≥80dp)
/// - Icon size: 24dp for regular, 32dp for large
/// - Placement: 16dp from screen edge, above navigation bars
pub struct MaterialFab<'a> {
    /// Color variant of the FAB
    variant: FabVariant,
    /// Size of the FAB
    size: FabSize,
    /// Material Design icon name
    icon: Option<String>,
    /// Text content (for extended FABs)
    text: Option<String>,
    /// Custom SVG icon data
    svg_icon: Option<SvgIcon>,
    /// Whether the FAB is interactive
    enabled: bool,
    /// Action callback when FAB is pressed
    action: Option<Box<dyn Fn() + 'a>>,
}

/// SVG icon data for custom FAB icons
#[derive(Clone)]
pub struct SvgIcon {
    /// Vector of SVG paths that make up the icon
    pub paths: Vec<SvgPath>,
    /// Viewbox dimensions of the SVG
    pub viewbox_size: Vec2,
}

/// Individual SVG path with styling
#[derive(Clone)]
pub struct SvgPath {
    /// SVG path data string
    pub path: String,
    /// Fill color for the path
    pub fill: Color32,
}

impl<'a> MaterialFab<'a> {
    /// Create a new FAB with the specified variant
    ///
    /// ## Parameters
    /// - `variant`: The color variant to use for the FAB
    pub fn new(variant: FabVariant) -> Self {
        Self {
            variant,
            size: FabSize::Regular,
            icon: None,
            text: None,
            svg_icon: None,
            enabled: true,
            action: None,
        }
    }

    /// Create a surface FAB
    pub fn surface() -> Self {
        Self::new(FabVariant::Surface)
    }

    /// Create a primary FAB
    pub fn primary() -> Self {
        Self::new(FabVariant::Primary)
    }

    /// Create a secondary FAB
    pub fn secondary() -> Self {
        Self::new(FabVariant::Secondary)
    }

    /// Create a tertiary FAB
    pub fn tertiary() -> Self {
        Self::new(FabVariant::Tertiary)
    }

    /// Create a branded FAB
    pub fn branded() -> Self {
        Self::new(FabVariant::Branded)
    }

    /// Set the size of the FAB
    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// Set the icon of the FAB
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the text of the FAB (for extended FABs)
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self.size = FabSize::Extended;
        self
    }

    /// Enable or disable the FAB
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the lowered state of the FAB (elevation effect)
    pub fn lowered(self, _lowered: bool) -> Self {
        // Placeholder for lowered state (elevation effect)
        // In a real implementation, this would affect the visual elevation
        self
    }

    /// Set a custom SVG icon for the FAB
    pub fn svg_icon(mut self, svg_icon: SvgIcon) -> Self {
        self.svg_icon = Some(svg_icon);
        self
    }

    /// Set the action to perform when the FAB is clicked
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

impl<'a> Widget for MaterialFab<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = match self.size {
            FabSize::Small => Vec2::splat(40.0),
            FabSize::Regular => Vec2::splat(56.0),
            FabSize::Large => Vec2::splat(96.0),
            FabSize::Extended => {
                let left_margin = 16.0;
                let right_margin = 24.0;
                let icon_width = if self.icon.is_some() || self.svg_icon.is_some() {
                    24.0 + 12.0
                } else {
                    0.0
                };

                let text_width = if let Some(ref text) = self.text {
                    ui.fonts(|fonts| {
                        let font_id = egui::FontId::proportional(14.0);
                        fonts
                            .layout_no_wrap(text.clone(), font_id, Color32::WHITE)
                            .size()
                            .x
                    })
                } else {
                    0.0
                };

                let total_width = left_margin + icon_width + text_width + right_margin;
                Vec2::new(total_width.max(80.0), 56.0) // Minimum width of 80px
            }
        };

        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        // Extract all needed data before partial move
        let action = self.action;
        let enabled = self.enabled;
        let variant = self.variant;
        let size_enum = self.size;
        let icon = self.icon;
        let text = self.text;
        let svg_icon = self.svg_icon;

        let clicked = response.clicked() && enabled;

        if clicked {
            if let Some(action) = action {
                action();
            }
        }

        // Material Design colors
        let primary_color = get_global_color("primary");
        let secondary_color = get_global_color("secondary");
        let tertiary_color = get_global_color("tertiary");
        let surface = get_global_color("surface");
        let on_primary = get_global_color("onPrimary");
        let on_surface = get_global_color("onSurface");

        let (bg_color, icon_color) = if !enabled {
            (
                get_global_color("surfaceContainer"),
                get_global_color("outline"),
            )
        } else {
            match variant {
                FabVariant::Surface => {
                    if response.hovered() {
                        (get_global_color("surfaceContainerHigh"), on_surface)
                    } else {
                        (surface, on_surface)
                    }
                }
                FabVariant::Primary => {
                    if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                primary_color.r().saturating_add(20),
                                primary_color.g().saturating_add(20),
                                primary_color.b().saturating_add(20),
                                255,
                            ),
                            on_primary,
                        )
                    } else {
                        (primary_color, on_primary)
                    }
                }
                FabVariant::Secondary => {
                    if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                secondary_color.r().saturating_add(20),
                                secondary_color.g().saturating_add(20),
                                secondary_color.b().saturating_add(20),
                                255,
                            ),
                            on_primary,
                        )
                    } else {
                        (secondary_color, on_primary)
                    }
                }
                FabVariant::Tertiary => {
                    if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                tertiary_color.r().saturating_add(20),
                                tertiary_color.g().saturating_add(20),
                                tertiary_color.b().saturating_add(20),
                                255,
                            ),
                            on_primary,
                        )
                    } else {
                        (tertiary_color, on_primary)
                    }
                }
                FabVariant::Branded => {
                    // Google brand colors
                    let google_brand = Color32::from_rgb(66, 133, 244);
                    if response.hovered() {
                        (
                            Color32::from_rgba_premultiplied(
                                google_brand.r().saturating_add(20),
                                google_brand.g().saturating_add(20),
                                google_brand.b().saturating_add(20),
                                255,
                            ),
                            on_primary,
                        )
                    } else {
                        (google_brand, on_primary)
                    }
                }
            }
        };

        // Calculate corner radius for FAB
        let corner_radius = match size_enum {
            FabSize::Small => 12.0,
            FabSize::Large => 16.0,
            _ => 14.0,
        };

        // Draw FAB background with less rounded corners
        ui.painter().rect_filled(rect, corner_radius, bg_color);

        // Draw content
        match size_enum {
            FabSize::Extended => {
                // Draw icon and text with proper spacing
                let left_margin = 16.0;
                let _right_margin = 24.0;
                let icon_text_gap = 12.0;
                let mut content_x = rect.min.x + left_margin;

                if let Some(ref icon_name) = icon {
                    let icon_rect = Rect::from_min_size(
                        Pos2::new(content_x, rect.center().y - 12.0),
                        Vec2::splat(24.0),
                    );

                    // Draw material icon
                    let icon_char = icon_text(icon_name);
                    let icon = MaterialIcon::new(icon_char).size(24.0).color(icon_color);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(icon_rect), |ui| {
                        ui.add(icon);
                    });

                    content_x += 24.0 + icon_text_gap;
                } else if let Some(ref _svg_icon) = svg_icon {
                    // Render simplified Google logo for branded FAB
                    draw_google_logo(ui, Pos2::new(content_x + 12.0, rect.center().y), 24.0);
                    content_x += 24.0 + icon_text_gap;
                }

                if let Some(ref text) = text {
                    let text_pos = Pos2::new(content_x, rect.center().y);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        text,
                        egui::FontId::proportional(14.0),
                        icon_color,
                    );
                }
            }
            _ => {
                // Draw centered icon
                if let Some(ref _svg_icon) = svg_icon {
                    let icon_size = match size_enum {
                        FabSize::Small => 18.0,
                        FabSize::Large => 36.0,
                        _ => 24.0,
                    };

                    // Render simplified Google logo for branded FAB
                    draw_google_logo(ui, rect.center(), icon_size);
                } else if let Some(ref icon_name) = icon {
                    let icon_size = match size_enum {
                        FabSize::Small => 18.0,
                        FabSize::Large => 36.0,
                        _ => 24.0,
                    };

                    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
                    let icon_char = icon_text(icon_name);
                    let icon = MaterialIcon::new(icon_char)
                        .size(icon_size)
                        .color(icon_color);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(icon_rect), |ui| {
                        ui.add(icon);
                    });
                } else {
                    // Default add icon
                    let icon_size = match size_enum {
                        FabSize::Small => 18.0,
                        FabSize::Large => 36.0,
                        _ => 24.0,
                    };

                    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
                    let icon_char = icon_text("add");
                    let icon = MaterialIcon::new(icon_char).size(icon_size).color(icon_color);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(icon_rect), |ui| {
                        ui.add(icon);
                    });
                }
            }
        }

        // Add ripple effect on click
        if response.hovered() && enabled {
            let ripple_color = Color32::from_rgba_premultiplied(
                icon_color.r(),
                icon_color.g(),
                icon_color.b(),
                30,
            );
            ui.painter().rect_filled(rect, corner_radius, ripple_color);
        }

        response
    }
}

// Helper function to draw Google logo
fn draw_google_logo(ui: &mut Ui, center: Pos2, size: f32) {
    let half_size = size / 2.0;
    let quarter_size = size / 4.0;

    // Google 4-color logo - simplified version
    // Green (top right)
    ui.painter().rect_filled(
        Rect::from_min_size(
            Pos2::new(center.x, center.y - half_size),
            Vec2::new(half_size, quarter_size),
        ),
        0.0,
        Color32::from_rgb(52, 168, 83), // Green #34A853
    );

    // Blue (right)
    ui.painter().rect_filled(
        Rect::from_min_size(
            Pos2::new(center.x, center.y - quarter_size),
            Vec2::new(half_size, half_size),
        ),
        0.0,
        Color32::from_rgb(66, 133, 244), // Blue #4285F4
    );

    // Yellow (bottom left)
    ui.painter().rect_filled(
        Rect::from_min_size(
            Pos2::new(center.x - half_size, center.y + quarter_size),
            Vec2::new(half_size, quarter_size),
        ),
        0.0,
        Color32::from_rgb(251, 188, 5), // Yellow #FBBC05
    );

    // Red (left)
    ui.painter().rect_filled(
        Rect::from_min_size(
            Pos2::new(center.x - half_size, center.y - half_size),
            Vec2::new(quarter_size, size),
        ),
        0.0,
        Color32::from_rgb(234, 67, 53), // Red #EA4335
    );
}

pub fn fab_surface() -> MaterialFab<'static> {
    MaterialFab::surface()
}

pub fn fab_primary() -> MaterialFab<'static> {
    MaterialFab::primary()
}

pub fn fab_secondary() -> MaterialFab<'static> {
    MaterialFab::secondary()
}

pub fn fab_tertiary() -> MaterialFab<'static> {
    MaterialFab::tertiary()
}

pub fn fab_branded() -> MaterialFab<'static> {
    MaterialFab::branded()
}

/// Create Google branded icon (4-color logo)
pub fn google_branded_icon() -> SvgIcon {
    SvgIcon {
        paths: vec![
            SvgPath {
                path: "M16 16v14h4V20z".to_string(),
                fill: Color32::from_rgb(52, 168, 83), // Green #34A853
            },
            SvgPath {
                path: "M30 16H20l-4 4h14z".to_string(),
                fill: Color32::from_rgb(66, 133, 244), // Blue #4285F4
            },
            SvgPath {
                path: "M6 16v4h10l4-4z".to_string(),
                fill: Color32::from_rgb(251, 188, 5), // Yellow #FBBC05
            },
            SvgPath {
                path: "M20 16V6h-4v14z".to_string(),
                fill: Color32::from_rgb(234, 67, 53), // Red #EA4335
            },
        ],
        viewbox_size: Vec2::new(36.0, 36.0),
    }
}
