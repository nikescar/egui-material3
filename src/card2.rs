// ============================================================================
// Module:       card2
// Description:  Material 3 elevated, filled and outlined cards: clickable,
//               selectable, with media, header, content and actions slots.
//
// Dependencies: egui; crate::{a11y, paint, tokens, MaterialTheme}
// ============================================================================

//! Material Design 3 cards.
//!
//! Token values come from Compose material3's `ElevatedCardTokens`,
//! `FilledCardTokens` and `OutlinedCardTokens` (`androidx-main`) and from
//! `CardDefaults` in `Card.kt` for the disabled colours.
//!
//! | | Elevated | Filled | Outlined |
//! |---|---|---|---|
//! | Container | surfaceContainerLow | surfaceContainerHighest | surface |
//! | Resting | Level1 | Level0 | Level0 |
//! | Hovered | Level2 | Level1 | Level1 |
//! | Focused / pressed | Level1 | Level0 | Level0 |
//! | Dragged | Level4 | Level3 | Level3 |
//! | Outline | none | none | outlineVariant, 1 dp |
//!
//! All three use the medium corner (12 dp, `ShapeKeyTokens.CornerMedium`),
//! `onSurface` content and state layers, and a `secondary` focus ring
//! (`FocusIndicatorColor`). Only clickable and selectable cards change
//! elevation or show a state layer, as in Compose, where a card without
//! `onClick` has no interaction source. M3 no longer tints elevated
//! surfaces, so the container colour never changes with elevation.
//!
//! **Selected (checked) cards.** Compose's card tokens define no selected
//! state; the styling follows Material Components for Android's M3
//! `MaterialCardView`: a `secondary` overlay at 16 %
//! (`m3_card_foreground_color`), a `secondary` outline on outlined cards
//! (`m3_card_stroke_color`), and a check icon in `secondary`, 24 dp
//! (`*CardTokens.IconSize`), 8 dp from the top-end corner
//! (`mtrl_card_checked_icon_margin`).

use crate::tokens::{
    state::DISABLED_CONTENT_OPACITY, ColorRole, CornerSize, Elevation, InteractionState,
    LayoutDirection, TypeRole,
};
use crate::{a11y, paint, MaterialTheme};
use egui::{
    epaint::{CornerRadius, Shape, Stroke, StrokeKind},
    Color32, IdSalt, ImageSource, Margin, Rect, Response, Role, Sense, Ui, UiBuilder, Vec2, Widget,
};

/// `*CardTokens.IconSize`: the check icon of a selected card.
const CHECK_ICON_SIZE: f32 = 24.0;
/// `mtrl_card_checked_icon_margin` (Material Components for Android).
const CHECK_ICON_MARGIN: f32 = 8.0;
/// `m3_card_foreground_color`: the `secondary` overlay of a checked card.
const SELECTED_OVERLAY_OPACITY: f32 = 0.16;
/// `OutlinedCardTokens.OutlineWidth`.
const OUTLINE_WIDTH: f32 = 1.0;
/// `ElevatedCardTokens.DisabledContainerOpacity` (also `FilledCardTokens`).
const DISABLED_CONTAINER_OPACITY: f32 = 0.38;
/// `OutlinedCardTokens.DisabledOutlineOpacity`.
const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
/// Padding of the header and content sections.
const SECTION_PADDING: f32 = 16.0;
/// Padding of the actions row.
const ACTIONS_PADDING: f32 = 8.0;

/// Material Design card variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Card2Variant {
    /// `surfaceContainerLow` with a Level1 shadow.
    Elevated,
    /// `surfaceContainerHighest`, flat.
    Filled,
    /// `surface` with an `outlineVariant` outline, flat.
    Outlined,
}

impl Card2Variant {
    /// Every variant.
    pub const ALL: [Card2Variant; 3] = [
        Card2Variant::Elevated,
        Card2Variant::Filled,
        Card2Variant::Outlined,
    ];

