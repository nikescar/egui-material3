// ============================================================================
// Module:       carousel
// Description:  Material 3 carousel: multi-browse, uncontained, hero and
//               full-screen layouts on Compose's keyline math.
//
// Dependencies: egui; crate::{a11y, paint, tokens, MaterialTheme}
// ============================================================================

//! Material Design 3 carousel.
//!
//! Items are laid out with the keyline math of Compose material3's
//! carousel (`androidx.compose.material3.carousel`, `androidx-main`): each
//! strategy builds a list of keylines (sizes at fixed points of the
//! viewport), and an item takes the size of the keylines around its centre
//! as it scrolls. Items are drawn at full size and masked to that keyline
//! size around their centre, so content stays put while the mask narrows
//! (the parallax of Compose's `Modifier.carouselItem`).
//!
//! - [`CarouselStrategy::MultiBrowse`] is `HorizontalMultiBrowseCarousel`
//!   (`multiBrowseKeylineList`): large items, then a medium and a small one.
//! - [`CarouselStrategy::Uncontained`] is `HorizontalUncontainedCarousel`
//!   (`uncontainedKeylineList`): fixed-width items, the last one cut off.
//! - [`CarouselStrategy::Hero`] is `HorizontalCenteredHeroCarousel`
//!   (`heroKeylineList`), centred or start-aligned.
//! - [`CarouselStrategy::FullScreen`] is one item filling the viewport
//!   (`heroKeylineList` with a single item, which allows no small items).
//!
//! The item mask uses the extra-large corner (28 dp,
//! `ShapeTokens.CornerExtraLarge`), clamped to half the mask width.
//! Snapping follows Compose's `CarouselDefaults.singleAdvanceFlingBehavior`:
//! at most one item per fling, with `spring(stiffness = StiffnessMediumLow)`.
//!
//! # Colour roles
//!
//! - **surfaceContainerHighest**: background of text items
//! - **onSurface**: text items' label and state layer
//! - **secondary**: focus ring

mod keylines;
mod strategy;

pub use keylines::Keyline;

use crate::tokens::{ColorRole, CornerSize, InteractionState, LayoutDirection, SpringSpec};
use crate::{a11y, paint, MaterialTheme};
use egui::{self, Align2, FontId, ImageSource, Key, Rangef, Rect, Response, Role, Sense, Ui, Vec2};
use keylines::KeylineList;
use strategy::Strategy;

/// `CarouselDefaults.MinSmallItemSize`.
pub const MIN_SMALL_ITEM_WIDTH: f32 = 40.0;
/// `CarouselDefaults.MaxSmallItemSize`.
pub const MAX_SMALL_ITEM_WIDTH: f32 = 56.0;
/// `CarouselDefaults.ItemSpacing`.
pub const ITEM_SPACING: f32 = 0.0;

/// The snap spring: Compose's `singleAdvanceFlingBehavior` default
/// `spring(stiffness = Spring.StiffnessMediumLow)` (400) with the default
/// `Spring.DampingRatioNoBouncy` (1).
const SNAP_SPRING: SpringSpec = SpringSpec::new(1.0, 400.0);
/// A release faster than this (points per second) advances one item:
/// Compose's `MinFlingVelocityDp` (400 dp/s) in `PagerDefaults`.
const MIN_FLING_VELOCITY: f32 = 400.0;
/// Seconds without wheel input after which a wheel scroll snaps.
const WHEEL_SETTLE_SECONDS: f64 = 0.15;

/// How a carousel arranges its items.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarouselStrategy {
    /// Large items followed by a medium and a small item, which grow as
    /// they scroll in. `preferred_item_width` is a target; the strategy
    /// picks the closest arrangement that fills the viewport exactly.
    MultiBrowse {
        /// Target width of large items.
        preferred_item_width: f32,
    },
    /// Items of one width, as many as fit, plus a cut-off one. Does not
    /// snap by default.
    Uncontained {
        /// Width of every item.
        item_width: f32,
    },
    /// One or more large "hero" items with small items beside them.
    Hero {
        /// Target width of large items; `None` fills the viewport less the
        /// small items.
        preferred_item_width: Option<f32>,
        /// Centre the large item between two small ones (needs at least
        /// three items; fewer are start-aligned).
        centered: bool,
    },
    /// One item filling the viewport, paging one at a time.
    FullScreen,
}

