// ============================================================================
// Module:       date_picker::common
// Description:  Drawing helpers shared by the date and time pickers: text,
//               painted icons, icon buttons, the outlined input field and
//               the modal dialog shell.
//
// Dependencies: egui; crate::{a11y, paint, tokens, MaterialTheme, button}
// ============================================================================

//! Pieces both pickers draw with. Everything is painted with shapes, so the
//! pickers work without the Material Symbols font loaded.

use std::sync::Arc;

use egui::{
    Align, Color32, CornerRadius, Frame, Galley, Id, Margin, Painter, Pos2, Rect, Response, Role,
    Sense, Stroke, StrokeKind, TextEdit, Ui, Vec2,
};

use super::tokens as t;
use crate::tokens::{ColorRole, Elevation, InteractionState, Shape, TypeRole};
use crate::MaterialTheme;

/// Lays out `text` in the type role `role`, in `color`.
pub(crate) fn galley(ui: &Ui, role: TypeRole, text: &str, color: Color32) -> Arc<Galley> {
    let job = role.style().layout_job(ui.ctx(), text, color);
    ui.painter().layout_job(job)
}

/// Paints `galley` centred on `center`.
pub(crate) fn paint_centered(painter: &Painter, galley: Arc<Galley>, center: Pos2) {
    let pos = center - galley.size() / 2.0;
    painter.galley(pos, galley, Color32::PLACEHOLDER);
}

/// Paints `galley` with its left edge at `x`, centred vertically on `y`.
pub(crate) fn paint_left(painter: &Painter, galley: Arc<Galley>, x: f32, y: f32) {
    let pos = Pos2::new(x, y - galley.size().y / 2.0);
    painter.galley(pos, galley, Color32::PLACEHOLDER);
}

/// The corner radius of `shape` on `rect` in the theme's layout direction.
pub(crate) fn radius(theme: &MaterialTheme, shape: Shape, rect: Rect) -> CornerRadius {
    shape.corner_radius(rect, theme.layout_direction)
}

/// An icon the pickers paint themselves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Glyph {
    /// `<`: previous month.
    ChevronLeft,
    /// `>`: next month.
    ChevronRight,
    /// A small downward triangle: open the year list.
    DropDown,
    /// A small upward triangle: close the year list.
    DropUp,
    /// A pencil: switch to text input.
    Edit,
    /// A calendar: switch to the calendar.
    Calendar,
    /// A keyboard: switch the time picker to input.
    Keyboard,
    /// A clock: switch the time picker to the dial.
    Clock,
}

/// Paints `glyph` in a 24-point box centred on `center`.
pub(crate) fn paint_glyph(painter: &Painter, glyph: Glyph, center: Pos2, color: Color32) {
    let stroke = Stroke::new(2.0, color);
    let p = |x: f32, y: f32| center + Vec2::new(x, y);
    match glyph {
        Glyph::ChevronLeft => {
            painter.line(vec![p(3.5, -6.0), p(-2.5, 0.0), p(3.5, 6.0)], stroke);
        }
        Glyph::ChevronRight => {
            painter.line(vec![p(-3.5, -6.0), p(2.5, 0.0), p(-3.5, 6.0)], stroke);
        }
        Glyph::DropDown => {
            painter.add(egui::Shape::convex_polygon(
                vec![p(-5.0, -2.5), p(5.0, -2.5), p(0.0, 2.5)],
                color,
                Stroke::NONE,
            ));
        }
        Glyph::DropUp => {
            painter.add(egui::Shape::convex_polygon(
                vec![p(-5.0, 2.5), p(0.0, -2.5), p(5.0, 2.5)],
                color,
                Stroke::NONE,
            ));
        }
        Glyph::Edit => {
            painter.line(vec![p(-7.0, 7.0), p(-7.0, 3.0), p(4.0, -8.0)], stroke);
            painter.line(vec![p(4.0, -8.0), p(8.0, -4.0), p(-3.0, 7.0)], stroke);
            painter.line_segment([p(-7.0, 7.0), p(-3.0, 7.0)], stroke);
        }
        Glyph::Calendar => {
            let body = Rect::from_center_size(p(0.0, 1.0), Vec2::new(16.0, 16.0));
            painter.rect_stroke(body, 2.0, stroke, StrokeKind::Middle);
            painter.line_segment([p(-8.0, -3.0), p(8.0, -3.0)], stroke);
            painter.line_segment([p(-4.0, -9.0), p(-4.0, -5.0)], stroke);
            painter.line_segment([p(4.0, -9.0), p(4.0, -5.0)], stroke);
        }
        Glyph::Keyboard => {
            let body = Rect::from_center_size(center, Vec2::new(20.0, 14.0));
            painter.rect_stroke(body, 2.0, stroke, StrokeKind::Middle);
            for x in [-5.0, 0.0, 5.0] {
                painter.circle_filled(p(x, -2.5), 1.2, color);
            }
            painter.line_segment([p(-4.0, 3.0), p(4.0, 3.0)], stroke);
        }
        Glyph::Clock => {
            painter.circle_stroke(center, 9.0, stroke);
            painter.line(vec![p(0.0, -5.0), p(0.0, 0.0), p(4.0, 3.0)], stroke);
        }
    }
}

