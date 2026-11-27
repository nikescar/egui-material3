//! Material Design 3 dialogs: the basic dialog and the full-screen dialog.
//!
//! # Tokens
//!
//! Basic dialog values come from Compose's `DialogTokens.kt` and the
//! layout constants in `AlertDialog.kt`:
//!
//! - container `SurfaceContainerHigh`, shape `CornerExtraLarge` (28 dp),
//!   elevation `Level3` (`DialogTokens.ContainerColor`, `ContainerShape`,
//!   `ContainerElevation`);
//! - headline `HeadlineSmall` in `OnSurface`; supporting text
//!   `BodyMedium` in `OnSurfaceVariant`; icon 24 dp in `Secondary`;
//!   actions `LabelLarge` in `Primary` (`DialogTokens.Headline*`,
//!   `SupportingText*`, `Icon*`, `ActionLabelText*`);
//! - width 280–560 dp (`DialogMinWidth`, `DialogMaxWidth`), 24 dp padding
//!   (`DialogPadding`), 16 dp under the icon and the headline
//!   (`IconPadding`, `TitlePadding`), 24 dp between the text and the
//!   actions (`TextPadding`), 8 dp between actions
//!   (`ButtonsMainAxisSpacing`);
//! - the scrim is `ScrimTokens` (see [`crate::scrim`]).
//!
//! Full-screen dialog values come from `FullScreenDialogTokens.kt`:
//! container `Surface` at `Level0`, a 56 dp header with a `TitleLarge`
//! headline in `OnSurface`, a 24 dp `OnSurface` close icon and a
//! `LabelLarge` action in `Primary`.
//!
//! # Behaviour
//!
//! A dialog is modal: it sits in egui's modal layer, so the app behind it
//! neither gets pointer input nor keyboard focus, and Tab cycles through
//! the dialog's own widgets only. Opening it takes focus from the app;
//! closing it gives focus back to the widget that had it. Escape closes
//! the top dialog only. It enters and leaves with emphasized motion
//! ([`Transition::EMPHASIZED`]).
//!
//! [`MaterialDialog::show`] returns a [`DialogResponse`]: what the content
//! closure returned, and why the dialog closed this frame, if it did.

use egui::{Align, Context, Id, Layout, Response, RichText, Ui, Vec2};

use crate::scrim::{self, FocusReturn, Transition};
use crate::tokens::{ColorRole, CornerSize, Elevation, TypeRole};
use crate::{MaterialButton, MaterialTheme};

/// `AlertDialog.kt` `DialogMinWidth`.
pub const DIALOG_MIN_WIDTH: f32 = 280.0;
/// `AlertDialog.kt` `DialogMaxWidth`.
pub const DIALOG_MAX_WIDTH: f32 = 560.0;
/// `AlertDialog.kt` `DialogPadding`.
const DIALOG_PADDING: f32 = 24.0;
/// `AlertDialog.kt` `IconPadding` and `TitlePadding` (bottom).
const ICON_TITLE_GAP: f32 = 16.0;
/// `AlertDialog.kt` `TextPadding` (bottom): text to actions.
const TEXT_ACTIONS_GAP: f32 = 24.0;
/// `AlertDialog.kt` `ButtonsMainAxisSpacing`.
const ACTIONS_SPACING: f32 = 8.0;
/// `DialogTokens.IconSize`.
const ICON_SIZE: f32 = 24.0;
/// Side of the close button's touch target.
const CLOSE_BUTTON_SIZE: f32 = 48.0;

/// Material Design dialog types.
///
/// Material 3 has one basic dialog; the type only changes its default
/// width. [`DialogType::Form`] takes the full maximum width (forms read
/// better wide); the others size to their content within 280–560 dp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogType {
    /// Standard dialog for general purpose use
    Standard,
    /// Alert dialog for important notifications requiring acknowledgment
    Alert,
    /// Confirmation dialog for confirming actions before proceeding
    Confirm,
    /// Form dialog containing input fields and form elements
    Form,
}

