//! Material Design 3 Snackbar Components
//!
//! Two ways to show a snackbar:
//!
//! - [`SnackbarHost`]: a queue kept in the egui context. Call
//!   [`SnackbarHost::enqueue`] from anywhere and [`SnackbarHost::show`] once
//!   per frame; it shows one snackbar at a time, times it out, animates it in
//!   and out, and reports how each one ended. This mirrors Compose's
//!   `SnackbarHostState` and `SnackbarHost`.
//! - [`MaterialSnackbar`]: a single snackbar widget for code that keeps the
//!   "is it showing" flag itself. Bind the flag with
//!   [`MaterialSnackbar::show_if`] and the widget writes `false` back when it
//!   times out or its action or dismiss icon is clicked.
//!
//! Both draw in a foreground layer anchored to the bottom (or top) of the
//! screen, so a snackbar is never clipped by the panel that adds it.
//!
//! # Tokens
//!
//! From Compose's `SnackbarTokens.kt` (baseline set):
//!
//! - container `InverseSurface`, shape `CornerExtraSmall` (4), elevation
//!   `Level3`;
//! - supporting text `InverseOnSurface` in `BodyMedium`;
//! - action label `InversePrimary` in `LabelLarge`, with the M3 state layer;
//! - dismiss icon `InverseOnSurface`, 24 dp;
//! - height 48 (`ContainerHeight`) for one line, 68 (`LargeContainerHeight`)
//!   for two.
//!
//! Layout constants come from Compose's `Snackbar.kt` (`ContainerMaxWidth`
//! 600, `HorizontalSpacing` 16, `TextEndExtraSpacing` 8,
//! `SnackbarVerticalPadding` 14, `SeparateButtonExtraY` 2,
//! `ActionButtonBottomPadding` 4) and the host's 12 dp padding around the
//! snackbar. Durations come from `SnackbarHost.kt` (`Short` 4 s, `Long`
//! 10 s, `Indefinite`); motion is the host's fade (`FastEffects` spring)
//! and 0.8 to 1 scale (`FastSpatial` spring).
//!
//! The snackbar is a `Status` node with a polite live region in the
//! AccessKit tree, so screen readers announce it without moving focus.

use crate::paint::{paint_elevation, paint_focus_ring_if_focused, paint_state_layer};
use crate::tokens::{
    animate_value_with_spring, ColorRole, Elevation, InteractionState, LayoutDirection, Shape,
    SpringSpeed, TypeRole,
};
use crate::MaterialTheme;
use egui::{
    accesskit,
    ecolor::Color32,
    emath::TSTransform,
    epaint::{CornerRadius, Galley, Shadow, Stroke},
    Align2, Area, Context, Id, Order, Rect, Response, Role, Sense, Ui, Vec2, Widget,
};
use std::collections::VecDeque;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

/// `Snackbar.kt` `ContainerMaxWidth`.
const CONTAINER_MAX_WIDTH: f32 = 600.0;
/// `SnackbarTokens.ContainerHeight`: one line of text.
const CONTAINER_HEIGHT: f32 = 48.0;
/// `SnackbarTokens.LargeContainerHeight`: two lines of text, or an action
/// on its own line.
const LARGE_CONTAINER_HEIGHT: f32 = 68.0;
/// `Snackbar.kt` `HorizontalSpacing`: start padding.
const HORIZONTAL_SPACING: f32 = 16.0;
/// `Snackbar.kt` `TextEndExtraSpacing`: end padding without a dismiss icon.
const TEXT_END_EXTRA_SPACING: f32 = 8.0;
/// `Snackbar.kt` `SnackbarVerticalPadding`: above and below the text.
const VERTICAL_PADDING: f32 = 14.0;
/// `Snackbar.kt` `SeparateButtonExtraY`: text to an action on its own line.
const SEPARATE_BUTTON_EXTRA_Y: f32 = 2.0;
/// `Snackbar.kt` `ActionButtonBottomPadding`: below an action on its own line.
const ACTION_BUTTON_BOTTOM_PADDING: f32 = 4.0;
/// `SnackbarHost`'s default `Snackbar(data, Modifier.padding(12.dp))`: the
/// gap between a floating snackbar and the screen edges.
const HOST_PADDING: f32 = 12.0;
/// `SnackbarTokens.IconSize`.
const ICON_SIZE: f32 = 24.0;
/// The dismiss icon button's touch target (`IconButton`'s 48 dp minimum).
const ICON_BUTTON_SIZE: f32 = 48.0;
/// The dismiss icon button's state layer (`IconButtonTokens.StateLayerSize`).
const ICON_BUTTON_STATE_LAYER: f32 = 40.0;
/// The action `TextButton`'s height (`ButtonDefaults.MinHeight`).
const ACTION_HEIGHT: f32 = 40.0;
/// The action `TextButton`'s minimum width (`ButtonDefaults.MinWidth`).
const ACTION_MIN_WIDTH: f32 = 58.0;
/// The action `TextButton`'s horizontal content padding
/// (`ButtonDefaults.TextButtonContentPadding`).
const ACTION_PADDING: f32 = 12.0;
/// The scale a snackbar grows from as it appears (`SnackbarHost.kt`
/// `animatedScale`).
const ENTER_SCALE: f32 = 0.8;

