// ============================================================================
// Module:       badge
// Description:  Material 3 small (dot) and large (labelled) badges.
//
// Dependencies: egui; crate::{tokens, MaterialTheme}
// ============================================================================

//! Material Design 3 badges.
//!
//! Values from Compose's `BadgeTokens.kt` and `Badge.kt`:
//!
//! - Small badge (a dot): 6 across (`BadgeTokens.Size`), fully round
//!   (`Shape` = `CornerFull`).
//! - Large badge: 16 high and at least 16 wide (`LargeSize`), fully round
//!   (`LargeShape`), 4 of horizontal padding
//!   (`BadgeWithContentHorizontalPadding`), LabelSmall text
//!   (`LargeLabelTextFont`). [`MaterialBadge::count`] caps at "999+", the
//!   four characters M3 allows.
//! - Colours: `error` container, `onError` text (`Color`, `LargeColor`,
//!   `LargeLabelTextColor`).
//! - Placement on an anchor (`BadgedBox`): a dot's start edge sits 6 in from
//!   the anchor's end edge and its top on the anchor's top
//!   (`BadgeOffset`); a large badge's start edge sits 12 in from the end
//!   (`BadgeWithContentHorizontalOffset`) and it overlaps the anchor by 14
//!   vertically (`BadgeWithContentVerticalOffset`), so it rises 2 above it.
//!
//! [`MaterialBadge::show_on`] also appends the badge's text (or its
//! [`description`](MaterialBadge::description)) to the anchor's
//! accessibility label, so a screen reader reads "Inbox, 3 new messages".

use std::sync::Arc;

use egui::{
    Color32, Galley, IdSalt, Pos2, Rect, Response, Role, Sense, Ui, Vec2, Widget, WidgetInfo,
};

use crate::tokens::{ColorRole, TypeRole};
use crate::MaterialTheme;

/// `BadgeTokens.Size`: the small badge (dot), 6.
const DOT_SIZE: f32 = 6.0;
/// `BadgeTokens.LargeSize`: the large badge's height and minimum width, 16.
const LARGE_SIZE: f32 = 16.0;
/// `BadgeWithContentHorizontalPadding`: 4 either side of the text.
const HORIZONTAL_PADDING: f32 = 4.0;
/// `BadgeWithContentHorizontalOffset`: a large badge starts 12 in from
/// the anchor's end edge.
const WITH_CONTENT_HORIZONTAL_OFFSET: f32 = 12.0;
/// `BadgeWithContentVerticalOffset`: a large badge overlaps the anchor's
/// top by 14.
const WITH_CONTENT_VERTICAL_OFFSET: f32 = 14.0;
/// `BadgeOffset`: a dot starts 6 in from the anchor's end edge and
/// overlaps its top by 6.
const DOT_OFFSET: f32 = 6.0;
/// The largest number [`MaterialBadge::count`] shows; above it the badge
/// reads "999+".
pub const MAX_COUNT: u32 = 999;
/// What a dot adds to its anchor's accessibility label when it has no
/// [`description`](MaterialBadge::description).
const DOT_DESCRIPTION: &str = "New";

/// Badge colours.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeColor {
    /// `primary` container, `onPrimary` text.
    Primary,
    /// `error` container, `onError` text: the Material 3 badge, and the
    /// default.
    Error,
    /// A fixed green. Not a Material 3 colour role.
    Success,
    /// A fixed amber. Not a Material 3 colour role.
    Warning,
    /// A fixed grey. Not a Material 3 colour role.
    Neutral,
    /// A custom container and text colour.
    Custom(Color32, Color32),
}

impl BadgeColor {
    /// The container and text colours in `theme`.
    fn colors(self, theme: &MaterialTheme) -> (Color32, Color32) {
        match self {
            BadgeColor::Primary => (
                theme.color(ColorRole::Primary),
                theme.color(ColorRole::OnPrimary),
            ),
            BadgeColor::Error => (
                theme.color(ColorRole::Error),
                theme.color(ColorRole::OnError),
            ),
            BadgeColor::Success => (Color32::from_rgb(34, 197, 94), Color32::WHITE),
            BadgeColor::Warning => (Color32::from_rgb(234, 179, 8), Color32::BLACK),
            BadgeColor::Neutral => (Color32::from_rgb(107, 114, 128), Color32::WHITE),
            BadgeColor::Custom(container, text) => (container, text),
        }
    }
}