/// Why a dialog closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DialogCloseReason {
    /// Escape was pressed while it was the top modal.
    Escape,
    /// The scrim outside the dialog was clicked.
    Scrim,
    /// The action button at this index (in the order the actions were
    /// added) was clicked. A full-screen dialog's action is index 0.
    Action(usize),
    /// The close button was clicked.
    CloseButton,
    /// The content closure set the open flag to `false` (or called
    /// `ui.close()`).
    Content,
}

/// What [`MaterialDialog::show`] and [`FullScreenDialog::show`] report.
#[derive(Debug)]
pub struct DialogResponse<R> {
    /// What the content closure returned, when the dialog was open and
    /// interactive this frame. `None` while closed or animating out.
    pub inner: Option<R>,
    /// Why the dialog closed this frame, if it did. A dialog closed by the
    /// caller (setting the flag to `false`) reports nothing.
    pub close_reason: Option<DialogCloseReason>,
    /// The dialog surface, when it was drawn this frame.
    pub response: Option<Response>,
}

impl<R> DialogResponse<R> {
    fn hidden() -> Self {
        Self {
            inner: None,
            close_reason: None,
            response: None,
        }
    }

    /// The dialog closed this frame.
    pub fn closed(&self) -> bool {
        self.close_reason.is_some()
    }
}

/// Renders the dialog body; receives the UI and the dialog's open flag.
type DialogContentFn<'a, R> = Box<dyn FnOnce(&mut Ui, &mut bool) -> R + 'a>;

/// Material Design 3 basic dialog.
///
/// Dialogs interrupt users with overlaid content that requires a response.
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{dialog::DialogCloseReason, MaterialDialog};
///
/// let mut open = true;
/// let mut name = String::new();
/// let response = MaterialDialog::new(egui::Id::unique("rename"), "Rename", &mut open)
///     .close_button(true)
///     .content(|ui, _open| ui.text_edit_singleline(&mut name).changed())
///     .text_action("Cancel", || {})
///     .filled_action("Rename", || {})
///     .show(ui.ctx());
/// if response.close_reason == Some(DialogCloseReason::Action(1)) {
///     // Rename confirmed.
/// }
/// let _edited = response.inner.unwrap_or(false);
/// # });
/// ```
pub struct MaterialDialog<'a, R = ()> {
    id: Id,
    title: String,
    open: &'a mut bool,
    dialog_type: DialogType,
    icon: Option<String>,
    content: DialogContentFn<'a, R>,
    actions: Vec<DialogAction<'a>>,
    quick: bool,
    no_focus_trap: bool,
    close_button: bool,
    dismiss_on_escape: bool,
    dismiss_on_scrim: bool,
    max_width: Option<f32>,
    min_width: Option<f32>,
    max_height: Option<f32>,
    title_padding: Option<[f32; 4]>,
    content_padding: Option<[f32; 4]>,
    actions_padding: Option<[f32; 4]>,
    button_padding: Option<[f32; 2]>,
    scrollable: bool,
    actions_spacing: f32,
}

/// An action button in a Material Design dialog.
pub struct DialogAction<'a> {
    text: String,
    action_type: ActionType,
    action: Box<dyn FnOnce() + 'a>,
}

/// Material Design action button styles for dialogs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionType {
    /// Text button - lowest emphasis, used for secondary actions
    Text,
    /// Filled tonal button - medium emphasis, used for secondary actions
    FilledTonal,
    /// Filled button - highest emphasis, used for primary actions
    Filled,
}

impl<'a> MaterialDialog<'a, ()> {
    /// A new dialog with `title`, shown while `*open` is true.
    ///
    /// `id` must be unique among the dialogs shown at once.
    pub fn new(id: impl Into<Id>, title: impl Into<String>, open: &'a mut bool) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            open,
            dialog_type: DialogType::Standard,
            icon: None,
            content: Box::new(|_, _| ()),
            actions: Vec::new(),
            quick: false,
            no_focus_trap: false,
            close_button: false,
            dismiss_on_escape: true,
            dismiss_on_scrim: true,
            max_width: None,
            min_width: None,
            max_height: None,
            title_padding: None,
            content_padding: None,
            actions_padding: None,
            button_padding: None,
            scrollable: false,
            actions_spacing: ACTIONS_SPACING,
        }
    }
}

