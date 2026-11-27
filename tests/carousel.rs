// ============================================================================
// Module:       carousel (integration test)
// Description:  Carousel keyline math against Compose's tests, layout,
//               keyboard, snapping, AccessKit and snapshots.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! The carousel (WP-14). The keyline expectations are the values in
//! Compose material3's `MultiBrowseTest.kt`, `UncontainedTest.kt` and
//! `CenteredHeroTest.kt` (`androidHostTest`, `androidx-main`), so item
//! widths match Compose's for the same viewport.

mod support;

use std::cell::Cell;

use egui::accesskit::Role;
use egui::Key;
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use egui_material3::carousel::Keyline;
use egui_material3::tokens::ColorRole;
use egui_material3::{CarouselMetrics, CarouselStrategy, MaterialCarousel, MaterialTheme};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn metrics(width: f32, count: usize, spacing: f32) -> CarouselMetrics {
    CarouselMetrics {
        item_spacing: spacing,
        ..CarouselMetrics::new(width, count)
    }
}

fn sizes(keylines: &[Keyline]) -> Vec<f32> {
    keylines.iter().map(|k| k.size).collect()
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

#[test]
fn multi_browse_adjusts_for_item_spacing() -> TestResult {
    // MultiBrowseTest.testMultiBrowse_adjustsForItemSpacing.
    let strategy = CarouselStrategy::MultiBrowse {
        preferred_item_width: 186.0,
    };
    let m = metrics(380.0, 10, 8.0);
    let keylines = strategy.keylines(&m);
    let focal = keylines
        .iter()
        .find(|k| k.is_focal)
        .ok_or("focal keyline")?;
    assert_eq!(focal.size, 186.0, "large items keep their preferred width");
    assert_eq!(focal.offset, 93.0, "first large item at the start");
    let last = keylines
        .iter()
        .rev()
        .find(|k| !k.is_anchor)
        .ok_or("last visible keyline")?;
    assert_eq!(
        last.offset + last.size / 2.0,
        380.0,
        "last item ends at the edge"
    );
    let small = keylines.get(3).ok_or("fourth keyline")?;
    assert_eq!(small.size, 56.0, "small item at its maximum");
    assert_eq!(small.offset, 380.0 - 28.0, "small item flush right");
    let unadjusted: Vec<f32> = keylines.iter().map(|k| k.unadjusted_offset).collect();
    assert_eq!(
        unadjusted,
        vec![-101.0, 93.0, 287.0, 481.0, 675.0],
        "unadjusted offsets"
    );
    assert_eq!(strategy.item_width(&m), 186.0, "item width");
    Ok(())
}

#[test]
fn multi_browse_keyline_shapes() {
    let strategy = CarouselStrategy::MultiBrowse {
        preferred_item_width: 120.0,
    };
    // testMultiBrowse_doesNotResizeLargeWhenEnoughRoom.
    assert_eq!(
        strategy.item_width(&metrics(500.0, 10, 0.0)),
        120.0,
        "no resize"
    );
    // testMultiBrowse_resizesItemLargerThanContainerToFit1Small.
    let big = CarouselStrategy::MultiBrowse {
        preferred_item_width: 200.0,
    };
    let k = big.keylines(&metrics(100.0, 10, 0.0));
    assert_eq!(k.len(), 4, "[xSmall, large, small, xSmall]");
    assert!(big.item_width(&metrics(100.0, 10, 0.0)) <= 100.0, "fits");
    assert_eq!(
        k.get(2).map(|k| k.size),
        Some(40.0),
        "small item at minimum"
    );
    // testMultiBrowse_hasNoSmallItemsIfNotEnoughRoom.
    assert_eq!(
        big.item_width(&metrics(40.0, 10, 0.0)),
        40.0,
        "only a large item"
    );
    // testMultiBrowse_adjustsMediumSizeToBeProportional.
    let k = big.keylines(&metrics(200.0 * 2.0 + 56.0 * 2.0, 10, 0.0));
    let s = sizes(&k);
    assert_eq!(
        s.len(),
        6,
        "[xSmall, large, large, medium, small, xSmall]: {s:?}"
    );
    assert!(
        matches!(s.as_slice(), [_, l1, l2, m, sm, _] if l1 == l2 && m < l2 && sm < m),
        "medium between large and small: {s:?}"
    );
    // testMultiBrowse_withLessItemsThanKeylines.
    let k = big.keylines(&metrics(200.0 * 2.0 + 56.0 * 2.0, 3, 0.0));
    assert_eq!(k.len(), 5, "a surplus small item is dropped");
    // testMultiBrowse_isNullIfAvailableSpaceIsZero / negative size.
    assert!(
        big.keylines(&metrics(0.0, 10, 0.0)).is_empty(),
        "zero width"
    );
    assert!(
        big.keylines(&metrics(-10.0, 10, 0.0)).is_empty(),
        "negative width"
    );
}

#[test]
fn uncontained_matches_compose() {
    let s = |width: f32, item: f32| {
        CarouselStrategy::Uncontained { item_width: item }.keylines(&metrics(width, 10, 0.0))
    };
    // testLargeItem_withFullCarouselWidth and ..._largerThanFullCarouselWidth.
    for (width, item) in [(500.0, 500.0), (400.0, 500.0)] {
        let k = s(width, item);
        assert_eq!(k.len(), 3, "{width}/{item}: [xSmall, large, xSmall]");
        assert_eq!(k.first().map(|k| k.offset), Some(-5.0), "left anchor");
        assert_eq!(k.get(1).map(|k| k.size), Some(width), "large fills");
        assert_eq!(
            k.get(2).map(|k| k.offset),
            Some(width + 5.0),
            "right anchor"
        );
    }
    // testLargeItem_largerThanHalfCarouselWidth.
    let k = s(1000.0, 501.0);
    assert_eq!(k.len(), 4, "1000/501");
    assert_eq!(
        k.first().map(|k| k.offset),
        Some(-125.25),
        "left anchor offset"
    );
    assert_eq!(
        k.get(3).map(|k| k.offset),
        Some(501.0 * 2.0 + 5.0),
        "right anchor"
    );
    // testRemainingSpaceWithItemSize_fitsItemWithThirdCutoff.
    let k = s(400.0, 125.0);
    assert_eq!(
        sizes(&k),
        vec![18.75, 125.0, 125.0, 125.0, 37.5, 10.0],
        "400/125 sizes"
    );
    assert_eq!(k.get(4).map(|k| k.offset), Some(393.75), "cut-off item");
    assert_eq!(k.first().map(|k| k.offset), Some(-9.375), "left anchor");
    // testRemainingSpaceWithItemSize_fitsMediumItemWithCutoff.
    let k = s(400.0, 105.0);
    let medium = 85.0 * 1.2;
    assert!(
        k.get(4).is_some_and(|k| close(k.size, medium))
            && k.first().is_some_and(|k| close(k.size, medium * 0.5)),
        "400/105: medium at 120 % of the remaining space: {:?}",
        sizes(&k)
    );
}

#[test]
fn hero_matches_compose() {
    let hero = |preferred: Option<f32>| CarouselStrategy::Hero {
        preferred_item_width: preferred,
        centered: true,
    };
    // CenteredHeroTest.enoughRoom_shouldFitOneLargeTwoSmall.
    assert_eq!(
        hero(None).item_width(&metrics(180.0, 6, 0.0)),
        100.0,
        "1 large, 2 small"
    );
    // lessThan3Items_shouldChangeToLeftAlignedArrangement.
    let m = metrics(180.0, 2, 0.0);
    assert!(
        hero(None).item_width(&m) > 100.0,
        "start-aligned with 2 items"
    );
    let k = hero(None).keylines(&m);
    assert_eq!(k.iter().position(|k| k.is_focal), Some(1), "focal first");
    // oneItemOnly_shouldGoFullscreen / lessThanFullscreenThreshold.
    assert_eq!(
        hero(None).item_width(&metrics(120.0, 1, 0.0)),
        120.0,
        "one item"
    );
    assert_eq!(
        hero(None).item_width(&metrics(120.0, 6, 0.0)),
        120.0,
        "too narrow"
    );
    // maxItemWidth_with(out)ItemSpacing_firstAndLastDefaultItemsShouldAlignToStartAndEnd.
    for spacing in [0.0, 12.0] {
        let width = 300.0 * 3.0 + 40.0 * 2.0;
        let k = hero(Some(300.0)).keylines(&metrics(width, 7, spacing));
        let first = k.iter().find(|k| !k.is_anchor).map(|k| k.offset);
        let last = k.iter().rev().find(|k| !k.is_anchor).map(|k| k.offset);
        assert_eq!(
            first,
            Some(20.0),
            "spacing {spacing}: first small flush left"
        );
        assert_eq!(
            last,
            Some(width - 20.0),
            "spacing {spacing}: last small flush right"
        );
    }
}

#[test]
fn full_screen_items_fill_the_viewport() {
    let m = metrics(360.0, 5, 0.0);
    assert_eq!(
        CarouselStrategy::FullScreen.item_width(&m),
        360.0,
        "full width"
    );
    let snaps: Vec<f32> = (0..5)
        .map(|i| CarouselStrategy::FullScreen.snap_offset(&m, i))
        .collect();
    assert_eq!(
        snaps,
        vec![0.0, 360.0, 720.0, 1080.0, 1440.0],
        "one page each"
    );
}

#[test]
fn layout_masks_items_between_keylines() {
    let strategy = CarouselStrategy::MultiBrowse {
        preferred_item_width: 186.0,
    };
    let m = metrics(380.0, 10, 8.0);
    let at_start = strategy.layout(&m, 0.0);
    let first = at_start.first().map(|l| (l.index, l.mask.min, l.mask.max));
    assert_eq!(
        first,
        Some((0, 0.0, 186.0)),
        "first item large at the start"
    );
    for pair in at_start.windows(2) {
        if let [a, b] = pair {
            assert!(
                b.mask.min >= a.mask.max - 0.01,
                "masks do not overlap: {a:?} {b:?}"
            );
            assert!(
                close(a.mask.center(), a.content.center()),
                "content is centred on its mask (parallax)"
            );
        }
    }
    let max = strategy.max_scroll(&m);
    let at_end = strategy.layout(&m, max);
    let last = at_end
        .last()
        .ok_or("an item")
        .map(|l| (l.index, l.mask.max));
    assert!(
        matches!(last, Ok((9, end)) if close(end, 380.0)),
        "last item reaches the end: {last:?}"
    );
    let focal = at_end
        .iter()
        .find(|l| l.index == 9)
        .map(|l| l.mask.max - l.mask.min);
    assert!(
        focal.is_some_and(|w| close(w, 186.0)),
        "scrolled to the end, the last item is large: {focal:?}"
    );
}

// ---------------------------------------------------------------------------
// The widget

fn carousel_harness<'a>(
    offset: &'a Cell<f32>,
    clicked: &'a Cell<Option<usize>>,
    strategy: CarouselStrategy,
) -> Harness<'a> {
    support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(380.0, 200.0),
        move |ui| {
            let mut value = offset.get();
            let mut c = MaterialCarousel::new(&mut value)
                .strategy(strategy)
                .item_spacing(8.0)
                .label("Photos");
            for i in 0..6 {
                c = c.item_text(format!("Photo {i}"));
            }
            let response = c.show(ui);
            if response.clicked.is_some() {
                clicked.set(response.clicked);
            }
            offset.set(value);
        },
    )
}

