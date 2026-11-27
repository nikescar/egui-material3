//! Material Design 3 Action Sheet Components
//!
//! # M3 Color Role Usage
//!
//! - **surfaceContainerLow**: Sheet background (from `BottomSheet`)
//! - **onSurface**: Action text
//! - **onSurfaceVariant**: Supporting text
//! - **scrim @ 32%**: Modal overlay
//! - **State layers**: onSurface (`StateLayerTokens`)
//!
//! Built on a modal [`BottomSheet`]: drag handle, spring motion, drag to
//! dismiss, Escape and focus handling come from there.

use crate::bottom_sheet::BottomSheet;
use crate::tokens::{ColorRole, InteractionState, TypeRole};
use crate::MaterialTheme;
use egui::{
    epaint::{CornerRadius, Stroke},
    pos2, Area, Id, Response, Sense, Ui, Vec2,
};

/// Material Design action sheet component.
///
/// Action sheets present a set of actions to the user in a slide-up panel.
/// They appear from the bottom of the screen and can contain multiple groups of actions.
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut sheet_open = false;
///
/// let sheet = MaterialActionSheet::new("my_sheet", &mut sheet_open)
///     .label("Do something")
///     .button("Button 1", || {
///         println!("Button 1 clicked");
///     })
///     .button("Button 2", || {
///         println!("Button 2 clicked");
///     })
///     .button("Cancel", || {
///         println!("Cancel clicked");
///     });
///
/// sheet.show(ui.ctx());
/// # });
/// ```
#[must_use = "You should call .show() to display the action sheet"]
pub struct MaterialActionSheet<'a> {
    /// Unique identifier for this action sheet
    id: Id,
    /// Reference to open/closed state
    open: &'a mut bool,
    /// Whether to show backdrop/scrim
    backdrop: bool,
    /// Whether tapping backdrop closes the sheet
    backdrop_dismissible: bool,
    /// Groups of action buttons
    groups: Vec<ActionGroup<'a>>,
    /// Current group being built
    current_group: ActionGroup<'a>,
    /// Maximum width of the sheet
    max_width: f32,
}

/// A group of action buttons with an optional label
pub struct ActionGroup<'a> {
    /// Optional label for this group
    pub label: Option<String>,
    /// Buttons in this group
    pub buttons: Vec<ActionButton<'a>>,
}

/// An individual action button in the sheet
pub struct ActionButton<'a> {
    /// Button text
    pub text: String,
    /// Whether text should be bold
    pub bold: bool,
    /// Optional callback
    pub on_click: Option<Box<dyn FnOnce() + 'a>>,
    /// Whether button is enabled
    pub enabled: bool,
}

impl<'a> MaterialActionSheet<'a> {
    /// Create a new action sheet
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this action sheet
    /// * `open` - Mutable reference to the open/closed state
    pub fn new(id: impl Into<Id>, open: &'a mut bool) -> Self {
        Self {
            id: id.into(),
            open,
            backdrop: true,
            backdrop_dismissible: true,
            groups: Vec::new(),
            current_group: ActionGroup {
                label: None,
                buttons: Vec::new(),
            },
            max_width: 448.0, // Material Design 3 max width for mobile
        }
    }

    /// Set whether to show backdrop (default: true)
    pub fn backdrop(mut self, show: bool) -> Self {
        self.backdrop = show;
        self
    }

    /// Set whether tapping backdrop dismisses the sheet (default: true)
    pub fn backdrop_dismissible(mut self, dismissible: bool) -> Self {
        self.backdrop_dismissible = dismissible;
        self
    }