impl<'a, R> MaterialDialog<'a, R> {
    /// Set the dialog type (changes the default width).
    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }

    /// An icon above the headline, by Material Symbols name (`"delete"`).
    /// The headline is centred when there is an icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// The dialog body. The closure gets the UI and the dialog's open flag
    /// (set it to `false` to close), and what it returns comes back in
    /// [`DialogResponse::inner`].
    ///
    /// While the dialog animates out the closure is still called, with a
    /// disabled UI, so the surface keeps its content; its return value is
    /// dropped then. Use [`MaterialDialog::quick`] to skip the animation.
    pub fn content<R2, F>(self, content: F) -> MaterialDialog<'a, R2>
    where
        F: FnOnce(&mut Ui, &mut bool) -> R2 + 'a,
    {
        MaterialDialog {
            id: self.id,
            title: self.title,
            open: self.open,
            dialog_type: self.dialog_type,
            icon: self.icon,
            content: Box::new(content),
            actions: self.actions,
            quick: self.quick,
            no_focus_trap: self.no_focus_trap,
            close_button: self.close_button,
            dismiss_on_escape: self.dismiss_on_escape,
            dismiss_on_scrim: self.dismiss_on_scrim,
            max_width: self.max_width,
            min_width: self.min_width,
            max_height: self.max_height,
            title_padding: self.title_padding,
            content_padding: self.content_padding,
            actions_padding: self.actions_padding,
            button_padding: self.button_padding,
            scrollable: self.scrollable,
            actions_spacing: self.actions_spacing,
        }
    }

    /// A quick dialog appears and disappears without animation.
    pub fn quick(mut self, quick: bool) -> Self {
        self.quick = quick;
        self
    }

    /// Leave keyboard focus alone: do not take it from the app on opening,
    /// nor give it back on closing. Tab still only reaches the dialog's
    /// widgets while it is open (egui's modal layer).
    pub fn no_focus_trap(mut self, no_focus_trap: bool) -> Self {
        self.no_focus_trap = no_focus_trap;
        self
    }

    /// Show a close (×) button at the top end corner. A click closes the
    /// dialog with [`DialogCloseReason::CloseButton`].
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }

    /// Whether Escape closes the dialog (default `true`; Compose's
    /// `DialogProperties.dismissOnBackPress`).
    pub fn dismiss_on_escape(mut self, dismiss: bool) -> Self {
        self.dismiss_on_escape = dismiss;
        self
    }

    /// Whether a click on the scrim closes the dialog (default `true`;
    /// Compose's `DialogProperties.dismissOnClickOutside`).
    pub fn dismiss_on_scrim(mut self, dismiss: bool) -> Self {
        self.dismiss_on_scrim = dismiss;
        self
    }

    /// Maximum width (default 560, never more than the window).
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Minimum width (default 280, never more than the maximum).
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Maximum height of a [`scrollable`](Self::scrollable) dialog
    /// (default 90 % of the window, at most 800).
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Padding around the headline, `[left, right, top, bottom]`
    /// (default 24, 24, 24, 16; the top is 0 under an icon).
    pub fn title_padding(mut self, padding: [f32; 4]) -> Self {
        self.title_padding = Some(padding);
        self
    }

    /// Padding around the content, `[left, right, top, bottom]`
    /// (default 24, 24, 0, 24).
    pub fn content_padding(mut self, padding: [f32; 4]) -> Self {
        self.content_padding = Some(padding);
        self
    }

    /// Padding around the actions row, `[left, right, top, bottom]`
    /// (default 24, 24, 0, 24).
    pub fn actions_padding(mut self, padding: [f32; 4]) -> Self {
        self.actions_padding = Some(padding);
        self
    }

    /// Minimum padding inside each action button, `[horizontal,
    /// vertical]`. The buttons follow the button tokens; this can only make
    /// them taller.
    pub fn button_padding(mut self, padding: [f32; 2]) -> Self {
        self.button_padding = Some(padding);
        self
    }

    /// Put the content in a vertical scroll area.
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    /// Spacing between action buttons (default 8).
    pub fn actions_spacing(mut self, spacing: f32) -> Self {
        self.actions_spacing = spacing;
        self
    }

    fn push_action<F>(mut self, text: impl Into<String>, action_type: ActionType, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.actions.push(DialogAction {
            text: text.into(),
            action_type,
            action: Box::new(action),
        });
        self
    }

    /// Add a text action button; `action` runs when it is clicked, and the
    /// dialog closes.
    pub fn text_action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.push_action(text, ActionType::Text, action)
    }

    /// Add a filled tonal action button.
    pub fn filled_tonal_action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.push_action(text, ActionType::FilledTonal, action)
    }

    /// Add a filled action button.
    pub fn filled_action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.push_action(text, ActionType::Filled, action)
    }

    /// Same as [`text_action`](Self::text_action).
    pub fn action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.text_action(text, action)
    }

    /// Same as [`filled_action`](Self::filled_action).
    pub fn primary_action<F>(self, text: impl Into<String>, action: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.filled_action(text, action)
    }

    /// Shows the dialog while it is open (and while it animates out).
    pub fn show(self, ctx: &Context) -> DialogResponse<R> {
        let Self {
            id,
            title,
            open,
            dialog_type,
            icon,
            content,
            actions,
            quick,
            no_focus_trap,
            close_button,
            dismiss_on_escape,
            dismiss_on_scrim,
            max_width,
            min_width,
            max_height,
            title_padding,
            content_padding,
            actions_padding,
            button_padding,
            scrollable,
            actions_spacing,
        } = self;

        if !no_focus_trap {
            FocusReturn::update(ctx, id, *open);
        }
        let transition = if quick {
            Transition::NONE
        } else {
            Transition::EMPHASIZED
        };
        let visibility = transition.animate(ctx, id.with("transition"), *open);
        if !*open && visibility <= 0.0 {
            return DialogResponse::hidden();
        }
        let interactive = *open;

        let theme = MaterialTheme::of(ctx);
        let screen = ctx.content_rect();
        let max_width = max_width
            .unwrap_or(DIALOG_MAX_WIDTH)
            .min(screen.width())
            .max(0.0);
        let min_width = match dialog_type {
            DialogType::Form => max_width,
            DialogType::Standard | DialogType::Alert | DialogType::Confirm => {
                min_width.unwrap_or(DIALOG_MIN_WIDTH)
            }
        }
        .min(max_width);
        let max_height = max_height.unwrap_or((screen.height() * 0.9).min(800.0));

        // Enter: slide down 24 dp into place while fading in.
        let area = scrim::modal_area(id).anchor(
            egui::Align2::CENTER_CENTER,
            Vec2::new(0.0, (visibility - 1.0) * 24.0),
        );
        let scrim_color = scrim::scrim_color(&theme, visibility);

        let mut local_open = true;
        let mut clicked_action: Option<(usize, Box<dyn FnOnce() + 'a>)> = None;
        let mut close_clicked = false;

        let shown = scrim::show_modal(ctx, area, scrim_color, interactive, |ui| {
            ui.multiply_opacity(visibility);
            if !interactive {
                ui.disable();
            }
            let corner = CornerSize::ExtraLarge.value().unwrap_or(28.0);
            let surface = scrim::elevated_surface(
                ui,
                theme.color(ColorRole::SurfaceContainerHigh),
                egui::CornerRadius::same(corner as u8),
                Elevation::Level3,
                egui::Margin::ZERO,
                |ui| {
                    ui.set_min_width(min_width);
                    ui.set_max_width(max_width);
                    ui.spacing_mut().item_spacing.y = 0.0;

                    if let Some(icon) = &icon {
                        ui.add_space(DIALOG_PADDING);
                        ui.vertical_centered(|ui| {
                            ui.add(
                                crate::icon::MaterialIcon::new(
                                    crate::material_symbol::material_symbol_text(icon),
                                )
                                .size(ICON_SIZE)
                                .color(theme.color(ColorRole::Secondary)),
                            );
                        });
                        ui.add_space(ICON_TITLE_GAP);
                    }

                    // Keep the headline clear of the close button.
                    let close_reserve = if close_button {
                        CLOSE_BUTTON_SIZE - 12.0
                    } else {
                        0.0
                    };
                    let [left, right, top, bottom] = title_padding.unwrap_or([
                        DIALOG_PADDING,
                        DIALOG_PADDING,
                        if icon.is_some() { 0.0 } else { DIALOG_PADDING },
                        ICON_TITLE_GAP,
                    ]);
                    if !title.is_empty() {
                        let style = TypeRole::HeadlineSmall.style();
                        egui::Frame::NONE
                            .inner_margin(scrim::margin([left, right + close_reserve, top, bottom]))
                            .show(ui, |ui| {
                                let layout = if icon.is_some() {
                                    Layout::top_down(Align::Center)
                                } else {
                                    Layout::top_down(Align::Min)
                                };
                                ui.with_layout(layout, |ui| {
                                    ui.label(
                                        RichText::new(&title)
                                            .font(style.font_id(ui.ctx()))
                                            .line_height(Some(style.line_height))
                                            .color(theme.color(ColorRole::OnSurface)),
                                    );
                                });
                            });
                    } else if icon.is_none() {
                        ui.add_space(top);
                    }

                    let [left, right, top, bottom] = content_padding.unwrap_or([
                        DIALOG_PADDING,
                        DIALOG_PADDING,
                        0.0,
                        TEXT_ACTIONS_GAP,
                    ]);
                    let body_style = TypeRole::BodyMedium.style();
                    let inner = egui::Frame::NONE
                        .inner_margin(scrim::margin([left, right, top, bottom]))
                        .show(ui, |ui| {
                            ui.style_mut().override_font_id = Some(body_style.font_id(ui.ctx()));
                            ui.visuals_mut().override_text_color =
                                Some(theme.color(ColorRole::OnSurfaceVariant));
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                            if scrollable {
                                egui::ScrollArea::vertical()
                                    .id_salt("dialog_content_scroll")
                                    .max_height((max_height - 160.0).max(48.0))
                                    .show(ui, |ui| content(ui, &mut local_open))
                                    .inner
                            } else {
                                content(ui, &mut local_open)
                            }
                        })
                        .inner;

                    if !actions.is_empty() {
                        let [left, right, top, bottom] = actions_padding.unwrap_or([
                            DIALOG_PADDING,
                            DIALOG_PADDING,
                            0.0,
                            DIALOG_PADDING,
                        ]);
                        egui::Frame::NONE
                            .inner_margin(scrim::margin([left, right, top, bottom]))
                            .show(ui, |ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.spacing_mut().item_spacing.x = actions_spacing;
                                    // Right to left: the last action is
                                    // placed first, at the end.
                                    for (index, action) in actions.into_iter().enumerate().rev() {
                                        let mut button = match action.action_type {
                                            ActionType::Text => MaterialButton::text(&action.text),
                                            ActionType::FilledTonal => {
                                                MaterialButton::filled_tonal(&action.text)
                                            }
                                            ActionType::Filled => {
                                                MaterialButton::filled(&action.text)
                                            }
                                        };
                                        if let Some([_, vertical]) = button_padding {
                                            button = button.min_size(Vec2::new(
                                                0.0,
                                                (20.0 + 2.0 * vertical).max(40.0),
                                            ));
                                        }
                                        if ui.add(button).clicked() && clicked_action.is_none() {
                                            clicked_action = Some((index, action.action));
                                        }
                                    }
                                });
                            });
                    }
                    inner
                },
            );

            if close_button {
                let rect = egui::Rect::from_min_size(
                    egui::pos2(
                        surface.response.rect.right() - CLOSE_BUTTON_SIZE - 4.0,
                        surface.response.rect.top() + 4.0,
                    ),
                    Vec2::splat(CLOSE_BUTTON_SIZE),
                );
                let button = crate::MaterialIconButton::standard(
                    crate::material_symbol::material_symbol_text("close"),
                )
                .size(CLOSE_BUTTON_SIZE)
                .label("Close")
                .icon_color(theme.color(ColorRole::OnSurfaceVariant));
                if ui.put(rect, button).clicked() {
                    close_clicked = true;
                }
            }

            crate::a11y::describe(&surface.response, egui::Role::Dialog, true, &title, None);
            (surface.inner, surface.response)
        });

        let (inner, surface_response) = shown.inner;
        if !interactive {
            return DialogResponse {
                inner: None,
                close_reason: None,
                response: Some(surface_response),
            };
        }

        let close_reason = if let Some((index, action)) = clicked_action {
            action();
            Some(DialogCloseReason::Action(index))
        } else if close_clicked {
            Some(DialogCloseReason::CloseButton)
        } else if !local_open || shown.response.should_close() {
            Some(DialogCloseReason::Content)
        } else if shown.escape_pressed && dismiss_on_escape {
            Some(DialogCloseReason::Escape)
        } else if shown.scrim_clicked && dismiss_on_scrim {
            Some(DialogCloseReason::Scrim)
        } else {
            None
        };
        if close_reason.is_some() {
            *open = false;
            if !no_focus_trap {
                FocusReturn::update(ctx, id, false);
            }
        }
        DialogResponse {
            inner: Some(inner),
            close_reason,
            response: Some(surface_response),
        }
    }
}

