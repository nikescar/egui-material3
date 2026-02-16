use crate::get_global_color;
use eframe::egui::{self, Color32, Context, Id, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Corner position for menu positioning.
#[derive(Clone, Copy, PartialEq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Focus state for keyboard navigation.
#[derive(Clone, Copy, PartialEq)]
pub enum FocusState {
    None,
    ListRoot,
    FirstItem,
}

/// Positioning mode for the menu.
#[derive(Clone, Copy, PartialEq)]
pub enum Positioning {
    Absolute,
    Fixed,
    Document,
    Popover,
}

/// The visual properties that menus have in common.
///
/// Based on Flutter's `MenuStyle`. Controls the appearance of menus
/// created by `MaterialMenu`. Properties that are `None` use Material
/// Design 3 defaults.
///
/// # M3 Defaults
/// - `background_color`: `surfaceContainer`
/// - `shadow_color`: `shadow`
/// - `elevation`: `3.0`
/// - `padding`: `8.0` (vertical)
/// - `corner_radius`: `4.0`
/// - `min_width`: `112.0`
/// - `max_width`: `280.0`
#[derive(Clone, Debug)]
pub struct MenuStyle {
    /// The menu's background fill color.
    pub background_color: Option<Color32>,
    /// The shadow color of the menu's surface.
    pub shadow_color: Option<Color32>,
    /// The surface tint color of the menu.
    pub surface_tint_color: Option<Color32>,
    /// The elevation of the menu (affects shadow size).
    pub elevation: Option<f32>,
    /// The vertical padding inside the menu.
    pub padding: Option<f32>,
    /// The minimum width of the menu.
    pub min_width: Option<f32>,
    /// The maximum width of the menu.
    pub max_width: Option<f32>,
    /// The corner radius of the menu shape.
    pub corner_radius: Option<f32>,
}

impl Default for MenuStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            shadow_color: None,
            surface_tint_color: None,
            elevation: None,
            padding: None,
            min_width: None,
            max_width: None,
            corner_radius: None,
        }
    }
}

impl MenuStyle {
    /// Returns a copy of this `MenuStyle` where `None` fields are
    /// filled in from `other`.
    pub fn merge(&self, other: &MenuStyle) -> MenuStyle {
        MenuStyle {
            background_color: self.background_color.or(other.background_color),
            shadow_color: self.shadow_color.or(other.shadow_color),
            surface_tint_color: self.surface_tint_color.or(other.surface_tint_color),
            elevation: self.elevation.or(other.elevation),
            padding: self.padding.or(other.padding),
            min_width: self.min_width.or(other.min_width),
            max_width: self.max_width.or(other.max_width),
            corner_radius: self.corner_radius.or(other.corner_radius),
        }
    }

    /// Resolve all style values, applying M3 defaults for `None` fields.
    fn resolve(&self) -> ResolvedMenuStyle {
        ResolvedMenuStyle {
            background_color: self
                .background_color
                .unwrap_or_else(|| get_global_color("surfaceContainer")),
            shadow_color: self
                .shadow_color
                .unwrap_or_else(|| get_global_color("shadow")),
            elevation: self.elevation.unwrap_or(3.0),
            padding: self.padding.unwrap_or(8.0),
            _min_width: self.min_width.unwrap_or(112.0),
            max_width: self.max_width.unwrap_or(280.0),
            corner_radius: self.corner_radius.unwrap_or(4.0),
        }
    }
}

/// Fully resolved menu style with no optional values.
struct ResolvedMenuStyle {
    background_color: Color32,
    shadow_color: Color32,
    elevation: f32,
    padding: f32,
    _min_width: f32,
    max_width: f32,
    corner_radius: f32,
}

/// Theme data for menus created by `MaterialMenu`.
///
/// Based on Flutter's `MenuThemeData`. Typically specified as part of
/// an overall theme. All properties are optional and default to `None`.
#[derive(Clone, Debug, Default)]
pub struct MenuThemeData {
    /// The `MenuStyle` for submenus.
    pub style: Option<MenuStyle>,
}