    /// The container colour role (`*CardTokens.ContainerColor`).
    pub const fn container_role(self) -> ColorRole {
        match self {
            Card2Variant::Elevated => ColorRole::SurfaceContainerLow,
            Card2Variant::Filled => ColorRole::SurfaceContainerHighest,
            Card2Variant::Outlined => ColorRole::Surface,
        }
    }

    /// The elevation for an interaction state (`*CardTokens.*ContainerElevation`).
    /// Cards that are not clickable or selectable stay at the resting level.
    pub fn elevation(self, enabled: bool, interactive: bool, state: InteractionState) -> Elevation {
        let (resting, hovered, focused, pressed, dragged, disabled) = match self {
            Card2Variant::Elevated => (
                Elevation::Level1,
                Elevation::Level2,
                Elevation::Level1,
                Elevation::Level1,
                Elevation::Level4,
                Elevation::Level1,
            ),
            Card2Variant::Filled | Card2Variant::Outlined => (
                Elevation::Level0,
                Elevation::Level1,
                Elevation::Level0,
                Elevation::Level0,
                Elevation::Level3,
                Elevation::Level0,
            ),
        };
        if !enabled {
            disabled
        } else if !interactive {
            resting
        } else if state.dragged {
            dragged
        } else if state.pressed {
            pressed
        } else if state.hovered {
            hovered
        } else if state.focused {
            focused
        } else {
            resting
        }
    }
}

type Slot<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

enum Media<'a> {
    Custom(Slot<'a>),
    Image(ImageSource<'a>),
}

enum Selection<'a> {
    None,
    Fixed(bool),
    Toggle(&'a mut bool),
}

/// A Material Design 3 card.
///
/// Content-driven: the card is as tall as its sections (header, media,
/// content, actions), and as wide as the available width unless
/// [`Self::width`] is set.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::MaterialCard2;
/// let mut picked = false;
/// // A selectable card: clicking (or Space/Enter) toggles `picked`.
/// ui.add(MaterialCard2::outlined()
///     .header("Card title", Some("Subtitle"))
///     .content(|ui| {
///         ui.label("Main card content");
///     })
///     .checked(&mut picked));
///
/// // A clickable card with actions.
/// let response = ui.add(MaterialCard2::elevated()
///     .header("Open me", None::<String>)
///     .clickable(true)
///     .actions(|ui| {
///         if ui.button("Action").clicked() {
///             println!("Action clicked!");
///         }
///     }));
/// if response.clicked() {
///     println!("Card clicked!");
/// }
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialCard2<'a> {
    variant: Card2Variant,
    header_title: Option<String>,
    header_subtitle: Option<String>,
    media: Option<Media<'a>>,
    main_content: Option<Slot<'a>>,
    actions_content: Option<Slot<'a>>,
    min_size: Vec2,
    width: Option<f32>,
    corner_radius: Option<CornerRadius>,
    clickable: bool,
    enabled: bool,
    selection: Selection<'a>,
    label: Option<String>,
    id_salt: Option<IdSalt>,
    media_height: f32,
    elevation: Option<Elevation>,
    shadow_color: Option<Color32>,
    margin: f32,
    clip_behavior: bool,
    border_on_foreground: bool,
}

impl<'a> MaterialCard2<'a> {
    /// An elevated card.
    pub fn elevated() -> Self {
        Self::new_with_variant(Card2Variant::Elevated)
    }

    /// A filled card.
    pub fn filled() -> Self {
        Self::new_with_variant(Card2Variant::Filled)
    }

    /// An outlined card.
    pub fn outlined() -> Self {
        Self::new_with_variant(Card2Variant::Outlined)
    }

    /// A card of `variant`.
    pub fn new_with_variant(variant: Card2Variant) -> Self {
        Self {
            variant,
            header_title: None,
            header_subtitle: None,
            media: None,
            main_content: None,
            actions_content: None,
            min_size: Vec2::ZERO,
            width: None,
            corner_radius: None,
            clickable: false,
            enabled: true,
            selection: Selection::None,
            label: None,
            id_salt: None,
            media_height: 160.0,
            elevation: None,
            shadow_color: None,
            margin: 4.0,
            clip_behavior: true,
            border_on_foreground: true,
        }
    }