impl Default for CarouselStrategy {
    fn default() -> Self {
        CarouselStrategy::MultiBrowse {
            preferred_item_width: 180.0,
        }
    }
}

/// The viewport and item constraints a strategy is evaluated for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarouselMetrics {
    /// Width of the carousel viewport.
    pub viewport_width: f32,
    /// Gap between items.
    pub item_spacing: f32,
    /// Number of items in the carousel.
    pub item_count: usize,
    /// Smallest a small item may be.
    pub min_small_item_width: f32,
    /// Largest a small item may be.
    pub max_small_item_width: f32,
}

impl CarouselMetrics {
    /// `item_count` items in a viewport `viewport_width` wide, with the
    /// `CarouselDefaults` spacing and small-item range.
    pub fn new(viewport_width: f32, item_count: usize) -> Self {
        Self {
            viewport_width,
            item_spacing: ITEM_SPACING,
            item_count,
            min_small_item_width: MIN_SMALL_ITEM_WIDTH,
            max_small_item_width: MAX_SMALL_ITEM_WIDTH,
        }
    }
}

/// Where one item is drawn, along the scrolling axis, from the viewport's
/// start edge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarouselItemLayout {
    /// The item's index.
    pub index: usize,
    /// The visible (masked) part of the item.
    pub mask: Rangef,
    /// The item's full-size content box, centred on the mask.
    pub content: Rangef,
}

impl CarouselStrategy {
    fn keyline_list(&self, m: &CarouselMetrics) -> KeylineList {
        let width = m.viewport_width;
        match *self {
            CarouselStrategy::MultiBrowse {
                preferred_item_width,
            } => keylines::multi_browse(
                width,
                preferred_item_width.max(1.0),
                m.item_spacing,
                m.item_count,
                m.min_small_item_width,
                m.max_small_item_width,
            ),
            CarouselStrategy::Uncontained { item_width } => {
                keylines::uncontained(width, item_width.max(1.0), m.item_spacing)
            }
            CarouselStrategy::Hero {
                preferred_item_width,
                centered,
            } => keylines::hero(
                width,
                preferred_item_width.map(|w| w.max(1.0)),
                m.item_spacing,
                m.item_count,
                centered,
                m.min_small_item_width,
                m.max_small_item_width,
            ),
            CarouselStrategy::FullScreen => keylines::hero(
                width,
                None,
                m.item_spacing,
                1,
                false,
                m.min_small_item_width,
                m.max_small_item_width,
            ),
        }
    }

    fn strategy(&self, m: &CarouselMetrics) -> Strategy {
        if !(m.viewport_width.is_finite() && m.viewport_width > 0.0) {
            return Strategy::default();
        }
        Strategy::new(self.keyline_list(m), m.viewport_width, m.item_spacing)
    }

    /// The resting keylines, anchors included, left to right.
    pub fn keylines(&self, m: &CarouselMetrics) -> Vec<Keyline> {
        self.strategy(m).default_keylines.into_vec()
    }

    /// The width of a fully visible (focal) item.
    pub fn item_width(&self, m: &CarouselMetrics) -> f32 {
        self.strategy(m).item_main_axis_size()
    }

    /// The largest scroll offset.
    pub fn max_scroll(&self, m: &CarouselMetrics) -> f32 {
        max_scroll(&self.strategy(m), m.item_count)
    }

    /// The scroll offset at which item `index` rests in a focal position.
    pub fn snap_offset(&self, m: &CarouselMetrics, index: usize) -> f32 {
        snap_target(&self.strategy(m), m.item_count, index)
    }