// Convenience constructors

/// A standard dialog; see [`MaterialDialog::new`].
pub fn dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open)
}

/// An alert dialog; see [`MaterialDialog::new`].
pub fn alert_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Alert)
}

/// A confirmation dialog; see [`MaterialDialog::new`].
pub fn confirm_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Confirm)
}

/// A form dialog (full maximum width); see [`MaterialDialog::new`].
pub fn form_dialog(
    id: impl Into<egui::Id>,
    title: impl Into<String>,
    open: &mut bool,
) -> MaterialDialog<'_> {
    MaterialDialog::new(id, title, open).dialog_type(DialogType::Form)
}

// ---------------------------------------------------------------------------
// Full-screen dialog

/// `FullScreenDialogTokens.HeaderContainerHeight`.
pub const FULL_SCREEN_HEADER_HEIGHT: f32 = 56.0;

/// Material Design 3 full-screen dialog: covers the whole window, with a
/// header holding a close button, the headline and an optional confirm
/// action. Use it on compact windows for tasks that need the room.
///
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::dialog::FullScreenDialog;
///
/// let mut open = true;
/// let response = FullScreenDialog::new(egui::Id::unique("new_event"), "New event", &mut open)
///     .action("Save")
///     .show(ui.ctx(), |ui, _open| {
///         ui.label("Event details");
///     });
/// if response.closed() {
///     // Saved or dismissed: see response.close_reason.
/// }
/// # });
/// ```
pub struct FullScreenDialog<'a> {
    id: Id,
    title: String,
    open: &'a mut bool,
    action: Option<String>,
    dismiss_on_escape: bool,
}