/// Defines where a SnackBar should appear and how its location should be adjusted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnackBarBehavior {
    /// Edge to edge at the bottom (or top) of the screen, square corners
    /// unless [`MaterialSnackbar::corner_radius`] is set.
    Fixed,
    /// Floats 12 dp from the screen edges, up to 600 dp wide, with 4 dp
    /// corners: the Material 3 snackbar, and the default.
    Floating,
}

/// Where a snackbar appears on screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SnackbarPosition {
    /// At the bottom of the screen.
    Bottom,
    /// At the top of the screen.
    Top,
}

/// How long a snackbar stays up (`SnackbarHost.kt` `SnackbarDuration`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SnackbarDuration {
    /// 4 seconds.
    #[default]
    Short,
    /// 10 seconds.
    Long,
    /// Until the action or dismiss icon is clicked, or it is dismissed in code.
    Indefinite,
}

impl SnackbarDuration {
    /// The time before it dismisses itself, or `None` for
    /// [`SnackbarDuration::Indefinite`].
    pub const fn timeout(self) -> Option<Duration> {
        match self {
            SnackbarDuration::Short => Some(Duration::from_millis(4000)),
            SnackbarDuration::Long => Some(Duration::from_millis(10000)),
            SnackbarDuration::Indefinite => None,
        }
    }
}

/// Material Design snackbar component.
///
/// Snackbars provide brief messages about app processes at the bottom of the screen.
/// They inform users of a process that an app has performed or will perform.
///
/// Bind the visibility flag with [`MaterialSnackbar::show_if`]: the widget
/// sets it to `false` when the snackbar times out or its action or dismiss
/// icon is clicked, and marks the response changed on that frame.
///
/// ```
/// # use egui_material3::*;
/// # egui::__run_test_ui(|ui| {
/// let mut snackbar_visible = true;
/// let snackbar = MaterialSnackbar::new("File deleted successfully")
///     .action("Undo", || println!("Undo clicked!"))
///     .show_if(&mut snackbar_visible);
///
/// ui.add(snackbar);
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialSnackbar<'a> {
    message: String,
    action_text: Option<String>,
    action_callback: Option<Box<dyn Fn() + Send + Sync + 'a>>,
    visible: bool,
    binding: Option<&'a mut bool>,
    auto_dismiss: Option<Duration>,
    position: SnackbarPosition,
    corner_radius: Option<CornerRadius>,
    elevation: Option<Shadow>,
    behavior: SnackBarBehavior,
    width: Option<f32>,
    margin: Option<Vec2>,
    show_close_icon: bool,
    close_icon_color: Option<Color32>,
    leading_icon: Option<String>,
    action_overflow_threshold: f32,
    on_visible: Option<Box<dyn Fn() + Send + Sync + 'a>>,
    id_salt: Option<Id>,
}

