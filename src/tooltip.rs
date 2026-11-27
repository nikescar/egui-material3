//! Material Design 3 Tooltip Components
//!
//! Both tooltips are built on egui's own [`egui::Tooltip`], so they appear
//! after egui's hover delay (`Style::interaction::tooltip_delay`, 0.5 s by
//! default), hand over to a neighbouring widget without a second delay
//! within `tooltip_grace_time`, show one at a time, and are named in the
//! AccessKit tree.
//!
//! # Plain tooltip
//!
//! From Compose's `PlainTooltipTokens.kt`: container `InverseSurface`,
//! shape `CornerExtraSmall` (4), text `InverseOnSurface` in `BodySmall`,
//! padding 8 × 4 (`PaddingHorizontal` / `PaddingVertical`), minimum 40 × 24,
//! maximum width 320, a 4 dp gap to the anchor (`GapTrigger`), and no
//! shadow (`PlainTooltip`'s `shadowElevation` is 0).
//!
//! # Rich tooltip
//!
//! From Compose's `RichTooltipTokens.kt` and `Tooltip.kt`: container
//! `SurfaceContainer`, shape `CornerMedium` (12), elevation `Level2`,
//! subhead `OnSurfaceVariant` in `TitleSmall`, supporting text
//! `OnSurfaceVariant` in `BodyMedium`, actions `Primary` in `LabelLarge`,
//! 16 dp horizontal padding (`RichTooltipHorizontalPadding`), the same
//! 320 dp maximum width, and 36 dp action rows with 8 dp below
//! (`ActionLabelMinHeight`, `ActionLabelBottomPadding`). Rich tooltips are
//! persistent by default: they stay open while the pointer is on them.

use crate::paint::{paint_focus_ring_if_focused, paint_state_layer};
use crate::tokens::{ColorRole, CornerSize, Elevation, InteractionState, Shape, TypeRole};
use crate::MaterialTheme;
use egui::{
    epaint::CornerRadius, AreaState, Frame, Id, Label, Margin, Rect, RectAlign, Response, Sense,
    Tooltip, Ui, Vec2,
};

/// `PlainTooltipTokens.MaxWidth` and the rich tooltip's maximum width.
const MAX_WIDTH: f32 = 320.0;
/// `PlainTooltipTokens.MinWidth`.
const MIN_WIDTH: f32 = 40.0;
/// `PlainTooltipTokens.MinHeight`.
const MIN_HEIGHT: f32 = 24.0;
/// `PlainTooltipTokens.GapTrigger`: the gap between anchor and tooltip.
const GAP: f32 = 4.0;
/// `PlainTooltipTokens.PaddingHorizontal` / `PaddingVertical`.
const PLAIN_PADDING: Vec2 = Vec2::new(8.0, 4.0);
/// `Tooltip.kt` `RichTooltipHorizontalPadding`.
const RICH_HORIZONTAL_PADDING: f32 = 16.0;
/// Distance from a 14/20 text line's top to its baseline: turns Compose's
/// `paddingFromBaseline` values into top paddings.
const BASELINE_FROM_TOP: f32 = 16.0;
/// `Tooltip.kt` `HeightToSubheadFirstLine` (to the baseline).
const HEIGHT_TO_SUBHEAD_FIRST_LINE: f32 = 28.0;
/// `Tooltip.kt` `HeightFromSubheadToTextFirstLine` (to the baseline).
const HEIGHT_FROM_SUBHEAD_TO_TEXT_FIRST_LINE: f32 = 24.0;
/// `Tooltip.kt` `TextBottomPadding`.
const TEXT_BOTTOM_PADDING: f32 = 16.0;
/// `Tooltip.kt` `ActionLabelMinHeight`.
const ACTION_LABEL_MIN_HEIGHT: f32 = 36.0;
/// `Tooltip.kt` `ActionLabelBottomPadding`.
const ACTION_LABEL_BOTTOM_PADDING: f32 = 8.0;
/// A rich tooltip action's horizontal padding (`TextButton` content padding).
const ACTION_PADDING: f32 = 12.0;

/// Tooltip position relative to target element
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TooltipPosition {
    /// Above the target element
    Top,
    /// Below the target element
    Bottom,
    /// To the left of the target element
    Left,
    /// To the right of the target element
    Right,
    /// Automatically choose best position
    Auto,
}

impl TooltipPosition {
    /// The popup alignment, or `None` to let egui choose.
    fn align(self) -> Option<RectAlign> {
        match self {
            TooltipPosition::Top => Some(RectAlign::TOP),
            TooltipPosition::Bottom => Some(RectAlign::BOTTOM),
            TooltipPosition::Left => Some(RectAlign::LEFT),
            TooltipPosition::Right => Some(RectAlign::RIGHT),
            TooltipPosition::Auto => None,
        }
    }
}

