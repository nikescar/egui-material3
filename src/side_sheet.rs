// ============================================================================
// Module:       side_sheet
// Description:  Material 3 side sheets, standard and modal, sliding in from
//               the end edge.
//
// Dependencies: egui; crate::bottom_sheet, crate::scrim, crate::tokens,
//               crate::material_theme
// ============================================================================

//! Material Design 3 side sheets.
//!
//! Compose material3 has no side sheet and no side-sheet token file, so
//! the values follow the Material 3 side sheet guidance, built from the
//! token scales that do exist in Compose:
//!
//! - a **standard** sheet is a `Surface` panel at `Level0` with square
//!   corners;
//! - a **modal** sheet is `SurfaceContainerLow` at `Level1` with
//!   `CornerLargeStart` corners (16 dp on the edge facing the app;
//!   `ShapeTokens.CornerLargeStart`), over the scrim (see
//!   [`crate::scrim`]);
//! - the width defaults to 320 dp and never exceeds the window;
//! - motion is the `DefaultSpatial` spring, like the bottom sheet.
//!
//! The sheet sits on the end edge: the right in left-to-right layouts, the
//! left in right-to-left ones ([`MaterialTheme::layout_direction`]).
//! A modal sheet closes on Escape (top modal only) and on a scrim click.

use egui::{Align2, Context, Id, Order, Pos2, Response, RichText, Ui, Vec2};

use crate::bottom_sheet::{SheetCloseReason, SpringMotion};
use crate::scrim::{self, FocusReturn};
use crate::tokens::{ColorRole, Elevation, LayoutDirection, Shape, SpringSpeed, TypeRole};
use crate::MaterialTheme;

/// Default side sheet width.
pub const SIDE_SHEET_WIDTH: f32 = 320.0;
/// Padding inside the sheet.
const SIDE_SHEET_PADDING: f32 = 24.0;

/// Standard or modal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SideSheetKind {
    /// A panel over the app's end edge that does not block it.
    Standard,
    /// Blocks the app behind a scrim until dismissed.
    #[default]
    Modal,
}

/// What [`SideSheet::show`] reports.
#[derive(Debug)]
pub struct SideSheetResponse<R> {
    /// What the content closure returned, while the sheet was open.
    pub inner: Option<R>,
    /// Why the sheet closed this frame, if it did.
    pub close_reason: Option<SheetCloseReason>,
    /// The sheet surface, when it was drawn this frame.
    pub response: Option<Response>,
}

impl<R> SideSheetResponse<R> {
    /// The sheet closed this frame.
    pub fn closed(&self) -> bool {
        self.close_reason.is_some()
    }
}

/// Material Design 3 side sheet.
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::SideSheet;
///
/// let mut open = true;
/// SideSheet::modal(egui::Id::unique("filters"), &mut open)
///     .title("Filters")
///     .show(ui.ctx(), |ui, _open| {
///         ui.label("Price range");
///     });
/// # });
/// ```
pub struct SideSheet<'a> {
    id: Id,
    open: &'a mut bool,
    kind: SideSheetKind,
    width: f32,
    title: Option<String>,
    dismiss_on_escape: bool,
    dismiss_on_scrim: bool,
}

impl<'a> SideSheet<'a> {
    /// A side sheet of `kind`, shown while `*open` is true.
    pub fn new(id: impl Into<Id>, open: &'a mut bool, kind: SideSheetKind) -> Self {
        Self {
            id: id.into(),
            open,
            kind,
            width: SIDE_SHEET_WIDTH,
            title: None,
            dismiss_on_escape: true,
            dismiss_on_scrim: true,
        }
    }

    /// A modal side sheet.
    pub fn modal(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self::new(id, open, SideSheetKind::Modal)
    }

    /// A standard side sheet.
    pub fn standard(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self::new(id, open, SideSheetKind::Standard)
    }

