//! Material Design 3 text fields.
//!
//! [`MaterialInput`] draws the filled and outlined text fields from the M3
//! spec, with every colour, size and width taken from Compose's
//! `FilledTextFieldTokens` and `OutlinedTextFieldTokens`:
//!
//! - **Container**: filled is `surfaceContainerHighest` with extra-small top
//!   corners (4 dp); outlined is transparent with extra-small corners.
//! - **Indicator**: the filled bottom line is 1 dp, 2 dp when focused; the
//!   outline is 1 dp, 3 dp when focused. Unfocused it is `onSurfaceVariant`
//!   (filled) or `outline` (outlined), `onSurface` on hover, `primary` when
//!   focused, `error` in error (`onErrorContainer` on hover).
//! - **Label**: rests inside the field in BodyLarge and floats to BodySmall
//!   when the field is focused or has text, over 150 ms on the emphasized
//!   easing curve. The outlined label floats into a notch cut in the outline.
//! - **Slots**: leading and trailing 24 dp icons centred 12 dp in from the
//!   edges, pushing the text to 52 dp; prefix and suffix text; supporting
//!   (helper or error) text and a character counter in BodySmall below.
//! - **Disabled**: content in `onSurface` at 38%; the filled container at
//!   4%, the outline at 12%.
//! - **Caret**: `primary`, `error` in error.

use crate::paint::paint_state_layer;
use crate::tokens::motion::DURATION_SHORT3;
use crate::tokens::state::DISABLED_CONTENT_OPACITY;
use crate::tokens::{ColorRole, CubicBezier, InteractionState, LayoutDirection, TypeRole};
use crate::MaterialTheme;
use egui::epaint::PathShape;
use egui::{
    self, Align2, Color32, FontId, Id, Pos2, Rect, Response, RichText, Sense, Stroke, TextBuffer,
    TextEdit, Ui, Vec2, Widget,
};

/// `TextFieldDefaults.MinHeight`: the container height of a one-line field.
pub const CONTAINER_HEIGHT: f32 = 56.0;
/// `FilledTextFieldTokens.ActiveIndicatorHeight`: the filled bottom line.
pub const FILLED_INDICATOR_WIDTH: f32 = 1.0;
/// `FilledTextFieldTokens.FocusActiveIndicatorHeight`.
pub const FILLED_FOCUS_INDICATOR_WIDTH: f32 = 2.0;
/// `OutlinedTextFieldTokens.OutlineWidth`.
pub const OUTLINE_WIDTH: f32 = 1.0;
/// `OutlinedTextFieldTokens.FocusOutlineWidth`.
pub const OUTLINE_FOCUS_WIDTH: f32 = 3.0;
/// `FilledTextFieldTokens.DisabledContainerOpacity`.
pub const FILLED_DISABLED_CONTAINER_OPACITY: f32 = 0.04;
/// `OutlinedTextFieldTokens.DisabledOutlineOpacity`.
pub const OUTLINED_DISABLED_OUTLINE_OPACITY: f32 = 0.12;
/// `LeadingIconSize` / `TrailingIconSize`.
pub const ICON_SIZE: f32 = 24.0;
/// Distance from the container edge to a leading or trailing icon.
pub const ICON_PADDING: f32 = 12.0;
/// `TextFieldPadding`: start and end padding of the text without icons.
pub const HORIZONTAL_PADDING: f32 = 16.0;
/// Start (or end) of the text when a leading (or trailing) icon is present:
/// the icon's 12 dp padding, its 24 dp, and 16 dp.
pub const TEXT_PADDING_WITH_ICON: f32 = ICON_PADDING + ICON_SIZE + HORIZONTAL_PADDING;
/// Duration of the label float animation (`MotionTokens.DurationShort3`).
pub const LABEL_ANIMATION_SECONDS: f32 = DURATION_SHORT3.as_millis() as f32 / 1000.0;
/// Gap between the floated outlined label and the ends of its notch.
const NOTCH_PADDING: f32 = 4.0;
/// Space between the container and the supporting text.
const SUPPORTING_TOP_PADDING: f32 = 4.0;
/// Space between a prefix or suffix and the text.
const PREFIX_SUFFIX_PADDING: f32 = 2.0;

