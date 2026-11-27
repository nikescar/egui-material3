// ============================================================================
// Module:       carousel::strategy
// Description:  Scroll-aware keyline lists and snap positions, ported from
//               Compose material3's carousel Strategy.
//
// Dependencies: super::keylines
// ============================================================================

//! The scroll side of the carousel: a port of Compose material3's
//! `carousel/Strategy.kt` and `KeylineSnapPosition.kt` (`androidx-main`).
//!
//! From the default keylines, [`Strategy`] derives the "steps" that move the
//! focal range to the start of the viewport when scrolled to the start of
//! the list, and to the end when scrolled to the end, so every item becomes
//! large at some scroll position. Content padding (zero by default in
//! Compose) is not supported.

use super::keylines::{lerp, Keyline, KeylineList, KeylineListBuilder};

/// Keylines corrected for scroll offset (Compose's `Strategy`).
#[derive(Clone, Debug, Default)]
pub(crate) struct Strategy {
    pub(crate) default_keylines: KeylineList,
    pub(crate) start_steps: Vec<KeylineList>,
    pub(crate) end_steps: Vec<KeylineList>,
    pub(crate) available_space: f32,
    pub(crate) item_spacing: f32,
    start_shift_distance: f32,
    end_shift_distance: f32,
    start_shift_points: Vec<f32>,
    end_shift_points: Vec<f32>,
}

impl Strategy {
    pub(crate) fn new(
        default_keylines: KeylineList,
        available_space: f32,
        item_spacing: f32,
    ) -> Self {
        let start_steps = start_keyline_steps(&default_keylines, available_space, item_spacing);
        let end_steps = end_keyline_steps(&default_keylines, available_space, item_spacing);
        let start_shift_distance = match (start_steps.first(), start_steps.last()) {
            (Some(first), Some(last)) => {
                (first_unadjusted(last) - first_unadjusted(first)).max(0.0)
            }
            (None, _) | (_, None) => 0.0,
        };
        let end_shift_distance = match (end_steps.first(), end_steps.last()) {
            (Some(first), Some(last)) => (last_unadjusted(first) - last_unadjusted(last)).max(0.0),
            (None, _) | (_, None) => 0.0,
        };
        let start_shift_points = interpolation_points(start_shift_distance, &start_steps, true);
        let end_shift_points = interpolation_points(end_shift_distance, &end_steps, false);
        Self {
            default_keylines,
            start_steps,
            end_steps,
            available_space,
            item_spacing,
            start_shift_distance,
            end_shift_distance,
            start_shift_points,
            end_shift_points,
        }
    }

    /// The size of focal (fully unmasked) items.
    pub(crate) fn item_main_axis_size(&self) -> f32 {
        self.default_keylines.first_focal().map_or(0.0, |k| k.size)
    }

    pub(crate) fn is_valid(&self) -> bool {
        !self.default_keylines.is_empty()
            && self.available_space != 0.0
            && self.item_main_axis_size() != 0.0
    }

