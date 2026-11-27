// ============================================================================
// Module:       carousel::keylines
// Description:  Carousel keylines and the multi-browse, uncontained and hero
//               arrangements, ported from Compose material3.
//
// Dependencies: (none)
// ============================================================================

//! Keylines: where carousel items sit and how big they are.
//!
//! A port of Compose material3's `carousel/Keylines.kt`, `KeylineList.kt`
//! and `Arrangement.kt` (`androidx-main`). A keyline is a fixed point along
//! the scrolling axis; an item whose centre is at a keyline takes that
//! keyline's size, and an item between two keylines interpolates between
//! them. The functions here build the keyline list for each strategy; the
//! math, including its float comparisons, follows Compose line for line so
//! item sizes match Compose's for the same viewport.

/// `CarouselDefaults.AnchorSize`: the size of the off-screen anchor keylines.
pub(crate) const ANCHOR_SIZE: f32 = 10.0;
/// `CarouselDefaults.MediumLargeItemDiffThreshold`.
const MEDIUM_LARGE_ITEM_DIFF_THRESHOLD: f32 = 0.85;
/// `Arrangement.MediumItemFlexPercentage`.
const MEDIUM_ITEM_FLEX_PERCENTAGE: f32 = 0.1;

/// One keyline: the size an item takes when its centre is at `offset`
/// (Compose's `Keyline`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Keyline {
    /// Size of an item centred on this keyline, in points along the axis.
    pub size: f32,
    /// Where the item's centre is drawn, from the viewport's start edge.
    pub offset: f32,
    /// Where the item's centre would be if every item were laid out
    /// end to end at full size.
    pub unadjusted_offset: f32,
    /// An item here is fully visible ("focal", large).
    pub is_focal: bool,
    /// An off-screen anchor keyline that never moves.
    pub is_anchor: bool,
    /// The keyline every other offset was computed from.
    pub is_pivot: bool,
    /// How far an item here bleeds past the viewport edge (0 when it is
    /// fully inside or fully outside).
    pub cutoff: f32,
}

/// Interpolates every field of two keylines (Compose's `lerp(Keyline,...)`).
pub(crate) fn lerp_keyline(start: &Keyline, end: &Keyline, fraction: f32) -> Keyline {
    let pick_end = fraction >= 0.5;
    Keyline {
        size: lerp(start.size, end.size, fraction),
        offset: lerp(start.offset, end.offset, fraction),
        unadjusted_offset: lerp(start.unadjusted_offset, end.unadjusted_offset, fraction),
        is_focal: if pick_end {
            end.is_focal
        } else {
            start.is_focal
        },
        is_anchor: if pick_end {
            end.is_anchor
        } else {
            start.is_anchor
        },
        is_pivot: if pick_end {
            end.is_pivot
        } else {
            start.is_pivot
        },
        cutoff: lerp(start.cutoff, end.cutoff, fraction),
    }
}

pub(crate) fn lerp(start: f32, stop: f32, fraction: f32) -> f32 {
    (1.0 - fraction) * start + fraction * stop
}

/// Kotlin's `coerceIn`, without its panic when `lo > hi` (then `lo` wins).
fn coerce_in(value: f32, lo: f32, hi: f32) -> f32 {
    if value < lo {
        lo
    } else if value > hi {
        hi
    } else {
        value
    }
}

/// An ordered list of keylines (Compose's `KeylineList`).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct KeylineList {
    keylines: Vec<Keyline>,
}

impl KeylineList {
    pub(crate) fn new(keylines: Vec<Keyline>) -> Self {
        Self { keylines }
    }

    pub(crate) fn as_slice(&self) -> &[Keyline] {
        &self.keylines
    }

    pub(crate) fn into_vec(self) -> Vec<Keyline> {
        self.keylines
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.keylines.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.keylines.len()
    }

    pub(crate) fn get(&self, index: usize) -> Option<&Keyline> {
        self.keylines.get(index)
    }

    pub(crate) fn first(&self) -> Option<&Keyline> {
        self.keylines.first()
    }