/// Material Design text input field following Material Design 3 specifications
///
/// Provides text input with two visual variants (filled and outlined) and support for
/// labels, helper text, error states, icons, prefix and suffix text, multiple lines and
/// character counting.
///
/// ## Usage Examples
/// ```rust
/// # use egui_material3::MaterialInput;
/// # egui::__run_test_ui(|ui| {
/// let mut text = String::new();
///
/// // Basic filled input
/// ui.add(MaterialInput::filled(&mut text));
///
/// // Outlined input with label
/// ui.add(MaterialInput::outlined(&mut text)
///     .label("Email")
///     .helper_text("Enter your email address"));
///
/// // Input with error state
/// let mut email = String::new();
/// let invalid = !email.contains('@');
/// ui.add(MaterialInput::filled(&mut email)
///     .label("Email")
///     .error_text("Invalid email format")
///     .is_error(invalid));
///
/// // A clickable trailing icon
/// let mut query = String::from("rust");
/// let output = MaterialInput::outlined(&mut query)
///     .label("Search")
///     .trailing_icon_button("close", "Clear")
///     .show(ui);
/// if output.trailing_icon_clicked {
///     query.clear();
/// }
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialInput<'a> {
    text: &'a mut String,
    variant: InputVariant,
    label: Option<String>,
    placeholder: Option<String>,
    helper_text: Option<String>,
    error_text: Option<String>,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    trailing_icon_label: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
    enabled: bool,
    readonly: bool,
    is_error: bool,
    password: bool,
    max_length: Option<usize>,
    show_character_count: bool,
    multiline: bool,
    min_lines: usize,
    width: Option<f32>,
    id_salt: Option<egui::IdSalt>,
}

/// Visual variants for Material Design text inputs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputVariant {
    /// Filled input with surface background and bottom border
    Filled,
    /// Outlined input with transparent background and full border
    Outlined,
}

/// What [`MaterialInput::show`] returns.
pub struct TextFieldResponse {
    /// The field's response: its id is the inner `TextEdit`'s, so
    /// `has_focus`, `changed` and `request_focus` refer to the text, and its
    /// rect is the container.
    pub response: Response,
    /// The trailing icon set with [`MaterialInput::trailing_icon_button`]
    /// was clicked this frame.
    pub trailing_icon_clicked: bool,
}

impl<'a> MaterialInput<'a> {
    /// Create a new filled Material Design text input (default variant)
    pub fn filled(text: &'a mut String) -> Self {
        Self::new(text, InputVariant::Filled)
    }

    /// Create a new outlined Material Design text input
    pub fn outlined(text: &'a mut String) -> Self {
        Self::new(text, InputVariant::Outlined)
    }

    fn new(text: &'a mut String, variant: InputVariant) -> Self {
        Self {
            text,
            variant,
            label: None,
            placeholder: None,
            helper_text: None,
            error_text: None,
            leading_icon: None,
            trailing_icon: None,
            trailing_icon_label: None,
            prefix: None,
            suffix: None,
            enabled: true,
            readonly: false,
            is_error: false,
            password: false,
            max_length: None,
            show_character_count: false,
            multiline: false,
            min_lines: 1,
            width: None,
            id_salt: None,
        }
    }

    /// Set the floating label text
    ///
    /// The label rests inside the field and floats above the text when the
    /// field is focused or has text.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the placeholder text
    ///
    /// Shown while the field is empty: always without a label, and once the
    /// label has floated (the field is focused) with one.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the helper text shown below the input
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Set the error text shown below the input
    ///
    /// Takes the place of the helper text when `is_error` is true.
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    /// Set the leading icon (Material Symbol name)
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set the trailing icon (Material Symbol name), drawn as decoration.
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set a clickable trailing icon: a button named `label` for screen
    /// readers. [`TextFieldResponse::trailing_icon_clicked`] reports clicks
    /// (use [`Self::show`] instead of `ui.add`).
    pub fn trailing_icon_button(
        mut self,
        icon: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        self.trailing_icon = Some(icon.into());
        self.trailing_icon_label = Some(label.into());
        self
    }

    /// Text shown before the input once the label has floated, such as `$`.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Text shown after the input once the label has floated, such as `kg`.
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set whether the input is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set whether the input is read-only: it can be focused, selected and
    /// copied, but not edited.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Set whether to show error state styling
    pub fn is_error(mut self, is_error: bool) -> Self {
        self.is_error = is_error;
        self
    }

    /// Set whether to mask the text (password field)
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Set the maximum length in characters (not bytes). Typing stops at the
    /// limit, and longer text set from outside is cut on a character
    /// boundary.
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Set whether to show character count below input
    pub fn show_character_count(mut self, show: bool) -> Self {
        self.show_character_count = show;
        self
    }

    /// Alias of [`Self::placeholder`], kept for compatibility.
    pub fn hint(self, hint: impl Into<String>) -> Self {
        self.placeholder(hint)
    }