impl<'a> MaterialSnackbar<'a> {
    /// Create a new snackbar with a message.
    ///
    /// It dismisses itself after [`SnackbarDuration::Short`] (4 s); change
    /// that with [`Self::duration`] or [`Self::auto_dismiss`].
    ///
    /// # Example
    /// ```rust
    /// # use egui_material3::*;
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("File saved successfully");
    /// # });
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            action_text: None,
            action_callback: None,
            visible: true,
            binding: None,
            auto_dismiss: SnackbarDuration::Short.timeout(),
            position: SnackbarPosition::Bottom,
            corner_radius: None,
            elevation: None,
            behavior: SnackBarBehavior::Floating,
            width: None,
            margin: None,
            show_close_icon: false,
            close_icon_color: None,
            leading_icon: None,
            action_overflow_threshold: 0.25,
            on_visible: None,
            id_salt: None,
        }
    }

    /// Add an action button to the snackbar. Clicking it calls `callback`
    /// and dismisses the snackbar.
    ///
    /// # Example
    /// ```rust
    /// # use egui_material3::*;
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("File deleted")
    ///     .action("Undo", || println!("Undo action performed"));
    /// # });
    /// ```
    pub fn action<F>(mut self, text: impl Into<String>, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.action_text = Some(text.into());
        self.action_callback = Some(Box::new(callback));
        self
    }

    /// Set auto-dismiss duration. Set to None to disable auto-dismiss.
    ///
    /// # Example
    /// ```rust
    /// # use egui_material3::*;
    /// use std::time::Duration;
    /// # egui::__run_test_ui(|ui| {
    /// // Auto-dismiss after 6 seconds
    /// let snackbar = MaterialSnackbar::new("Custom timeout")
    ///     .auto_dismiss(Some(Duration::from_secs(6)));
    ///
    /// // Never auto-dismiss
    /// let persistent = MaterialSnackbar::new("Persistent message")
    ///     .auto_dismiss(None);
    /// # });
    /// ```
    pub fn auto_dismiss(mut self, duration: Option<Duration>) -> Self {
        self.auto_dismiss = duration;
        self
    }

    /// Set the auto-dismiss time to one of the Material durations.
    pub fn duration(mut self, duration: SnackbarDuration) -> Self {
        self.auto_dismiss = duration.timeout();
        self
    }

    /// Set the position of the snackbar.
    ///
    /// # Example
    /// ```rust
    /// # use egui_material3::*;
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("Top notification")
    ///     .position(SnackbarPosition::Top);
    /// # });
    /// ```
    pub fn position(mut self, position: SnackbarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set corner radius for rounded corners (default: `CornerExtraSmall`,
    /// 4, when floating; square when fixed).
    ///
    /// # Example
    /// ```rust
    /// # use egui_material3::*;
    /// # egui::__run_test_ui(|ui| {
    /// let snackbar = MaterialSnackbar::new("Rounded snackbar")
    ///     .corner_radius(8.0);
    /// # });
    /// ```
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    /// Replace the `Level3` elevation shadow with a custom one.
    ///
    /// # Example
    /// ```rust
    /// # use egui_material3::*;
    /// # egui::__run_test_ui(|ui| {
    /// use egui::epaint::Shadow;
    /// let shadow = Shadow {
    ///     offset: [0, 2],
    ///     blur: 8,
    ///     spread: 0,
    ///     color: egui::Color32::from_black_alpha(60),
    /// };
    /// let snackbar = MaterialSnackbar::new("Elevated snackbar")
    ///     .elevation(shadow);
    /// # });
    /// ```
    pub fn elevation(mut self, elevation: impl Into<Shadow>) -> Self {
        self.elevation = Some(elevation.into());
        self
    }

    /// Set the behavior of the snackbar (Fixed or Floating).
    pub fn behavior(mut self, behavior: SnackBarBehavior) -> Self {
        self.behavior = behavior;
        self
    }

    /// Set a fixed width. It is limited to the screen width (less the
    /// margins when floating).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the gap to the screen edges for a floating snackbar
    /// (horizontal, vertical; default 12 dp both ways).
    pub fn margin(mut self, margin: Vec2) -> Self {
        self.margin = Some(margin);
        self
    }

    /// Show a dismiss icon button at the end.
    pub fn show_close_icon(mut self, show: bool) -> Self {
        self.show_close_icon = show;
        self
    }

    /// Set the color of the dismiss icon (default `InverseOnSurface`).
    pub fn close_icon_color(mut self, color: Color32) -> Self {
        self.close_icon_color = Some(color);
        self
    }

    /// Add a leading icon (a glyph, such as a Material Symbol or emoji)
    /// before the message.
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set the action overflow threshold.
    /// When the action and dismiss icon take more than this fraction of the
    /// snackbar's width, the action moves to a line of its own.
    ///
    /// # Arguments
    /// * `threshold` - Value between 0.0 and 1.0 (default: 0.25)
    pub fn action_overflow_threshold(mut self, threshold: f32) -> Self {
        self.action_overflow_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set a callback to be called when the snackbar first becomes visible.
    pub fn on_visible<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'a,
    {
        self.on_visible = Some(Box::new(callback));
        self
    }

    /// Key the snackbar's timer and animation by `salt` instead of by its
    /// message, so a snackbar whose text changes keeps its state.
    pub fn id_salt(mut self, salt: impl Hash + std::fmt::Debug) -> Self {
        self.id_salt = Some(Id::unique(salt));
        self
    }

    /// Show the snackbar while `*visible` is true, and set it to `false`
    /// when the snackbar is dismissed (timed out, action or dismiss icon
    /// clicked).
    ///
    /// # Example
    /// ```rust
    /// # use egui_material3::*;
    /// # egui::__run_test_ui(|ui| {
    /// let mut show_notification = true;
    /// let snackbar = MaterialSnackbar::new("Conditional message")
    ///     .show_if(&mut show_notification);
    /// ui.add(snackbar);
    /// # });
    /// ```
    pub fn show_if(mut self, visible: &'a mut bool) -> Self {
        self.visible = *visible;
        self.binding = Some(visible);
        self
    }

    /// Like [`Self::show_if`], moved `vertical_offset` points away from the
    /// screen edge, for stacking several snackbars.
    pub fn show_with_offset(
        self,
        visible: &'a mut bool,
        vertical_offset: f32,
    ) -> MaterialSnackbarWithOffset<'a> {
        MaterialSnackbarWithOffset {
            snackbar: self.show_if(visible),
            vertical_offset,
        }
    }

    /// Show the snackbar.
    pub fn show(mut self) -> Self {
        self.visible = true;
        self
    }

    /// Hide the snackbar.
    pub fn hide(mut self) -> Self {
        self.visible = false;
        self
    }

    /// The memory key holding when this snackbar was first shown.
    fn state_id(&self, ui: &Ui) -> Id {
        match self.id_salt {
            Some(salt) => ui.scope_id().with(("material_snackbar", salt)),
            None => ui
                .scope_id()
                .with(("material_snackbar", self.message.as_str())),
        }
    }

    fn is_visible(&self) -> bool {
        self.binding.as_deref().copied().unwrap_or(self.visible)
    }

    /// Tracks when the snackbar was first shown in egui memory (the widget
    /// itself is rebuilt every frame). Fires `on_visible` once, on the first
    /// visible frame.
    fn phase(&self, ctx: &Context, id: Id) -> Phase {
        if !self.is_visible() {
            ctx.data_mut(|d| d.remove::<TimerState>(id));
            return Phase::Hidden;
        }
        let now = ctx.input(|i| i.time);
        let pass = ctx.cumulative_pass_nr();
        let (state, first) = ctx.data_mut(|d| {
            // A snackbar that was not added last pass is a new one: the
            // caller stopped showing it, so its old timer no longer applies.
            let previous = d
                .get_temp::<TimerState>(id)
                .filter(|s| s.last_pass.saturating_add(1) >= pass);
            let (state, first) = match previous {
                Some(state) => (state, false),
                None => (
                    TimerState {
                        shown_at: now,
                        dismissed: false,
                        last_pass: pass,
                    },
                    true,
                ),
            };
            let state = TimerState {
                last_pass: pass,
                ..state
            };
            d.insert_temp(id, state);
            (state, first)
        });
        if state.dismissed {
            return Phase::Dismissed;
        }
        if first {
            if let Some(on_visible) = &self.on_visible {
                on_visible();
            }
        }
        if let Some(auto_dismiss) = self.auto_dismiss {
            let remaining = auto_dismiss.as_secs_f64() - (now - state.shown_at);
            if remaining <= 0.0 {
                return Phase::TimedOut;
            }
            ctx.request_repaint_after(Duration::from_secs_f64(remaining));
        }
        Phase::Showing { first }
    }

    /// Marks the snackbar dismissed and writes `false` to the bound flag.
    fn dismiss(&mut self, ctx: &Context, id: Id) {
        ctx.data_mut(|d| {
            let previous = d.get_temp::<TimerState>(id);
            d.insert_temp(
                id,
                TimerState {
                    shown_at: previous.map_or(0.0, |s| s.shown_at),
                    dismissed: true,
                    last_pass: previous.map_or(0, |s| s.last_pass),
                },
            );
        });
        if let Some(binding) = self.binding.as_deref_mut() {
            *binding = false;
        }
        self.visible = false;
    }

    /// Shows the snackbar for one frame: the timer, the overlay, and what a
    /// click on the action or dismiss icon does.
    fn show_in(mut self, ui: &mut Ui, vertical_offset: f32) -> Response {
        let ctx = ui.ctx().clone();
        let id = self.state_id(ui);
        match self.phase(&ctx, id) {
            Phase::Hidden | Phase::Dismissed => ui.allocate_response(Vec2::ZERO, Sense::hover()),
            Phase::TimedOut => {
                self.dismiss(&ctx, id);
                let mut response = ui.allocate_response(Vec2::ZERO, Sense::hover());
                response.mark_changed();
                response
            }
            Phase::Showing { first } => {
                let (alpha, scale) = motion(&ctx, id, first, true);
                let outcome = self.show_overlay(&ctx, id, vertical_offset, alpha, scale, true);
                let mut response = outcome.response;
                if outcome.action_clicked {
                    if let Some(callback) = &self.action_callback {
                        callback();
                    }
                }
                if outcome.action_clicked || outcome.close_clicked {
                    self.dismiss(&ctx, id);
                    response.mark_changed();
                }
                response
            }
        }
    }

    /// The snackbar's width and its gap to the screen edges (x, y).
    fn resolve_width(&self, screen: Rect) -> (f32, Vec2) {
        match self.behavior {
            SnackBarBehavior::Floating => {
                let margin = self.margin.unwrap_or(Vec2::splat(HOST_PADDING));
                let available = (screen.width() - 2.0 * margin.x).max(0.0);
                let width = self
                    .width
                    .unwrap_or(CONTAINER_MAX_WIDTH)
                    .min(available)
                    .max(0.0);
                (width, margin)
            }
            SnackBarBehavior::Fixed => {
                let width = self
                    .width
                    .map_or(screen.width(), |w| w.min(screen.width()))
                    .max(0.0);
                (width, Vec2::ZERO)
            }
        }
    }

    fn resolve_corner_radius(&self, rect: Rect, direction: LayoutDirection) -> CornerRadius {
        match (self.corner_radius, self.behavior) {
            (Some(radius), SnackBarBehavior::Fixed | SnackBarBehavior::Floating) => radius,
            (None, SnackBarBehavior::Floating) => Shape::EXTRA_SMALL.corner_radius(rect, direction),
            (None, SnackBarBehavior::Fixed) => CornerRadius::ZERO,
        }
    }

    /// Lays out the content for a snackbar `width` points wide, left to
    /// right; [`SnackbarLayout::mirrored`] flips it for right-to-left.
    fn layout(&self, ui: &Ui, theme: &MaterialTheme, width: f32) -> SnackbarLayout {
        let ctx = ui.ctx();
        let painter = ui.painter();
        let text_color = theme.color(ColorRole::InverseOnSurface);
        let action_color = theme.color(ColorRole::InversePrimary);

        let close_width = if self.show_close_icon {
            ICON_BUTTON_SIZE
        } else {
            0.0
        };
        let end_padding = if self.show_close_icon {
            0.0
        } else {
            TEXT_END_EXTRA_SPACING
        };
        let inner_width = width - HORIZONTAL_SPACING - end_padding;

        let icon = self.leading_icon.as_ref().map(|icon| {
            painter.layout_no_wrap(
                icon.clone(),
                egui::FontId::proportional(ICON_SIZE),
                text_color,
            )
        });
        let icon_width = icon
            .as_ref()
            .map_or(0.0, |g| g.size().x + HORIZONTAL_SPACING);

        let action = self.action_text.as_ref().map(|label| {
            painter.layout_job(TypeRole::LabelLarge.style().layout_job(
                ctx,
                label.clone(),
                action_color,
            ))
        });
        let action_width = action.as_ref().map_or(0.0, |g| {
            (g.size().x + 2.0 * ACTION_PADDING).max(ACTION_MIN_WIDTH)
        });

        let new_line = action.is_some()
            && width > 0.0
            && (action_width + close_width) / width > self.action_overflow_threshold;

        let text_max_width = if new_line {
            inner_width - icon_width - TEXT_END_EXTRA_SPACING
        } else {
            inner_width - icon_width - action_width - close_width - end_padding
        }
        .max(1.0);
        let mut job =
            TypeRole::BodyMedium
                .style()
                .layout_job(ctx, self.message.clone(), text_color);
        job.wrap.max_width = text_max_width;
        let text = painter.layout_job(job);
        let text_height = text.size().y;
        let close_x = width - close_width;
        let action_x = close_x - action_width - if new_line { end_padding } else { 0.0 };

        let (height, text_y, action_y, close_y) = if new_line {
            let action_y = VERTICAL_PADDING + text_height + SEPARATE_BUTTON_EXTRA_Y;
            let height = (action_y + ACTION_HEIGHT + ACTION_BUTTON_BOTTOM_PADDING)
                .max(LARGE_CONTAINER_HEIGHT);
            let close_y = action_y + (ACTION_HEIGHT - ICON_BUTTON_SIZE) / 2.0;
            (height, VERTICAL_PADDING, action_y, close_y)
        } else {
            let height = (text_height + 2.0 * VERTICAL_PADDING)
                .max(CONTAINER_HEIGHT)
                .max(if action.is_some() { ACTION_HEIGHT } else { 0.0 })
                .max(close_width);
            (
                height,
                (height - text_height) / 2.0,
                (height - ACTION_HEIGHT) / 2.0,
                (height - ICON_BUTTON_SIZE) / 2.0,
            )
        };

        let icon = icon.map(|galley| {
            let y = if new_line {
                text_y
            } else {
                (height - galley.size().y) / 2.0
            };
            let rect = Rect::from_min_size(egui::pos2(HORIZONTAL_SPACING, y), galley.size());
            (galley, rect)
        });
        let text_rect = Rect::from_min_size(
            egui::pos2(HORIZONTAL_SPACING + icon_width, text_y),
            text.size(),
        );
        let action = action.map(|galley| {
            let rect = Rect::from_min_size(
                egui::pos2(action_x, action_y),
                Vec2::new(action_width, ACTION_HEIGHT),
            );
            (galley, rect)
        });
        let close = self.show_close_icon.then(|| {
            Rect::from_min_size(egui::pos2(close_x, close_y), Vec2::splat(ICON_BUTTON_SIZE))
        });

        SnackbarLayout {
            size: Vec2::new(width, height),
            text,
            text_rect,
            icon,
            action,
            close,
        }
    }

    /// Shows the snackbar in its own foreground layer, anchored to the
    /// screen edge, at `alpha` opacity and `scale`.
    fn show_overlay(
        &self,
        ctx: &Context,
        id: Id,
        vertical_offset: f32,
        alpha: f32,
        scale: f32,
        interactive: bool,
    ) -> Outcome {
        let theme = MaterialTheme::of(ctx);
        let screen = ctx.content_rect();
        let (width, margin) = self.resolve_width(screen);
        let (align, offset) = match self.position {
            SnackbarPosition::Bottom => (
                Align2::CENTER_BOTTOM,
                Vec2::new(0.0, -(margin.y + vertical_offset)),
            ),
            SnackbarPosition::Top => (
                Align2::CENTER_TOP,
                Vec2::new(0.0, margin.y + vertical_offset),
            ),
        };
        let inner = Area::new(id)
            .order(Order::Foreground)
            .anchor(align, offset)
            .interactable(interactive)
            .constrain(false)
            .show(ctx, |ui| {
                ui.multiply_opacity(alpha.clamp(0.0, 1.0));
                let layout = self.layout(ui, &theme, width);
                let layout = match theme.layout_direction {
                    LayoutDirection::Ltr => layout,
                    LayoutDirection::Rtl => layout.mirrored(),
                };
                self.paint(ui, id, &theme, layout)
            });
        if (scale - 1.0).abs() > f32::EPSILON {
            let center = inner.response.rect.center().to_vec2();
            ctx.transform_layer_shapes_after_rounding(
                inner.response.layer_id,
                TSTransform::new(center * (1.0 - scale), scale),
            );
        }
        inner.inner
    }

    fn paint(&self, ui: &mut Ui, id: Id, theme: &MaterialTheme, layout: SnackbarLayout) -> Outcome {
        let (rect, response) = ui.allocate_exact_size(layout.size, Sense::hover());
        crate::a11y::describe(&response, Role::Status, true, &self.message, None);
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_live(accesskit::Live::Polite);
        });

        let corner_radius = self.resolve_corner_radius(rect, theme.layout_direction);
        let painter = ui.painter().clone();
        match self.elevation {
            Some(shadow) => {
                painter.add(shadow.as_shape(rect, corner_radius));
            }
            None => paint_elevation(
                &painter,
                rect,
                corner_radius,
                Elevation::Level3,
                theme.color(ColorRole::Shadow),
            ),
        }
        painter.rect_filled(rect, corner_radius, theme.color(ColorRole::InverseSurface));

        let text_color = theme.color(ColorRole::InverseOnSurface);
        let origin = rect.min.to_vec2();
        if let Some((galley, icon_rect)) = layout.icon {
            painter.galley(icon_rect.min + origin, galley, text_color);
        }
        painter.galley(layout.text_rect.min + origin, layout.text, text_color);

        let mut action_clicked = false;
        if let Some((galley, action_rect)) = layout.action {
            let action_rect = action_rect.translate(origin);
            let action_color = theme.color(ColorRole::InversePrimary);
            let action_response = ui.interact(action_rect, id.with("action"), Sense::click());
            crate::a11y::button(&action_response, true, galley.text());
            // The action is a TextButton: a full (pill) shape.
            let radius = CornerRadius::same((ACTION_HEIGHT / 2.0) as u8);
            paint_state_layer(
                &painter,
                action_rect,
                radius,
                action_color,
                InteractionState::from_response(&action_response),
            );
            paint_focus_ring_if_focused(&painter, &action_response, radius);
            let text_pos = action_rect.center() - galley.size() / 2.0;
            painter.galley(text_pos, galley, action_color);
            action_clicked = action_response.clicked();
        }

        let mut close_clicked = false;
        if let Some(close_rect) = layout.close {
            let close_rect = close_rect.translate(origin);
            let icon_color = self.close_icon_color.unwrap_or(text_color);
            let close_response = ui.interact(close_rect, id.with("close"), Sense::click());
            crate::a11y::button(&close_response, true, "Dismiss");
            let layer_rect =
                Rect::from_center_size(close_rect.center(), Vec2::splat(ICON_BUTTON_STATE_LAYER));
            let radius = CornerRadius::same((ICON_BUTTON_STATE_LAYER / 2.0) as u8);
            paint_state_layer(
                &painter,
                layer_rect,
                radius,
                icon_color,
                InteractionState::from_response(&close_response),
            );
            paint_focus_ring_if_focused(&painter, &close_response, radius);
            // The "close" glyph: a 14 dp cross centred in the 24 dp icon box.
            let half = 7.0;
            let c = close_rect.center();
            let stroke = Stroke::new(2.0, icon_color);
            painter.line_segment(
                [
                    egui::pos2(c.x - half, c.y - half),
                    egui::pos2(c.x + half, c.y + half),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    egui::pos2(c.x + half, c.y - half),
                    egui::pos2(c.x - half, c.y + half),
                ],
                stroke,
            );
            close_clicked = close_response.clicked();
        }

        Outcome {
            response,
            action_clicked,
            close_clicked,
        }
    }
}