/// Places `tooltip` on the requested side, if any (egui still flips it when
/// there is no room).
fn place(mut tooltip: Tooltip<'_>, position: TooltipPosition) -> Tooltip<'_> {
    if let Some(align) = position.align() {
        tooltip.popup = tooltip.popup.align(align);
    }
    tooltip
}

/// Whether the pointer is over the tooltip `response`'s widget showed last
/// frame.
fn pointer_over_tooltip(response: &Response) -> bool {
    let ctx = &response.ctx;
    if !Tooltip::was_tooltip_open_last_frame(ctx, response.id) {
        return false;
    }
    let Some(area) = AreaState::load(ctx, Tooltip::tooltip_id(response.id, 0)) else {
        return false;
    };
    ctx.input(|i| i.pointer.hover_pos())
        .is_some_and(|pos| area.rect().contains(pos))
}

/// Clamps a padding in points to egui's `i8` margins.
fn margin_i8(v: f32) -> i8 {
    v.round().clamp(0.0, f32::from(i8::MAX)) as i8
}

/// Material Design plain tooltip.
///
/// A short label for a widget, shown on hover after egui's tooltip delay.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialButton, show_tooltip_on_hover, TooltipPosition};
///
/// let button_response = ui.add(MaterialButton::filled("Hover me"));
/// show_tooltip_on_hover(ui, &button_response, "This is a tooltip", TooltipPosition::Top);
/// # });
/// ```
pub struct MaterialTooltip {
    text: String,
    position: TooltipPosition,
    max_width: f32,
    padding: Vec2,
    font_size: Option<f32>,
}

impl MaterialTooltip {
    /// Create a new tooltip
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: TooltipPosition::Auto,
            max_width: MAX_WIDTH,
            padding: PLAIN_PADDING,
            font_size: None,
        }
    }

    /// Set the tooltip position
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the maximum width (default 320)
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Set the padding (default 8 × 4)
    pub fn padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    /// Set the font size (default: `BodySmall`, 12)
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Show the tooltip for `response` when it is hovered, after egui's
    /// tooltip delay. Returns whether it is showing.
    pub fn show_for(&self, response: &Response) -> bool {
        let open = Tooltip::should_show_tooltip(response, false);
        let mut tooltip = Tooltip::for_widget(response);
        tooltip.popup = tooltip.popup.open(open);
        self.show_tooltip(tooltip)
    }

    /// Show the tooltip at `target_rect` now, with no hover check or delay.
    pub fn show(&self, ui: &mut Ui, target_rect: Rect) {
        let id = ui.next_auto_id().with("material_tooltip");
        let tooltip = Tooltip::always_open(ui.ctx().clone(), ui.layer_id(), id, target_rect);
        // Always open: whether it showed only depends on the anchor existing.
        self.show_tooltip(tooltip);
    }

    fn show_tooltip(&self, tooltip: Tooltip<'_>) -> bool {
        let ctx = tooltip.popup.ctx().clone();
        let theme = MaterialTheme::of(&ctx);
        let text_color = theme.color(ColorRole::InverseOnSurface);
        let frame = Frame {
            fill: theme.color(ColorRole::InverseSurface),
            corner_radius: CornerRadius::same(
                CornerSize::ExtraSmall
                    .value()
                    .map_or(0, |r| r.round().clamp(0.0, 255.0) as u8),
            ),
            inner_margin: Margin::symmetric(margin_i8(self.padding.x), margin_i8(self.padding.y)),
            ..Frame::NONE
        };
        let mut tooltip = place(tooltip, self.position)
            .gap(GAP)
            .width(self.max_width)
            .accessible_name(self.text.clone());
        tooltip.popup = tooltip.popup.frame(frame);
        let content_max = (self.max_width - 2.0 * self.padding.x).max(1.0);
        tooltip
            .show(|ui| {
                ui.set_max_width(content_max);
                ui.set_min_size(Vec2::new(
                    (MIN_WIDTH - 2.0 * self.padding.x).max(0.0),
                    (MIN_HEIGHT - 2.0 * self.padding.y).max(0.0),
                ));
                let mut job =
                    TypeRole::BodySmall
                        .style()
                        .layout_job(ui.ctx(), self.text.clone(), text_color);
                if let Some(size) = self.font_size {
                    if let Some(section) = job.sections.first_mut() {
                        section.format.font_id.size = size;
                        section.format.line_height = None;
                    }
                }
                ui.add(Label::new(job).wrap());
            })
            .is_some()
    }
}

