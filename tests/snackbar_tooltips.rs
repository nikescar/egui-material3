// ============================================================================
// Module:       snackbar_tooltips (integration test)
// Description:  Snackbar, snackbar host, plain and rich tooltips, notification.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Snackbars, the snackbar host, plain and rich tooltips, and notification
//! auto-dismiss (WP-13): timing under simulated time, write-back of the
//! visibility flag, token geometry and colours, AccessKit roles, hover
//! delay, persistence, and snapshots in light and dark.

mod support;

use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use egui::accesskit::{Live, Role};
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape, TextShape};
use egui::{Color32, Event, Modifiers, PointerButton, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::tokens::{ColorRole, LayoutDirection};
use egui_material3::{
    notification, MaterialSnackbar, MaterialTheme, MaterialTooltip, RichTooltip, SnackbarDuration,
    SnackbarHost, SnackbarResult, SnackbarVisuals, TooltipPosition,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ---------------------------------------------------------------------------
// A raw-context driver: exact control of time and pointer, and access to the
// painted shapes.

const SCREEN: egui::Vec2 = egui::vec2(1200.0, 800.0);

struct Driver {
    ctx: egui::Context,
    time: f64,
}

impl Driver {
    fn new() -> Self {
        Self::with_theme(MaterialTheme::baseline(false))
    }

    fn with_theme(theme: MaterialTheme) -> Self {
        let ctx = egui::Context::default();
        MaterialTheme::set(&ctx, theme);
        Self { ctx, time: 0.0 }
    }

    /// Runs one frame `dt` seconds after the last, with `events`.
    fn frame(
        &mut self,
        dt: f64,
        events: Vec<Event>,
        app: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<ClippedShape> {
        self.time += dt;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let mut output = self.ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        output.shapes
    }

    /// Runs frames `dt` apart until `seconds` have passed, returning the
    /// last frame's shapes.
    fn run_for(
        &mut self,
        seconds: f64,
        dt: f64,
        app: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<ClippedShape> {
        let mut shapes = Vec::new();
        let mut elapsed = 0.0;
        while elapsed < seconds {
            shapes = self.frame(dt, Vec::new(), app);
            elapsed += dt;
        }
        shapes
    }
}

fn press(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::default(),
    }
}

/// Clicks at `pos`: move, press, release, one frame each.
fn click(driver: &mut Driver, pos: Pos2, app: &mut dyn FnMut(&mut egui::Ui)) {
    driver.frame(0.05, vec![Event::PointerMoved(pos)], app);
    driver.frame(0.05, vec![press(pos, true)], app);
    driver.frame(0.05, vec![press(pos, false)], app);
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

fn rects(shapes: &[ClippedShape]) -> Vec<RectShape> {
    fn collect(shape: &PaintShape, out: &mut Vec<RectShape>) {
        match shape {
            PaintShape::Rect(r) => out.push(r.clone()),
            PaintShape::Vec(v) => v.iter().for_each(|s| collect(s, out)),
            PaintShape::Noop
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Band(_)
            | PaintShape::Text(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => {}
        }
    }
    let mut out = Vec::new();
    shapes.iter().for_each(|c| collect(&c.shape, &mut out));
    out
}

fn texts(shapes: &[ClippedShape]) -> Vec<TextShape> {
    shapes
        .iter()
        .filter_map(|c| match &c.shape {
            PaintShape::Text(t) => Some(t.clone()),
            PaintShape::Noop
            | PaintShape::Vec(_)
            | PaintShape::Circle(_)
            | PaintShape::Ellipse(_)
            | PaintShape::LineSegment { .. }
            | PaintShape::Path(_)
            | PaintShape::Band(_)
            | PaintShape::Rect(_)
            | PaintShape::Mesh(_)
            | PaintShape::QuadraticBezier(_)
            | PaintShape::CubicBezier(_)
            | PaintShape::Callback(_) => None,
        })
        .collect()
}

/// The painted text shape whose text is `text`.
fn text_shape(shapes: &[ClippedShape], text: &str) -> Option<TextShape> {
    texts(shapes).into_iter().find(|t| t.galley.text() == text)
}

/// The rect of the painted text `text`.
fn text_rect(shapes: &[ClippedShape], text: &str) -> Option<Rect> {
    text_shape(shapes, text).map(|t| Rect::from_min_size(t.pos, t.galley.size()))
}

/// The first rect filled with `color`.
fn filled(shapes: &[ClippedShape], color: Color32) -> Option<RectShape> {
    rects(shapes).into_iter().find(|r| r.fill == color)
}

fn color(role: ColorRole) -> Color32 {
    MaterialTheme::baseline(false).color(role)
}

// ---------------------------------------------------------------------------
// Snackbar widget

#[test]
fn snackbar_disappears_after_four_seconds_of_simulated_time() -> TestResult {
    let mut driver = Driver::new();
    let mut visible = true;
    let mut changed_at = None;
    let mut app = |ui: &mut egui::Ui| {
        let r = ui.add(MaterialSnackbar::new("Saved").show_if(&mut visible));
        if r.changed() {
            changed_at = Some(ui.input(|i| i.time));
        }
    };
    driver.frame(0.0, Vec::new(), &mut app);
    let shapes = driver.run_for(3.9, 0.1, &mut app);
    assert!(
        text_shape(&shapes, "Saved").is_some(),
        "the snackbar is still up just before 4 s"
    );
    let shapes = driver.run_for(0.2, 0.1, &mut app);
    assert!(
        text_shape(&shapes, "Saved").is_none(),
        "the snackbar is gone after 4 s"
    );
    drop(app);
    assert!(!visible, "the timeout writes false back to the bound flag");
    let at = changed_at.ok_or("the response reports the dismissal as a change")?;
    assert!(
        (4.0..4.2).contains(&at),
        "dismissed at {at}, expected just after 4 s"
    );
    Ok(())
}

#[test]
fn long_and_indefinite_durations() -> TestResult {
    for (duration, up_at, gone_at) in [
        (SnackbarDuration::Long, 9.8, Some(10.2)),
        (SnackbarDuration::Indefinite, 60.0, None),
    ] {
        let mut driver = Driver::new();
        let mut visible = true;
        let mut app = |ui: &mut egui::Ui| {
            ui.add(
                MaterialSnackbar::new("Uploading")
                    .duration(duration)
                    .show_if(&mut visible),
            );
        };
        driver.frame(0.0, Vec::new(), &mut app);
        let shapes = driver.run_for(up_at, 0.2, &mut app);
        assert!(
            text_shape(&shapes, "Uploading").is_some(),
            "{duration:?}: still up at {up_at} s"
        );
        if let Some(gone_at) = gone_at {
            let shapes = driver.run_for(gone_at - up_at, 0.2, &mut app);
            assert!(
                text_shape(&shapes, "Uploading").is_none(),
                "{duration:?}: gone at {gone_at} s"
            );
        }
    }
    Ok(())
}

#[test]
fn action_click_hides_the_snackbar_and_calls_back() -> TestResult {
    let mut driver = Driver::new();
    let mut visible = true;
    let clicks = AtomicUsize::new(0);
    let mut app = |ui: &mut egui::Ui| {
        ui.add(
            MaterialSnackbar::new("Message archived")
                .action("Undo", || {
                    clicks.fetch_add(1, Ordering::Relaxed);
                })
                .show_if(&mut visible),
        );
    };
    driver.frame(0.0, Vec::new(), &mut app);
    let shapes = driver.run_for(0.5, 0.1, &mut app);
    let undo = text_rect(&shapes, "Undo").ok_or("the action label is painted")?;
    click(&mut driver, undo.center(), &mut app);
    let shapes = driver.frame(0.05, Vec::new(), &mut app);
    assert!(
        text_shape(&shapes, "Message archived").is_none(),
        "clicking the action hides the snackbar"
    );
    drop(app);
    assert!(!visible, "the click writes false back to the bound flag");
    assert_eq!(
        clicks.load(Ordering::Relaxed),
        1,
        "the action callback runs once"
    );
    Ok(())
}

#[test]
fn dismiss_icon_hides_the_snackbar() -> TestResult {
    let visible = Cell::new(true);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(800.0, 400.0),
        |ui| {
            let mut v = visible.get();
            ui.add(
                MaterialSnackbar::new("Draft discarded")
                    .show_close_icon(true)
                    .auto_dismiss(None)
                    .show_if(&mut v),
            );
            visible.set(v);
        },
    );
    harness.get_by_label("Dismiss").click();
    harness.run();
    assert!(!visible.get(), "the dismiss icon writes false back");
    assert!(
        harness.query_by_label("Draft discarded").is_none(),
        "the snackbar is gone from the AccessKit tree"
    );
    Ok(())
}

#[test]
fn keyboard_reaches_and_activates_the_action() -> TestResult {
    let visible = Cell::new(true);
    let clicks = AtomicUsize::new(0);
    let mut harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(800.0, 400.0),
        |ui| {
            let mut v = visible.get();
            ui.add(
                MaterialSnackbar::new("Message archived")
                    .action("Undo", || {
                        clicks.fetch_add(1, Ordering::Relaxed);
                    })
                    .auto_dismiss(None)
                    .show_if(&mut v),
            );
            visible.set(v);
        },
    );
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Undo").accesskit_node().is_focused(),
        "Tab focuses the action"
    );
    harness.key_press(egui::Key::Enter);
    harness.run();
    drop(harness);
    assert_eq!(
        clicks.load(Ordering::Relaxed),
        1,
        "Enter activates the action"
    );
    assert!(!visible.get(), "and dismisses the snackbar");
    Ok(())
}

#[test]
fn a_hidden_snackbar_shown_again_restarts_its_timer() -> TestResult {
    let mut driver = Driver::new();
    let mut visible = true;
    let mut shown = true;
    let frame = |driver: &mut Driver, dt: f64, add: bool, visible: &mut bool| {
        driver.frame(dt, Vec::new(), &mut |ui| {
            if add {
                ui.add(MaterialSnackbar::new("Copied").show_if(&mut *visible));
            }
        })
    };
    frame(&mut driver, 0.0, shown, &mut visible);
    for _ in 0..45 {
        frame(&mut driver, 0.1, shown, &mut visible);
    }
    assert!(!visible, "timed out after 4 s");
    // The caller stops adding it, then shows it again later.
    shown = false;
    frame(&mut driver, 1.0, shown, &mut visible);
    shown = true;
    visible = true;
    let shapes = frame(&mut driver, 0.1, shown, &mut visible);
    let shapes = if text_shape(&shapes, "Copied").is_some() {
        shapes
    } else {
        frame(&mut driver, 0.1, shown, &mut visible)
    };
    assert!(
        text_shape(&shapes, "Copied").is_some() && visible,
        "showing it again starts a new 4 s"
    );
    Ok(())
}

#[test]
fn snackbar_geometry_follows_the_tokens() -> TestResult {
    let container = color(ColorRole::InverseSurface);
    let measure = |message: &'static str, action: Option<&'static str>| {
        let mut driver = Driver::new();
        let mut app = |ui: &mut egui::Ui| {
            let mut s = MaterialSnackbar::new(message).auto_dismiss(None);
            if let Some(action) = action {
                s = s.action(action, || {});
            }
            ui.add(s);
        };
        driver.frame(0.0, Vec::new(), &mut app);
        let shapes = driver.run_for(1.0, 0.1, &mut app);
        filled(&shapes, container).map(|r| (r.rect, r.corner_radius, shapes))
    };

    let (one, radius, shapes) = measure("Saved", None).ok_or("one-line container")?;
    assert_eq!(one.height(), 48.0, "SnackbarTokens.ContainerHeight is 48");
    assert_eq!(
        one.width(),
        600.0,
        "ContainerMaxWidth caps the width at 600"
    );
    assert_eq!(
        one.max.y,
        SCREEN.y - 12.0,
        "the host padding keeps it 12 dp above the bottom"
    );
    assert_eq!(
        one.center().x,
        SCREEN.x / 2.0,
        "a floating snackbar is centred"
    );
    assert_eq!(
        radius.nw, 4,
        "SnackbarTokens.ContainerShape is CornerExtraSmall"
    );
    let text = text_rect(&shapes, "Saved").ok_or("message painted")?;
    assert_eq!(
        text.min.x - one.min.x,
        16.0,
        "Snackbar.kt HorizontalSpacing is 16"
    );
    let text_color = text_shape(&shapes, "Saved")
        .and_then(|t| t.galley.job.sections.first().map(|s| s.format.color))
        .ok_or("message format")?;
    assert_eq!(
        text_color,
        color(ColorRole::InverseOnSurface),
        "supporting text is InverseOnSurface"
    );

    let (two, _, _) =
        measure("First line\nSecond line", Some("Retry")).ok_or("two-line container")?;
    assert_eq!(
        two.height(),
        68.0,
        "SnackbarTokens.LargeContainerHeight is 68 for two lines"
    );

    let (_, _, shapes) = measure("Deleted", Some("Undo")).ok_or("action container")?;
    let action = text_shape(&shapes, "Undo").ok_or("action painted")?;
    let action_color = action
        .galley
        .job
        .sections
        .first()
        .map(|s| s.format.color)
        .ok_or("action format")?;
    assert_eq!(
        action_color,
        color(ColorRole::InversePrimary),
        "the action label is InversePrimary"
    );
    Ok(())
}

#[test]
fn hovering_the_action_paints_an_inverse_primary_state_layer() -> TestResult {
    let mut driver = Driver::new();
    let mut app = |ui: &mut egui::Ui| {
        ui.add(
            MaterialSnackbar::new("Deleted")
                .action("Undo", || {})
                .auto_dismiss(None),
        );
    };
    driver.frame(0.0, Vec::new(), &mut app);
    let shapes = driver.run_for(1.0, 0.1, &mut app);
    let undo = text_rect(&shapes, "Undo").ok_or("action painted")?;
    let shapes = driver.frame(0.1, vec![Event::PointerMoved(undo.center())], &mut app);
    let layer = color(ColorRole::InversePrimary).gamma_multiply(0.08);
    let found = rects(&shapes)
        .into_iter()
        .any(|r| r.fill == layer && r.rect.contains(undo.center()) && r.rect.height() == 40.0);
    assert!(
        found,
        "hover paints InversePrimary at 8% over the 40 dp action"
    );
    Ok(())
}

#[test]
fn right_to_left_puts_the_action_at_the_start() -> TestResult {
    let mut theme = MaterialTheme::baseline(false);
    theme.layout_direction = LayoutDirection::Rtl;
    let mut driver = Driver::with_theme(theme);
    let mut app = |ui: &mut egui::Ui| {
        ui.add(
            MaterialSnackbar::new("Deleted")
                .action("Undo", || {})
                .auto_dismiss(None),
        );
    };
    driver.frame(0.0, Vec::new(), &mut app);
    let shapes = driver.run_for(1.0, 0.1, &mut app);
    let undo = text_rect(&shapes, "Undo").ok_or("action painted")?;
    let message = text_rect(&shapes, "Deleted").ok_or("message painted")?;
    assert!(
        undo.max.x < message.min.x,
        "in RTL the action is left of the message"
    );
    Ok(())
}

#[test]
fn snackbar_is_a_polite_live_status() -> TestResult {
    let harness = support::harness_for(
        MaterialTheme::baseline(false),
        egui::vec2(800.0, 400.0),
        |ui| {
            ui.add(MaterialSnackbar::new("Connection restored").auto_dismiss(None));
        },
    );
    let node = harness.get_by_label("Connection restored");
    assert_eq!(
        node.accesskit_node().role(),
        Role::Status,
        "the snackbar is a status node"
    );
    assert_eq!(
        node.accesskit_node().live(),
        Live::Polite,
        "the snackbar is a polite live region"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Snackbar host

#[test]
fn host_shows_one_at_a_time_and_reports_timeouts() -> TestResult {
    let mut driver = Driver::new();
    let host = SnackbarHost::with_id("host_test_queue");
    let first = host.enqueue(&driver.ctx, SnackbarVisuals::new("First"));
    let second = host.enqueue(&driver.ctx, SnackbarVisuals::new("Second"));
    let mut outcomes = Vec::new();
    let mut app = |ui: &mut egui::Ui| {
        if let Some(outcome) = host.show(ui.ctx()) {
            outcomes.push((outcome, ui.input(|i| i.time)));
        }
    };
    let shapes = driver.run_for(1.0, 0.1, &mut app);
    assert!(
        text_shape(&shapes, "First").is_some() && text_shape(&shapes, "Second").is_none(),
        "only the first snackbar shows"
    );
    assert_eq!(host.pending(&driver.ctx), 1, "the second one waits");
    let shapes = driver.run_for(4.0, 0.1, &mut app);
    assert!(
        text_shape(&shapes, "Second").is_some() && text_shape(&shapes, "First").is_none(),
        "the second follows once the first times out"
    );
    drop(app);
    let (outcome, at) = outcomes.first().copied().ok_or("the first one ended")?;
    assert_eq!(outcome.id, first, "the outcome names the first snackbar");
    assert_eq!(
        outcome.result,
        SnackbarResult::Dismissed,
        "a timeout is a dismissal"
    );
    assert!((4.0..4.2).contains(&at), "it ended at {at}, after 4 s");
    assert!(
        outcomes.iter().all(|(o, _)| o.id != second),
        "the second has not ended yet"
    );
    Ok(())
}

#[test]
fn host_action_click_reports_action_performed() -> TestResult {
    let mut driver = Driver::new();
    let host = SnackbarHost::with_id("host_test_action");
    host.enqueue(
        &driver.ctx,
        SnackbarVisuals::new("Archived")
            .action("Undo")
            .duration(SnackbarDuration::Indefinite),
    );
    let mut outcome = None;
    let mut app = |ui: &mut egui::Ui| {
        if let Some(o) = host.show(ui.ctx()) {
            outcome = Some(o);
        }
    };
    let shapes = driver.run_for(30.0, 0.5, &mut app);
    let undo = text_rect(&shapes, "Undo").ok_or("an indefinite snackbar is still up at 30 s")?;
    click(&mut driver, undo.center(), &mut app);
    let shapes = driver.run_for(1.0, 0.05, &mut app);
    drop(app);
    assert_eq!(
        outcome.map(|o| o.result),
        Some(SnackbarResult::ActionPerformed),
        "the action click is reported"
    );
    assert!(
        text_shape(&shapes, "Archived").is_none(),
        "it has faded out"
    );
    assert!(host.current(&driver.ctx).is_none(), "nothing is showing");
    Ok(())
}

#[test]
fn host_dismiss_current_and_dismiss_icon() -> TestResult {
    let mut driver = Driver::new();
    let host = SnackbarHost::with_id("host_test_dismiss");
    host.enqueue(&driver.ctx, SnackbarVisuals::new("One"));
    host.enqueue(
        &driver.ctx,
        SnackbarVisuals::new("Two").with_dismiss_action(true),
    );
    let mut results = Vec::new();
    let mut app = |ui: &mut egui::Ui| {
        if let Some(o) = host.show(ui.ctx()) {
            results.push(o.result);
        }
    };
    driver.run_for(0.5, 0.1, &mut app);
    host.dismiss_current(&driver.ctx);
    let shapes = driver.run_for(1.0, 0.1, &mut app);
    assert!(
        text_shape(&shapes, "Two").is_some(),
        "dismissing the first shows the second"
    );
    let two = filled(&shapes, color(ColorRole::InverseSurface)).ok_or("second container")?;
    // The dismiss icon sits in the 48 dp slot at the end.
    let icon = egui::pos2(two.rect.max.x - 24.0, two.rect.center().y);
    click(&mut driver, icon, &mut app);
    drop(app);
    assert_eq!(
        results,
        vec![SnackbarResult::Dismissed, SnackbarResult::Dismissed],
        "both end as dismissed"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Tooltips

/// The time from hovering a button until its tooltip's text is painted.
fn tooltip_appears_after(
    tooltip: &dyn Fn(&egui::Response),
    text: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut driver = Driver::new();
    let button = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        ui.add_space(200.0);
        let response = ui.button("Anchor");
        button.set(response.rect);
        tooltip(&response);
    };
    driver.run_for(1.0, 0.1, &mut app);
    driver.frame(
        0.0,
        vec![Event::PointerMoved(button.get().center())],
        &mut app,
    );
    let hovered_at = driver.time;
    while driver.time - hovered_at < 2.0 {
        let shapes = driver.frame(0.05, Vec::new(), &mut app);
        if text_shape(&shapes, text).is_some() {
            return Ok(driver.time - hovered_at);
        }
    }
    Err(format!("the tooltip {text:?} never appeared").into())
}

#[test]
fn plain_tooltip_appears_after_the_hover_delay_and_not_before() -> TestResult {
    let after = tooltip_appears_after(
        &|r| {
            MaterialTooltip::new("Save").show_for(r);
        },
        "Save",
    )?;
    // egui's default tooltip_delay is 0.5 s.
    assert!(after >= 0.5, "appeared after {after} s, before the delay");
    assert!(
        after < 0.8,
        "appeared after {after} s, long after the delay"
    );
    Ok(())
}

#[test]
fn rich_tooltip_appears_after_the_hover_delay_and_not_before() -> TestResult {
    let after = tooltip_appears_after(
        &|r| {
            RichTooltip::new("Old backups use 20 GB.")
                .subhead("Storage")
                .show_for(r);
        },
        "Old backups use 20 GB.",
    )?;
    assert!(after >= 0.5, "appeared after {after} s, before the delay");
    assert!(
        after < 0.8,
        "appeared after {after} s, long after the delay"
    );
    Ok(())
}

/// Shows an always-open tooltip under a fixed anchor and returns the shapes
/// once it has settled, with the anchor rect.
fn always_open(show: &dyn Fn(&mut egui::Ui, Rect)) -> (Vec<ClippedShape>, Rect) {
    let anchor = Rect::from_min_size(egui::pos2(500.0, 300.0), egui::vec2(100.0, 40.0));
    let mut driver = Driver::new();
    let mut app = |ui: &mut egui::Ui| show(ui, anchor);
    (driver.run_for(0.5, 0.1, &mut app), anchor)
}

#[test]
fn plain_tooltip_geometry_follows_the_tokens() -> TestResult {
    let (shapes, anchor) = always_open(&|ui, anchor| {
        MaterialTooltip::new("Save")
            .position(TooltipPosition::Bottom)
            .show(ui, anchor);
    });
    let container = filled(&shapes, color(ColorRole::InverseSurface)).ok_or("container painted")?;
    let text = text_rect(&shapes, "Save").ok_or("text painted")?;
    assert_eq!(
        container.rect.min.y - anchor.max.y,
        4.0,
        "PlainTooltipTokens.GapTrigger is 4"
    );
    assert_eq!(
        container.rect.height(),
        24.0,
        "one BodySmall line (16) plus 4 + 4 padding is the 24 dp minimum"
    );
    assert_eq!(
        text.min.x - container.rect.min.x,
        8.0,
        "8 dp horizontal padding"
    );
    assert_eq!(
        text.min.y - container.rect.min.y,
        4.0,
        "4 dp vertical padding"
    );
    assert_eq!(container.corner_radius.nw, 4, "CornerExtraSmall");
    assert_eq!(
        container.stroke.width, 0.0,
        "a plain tooltip has no outline"
    );

    let long = "A much longer tooltip that has to wrap onto more than one line because it is far wider than the maximum width allows";
    let (shapes, _) = always_open(&|ui, anchor| {
        MaterialTooltip::new(long).show(ui, anchor);
    });
    let container =
        filled(&shapes, color(ColorRole::InverseSurface)).ok_or("long container painted")?;
    assert!(
        container.rect.width() <= 320.0 && container.rect.width() > 300.0,
        "the text wraps at PlainTooltipTokens.MaxWidth 320, got {}",
        container.rect.width()
    );
    Ok(())
}

#[test]
fn rich_tooltip_geometry_follows_the_tokens() -> TestResult {
    let (shapes, _) = always_open(&|ui, anchor| {
        RichTooltip::new("Old backups use 20 GB.")
            .subhead("Storage")
            .action("Manage")
            .show(ui, anchor);
    });
    let container =
        filled(&shapes, color(ColorRole::SurfaceContainer)).ok_or("container painted")?;
    assert_eq!(container.corner_radius.nw, 12, "CornerMedium");
    let text = text_rect(&shapes, "Old backups use 20 GB.").ok_or("text painted")?;
    assert_eq!(
        text.min.x - container.rect.min.x,
        16.0,
        "RichTooltipHorizontalPadding is 16"
    );
    let subhead = text_shape(&shapes, "Storage").ok_or("subhead painted")?;
    let subhead_color = subhead
        .galley
        .job
        .sections
        .first()
        .map(|s| s.format.color)
        .ok_or("subhead format")?;
    assert_eq!(
        subhead_color,
        color(ColorRole::OnSurfaceVariant),
        "the subhead is OnSurfaceVariant"
    );
    let action = text_rect(&shapes, "Manage").ok_or("action painted")?;
    assert_eq!(
        container.rect.max.y - action.center().y,
        8.0 + 18.0,
        "a 36 dp action row with 8 dp below"
    );
    let shadowed = rects(&shapes)
        .iter()
        .any(|r| r.blur_width > 0.0 && r.rect.contains(container.rect.center()));
    assert!(shadowed, "the Level2 shadow is painted under the container");
    Ok(())
}

/// Hovers an anchor until its rich tooltip shows, then moves the pointer
/// onto the tooltip and reports whether it is still showing.
fn stays_open_on_tooltip(persistent: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let mut driver = Driver::new();
    let button = Cell::new(Rect::NOTHING);
    let mut app = |ui: &mut egui::Ui| {
        ui.add_space(200.0);
        let response = ui.button("Anchor");
        button.set(response.rect);
        RichTooltip::new("Details about the anchor.")
            .position(TooltipPosition::Bottom)
            .persistent(persistent)
            .show_for(&response);
    };
    driver.run_for(0.5, 0.1, &mut app);
    driver.frame(
        0.0,
        vec![Event::PointerMoved(button.get().center())],
        &mut app,
    );
    let shapes = driver.run_for(1.0, 0.1, &mut app);
    let text = text_rect(&shapes, "Details about the anchor.").ok_or("tooltip shown")?;
    driver.frame(0.05, vec![Event::PointerMoved(text.center())], &mut app);
    let shapes = driver.run_for(0.5, 0.1, &mut app);
    Ok(text_shape(&shapes, "Details about the anchor.").is_some())
}

#[test]
fn persistent_rich_tooltip_stays_open_under_the_pointer() -> TestResult {
    assert!(
        stays_open_on_tooltip(true)?,
        "a persistent rich tooltip stays open while hovered"
    );
    assert!(
        !stays_open_on_tooltip(false)?,
        "a non-persistent one closes when the pointer leaves the anchor"
    );
    Ok(())
}

#[test]
fn rich_tooltip_action_can_be_clicked() -> TestResult {
    let mut driver = Driver::new();
    let button = Cell::new(Rect::NOTHING);
    let mut clicked = None;
    let mut app = |ui: &mut egui::Ui| {
        ui.add_space(200.0);
        let response = ui.button("Anchor");
        button.set(response.rect);
        let tip = RichTooltip::new("Storage almost full.")
            .action("Manage")
            .position(TooltipPosition::Bottom)
            .show_for(&response);
        if tip.action_clicked.is_some() {
            clicked = tip.action_clicked;
        }
    };
    driver.run_for(0.5, 0.1, &mut app);
    driver.frame(
        0.0,
        vec![Event::PointerMoved(button.get().center())],
        &mut app,
    );
    let shapes = driver.run_for(1.0, 0.1, &mut app);
    let manage = text_rect(&shapes, "Manage").ok_or("action shown")?;
    driver.frame(0.05, vec![Event::PointerMoved(manage.center())], &mut app);
    driver.run_for(0.2, 0.1, &mut app);
    driver.frame(0.05, vec![press(manage.center(), true)], &mut app);
    driver.frame(0.05, vec![press(manage.center(), false)], &mut app);
    drop(app);
    assert_eq!(clicked, Some(0), "the action click is reported by index");
    Ok(())
}

#[test]
fn with_tooltip_shows_for_its_contents() -> TestResult {
    let after = {
        let mut driver = Driver::new();
        let inner = Cell::new(Rect::NOTHING);
        let mut app = |ui: &mut egui::Ui| {
            ui.add_space(200.0);
            egui_material3::with_tooltip(ui, "Grouped", TooltipPosition::Top, |ui| {
                inner.set(ui.button("Inside").rect);
            });
        };
        driver.run_for(0.5, 0.1, &mut app);
        driver.frame(
            0.0,
            vec![Event::PointerMoved(inner.get().center())],
            &mut app,
        );
        let shapes = driver.run_for(1.0, 0.1, &mut app);
        text_shape(&shapes, "Grouped").is_some()
    };
    assert!(after, "with_tooltip shows its tooltip over its contents");
    Ok(())
}

// ---------------------------------------------------------------------------
// Notification

#[test]
fn notification_auto_dismiss_hides_it_and_reports_once() -> TestResult {
    let mut driver = Driver::new();
    let mut changes = Vec::new();
    let mut app = |ui: &mut egui::Ui| {
        let r = ui.add(
            notification()
                .title("Sync complete")
                .auto_dismiss(Duration::from_secs(3)),
        );
        if r.changed() {
            changes.push(ui.input(|i| i.time));
        }
    };
    driver.frame(0.0, Vec::new(), &mut app);
    let shapes = driver.run_for(2.9, 0.1, &mut app);
    assert!(
        text_shape(&shapes, "Sync complete").is_some(),
        "still up before 3 s"
    );
    let shapes = driver.run_for(1.0, 0.1, &mut app);
    assert!(
        text_shape(&shapes, "Sync complete").is_none(),
        "gone after 3 s"
    );
    drop(app);
    assert_eq!(changes.len(), 1, "the timeout is reported once");
    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshots

/// Rendering tests take this lock: several wgpu renderers created at once
/// in one process can crash the test binary on Windows.
fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn snapshot(harness: &mut egui_kittest::Harness<'_>, name: &str) -> TestResult {
    let _renderer = one_renderer_at_a_time();
    // Software and hardware rasterisers differ in anti-aliasing; allow that,
    // not a missing or moved surface.
    let options = egui_kittest::SnapshotOptions::new()
        .threshold(2.0)
        .max_failed_pixels(600);
    match no_adapter_is_an_error(|| harness.try_snapshot_options(name, &options)) {
        Ok(()) => Ok(()),
        // No GPU adapter (headless CI without a software renderer): the
        // geometry is still covered by the tests above.
        Err(egui_kittest::SnapshotError::RenderError { err }) => {
            eprintln!("skipping snapshot, no renderer: {err}");
            Ok(())
        }
        Err(err) => Err(err.to_string().into()),
    }
}

/// The light and dark themes of the default theme file.
fn light_and_dark() -> Vec<(&'static str, MaterialTheme)> {
    support::themes()
        .into_iter()
        .filter(|(name, _)| *name == "light" || *name == "dark")
        .collect()
}

#[test]
fn snackbar_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(640.0, 220.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            ui.add(
                MaterialSnackbar::new("Photo saved to the gallery")
                    .auto_dismiss(None)
                    .position(egui_material3::SnackbarPosition::Top),
            );
            ui.add(
                MaterialSnackbar::new("Couldn't send the message.\nCheck the connection.")
                    .action("Retry", || {})
                    .show_close_icon(true)
                    .auto_dismiss(None),
            );
        });
        snapshot(&mut harness, &format!("snackbar_{name}"))?;
    }
    Ok(())
}

#[test]
fn tooltips_snapshot() -> TestResult {
    for (name, theme) in light_and_dark() {
        let background = theme.color(ColorRole::Surface);
        let mut harness = support::harness_for(theme, egui::vec2(640.0, 260.0), move |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, background);
            let plain = Rect::from_min_size(egui::pos2(40.0, 20.0), egui::vec2(80.0, 32.0));
            ui.painter().rect_filled(plain, 4.0, Color32::GRAY);
            MaterialTooltip::new("Save")
                .position(TooltipPosition::Bottom)
                .show(ui, plain);
            let rich = Rect::from_min_size(egui::pos2(300.0, 20.0), egui::vec2(80.0, 32.0));
            ui.painter().rect_filled(rich, 4.0, Color32::GRAY);
            RichTooltip::new("Free up space by deleting old backups.")
                .subhead("Storage almost full")
                .action("Manage")
                .position(TooltipPosition::Bottom)
                .show(ui, rich);
        });
        snapshot(&mut harness, &format!("tooltips_{name}"))?;
    }
    Ok(())
}
