// ============================================================================
// Module:       bottom_sheet
// Description:  Material 3 bottom sheets, standard and modal: drag handle,
//               spring motion, drag to expand, collapse and dismiss.
//
// Dependencies: egui; crate::scrim, crate::tokens, crate::material_theme
// ============================================================================

//! Material Design 3 bottom sheets.
//!
//! # Tokens
//!
//! From Compose's `SheetBottomTokens.kt` and `SheetDefaults.kt`:
//!
//! - container `SurfaceContainerLow` (`DockedContainerColor`), shape
//!   `CornerExtraLargeTop` (28 dp top corners, `DockedContainerShape`),
//!   elevation `Level1` (`DockedModalContainerElevation`,
//!   `DockedStandardContainerElevation`);
//! - drag handle 32 × 4 dp in `OnSurfaceVariant` (`DockedDragHandleWidth`,
//!   `DockedDragHandleHeight`, `DockedDragHandleColor`), with 22 dp above
//!   and below (`DragHandleVerticalPadding`);
//! - maximum width 640 dp (`BottomSheetDefaults.SheetMaxWidth`), peek height
//!   56 dp (`BottomSheetDefaults.SheetPeekHeight`);
//! - a drag settles on the next anchor after 56 dp (`PositionalThreshold`)
//!   or a fling faster than 125 dp/s (`VelocityThreshold`);
//! - motion is the `DefaultSpatial` spring of the theme's motion scheme
//!   ([`MotionScheme::spatial`]`(`[`SpringSpeed::Default`]`)`).
//!
//! A **standard** sheet shows over the app without blocking it, and moves
//! between expanded and collapsed (peek height): drag it, or click the
//! handle. A **modal** sheet blocks the app behind a scrim (see
//! [`crate::scrim`]); dragging it down past the threshold, clicking the
//! scrim or the handle, or pressing Escape dismisses it.
//!
//! [`MotionScheme::spatial`]: crate::tokens::MotionScheme::spatial

use egui::{Align2, Context, Id, Order, Pos2, Rect, Response, Sense, Ui, Vec2};

use crate::scrim::{self, FocusReturn};
use crate::tokens::{ColorRole, Elevation, Shape, SpringSpec, SpringSpeed};
use crate::MaterialTheme;

/// `BottomSheetDefaults.SheetMaxWidth`.
pub const SHEET_MAX_WIDTH: f32 = 640.0;
/// `BottomSheetDefaults.SheetPeekHeight`.
pub const SHEET_PEEK_HEIGHT: f32 = 56.0;
/// `SheetBottomTokens.DockedDragHandleWidth`.
pub const DRAG_HANDLE_WIDTH: f32 = 32.0;
/// `SheetBottomTokens.DockedDragHandleHeight`.
pub const DRAG_HANDLE_HEIGHT: f32 = 4.0;
/// `SheetDefaults.kt` `DragHandleVerticalPadding`.
const DRAG_HANDLE_VERTICAL_PADDING: f32 = 22.0;
/// `SheetDefaults.kt` `PositionalThreshold`: a drag this far from the
/// current anchor settles on the next one.
pub const POSITIONAL_THRESHOLD: f32 = 56.0;
/// `SheetDefaults.kt` `VelocityThreshold` (dp per second): a faster fling
/// settles on the next anchor in its direction.
pub const VELOCITY_THRESHOLD: f32 = 125.0;

/// Standard or modal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BottomSheetKind {
    /// Shows over the app without blocking it; expands and collapses.
    Standard,
    /// Blocks the app behind a scrim until dismissed.
    #[default]
    Modal,
}

/// Why a sheet closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SheetCloseReason {
    /// Escape was pressed while it was the top modal.
    Escape,
    /// The scrim was clicked.
    Scrim,
    /// It was dragged down past the threshold, or flung down.
    Dragged,
    /// The drag handle was clicked.
    DragHandle,
    /// The content closure set the open flag to `false`.
    Content,
}

/// What [`BottomSheet::show`] reports.
#[derive(Debug)]
pub struct BottomSheetResponse<R> {
    /// What the content closure returned, while the sheet was open.
    pub inner: Option<R>,
    /// Why the sheet closed this frame, if it did. A sheet closed by the
    /// caller reports nothing.
    pub close_reason: Option<SheetCloseReason>,
    /// The sheet surface, when it was drawn this frame.
    pub response: Option<Response>,
    /// A standard sheet is expanded (rather than collapsed to its peek
    /// height). Always true for a modal sheet.
    pub expanded: bool,
}