/// Where a snackbar is in its life, for one frame.
enum Phase {
    /// Not visible: nothing to draw, and the timer is reset.
    Hidden,
    /// Dismissed earlier and still asked to show (the flag was not bound).
    Dismissed,
    /// The timeout passed this frame.
    TimedOut,
    /// On screen; `first` on its first frame.
    Showing { first: bool },
}

/// A snackbar's timer, kept in egui memory.
#[derive(Clone, Copy, Debug)]
struct TimerState {
    shown_at: f64,
    dismissed: bool,
    /// The last pass the snackbar was added in.
    last_pass: u64,
}

/// Content positions relative to the snackbar's top-left corner.
struct SnackbarLayout {
    size: Vec2,
    text: Arc<Galley>,
    text_rect: Rect,
    icon: Option<(Arc<Galley>, Rect)>,
    action: Option<(Arc<Galley>, Rect)>,
    close: Option<Rect>,
}

impl SnackbarLayout {
    /// The same layout with start and end swapped, for right-to-left.
    fn mirrored(self) -> Self {
        let width = self.size.x;
        let flip = |r: Rect| Rect::from_min_size(egui::pos2(width - r.max.x, r.min.y), r.size());
        Self {
            size: self.size,
            text_rect: flip(self.text_rect),
            text: self.text,
            icon: self.icon.map(|(g, r)| (g, flip(r))),
            action: self.action.map(|(g, r)| (g, flip(r))),
            close: self.close.map(flip),
        }
    }
}