    /// Sets the header: a title (Title Large) and optional subtitle (Body
    /// Medium, `onSurfaceVariant`). The title is also the accessible label.
    pub fn header(mut self, title: impl Into<String>, subtitle: Option<impl Into<String>>) -> Self {
        self.header_title = Some(title.into());
        self.header_subtitle = subtitle.map(Into::into);
        self
    }

    /// Sets the media area, drawn edge to edge below the header and
    /// clipped to its rect.
    pub fn media_area<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.media = Some(Media::Custom(Box::new(content)));
        self
    }

    /// Sets an image as the media area, clipped to the card's corner radius
    /// where it touches the card's edge.
    pub fn media_image(mut self, image: impl Into<ImageSource<'a>>) -> Self {
        self.media = Some(Media::Image(image.into()));
        self
    }

    /// Sets the media area height (default 160).
    pub fn media_height(mut self, height: f32) -> Self {
        self.media_height = height.max(0.0);
        self
    }

    /// Sets the main content slot, padded 16 on each side.
    pub fn content<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.main_content = Some(Box::new(content));
        self
    }

    /// Sets the actions row, laid out right to left at the bottom.
    pub fn actions<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Ui) + 'a,
    {
        self.actions_content = Some(Box::new(content));
        self
    }

    /// Sets the minimum size of the card.
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size.max(Vec2::ZERO);
        self
    }

    /// Sets the card's width (default: the available width).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    /// Overrides the corner radius (default 12, `CornerMedium`).
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    /// Makes the card clickable: it takes focus, shows state layers, rises
    /// on hover and reports `clicked()`, also for Space and Enter.
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Enables or disables the card (default enabled). A disabled card and
    /// its content cannot be interacted with.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Makes the card selectable, showing `selected`. The caller toggles it
    /// on `clicked()`; see [`Self::checked`] for a card that toggles itself.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selection = Selection::Fixed(selected);
        self
    }

    /// Makes the card checkable: a click, Space or Enter toggles `checked`
    /// and marks the response changed.
    pub fn checked(mut self, checked: &'a mut bool) -> Self {
        self.selection = Selection::Toggle(checked);
        self
    }

    /// Sets the accessible label (default: the header title).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets an ID salt, for keeping a card's identity when cards move.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = Some(IdSalt::new(salt));
        self
    }

    /// Overrides the resting elevation, in dp; rounded to the nearest
    /// Material level (0, 1, 3, 6, 8, 12).
    pub fn elevation(mut self, elevation: f32) -> Self {
        let level = Elevation::ALL
            .into_iter()
            .min_by(|a, b| {
                (a.dp() - elevation)
                    .abs()
                    .total_cmp(&(b.dp() - elevation).abs())
            })
            .unwrap_or_default();
        self.elevation = Some(level);
        self
    }

    /// Kept for compatibility: Material 3 no longer tints elevated surfaces,
    /// so this does nothing.
    #[deprecated(note = "Material 3 cards are not tinted; this has no effect")]
    pub fn surface_tint_color(self, _color: Color32) -> Self {
        self
    }

    /// Sets the shadow colour (default: the `shadow` role).
    pub fn shadow_color(mut self, color: Color32) -> Self {
        self.shadow_color = Some(color);
        self
    }

    /// Sets the margin around the card (default 4).
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin.max(0.0);
        self
    }

    /// Sets whether content is clipped to the card's width (default true).
    pub fn clip_behavior(mut self, clip: bool) -> Self {
        self.clip_behavior = clip;
        self
    }

    /// Sets whether the outline is painted over the content (default true).
    pub fn border_on_foreground(mut self, on_foreground: bool) -> Self {
        self.border_on_foreground = on_foreground;
        self
    }
}

impl Default for MaterialCard2<'_> {
    fn default() -> Self {
        Self::elevated()
    }
}