/// Theme data for menu bars.
///
/// Based on Flutter's `MenuBarThemeData`. Defines the visual properties
/// of menu bar widgets themselves, but not their submenus.
#[derive(Clone, Debug, Default)]
pub struct MenuBarThemeData {
    /// The `MenuStyle` for the menu bar.
    pub style: Option<MenuStyle>,
}

/// Theme data for menu buttons (menu items).
///
/// Based on Flutter's `MenuButtonThemeData` and `_MenuButtonDefaultsM3`.
/// Controls the appearance of individual menu items.
///
/// # M3 Defaults
/// - `foreground_color`: `onSurface`
/// - `icon_color`: `onSurfaceVariant`
/// - `disabled_foreground_color`: `onSurface` at 38% opacity
/// - `disabled_icon_color`: `onSurface` at 38% opacity
/// - `hover_overlay_opacity`: `0.08`
/// - `pressed_overlay_opacity`: `0.10`
/// - `min_height`: `48.0`
/// - `icon_size`: `24.0`
/// - `padding_horizontal`: `12.0`
#[derive(Clone, Debug)]
pub struct MenuButtonThemeData {
    /// Text/foreground color for enabled menu items.
    pub foreground_color: Option<Color32>,
    /// Icon color for enabled menu items.
    pub icon_color: Option<Color32>,
    /// Text/foreground color for disabled menu items.
    pub disabled_foreground_color: Option<Color32>,
    /// Icon color for disabled menu items.
    pub disabled_icon_color: Option<Color32>,
    /// Opacity of hover overlay (0.0 to 1.0).
    pub hover_overlay_opacity: Option<f32>,
    /// Opacity of pressed overlay (0.0 to 1.0).
    pub pressed_overlay_opacity: Option<f32>,
    /// Font for menu item text.
    pub text_font: Option<egui::FontId>,
    /// Minimum height of a menu item.
    pub min_height: Option<f32>,
    /// Size of leading/trailing icons.
    pub icon_size: Option<f32>,
    /// Horizontal padding inside each menu item.
    pub padding_horizontal: Option<f32>,
}

impl Default for MenuButtonThemeData {
    fn default() -> Self {
        Self {
            foreground_color: None,
            icon_color: None,
            disabled_foreground_color: None,
            disabled_icon_color: None,
            hover_overlay_opacity: None,
            pressed_overlay_opacity: None,
            text_font: None,
            min_height: None,
            icon_size: None,
            padding_horizontal: None,
        }
    }
}

impl MenuButtonThemeData {
    /// Resolve all button theme values, applying M3 defaults for `None` fields.
    fn resolve(&self) -> ResolvedMenuButtonTheme {
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let disabled_color = Color32::from_rgba_premultiplied(
            on_surface.r(),
            on_surface.g(),
            on_surface.b(),
            97, // ~38% of 255
        );

        ResolvedMenuButtonTheme {
            foreground_color: self.foreground_color.unwrap_or(on_surface),
            icon_color: self.icon_color.unwrap_or(on_surface_variant),
            disabled_foreground_color: self.disabled_foreground_color.unwrap_or(disabled_color),
            disabled_icon_color: self.disabled_icon_color.unwrap_or(disabled_color),
            hover_overlay_opacity: self.hover_overlay_opacity.unwrap_or(0.08),
            pressed_overlay_opacity: self.pressed_overlay_opacity.unwrap_or(0.10),
            text_font: self.text_font.clone().unwrap_or_default(),
            min_height: self.min_height.unwrap_or(48.0),
            icon_size: self.icon_size.unwrap_or(24.0),
            padding_horizontal: self.padding_horizontal.unwrap_or(12.0),
        }
    }
}

/// Fully resolved menu button theme with no optional values.
struct ResolvedMenuButtonTheme {
    foreground_color: Color32,
    icon_color: Color32,
    disabled_foreground_color: Color32,
    disabled_icon_color: Color32,
    hover_overlay_opacity: f32,
    pressed_overlay_opacity: f32,
    text_font: egui::FontId,
    min_height: f32,
    icon_size: f32,
    padding_horizontal: f32,
}