    /// The visible items at `scroll_offset`.
    pub fn layout(&self, m: &CarouselMetrics, scroll_offset: f32) -> Vec<CarouselItemLayout> {
        layout_items(&self.strategy(m), m.item_count, scroll_offset)
    }

    /// Multi-browse, hero and full-screen carousels snap; uncontained ones
    /// scroll freely (Compose's default fling behaviours).
    fn snaps_by_default(&self) -> bool {
        match self {
            CarouselStrategy::MultiBrowse { .. }
            | CarouselStrategy::Hero { .. }
            | CarouselStrategy::FullScreen => true,
            CarouselStrategy::Uncontained { .. } => false,
        }
    }
}

fn max_scroll(strategy: &Strategy, item_count: usize) -> f32 {
    if !strategy.is_valid() || item_count == 0 {
        return 0.0;
    }
    let n = item_count as f32;
    let total = strategy.item_main_axis_size() * n + strategy.item_spacing * (n - 1.0);
    (total - strategy.available_space).max(0.0)
}

fn snap_target(strategy: &Strategy, item_count: usize, index: usize) -> f32 {
    let step = strategy.item_main_axis_size() + strategy.item_spacing;
    let raw = index as f32 * step - strategy.snap_position_offset(index, item_count);
    raw.clamp(0.0, max_scroll(strategy, item_count))
}

fn nearest_snap_index(strategy: &Strategy, item_count: usize, offset: f32) -> usize {
    (0..item_count)
        .min_by(|&a, &b| {
            let da = (snap_target(strategy, item_count, a) - offset).abs();
            let db = (snap_target(strategy, item_count, b) - offset).abs();
            da.total_cmp(&db)
        })
        .unwrap_or(0)
}

/// Compose's `Modifier.carouselItem` placement for every visible item.
fn layout_items(strategy: &Strategy, item_count: usize, scroll: f32) -> Vec<CarouselItemLayout> {
    if !strategy.is_valid() {
        return Vec::new();
    }
    let size = strategy.item_main_axis_size();
    let step = size + strategy.item_spacing;
    let keylines = strategy.keylines_for_scroll(scroll, max_scroll(strategy, item_count), false);
    let viewport = strategy.available_space;
    let mut out = Vec::new();
    for index in 0..item_count {
        let center = index as f32 * step + size / 2.0 - scroll;
        // Items this far outside the viewport cannot be shifted into it.
        if center + size * 2.0 < 0.0 || center - size * 2.0 > viewport {
            continue;
        }
        let (Some(before), Some(after)) = (
            keylines.keyline_before(center),
            keylines.keyline_after(center),
        ) else {
            continue;
        };
        let out_of_bounds = before == after;
        let progress = if out_of_bounds {
            1.0
        } else {
            (center - before.unadjusted_offset)
                / (after.unadjusted_offset - before.unadjusted_offset)
        };
        let k = keylines::lerp_keyline(before, after, progress);
        let mut translation = k.offset - center;
        if out_of_bounds && k.size != 0.0 {
            translation += (center - k.unadjusted_offset) / k.size;
        }
        let shown = center + translation;
        let mask = Rangef::new(shown - k.size / 2.0, shown + k.size / 2.0);
        if k.size <= 0.0 || mask.max <= 0.0 || mask.min >= viewport {
            continue;
        }
        out.push(CarouselItemLayout {
            index,
            mask,
            content: Rangef::new(shown - size / 2.0, shown + size / 2.0),
        });
    }
    out
}

type ItemFn<'a> = Box<dyn FnOnce(&mut Ui, Rect) + 'a>;

enum ItemContent<'a> {
    Custom(ItemFn<'a>),
    Text(String),
    Image(ImageSource<'a>),
}

/// A single item in a carousel.
pub struct CarouselItem<'a> {
    label: Option<String>,
    content: ItemContent<'a>,
}