/// What happened to a snackbar this frame.
struct Outcome {
    response: Response,
    action_clicked: bool,
    close_clicked: bool,
}

/// Fade (`FastEffects` spring) and scale (`FastSpatial` spring) towards
/// shown or hidden, as `SnackbarHost.kt`'s `FadeInFadeOutWithScale` does.
/// On the `first` frame the springs start from hidden.
fn motion(ctx: &Context, id: Id, first: bool, shown: bool) -> (f32, f32) {
    let theme = MaterialTheme::of(ctx);
    let effects = theme.motion.effects(SpringSpeed::Fast);
    let spatial = theme.motion.spatial(SpringSpeed::Fast);
    let alpha_id = id.with("snackbar_alpha");
    let scale_id = id.with("snackbar_scale");
    if first {
        animate_value_with_spring(ctx, alpha_id, 0.0, effects);
        animate_value_with_spring(ctx, scale_id, ENTER_SCALE, spatial);
    }
    let (alpha, scale) = if shown {
        (1.0, 1.0)
    } else {
        (0.0, ENTER_SCALE)
    };
    (
        animate_value_with_spring(ctx, alpha_id, alpha, effects).clamp(0.0, 1.0),
        animate_value_with_spring(ctx, scale_id, scale, spatial),
    )
}