const MULTI_BROWSE: CarouselStrategy = CarouselStrategy::MultiBrowse {
    preferred_item_width: 186.0,
};

#[test]
fn reports_a_list_of_labelled_items() -> TestResult {
    let offset = Cell::new(0.0);
    let clicked = Cell::new(None);
    let harness = carousel_harness(&offset, &clicked, MULTI_BROWSE);
    assert_eq!(
        harness.get_by_label("Photos").accesskit_node().role(),
        Role::List,
        "carousel is a list"
    );
    assert_eq!(
        harness.get_by_label("Photo 0").accesskit_node().role(),
        Role::ListItem,
        "items are list items"
    );
    Ok(())
}

#[test]
fn arrow_keys_move_focus_and_snap() -> TestResult {
    let offset = Cell::new(0.0);
    let clicked = Cell::new(None);
    let mut harness = carousel_harness(&offset, &clicked, MULTI_BROWSE);
    harness.key_press(Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Photo 0").is_focused(),
        "Tab reaches item 0"
    );
    let m = metrics(harness.get_by_label("Photos").rect().width(), 6, 8.0);
    for i in 1..=3 {
        harness.key_press(Key::ArrowRight);
        harness.run();
        assert!(
            harness.get_by_label(&format!("Photo {i}")).is_focused(),
            "ArrowRight focuses item {i}"
        );
        assert!(
            close(offset.get(), MULTI_BROWSE.snap_offset(&m, i)),
            "item {i}: scrolled to its snap position, got {}",
            offset.get()
        );
    }
    harness.key_press(Key::Home);
    harness.run();
    assert!(close(offset.get(), 0.0), "Home goes back to the start");
    harness.key_press(Key::End);
    harness.run();
    assert!(
        close(offset.get(), MULTI_BROWSE.max_scroll(&m)),
        "End goes to the last item: {} vs {}",
        offset.get(),
        MULTI_BROWSE.max_scroll(&m)
    );
    Ok(())
}