    pub(crate) fn last(&self) -> Option<&Keyline> {
        self.keylines.last()
    }

    fn position(&self, f: impl Fn(&Keyline) -> bool) -> Option<usize> {
        self.keylines.iter().position(f)
    }

    fn rposition(&self, f: impl Fn(&Keyline) -> bool) -> Option<usize> {
        self.keylines.iter().rposition(f)
    }

    pub(crate) fn pivot_index(&self) -> Option<usize> {
        self.position(|k| k.is_pivot)
    }

    pub(crate) fn first_non_anchor_index(&self) -> Option<usize> {
        self.position(|k| !k.is_anchor)
    }

    pub(crate) fn last_non_anchor_index(&self) -> Option<usize> {
        self.rposition(|k| !k.is_anchor)
    }

    pub(crate) fn first_focal_index(&self) -> Option<usize> {
        self.position(|k| k.is_focal)
    }

    pub(crate) fn last_focal_index(&self) -> Option<usize> {
        self.rposition(|k| k.is_focal)
    }

    pub(crate) fn first_focal(&self) -> Option<&Keyline> {
        self.first_focal_index().and_then(|i| self.get(i))
    }

    pub(crate) fn last_focal(&self) -> Option<&Keyline> {
        self.last_focal_index().and_then(|i| self.get(i))
    }

    /// The number of focal keylines.
    pub(crate) fn focal_count(&self) -> usize {
        match (self.first_focal_index(), self.last_focal_index()) {
            (Some(first), Some(last)) => last.saturating_sub(first) + 1,
            (None, _) | (_, None) => 0,
        }
    }

    /// The first focal item starts inside the viewport and is the first
    /// non-anchor keyline: the focal range cannot shift further left.
    pub(crate) fn is_first_focal_item_at_start_of_container(&self) -> bool {
        let Some(first_focal) = self.first_focal() else {
            return false;
        };
        let first_non_anchor = self.first_non_anchor_index().and_then(|i| self.get(i));
        first_focal.offset - first_focal.size / 2.0 >= 0.0 && Some(first_focal) == first_non_anchor
    }

    /// The last focal item ends inside the viewport and is the last
    /// non-anchor keyline.
    pub(crate) fn is_last_focal_item_at_end_of_container(&self, main_axis_size: f32) -> bool {
        let Some(last_focal) = self.last_focal() else {
            return false;
        };
        let last_non_anchor = self.last_non_anchor_index().and_then(|i| self.get(i));
        last_focal.offset + last_focal.size / 2.0 <= main_axis_size
            && Some(last_focal) == last_non_anchor
    }

    /// The first index from the last focal keyline on whose size is `size`,
    /// or the last index.
    pub(crate) fn first_index_after_focal_range_with_size(&self, size: f32) -> usize {
        let last = self.len().saturating_sub(1);
        let from = self.last_focal_index().unwrap_or(0);
        (from..=last)
            .find(|&i| self.get(i).is_some_and(|k| k.size == size))
            .unwrap_or(last)
    }

    /// The last index before the first focal keyline whose size is `size`,
    /// or 0.
    pub(crate) fn last_index_before_focal_range_with_size(&self, size: f32) -> usize {
        let Some(first_focal) = self.first_focal_index() else {
            return 0;
        };
        (0..first_focal)
            .rev()
            .find(|&i| self.get(i).is_some_and(|k| k.size == size))
            .unwrap_or(0)
    }

    /// The last keyline whose unadjusted offset is below `unadjusted_offset`,
    /// or the first keyline.
    pub(crate) fn keyline_before(&self, unadjusted_offset: f32) -> Option<&Keyline> {
        self.keylines
            .iter()
            .rev()
            .find(|k| k.unadjusted_offset < unadjusted_offset)
            .or_else(|| self.first())
    }

    /// The first keyline whose unadjusted offset is at or above
    /// `unadjusted_offset`, or the last keyline.
    pub(crate) fn keyline_after(&self, unadjusted_offset: f32) -> Option<&Keyline> {
        self.keylines
            .iter()
            .find(|k| k.unadjusted_offset >= unadjusted_offset)
            .or_else(|| self.last())
    }