    /// The keylines for `scroll_offset` (Compose's
    /// `getKeylineListForScrollOffset`).
    pub(crate) fn keylines_for_scroll(
        &self,
        scroll_offset: f32,
        max_scroll_offset: f32,
        round_to_nearest_step: bool,
    ) -> KeylineList {
        let position = scroll_offset.max(0.0);
        let start_shift_offset = self.start_shift_distance;
        let end_shift_offset = (max_scroll_offset - self.end_shift_distance).max(0.0);
        if start_shift_offset <= position && position <= end_shift_offset {
            return self.default_keylines.clone();
        }

        let mut interpolation = lerp_range(1.0, 0.0, 0.0, start_shift_offset, position);
        let mut points: &[f32] = &self.start_shift_points;
        let mut steps: Vec<&KeylineList> = self.start_steps.iter().collect();
        if position > end_shift_offset {
            interpolation = lerp_range(0.0, 1.0, end_shift_offset, max_scroll_offset, position);
            points = &self.end_shift_points;
            steps = self.end_steps.iter().collect();
            if end_shift_offset < 0.01 && self.start_steps.len() == 2 && self.end_steps.len() == 2 {
                if let (Some(start), Some(end)) = (self.start_steps.last(), self.end_steps.last()) {
                    steps = vec![start, end];
                }
            }
        }

        let mut from = 0;
        let mut to = 0;
        let mut stepped = 0.0;
        let mut lower = points.first().copied().unwrap_or(0.0);
        for i in 1..steps.len() {
            let Some(&upper) = points.get(i) else {
                break;
            };
            if interpolation <= upper {
                from = i - 1;
                to = i;
                stepped = lerp_range(0.0, 1.0, lower, upper, interpolation);
                break;
            }
            lower = upper;
        }

        let (Some(from_step), Some(to_step)) = (steps.get(from), steps.get(to)) else {
            return self.default_keylines.clone();
        };
        if round_to_nearest_step {
            return if stepped.round() == 0.0 {
                (*from_step).clone()
            } else {
                (*to_step).clone()
            };
        }
        KeylineList::lerp(from_step, to_step, stepped)
    }

    /// The scroll offset at which item `index` rests in a focal position
    /// (Compose's `getSnapPositionOffset`), before it is subtracted from the
    /// item's end-to-end position.
    pub(crate) fn snap_position_offset(&self, index: usize, item_count: usize) -> f32 {
        if !self.is_valid() {
            return 0.0;
        }
        let half = self.item_main_axis_size() / 2.0;
        let mut offset = self
            .default_keylines
            .first_focal()
            .map_or(0.0, |k| (k.unadjusted_offset - half).round());
        let start_last = self.start_steps.len().saturating_sub(1);
        if !self.start_steps.is_empty() && index <= start_last {
            let step = start_last.saturating_sub(index).min(start_last);
            if let Some(focal) = self
                .start_steps
                .get(step)
                .and_then(KeylineList::first_focal)
            {
                offset = (focal.unadjusted_offset - half).round();
            }
        }
        let end_last = self.end_steps.len().saturating_sub(1);
        let last_item = item_count.saturating_sub(1);
        if !self.end_steps.is_empty()
            && index + end_last >= last_item
            && item_count > self.default_keylines.focal_count()
        {
            let from_end = last_item.saturating_sub(index);
            let step = end_last.saturating_sub(from_end).min(end_last);
            if let Some(focal) = self.end_steps.get(step).and_then(KeylineList::last_focal) {
                offset = (focal.unadjusted_offset - half).round();
            }
        }
        offset
    }
}

fn first_unadjusted(list: &KeylineList) -> f32 {
    list.first().map_or(0.0, |k| k.unadjusted_offset)
}

fn last_unadjusted(list: &KeylineList) -> f32 {
    list.last().map_or(0.0, |k| k.unadjusted_offset)
}

/// Compose's `lerp(outputMin, outputMax, inputMin, inputMax, value)`.
pub(crate) fn lerp_range(
    output_min: f32,
    output_max: f32,
    input_min: f32,
    input_max: f32,
    value: f32,
) -> f32 {
    if value <= input_min {
        output_min
    } else if value >= input_max {
        output_max
    } else {
        lerp(
            output_min,
            output_max,
            (value - input_min) / (input_max - input_min),
        )
    }
}