    /// Accept several lines: Enter inserts a newline and the field grows
    /// with its text.
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// The fewest lines a multi-line field shows (default 1).
    pub fn min_lines(mut self, lines: usize) -> Self {
        self.min_lines = lines.max(1);
        self
    }

    /// A fixed width; by default the field fills the available width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Derive the field's ids from `salt` instead of its position in the
    /// layout, so its focus and animation state follow it when other
    /// widgets are added or removed before it.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = Some(egui::IdSalt::new(salt));
        self
    }

    /// Show the field, returning its response and whether the trailing icon
    /// button was clicked.
    pub fn show(self, ui: &mut Ui) -> TextFieldResponse {
        let Self {
            text,
            variant,
            label,
            placeholder,
            helper_text,
            error_text,
            leading_icon,
            trailing_icon,
            trailing_icon_label,
            prefix,
            suffix,
            enabled,
            readonly,
            is_error,
            password,
            max_length,
            show_character_count,
            multiline,
            min_lines,
            width,
            id_salt,
        } = self;

        let ctx = ui.ctx().clone();
        let theme = MaterialTheme::of(&ctx);
        let enabled = enabled && ui.is_enabled();
        let body = TypeRole::BodyLarge.style();
        let font = body.font_id(&ctx);

        // Horizontal layout: icons push the text in to 52 dp.
        let width = width.unwrap_or_else(|| ui.available_width());
        let text_start = if leading_icon.is_some() {
            TEXT_PADDING_WITH_ICON
        } else {
            HORIZONTAL_PADDING
        };
        let text_end = if trailing_icon.is_some() {
            TEXT_PADDING_WITH_ICON
        } else {
            HORIZONTAL_PADDING
        };
        let measure = |s: &str| {
            ui.painter()
                .layout_no_wrap(s.to_owned(), font.clone(), Color32::WHITE)
                .size()
                .x
        };
        let prefix_width = prefix
            .as_deref()
            .map_or(0.0, |p| measure(p) + PREFIX_SUFFIX_PADDING);
        let suffix_width = suffix
            .as_deref()
            .map_or(0.0, |s| measure(s) + PREFIX_SUFFIX_PADDING);
        let input_width = (width - text_start - text_end - prefix_width - suffix_width).max(1.0);

        // Vertical layout: a filled field with a label puts the input under
        // the floated label (8 dp + 16 dp label line); otherwise it is
        // centred with 16 dp above and below.
        let has_label = label.is_some();
        let (top_padding, bottom_padding) = if variant == InputVariant::Filled && has_label {
            (24.0, 8.0)
        } else {
            (16.0, 16.0)
        };
        let input_height = if multiline {
            let galley =
                ui.painter()
                    .layout(text.clone(), font.clone(), Color32::WHITE, input_width);
            let row = ui
                .painter()
                .layout_no_wrap("A".into(), font.clone(), Color32::WHITE);
            let rows = galley.rows.len().max(min_lines) as f32;
            (rows * row.size().y).max(body.line_height)
        } else {
            body.line_height
        };
        let container_height = (top_padding + input_height + bottom_padding).max(CONTAINER_HEIGHT);

        let supporting = if is_error {
            error_text.or(helper_text)
        } else {
            helper_text
        };
        let counter = show_character_count.then(|| {
            let count = text.chars().count();
            match max_length {
                Some(max) => format!("{count}/{max}"),
                None => count.to_string(),
            }
        });
        let supporting_height = if supporting.is_some() || counter.is_some() {
            SUPPORTING_TOP_PADDING + TypeRole::BodySmall.style().line_height
        } else {
            0.0
        };

        let (total_rect, allocated) = ui.allocate_exact_size(
            Vec2::new(width, container_height + supporting_height),
            Sense::hover(),
        );
        let id = id_salt.map_or(allocated.id, |salt| ui.make_persistent_id(salt));
        let edit_id = id.with("text_edit");
        let container_rect =
            Rect::from_min_size(total_rect.min, Vec2::new(width, container_height));
        // Clicks on the container outside the text focus the text. Not
        // focusable itself: the text is the one tab stop.
        let container = ui.interact(
            container_rect,
            id.with("container"),
            if enabled {
                Sense::CLICK
            } else {
                Sense::hover()
            },
        );
        if container.clicked() {
            ctx.memory_mut(|m| m.request_focus(edit_id));
        }
        let was_focused = ctx.memory(|m| m.has_focus(edit_id));

        // Colours that do not depend on this frame's focus or hover.
        let base = field_colors(
            &theme,
            variant,
            FieldState {
                enabled,
                error: is_error,
                focused: false,
                hovered: false,
            },
        );

        // The container goes under the text, which is laid out first so its
        // focus and hover are this frame's.
        let container_slot = ui.painter().add(egui::Shape::Noop);

        let input_rect = Rect::from_min_size(
            Pos2::new(
                container_rect.left() + text_start + prefix_width,
                container_rect.top() + top_padding,
            ),
            Vec2::new(input_width, input_height),
        );
        let show_placeholder = !has_label || was_focused;
        let text_response = {
            // A child that does not allocate: the field already holds its space.
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(input_rect));
            let ui = &mut child;
            let visuals = ui.visuals_mut();
            visuals.text_cursor.stroke.color = base.caret;
            visuals.selection.bg_fill = base.caret.gamma_multiply(0.4);
            visuals.selection.stroke.color = base.input;
            let options = EditOptions {
                id: edit_id,
                font: font.clone(),
                color: base.input,
                size: input_rect.size(),
                multiline,
                min_lines,
                enabled,
                password,
                max_length,
                placeholder: placeholder
                    .as_deref()
                    .filter(|_| show_placeholder)
                    .map(|p| RichText::new(p).font(font.clone()).color(base.placeholder)),
            };
            if readonly {
                let mut view: &str = text.as_str();
                ui.add(options.build(&mut view))
            } else {
                ui.add(options.build(text))
            }
        };

        // Text set from outside can be longer than the limit; cut it on a
        // character boundary (a byte index could split a character).
        if let Some(max) = max_length {
            if let Some((cut, _)) = text.char_indices().nth(max) {
                text.truncate(cut);
            }
        }

        let painter = ui.painter().clone();
        let leading_center = Pos2::new(
            container_rect.left() + ICON_PADDING + ICON_SIZE / 2.0,
            container_rect.center().y,
        );
        let trailing_center = Pos2::new(
            container_rect.right() - ICON_PADDING - ICON_SIZE / 2.0,
            container_rect.center().y,
        );

        let mut trailing_icon_clicked = false;
        let mut trailing_hovered = false;
        let trailing_button = match (&trailing_icon, &trailing_icon_label) {
            (Some(_), Some(button_label)) => {
                let rect = Rect::from_center_size(trailing_center, Vec2::splat(40.0));
                let response = ui.interact(
                    rect,
                    id.with("trailing_icon"),
                    crate::a11y::sense(enabled, Sense::click()),
                );
                crate::a11y::button(&response, enabled, button_label);
                trailing_icon_clicked = response.clicked();
                trailing_hovered = response.hovered();
                Some(response)
            }
            _ => None,
        };

        let focused = text_response.has_focus();
        let hovered =
            enabled && (container.hovered() || text_response.hovered() || trailing_hovered);
        let colors = field_colors(
            &theme,
            variant,
            FieldState {
                enabled,
                error: is_error,
                focused,
                hovered,
            },
        );
        let floated = focused || !text.is_empty();
        let progress = if has_label {
            label_progress(&ctx, id.with("label"), floated)
        } else {
            1.0
        };

        // Label, and the outlined notch it floats into.
        let label_geometry = LabelGeometry {
            rest: Pos2::new(
                container_rect.left() + text_start,
                container_rect.top() + CONTAINER_HEIGHT / 2.0,
            ),
            floated: floated_label_pos(variant, container_rect, text_start),
        };
        let notch = match (&label, variant) {
            (Some(label), InputVariant::Outlined) => {
                notch_for(&painter, &ctx, label, label_geometry.floated, progress)
            }
            _ => None,
        };
        painter.set(
            container_slot,
            container_shape(container_rect, variant, &colors, notch),
        );
        if let Some(label) = &label {
            paint_label(
                &painter,
                &ctx,
                label,
                label_geometry,
                progress,
                colors.label,
            );
        }

        // Prefix and suffix fade in with the label.
        let affix_color = colors.affix.gamma_multiply(progress);
        let affix_y = if multiline {
            input_rect.top() + body.line_height / 2.0
        } else {
            input_rect.center().y
        };
        if let Some(prefix) = &prefix {
            painter.text(
                Pos2::new(input_rect.left() - prefix_width, affix_y),
                Align2::LEFT_CENTER,
                prefix,
                font.clone(),
                affix_color,
            );
        }
        if let Some(suffix) = &suffix {
            painter.text(
                Pos2::new(input_rect.right() + PREFIX_SUFFIX_PADDING, affix_y),
                Align2::LEFT_CENTER,
                suffix,
                font.clone(),
                affix_color,
            );
        }

        if let Some(icon) = &leading_icon {
            paint_icon(&painter, leading_center, icon, colors.leading_icon);
        }
        if let Some(icon) = &trailing_icon {
            if let Some(button) = &trailing_button {
                paint_state_layer(
                    &painter,
                    button.rect,
                    20,
                    colors.trailing_icon,
                    InteractionState::from_response(button),
                );
            }
            paint_icon(&painter, trailing_center, icon, colors.trailing_icon);
        }

        if supporting_height > 0.0 {
            let row = Rect::from_min_max(
                Pos2::new(
                    total_rect.left(),
                    container_rect.bottom() + SUPPORTING_TOP_PADDING,
                ),
                total_rect.max,
            );
            paint_supporting(
                &painter,
                &ctx,
                row,
                supporting.as_deref(),
                counter.as_deref(),
                colors.supporting,
            );
        }

        // Screen readers name the text by the label painted over it, and
        // hear that a disabled field is disabled.
        let name = label.clone();
        ctx.accesskit_node_builder(text_response.id, |node| {
            if let Some(name) = name {
                node.set_label(name);
            }
            if !enabled {
                node.set_disabled();
            }
        });

        TextFieldResponse {
            response: text_response.union(container),
            trailing_icon_clicked,
        }
    }
}