impl Widget for MaterialSnackbar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show_in(ui, 0.0)
    }
}

/// A wrapper for MaterialSnackbar that includes vertical offset for stacking
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialSnackbarWithOffset<'a> {
    snackbar: MaterialSnackbar<'a>,
    vertical_offset: f32,
}

impl Widget for MaterialSnackbarWithOffset<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.snackbar.show_in(ui, self.vertical_offset)
    }
}

/// Convenience function to create a simple snackbar.
pub fn snackbar<'a>(message: impl Into<String>) -> MaterialSnackbar<'a> {
    MaterialSnackbar::new(message)
}

/// Convenience function to create a snackbar with an action.
pub fn snackbar_with_action<'a, F>(
    message: impl Into<String>,
    action_text: impl Into<String>,
    callback: F,
) -> MaterialSnackbar<'a>
where
    F: Fn() + Send + Sync + 'a,
{
    MaterialSnackbar::new(message).action(action_text, callback)
}

// ---------------------------------------------------------------------------
// The host: a queue in the context, one snackbar at a time.

/// What a queued snackbar shows (Compose's `SnackbarVisuals`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnackbarVisuals {
    /// The message.
    pub message: String,
    /// The action button's label, if any.
    pub action_label: Option<String>,
    /// Whether to show a dismiss icon button.
    pub with_dismiss_action: bool,
    /// How long it stays up.
    pub duration: SnackbarDuration,
}