/// A 48-point icon button with a 40-point circular state layer and the
/// focus ring, reported to AccessKit as a button named `label`.
pub(crate) fn icon_button(
    ui: &mut Ui,
    id: Id,
    glyph: Glyph,
    label: &str,
    enabled: bool,
    color: Color32,
) -> Response {
    let size = Vec2::splat(t::ICON_BUTTON_SIZE);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let response = ui.interact(rect, id, crate::a11y::sense(enabled, Sense::click()));
    crate::a11y::button(&response, enabled, label);
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let layer = Rect::from_center_size(rect.center(), Vec2::splat(40.0));
        let state = InteractionState::from_response(&response);
        crate::paint::paint_state_layer(painter, layer, CornerRadius::same(20), color, state);
        let color = if enabled {
            color
        } else {
            color.gamma_multiply(crate::tokens::state::DISABLED_CONTENT_OPACITY)
        };
        paint_glyph(painter, glyph, rect.center(), color);
        crate::paint::paint_focus_ring_if_focused(painter, &response, CornerRadius::same(20));
    }
    response
}

/// What the outlined field reports.
pub(crate) struct FieldOutput {
    /// The text edit's response.
    pub(crate) response: Response,
    /// Whether the trailing icon (if any) was clicked.
    pub(crate) trailing_clicked: bool,
}

/// An outlined text field (`OutlinedTextFieldTokens`): a 56-point box with a
/// floating label on the outline, the input text, and a supporting or
/// error line below. `surface` is the colour behind the field, used to
/// cut the outline under the label.
pub(crate) struct OutlinedField<'a> {
    pub(crate) id: Id,
    pub(crate) text: &'a mut String,
    pub(crate) label: &'a str,
    pub(crate) placeholder: &'a str,
    pub(crate) supporting: Option<&'a str>,
    pub(crate) error: Option<&'a str>,
    pub(crate) width: f32,
    pub(crate) surface: Color32,
    pub(crate) trailing: Option<(Glyph, &'a str)>,
}

impl OutlinedField<'_> {
    /// Draws the field.
    pub(crate) fn show(self, ui: &mut Ui) -> FieldOutput {
        let theme = MaterialTheme::of(ui.ctx());
        let support_height = TypeRole::BodySmall.style().line_height + 4.0;
        let (outer, _) = ui.allocate_exact_size(
            Vec2::new(self.width, t::FIELD_HEIGHT + support_height),
            Sense::hover(),
        );
        let field = Rect::from_min_size(outer.min, Vec2::new(self.width, t::FIELD_HEIGHT));
        let has_error = self.error.is_some();

        // The trailing icon button sits inside the box at the end.
        let trailing_width = if self.trailing.is_some() {
            t::ICON_BUTTON_SIZE
        } else {
            0.0
        };
        let text_rect = Rect::from_min_max(
            Pos2::new(field.min.x + t::FIELD_HORIZONTAL_PADDING, field.min.y + 8.0),
            Pos2::new(
                field.max.x - t::FIELD_HORIZONTAL_PADDING.max(trailing_width),
                field.max.y - 8.0,
            ),
        );
        let input_style = t::FIELD_INPUT_FONT.style();
        let font_id = input_style.font_id(ui.ctx());
        let text_height = input_style.line_height;
        let edit_rect = Rect::from_center_size(
            text_rect.center(),
            Vec2::new(text_rect.width(), text_height),
        );
        let placeholder = self.placeholder.to_owned();
        let response = ui.put(
            edit_rect,
            TextEdit::singleline(self.text)
                .id(self.id)
                .frame(Frame::NONE)
                .margin(Margin::ZERO)
                .font(font_id)
                .text_color(theme.color(t::FIELD_INPUT_COLOR))
                .hint_text(placeholder)
                .align(egui::Align2::LEFT_CENTER)
                .desired_width(edit_rect.width()),
        );
        response.ctx.accesskit_node_builder(response.id, |node| {
            node.set_label(self.label.to_owned());
            if has_error {
                node.set_invalid(egui::accesskit::Invalid::True);
            }
        });
        let focused = response.has_focus();

        let mut trailing_clicked = false;
        if let Some((glyph, label)) = self.trailing {
            let button_rect = Rect::from_center_size(
                Pos2::new(field.max.x - t::ICON_BUTTON_SIZE / 2.0, field.center().y),
                Vec2::splat(t::ICON_BUTTON_SIZE),
            );
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(button_rect));
            let color = theme.color(ColorRole::OnSurfaceVariant);
            trailing_clicked = icon_button(
                &mut child,
                self.id.with("trailing"),
                glyph,
                label,
                true,
                color,
            )
            .clicked();
        }

        let painter = ui.painter();
        let (outline_color, outline_width) = if has_error {
            (
                theme.color(t::FIELD_ERROR_COLOR),
                if focused {
                    t::FIELD_FOCUS_OUTLINE_WIDTH
                } else {
                    t::FIELD_OUTLINE_WIDTH
                },
            )
        } else if focused {
            (
                theme.color(t::FIELD_FOCUS_OUTLINE_COLOR),
                t::FIELD_FOCUS_OUTLINE_WIDTH,
            )
        } else {
            (theme.color(t::FIELD_OUTLINE_COLOR), t::FIELD_OUTLINE_WIDTH)
        };
        painter.rect_stroke(
            field,
            radius(&theme, t::FIELD_SHAPE, field),
            Stroke::new(outline_width, outline_color),
            StrokeKind::Inside,
        );

        // The floating label, on a patch of the surface that cuts the outline.
        let label_color = if has_error {
            theme.color(t::FIELD_ERROR_COLOR)
        } else if focused {
            theme.color(t::FIELD_FOCUS_OUTLINE_COLOR)
        } else {
            theme.color(t::FIELD_LABEL_COLOR)
        };
        let label = galley(ui, t::FIELD_FLOATING_LABEL_FONT, self.label, label_color);
        let label_x = field.min.x + t::FIELD_HORIZONTAL_PADDING - 4.0;
        let patch = Rect::from_min_size(
            Pos2::new(label_x, field.min.y - label.size().y / 2.0),
            Vec2::new(label.size().x + 8.0, label.size().y),
        );
        painter.rect_filled(patch, 0.0, self.surface);
        paint_left(painter, label, label_x + 4.0, field.min.y);

        let support = match (self.error, self.supporting) {
            (Some(error), _) => Some((error, theme.color(t::FIELD_ERROR_COLOR))),
            (None, Some(text)) => Some((text, theme.color(t::FIELD_SUPPORTING_COLOR))),
            (None, None) => None,
        };
        if let Some((text, color)) = support {
            let g = galley(ui, t::FIELD_SUPPORTING_FONT, text, color);
            let y = field.max.y + 4.0 + g.size().y / 2.0;
            paint_left(
                ui.painter(),
                g,
                field.min.x + t::FIELD_HORIZONTAL_PADDING,
                y,
            );
        }
        FieldOutput {
            response,
            trailing_clicked,
        }
    }
}

