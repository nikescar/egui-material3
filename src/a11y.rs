//! Accessibility helpers: what every interactive widget tells AccessKit.
//!
//! A widget that paints its own text and shapes is an unlabeled node to
//! screen readers and UI-test tools unless it reports a role, a label and
//! its state. Components call [`describe`] (or one of the shorthands) right
//! after allocating their response, and allocate with [`sense`] so a
//! disabled widget neither takes focus nor reports `clicked()`.

use egui::{Response, Role, Sense, WidgetInfo};

/// `sense` when enabled; hover only when disabled, so the widget cannot be
/// clicked, focused or dragged.
pub fn sense(enabled: bool, sense: Sense) -> Sense {
    if enabled {
        sense
    } else {
        Sense::hover()
    }
}

/// Reports `role`, `label`, enabled state and, for toggles and selectable
/// items, `selected` (shown as checked/pressed/selected by the platform).
pub fn describe(
    response: &Response,
    role: Role,
    enabled: bool,
    label: &str,
    selected: Option<bool>,
) {
    let label = label.to_owned();
    response.widget_info(move || {
        let mut info = WidgetInfo::labeled(role, enabled, label.clone());
        info.selected = selected;
        info
    });
}

/// A push button.
pub fn button(response: &Response, enabled: bool, label: &str) {
    describe(response, Role::Button, enabled, label, None);
}

/// A two-state control (checkbox, switch, toggle button, filter chip,
/// radio button, tab).
pub fn toggle(response: &Response, role: Role, enabled: bool, label: &str, selected: bool) {
    describe(response, role, enabled, label, Some(selected));
}

/// A slider at `value`.
pub fn slider(response: &Response, enabled: bool, label: &str, value: f64) {
    let label = label.to_owned();
    response.widget_info(move || WidgetInfo::slider(enabled, value, label.clone()));
}