/// Compose's `getStartKeylineSteps`: move the keyline at the start of the
/// viewport to after the focal range, one step at a time, until the first
/// focal item is at the start.
fn start_keyline_steps(
    default: &KeylineList,
    main_axis_size: f32,
    item_spacing: f32,
) -> Vec<KeylineList> {
    if default.is_empty() {
        return Vec::new();
    }
    let mut steps = vec![default.clone()];
    if default.is_first_focal_item_at_start_of_container() {
        return steps;
    }
    let (Some(start_index), Some(end_index)) = (
        default.first_non_anchor_index(),
        default.first_focal_index(),
    ) else {
        return steps;
    };
    let number_of_steps = end_index.saturating_sub(start_index);
    if number_of_steps == 0 {
        if default.first_focal().is_some_and(|k| k.cutoff > 0.0) {
            steps.push(move_keyline(default, 0, 0, main_axis_size, item_spacing));
        }
        return steps;
    }
    for i in 0..number_of_steps {
        let Some(prev) = steps.last() else {
            break;
        };
        let original_item_index = start_index + i;
        let mut dst = default.len().saturating_sub(1);
        if original_item_index > 0 {
            if let Some(neighbor) = default.get(original_item_index - 1) {
                dst = prev
                    .first_index_after_focal_range_with_size(neighbor.size)
                    .saturating_sub(1);
            }
        }
        let next = move_keyline(prev, start_index, dst, main_axis_size, item_spacing);
        steps.push(next);
    }
    steps
}

/// Compose's `getEndKeylineSteps`: the mirror of [`start_keyline_steps`].
fn end_keyline_steps(
    default: &KeylineList,
    main_axis_size: f32,
    item_spacing: f32,
) -> Vec<KeylineList> {
    if default.is_empty() {
        return Vec::new();
    }
    let mut steps = vec![default.clone()];
    if default.is_last_focal_item_at_end_of_container(main_axis_size) {
        return steps;
    }
    let (Some(start_index), Some(end_index)) =
        (default.last_focal_index(), default.last_non_anchor_index())
    else {
        return steps;
    };
    let number_of_steps = end_index.saturating_sub(start_index);
    if number_of_steps == 0 {
        if default.last_focal().is_some_and(|k| k.cutoff > 0.0) {
            steps.push(move_keyline(default, 0, 0, main_axis_size, item_spacing));
        }
        return steps;
    }
    for i in 0..number_of_steps {
        let Some(prev) = steps.last() else {
            break;
        };
        let original_item_index = end_index - i;
        let mut dst = 0;
        if original_item_index + 1 < default.len() {
            if let Some(neighbor) = default.get(original_item_index + 1) {
                dst = prev.last_index_before_focal_range_with_size(neighbor.size) + 1;
            }
        }
        let next = move_keyline(prev, end_index, dst, main_axis_size, item_spacing);
        steps.push(next);
    }
    steps
}

/// Compose's `moveKeylineAndCreateShiftedKeylineList`.
fn move_keyline(
    from: &KeylineList,
    src: usize,
    dst: usize,
    main_axis_size: f32,
    item_spacing: f32,
) -> KeylineList {
    let (Some(moved), Some(pivot_index)) = (from.get(src).copied(), from.pivot_index()) else {
        return KeylineList::default();
    };
    let Some(pivot) = from.get(pivot_index) else {
        return KeylineList::default();
    };
    let shift_right = src > dst;
    let delta = moved.size - moved.cutoff + item_spacing;
    let (new_pivot_index, new_pivot_offset) = if shift_right {
        (Some(pivot_index + 1), pivot.offset + delta)
    } else {
        (pivot_index.checked_sub(1), pivot.offset - delta)
    };
    let Some(new_pivot_index) = new_pivot_index else {
        return KeylineList::default();
    };
    let mut keylines: Vec<Keyline> = from.as_slice().to_vec();
    if src < keylines.len() {
        let k = keylines.remove(src);
        keylines.insert(dst.min(keylines.len()), k);
    }
    let mut builder = KeylineListBuilder::default();
    for k in &keylines {
        builder.add(k.size, k.is_anchor);
    }
    builder.create_with_pivot(
        main_axis_size,
        item_spacing,
        new_pivot_index,
        new_pivot_offset,
    )
}