/// The frame of a picker dialog: container colour, shape and Level3
/// shadow, no inner margin.
pub(crate) fn dialog_frame(theme: &MaterialTheme, container: ColorRole, shape: Shape) -> Frame {
    let r = shape.size.value().unwrap_or(28.0).round().clamp(0.0, 255.0) as u8;
    let shadows = Elevation::Level3.shadows(theme.color(ColorRole::Shadow));
    let [key, _ambient] = shadows;
    Frame::NONE
        .fill(theme.color(container))
        .corner_radius(CornerRadius::same(r))
        .shadow(key)
}

/// The scrim behind a modal picker: `ScrimTokens` 32 % of `Scrim`.
pub(crate) fn scrim_color(theme: &MaterialTheme) -> Color32 {
    theme.color(ColorRole::Scrim).gamma_multiply(0.32)
}

/// How a modal picker dialog ended this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DialogAction {
    /// Still open: nothing to do.
    Open,
    /// The confirm button was pressed; the new value has been written.
    Confirmed,
    /// Cancelled by the dismiss button, Escape or a click on the scrim; the
    /// value is unchanged.
    Dismissed,
}

/// The dialog's button row (`DialogButtonsPadding`): dismiss then confirm,
/// end-aligned. Returns `(dismissed, confirmed)`.
pub(crate) fn dialog_buttons(
    ui: &mut Ui,
    width: f32,
    dismiss: &str,
    confirm: &str,
    confirm_enabled: bool,
) -> (bool, bool) {
    let mut dismissed = false;
    let mut confirmed = false;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, t::BUTTONS_ROW_HEIGHT), Sense::hover());
    let inner = Rect::from_min_max(
        rect.min,
        Pos2::new(
            rect.max.x - t::BUTTONS_END_PADDING,
            rect.max.y - t::BUTTONS_BOTTOM_PADDING,
        ),
    );
    let mut row = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inner)
            .layout(egui::Layout::right_to_left(Align::Center)),
    );
    row.spacing_mut().item_spacing.x = t::BUTTONS_SPACING;
    if row
        .add_enabled(confirm_enabled, crate::MaterialButton::text(confirm))
        .clicked()
    {
        confirmed = true;
    }
    if row.add(crate::MaterialButton::text(dismiss)).clicked() {
        dismissed = true;
    }
    (dismissed, confirmed)
}

/// A selectable pill or circle (days, years, period buttons): reports a
/// role, label and selected state, and paints the state layer.
pub(crate) fn selectable_cell(
    ui: &mut Ui,
    rect: Rect,
    id: Id,
    enabled: bool,
    label: &str,
    selected: bool,
) -> Response {
    let response = ui.interact(rect, id, crate::a11y::sense(enabled, Sense::click()));
    crate::a11y::describe(&response, Role::Button, enabled, label, Some(selected));
    response
}