    /// Interpolates two lists keyline by keyline.
    pub(crate) fn lerp(from: &KeylineList, to: &KeylineList, fraction: f32) -> KeylineList {
        KeylineList::new(
            from.keylines
                .iter()
                .zip(to.keylines.iter())
                .map(|(a, b)| lerp_keyline(a, b, fraction))
                .collect(),
        )
    }
}

/// Where the focal range sits in the viewport (Compose's
/// `CarouselAlignment`; `End` is not used by any strategy).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CarouselAlignment {
    /// Focal items at the start.
    Start,
    /// Focal items in the middle.
    Center,
}

/// Collects keyline sizes in order, then lays them out from a pivot
/// (Compose's `KeylineListScopeImpl`).
#[derive(Default)]
pub(crate) struct KeylineListBuilder {
    tmp: Vec<(f32, bool)>,
    first_focal_index: Option<usize>,
    focal_item_size: f32,
}

impl KeylineListBuilder {
    pub(crate) fn add(&mut self, size: f32, is_anchor: bool) {
        self.tmp.push((size, is_anchor));
        if !is_anchor && size > self.focal_item_size {
            self.first_focal_index = Some(self.tmp.len() - 1);
            self.focal_item_size = size;
        }
    }

    fn last_focal_index(&self) -> Option<usize> {
        let first = self.first_focal_index?;
        let mut last = first;
        while self
            .tmp
            .get(last + 1)
            .is_some_and(|&(size, _)| size == self.focal_item_size)
        {
            last += 1;
        }
        Some(last)
    }

    pub(crate) fn create_with_alignment(
        self,
        main_axis_size: f32,
        item_spacing: f32,
        alignment: CarouselAlignment,
    ) -> KeylineList {
        let (Some(first_focal), Some(last_focal)) =
            (self.first_focal_index, self.last_focal_index())
        else {
            return KeylineList::default();
        };
        // Compose's `focalItemCount` is the index distance, not the count.
        let focal_item_count = last_focal - first_focal;
        let pivot_offset = match alignment {
            CarouselAlignment::Center => {
                let spacing_split = if item_spacing == 0.0 || focal_item_count % 2 == 0 {
                    0.0
                } else {
                    item_spacing / 2.0
                };
                let item_space_counts = (focal_item_count / 2) as f32 * item_spacing;
                main_axis_size / 2.0
                    - (self.focal_item_size / 2.0) * focal_item_count as f32
                    - spacing_split
                    - item_space_counts
            }
            CarouselAlignment::Start => self.focal_item_size / 2.0,
        };
        self.create_with_pivot(main_axis_size, item_spacing, first_focal, pivot_offset)
    }