impl<R> BottomSheetResponse<R> {
    /// The sheet closed this frame.
    pub fn closed(&self) -> bool {
        self.close_reason.is_some()
    }
}

/// A spring towards a target offset, restartable from any position and
/// velocity (a drag release).
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpringMotion {
    target: f32,
    start_time: f64,
    x0: f32,
    v0: f32,
}

impl SpringMotion {
    /// At rest at `value`.
    pub(crate) fn at(value: f32, now: f64) -> Self {
        Self {
            target: value,
            start_time: now,
            x0: 0.0,
            v0: 0.0,
        }
    }

    /// Value and velocity at `now`.
    pub(crate) fn sample(&self, spring: SpringSpec, now: f64) -> (f32, f32) {
        let t = (now - self.start_time).max(0.0) as f32;
        let (x, v) = spring.displacement(self.x0, self.v0, t);
        (self.target + x, v)
    }

    /// Heads for `target` from wherever it is now, keeping its velocity.
    pub(crate) fn retarget(self, spring: SpringSpec, target: f32, now: f64) -> Self {
        if (self.target - target).abs() <= f32::EPSILON {
            return self;
        }
        let (value, velocity) = self.sample(spring, now);
        self.launch(target, value, velocity, now)
    }

    /// Heads for `target` from `value` moving at `velocity`.
    pub(crate) fn launch(self, target: f32, value: f32, velocity: f32, now: f64) -> Self {
        Self {
            target,
            start_time: now,
            x0: value - target,
            v0: velocity,
        }
    }

    /// The value now, settling (and stopping repaints) once at rest.
    pub(crate) fn step(&mut self, ctx: &Context, spring: SpringSpec, now: f64) -> f32 {
        let (value, velocity) = self.sample(spring, now);
        if (value - self.target).abs() < 0.5 && velocity.abs() < 1.0 {
            *self = Self::at(self.target, now);
            self.target
        } else {
            ctx.request_repaint();
            value
        }
    }

    /// At rest on its target.
    pub(crate) fn settled(&self) -> bool {
        self.x0 == 0.0 && self.v0 == 0.0
    }

    /// Where it is heading.
    pub(crate) fn target(&self) -> f32 {
        self.target
    }
}

#[derive(Clone, Copy, Debug)]
struct SheetState {
    /// Height of the sheet surface, measured last frame.
    height: f32,
    expanded: bool,
    motion: SpringMotion,
    /// Offset while a drag is in progress.
    drag: Option<f32>,
}

/// Material Design 3 bottom sheet.
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::BottomSheet;
///
/// let mut open = true;
/// let response = BottomSheet::modal(egui::Id::unique("share_sheet"), &mut open).show(ui.ctx(), |ui, _open| {
///     ui.label("Share with");
/// });
/// if let Some(reason) = response.close_reason {
///     // Dismissed: dragged down, scrim, Escape...
///     let _ = reason;
/// }
/// # });
/// ```
pub struct BottomSheet<'a> {
    id: Id,
    open: &'a mut bool,
    kind: BottomSheetKind,
    max_width: f32,
    drag_handle: bool,
    peek_height: f32,
    dismiss_on_escape: bool,
    dismiss_on_scrim: bool,
    scrim: bool,
    label: String,
}

impl<'a> BottomSheet<'a> {
    /// A sheet of `kind`, shown while `*open` is true.
    pub fn new(id: impl Into<Id>, open: &'a mut bool, kind: BottomSheetKind) -> Self {
        Self {
            id: id.into(),
            open,
            kind,
            max_width: SHEET_MAX_WIDTH,
            drag_handle: true,
            peek_height: SHEET_PEEK_HEIGHT,
            dismiss_on_escape: true,
            dismiss_on_scrim: true,
            scrim: true,
            label: "Bottom sheet".to_owned(),
        }
    }

    /// A modal sheet.
    pub fn modal(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self::new(id, open, BottomSheetKind::Modal)
    }

    /// A standard sheet.
    pub fn standard(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self::new(id, open, BottomSheetKind::Standard)
    }

    /// Maximum width (default 640, never more than the window).
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Show the drag handle (default `true`).
    pub fn drag_handle(mut self, show: bool) -> Self {
        self.drag_handle = show;
        self
    }

    /// Height a collapsed standard sheet keeps on screen (default 56).
    pub fn peek_height(mut self, height: f32) -> Self {
        self.peek_height = height.max(0.0);
        self
    }

    /// Whether Escape dismisses a modal sheet (default `true`).
    pub fn dismiss_on_escape(mut self, dismiss: bool) -> Self {
        self.dismiss_on_escape = dismiss;
        self
    }