/// What a rich tooltip did this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RichTooltipResponse {
    /// Whether it is showing.
    pub shown: bool,
    /// The index of the action clicked this frame, if any.
    pub action_clicked: Option<usize>,
}

/// Material Design rich tooltip: an optional subhead, supporting text and
/// optional actions, on a raised surface.
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::RichTooltip;
///
/// let response = ui.button("Storage");
/// let tip = RichTooltip::new("Free up space by deleting old backups.")
///     .subhead("Storage almost full")
///     .action("Manage")
///     .show_for(&response);
/// if tip.action_clicked == Some(0) {
///     // open the storage settings
/// }
/// # });
/// ```
pub struct RichTooltip {
    subhead: Option<String>,
    text: String,
    actions: Vec<String>,
    position: TooltipPosition,
    max_width: f32,
    persistent: bool,
}

impl RichTooltip {
    /// A rich tooltip with supporting `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            subhead: None,
            text: text.into(),
            actions: Vec::new(),
            position: TooltipPosition::Auto,
            max_width: MAX_WIDTH,
            persistent: true,
        }
    }

    /// Add a subhead above the text.
    pub fn subhead(mut self, subhead: impl Into<String>) -> Self {
        self.subhead = Some(subhead.into());
        self
    }

    /// Add an action button. Actions are reported by index, in the order
    /// they were added.
    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.actions.push(label.into());
        self
    }

    /// Set the position.
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the maximum width (default 320).
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Whether it stays open while the pointer is over it (default true).
    /// A tooltip with actions always does, so they can be clicked.
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }

    /// Show the tooltip for `response` when it is hovered, after egui's
    /// tooltip delay.
    pub fn show_for(&self, response: &Response) -> RichTooltipResponse {
        let open = Tooltip::should_show_tooltip(response, true)
            || (self.persistent && pointer_over_tooltip(response));
        let mut tooltip = Tooltip::for_widget(response);
        tooltip.popup = tooltip.popup.open(open);
        self.show_tooltip(tooltip, response.id)
    }

    /// Show the tooltip at `target_rect` now, with no hover check or delay.
    pub fn show(&self, ui: &mut Ui, target_rect: Rect) -> RichTooltipResponse {
        let id = ui.next_auto_id().with("material_rich_tooltip");
        let tooltip = Tooltip::always_open(ui.ctx().clone(), ui.layer_id(), id, target_rect);
        self.show_tooltip(tooltip, id)
    }

    fn show_tooltip(&self, tooltip: Tooltip<'_>, id: Id) -> RichTooltipResponse {
        let mut tooltip = place(tooltip, self.position)
            .gap(GAP)
            .width(self.max_width)
            .accessible_name(match &self.subhead {
                Some(subhead) => format!("{subhead}: {}", self.text),
                None => self.text.clone(),
            });
        tooltip.popup = tooltip.popup.frame(Frame::NONE);
        match tooltip.show(|ui| self.content(ui, id)) {
            Some(inner) => RichTooltipResponse {
                shown: true,
                action_clicked: inner.inner,
            },
            None => RichTooltipResponse::default(),
        }
    }

    /// Paints one action button and returns whether it was clicked.
    fn action_button(ui: &mut Ui, id: Id, label: &str, color: egui::Color32) -> bool {
        let galley = ui
            .painter()
            .layout_job(
                TypeRole::LabelLarge
                    .style()
                    .layout_job(ui.ctx(), label.to_owned(), color),
            );
        let size = Vec2::new(
            galley.size().x + 2.0 * ACTION_PADDING,
            ACTION_LABEL_MIN_HEIGHT,
        );
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        let response = ui.interact(rect, id, Sense::click());
        crate::a11y::button(&response, true, label);
        let radius = CornerRadius::same((ACTION_LABEL_MIN_HEIGHT / 2.0) as u8);
        paint_state_layer(
            ui.painter(),
            rect,
            radius,
            color,
            InteractionState::from_response(&response),
        );
        paint_focus_ring_if_focused(ui.painter(), &response, radius);
        ui.painter()
            .galley(rect.center() - galley.size() / 2.0, galley, color);
        response.clicked()
    }

    fn content(&self, ui: &mut Ui, id: Id) -> Option<usize> {
        let ctx = ui.ctx().clone();
        let theme = MaterialTheme::of(&ctx);
        let content_color = theme.color(ColorRole::OnSurfaceVariant);
        let action_color = theme.color(ColorRole::Primary);
        // The container goes under the content, sized once the content is laid out.
        let container = ui.painter().add(egui::Shape::Noop);
        let has_extras = self.subhead.is_some() || !self.actions.is_empty();
        let text_max = (self.max_width - 2.0 * RICH_HORIZONTAL_PADDING).max(1.0);

        let inner = Frame::NONE
            .inner_margin(Margin::symmetric(margin_i8(RICH_HORIZONTAL_PADDING), 0))
            .show(ui, |ui| {
                ui.set_max_width(text_max);
                ui.set_min_width((MIN_WIDTH - 2.0 * RICH_HORIZONTAL_PADDING).max(0.0));
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                let wrapped = |role: TypeRole, text: &str| {
                    let mut job = role
                        .style()
                        .layout_job(&ctx, text.to_owned(), content_color);
                    job.wrap.max_width = text_max;
                    Label::new(job).wrap()
                };
                if let Some(subhead) = &self.subhead {
                    ui.add_space(HEIGHT_TO_SUBHEAD_FIRST_LINE - BASELINE_FROM_TOP);
                    ui.add(wrapped(TypeRole::TitleSmall, subhead));
                }
                let (top, bottom) = if has_extras {
                    (
                        HEIGHT_FROM_SUBHEAD_TO_TEXT_FIRST_LINE - BASELINE_FROM_TOP,
                        TEXT_BOTTOM_PADDING,
                    )
                } else {
                    (PLAIN_PADDING.y, PLAIN_PADDING.y)
                };
                ui.add_space(top);
                ui.add(wrapped(TypeRole::BodyMedium, &self.text));
                ui.add_space(bottom);
                let mut clicked = None;
                if !self.actions.is_empty() {
                    ui.horizontal(|ui| {
                        for (index, label) in self.actions.iter().enumerate() {
                            let action_id = id.with(("rich_tooltip_action", index));
                            if Self::action_button(ui, action_id, label, action_color) {
                                clicked = Some(index);
                            }
                        }
                    });
                    ui.add_space(ACTION_LABEL_BOTTOM_PADDING);
                }
                clicked
            });

        let rect = inner.response.rect;
        let corner_radius = Shape::MEDIUM.corner_radius(rect, theme.layout_direction);
        let mut shapes: Vec<egui::Shape> = Elevation::Level2
            .shadows(theme.color(ColorRole::Shadow))
            .into_iter()
            .filter(|shadow| shadow.color != egui::Color32::TRANSPARENT)
            .map(|shadow| shadow.as_shape(rect, corner_radius).into())
            .collect();
        shapes.push(egui::Shape::rect_filled(
            rect,
            corner_radius,
            theme.color(ColorRole::SurfaceContainer),
        ));
        ui.painter().set(container, egui::Shape::Vec(shapes));
        inner.inner
    }
}