impl<'a> FullScreenDialog<'a> {
    /// A full-screen dialog with `title`, shown while `*open` is true.
    pub fn new(id: impl Into<Id>, title: impl Into<String>, open: &'a mut bool) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            open,
            action: None,
            dismiss_on_escape: true,
        }
    }

    /// A text action at the end of the header ("Save"). A click closes the
    /// dialog with [`DialogCloseReason::Action`]`(0)`.
    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.action = Some(label.into());
        self
    }

    /// Whether Escape closes the dialog (default `true`).
    pub fn dismiss_on_escape(mut self, dismiss: bool) -> Self {
        self.dismiss_on_escape = dismiss;
        self
    }

    /// Shows the dialog with `content` under the header. The content gets
    /// the UI and the open flag, and its return value comes back in
    /// [`DialogResponse::inner`].
    pub fn show<R>(
        self,
        ctx: &Context,
        content: impl FnOnce(&mut Ui, &mut bool) -> R,
    ) -> DialogResponse<R> {
        let Self {
            id,
            title,
            open,
            action,
            dismiss_on_escape,
        } = self;
        FocusReturn::update(ctx, id, *open);
        let visibility = Transition::EMPHASIZED.animate(ctx, id.with("transition"), *open);
        if !*open && visibility <= 0.0 {
            return DialogResponse::hidden();
        }
        let interactive = *open;
        let theme = MaterialTheme::of(ctx);
        let screen = ctx.content_rect();

        // Enter: rise 48 dp from below while fading in.
        let area = scrim::modal_area(id).anchor(
            egui::Align2::LEFT_TOP,
            Vec2::new(0.0, (1.0 - visibility) * 48.0),
        );
        let mut local_open = true;
        let mut close_clicked = false;
        let mut action_clicked = false;

        let shown = scrim::show_modal(ctx, area, egui::Color32::TRANSPARENT, interactive, |ui| {
            ui.multiply_opacity(visibility);
            if !interactive {
                ui.disable();
            }
            let surface = scrim::elevated_surface(
                ui,
                theme.color(ColorRole::Surface),
                egui::CornerRadius::ZERO,
                Elevation::Level0,
                egui::Margin::ZERO,
                |ui| {
                    ui.set_min_size(screen.size());
                    ui.set_max_size(screen.size());
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.allocate_ui_with_layout(
                        Vec2::new(screen.width(), FULL_SCREEN_HEADER_HEIGHT),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.set_min_height(FULL_SCREEN_HEADER_HEIGHT);
                            ui.add_space(4.0);
                            let close = crate::MaterialIconButton::standard(
                                crate::material_symbol::material_symbol_text("close"),
                            )
                            .size(CLOSE_BUTTON_SIZE)
                            .label("Close")
                            .icon_color(theme.color(ColorRole::OnSurface));
                            close_clicked = ui.add(close).clicked();
                            ui.add_space(4.0);
                            let style = TypeRole::TitleLarge.style();
                            ui.label(
                                RichText::new(&title)
                                    .font(style.font_id(ui.ctx()))
                                    .color(theme.color(ColorRole::OnSurface)),
                            );
                            if let Some(action) = &action {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.add_space(12.0);
                                    action_clicked = ui.add(MaterialButton::text(action)).clicked();
                                });
                            }
                        },
                    );
                    egui::Frame::NONE
                        .inner_margin(scrim::margin([DIALOG_PADDING, DIALOG_PADDING, 0.0, 0.0]))
                        .show(ui, |ui| {
                            ui.visuals_mut().override_text_color =
                                Some(theme.color(ColorRole::OnSurfaceVariant));
                            content(ui, &mut local_open)
                        })
                        .inner
                },
            );
            crate::a11y::describe(&surface.response, egui::Role::Dialog, true, &title, None);
            (surface.inner, surface.response)
        });

        let (inner, surface_response) = shown.inner;
        if !interactive {
            return DialogResponse {
                inner: None,
                close_reason: None,
                response: Some(surface_response),
            };
        }
        let close_reason = if action_clicked {
            Some(DialogCloseReason::Action(0))
        } else if close_clicked {
            Some(DialogCloseReason::CloseButton)
        } else if !local_open || shown.response.should_close() {
            Some(DialogCloseReason::Content)
        } else if shown.escape_pressed && dismiss_on_escape {
            Some(DialogCloseReason::Escape)
        } else {
            None
        };
        if close_reason.is_some() {
            *open = false;
            FocusReturn::update(ctx, id, false);
        }
        DialogResponse {
            inner: Some(inner),
            close_reason,
            response: Some(surface_response),
        }
    }
}