    /// Whether a scrim click dismisses a modal sheet (default `true`).
    pub fn dismiss_on_scrim(mut self, dismiss: bool) -> Self {
        self.dismiss_on_scrim = dismiss;
        self
    }

    /// Whether a modal sheet paints its scrim (default `true`). Without
    /// it the sheet still blocks the app, and a click outside it still
    /// counts as a scrim click.
    pub fn scrim(mut self, scrim: bool) -> Self {
        self.scrim = scrim;
        self
    }

    /// Accessible name of the sheet (default "Bottom sheet").
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Shows the sheet with `content`, which gets the UI and the open flag
    /// and whose return value comes back in
    /// [`BottomSheetResponse::inner`].
    pub fn show<R>(
        self,
        ctx: &Context,
        content: impl FnOnce(&mut Ui, &mut bool) -> R,
    ) -> BottomSheetResponse<R> {
        let Self {
            id,
            open,
            kind,
            max_width,
            drag_handle,
            peek_height,
            dismiss_on_escape,
            dismiss_on_scrim,
            scrim: show_scrim,
            label,
        } = self;
        let modal = kind == BottomSheetKind::Modal;
        if modal {
            FocusReturn::update(ctx, id, *open);
        }

        let theme = MaterialTheme::of(ctx);
        let spring = theme.motion.spatial(SpringSpeed::Default);
        let now = ctx.input(|i| i.time);
        let screen = ctx.content_rect();
        let state_id = id.with("bottom_sheet_state");
        let previous = ctx.data(|d| d.get_temp::<SheetState>(state_id));
        let Some(mut state) = previous.or_else(|| {
            // First sight of an open sheet: start below the window and rise.
            open.then(|| SheetState {
                height: screen.height(),
                expanded: true,
                motion: SpringMotion::at(screen.height(), now),
                drag: None,
            })
        }) else {
            return BottomSheetResponse {
                inner: None,
                close_reason: None,
                response: None,
                expanded: false,
            };
        };

        // Offsets push the sheet down from fully shown (0).
        let hidden = state.height + 8.0;
        let collapsed = (state.height - peek_height).max(0.0);
        let target = if !*open {
            hidden
        } else if modal || state.expanded {
            0.0
        } else {
            collapsed
        };
        if state.drag.is_none() {
            state.motion = state.motion.retarget(spring, target, now);
        }
        let offset = match state.drag {
            Some(drag) => drag,
            None => state.motion.step(ctx, spring, now),
        }
        .max(0.0);

        if !*open && state.drag.is_none() && state.motion.settled() {
            ctx.data_mut(|d| d.remove::<SheetState>(state_id));
            return BottomSheetResponse {
                inner: None,
                close_reason: None,
                response: None,
                expanded: false,
            };
        }
        let interactive = *open;

        let width = max_width.min(screen.width()).max(0.0);
        let top_left = Pos2::new(
            screen.center().x - width / 2.0,
            screen.bottom() - state.height + offset,
        );
        let area_offset = top_left - screen.min;
        // While the sheet is (partly) open the scrim fades with it.
        let visibility = if state.height > 0.0 {
            (1.0 - offset / state.height).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let mut local_open = true;
        let mut handle_clicked = false;
        let mut drag_delta = 0.0;
        let mut drag_stopped = false;

        let body = |ui: &mut Ui| {
            if !interactive {
                ui.disable();
            }
            // Drag anywhere on the sheet; widgets in it win over this.
            let sheet_rect = Rect::from_min_size(top_left, Vec2::new(width, state.height));
            let background = ui.interact(sheet_rect, id.with("sheet_drag"), Sense::drag());
            let shape = Shape::EXTRA_LARGE_TOP.corner_radius(sheet_rect, theme.layout_direction);
            let surface = scrim::elevated_surface(
                ui,
                theme.color(ColorRole::SurfaceContainerLow),
                shape,
                Elevation::Level1,
                egui::Margin::ZERO,
                |ui| {
                    ui.set_width(width);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    let handle = if drag_handle {
                        let (rect, handle) = ui.allocate_exact_size(
                            Vec2::new(
                                width,
                                DRAG_HANDLE_HEIGHT + 2.0 * DRAG_HANDLE_VERTICAL_PADDING,
                            ),
                            Sense::click_and_drag(),
                        );
                        crate::a11y::button(&handle, true, "Drag handle");
                        let bar = Rect::from_center_size(
                            rect.center(),
                            Vec2::new(DRAG_HANDLE_WIDTH, DRAG_HANDLE_HEIGHT),
                        );
                        ui.painter().rect_filled(
                            bar,
                            DRAG_HANDLE_HEIGHT / 2.0,
                            theme.color(ColorRole::OnSurfaceVariant),
                        );
                        crate::paint::paint_focus_ring_if_focused(
                            ui.painter(),
                            &handle,
                            DRAG_HANDLE_HEIGHT / 2.0,
                        );
                        Some(handle)
                    } else {
                        ui.add_space(DRAG_HANDLE_VERTICAL_PADDING / 2.0);
                        None
                    };
                    let handle_height = if drag_handle {
                        DRAG_HANDLE_HEIGHT + 2.0 * DRAG_HANDLE_VERTICAL_PADDING
                    } else {
                        DRAG_HANDLE_VERTICAL_PADDING / 2.0
                    };
                    let inner = egui::ScrollArea::vertical()
                        .id_salt(id.with("sheet_scroll"))
                        .max_height((screen.height() - handle_height).max(0.0))
                        .show(ui, |ui| {
                            ui.set_width(width);
                            content(ui, &mut local_open)
                        })
                        .inner;
                    (inner, handle)
                },
            );
            let (inner, handle) = surface.inner;
            for response in std::iter::once(&background).chain(handle.as_ref()) {
                if response.dragged() {
                    drag_delta += response.drag_delta().y;
                }
                drag_stopped |= response.drag_stopped();
            }
            handle_clicked = handle.as_ref().is_some_and(Response::clicked);
            let role = if modal {
                egui::Role::Dialog
            } else {
                egui::Role::Pane
            };
            crate::a11y::describe(&surface.response, role, true, &label, None);
            (inner, surface.response)
        };

        let (inner, surface_response, scrim_clicked, escape_pressed) = if modal {
            let area = scrim::modal_area(id)
                .anchor(Align2::LEFT_TOP, area_offset)
                .constrain(false);
            let shown = scrim::show_modal(
                ctx,
                area,
                scrim::scrim_color(&theme, if show_scrim { visibility } else { 0.0 }),
                interactive,
                body,
            );
            let (inner, response) = shown.inner;
            (inner, response, shown.scrim_clicked, shown.escape_pressed)
        } else {
            let shown = egui::Area::new(id)
                .order(Order::Middle)
                .anchor(Align2::LEFT_TOP, area_offset)
                .constrain(false)
                .interactable(interactive)
                .show(ctx, body);
            let (inner, response) = shown.inner;
            (inner, response, false, false)
        };

        state.height = surface_response.rect.height();
        let mut close_reason = None;
        if interactive {
            if drag_delta != 0.0 {
                state.drag = Some((offset + drag_delta).max(0.0));
            }
            if drag_stopped {
                let position = state.drag.unwrap_or(offset);
                let velocity = ctx.input(|i| i.pointer.velocity().y);
                let anchor = state.motion.target();
                let toward_collapse =
                    position - anchor > POSITIONAL_THRESHOLD || velocity > VELOCITY_THRESHOLD;
                let toward_expand =
                    anchor - position > POSITIONAL_THRESHOLD || velocity < -VELOCITY_THRESHOLD;
                let next = if modal {
                    if toward_collapse {
                        close_reason = Some(SheetCloseReason::Dragged);
                        hidden
                    } else {
                        0.0
                    }
                } else if toward_collapse {
                    state.expanded = false;
                    collapsed
                } else if toward_expand {
                    state.expanded = true;
                    0.0
                } else {
                    anchor
                };
                state.motion = state.motion.launch(next, position, velocity, now);
                state.drag = None;
                ctx.request_repaint();
            }
            if close_reason.is_none() && handle_clicked {
                if modal {
                    close_reason = Some(SheetCloseReason::DragHandle);
                } else {
                    state.expanded = !state.expanded;
                }
            }
            if close_reason.is_none() {
                close_reason = if !local_open {
                    Some(SheetCloseReason::Content)
                } else if escape_pressed && dismiss_on_escape {
                    Some(SheetCloseReason::Escape)
                } else if scrim_clicked && dismiss_on_scrim {
                    Some(SheetCloseReason::Scrim)
                } else {
                    None
                };
            }
            if close_reason.is_some() {
                *open = false;
                if modal {
                    FocusReturn::update(ctx, id, false);
                }
            }
        } else {
            state.drag = None;
        }
        ctx.data_mut(|d| d.insert_temp(state_id, state));

        BottomSheetResponse {
            inner: interactive.then_some(inner),
            close_reason,
            response: Some(surface_response),
            expanded: modal || state.expanded,
        }
    }
}