    /// Width (default 320, never more than the window).
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(0.0);
        self
    }

    /// A headline at the top of the sheet (`TitleLarge`, `OnSurfaceVariant`).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
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

    /// Shows the sheet with `content`, which gets the UI and the open flag
    /// and whose return value comes back in [`SideSheetResponse::inner`].
    pub fn show<R>(
        self,
        ctx: &Context,
        content: impl FnOnce(&mut Ui, &mut bool) -> R,
    ) -> SideSheetResponse<R> {
        let Self {
            id,
            open,
            kind,
            width,
            title,
            dismiss_on_escape,
            dismiss_on_scrim,
        } = self;
        let modal = kind == SideSheetKind::Modal;
        if modal {
            FocusReturn::update(ctx, id, *open);
        }
        let theme = MaterialTheme::of(ctx);
        let spring = theme.motion.spatial(SpringSpeed::Default);
        let now = ctx.input(|i| i.time);
        let screen = ctx.content_rect();
        let width = width.min(screen.width());

        // Offset pushes the sheet out past the end edge; 0 is fully shown.
        let hidden = width + 8.0;
        let motion_id = id.with("side_sheet_motion");
        let previous = ctx.data(|d| d.get_temp::<SpringMotion>(motion_id));
        let Some(mut motion) = previous.or_else(|| open.then(|| SpringMotion::at(hidden, now)))
        else {
            return SideSheetResponse {
                inner: None,
                close_reason: None,
                response: None,
            };
        };
        motion = motion.retarget(spring, if *open { 0.0 } else { hidden }, now);
        let offset = motion.step(ctx, spring, now).max(0.0);
        if !*open && motion.settled() {
            ctx.data_mut(|d| d.remove::<SpringMotion>(motion_id));
            return SideSheetResponse {
                inner: None,
                close_reason: None,
                response: None,
            };
        }
        ctx.data_mut(|d| d.insert_temp(motion_id, motion));
        let interactive = *open;
        let visibility = (1.0 - offset / hidden.max(1.0)).clamp(0.0, 1.0);

        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let left = if rtl {
            screen.left() - offset
        } else {
            screen.right() - width + offset
        };
        let top_left = Pos2::new(left, screen.top());
        let (fill, shape, level) = if modal {
            (
                theme.color(ColorRole::SurfaceContainerLow),
                Shape::LARGE_START,
                Elevation::Level1,
            )
        } else {
            (
                theme.color(ColorRole::Surface),
                Shape::NONE,
                Elevation::Level0,
            )
        };
        let corner_radius = shape.corner_radius(
            egui::Rect::from_min_size(top_left, Vec2::new(width, screen.height())),
            theme.layout_direction,
        );

        let mut local_open = true;
        let body = |ui: &mut Ui| {
            if !interactive {
                ui.disable();
            }
            let surface = scrim::elevated_surface(
                ui,
                fill,
                corner_radius,
                level,
                scrim::margin([SIDE_SHEET_PADDING; 4]),
                |ui| {
                    let inner_size = Vec2::new(
                        (width - 2.0 * SIDE_SHEET_PADDING).max(0.0),
                        (screen.height() - 2.0 * SIDE_SHEET_PADDING).max(0.0),
                    );
                    ui.set_min_size(inner_size);
                    ui.set_max_size(inner_size);
                    if let Some(title) = &title {
                        let style = TypeRole::TitleLarge.style();
                        ui.label(
                            RichText::new(title)
                                .font(style.font_id(ui.ctx()))
                                .line_height(Some(style.line_height))
                                .color(theme.color(ColorRole::OnSurfaceVariant)),
                        );
                        ui.add_space(SIDE_SHEET_PADDING);
                    }
                    egui::ScrollArea::vertical()
                        .id_salt(id.with("side_sheet_scroll"))
                        .show(ui, |ui| content(ui, &mut local_open))
                        .inner
                },
            );
            let role = if modal {
                egui::Role::Dialog
            } else {
                egui::Role::Pane
            };
            let label = title.as_deref().unwrap_or("Side sheet");
            crate::a11y::describe(&surface.response, role, true, label, None);
            (surface.inner, surface.response)
        };

        let area_offset = top_left - screen.min;
        let (inner, response, scrim_clicked, escape_pressed) = if modal {
            let area = scrim::modal_area(id)
                .anchor(Align2::LEFT_TOP, area_offset)
                .constrain(false);
            let shown = scrim::show_modal(
                ctx,
                area,
                scrim::scrim_color(&theme, visibility),
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

        let mut close_reason = None;
        if interactive {
            close_reason = if !local_open {
                Some(SheetCloseReason::Content)
            } else if escape_pressed && dismiss_on_escape {
                Some(SheetCloseReason::Escape)
            } else if scrim_clicked && dismiss_on_scrim {
                Some(SheetCloseReason::Scrim)
            } else {
                None
            };
            if close_reason.is_some() {
                *open = false;
                if modal {
                    FocusReturn::update(ctx, id, false);
                }
            }
        }
        SideSheetResponse {
            inner: interactive.then_some(inner),
            close_reason,
            response: Some(response),
        }
    }
}