/// Badge size variants.
#[deprecated(
    note = "Material 3 has two badge sizes: `MaterialBadge::dot()` (6) and a labelled badge \
            (16 high); `MaterialBadge::size` no longer changes anything"
)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeSize {
    /// Formerly 16 high.
    Small,
    /// Formerly 20 high.
    Regular,
    /// Formerly 24 high.
    Large,
}

/// Where a badge sits on its anchor. The corners use Compose's offsets,
/// mirrored for the left and bottom edges.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BadgePosition {
    /// Top-right corner: where Compose's `BadgedBox` puts it.
    #[default]
    TopRight,
    /// Top-left corner.
    TopLeft,
    /// Bottom-right corner.
    BottomRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Centred at this offset from the anchor's centre.
    Custom(Vec2),
}

/// Material Design 3 badge.
///
/// ```rust
/// # use egui_material3::*;
/// # egui::__run_test_ui(|ui| {
/// // On an anchor: placed at its top-right corner, and read as part of
/// // its label ("Inbox, 3 new messages").
/// let inbox = ui.button("Inbox");
/// MaterialBadge::count(3)
///     .description("3 new messages")
///     .show_on(ui, &inbox, BadgePosition::TopRight);
///
/// // A dot.
/// let updates = ui.button("Updates");
/// MaterialBadge::dot().show_on(ui, &updates, BadgePosition::TopRight);
///
/// // Standalone.
/// ui.add(badge("NEW"));
/// # });
/// ```
#[must_use = "add it with `ui.add(...)` or `.show_on(ui, &anchor, position)`"]
#[derive(Clone, Debug)]
pub struct MaterialBadge {
    content: String,
    color: BadgeColor,
    dot: bool,
    position_offset: Vec2,
    id_salt: Option<IdSalt>,
    description: Option<String>,
}

impl MaterialBadge {
    /// A large badge showing `content`. Empty content gives a dot.
    pub fn new(content: impl Into<String>) -> Self {
        let content = content.into();
        Self {
            dot: content.is_empty(),
            content,
            color: BadgeColor::Error,
            position_offset: Vec2::ZERO,
            id_salt: None,
            description: None,
        }
    }

    /// A small badge: a 6 dot with no text.
    pub fn dot() -> Self {
        Self::new("")
    }

    /// A large badge showing `count`, or "999+" above [`MAX_COUNT`].
    pub fn count(count: u32) -> Self {
        if count > MAX_COUNT {
            Self::new(format!("{MAX_COUNT}+"))
        } else {
            Self::new(count.to_string())
        }
    }

    /// The colour. The default, [`BadgeColor::Error`], is the Material 3
    /// badge.
    pub fn color(mut self, color: BadgeColor) -> Self {
        self.color = color;
        self
    }