/// Convenience function to show a tooltip on hover
///
/// This function shows a tooltip when the user hovers over the target
/// element, after egui's tooltip delay.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialButton, show_tooltip_on_hover, TooltipPosition};
///
/// let response = ui.add(MaterialButton::filled("Hover me"));
/// show_tooltip_on_hover(ui, &response, "Tooltip text", TooltipPosition::Top);
/// # });
/// ```
pub fn show_tooltip_on_hover(
    _ui: &mut Ui,
    target_response: &Response,
    text: impl Into<String>,
    position: TooltipPosition,
) {
    // Whether it is showing is of no use to a fire-and-forget helper.
    MaterialTooltip::new(text)
        .position(position)
        .show_for(target_response);
}

/// Convenience function to show a tooltip with custom styling on hover
pub fn show_tooltip_on_hover_custom(
    _ui: &mut Ui,
    target_response: &Response,
    tooltip: MaterialTooltip,
) {
    // Whether it is showing is of no use to a fire-and-forget helper.
    tooltip.show_for(target_response);
}

/// Add a tooltip to any widget (builder pattern style)
///
/// The tooltip shows when the pointer is over anything `add_contents` adds.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{MaterialButton, with_tooltip, TooltipPosition};
///
/// let response = with_tooltip(ui, "Tooltip text", TooltipPosition::Top, |ui| {
///     ui.add(MaterialButton::filled("Button with tooltip"))
/// });
/// # });
/// ```
pub fn with_tooltip<R>(
    ui: &mut Ui,
    text: impl Into<String>,
    position: TooltipPosition,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let inner = ui.scope(add_contents);
    // Whether it is showing is of no use here; the caller gets its own result.
    MaterialTooltip::new(text)
        .position(position)
        .show_for(&inner.response);
    inner.inner
}

/// Create a tooltip component
pub fn tooltip(text: impl Into<String>) -> MaterialTooltip {
    MaterialTooltip::new(text)
}