/// Material Design menu component.
///
/// Menus display a list of choices on a temporary surface.
/// They appear when users interact with a button, action, or other control.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut menu_open = false;
///
/// if ui.button("Open Menu").clicked() {
///     menu_open = true;
/// }
///
/// let mut menu = MaterialMenu::new(&mut menu_open)
///     .item("Cut", Some(|| println!("Cut")))
///     .item("Copy", Some(|| println!("Copy")))
///     .item("Paste", Some(|| println!("Paste")));
///
/// if menu_open {
///     ui.add(menu);
/// }
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialMenu<'a> {
    /// Unique identifier for the menu
    id: Id,
    /// Reference to the menu open state
    open: &'a mut bool,
    /// Rectangle to anchor the menu to
    anchor_rect: Option<Rect>,
    /// List of menu items
    items: Vec<MenuItem<'a>>,
    /// Corner of the anchor element to align to
    anchor_corner: Corner,
    /// Corner of the menu to align with the anchor
    menu_corner: Corner,
    /// Initial focus state for keyboard navigation
    default_focus: FocusState,
    /// Positioning mode
    positioning: Positioning,
    /// Whether the menu uses quick animation
    quick: bool,
    /// Whether the menu has overflow scrolling
    has_overflow: bool,
    /// Keep menu open when clicking outside
    stay_open_on_outside_click: bool,
    /// Keep menu open when focus moves away
    stay_open_on_focusout: bool,
    /// Don't restore focus when menu closes
    skip_restore_focus: bool,
    /// Horizontal offset from anchor
    x_offset: f32,
    /// Vertical offset from anchor
    y_offset: f32,
    /// Prevent horizontal flipping when menu would go offscreen
    no_horizontal_flip: bool,
    /// Prevent vertical flipping when menu would go offscreen
    no_vertical_flip: bool,
    /// Delay for typeahead search in milliseconds
    typeahead_delay: f32,
    /// Tab index for keyboard navigation
    list_tab_index: i32,
    /// Optional menu style override
    menu_style: Option<MenuStyle>,
    /// Optional menu button theme override
    button_theme: Option<MenuButtonThemeData>,
}