    pub(crate) fn create_with_pivot(
        self,
        main_axis_size: f32,
        item_spacing: f32,
        pivot_index: usize,
        pivot_offset: f32,
    ) -> KeylineList {
        let last_focal = self.last_focal_index();
        let focal_range = |i: usize| match (self.first_focal_index, last_focal) {
            (Some(first), Some(last)) => first <= i && i <= last,
            (None, _) | (_, None) => false,
        };
        let Some(&(pivot_size, pivot_anchor)) = self.tmp.get(pivot_index) else {
            return KeylineList::default();
        };
        let item_size = self.focal_item_size;
        let cutoff_left =
            |size: f32, offset: f32| offset - size / 2.0 < 0.0 && offset + size / 2.0 > 0.0;
        let cutoff_right = |size: f32, offset: f32| {
            offset - size / 2.0 < main_axis_size && offset + size / 2.0 > main_axis_size
        };

        let pivot_cutoff = if cutoff_left(pivot_size, pivot_offset) {
            pivot_offset - pivot_size / 2.0
        } else if cutoff_right(pivot_size, pivot_offset) {
            pivot_offset + pivot_size / 2.0 - main_axis_size
        } else {
            0.0
        };

        let mut before = Vec::new();
        let mut offset = pivot_offset - item_size / 2.0 - item_spacing;
        let mut unadjusted = pivot_offset - item_size / 2.0 - item_spacing;
        for index in (0..pivot_index).rev() {
            let Some(&(size, is_anchor)) = self.tmp.get(index) else {
                continue;
            };
            let k_offset = offset - size / 2.0;
            let k_unadjusted = unadjusted - item_size / 2.0;
            let cutoff = if cutoff_left(size, k_offset) {
                (k_offset - size / 2.0).abs()
            } else {
                0.0
            };
            before.push(Keyline {
                size,
                offset: k_offset,
                unadjusted_offset: k_unadjusted,
                is_focal: focal_range(index),
                is_anchor,
                is_pivot: false,
                cutoff,
            });
            offset -= size + item_spacing;
            unadjusted -= item_size + item_spacing;
        }
        before.reverse();

        let mut keylines = before;
        keylines.push(Keyline {
            size: pivot_size,
            offset: pivot_offset,
            unadjusted_offset: pivot_offset,
            is_focal: focal_range(pivot_index),
            is_anchor: pivot_anchor,
            is_pivot: true,
            cutoff: pivot_cutoff,
        });

        let mut offset = pivot_offset + item_size / 2.0 + item_spacing;
        let mut unadjusted = pivot_offset + item_size / 2.0 + item_spacing;
        for (index, &(size, is_anchor)) in self.tmp.iter().enumerate().skip(pivot_index + 1) {
            let k_offset = offset + size / 2.0;
            let k_unadjusted = unadjusted + item_size / 2.0;
            let cutoff = if cutoff_right(size, k_offset) {
                k_offset + size / 2.0 - main_axis_size
            } else {
                0.0
            };
            keylines.push(Keyline {
                size,
                offset: k_offset,
                unadjusted_offset: k_unadjusted,
                is_focal: focal_range(index),
                is_anchor,
                is_pivot: false,
                cutoff,
            });
            offset += size + item_spacing;
            unadjusted += item_size + item_spacing;
        }
        KeylineList::new(keylines)
    }
}

/// A combination of large, medium and small items (Compose's
/// `Arrangement`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Arrangement {
    priority: u32,
    pub(crate) small_size: f32,
    pub(crate) small_count: usize,
    pub(crate) medium_size: f32,
    pub(crate) medium_count: usize,
    pub(crate) large_size: f32,
    pub(crate) large_count: usize,
}

impl Arrangement {
    fn is_valid(&self) -> bool {
        if self.large_count > 0 && self.small_count > 0 && self.medium_count > 0 {
            self.large_size > self.medium_size && self.medium_size > self.small_size
        } else if self.large_count > 0 && self.small_count > 0 {
            self.large_size > self.small_size
        } else {
            true
        }
    }

    fn cost(&self, target_large_size: f32) -> f32 {
        if !self.is_valid() {
            return f32::MAX;
        }
        (target_large_size - self.large_size).abs() * self.priority as f32
    }

    pub(crate) fn item_count(&self) -> usize {
        self.large_count + self.medium_count + self.small_count
    }