/// What [`MaterialCarousel::show`] returns.
pub struct CarouselResponse {
    /// The carousel's own response (drag and hover over the viewport).
    pub response: Response,
    /// The item clicked (or activated with the keyboard) this frame.
    pub clicked: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
struct SnapAnimation {
    target: f32,
    start_time: f64,
    x0: f32,
    v0: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct CarouselState {
    animation: Option<SnapAnimation>,
    drag_start_index: usize,
    focused: Option<usize>,
    last_wheel_time: Option<f64>,
}

/// A Material Design 3 carousel.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::{CarouselStrategy, MaterialCarousel};
/// let mut offset = 0.0f32;
/// ui.add(MaterialCarousel::new(&mut offset)
///     .strategy(CarouselStrategy::MultiBrowse { preferred_item_width: 186.0 })
///     .item_spacing(8.0)
///     .item_text("Item 0")
///     .item_text("Item 1")
///     .item_text("Item 2"));
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct MaterialCarousel<'a> {
    items: Vec<CarouselItem<'a>>,
    strategy: CarouselStrategy,
    height: f32,
    item_spacing: f32,
    min_small_item_width: f32,
    max_small_item_width: f32,
    corner_radius: Option<f32>,
    snapping: Option<bool>,
    scroll_offset: &'a mut f32,
    id_salt: Option<String>,
    label: String,
}

impl<'a> MaterialCarousel<'a> {
    /// A multi-browse carousel scrolled to `scroll_offset`, which it updates.
    pub fn new(scroll_offset: &'a mut f32) -> Self {
        Self {
            items: Vec::new(),
            strategy: CarouselStrategy::default(),
            height: 150.0,
            item_spacing: ITEM_SPACING,
            min_small_item_width: MIN_SMALL_ITEM_WIDTH,
            max_small_item_width: MAX_SMALL_ITEM_WIDTH,
            corner_radius: None,
            snapping: None,
            scroll_offset,
            id_salt: None,
            label: "Carousel".to_owned(),
        }
    }

    /// A multi-browse carousel with large items about `preferred_item_width`.
    pub fn multi_browse(scroll_offset: &'a mut f32, preferred_item_width: f32) -> Self {
        Self::new(scroll_offset).strategy(CarouselStrategy::MultiBrowse {
            preferred_item_width,
        })
    }

    /// An uncontained carousel of `item_width` items.
    pub fn uncontained(scroll_offset: &'a mut f32, item_width: f32) -> Self {
        Self::new(scroll_offset).strategy(CarouselStrategy::Uncontained { item_width })
    }

    /// A centred hero carousel.
    pub fn hero(scroll_offset: &'a mut f32) -> Self {
        Self::new(scroll_offset).strategy(CarouselStrategy::Hero {
            preferred_item_width: None,
            centered: true,
        })
    }

    /// A full-screen carousel: one item at a time.
    pub fn full_screen(scroll_offset: &'a mut f32) -> Self {
        Self::new(scroll_offset).strategy(CarouselStrategy::FullScreen)
    }