/// Individual menu item data.
pub struct MenuItem<'a> {
    /// Display text for the menu item
    text: String,
    /// Optional icon to display at the start of the item
    leading_icon: Option<String>,
    /// Optional icon to display at the end of the item
    trailing_icon: Option<String>,
    /// Whether the menu item is enabled and interactive
    enabled: bool,
    /// Whether to show a divider line after this item
    divider_after: bool,
    /// Callback function to execute when the item is clicked
    action: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MaterialMenu<'a> {
    /// Create a new MaterialMenu instance.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this menu
    /// * `open` - Mutable reference to the menu's open state
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut menu_open = false;
    /// let menu = MaterialMenu::new("main_menu", &mut menu_open);
    /// # });
    /// ```
    pub fn new(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self {
            id: id.into(),
            open,
            anchor_rect: None,
            items: Vec::new(),
            // Default values matching Material Web behavior
            anchor_corner: Corner::BottomLeft,
            menu_corner: Corner::TopLeft,
            default_focus: FocusState::None,
            positioning: Positioning::Absolute,
            quick: false,
            has_overflow: false,
            stay_open_on_outside_click: false,
            stay_open_on_focusout: false,
            skip_restore_focus: false,
            x_offset: 0.0,
            y_offset: 0.0,
            no_horizontal_flip: false,
            no_vertical_flip: false,
            typeahead_delay: 200.0,
            list_tab_index: -1,
            menu_style: None,
            button_theme: None,
        }
    }

    /// Set the anchor rectangle for the menu.
    ///
    /// The menu will be positioned relative to this rectangle.
    ///
    /// # Arguments
    /// * `rect` - The rectangle to anchor the menu to
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut menu_open = false;
    /// let button_rect = ui.available_rect_before_wrap();
    /// let menu = MaterialMenu::new("menu", &mut menu_open)
    ///     .anchor_rect(button_rect);
    /// # });
    /// ```
    pub fn anchor_rect(mut self, rect: Rect) -> Self {
        self.anchor_rect = Some(rect);
        self
    }

    /// Add an item to the menu.
    ///
    /// # Arguments
    /// * `item` - The menu item to add
    ///
    /// # Example
    /// ```rust
    /// # egui::__run_test_ui(|ui| {
    /// let mut menu_open = false;
    /// let item = MenuItem::new("Cut").action(|| println!("Cut"));
    /// let menu = MaterialMenu::new("menu", &mut menu_open).item(item);
    /// # });
    /// ```
    pub fn item(mut self, item: MenuItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    /// Set the menu style, overriding Material Design 3 defaults.
    ///
    /// # Arguments
    /// * `style` - The `MenuStyle` to apply
    pub fn style(mut self, style: MenuStyle) -> Self {
        self.menu_style = Some(style);
        self
    }

    /// Set the button theme, overriding Material Design 3 defaults.
    ///
    /// # Arguments
    /// * `theme` - The `MenuButtonThemeData` to apply
    pub fn button_theme(mut self, theme: MenuButtonThemeData) -> Self {
        self.button_theme = Some(theme);
        self
    }

    /// Set the elevation (shadow) of the menu.
    ///
    /// This is a shorthand for setting `MenuStyle.elevation`.
    /// Material Design defines typical elevation levels from 0 to 12.
    ///
    /// # Arguments
    /// * `elevation` - Elevation level
    pub fn elevation(mut self, elevation: f32) -> Self {
        let style = self.menu_style.get_or_insert_with(MenuStyle::default);
        style.elevation = Some(elevation);
        self
    }

    /// Set the anchor corner for the menu.
    pub fn anchor_corner(mut self, corner: Corner) -> Self {
        self.anchor_corner = corner;
        self
    }

    /// Set the menu corner for positioning.
    pub fn menu_corner(mut self, corner: Corner) -> Self {
        self.menu_corner = corner;
        self
    }

    /// Set the default focus state for the menu.
    pub fn default_focus(mut self, focus: FocusState) -> Self {
        self.default_focus = focus;
        self
    }

    /// Set the positioning mode for the menu.
    pub fn positioning(mut self, positioning: Positioning) -> Self {
        self.positioning = positioning;
        self
    }

    /// Enable or disable quick animation for the menu.
    pub fn quick(mut self, quick: bool) -> Self {
        self.quick = quick;
        self
    }

    /// Enable or disable overflow scrolling for the menu.
    pub fn has_overflow(mut self, has_overflow: bool) -> Self {
        self.has_overflow = has_overflow;
        self
    }

    /// Keep the menu open when clicking outside of it.
    pub fn stay_open_on_outside_click(mut self, stay_open: bool) -> Self {
        self.stay_open_on_outside_click = stay_open;
        self
    }

    /// Keep the menu open when focus moves away from it.
    pub fn stay_open_on_focusout(mut self, stay_open: bool) -> Self {
        self.stay_open_on_focusout = stay_open;
        self
    }

    /// Skip restoring focus when the menu closes.
    pub fn skip_restore_focus(mut self, skip: bool) -> Self {
        self.skip_restore_focus = skip;
        self
    }

    /// Set the horizontal offset for the menu.
    pub fn x_offset(mut self, offset: f32) -> Self {
        self.x_offset = offset;
        self
    }

    /// Set the vertical offset for the menu.
    pub fn y_offset(mut self, offset: f32) -> Self {
        self.y_offset = offset;
        self
    }

    /// Prevent horizontal flipping when the menu would go offscreen.
    pub fn no_horizontal_flip(mut self, no_flip: bool) -> Self {
        self.no_horizontal_flip = no_flip;
        self
    }

    /// Prevent vertical flipping when the menu would go offscreen.
    pub fn no_vertical_flip(mut self, no_flip: bool) -> Self {
        self.no_vertical_flip = no_flip;
        self
    }

    /// Set the typeahead delay for the menu.
    pub fn typeahead_delay(mut self, delay: f32) -> Self {
        self.typeahead_delay = delay;
        self
    }

    /// Set the tab index for keyboard navigation.
    pub fn list_tab_index(mut self, index: i32) -> Self {
        self.list_tab_index = index;
        self
    }

    /// Show the menu in the given context.
    pub fn show(self, ctx: &Context) {
        if !*self.open {
            return;
        }

        let resolved_style = self
            .menu_style
            .as_ref()
            .unwrap_or(&MenuStyle::default())
            .resolve();
        let resolved_button = self
            .button_theme
            .as_ref()
            .unwrap_or(&MenuButtonThemeData::default())
            .resolve();

        // Use a stable ID for the menu
        let stable_id = egui::Id::new(format!("menu_{}", self.id.value()));

        // Track if this is the frame when menu was opened
        let was_opened_this_frame = ctx.data_mut(|d| {
            let last_open_state = d
                .get_temp::<bool>(stable_id.with("was_open_last_frame"))
                .unwrap_or(false);
            let just_opened = !last_open_state && *self.open;
            d.insert_temp(stable_id.with("was_open_last_frame"), *self.open);
            just_opened
        });

        // Request focus when menu opens
        if was_opened_this_frame && !self.skip_restore_focus {
            ctx.memory_mut(|mem| mem.request_focus(stable_id));
        }

        let item_height = resolved_button.min_height;
        let vertical_padding = resolved_style.padding * 2.0;
        let total_height = self.items.len() as f32 * item_height
            + self.items.iter().filter(|item| item.divider_after).count() as f32
            + vertical_padding;
        let menu_width = resolved_style.max_width;

        let menu_size = Vec2::new(menu_width, total_height);

        // Determine position based on anchor corner and menu corner
        let position = if let Some(anchor) = self.anchor_rect {
            let anchor_point = match self.anchor_corner {
                Corner::TopLeft => anchor.min,
                Corner::TopRight => Pos2::new(anchor.max.x, anchor.min.y),
                Corner::BottomLeft => Pos2::new(anchor.min.x, anchor.max.y),
                Corner::BottomRight => anchor.max,
            };

            let menu_offset = match self.menu_corner {
                Corner::TopLeft => Vec2::ZERO,
                Corner::TopRight => Vec2::new(-menu_size.x, 0.0),
                Corner::BottomLeft => Vec2::new(0.0, -menu_size.y),
                Corner::BottomRight => -menu_size,
            };

            // Apply the corner positioning and offsets
            let base_position = anchor_point + menu_offset;
            Pos2::new(
                base_position.x + self.x_offset,
                base_position.y + self.y_offset + 4.0, // 4px spacing from anchor
            )
        } else {
            // Center on screen
            let screen_rect = ctx.screen_rect();
            screen_rect.center() - menu_size / 2.0
        };

        let open_ref = self.open;
        let _id = self.id;
        let items = self.items;
        let stay_open_on_outside_click = self.stay_open_on_outside_click;
        let _stay_open_on_focusout = self.stay_open_on_focusout;

        // Create a popup window for the menu with a stable layer and unique ID
        let _area_response = egui::Area::new(stable_id)
            .fixed_pos(position)
            .order(egui::Order::Foreground)
            .interactable(true)
            .show(ctx, |ui| {
                render_menu_content(
                    ui,
                    menu_size,
                    items,
                    &resolved_style,
                    &resolved_button,
                    open_ref,
                )
            });

        // Handle closing behavior based on settings
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *open_ref = false;
        } else if !stay_open_on_outside_click && !was_opened_this_frame {
            // Only handle outside clicks if not staying open and not just opened
            if ctx.input(|i| i.pointer.any_click()) {
                let pointer_pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();
                let menu_rect = Rect::from_min_size(position, menu_size);

                // Include anchor rect in the "inside" area to prevent closing when clicking trigger
                let mut inside_area = menu_rect;
                if let Some(anchor) = self.anchor_rect {
                    inside_area = inside_area.union(anchor);
                }

                // Only close if click was outside both menu and anchor areas
                if !inside_area.contains(pointer_pos) {
                    *open_ref = false;
                }
            }
        }
    }
}