impl Widget for MaterialInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

/// How the inner `TextEdit` is configured.
struct EditOptions {
    id: Id,
    font: FontId,
    color: Color32,
    size: Vec2,
    multiline: bool,
    min_lines: usize,
    enabled: bool,
    password: bool,
    max_length: Option<usize>,
    placeholder: Option<RichText>,
}

impl EditOptions {
    fn build<'t>(self, buffer: &'t mut dyn TextBuffer) -> TextEdit<'t> {
        let mut edit = if self.multiline {
            TextEdit::multiline(buffer)
                .desired_rows(self.min_lines)
                .align(Align2::LEFT_TOP)
        } else {
            TextEdit::singleline(buffer).align(Align2::LEFT_CENTER)
        };
        edit = edit
            .id(self.id)
            .font(self.font)
            .text_color(self.color)
            .frame(egui::Frame::NONE)
            .margin(egui::Margin::ZERO)
            .desired_width(self.size.x)
            .min_size(self.size)
            .password(self.password)
            .interactive(self.enabled);
        if let Some(max) = self.max_length {
            edit = edit.char_limit(max);
        }
        if let Some(placeholder) = self.placeholder {
            edit = edit.hint_text(placeholder);
        }
        edit
    }
}

// ---------------------------------------------------------------------------
// Field decoration shared with `MaterialSelect`.
// ---------------------------------------------------------------------------