    /// Formerly picked one of three non-Material sizes; does nothing now.
    #[deprecated(
        note = "Material 3 badges have fixed sizes: use `MaterialBadge::dot()` for the 6 dot \
                and `MaterialBadge::new` / `count` for the 16-high labelled badge"
    )]
    #[expect(
        deprecated,
        reason = "the deprecated builder names the deprecated size type"
    )]
    pub fn size(self, _size: BadgeSize) -> Self {
        self
    }

    /// Show as a dot, hiding the text.
    pub fn as_dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    /// Moves the badge by `offset` from where its position puts it.
    pub fn position_offset(mut self, offset: Vec2) -> Self {
        self.position_offset = offset;
        self
    }

    /// Distinguishes badges whose id would otherwise collide. By default a
    /// badge on an anchor takes its id from the anchor's rectangle.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = Some(IdSalt::new(&salt));
        self
    }

    /// What the badge adds to its anchor's accessibility label, such as
    /// "3 new messages". Defaults to the badge's text, or "New" for a dot.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The text a screen reader hears for this badge.
    fn spoken(&self) -> String {
        match &self.description {
            Some(description) => description.clone(),
            None if self.dot => DOT_DESCRIPTION.to_owned(),
            None => self.content.clone(),
        }
    }

    /// The laid-out label, for a large badge.
    fn galley(&self, ui: &Ui) -> Option<Arc<Galley>> {
        (!self.dot).then(|| {
            let job = TypeRole::LabelSmall.style().layout_job(
                ui.ctx(),
                self.content.clone(),
                Color32::PLACEHOLDER,
            );
            ui.painter().layout_job(job)
        })
    }

    /// The badge's size for `galley`.
    fn badge_size(galley: Option<&Arc<Galley>>) -> Vec2 {
        match galley {
            Some(galley) => Vec2::new(
                (galley.size().x + 2.0 * HORIZONTAL_PADDING).max(LARGE_SIZE),
                LARGE_SIZE,
            ),
            None => Vec2::splat(DOT_SIZE),
        }
    }

    /// Where a badge of `size` goes on `anchor`.
    fn placement(&self, anchor: Rect, size: Vec2, position: BadgePosition) -> Rect {
        let (h, v) = if self.dot {
            (DOT_OFFSET, DOT_OFFSET)
        } else {
            (WITH_CONTENT_HORIZONTAL_OFFSET, WITH_CONTENT_VERTICAL_OFFSET)
        };
        let right = anchor.max.x - h;
        let left = anchor.min.x + h - size.x;
        let top = anchor.min.y - size.y + v;
        let bottom = anchor.max.y - v;
        let min = match position {
            BadgePosition::TopRight => Pos2::new(right, top),
            BadgePosition::TopLeft => Pos2::new(left, top),
            BadgePosition::BottomRight => Pos2::new(right, bottom),
            BadgePosition::BottomLeft => Pos2::new(left, bottom),
            BadgePosition::Custom(offset) => anchor.center() + offset - size / 2.0,
        };
        Rect::from_min_size(min + self.position_offset, size)
    }

    fn paint(&self, ui: &Ui, rect: Rect, galley: Option<Arc<Galley>>) {
        let theme = MaterialTheme::of(ui.ctx());
        let (container, text) = self.color.colors(&theme);
        let radius = (rect.height() / 2.0).round().clamp(0.0, 255.0) as u8;
        ui.painter().rect_filled(rect, radius, container);
        if let Some(galley) = galley {
            let pos = rect.center() - galley.size() / 2.0;
            ui.painter().galley(pos, galley, text);
        }
    }

    /// Paints the badge on `target_rect` at `position` and returns its
    /// (hover-only) response. Prefer [`show_on`](Self::show_on), which
    /// also makes the badge part of the anchor's accessibility label.
    pub fn draw_on(&self, ui: &mut Ui, target_rect: Rect, position: BadgePosition) -> Response {
        let galley = self.galley(ui);
        let rect = self.placement(target_rect, Self::badge_size(galley.as_ref()), position);
        let id = match self.id_salt {
            Some(salt) => ui.scope_id().with_salt(salt),
            None => ui.scope_id().with((
                "egui_material3::badge",
                target_rect.min.x.to_bits(),
                target_rect.min.y.to_bits(),
                target_rect.max.x.to_bits(),
                target_rect.max.y.to_bits(),
            )),
        };
        self.paint(ui, rect, galley);
        ui.interact(rect, id, Sense::hover())
    }

    /// Paints the badge on `anchor` at `position` and appends its text
    /// (or [`description`](Self::description)) to the anchor's
    /// accessibility label. Call it right after adding the anchor.
    pub fn show_on(self, ui: &mut Ui, anchor: &Response, position: BadgePosition) -> Response {
        let response = self.draw_on(ui, anchor.rect, position);
        let spoken = self.spoken();
        ui.ctx().accesskit_node_builder(anchor.id, |node| {
            let label = match node.label() {
                Some(label) if !label.is_empty() => format!("{label}, {spoken}"),
                Some(_) | None => spoken,
            };
            node.set_label(label);
        });
        response
    }
}

impl Widget for MaterialBadge {
    /// A standalone badge, reported to accessibility as a label.
    fn ui(self, ui: &mut Ui) -> Response {
        let galley = self.galley(ui);
        let (rect, response) =
            ui.allocate_exact_size(Self::badge_size(galley.as_ref()), Sense::hover());
        self.paint(ui, rect, galley);
        let spoken = self.spoken();
        response.widget_info(|| WidgetInfo::labeled(Role::Label, true, spoken.clone()));
        response
    }
}

/// Creates a large badge showing `content`.
///
/// ```rust
/// # use egui_material3::*;
/// # egui::__run_test_ui(|ui| {
/// ui.add(badge("5"));
/// # });
/// ```
pub fn badge(content: impl Into<String>) -> MaterialBadge {
    MaterialBadge::new(content)
}

/// Creates a small (dot) badge.
///
/// ```rust
/// # use egui_material3::*;
/// # egui::__run_test_ui(|ui| {
/// ui.add(badge_dot());
/// # });
/// ```
pub fn badge_dot() -> MaterialBadge {
    MaterialBadge::dot()
}