    /// The lowest-cost arrangement of every count permutation, each fitted
    /// to `available_space` (Compose's `findLowestCostArrangement`).
    #[expect(
        clippy::too_many_arguments,
        reason = "mirrors Compose's findLowestCostArrangement parameter list"
    )]
    pub(crate) fn find_lowest_cost(
        available_space: f32,
        item_spacing: f32,
        target_small_size: f32,
        min_small_size: f32,
        max_small_size: f32,
        small_counts: &[usize],
        target_medium_size: f32,
        medium_counts: &[usize],
        target_large_size: f32,
        large_counts: &[usize],
    ) -> Option<Arrangement> {
        let mut lowest: Option<Arrangement> = None;
        let mut priority = 1;
        for &large_count in large_counts {
            for &medium_count in medium_counts {
                for &small_count in small_counts {
                    let arrangement = Self::fit(
                        priority,
                        available_space,
                        item_spacing,
                        (
                            small_count,
                            target_small_size,
                            min_small_size,
                            max_small_size,
                        ),
                        (medium_count, target_medium_size),
                        (large_count, target_large_size),
                    );
                    let better = match &lowest {
                        None => true,
                        Some(best) => {
                            arrangement.cost(target_large_size) < best.cost(target_large_size)
                        }
                    };
                    if better {
                        lowest = Some(arrangement);
                        if arrangement.cost(target_large_size) == 0.0 {
                            return lowest;
                        }
                    }
                    priority += 1;
                }
            }
        }
        lowest
    }

    /// Fits the counts into `available_space`, adjusting small items first,
    /// then medium, then large (Compose's `Arrangement.fit`).
    fn fit(
        priority: u32,
        available_space: f32,
        item_spacing: f32,
        (small_count, small_size, min_small_size, max_small_size): (usize, f32, f32, f32),
        (medium_count, medium_size): (usize, f32),
        (large_count, large_size): (usize, f32),
    ) -> Arrangement {
        let total = (large_count + medium_count + small_count) as f32;
        let space = available_space - (total - 1.0) * item_spacing;
        let (small_n, medium_n, large_n) =
            (small_count as f32, medium_count as f32, large_count as f32);
        let mut small = coerce_in(small_size, min_small_size, max_small_size);
        let taken = large_size * large_n + medium_size * medium_n + small * small_n;
        let delta = space - taken;
        if small_count > 0 && delta > 0.0 {
            small += (delta / small_n).min(max_small_size - small);
        } else if small_count > 0 && delta < 0.0 {
            small += (delta / small_n).max(min_small_size - small);
        }
        if small_count == 0 {
            small = 0.0;
        }
        let mut large = (space - (small_n + medium_n / 2.0) * small) / (large_n + medium_n / 2.0);
        let mut medium = (large + small) / 2.0;
        if medium_count > 0 && large != large_size {
            let target_adjustment = (large_size - large) * large_n;
            let available_flex = medium * MEDIUM_ITEM_FLEX_PERCENTAGE * medium_n;
            let distribute = target_adjustment.abs().min(available_flex);
            if target_adjustment > 0.0 {
                medium -= distribute / medium_n;
                large += distribute / large_n;
            } else {
                medium += distribute / medium_n;
                large -= distribute / large_n;
            }
        }
        Arrangement {
            priority,
            small_size: small,
            small_count,
            medium_size: medium,
            medium_count,
            large_size: large,
            large_count,
        }
    }
}

/// Large counts from `max` down to `min` (Compose's
/// `IntArray(max - min + 1) { max - it }`).
fn large_counts(min: usize, max: usize) -> Vec<usize> {
    (min..=max).rev().collect()
}

fn floor_count(value: f32) -> usize {
    // `as` saturates: NaN and negatives become 0.
    value.floor() as usize
}

fn ceil_count(value: f32) -> usize {
    value.ceil() as usize
}