/// The interaction state of a text-field-style container.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct FieldState {
    pub(crate) enabled: bool,
    pub(crate) error: bool,
    pub(crate) focused: bool,
    pub(crate) hovered: bool,
}

/// Every colour of a field in one state, from the text field tokens.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FieldColors {
    pub(crate) container: Color32,
    pub(crate) indicator: Color32,
    pub(crate) indicator_width: f32,
    pub(crate) label: Color32,
    pub(crate) input: Color32,
    pub(crate) placeholder: Color32,
    pub(crate) affix: Color32,
    pub(crate) leading_icon: Color32,
    pub(crate) trailing_icon: Color32,
    pub(crate) supporting: Color32,
    pub(crate) caret: Color32,
}

/// The token colours of a `variant` field in `state`.
pub(crate) fn field_colors(
    theme: &MaterialTheme,
    variant: InputVariant,
    state: FieldState,
) -> FieldColors {
    let c = |role| theme.color(role);
    let on_surface = c(ColorRole::OnSurface);
    let on_surface_variant = c(ColorRole::OnSurfaceVariant);
    let disabled = on_surface.gamma_multiply(DISABLED_CONTENT_OPACITY);
    let error = c(ColorRole::Error);
    let error_hover = c(ColorRole::OnErrorContainer);
    // Hover only shows while unfocused.
    let hovered = state.hovered && !state.focused;
    let filled = variant == InputVariant::Filled;

    if !state.enabled {
        return FieldColors {
            container: if filled {
                on_surface.gamma_multiply(FILLED_DISABLED_CONTAINER_OPACITY)
            } else {
                Color32::TRANSPARENT
            },
            indicator: if filled {
                disabled
            } else {
                on_surface.gamma_multiply(OUTLINED_DISABLED_OUTLINE_OPACITY)
            },
            indicator_width: if filled {
                FILLED_INDICATOR_WIDTH
            } else {
                OUTLINE_WIDTH
            },
            label: disabled,
            input: disabled,
            placeholder: disabled,
            affix: disabled,
            leading_icon: disabled,
            trailing_icon: disabled,
            supporting: disabled,
            caret: c(ColorRole::Primary),
        };
    }

    let indicator = if state.error {
        if state.focused {
            error
        } else if hovered {
            error_hover
        } else {
            error
        }
    } else if state.focused {
        c(ColorRole::Primary)
    } else if hovered {
        on_surface
    } else if filled {
        on_surface_variant
    } else {
        c(ColorRole::Outline)
    };
    let indicator_width = match (filled, state.focused) {
        (true, true) => FILLED_FOCUS_INDICATOR_WIDTH,
        (true, false) => FILLED_INDICATOR_WIDTH,
        (false, true) => OUTLINE_FOCUS_WIDTH,
        (false, false) => OUTLINE_WIDTH,
    };
    let label = if state.error {
        if hovered {
            error_hover
        } else {
            error
        }
    } else if state.focused {
        c(ColorRole::Primary)
    } else if hovered && !filled {
        // OutlinedTextFieldTokens.HoverLabelColor is onSurface; filled keeps
        // onSurfaceVariant.
        on_surface
    } else {
        on_surface_variant
    };
    let trailing_icon = if state.error {
        if hovered {
            error_hover
        } else {
            error
        }
    } else {
        on_surface_variant
    };
    FieldColors {
        container: if filled {
            c(ColorRole::SurfaceContainerHighest)
        } else {
            Color32::TRANSPARENT
        },
        indicator,
        indicator_width,
        label,
        input: on_surface,
        placeholder: on_surface_variant,
        affix: on_surface_variant,
        leading_icon: on_surface_variant,
        trailing_icon,
        supporting: if state.error {
            error
        } else {
            on_surface_variant
        },
        caret: if state.error {
            error
        } else {
            c(ColorRole::Primary)
        },
    }
}