impl SnackbarVisuals {
    /// A message with no action, shown for [`SnackbarDuration::Short`].
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            action_label: None,
            with_dismiss_action: false,
            duration: SnackbarDuration::Short,
        }
    }

    /// Add an action button. Compose's `showSnackbar` makes a snackbar with
    /// an action [`SnackbarDuration::Indefinite`] by default; here the
    /// duration stays what it is, so set it with [`Self::duration`].
    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.action_label = Some(label.into());
        self
    }

    /// Show a dismiss icon button.
    pub fn with_dismiss_action(mut self, dismiss: bool) -> Self {
        self.with_dismiss_action = dismiss;
        self
    }

    /// Set how long it stays up.
    pub fn duration(mut self, duration: SnackbarDuration) -> Self {
        self.duration = duration;
        self
    }
}

/// How a snackbar ended (Compose's `SnackbarResult`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SnackbarResult {
    /// It timed out, its dismiss icon was clicked, or it was dismissed in code.
    Dismissed,
    /// Its action was clicked.
    ActionPerformed,
}

/// Identifies one queued snackbar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SnackbarId(u64);

/// A finished snackbar and how it ended, from [`SnackbarHost::show`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SnackbarOutcome {
    /// Which snackbar.
    pub id: SnackbarId,
    /// How it ended.
    pub result: SnackbarResult,
}

#[derive(Clone, Debug)]
struct Showing {
    id: SnackbarId,
    visuals: SnackbarVisuals,
    shown_at: f64,
    first: bool,
}

#[derive(Clone, Debug, Default)]
struct HostState {
    next_id: u64,
    queue: VecDeque<(SnackbarId, SnackbarVisuals)>,
    current: Option<Showing>,
    leaving: Option<(SnackbarId, SnackbarVisuals)>,
    dismiss_requested: bool,
}

/// A queue of snackbars kept in the egui context, shown one at a time
/// (Compose's `SnackbarHostState` plus `SnackbarHost`).
///
/// ```
/// # use egui_material3::*;
/// # egui::__run_test_ui(|ui| {
/// let host = SnackbarHost::new();
/// if ui.button("Delete").clicked() {
///     host.enqueue(ui.ctx(), SnackbarVisuals::new("Deleted").action("Undo"));
/// }
/// // Once per frame, after the rest of the UI:
/// if let Some(outcome) = host.show(ui.ctx()) {
///     if outcome.result == SnackbarResult::ActionPerformed {
///         // undo
///     }
/// }
/// # });
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SnackbarHost {
    id: Id,
    position: SnackbarPosition,
}

impl Default for SnackbarHost {
    fn default() -> Self {
        Self::new()
    }
}

impl SnackbarHost {
    /// The default host. Every `SnackbarHost::new()` shares one queue.
    pub fn new() -> Self {
        Self::with_id("egui_material3::SnackbarHost")
    }

    /// A host with its own queue, keyed by `id`.
    pub fn with_id(id: impl Hash + std::fmt::Debug) -> Self {
        Self {
            id: Id::unique(id),
            position: SnackbarPosition::Bottom,
        }
    }

    /// Show the snackbars at the top of the screen instead of the bottom.
    pub fn position(mut self, position: SnackbarPosition) -> Self {
        self.position = position;
        self
    }

    fn with_state<R>(&self, ctx: &Context, f: impl FnOnce(&mut HostState) -> R) -> R {
        ctx.data_mut(|d| f(d.get_temp_mut_or_default::<HostState>(self.id)))
    }

    /// Queue a snackbar. It shows after the ones queued before it.
    pub fn enqueue(&self, ctx: &Context, visuals: SnackbarVisuals) -> SnackbarId {
        let id = self.with_state(ctx, |s| {
            let id = SnackbarId(s.next_id);
            s.next_id = s.next_id.wrapping_add(1);
            s.queue.push_back((id, visuals));
            id
        });
        ctx.request_repaint();
        id
    }

    /// Dismiss the snackbar on screen (its result is
    /// [`SnackbarResult::Dismissed`]), on the next [`Self::show`].
    pub fn dismiss_current(&self, ctx: &Context) {
        self.with_state(ctx, |s| s.dismiss_requested = s.current.is_some());
        ctx.request_repaint();
    }

    /// What the snackbar on screen shows, if one is.
    pub fn current(&self, ctx: &Context) -> Option<SnackbarVisuals> {
        self.with_state(ctx, |s| s.current.as_ref().map(|c| c.visuals.clone()))
    }