/// Multi-browse: large items, then a medium and a small one (Compose's
/// `multiBrowseKeylineList`).
pub(crate) fn multi_browse(
    main_axis_size: f32,
    preferred_item_size: f32,
    item_spacing: f32,
    item_count: usize,
    min_small_item_size: f32,
    max_small_item_size: f32,
) -> KeylineList {
    if main_axis_size == 0.0 || preferred_item_size == 0.0 {
        return KeylineList::default();
    }
    let mut small_counts: Vec<usize> = vec![1];
    let medium_counts: [usize; 2] = [1, 0];

    let target_large = preferred_item_size.min(main_axis_size);
    let target_small = coerce_in(target_large / 3.0, min_small_item_size, max_small_item_size);
    let target_medium = (target_large + target_small) / 2.0;

    if main_axis_size < min_small_item_size * 2.0 {
        small_counts = vec![0];
    }

    let max_small_count = small_counts.iter().copied().max().unwrap_or(0) as f32;
    let max_medium_count = medium_counts.iter().copied().max().unwrap_or(0) as f32;
    let min_available_large_space =
        main_axis_size - target_medium * max_medium_count - max_small_item_size * max_small_count;
    let min_large_count = floor_count(min_available_large_space / target_large).max(1);
    let max_large_count = ceil_count(main_axis_size / target_large);
    let counts = large_counts(min_large_count, max_large_count);

    let mut arrangement = Arrangement::find_lowest_cost(
        main_axis_size,
        item_spacing,
        target_small,
        min_small_item_size,
        max_small_item_size,
        &small_counts,
        target_medium,
        &medium_counts,
        target_large,
        &counts,
    );

    if let Some(found) = arrangement {
        if found.item_count() > item_count {
            let mut surplus = found.item_count() - item_count;
            let mut small_count = found.small_count;
            let mut medium_count = found.medium_count;
            while surplus > 0 {
                if small_count > 0 {
                    small_count -= 1;
                } else if medium_count > 1 {
                    medium_count -= 1;
                }
                surplus -= 1;
            }
            arrangement = Arrangement::find_lowest_cost(
                main_axis_size,
                item_spacing,
                target_small,
                min_small_item_size,
                max_small_item_size,
                &[small_count],
                target_medium,
                &[medium_count],
                target_large,
                &counts,
            );
        }
    }

    match arrangement {
        Some(arrangement) => left_aligned(
            main_axis_size,
            item_spacing,
            ANCHOR_SIZE,
            ANCHOR_SIZE,
            &arrangement,
        ),
        None => KeylineList::default(),
    }
}

/// Uncontained: as many full items as fit, then one cut-off item (Compose's
/// `uncontainedKeylineList`).
pub(crate) fn uncontained(main_axis_size: f32, item_size: f32, item_spacing: f32) -> KeylineList {
    if main_axis_size == 0.0 || item_size == 0.0 {
        return KeylineList::default();
    }
    let large_item_size = (item_size + item_spacing).min(main_axis_size);
    let large_count = floor_count(main_axis_size / large_item_size).max(1);
    let remaining = main_axis_size - large_count as f32 * large_item_size;
    let medium_count = usize::from(remaining > 0.0);
    let medium_item_size = medium_child_size(ANCHOR_SIZE, large_item_size, remaining);
    let arrangement = Arrangement {
        priority: 0,
        small_size: 0.0,
        small_count: 0,
        medium_size: medium_item_size,
        medium_count,
        large_size: large_item_size,
        large_count,
    };
    let x_small_size = ANCHOR_SIZE.min(item_size);
    let left_anchor_size = x_small_size.max(medium_item_size * 0.5);
    left_aligned(
        main_axis_size,
        item_spacing,
        left_anchor_size,
        ANCHOR_SIZE,
        &arrangement,
    )
}

/// Hero: one (or more) large items with small items beside them (Compose's
/// `heroKeylineList`). `preferred_item_size` of `None` fills the viewport
/// less the small items.
pub(crate) fn hero(
    main_axis_size: f32,
    preferred_item_size: Option<f32>,
    item_spacing: f32,
    item_count: usize,
    is_centered: bool,
    min_small_item_size: f32,
    max_small_item_size: f32,
) -> KeylineList {
    if main_axis_size == 0.0 {
        return KeylineList::default();
    }
    let should_center = is_centered && item_count >= 3;
    let mut small_counts: usize = if item_count <= 1 {
        0
    } else if should_center {
        2
    } else {
        1
    };
    let target_large = preferred_item_size
        .unwrap_or(main_axis_size)
        .min(main_axis_size);
    let target_small = coerce_in(target_large / 3.0, min_small_item_size, max_small_item_size);
    let fullscreen_threshold =
        min_small_item_size * small_counts as f32 + min_small_item_size * 1.25;
    if main_axis_size < fullscreen_threshold {
        small_counts = 0;
    }
    let min_available_large_space = main_axis_size - min_small_item_size * small_counts as f32;
    let min_large_count = floor_count(min_available_large_space / target_large).max(1);
    let max_large_count = ceil_count(main_axis_size / target_large);
    let counts = large_counts(min_large_count, max_large_count);
    let Some(arrangement) = Arrangement::find_lowest_cost(
        main_axis_size,
        item_spacing,
        target_small,
        min_small_item_size,
        max_small_item_size,
        &[small_counts],
        0.0,
        &[0],
        target_large,
        &counts,
    ) else {
        return KeylineList::default();
    };
    if should_center && item_count >= arrangement.item_count() {
        center_aligned(main_axis_size, item_spacing, &arrangement)
    } else {
        left_aligned(
            main_axis_size,
            item_spacing,
            ANCHOR_SIZE,
            ANCHOR_SIZE,
            &arrangement,
        )
    }
}