/// The label's left-centre point at rest and when floated.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LabelGeometry {
    pub(crate) rest: Pos2,
    pub(crate) floated: Pos2,
}

/// Where the label's left-centre point sits once floated: in the top 8 dp +
/// 16 dp line of a filled field, or centred on the top edge of an outlined
/// one, at the container's 16 dp start padding.
pub(crate) fn floated_label_pos(variant: InputVariant, container: Rect, text_start: f32) -> Pos2 {
    match variant {
        InputVariant::Filled => Pos2::new(container.left() + text_start, container.top() + 16.0),
        InputVariant::Outlined => Pos2::new(container.left() + HORIZONTAL_PADDING, container.top()),
    }
}

fn emphasized(t: f32) -> f32 {
    CubicBezier::EMPHASIZED.transform(t)
}

/// The label float progress (0 resting, 1 floated), animated over 150 ms on
/// the emphasized easing curve.
pub(crate) fn label_progress(ctx: &egui::Context, id: Id, floated: bool) -> f32 {
    ctx.animate_bool_with_time_and_easing(id, floated, LABEL_ANIMATION_SECONDS, emphasized)
}

/// The label font size at `progress`: BodyLarge resting, BodySmall floated.
pub(crate) fn label_size(progress: f32) -> f32 {
    let rest = TypeRole::BodyLarge.style().size;
    let floated = TypeRole::BodySmall.style().size;
    rest + (floated - rest) * progress
}

/// Paints the label at `progress` between its resting and floated places.
pub(crate) fn paint_label(
    painter: &egui::Painter,
    ctx: &egui::Context,
    label: &str,
    geometry: LabelGeometry,
    progress: f32,
    color: Color32,
) {
    let family = TypeRole::BodyLarge.style().font_id(ctx).family;
    let pos = geometry.rest.lerp(geometry.floated, progress);
    painter.text(
        pos,
        Align2::LEFT_CENTER,
        label,
        FontId::new(label_size(progress), family),
        color,
    );
}

/// The gap in the outline for a label floated at `floated`, opening with
/// `progress`.
pub(crate) fn notch_for(
    painter: &egui::Painter,
    ctx: &egui::Context,
    label: &str,
    floated: Pos2,
    progress: f32,
) -> Option<(f32, f32)> {
    if progress <= 0.0 {
        return None;
    }
    let family = TypeRole::BodyLarge.style().font_id(ctx).family;
    let width = painter
        .layout_no_wrap(
            label.to_owned(),
            FontId::new(label_size(1.0), family),
            Color32::WHITE,
        )
        .size()
        .x;
    let start = floated.x - NOTCH_PADDING;
    Some((start, start + (width + 2.0 * NOTCH_PADDING) * progress))
}