    /// Sets the layout strategy.
    pub fn strategy(mut self, strategy: CarouselStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Adds an item drawn by `content`. It receives the item's full-size
    /// content rect; the `Ui` is clipped to the visible (masked) part.
    pub fn item(mut self, content: impl FnOnce(&mut Ui, Rect) + 'a) -> Self {
        self.items.push(CarouselItem {
            label: None,
            content: ItemContent::Custom(Box::new(content)),
        });
        self
    }

    /// Adds a custom item with an accessible `label`.
    pub fn labeled_item(
        mut self,
        label: impl Into<String>,
        content: impl FnOnce(&mut Ui, Rect) + 'a,
    ) -> Self {
        self.items.push(CarouselItem {
            label: Some(label.into()),
            content: ItemContent::Custom(Box::new(content)),
        });
        self
    }

    /// Adds a text item on a `surfaceContainerHighest` background.
    pub fn item_text(mut self, label: impl Into<String>) -> Self {
        let label = label.into();
        self.items.push(CarouselItem {
            label: Some(label.clone()),
            content: ItemContent::Text(label),
        });
        self
    }

    /// Adds an image item, cropped by the mask with rounded corners.
    pub fn item_image(
        mut self,
        label: impl Into<String>,
        image: impl Into<ImageSource<'a>>,
    ) -> Self {
        self.items.push(CarouselItem {
            label: Some(label.into()),
            content: ItemContent::Image(image.into()),
        });
        self
    }

    /// Sets the target width of large items (the uncontained item width).
    pub fn item_extent(mut self, extent: f32) -> Self {
        self.strategy = match self.strategy {
            CarouselStrategy::MultiBrowse { .. } => CarouselStrategy::MultiBrowse {
                preferred_item_width: extent,
            },
            CarouselStrategy::Uncontained { .. } => {
                CarouselStrategy::Uncontained { item_width: extent }
            }
            CarouselStrategy::Hero { centered, .. } => CarouselStrategy::Hero {
                preferred_item_width: Some(extent),
                centered,
            },
            CarouselStrategy::FullScreen => CarouselStrategy::FullScreen,
        };
        self
    }

    /// Sets the smallest width a small item may shrink to (default 40).
    pub fn shrink_extent(mut self, extent: f32) -> Self {
        self.min_small_item_width = extent.max(0.0);
        self.max_small_item_width = self.max_small_item_width.max(self.min_small_item_width);
        self
    }

    /// Sets the range of small-item widths (default 40 to 56).
    pub fn small_item_width(mut self, min: f32, max: f32) -> Self {
        self.min_small_item_width = min.max(0.0);
        self.max_small_item_width = max.max(self.min_small_item_width);
        self
    }

    /// Sets the height of the carousel (default 150).
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(0.0);
        self
    }

    /// Sets the gap between items (default 0, as in Compose).
    pub fn item_spacing(mut self, spacing: f32) -> Self {
        self.item_spacing = spacing.max(0.0);
        self
    }

    /// Sets half the gap between items (kept for compatibility; prefer
    /// [`Self::item_spacing`]).
    pub fn padding(self, padding: f32) -> Self {
        self.item_spacing(padding * 2.0)
    }