#[test]
fn enter_activates_an_item() -> TestResult {
    let offset = Cell::new(0.0);
    let clicked = Cell::new(None);
    let mut harness = carousel_harness(&offset, &clicked, MULTI_BROWSE);
    harness.get_by_label("Photo 1").focus();
    harness.run();
    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(clicked.get(), Some(1), "Enter clicks the focused item");
    Ok(())
}

#[test]
fn a_drag_snaps_one_item_at_most() -> TestResult {
    let offset = Cell::new(0.0);
    let clicked = Cell::new(None);
    let mut harness = carousel_harness(&offset, &clicked, MULTI_BROWSE);
    let rect = harness.get_by_label("Photos").rect();
    let start = rect.center();
    harness.drag_at(start);
    harness.run();
    for step in 1..=10 {
        harness.hover_at(start - egui::vec2(step as f32 * 40.0, 0.0));
        harness.step();
    }
    harness.drop_at(start - egui::vec2(400.0, 0.0));
    harness.run();
    let m = metrics(harness.get_by_label("Photos").rect().width(), 6, 8.0);
    assert!(
        close(offset.get(), MULTI_BROWSE.snap_offset(&m, 1)),
        "a long drag advances one item: {}",
        offset.get()
    );
    Ok(())
}

#[test]
fn uncontained_does_not_snap() -> TestResult {
    let offset = Cell::new(0.0);
    let clicked = Cell::new(None);
    let uncontained = CarouselStrategy::Uncontained { item_width: 120.0 };
    let mut harness = carousel_harness(&offset, &clicked, uncontained);
    let start = harness.get_by_label("Photos").rect().center();
    harness.drag_at(start);
    harness.run();
    harness.hover_at(start - egui::vec2(30.0, 0.0));
    harness.step();
    harness.drop_at(start - egui::vec2(30.0, 0.0));
    harness.run();
    assert!(
        offset.get() > 20.0 && offset.get() < 40.0,
        "stays where the drag left it: {}",
        offset.get()
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshots

fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Runs a render, turning a panic into an error: with no GPU adapter at
/// all, egui_kittest's renderer panics instead of returning one.
fn no_adapter_is_an_error<T>(
    render: impl FnOnce() -> Result<T, egui_kittest::SnapshotError>,
) -> Result<T, egui_kittest::SnapshotError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(render)).unwrap_or_else(|_| {
        Err(egui_kittest::SnapshotError::RenderError {
            err: "no adapter: the renderer could not be created".to_owned(),
        })
    })
}

fn snapshot(harness: &mut Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(600);
    match no_adapter_is_an_error(|| harness.try_snapshot_options(name, &options)) {
        Ok(()) => Ok(()),
        Err(egui_kittest::SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

#[test]
fn strategies_snapshot() -> TestResult {
    for (name, theme) in support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
    {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(380.0, 520.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            let strategies = [
                MULTI_BROWSE,
                CarouselStrategy::Uncontained { item_width: 150.0 },
                CarouselStrategy::Hero {
                    preferred_item_width: None,
                    centered: true,
                },
                CarouselStrategy::FullScreen,
            ];
            for (i, strategy) in strategies.into_iter().enumerate() {
                let mut offset = 0.0;
                let mut c = MaterialCarousel::new(&mut offset)
                    .strategy(strategy)
                    .item_spacing(8.0)
                    .height(120.0)
                    .id_salt(format!("c{i}"));
                for item in 0..6 {
                    c = c.item_text(format!("{item}"));
                }
                ui.add(c);
                ui.add_space(8.0);
            }
        });
        harness.run();
        snapshot(&mut harness, &format!("carousel_strategies_{name}"))?;
    }
    Ok(())
}