/// The container: a filled background with its bottom indicator, or an
/// outline (with an optional notch `(start x, end x)` for the label).
pub(crate) fn container_shape(
    rect: Rect,
    variant: InputVariant,
    colors: &FieldColors,
    notch: Option<(f32, f32)>,
) -> egui::Shape {
    match variant {
        InputVariant::Filled => {
            let radius =
                crate::tokens::Shape::EXTRA_SMALL_TOP.corner_radius(rect, LayoutDirection::Ltr);
            let indicator = Rect::from_min_max(
                Pos2::new(rect.left(), rect.bottom() - colors.indicator_width),
                rect.max,
            );
            egui::Shape::Vec(vec![
                egui::Shape::rect_filled(rect, radius, colors.container),
                egui::Shape::rect_filled(indicator, 0.0, colors.indicator),
            ])
        }
        InputVariant::Outlined => {
            let radius = crate::tokens::Shape::EXTRA_SMALL
                .corner_radius(rect, LayoutDirection::Ltr)
                .nw as f32;
            outline_shape(
                rect,
                radius,
                Stroke::new(colors.indicator_width, colors.indicator),
                notch,
            )
        }
    }
}

/// A rounded-rectangle outline drawn inside `rect`, open between the notch
/// x coordinates on the top edge.
fn outline_shape(
    rect: Rect,
    radius: f32,
    stroke: Stroke,
    notch: Option<(f32, f32)>,
) -> egui::Shape {
    const ARC_SEGMENTS: usize = 6;
    let inner = rect.shrink(stroke.width / 2.0);
    let r = (radius - stroke.width / 2.0).clamp(0.0, inner.height().min(inner.width()) / 2.0);
    let arc = |points: &mut Vec<Pos2>, center: Pos2, from_deg: f32| {
        for i in 0..=ARC_SEGMENTS {
            let angle = (from_deg + 90.0 * i as f32 / ARC_SEGMENTS as f32).to_radians();
            points.push(center + r * Vec2::new(angle.cos(), angle.sin()));
        }
    };
    let notch = notch
        .map(|(a, b)| (a.max(inner.left() + r), b.min(inner.right() - r)))
        .filter(|(a, b)| b - a > 0.5);
    let mut points = Vec::with_capacity(4 * (ARC_SEGMENTS + 1) + 2);
    if let Some((_, end)) = notch {
        points.push(Pos2::new(end, inner.top()));
    }
    arc(
        &mut points,
        Pos2::new(inner.right() - r, inner.top() + r),
        -90.0,
    );
    arc(
        &mut points,
        Pos2::new(inner.right() - r, inner.bottom() - r),
        0.0,
    );
    arc(
        &mut points,
        Pos2::new(inner.left() + r, inner.bottom() - r),
        90.0,
    );
    arc(
        &mut points,
        Pos2::new(inner.left() + r, inner.top() + r),
        180.0,
    );
    match notch {
        Some((start, _)) => {
            points.push(Pos2::new(start, inner.top()));
            egui::Shape::Path(PathShape::line(points, stroke))
        }
        None => egui::Shape::Path(PathShape::closed_line(points, stroke)),
    }
}

/// Paints a 24 dp Material Symbol centred on `center`.
pub(crate) fn paint_icon(painter: &egui::Painter, center: Pos2, name: &str, color: Color32) {
    painter.text(
        center,
        Align2::CENTER_CENTER,
        crate::material_symbol::material_symbol_text(name),
        FontId::proportional(ICON_SIZE),
        color,
    );
}