fn left_aligned(
    main_axis_size: f32,
    item_spacing: f32,
    left_anchor_size: f32,
    right_anchor_size: f32,
    arrangement: &Arrangement,
) -> KeylineList {
    let mut builder = KeylineListBuilder::default();
    builder.add(left_anchor_size, true);
    for _ in 0..arrangement.large_count {
        builder.add(arrangement.large_size, false);
    }
    for _ in 0..arrangement.medium_count {
        builder.add(arrangement.medium_size, false);
    }
    for _ in 0..arrangement.small_count {
        builder.add(arrangement.small_size, false);
    }
    builder.add(right_anchor_size, true);
    builder.create_with_alignment(main_axis_size, item_spacing, CarouselAlignment::Start)
}

fn center_aligned(
    main_axis_size: f32,
    item_spacing: f32,
    arrangement: &Arrangement,
) -> KeylineList {
    let mut builder = KeylineListBuilder::default();
    builder.add(ANCHOR_SIZE, true);
    for _ in 0..arrangement.small_count / 2 {
        builder.add(arrangement.small_size, false);
    }
    for _ in 0..arrangement.medium_count / 2 {
        builder.add(arrangement.medium_size, false);
    }
    for _ in 0..arrangement.large_count {
        builder.add(arrangement.large_size, false);
    }
    for _ in 0..arrangement.medium_count / 2 {
        builder.add(arrangement.medium_size, false);
    }
    for _ in 0..arrangement.small_count / 2 {
        builder.add(arrangement.small_size, false);
    }
    builder.add(ANCHOR_SIZE, true);
    builder.create_with_alignment(main_axis_size, item_spacing, CarouselAlignment::Center)
}

/// Compose's `calculateMediumChildSize`.
fn medium_child_size(minimum_medium_size: f32, large_item_size: f32, remaining_space: f32) -> f32 {
    let mut medium = minimum_medium_size.max(remaining_space * 1.5);
    let threshold = large_item_size * MEDIUM_LARGE_ITEM_DIFF_THRESHOLD;
    if medium > threshold {
        let size_with_fifth_cut_off = remaining_space * 1.2;
        medium = threshold.max(size_with_fifth_cut_off).min(large_item_size);
    }
    medium
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sizes(list: &KeylineList) -> Vec<f32> {
        list.as_slice().iter().map(|k| k.size).collect()
    }

    #[test]
    fn coerce_in_never_panics() {
        assert_eq!(coerce_in(5.0, 10.0, 1.0), 10.0, "lo wins when lo > hi");
        assert!(coerce_in(f32::NAN, 1.0, 2.0).is_nan(), "NaN passes");
    }

    #[test]
    fn empty_inputs_give_no_keylines() {
        assert!(
            multi_browse(0.0, 100.0, 0.0, 5, 40.0, 56.0).is_empty(),
            "no width"
        );
        assert!(uncontained(300.0, 0.0, 0.0).is_empty(), "no item size");
        assert!(
            hero(0.0, None, 0.0, 5, true, 40.0, 56.0).is_empty(),
            "no width"
        );
    }

    #[test]
    fn anchors_bracket_every_list() {
        let list = multi_browse(400.0, 200.0, 8.0, 10, 40.0, 56.0);
        let s = sizes(&list);
        assert_eq!(s.first(), Some(&ANCHOR_SIZE), "leading anchor");
        assert_eq!(s.last(), Some(&ANCHOR_SIZE), "trailing anchor");
        assert!(
            list.first().is_some_and(|k| k.is_anchor) && list.last().is_some_and(|k| k.is_anchor),
            "anchors flagged"
        );
    }
}