fn render_menu_content<'a>(
    ui: &mut Ui,
    size: Vec2,
    items: Vec<MenuItem<'a>>,
    style: &ResolvedMenuStyle,
    button_theme: &ResolvedMenuButtonTheme,
    open_ref: &'a mut bool,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

    let outline_variant = get_global_color("outlineVariant");

    // Draw shadow for elevation
    let shadow_offset = style.elevation * 2.0;
    let shadow_rect = rect.expand(shadow_offset);
    let shadow_alpha = ((style.elevation * 10.0) as u8).min(80);
    ui.painter().rect_filled(
        shadow_rect,
        style.corner_radius,
        Color32::from_rgba_premultiplied(
            style.shadow_color.r(),
            style.shadow_color.g(),
            style.shadow_color.b(),
            shadow_alpha,
        ),
    );

    // Draw menu background
    ui.painter()
        .rect_filled(rect, style.corner_radius, style.background_color);

    // Draw menu border
    ui.painter().rect_stroke(
        rect,
        style.corner_radius,
        Stroke::new(1.0, outline_variant),
        egui::epaint::StrokeKind::Outside,
    );

    let mut current_y = rect.min.y + style.padding;
    let mut pending_actions = Vec::new();
    let mut should_close = false;

    for (index, item) in items.into_iter().enumerate() {
        let item_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 8.0, current_y),
            Vec2::new(rect.width() - 16.0, button_theme.min_height),
        );

        let item_response = ui.interact(
            item_rect,
            egui::Id::new(format!("menu_item_{}_{}", rect.min.x as i32, index)),
            Sense::click(),
        );

        // Draw item background on hover/press
        if item.enabled {
            let overlay_opacity = if item_response.is_pointer_button_down_on() {
                button_theme.pressed_overlay_opacity
            } else if item_response.hovered() {
                button_theme.hover_overlay_opacity
            } else {
                0.0
            };

            if overlay_opacity > 0.0 {
                let on_surface = button_theme.foreground_color;
                let overlay_alpha = (overlay_opacity * 255.0) as u8;
                let hover_color = Color32::from_rgba_premultiplied(
                    on_surface.r(),
                    on_surface.g(),
                    on_surface.b(),
                    overlay_alpha,
                );
                ui.painter().rect_filled(item_rect, 4.0, hover_color);
            }
        }

        // Handle click
        if item_response.clicked() && item.enabled {
            if let Some(action) = item.action {
                pending_actions.push(action);
                should_close = true;
            }
        }

        // Layout item content
        let mut content_x = item_rect.min.x + button_theme.padding_horizontal;
        let content_y = item_rect.center().y;

        // Draw leading icon
        if let Some(_icon) = &item.leading_icon {
            let half_icon = button_theme.icon_size / 2.0;
            let icon_rect = Rect::from_min_size(
                Pos2::new(content_x, content_y - half_icon),
                Vec2::splat(button_theme.icon_size),
            );

            let icon_color = if item.enabled {
                button_theme.icon_color
            } else {
                button_theme.disabled_icon_color
            };

            ui.painter()
                .circle_filled(icon_rect.center(), half_icon / 3.0 * 2.0, icon_color);
            content_x += button_theme.icon_size + button_theme.padding_horizontal;
        }

        // Draw text
        let text_color = if item.enabled {
            button_theme.foreground_color
        } else {
            button_theme.disabled_foreground_color
        };

        let text_pos = Pos2::new(content_x, content_y);
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            &item.text,
            button_theme.text_font.clone(),
            text_color,
        );

        // Draw trailing icon
        if let Some(_icon) = &item.trailing_icon {
            let half_icon = button_theme.icon_size / 2.0;
            let icon_rect = Rect::from_min_size(
                Pos2::new(
                    item_rect.max.x - button_theme.padding_horizontal - button_theme.icon_size,
                    content_y - half_icon,
                ),
                Vec2::splat(button_theme.icon_size),
            );

            let icon_color = if item.enabled {
                button_theme.icon_color
            } else {
                button_theme.disabled_icon_color
            };

            ui.painter()
                .circle_filled(icon_rect.center(), half_icon / 3.0 * 2.0, icon_color);
        }

        current_y += button_theme.min_height;

        // Draw divider
        if item.divider_after {
            let divider_y = current_y;
            let divider_start = Pos2::new(rect.min.x + 12.0, divider_y);
            let divider_end = Pos2::new(rect.max.x - 12.0, divider_y);

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

    if should_close {
        *open_ref = false;
    }

    response
}