/// Paints supporting text at the start and a counter at the end of `row`,
/// in BodySmall, inset by the 16 dp field padding.
pub(crate) fn paint_supporting(
    painter: &egui::Painter,
    ctx: &egui::Context,
    row: Rect,
    text: Option<&str>,
    counter: Option<&str>,
    color: Color32,
) {
    let font = TypeRole::BodySmall.style().font_id(ctx);
    if let Some(text) = text {
        painter.text(
            Pos2::new(row.left() + HORIZONTAL_PADDING, row.top()),
            Align2::LEFT_TOP,
            text,
            font.clone(),
            color,
        );
    }
    if let Some(counter) = counter {
        painter.text(
            Pos2::new(row.right() - HORIZONTAL_PADDING, row.top()),
            Align2::RIGHT_TOP,
            counter,
            font,
            color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indicator_widths_follow_the_tokens() {
        let theme = MaterialTheme::baseline(false);
        let width = |variant, focused| {
            let state = FieldState {
                enabled: true,
                focused,
                ..Default::default()
            };
            field_colors(&theme, variant, state).indicator_width
        };
        assert_eq!(width(InputVariant::Filled, false), 1.0, "filled unfocused");
        assert_eq!(width(InputVariant::Filled, true), 2.0, "filled focused");
        assert_eq!(
            width(InputVariant::Outlined, false),
            1.0,
            "outlined unfocused"
        );
        assert_eq!(width(InputVariant::Outlined, true), 3.0, "outlined focused");
    }

    #[test]
    fn colours_follow_the_tokens() {
        let theme = MaterialTheme::baseline(false);
        let colors = |variant, state| field_colors(&theme, variant, state);
        let idle = FieldState {
            enabled: true,
            ..Default::default()
        };
        let focused = FieldState {
            focused: true,
            ..idle
        };
        let hovered = FieldState {
            hovered: true,
            ..idle
        };
        let error = FieldState {
            error: true,
            ..idle
        };
        let error_hovered = FieldState {
            hovered: true,
            ..error
        };
        let filled = InputVariant::Filled;
        let outlined = InputVariant::Outlined;
        let c = |role| theme.color(role);
        assert_eq!(
            colors(filled, idle).container,
            c(ColorRole::SurfaceContainerHighest),
            "filled container"
        );
        assert_eq!(
            colors(outlined, idle).container,
            Color32::TRANSPARENT,
            "outlined container"
        );
        assert_eq!(
            colors(filled, idle).indicator,
            c(ColorRole::OnSurfaceVariant),
            "filled indicator"
        );
        assert_eq!(
            colors(outlined, idle).indicator,
            c(ColorRole::Outline),
            "outline"
        );
        assert_eq!(
            colors(outlined, hovered).indicator,
            c(ColorRole::OnSurface),
            "hover outline"
        );
        assert_eq!(
            colors(outlined, hovered).label,
            c(ColorRole::OnSurface),
            "outlined hover label"
        );
        assert_eq!(
            colors(filled, hovered).label,
            c(ColorRole::OnSurfaceVariant),
            "filled hover label"
        );
        assert_eq!(
            colors(filled, focused).indicator,
            c(ColorRole::Primary),
            "focus indicator"
        );
        assert_eq!(
            colors(filled, focused).label,
            c(ColorRole::Primary),
            "focus label"
        );
        assert_eq!(
            colors(filled, error).indicator,
            c(ColorRole::Error),
            "error indicator"
        );
        assert_eq!(
            colors(filled, error).caret,
            c(ColorRole::Error),
            "error caret"
        );
        assert_eq!(colors(filled, idle).caret, c(ColorRole::Primary), "caret");
        assert_eq!(
            colors(outlined, error_hovered).indicator,
            c(ColorRole::OnErrorContainer),
            "error hover outline"
        );
        let disabled = FieldState::default();
        assert_eq!(
            colors(filled, disabled).container,
            c(ColorRole::OnSurface).gamma_multiply(0.04),
            "disabled container"
        );
        assert_eq!(
            colors(outlined, disabled).indicator,
            c(ColorRole::OnSurface).gamma_multiply(0.12),
            "disabled outline"
        );
        assert_eq!(
            colors(filled, disabled).input,
            c(ColorRole::OnSurface).gamma_multiply(0.38),
            "disabled text"
        );
    }

    #[test]
    fn label_shrinks_from_body_large_to_body_small() {
        assert_eq!(label_size(0.0), 16.0, "resting label is BodyLarge");
        assert_eq!(label_size(1.0), 12.0, "floated label is BodySmall");
        assert!(
            (LABEL_ANIMATION_SECONDS - 0.15).abs() < 1e-6,
            "label floats over 150 ms"
        );
    }

    #[test]
    fn icon_slots_put_text_at_52() {
        assert_eq!(
            TEXT_PADDING_WITH_ICON, 52.0,
            "text starts at 52 dp next to an icon"
        );
        assert_eq!(ICON_PADDING, 12.0, "icons sit 12 dp from the edge");
    }

    #[test]
    fn notched_outline_is_open_and_plain_outline_is_closed() -> Result<(), String> {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 56.0));
        let stroke = Stroke::new(1.0, Color32::BLACK);
        let open = outline_shape(rect, 4.0, stroke, Some((12.0, 60.0)));
        let closed = outline_shape(rect, 4.0, stroke, None);
        let (egui::Shape::Path(open), egui::Shape::Path(closed)) = (open, closed) else {
            return Err("outlines are paths".to_owned());
        };
        assert!(!open.closed, "a notched outline is an open path");
        assert!(closed.closed, "an outline without a label is closed");
        let gap_end = open.points.first().map(|p| p.x);
        let gap_start = open.points.last().map(|p| p.x);
        assert_eq!(gap_end, Some(60.0), "path starts at the notch end");
        assert_eq!(gap_start, Some(12.0), "path ends at the notch start");
        Ok(())
    }
}