    /// How many snackbars wait behind the one on screen.
    pub fn pending(&self, ctx: &Context) -> usize {
        self.with_state(ctx, |s| s.queue.len())
    }

    /// Draws the current snackbar (and the previous one fading out) and
    /// advances the queue. Call it once per frame, after the rest of the
    /// UI. Returns how a snackbar ended, on the frame it ends.
    pub fn show(&self, ctx: &Context) -> Option<SnackbarOutcome> {
        let now = ctx.input(|i| i.time);
        let mut state = self.with_state(ctx, std::mem::take);
        let mut finished = None;

        if state.current.is_none() {
            if let Some((id, visuals)) = state.queue.pop_front() {
                state.current = Some(Showing {
                    id,
                    visuals,
                    shown_at: now,
                    first: true,
                });
            }
        }

        if let Some(current) = state.current.as_mut() {
            let timed_out = match current.visuals.duration.timeout() {
                Some(timeout) => {
                    let remaining = timeout.as_secs_f64() - (now - current.shown_at);
                    if remaining > 0.0 {
                        ctx.request_repaint_after(Duration::from_secs_f64(remaining));
                    }
                    remaining <= 0.0
                }
                None => false,
            };
            if timed_out || state.dismiss_requested {
                finished = Some(SnackbarResult::Dismissed);
            } else {
                let id = self.id.with(current.id);
                let snackbar = self.snackbar_for(&current.visuals);
                let (alpha, scale) = motion(ctx, id, current.first, true);
                current.first = false;
                let outcome = snackbar.show_overlay(ctx, id, 0.0, alpha, scale, true);
                if outcome.action_clicked {
                    finished = Some(SnackbarResult::ActionPerformed);
                } else if outcome.close_clicked {
                    finished = Some(SnackbarResult::Dismissed);
                }
            }
        }
        state.dismiss_requested = false;

        let outcome = match (finished, state.current.take()) {
            (Some(result), Some(current)) => {
                state.leaving = Some((current.id, current.visuals));
                ctx.request_repaint();
                Some(SnackbarOutcome {
                    id: current.id,
                    result,
                })
            }
            (None, current) => {
                state.current = current;
                None
            }
            (Some(_), None) => None,
        };

        if let Some((id, visuals)) = state.leaving.take() {
            let area_id = self.id.with(id);
            let (alpha, scale) = motion(ctx, area_id, false, false);
            if alpha > 0.01 {
                let snackbar = self.snackbar_for(&visuals);
                // Not interactive while it fades: the result is already in.
                snackbar.show_overlay(ctx, area_id, 0.0, alpha, scale, false);
                state.leaving = Some((id, visuals));
            }
        }

        self.with_state(ctx, |s| {
            // Keep anything queued while this frame ran.
            state.queue.extend(s.queue.drain(..));
            state.next_id = state.next_id.max(s.next_id);
            state.dismiss_requested |= s.dismiss_requested;
            *s = state;
        });
        outcome
    }

    fn snackbar_for(&self, visuals: &SnackbarVisuals) -> MaterialSnackbar<'static> {
        let mut snackbar = MaterialSnackbar::new(visuals.message.clone())
            .auto_dismiss(None)
            .position(self.position)
            .show_close_icon(visuals.with_dismiss_action);
        if let Some(label) = &visuals.action_label {
            snackbar = snackbar.action(label.clone(), || {});
        }
        snackbar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Regression test for nikescar/egui-material3#12: the auto-dismiss timer
    /// survives the widget being rebuilt every frame, and `on_visible` fires once.
    #[test]
    fn test_auto_dismiss_fires_and_on_visible_once() {
        let ctx = egui::Context::default();
        let shown = Arc::new(AtomicUsize::new(0));
        let frame = |time: f64| {
            let mut size = Vec2::ZERO;
            let input = egui::RawInput {
                time: Some(time),
                ..Default::default()
            };
            ctx.run_ui(input, |ui| {
                let shown = Arc::clone(&shown);
                let snackbar = MaterialSnackbar::new("Saved")
                    .auto_dismiss(Some(Duration::from_secs(4)))
                    .on_visible(move || {
                        shown.fetch_add(1, Ordering::Relaxed);
                    })
                    .show();
                size = ui.add(snackbar).rect.size();
            })
            .textures_delta
            .clear();
            size
        };
        assert!(
            frame(0.0) != Vec2::ZERO,
            "the snackbar should show at first"
        );
        assert!(
            frame(1.0) != Vec2::ZERO,
            "the snackbar should still show at 1 s"
        );
        assert!(
            frame(5.0) == Vec2::ZERO,
            "the snackbar should be dismissed after 4 s"
        );
        assert_eq!(
            shown.load(Ordering::Relaxed),
            1,
            "on_visible should fire once"
        );
    }

    #[test]
    fn durations_follow_compose() {
        assert_eq!(
            SnackbarDuration::Short.timeout(),
            Some(Duration::from_millis(4000)),
            "SnackbarDuration.Short is 4000 ms"
        );
        assert_eq!(
            SnackbarDuration::Long.timeout(),
            Some(Duration::from_millis(10000)),
            "SnackbarDuration.Long is 10000 ms"
        );
        assert_eq!(
            SnackbarDuration::Indefinite.timeout(),
            None,
            "SnackbarDuration.Indefinite never times out"
        );
    }
}