impl<'a> MenuItem<'a> {
    /// Create a new menu item.
    ///
    /// # Arguments
    /// * `text` - Display text for the menu item
    ///
    /// # Example
    /// ```rust
    /// let item = MenuItem::new("Copy");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            leading_icon: None,
            trailing_icon: None,
            enabled: true,
            divider_after: false,
            action: None,
        }
    }

    /// Set the leading icon for the menu item.
    ///
    /// # Arguments
    /// * `icon` - Icon identifier (e.g., "copy", "cut", "paste")
    ///
    /// # Example
    /// ```rust
    /// let item = MenuItem::new("Copy").leading_icon("content_copy");
    /// ```
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set the trailing icon for the menu item.
    ///
    /// # Arguments
    /// * `icon` - Icon identifier (e.g., "keyboard_arrow_right", "check")
    ///
    /// # Example
    /// ```rust
    /// let item = MenuItem::new("Save").trailing_icon("keyboard_arrow_right");
    /// ```
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Enable or disable the menu item.
    ///
    /// # Arguments
    /// * `enabled` - Whether the menu item should be interactive
    ///
    /// # Example
    /// ```rust
    /// let item = MenuItem::new("Paste").enabled(false); // Disabled item
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add a divider after the menu item.
    ///
    /// # Arguments
    /// * `divider` - Whether to show a divider line after this item
    ///
    /// # Example
    /// ```rust
    /// let item = MenuItem::new("Copy").divider_after(true); // Show divider after this item
    /// ```
    pub fn divider_after(mut self, divider: bool) -> Self {
        self.divider_after = divider;
        self
    }

    /// Set the action to be performed when the menu item is clicked.
    ///
    /// # Arguments
    /// * `f` - Closure to execute when the item is clicked
    ///
    /// # Example
    /// ```rust
    /// let item = MenuItem::new("Delete")
    ///     .on_click(|| println!("Item deleted"));
    /// ```
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'a,
    {
        self.action = Some(Box::new(f));
        self
    }
}

/// Convenience function to create a new menu instance.
///
/// Shorthand for `MaterialMenu::new()`.
///
/// # Arguments
/// * `id` - Unique identifier for this menu
/// * `open` - Mutable reference to the menu's open state
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut menu_open = false;
/// let menu = menu("context_menu", &mut menu_open);
/// # });
/// ```
pub fn menu(id: impl Into<egui::Id>, open: &mut bool) -> MaterialMenu<'_> {
    MaterialMenu::new(id, open)
}

/// Convenience function to create a new menu item.
///
/// Shorthand for `MenuItem::new()`.
///
/// # Arguments
/// * `text` - Display text for the menu item
///
/// # Example
/// ```rust
/// let item = menu_item("Copy")
///     .leading_icon("content_copy")
///     .on_click(|| println!("Copy action"));
/// ```
pub fn menu_item(text: impl Into<String>) -> MenuItem<'static> {
    MenuItem::new(text)
}