    /// Set the maximum width of the action sheet
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Add a label for the current group
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.current_group.label = Some(text.into());
        self
    }

    /// Add an action button to the current group
    pub fn button<F>(mut self, text: impl Into<String>, callback: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.current_group.buttons.push(ActionButton {
            text: text.into(),
            bold: false,
            on_click: Some(Box::new(callback)),
            enabled: true,
        });
        self
    }

    /// Add a bold action button to the current group
    pub fn bold_button<F>(mut self, text: impl Into<String>, callback: F) -> Self
    where
        F: FnOnce() + 'a,
    {
        self.current_group.buttons.push(ActionButton {
            text: text.into(),
            bold: true,
            on_click: Some(Box::new(callback)),
            enabled: true,
        });
        self
    }

    /// Add a button without callback
    pub fn simple_button(mut self, text: impl Into<String>) -> Self {
        self.current_group.buttons.push(ActionButton {
            text: text.into(),
            bold: false,
            on_click: None,
            enabled: true,
        });
        self
    }

    /// Start a new group (finalizes the current group)
    pub fn new_group(mut self) -> Self {
        if !self.current_group.buttons.is_empty() || self.current_group.label.is_some() {
            let group = std::mem::replace(
                &mut self.current_group,
                ActionGroup {
                    label: None,
                    buttons: Vec::new(),
                },
            );
            self.groups.push(group);
        }
        self
    }

    /// Show the action sheet: a modal [`BottomSheet`] holding the groups.
    ///
    /// Clicking an enabled button runs its callback and closes the sheet;
    /// so do Escape, a click on the backdrop (when
    /// [`backdrop_dismissible`](Self::backdrop_dismissible)), a drag down
    /// and the drag handle.
    pub fn show(mut self, ctx: &egui::Context) -> Response {
        // Finalize any remaining group
        if !self.current_group.buttons.is_empty() || self.current_group.label.is_some() {
            let group = std::mem::replace(
                &mut self.current_group,
                ActionGroup {
                    label: None,
                    buttons: Vec::new(),
                },
            );
            self.groups.push(group);
        }

        let id = self.id;
        let groups = self.groups;
        let mut clicked: Option<Option<ActionCallback<'a>>> = None;
        let shown = BottomSheet::modal(id, self.open)
            .max_width(self.max_width)
            .scrim(self.backdrop)
            .dismiss_on_scrim(self.backdrop_dismissible)
            .label("Action sheet")
            .show(ctx, |ui, open| {
                clicked = render_groups(ui, groups);
                if clicked.is_some() {
                    *open = false;
                }
            });
        if let Some(Some(callback)) = clicked {
            callback();
        }
        match shown.response {
            Some(response) => response,
            // Closed: an empty response, as before.
            None => {
                Area::new(id.with("dummy"))
                    .fixed_pos(pos2(-1000.0, -1000.0))
                    .show(ctx, |ui| ui.allocate_response(Vec2::ZERO, Sense::hover()))
                    .response
            }
        }
    }
}

/// A button's click callback.
type ActionCallback<'a> = Box<dyn FnOnce() + 'a>;

/// `ListTokens.ListItemOneLineContainerHeight`: one row per action.
const ROW_HEIGHT: f32 = 56.0;
/// `ListTokens.ListItemLeadingSpace`.
const ROW_PADDING: f32 = 16.0;

/// Draws the groups. Returns `Some(callback)` when an enabled button was
/// clicked (its callback may itself be `None`).
fn render_groups<'a>(
    ui: &mut Ui,
    groups: Vec<ActionGroup<'a>>,
) -> Option<Option<ActionCallback<'a>>> {
    let theme = MaterialTheme::of(ui.ctx());
    let width = ui.available_width();
    let left = ui.max_rect().left();
    let groups_len = groups.len();
    let mut clicked = None;
    for (group_idx, group) in groups.into_iter().enumerate() {
        if let Some(label) = &group.label {
            let (rect, _) = ui.allocate_exact_size(Vec2::new(width, ROW_HEIGHT), Sense::hover());
            ui.painter().text(
                pos2(rect.min.x + ROW_PADDING, rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                TypeRole::TitleSmall.style().font_id(ui.ctx()),
                theme.color(ColorRole::Primary),
            );
        }
        for button in group.buttons {
            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(width, ROW_HEIGHT),
                crate::a11y::sense(button.enabled, Sense::click()),
            );
            crate::a11y::button(&response, button.enabled, &button.text);
            let on_surface = theme.color(ColorRole::OnSurface);
            crate::paint::paint_state_layer(
                ui.painter(),
                rect,
                CornerRadius::ZERO,
                on_surface,
                InteractionState::from_response(&response),
            );
            crate::paint::paint_focus_ring_if_focused(ui.painter(), &response, CornerRadius::ZERO);
            let text_color = if button.enabled {
                on_surface
            } else {
                on_surface.gamma_multiply(crate::tokens::state::DISABLED_CONTENT_OPACITY)
            };
            let style = if button.bold {
                TypeRole::BodyLarge.emphasized()
            } else {
                TypeRole::BodyLarge.style()
            };
            ui.painter().text(
                pos2(rect.min.x + ROW_PADDING, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &button.text,
                style.font_id(ui.ctx()),
                text_color,
            );
            if response.clicked() && button.enabled && clicked.is_none() {
                clicked = Some(button.on_click);
            }
        }
        if group_idx + 1 < groups_len {
            let y = ui.cursor().top();
            ui.painter().line_segment(
                [pos2(left, y), pos2(left + width, y)],
                Stroke::new(1.0, theme.color(ColorRole::OutlineVariant)),
            );
        }
    }
    ui.add_space(8.0);
    clicked
}

/// Convenience function to create an action sheet
///
/// # Example
/// ```rust
/// # egui::__run_test_ui(|ui| {
/// let mut open = false;
/// action_sheet("my_sheet", &mut open)
///     .button("Action 1", || {})
///     .button("Action 2", || {})
///     .show(ui.ctx());
/// # });
/// ```
pub fn action_sheet<'a>(id: impl Into<Id>, open: &'a mut bool) -> MaterialActionSheet<'a> {
    MaterialActionSheet::new(id, open)
}