    /// Overrides the item corner radius (default: the 28 dp extra-large
    /// corner).
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius.max(0.0));
        self
    }

    /// Turns snapping on or off (default: on, except for uncontained).
    pub fn item_snapping(mut self, snapping: bool) -> Self {
        self.snapping = Some(snapping);
        self
    }

    /// Sets an ID salt, for several carousels in one `Ui`.
    pub fn id_salt(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Sets the accessible label of the carousel (default "Carousel").
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Shows the carousel, returning its response and the clicked item.
    pub fn show(self, ui: &mut Ui) -> CarouselResponse {
        let theme = MaterialTheme::of(ui.ctx());
        let id = ui.make_persistent_id(self.id_salt.as_deref().unwrap_or("material_carousel"));
        let width = ui.available_width().max(0.0);
        let (outer, response) = ui.allocate_exact_size(Vec2::new(width, self.height), Sense::DRAG);
        a11y::describe(&response, Role::List, ui.is_enabled(), &self.label, None);

        let item_count = self.items.len();
        let metrics = CarouselMetrics {
            viewport_width: width,
            item_spacing: self.item_spacing,
            item_count,
            min_small_item_width: self.min_small_item_width,
            max_small_item_width: self.max_small_item_width,
        };
        let strategy = self.strategy.strategy(&metrics);
        if item_count == 0 || !strategy.is_valid() {
            return CarouselResponse {
                response,
                clicked: None,
            };
        }
        let last_index = item_count - 1;
        let snapping = self
            .snapping
            .unwrap_or_else(|| self.strategy.snaps_by_default());
        let rtl = theme.layout_direction == LayoutDirection::Rtl;
        let sign = if rtl { -1.0 } else { 1.0 };
        let max = max_scroll(&strategy, item_count);
        let item_id = |i: usize| id.with(("item", i));
        let now = ui.input(|i| i.time);
        let snap_to = |offset: f32, index: usize, velocity: f32| {
            let target = snap_target(&strategy, item_count, index);
            SnapAnimation {
                target,
                start_time: now,
                x0: offset - target,
                v0: velocity,
            }
        };

        let mut state = ui
            .ctx()
            .data(|d| d.get_temp::<CarouselState>(id))
            .unwrap_or_default();
        let mut offset = self.scroll_offset.clamp(0.0, max);

        // Wheel: horizontal scrolling (shift + wheel on most mice).
        let wheel = if response.contains_pointer() {
            ui.input(|i| i.smooth_scroll_delta.x)
        } else {
            0.0
        };
        if wheel != 0.0 {
            offset -= sign * wheel;
            state.animation = None;
            state.last_wheel_time = Some(now);
        }

        // Drag, with single-advance snapping on release.
        if response.drag_started() {
            state.drag_start_index = nearest_snap_index(&strategy, item_count, offset);
            state.animation = None;
        }
        if response.dragged() {
            offset -= sign * response.drag_delta().x;
            state.animation = None;
        }
        if response.drag_stopped() && snapping {
            // Scroll velocity: positive when moving towards later items.
            let velocity = -sign * ui.input(|i| i.pointer.velocity().x);
            let start = state.drag_start_index;
            let mut target = nearest_snap_index(&strategy, item_count, offset)
                .clamp(start.saturating_sub(1), (start + 1).min(last_index));
            if target == start && velocity.abs() > MIN_FLING_VELOCITY {
                target = if velocity > 0.0 {
                    (start + 1).min(last_index)
                } else {
                    start.saturating_sub(1)
                };
            }
            state.animation = Some(snap_to(offset, target, velocity));
        }

        // Keyboard: arrows, Home and End move between items.
        let focused_id = ui.memory(|m| m.focused());
        let focused = focused_id.and_then(|f| (0..item_count).find(|&i| item_id(i) == f));
        if let Some(current) = focused {
            let (next_key, prev_key) = if rtl {
                (Key::ArrowLeft, Key::ArrowRight)
            } else {
                (Key::ArrowRight, Key::ArrowLeft)
            };
            let mut next = current;
            ui.input_mut(|i| {
                if i.consume_key(egui::Modifiers::NONE, next_key) {
                    next = (current + 1).min(last_index);
                }
                if i.consume_key(egui::Modifiers::NONE, prev_key) {
                    next = current.saturating_sub(1);
                }
                if i.consume_key(egui::Modifiers::NONE, Key::Home) {
                    next = 0;
                }
                if i.consume_key(egui::Modifiers::NONE, Key::End) {
                    next = last_index;
                }
            });
            if next != current {
                ui.memory_mut(|m| m.request_focus(item_id(next)));
                state.focused = Some(next);
                state.animation = Some(snap_to(offset, next, 0.0));
            } else if state.focused != Some(current) {
                // Focus arrived (Tab or click): bring the item into view.
                state.focused = Some(current);
                if snapping {
                    state.animation = Some(snap_to(offset, current, 0.0));
                }
            }
        } else {
            state.focused = None;
        }

        // A wheel scroll that has settled snaps to the nearest item.
        if let Some(last) = state.last_wheel_time {
            if !snapping || response.dragged() {
                state.last_wheel_time = None;
            } else if now - last >= WHEEL_SETTLE_SECONDS {
                state.last_wheel_time = None;
                let nearest = nearest_snap_index(&strategy, item_count, offset);
                state.animation = Some(snap_to(offset, nearest, 0.0));
            } else {
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_secs_f64(
                        WHEEL_SETTLE_SECONDS,
                    ));
            }
        }

        if let Some(animation) = state.animation {
            let t = (now - animation.start_time).max(0.0) as f32;
            let (x, v) = SNAP_SPRING.displacement(animation.x0, animation.v0, t);
            if x.abs() < 0.5 && v.abs() < 1.0 {
                offset = animation.target;
                state.animation = None;
            } else {
                offset = animation.target + x;
                ui.ctx().request_repaint();
            }
        }
        offset = offset.clamp(0.0, max);
        *self.scroll_offset = offset;

        // Paint and interact, item by item.
        let to_screen = |r: Rangef| {
            if rtl {
                Rangef::new(outer.right() - r.max, outer.right() - r.min)
            } else {
                Rangef::new(outer.left() + r.min, outer.left() + r.max)
            }
        };
        let clip = outer.intersect(ui.clip_rect());
        let on_surface = theme.color(ColorRole::OnSurface);
        let mut items: Vec<Option<CarouselItem<'a>>> = self.items.into_iter().map(Some).collect();
        let mut clicked = None;
        for layout in layout_items(&strategy, item_count, offset) {
            let index = layout.index;
            let mask = Rect::from_x_y_ranges(to_screen(layout.mask), outer.y_range());
            let content = Rect::from_x_y_ranges(to_screen(layout.content), outer.y_range());
            let visible = mask.intersect(clip);
            if visible.width() <= 0.0 || visible.height() <= 0.0 {
                continue;
            }
            let Some(item) = items.get_mut(index).and_then(Option::take) else {
                continue;
            };
            let half = mask.width().min(mask.height()) / 2.0;
            let radius = self
                .corner_radius
                .map_or_else(|| CornerSize::ExtraLarge.radius(mask), |r| r.min(half));
            let corner = egui::CornerRadius::same(radius.round().clamp(0.0, 255.0) as u8);

            let item_response = ui.interact(visible, item_id(index), Sense::click());
            ui.memory_mut(|m| {
                m.set_focus_lock_filter(
                    item_response.id,
                    egui::EventFilter {
                        horizontal_arrows: true,
                        ..Default::default()
                    },
                );
            });
            let label = item
                .label
                .clone()
                .unwrap_or_else(|| format!("Item {} of {}", index + 1, item_count));
            a11y::describe(
                &item_response,
                Role::ListItem,
                ui.is_enabled(),
                &label,
                None,
            );
            let interaction = InteractionState::from_response(&item_response);
            let painter = ui.painter().with_clip_rect(clip);

            match item.content {
                ItemContent::Text(text) => {
                    let fill = theme.color(ColorRole::SurfaceContainerHighest);
                    painter.rect_filled(mask, corner, fill);
                    paint::paint_state_layer(&painter, mask, corner, on_surface, interaction);
                    painter.with_clip_rect(visible).text(
                        content.center(),
                        Align2::CENTER_CENTER,
                        text,
                        FontId::proportional(14.0),
                        on_surface,
                    );
                }
                ItemContent::Image(source) => {
                    let width = content.width().max(f32::EPSILON);
                    let uv = Rect::from_min_max(
                        egui::pos2((mask.left() - content.left()) / width, 0.0),
                        egui::pos2((mask.right() - content.left()) / width, 1.0),
                    );
                    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(outer));
                    child.set_clip_rect(clip);
                    egui::Image::new(source)
                        .uv(uv)
                        .corner_radius(corner)
                        .paint_at(&child, mask);
                    paint::paint_state_layer(&painter, mask, corner, on_surface, interaction);
                }
                ItemContent::Custom(draw) => {
                    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(content).layout(
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    ));
                    child.set_clip_rect(visible);
                    draw(&mut child, content);
                    paint::paint_state_layer(&painter, mask, corner, on_surface, interaction);
                }
            }
            paint::paint_focus_ring_if_focused(ui.painter(), &item_response, corner);

            if item_response.clicked() {
                clicked = Some(index);
                state.animation = Some(snap_to(offset, index, 0.0));
                ui.ctx().request_repaint();
            }
        }
        ui.ctx().data_mut(|d| d.insert_temp(id, state));

        CarouselResponse { response, clicked }
    }
}

impl egui::Widget for MaterialCarousel<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

/// A new multi-browse carousel scrolled to `scroll_offset`.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let mut offset = 0.0f32;
/// ui.add(egui_material3::carousel(&mut offset)
///     .item_text("First")
///     .item_text("Second"));
/// # });
/// ```
pub fn carousel<'a>(scroll_offset: &'a mut f32) -> MaterialCarousel<'a> {
    MaterialCarousel::new(scroll_offset)
}