/// Compose's `getStepInterpolationPoints`.
fn interpolation_points(total: f32, steps: &[KeylineList], shifting_left: bool) -> Vec<f32> {
    let mut points = vec![0.0];
    if total == 0.0 || steps.is_empty() {
        return points;
    }
    for (i, pair) in steps.windows(2).enumerate() {
        let [prev, curr] = pair else {
            continue;
        };
        let distance = if shifting_left {
            first_unadjusted(curr) - first_unadjusted(prev)
        } else {
            last_unadjusted(prev) - last_unadjusted(curr)
        };
        let previous = points.last().copied().unwrap_or(0.0);
        let point = if i + 2 == steps.len() {
            1.0
        } else {
            previous + distance / total
        };
        points.push(point);
    }
    points
}

#[cfg(test)]
mod tests {
    //! Ported from Compose's `KeylineSnapPositionTest.kt` and
    //! `StrategyTest.kt` (`androidHostTest`, `androidx-main`).

    use super::super::keylines::CarouselAlignment;
    use super::*;

    fn strategy(sizes: &[(f32, bool)], alignment: CarouselAlignment) -> Strategy {
        let mut builder = KeylineListBuilder::default();
        for &(size, anchor) in sizes {
            builder.add(size, anchor);
        }
        Strategy::new(
            builder.create_with_alignment(1000.0, 0.0, alignment),
            1000.0,
            0.0,
        )
    }

    fn snaps(strategy: &Strategy, count: usize) -> Vec<f32> {
        (0..count)
            .map(|i| strategy.snap_position_offset(i, count))
            .collect()
    }

    #[test]
    fn center_aligned_snap_positions() {
        let s = strategy(
            &[
                (5.0, true),
                (100.0, false),
                (200.0, false),
                (400.0, false),
                (200.0, false),
                (100.0, false),
                (5.0, true),
            ],
            CarouselAlignment::Center,
        );
        assert_eq!(
            snaps(&s, 6),
            vec![0.0, 200.0, 300.0, 300.0, 400.0, 600.0],
            "testCenterAlignedSnapPosition_singleFocalItem"
        );
    }

    #[test]
    fn start_aligned_snap_positions() {
        let s = strategy(
            &[
                (5.0, true),
                (400.0, false),
                (200.0, false),
                (200.0, false),
                (100.0, false),
                (100.0, false),
                (5.0, true),
            ],
            CarouselAlignment::Start,
        );
        assert_eq!(
            snaps(&s, 6),
            vec![0.0, 0.0, 100.0, 200.0, 400.0, 600.0],
            "testSnapPosition_forStartAlignedStrategy"
        );
    }

    #[test]
    fn start_aligned_multiple_focal_snap_positions() {
        let s = strategy(
            &[
                (5.0, true),
                (400.0, false),
                (400.0, false),
                (125.0, false),
                (75.0, false),
                (5.0, true),
            ],
            CarouselAlignment::Start,
        );
        assert_eq!(
            snaps(&s, 5),
            vec![0.0, 0.0, 400.0, 475.0, 600.0],
            "testSnapPosition_forStartAlignedStrategyWithMultipleFocal"
        );
        assert_eq!(
            snaps(&s, 4),
            vec![0.0, 400.0, 475.0, 600.0],
            "...WithMultipleFocalAndLessItems"
        );
        assert_eq!(snaps(&s, 1), vec![0.0], "...WithLessItemsThanFocal");
    }

    #[test]
    fn scrolled_to_either_end_every_item_reaches_focal_size() {
        let s = strategy(
            &[
                (5.0, true),
                (400.0, false),
                (200.0, false),
                (200.0, false),
                (100.0, false),
                (100.0, false),
                (5.0, true),
            ],
            CarouselAlignment::Start,
        );
        let last = s.keylines_for_scroll(600.0, 600.0, false);
        assert_eq!(
            last.last_focal().map(|k| k.offset + k.size / 2.0),
            Some(1000.0),
            "at the end the focal item ends at the viewport edge"
        );
    }
}