/// Paints the check icon: a `color` disc with a check mark in `mark`.
fn paint_check_icon(painter: &egui::Painter, center: egui::Pos2, color: Color32, mark: Color32) {
    let r = CHECK_ICON_SIZE / 2.0;
    // The Material `check_circle` glyph: a 20 dp disc in a 24 dp box.
    painter.circle_filled(center, r * 20.0 / 24.0, color);
    let s = r / 12.0;
    let points = vec![
        center + egui::vec2(-5.0 * s, 0.0),
        center + egui::vec2(-1.5 * s, 3.5 * s),
        center + egui::vec2(5.5 * s, -3.5 * s),
    ];
    painter.add(Shape::line(points, Stroke::new(2.0 * s, mark)));
}

fn section<R>(ui: &mut Ui, margin: Margin, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::new()
        .inner_margin(margin)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add(ui)
        })
        .inner
}

fn margin(left: f32, right: f32, top: f32, bottom: f32) -> Margin {
    let m = |v: f32| v.round().clamp(0.0, 127.0) as i8;
    Margin {
        left: m(left),
        right: m(right),
        top: m(top),
        bottom: m(bottom),
    }
}

impl Widget for MaterialCard2<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let MaterialCard2 {
            variant,
            header_title,
            header_subtitle,
            media,
            main_content,
            actions_content,
            min_size,
            width,
            corner_radius,
            clickable,
            enabled,
            selection,
            label,
            id_salt,
            media_height,
            elevation,
            shadow_color,
            margin: outer_margin,
            clip_behavior,
            border_on_foreground,
        } = self;

        let theme = MaterialTheme::of(ui.ctx());
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let selectable = !matches!(selection, Selection::None);
        let interactive = clickable || selectable;
        let enabled = enabled && ui.is_enabled();
        let sense = a11y::sense(
            enabled,
            if interactive {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        let width = width
            .unwrap_or(ui.available_width() - 2.0 * outer_margin)
            .max(min_size.x)
            .max(0.0);
        // `*CardTokens.ContainerShape`: `CornerMedium`.
        let corner = corner_radius.unwrap_or_else(|| {
            CornerRadius::same(CornerSize::Medium.value().unwrap_or(12.0) as u8)
        });
        let content_color = theme.color(ColorRole::OnSurface);
        let has_header = header_title.is_some();
        let has_media = media.is_some();
        let has_after_media = main_content.is_some() || actions_content.is_some();

        let frame = egui::Frame::new().outer_margin(margin(
            outer_margin,
            outer_margin,
            outer_margin,
            outer_margin,
        ));
        let (response, shadow_idx, fill_idx, layer_idx) = frame
            .show(ui, |ui| {
                let shadow_idx = ui.painter().add(Shape::Noop);
                let fill_idx = ui.painter().add(Shape::Noop);
                let layer_idx = ui.painter().add(Shape::Noop);
                let mut builder = UiBuilder::new()
                    .sense(sense)
                    .layout(egui::Layout::top_down(egui::Align::Min));
                if let Some(salt) = id_salt {
                    builder = builder.id_salt(salt);
                }
                let inner = ui.scope_builder(builder, |ui| {
                    ui.set_min_size(Vec2::new(width, min_size.y));
                    ui.set_max_width(width);
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.visuals_mut().override_text_color = Some(content_color);
                    if clip_behavior {
                        let left = ui.max_rect().left();
                        let clip = ui.clip_rect();
                        ui.set_clip_rect(Rect::from_x_y_ranges(
                            left.max(clip.left())..=(left + width).min(clip.right()),
                            clip.y_range(),
                        ));
                    }
                    if !enabled {
                        // Compose: content at onSurface.copy(DisabledAlpha).
                        ui.visuals_mut().disabled_alpha = DISABLED_CONTENT_OPACITY;
                        ui.disable();
                    }
                    let ctx = ui.ctx().clone();

                    if let Some(title) = &header_title {
                        let end_pad = if selectable {
                            CHECK_ICON_SIZE + CHECK_ICON_MARGIN * 2.0
                        } else {
                            SECTION_PADDING
                        };
                        let (left, right) = if rtl {
                            (end_pad, SECTION_PADDING)
                        } else {
                            (SECTION_PADDING, end_pad)
                        };
                        let bottom = if has_media || main_content.is_none() {
                            SECTION_PADDING
                        } else {
                            0.0
                        };
                        section(ui, margin(left, right, SECTION_PADDING, bottom), |ui| {
                            let job = TypeRole::TitleLarge.style().layout_job(
                                &ctx,
                                title.clone(),
                                content_color,
                            );
                            ui.add(egui::Label::new(job).selectable(false).wrap());
                            if let Some(subtitle) = &header_subtitle {
                                let job = TypeRole::BodyMedium.style().layout_job(
                                    &ctx,
                                    subtitle.clone(),
                                    theme.color(ColorRole::OnSurfaceVariant),
                                );
                                ui.add(egui::Label::new(job).selectable(false).wrap());
                            }
                        });
                    }

                    if let Some(media) = media {
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::new(width, media_height), Sense::hover());
                        let top = if has_header { 0 } else { u8::MAX };
                        let bottom = if has_after_media { 0 } else { u8::MAX };
                        let media_corner = CornerRadius {
                            nw: corner.nw.min(top),
                            ne: corner.ne.min(top),
                            sw: corner.sw.min(bottom),
                            se: corner.se.min(bottom),
                        };
                        match media {
                            Media::Image(source) => {
                                egui::Image::new(source)
                                    .corner_radius(media_corner)
                                    .paint_at(ui, rect);
                            }
                            Media::Custom(draw) => {
                                ui.painter().rect_filled(
                                    rect,
                                    media_corner,
                                    theme.color(ColorRole::SurfaceVariant),
                                );
                                let mut child = ui.new_child(UiBuilder::new().max_rect(rect));
                                child.set_clip_rect(rect.intersect(ui.clip_rect()));
                                draw(&mut child);
                            }
                        }
                    }

                    if let Some(content) = main_content {
                        let top = if has_header && !has_media {
                            0.0
                        } else {
                            SECTION_PADDING
                        };
                        let bottom = if actions_content.is_some() {
                            0.0
                        } else {
                            SECTION_PADDING
                        };
                        section(
                            ui,
                            margin(SECTION_PADDING, SECTION_PADDING, top, bottom),
                            content,
                        );
                    }

                    if let Some(actions) = actions_content {
                        section(ui, Margin::same(ACTIONS_PADDING as i8), |ui| {
                            let layout = if rtl {
                                egui::Layout::left_to_right(egui::Align::Center)
                            } else {
                                egui::Layout::right_to_left(egui::Align::Center)
                            };
                            ui.spacing_mut().item_spacing.x = ACTIONS_PADDING;
                            ui.with_layout(layout, actions);
                        });
                    }
                });
                (inner.response, shadow_idx, fill_idx, layer_idx)
            })
            .inner;
        let mut response = response;
        let rect = response.rect;

        // Selection.
        let mut selected = match &selection {
            Selection::None => false,
            Selection::Fixed(selected) => *selected,
            Selection::Toggle(checked) => **checked,
        };
        if let Selection::Toggle(checked) = selection {
            if enabled && response.clicked() {
                *checked = !*checked;
                selected = *checked;
                response.mark_changed();
            }
        }

        // Accessibility: a checkbox when selectable, a button when clickable.
        let label = label
            .or_else(|| header_title.clone())
            .unwrap_or_else(|| "Card".to_owned());
        if selectable {
            a11y::toggle(&response, Role::CheckBox, enabled, &label, selected);
        } else if clickable {
            a11y::button(&response, enabled, &label);
        } else {
            a11y::describe(&response, Role::Group, enabled, &label, None);
        }

        if !ui.is_rect_visible(rect) {
            return response;
        }

        // Container, elevation and outline.
        let interaction = if interactive {
            InteractionState::from_response(&response)
        } else {
            InteractionState::default()
        };
        let level = variant.elevation(enabled, interactive, interaction);
        let resting = variant.elevation(enabled, false, InteractionState::default());
        let level = match elevation {
            Some(custom) if level == resting => custom,
            Some(_) | None => level,
        };
        let container_role = variant.container_role();
        let container = if enabled {
            theme.color(container_role)
        } else {
            let disabled_role = match variant {
                Card2Variant::Elevated | Card2Variant::Outlined => ColorRole::Surface,
                Card2Variant::Filled => ColorRole::SurfaceVariant,
            };
            match variant {
                Card2Variant::Outlined => theme.color(ColorRole::Surface),
                Card2Variant::Elevated => paint::state_layer_over(
                    theme.color(ColorRole::Surface),
                    theme.color(disabled_role),
                    DISABLED_CONTAINER_OPACITY,
                ),
                Card2Variant::Filled => paint::state_layer_over(
                    theme.color(container_role),
                    theme.color(disabled_role),
                    DISABLED_CONTAINER_OPACITY,
                ),
            }
        };
        let secondary = theme.color(ColorRole::Secondary);
        let outline = match variant {
            Card2Variant::Outlined => Some(if !enabled {
                paint::state_layer_over(
                    theme.color(ColorRole::SurfaceContainerLow),
                    theme.color(ColorRole::Outline),
                    DISABLED_OUTLINE_OPACITY,
                )
            } else if selected {
                secondary
            } else if interaction.focused {
                // `OutlinedCardTokens.FocusOutlineColor`.
                theme.color(ColorRole::OnSurface)
            } else {
                theme.color(ColorRole::OutlineVariant)
            }),
            Card2Variant::Elevated | Card2Variant::Filled => None,
        };
        let shadow = shadow_color.unwrap_or_else(|| theme.color(ColorRole::Shadow));

        let painter = ui.painter();
        let shadows: Vec<Shape> = level
            .shadows(shadow)
            .into_iter()
            .filter(|s| s.color != Color32::TRANSPARENT)
            .map(|s| Shape::from(s.as_shape(rect, corner)))
            .collect();
        painter.set(shadow_idx, Shape::Vec(shadows));

        let outline_shape = outline.map(|color| {
            Shape::from(egui::epaint::RectShape::stroke(
                rect,
                corner,
                Stroke::new(OUTLINE_WIDTH, color),
                StrokeKind::Inside,
            ))
        });
        let mut fill = vec![Shape::from(egui::epaint::RectShape::filled(
            rect, corner, container,
        ))];
        if !border_on_foreground {
            fill.extend(outline_shape.clone());
        }
        painter.set(fill_idx, Shape::Vec(fill));

        let mut layers = Vec::new();
        if selected {
            layers.push(Shape::from(egui::epaint::RectShape::filled(
                rect,
                corner,
                secondary.gamma_multiply(SELECTED_OVERLAY_OPACITY),
            )));
        }
        let opacity = interaction.opacity();
        if opacity > 0.0 {
            layers.push(Shape::from(egui::epaint::RectShape::filled(
                rect,
                corner,
                content_color.gamma_multiply(opacity),
            )));
        }
        painter.set(layer_idx, Shape::Vec(layers));

        if border_on_foreground {
            if let Some(shape) = outline_shape {
                painter.add(shape);
            }
        }
        if selected {
            let x = if rtl {
                rect.left() + CHECK_ICON_MARGIN + CHECK_ICON_SIZE / 2.0
            } else {
                rect.right() - CHECK_ICON_MARGIN - CHECK_ICON_SIZE / 2.0
            };
            let center = egui::pos2(x, rect.top() + CHECK_ICON_MARGIN + CHECK_ICON_SIZE / 2.0);
            let icon = if enabled {
                secondary
            } else {
                content_color.gamma_multiply(DISABLED_CONTENT_OPACITY)
            };
            paint_check_icon(painter, center, icon, container);
        }
        paint::paint_focus_ring_if_focused(painter, &response, corner);

        response
    }
}

/// An elevated card.
pub fn elevated_card2() -> MaterialCard2<'static> {
    MaterialCard2::elevated()
}

/// A filled card.
pub fn filled_card2() -> MaterialCard2<'static> {
    MaterialCard2::filled()
}

/// An outlined card.
pub fn outlined_card2() -> MaterialCard2<'static> {
    MaterialCard2::outlined()
}
