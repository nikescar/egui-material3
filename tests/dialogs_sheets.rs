// ============================================================================
// Module:       dialogs_sheets (integration test)
// Description:  Dialogs, full-screen dialogs, bottom and side sheets, the
//               action sheet and the scrim.
//
// Dependencies: egui, egui_kittest, egui_material3
// ============================================================================

//! Dialogs, sheets and the scrim (WP-12): token geometry, the 32 % scrim
//! (shape and pixel sample), enter/exit motion, close reasons, Escape
//! closing only the top layer, the Tab focus trap and focus restore, drag
//! to dismiss, and the AccessKit roles.

mod support;

use std::cell::Cell;

use egui::accesskit::Role;
use egui::epaint::{ClippedShape, RectShape, Shape as PaintShape};
use egui::{Color32, Event, Key, Modifiers, PointerButton, Pos2, Rect, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::bottom_sheet::{
    DRAG_HANDLE_HEIGHT, DRAG_HANDLE_WIDTH, SHEET_MAX_WIDTH, SHEET_PEEK_HEIGHT,
};
use egui_material3::dialog::{DialogType, FULL_SCREEN_HEADER_HEIGHT};
use egui_material3::scrim::SCRIM_OPACITY;
use egui_material3::side_sheet::SIDE_SHEET_WIDTH;
use egui_material3::tokens::{ColorRole, LayoutDirection};
use egui_material3::{
    BottomSheet, BottomSheetKind, DialogCloseReason, FullScreenDialog, MaterialActionSheet,
    MaterialDialog, MaterialTheme, SheetCloseReason, SideSheet,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ---------------------------------------------------------------------------
// A raw-context driver: exact control of time and input, and access to the
// painted shapes.

/// A context with its own clock.
pub struct Driver {
    pub ctx: egui::Context,
    pub time: f64,
    pub screen: Rect,
}

impl Driver {
    /// A context of `size` points with the baseline light theme.
    pub fn new(size: egui::Vec2) -> Self {
        let ctx = egui::Context::default();
        MaterialTheme::set(&ctx, MaterialTheme::baseline(false));
        Self {
            ctx,
            time: 0.0,
            screen: Rect::from_min_size(Pos2::ZERO, size),
        }
    }

    /// Runs one frame `dt` seconds after the last, with `events`.
    pub fn frame(
        &mut self,
        dt: f64,
        events: Vec<Event>,
        app: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<ClippedShape> {
        self.time += dt;
        let input = egui::RawInput {
            screen_rect: Some(self.screen),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let mut output = self.ctx.run_ui(input, |ui| app(ui));
        output.textures_delta.clear();
        output.shapes
    }

    /// Runs `seconds` of idle frames at 60 Hz; returns the last frame's
    /// shapes.
    pub fn settle(
        &mut self,
        seconds: f64,
        app: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<ClippedShape> {
        let mut shapes = Vec::new();
        let mut elapsed = 0.0;
        while elapsed < seconds {
            shapes = self.frame(1.0 / 60.0, vec![], app);
            elapsed += 1.0 / 60.0;
        }
        shapes
    }

    /// A click at `pos`: move, press, release, over three frames.
    pub fn click(&mut self, pos: Pos2, app: &mut dyn FnMut(&mut egui::Ui)) {
        self.frame(1.0 / 60.0, vec![Event::PointerMoved(pos)], app);
        self.frame(1.0 / 60.0, vec![button(pos, true)], app);
        self.frame(1.0 / 60.0, vec![button(pos, false)], app);
    }

    /// Escape pressed and released in one frame.
    pub fn escape(&mut self, app: &mut dyn FnMut(&mut egui::Ui)) {
        self.frame(
            1.0 / 60.0,
            vec![key(Key::Escape, true), key(Key::Escape, false)],
            app,
        );
    }
}

/// A primary pointer button event.
pub fn button(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

/// A key event.
pub fn key(key: Key, pressed: bool) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

/// Every rect painted, nested shape lists included.
pub fn rects(shapes: &[ClippedShape]) -> Vec<RectShape> {
    let mut out = Vec::new();
    for clipped in shapes {
        collect_rects(&clipped.shape, &mut out);
    }
    out
}

fn collect_rects(shape: &PaintShape, out: &mut Vec<RectShape>) {
    match shape {
        PaintShape::Rect(r) => out.push(r.clone()),
        PaintShape::Vec(shapes) => {
            for s in shapes {
                collect_rects(s, out);
            }
        }
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

/// The fill of the rect covering the whole of `screen`, painted last (the
/// top scrim), if any.
pub fn scrim_fill(shapes: &[ClippedShape], screen: Rect) -> Option<egui::Color32> {
    rects(shapes)
        .into_iter()
        .filter(|r| r.rect.contains_rect(screen) && r.blur_width == 0.0)
        .map(|r| r.fill)
        .rfind(|fill| *fill != egui::Color32::TRANSPARENT)
}

// ---------------------------------------------------------------------------
// Rendering helpers: one renderer at a time, and no GPU adapter is a skip.

fn one_renderer_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    static RENDERER: std::sync::Mutex<()> = std::sync::Mutex::new(());
    RENDERER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Runs a render, turning a panic into an error: with no GPU adapter at
/// all, egui_kittest's renderer panics instead of returning one.
fn no_adapter_is_an_error<T>(render: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(render))
        .unwrap_or_else(|_| Err("no adapter: the renderer could not be created".to_owned()))
}

// ---------------------------------------------------------------------------
// Shared fixtures

const SIZE: Vec2 = Vec2::new(800.0, 600.0);

fn theme() -> MaterialTheme {
    MaterialTheme::baseline(false)
}

/// The surface rect painted in `fill`, largest first.
fn surface(shapes: &[ClippedShape], fill: Color32) -> Option<RectShape> {
    let mut found: Vec<RectShape> = rects(shapes)
        .into_iter()
        .filter(|r| r.fill == fill)
        .collect();
    found.sort_by(|a, b| b.rect.area().total_cmp(&a.rect.area()));
    found.into_iter().next()
}

// ---------------------------------------------------------------------------
// Scrim

#[test]
fn scrim_is_the_scrim_role_at_32_percent() -> TestResult {
    assert!(
        (SCRIM_OPACITY - 0.32).abs() < f32::EPSILON,
        "ScrimTokens.ContainerOpacity is 0.32"
    );
    let mut driver = Driver::new(SIZE);
    let open = Cell::new(true);
    let mut app = |ui: &mut egui::Ui| {
        let mut o = open.get();
        MaterialDialog::new(egui::Id::unique("scrim"), "Title", &mut o).show(ui.ctx());
        open.set(o);
    };
    let shapes = driver.settle(1.0, &mut app);
    let fill = scrim_fill(&shapes, driver.screen).ok_or("a scrim covers the window")?;
    let expected = theme().color(ColorRole::Scrim).gamma_multiply(0.32);
    assert_eq!(fill, expected, "scrim colour");
    assert!(
        (81..=82).contains(&fill.a()),
        "alpha is 32 % of 255: {}",
        fill.a()
    );
    Ok(())
}

#[test]
fn scrim_pixel_darkens_white_by_32_percent() -> TestResult {
    let mut harness = support::harness_for(theme(), SIZE, |ui| {
        ui.painter()
            .rect_filled(ui.ctx().content_rect(), 0.0, Color32::WHITE);
        let mut open = true;
        MaterialDialog::new(egui::Id::unique("pixel"), "Title", &mut open).show(ui.ctx());
    });
    harness.run();
    let _renderer = one_renderer_at_a_time();
    let image = match no_adapter_is_an_error(|| harness.render()) {
        Ok(image) => image,
        Err(err) => {
            eprintln!("skipping pixel sample, no renderer: {err}");
            return Ok(());
        }
    };
    // Top left corner: the scrim over the white background.
    let pixel = image.get_pixel(4, 4).0;
    let [r, g, b, _] = pixel;
    let expected = 255.0 * (1.0 - 0.32);
    for (channel, value) in [("r", r), ("g", g), ("b", b)] {
        assert!(
            (f32::from(value) - expected).abs() <= 6.0,
            "{channel} = {value}, expected about {expected} (white under a 32 % black scrim)"
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Basic dialog: tokens and geometry

fn dialog_frame(
    width_text: &'static str,
    configure: fn(MaterialDialog<'_>) -> MaterialDialog<'_>,
) -> Result<RectShape, Box<dyn std::error::Error>> {
    let mut driver = Driver::new(SIZE);
    let mut app = |ui: &mut egui::Ui| {
        let mut open = true;
        configure(MaterialDialog::new(
            egui::Id::unique("geometry"),
            "Title",
            &mut open,
        ))
        .content(move |ui, _| {
            ui.label(width_text);
        })
        .show(ui.ctx());
    };
    let shapes = driver.settle(1.0, &mut app);
    let container = surface(&shapes, theme().color(ColorRole::SurfaceContainerHigh))
        .ok_or("dialog container painted in surfaceContainerHigh")?;
    Ok(container)
}

#[test]
fn dialog_container_follows_the_dialog_tokens() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let mut app = |ui: &mut egui::Ui| {
        let mut open = true;
        MaterialDialog::new(egui::Id::unique("tokens"), "Title", &mut open)
            .icon("delete")
            .text_action("OK", || {})
            .show(ui.ctx());
    };
    let shapes = driver.settle(1.0, &mut app);
    let container = surface(&shapes, theme().color(ColorRole::SurfaceContainerHigh))
        .ok_or("container painted")?;
    assert_eq!(
        container.corner_radius,
        egui::CornerRadius::same(28),
        "DialogTokens.ContainerShape: CornerExtraLarge"
    );
    let shadows: Vec<RectShape> = rects(&shapes)
        .into_iter()
        .filter(|r| r.blur_width > 0.0 && r.rect.intersects(container.rect))
        .collect();
    assert_eq!(shadows.len(), 2, "Level3: a key and an ambient shadow");
    assert!(
        shadows.iter().any(|s| (s.blur_width - 12.0).abs() < 0.5),
        "Level3 (6 dp) shadows blur 12: {shadows:?}"
    );
    Ok(())
}

#[test]
fn dialog_width_stays_within_280_and_560() -> TestResult {
    let narrow = dialog_frame("Hi", |d| d)?;
    assert!(
        (narrow.rect.width() - 280.0).abs() < 0.5,
        "short content: the 280 dp minimum, got {}",
        narrow.rect.width()
    );
    let long = "A long supporting text that keeps going well past the maximum width of a \
                basic dialog so that it has to wrap onto more than one line.";
    let wide = dialog_frame(long, |d| d)?;
    assert!(
        wide.rect.width() <= 560.5 && wide.rect.width() > 500.0,
        "long content wraps within the 560 dp maximum, got {}",
        wide.rect.width()
    );
    let custom_min = dialog_frame("Hi", |d| d.min_width(320.0))?;
    assert!(
        (custom_min.rect.width() - 320.0).abs() < 0.5,
        "min_width(320) honoured, got {}",
        custom_min.rect.width()
    );
    let custom_max = dialog_frame(long, |d| d.max_width(400.0))?;
    assert!(
        custom_max.rect.width() <= 400.5 && custom_max.rect.width() > 350.0,
        "max_width(400) honoured, got {}",
        custom_max.rect.width()
    );
    let form = dialog_frame("Hi", |d| d.dialog_type(DialogType::Form))?;
    assert!(
        (form.rect.width() - 560.0).abs() < 0.5,
        "a form dialog takes the full width, got {}",
        form.rect.width()
    );
    Ok(())
}

#[test]
fn dialog_enters_and_leaves_with_motion() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let open = Cell::new(true);
    let mut app = |ui: &mut egui::Ui| {
        let mut o = open.get();
        MaterialDialog::new(egui::Id::unique("motion"), "Title", &mut o).show(ui.ctx());
        open.set(o);
    };
    driver.frame(0.0, vec![], &mut app);
    let early = driver.frame(0.05, vec![], &mut app);
    let early_alpha = scrim_fill(&early, driver.screen).map_or(0, |c| c.a());
    assert!(
        early_alpha < 81,
        "50 ms in, the scrim is still fading in: {early_alpha}"
    );
    let shown = driver.settle(0.5, &mut app);
    let shown_alpha = scrim_fill(&shown, driver.screen).map_or(0, |c| c.a());
    assert!(
        shown_alpha >= 81,
        "after 400 ms the scrim is at 32 %: {shown_alpha}"
    );

    open.set(false);
    driver.frame(1.0 / 60.0, vec![], &mut app);
    let leaving = driver.frame(0.05, vec![], &mut app);
    let leaving_alpha = scrim_fill(&leaving, driver.screen).map_or(0, |c| c.a());
    assert!(
        leaving_alpha > 0 && leaving_alpha < 81,
        "closing fades the scrim out: {leaving_alpha}"
    );
    let gone = driver.settle(0.4, &mut app);
    assert!(
        scrim_fill(&gone, driver.screen).is_none(),
        "after the 200 ms exit nothing is painted"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Basic dialog: closing and the response

#[test]
fn escape_closes_only_the_top_dialog() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let lower = Cell::new(true);
    let upper = Cell::new(false);
    let lower_reason = Cell::new(None);
    let upper_reason = Cell::new(None);
    let mut app = |ui: &mut egui::Ui| {
        let mut o = lower.get();
        let r = MaterialDialog::new(egui::Id::unique("lower"), "Lower", &mut o).show(ui.ctx());
        lower.set(o);
        if r.close_reason.is_some() {
            lower_reason.set(r.close_reason);
        }
        let mut o = upper.get();
        let r = MaterialDialog::new(egui::Id::unique("upper"), "Upper", &mut o).show(ui.ctx());
        upper.set(o);
        if r.close_reason.is_some() {
            upper_reason.set(r.close_reason);
        }
    };
    driver.settle(0.5, &mut app);
    upper.set(true);
    driver.settle(0.5, &mut app);

    driver.escape(&mut app);
    assert!(!upper.get(), "Escape closed the top dialog");
    assert!(lower.get(), "the dialog underneath stays open");
    assert_eq!(
        upper_reason.get(),
        Some(DialogCloseReason::Escape),
        "reason"
    );

    driver.settle(0.5, &mut app);
    driver.escape(&mut app);
    assert!(!lower.get(), "a second Escape closes the remaining dialog");
    assert_eq!(
        lower_reason.get(),
        Some(DialogCloseReason::Escape),
        "reason"
    );
    Ok(())
}

#[test]
fn scrim_click_closes_unless_disabled() -> TestResult {
    for dismissible in [true, false] {
        let mut driver = Driver::new(SIZE);
        let open = Cell::new(true);
        let reason = Cell::new(None);
        let mut app = |ui: &mut egui::Ui| {
            let mut o = open.get();
            let r = MaterialDialog::new(egui::Id::unique("scrim_click"), "Title", &mut o)
                .dismiss_on_scrim(dismissible)
                .show(ui.ctx());
            open.set(o);
            if r.close_reason.is_some() {
                reason.set(r.close_reason);
            }
        };
        driver.settle(0.5, &mut app);
        driver.click(Pos2::new(10.0, 10.0), &mut app);
        if dismissible {
            assert!(!open.get(), "a scrim click closes the dialog");
            assert_eq!(reason.get(), Some(DialogCloseReason::Scrim), "reason");
        } else {
            assert!(open.get(), "dismiss_on_scrim(false) keeps it open");
            assert_eq!(reason.get(), None, "no close reason");
        }
    }
    Ok(())
}

#[test]
fn an_action_closes_with_its_index_and_runs() -> TestResult {
    let open = Cell::new(true);
    let reason = Cell::new(None);
    let inner = Cell::new(None);
    let confirmed = Cell::new(false);
    let mut harness = support::harness_for(theme(), SIZE, |ui| {
        let mut o = open.get();
        let r = MaterialDialog::new(egui::Id::unique("actions"), "Delete file?", &mut o)
            .content(|ui, _| {
                ui.label("It cannot be restored.");
                42
            })
            .text_action("Cancel", || {})
            .filled_action("Delete", || confirmed.set(true))
            .show(ui.ctx());
        open.set(o);
        if r.inner.is_some() {
            inner.set(r.inner);
        }
        if r.close_reason.is_some() {
            reason.set(r.close_reason);
        }
    });
    harness.run();
    assert_eq!(
        inner.get(),
        Some(42),
        "the content's return value comes back"
    );
    harness.get_by_label("Delete").click();
    harness.run();
    assert!(!open.get(), "an action closes the dialog");
    assert!(confirmed.get(), "the action's callback ran");
    assert_eq!(
        reason.get(),
        Some(DialogCloseReason::Action(1)),
        "the second action added"
    );
    Ok(())
}

#[test]
fn the_close_button_closes() -> TestResult {
    let open = Cell::new(true);
    let reason = Cell::new(None);
    let mut harness = support::harness_for(theme(), SIZE, |ui| {
        let mut o = open.get();
        let r = MaterialDialog::new(egui::Id::unique("close"), "Settings", &mut o)
            .close_button(true)
            .show(ui.ctx());
        open.set(o);
        if r.close_reason.is_some() {
            reason.set(r.close_reason);
        }
    });
    harness.run();
    let close = harness.get_by_label("Close");
    assert_eq!(close.accesskit_node().role(), Role::Button, "a button");
    close.click();
    harness.run();
    assert!(!open.get(), "the close button closes the dialog");
    assert_eq!(reason.get(), Some(DialogCloseReason::CloseButton), "reason");
    Ok(())
}

#[test]
fn content_can_close_with_a_reason() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let open = Cell::new(true);
    let close_now = Cell::new(false);
    let reason = Cell::new(None);
    let mut app = |ui: &mut egui::Ui| {
        let mut o = open.get();
        let r = MaterialDialog::new(egui::Id::unique("content_close"), "Title", &mut o)
            .content(|_, open| {
                if close_now.get() {
                    *open = false;
                }
            })
            .show(ui.ctx());
        open.set(o);
        if r.close_reason.is_some() {
            reason.set(r.close_reason);
        }
    };
    driver.settle(0.2, &mut app);
    assert!(open.get(), "still open");
    close_now.set(true);
    driver.frame(1.0 / 60.0, vec![], &mut app);
    assert!(!open.get(), "content closed it");
    assert_eq!(reason.get(), Some(DialogCloseReason::Content), "reason");
    Ok(())
}

// ---------------------------------------------------------------------------
// Basic dialog: accessibility and keyboard

#[test]
fn the_dialog_is_a_labelled_dialog_node() -> TestResult {
    for (name, theme) in support::themes() {
        let harness = support::harness_for(theme, SIZE, |ui| {
            let mut open = true;
            MaterialDialog::new(egui::Id::unique("a11y"), "Discard draft?", &mut open)
                .show(ui.ctx());
        });
        let dialogs: Vec<String> = harness
            .get_all_by_role(Role::Dialog)
            .filter_map(|n| n.accesskit_node().label())
            .collect();
        assert!(
            dialogs.iter().any(|l| l == "Discard draft?"),
            "{name}: a Dialog node named by the headline: {dialogs:?}"
        );
    }
    Ok(())
}

#[test]
fn tab_stays_inside_the_dialog_and_focus_returns() -> TestResult {
    let open = Cell::new(false);
    let focus_behind = Cell::new(true);
    let mut harness = support::harness_for(theme(), SIZE, |ui| {
        let behind = ui.button("Behind");
        if focus_behind.replace(false) {
            behind.request_focus();
        }
        let mut o = open.get();
        MaterialDialog::new(egui::Id::unique("trap"), "Trap", &mut o)
            .content(|ui, _| {
                let _one = ui.button("One");
                let _two = ui.button("Two");
            })
            .show(ui.ctx());
        open.set(o);
    });
    harness.run();
    assert!(harness.get_by_label("Behind").is_focused(), "starts behind");

    open.set(true);
    harness.run();
    assert!(
        !harness.get_by_label("Behind").is_focused(),
        "opening takes focus from the app"
    );

    let mut seen = Vec::new();
    for _ in 0..5 {
        harness.key_press(egui::Key::Tab);
        harness.run();
        assert!(
            !harness.get_by_label("Behind").is_focused(),
            "Tab never leaves the dialog"
        );
        let focused = ["One", "Two"]
            .into_iter()
            .find(|label| harness.get_by_label(label).is_focused());
        seen.push(focused);
    }
    assert_eq!(
        seen,
        vec![
            Some("One"),
            Some("Two"),
            Some("One"),
            Some("Two"),
            Some("One")
        ],
        "Tab cycles through the dialog's widgets"
    );

    harness.key_press(egui::Key::Escape);
    harness.run();
    assert!(!open.get(), "Escape closed it");
    assert!(
        harness.get_by_label("Behind").is_focused(),
        "focus goes back to the widget that had it"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Full-screen dialog

#[test]
fn full_screen_dialog_covers_the_window() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let open = Cell::new(true);
    let mut app = |ui: &mut egui::Ui| {
        let mut o = open.get();
        FullScreenDialog::new(egui::Id::unique("full"), "New event", &mut o)
            .action("Save")
            .show(ui.ctx(), |ui, _| {
                ui.label("Details");
            });
        open.set(o);
    };
    let shapes = driver.settle(1.0, &mut app);
    let container = surface(&shapes, theme().color(ColorRole::Surface)).ok_or("surface painted")?;
    assert!(
        (container.rect.min - driver.screen.min).length() < 0.5
            && (container.rect.max - driver.screen.max).length() < 0.5,
        "covers the window: {:?}",
        container.rect
    );
    assert!(
        (FULL_SCREEN_HEADER_HEIGHT - 56.0).abs() < f32::EPSILON,
        "FullScreenDialogTokens.HeaderContainerHeight"
    );
    Ok(())
}

#[test]
fn full_screen_dialog_action_and_close() -> TestResult {
    for (label, expected) in [
        ("Save", DialogCloseReason::Action(0)),
        ("Close", DialogCloseReason::CloseButton),
    ] {
        let open = Cell::new(true);
        let reason = Cell::new(None);
        let mut harness = support::harness_for(theme(), SIZE, |ui| {
            let mut o = open.get();
            let r = FullScreenDialog::new(egui::Id::unique("full_actions"), "New event", &mut o)
                .action("Save")
                .show(ui.ctx(), |ui, _| {
                    ui.label("Details");
                });
            open.set(o);
            if r.close_reason.is_some() {
                reason.set(r.close_reason);
            }
        });
        harness.run();
        harness.get_by_label(label).click();
        harness.run();
        assert!(!open.get(), "{label} closes the dialog");
        assert_eq!(reason.get(), Some(expected), "{label}: reason");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Bottom sheet

fn handle_center(shapes: &[ClippedShape]) -> Option<Pos2> {
    let color = theme().color(ColorRole::OnSurfaceVariant);
    rects(shapes)
        .into_iter()
        .find(|r| {
            r.fill == color
                && (r.rect.width() - DRAG_HANDLE_WIDTH).abs() < 0.5
                && (r.rect.height() - DRAG_HANDLE_HEIGHT).abs() < 0.5
        })
        .map(|r| r.rect.center())
}

struct SheetFixture {
    open: Cell<bool>,
    reason: Cell<Option<SheetCloseReason>>,
    expanded: Cell<bool>,
}

impl SheetFixture {
    fn new() -> Self {
        Self {
            open: Cell::new(true),
            reason: Cell::new(None),
            expanded: Cell::new(false),
        }
    }

    fn show(&self, ui: &mut egui::Ui, kind: BottomSheetKind) {
        let mut o = self.open.get();
        let r =
            BottomSheet::new(egui::Id::unique("sheet"), &mut o, kind).show(ui.ctx(), |ui, _| {
                ui.label("Sheet body");
                ui.add_space(240.0);
            });
        self.open.set(o);
        self.expanded.set(r.expanded);
        if r.close_reason.is_some() {
            self.reason.set(r.close_reason);
        }
    }
}

#[test]
fn bottom_sheet_follows_the_sheet_tokens() -> TestResult {
    let mut driver = Driver::new(Vec2::new(1200.0, 600.0));
    let sheet = SheetFixture::new();
    let mut app = |ui: &mut egui::Ui| sheet.show(ui, BottomSheetKind::Modal);
    let shapes = driver.settle(1.5, &mut app);
    let container = surface(&shapes, theme().color(ColorRole::SurfaceContainerLow))
        .ok_or("sheet painted in surfaceContainerLow")?;
    assert!(
        (container.rect.width() - SHEET_MAX_WIDTH).abs() < 0.5,
        "640 dp maximum width: {}",
        container.rect.width()
    );
    assert!(
        (container.rect.center().x - 600.0).abs() < 0.5,
        "centred: {:?}",
        container.rect
    );
    assert!(
        (container.rect.bottom() - 600.0).abs() < 0.5,
        "docked to the bottom: {:?}",
        container.rect
    );
    assert_eq!(
        container.corner_radius,
        egui::CornerRadius {
            nw: 28,
            ne: 28,
            sw: 0,
            se: 0
        },
        "CornerExtraLargeTop"
    );
    let handle = handle_center(&shapes).ok_or("a 32 x 4 drag handle")?;
    assert!(
        (handle.y - (container.rect.top() + 24.0)).abs() < 0.5,
        "handle centred in 22 dp padding: {handle:?}"
    );
    let scrim = scrim_fill(&shapes, driver.screen).ok_or("modal sheet has a scrim")?;
    assert!((81..=82).contains(&scrim.a()), "32 % scrim: {}", scrim.a());
    Ok(())
}

#[test]
fn dragging_a_modal_sheet_past_the_threshold_dismisses_it() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let sheet = SheetFixture::new();
    let mut app = |ui: &mut egui::Ui| sheet.show(ui, BottomSheetKind::Modal);
    let shapes = driver.settle(1.5, &mut app);
    let start = handle_center(&shapes).ok_or("handle")?;

    driver.frame(1.0 / 60.0, vec![Event::PointerMoved(start)], &mut app);
    driver.frame(1.0 / 60.0, vec![button(start, true)], &mut app);
    for step in 1..=8 {
        let pos = start + Vec2::new(0.0, 15.0 * step as f32);
        driver.frame(1.0 / 60.0, vec![Event::PointerMoved(pos)], &mut app);
    }
    let end = start + Vec2::new(0.0, 120.0);
    driver.frame(1.0 / 60.0, vec![button(end, false)], &mut app);
    assert!(!sheet.open.get(), "a 120 dp drag down dismisses the sheet");
    assert_eq!(
        sheet.reason.get(),
        Some(SheetCloseReason::Dragged),
        "reason"
    );
    let gone = driver.settle(1.5, &mut app);
    assert!(
        surface(&gone, theme().color(ColorRole::SurfaceContainerLow)).is_none(),
        "the sheet slides away"
    );
    Ok(())
}

#[test]
fn a_short_slow_drag_springs_back() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let sheet = SheetFixture::new();
    let mut app = |ui: &mut egui::Ui| sheet.show(ui, BottomSheetKind::Modal);
    let shapes = driver.settle(1.5, &mut app);
    let start = handle_center(&shapes).ok_or("handle")?;
    let resting = surface(&shapes, theme().color(ColorRole::SurfaceContainerLow))
        .ok_or("sheet")?
        .rect;

    driver.frame(1.0 / 60.0, vec![Event::PointerMoved(start)], &mut app);
    driver.frame(1.0 / 60.0, vec![button(start, true)], &mut app);
    for step in 1..=10 {
        let pos = start + Vec2::new(0.0, 3.0 * step as f32);
        driver.frame(0.1, vec![Event::PointerMoved(pos)], &mut app);
    }
    let end = start + Vec2::new(0.0, 30.0);
    // Hold still so the release is not a fling.
    driver.frame(0.3, vec![], &mut app);
    driver.frame(1.0 / 60.0, vec![button(end, false)], &mut app);
    assert!(sheet.open.get(), "30 dp is under the 56 dp threshold");
    let shapes = driver.settle(1.5, &mut app);
    let back = surface(&shapes, theme().color(ColorRole::SurfaceContainerLow))
        .ok_or("sheet")?
        .rect;
    assert!(
        (back.top() - resting.top()).abs() < 1.0,
        "springs back to {resting:?}, got {back:?}"
    );
    Ok(())
}

#[test]
fn a_standard_sheet_collapses_and_expands_from_its_handle() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let sheet = SheetFixture::new();
    let mut app = |ui: &mut egui::Ui| sheet.show(ui, BottomSheetKind::Standard);
    let shapes = driver.settle(1.5, &mut app);
    assert!(
        scrim_fill(&shapes, driver.screen).is_none(),
        "a standard sheet has no scrim"
    );
    assert!(sheet.expanded.get(), "opens expanded");
    let handle = handle_center(&shapes).ok_or("handle")?;

    driver.click(handle, &mut app);
    let shapes = driver.settle(1.5, &mut app);
    assert!(!sheet.expanded.get(), "the handle collapses it");
    let collapsed = surface(&shapes, theme().color(ColorRole::SurfaceContainerLow))
        .ok_or("sheet")?
        .rect;
    assert!(
        (600.0 - collapsed.top() - SHEET_PEEK_HEIGHT).abs() < 1.0,
        "the peek height stays on screen: {collapsed:?}"
    );

    let handle = handle_center(&shapes).ok_or("handle still on screen")?;
    driver.click(handle, &mut app);
    driver.settle(1.5, &mut app);
    assert!(sheet.expanded.get(), "and expands it again");
    assert!(sheet.open.get(), "a standard sheet stays open");
    Ok(())
}

#[test]
fn escape_closes_a_dialog_over_a_sheet_first() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let sheet = SheetFixture::new();
    let dialog_open = Cell::new(false);
    let mut app = |ui: &mut egui::Ui| {
        sheet.show(ui, BottomSheetKind::Modal);
        let mut o = dialog_open.get();
        MaterialDialog::new(egui::Id::unique("over_sheet"), "Confirm", &mut o).show(ui.ctx());
        dialog_open.set(o);
    };
    driver.settle(1.0, &mut app);
    dialog_open.set(true);
    driver.settle(0.6, &mut app);

    driver.escape(&mut app);
    assert!(!dialog_open.get(), "Escape closed the dialog on top");
    assert!(sheet.open.get(), "the sheet underneath stays open");

    driver.settle(0.6, &mut app);
    driver.escape(&mut app);
    assert!(!sheet.open.get(), "the next Escape closes the sheet");
    assert_eq!(sheet.reason.get(), Some(SheetCloseReason::Escape), "reason");
    Ok(())
}

#[test]
fn a_scrim_click_dismisses_a_modal_sheet() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let sheet = SheetFixture::new();
    let mut app = |ui: &mut egui::Ui| sheet.show(ui, BottomSheetKind::Modal);
    driver.settle(1.5, &mut app);
    driver.click(Pos2::new(10.0, 10.0), &mut app);
    assert!(!sheet.open.get(), "dismissed");
    assert_eq!(sheet.reason.get(), Some(SheetCloseReason::Scrim), "reason");
    Ok(())
}

#[test]
fn the_drag_handle_is_a_labelled_button() -> TestResult {
    let harness = support::harness_for(theme(), SIZE, |ui| {
        let mut open = true;
        BottomSheet::modal(egui::Id::unique("a11y_sheet"), &mut open).show(ui.ctx(), |ui, _| {
            ui.label("Body");
        });
    });
    assert_eq!(
        harness.get_by_label("Drag handle").accesskit_node().role(),
        Role::Button,
        "drag handle role"
    );
    assert!(
        harness
            .query_by_role_and_label(Role::Dialog, "Bottom sheet")
            .is_some(),
        "a modal sheet is a dialog node"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Side sheet

#[test]
fn a_modal_side_sheet_docks_to_the_end_edge() -> TestResult {
    for direction in [LayoutDirection::Ltr, LayoutDirection::Rtl] {
        let mut driver = Driver::new(SIZE);
        MaterialTheme::set(
            &driver.ctx,
            MaterialTheme {
                layout_direction: direction,
                ..theme()
            },
        );
        let open = Cell::new(true);
        let reason = Cell::new(None);
        let mut app = |ui: &mut egui::Ui| {
            let mut o = open.get();
            let r = SideSheet::modal(egui::Id::unique("side"), &mut o)
                .title("Filters")
                .show(ui.ctx(), |ui, _| {
                    ui.label("Body");
                });
            open.set(o);
            if r.close_reason.is_some() {
                reason.set(r.close_reason);
            }
        };
        let shapes = driver.settle(1.5, &mut app);
        let container = surface(&shapes, theme().color(ColorRole::SurfaceContainerLow))
            .ok_or("side sheet painted")?;
        assert!(
            (container.rect.width() - SIDE_SHEET_WIDTH).abs() < 0.5,
            "{direction:?}: width {}",
            container.rect.width()
        );
        match direction {
            LayoutDirection::Ltr => assert!(
                (container.rect.right() - 800.0).abs() < 0.5,
                "LTR: right edge, {:?}",
                container.rect
            ),
            LayoutDirection::Rtl => assert!(
                container.rect.left().abs() < 0.5,
                "RTL: left edge, {:?}",
                container.rect
            ),
        }
        assert!(
            scrim_fill(&shapes, driver.screen).is_some(),
            "{direction:?}: scrim"
        );
        driver.escape(&mut app);
        assert!(!open.get(), "{direction:?}: Escape closes it");
        assert_eq!(reason.get(), Some(SheetCloseReason::Escape), "reason");
    }
    Ok(())
}

#[test]
fn a_standard_side_sheet_has_no_scrim() -> TestResult {
    let mut driver = Driver::new(SIZE);
    let mut app = |ui: &mut egui::Ui| {
        let mut open = true;
        SideSheet::standard(egui::Id::unique("side_standard"), &mut open).show(
            ui.ctx(),
            |ui, _| {
                ui.label("Body");
            },
        );
    };
    let shapes = driver.settle(1.5, &mut app);
    assert!(
        surface(&shapes, theme().color(ColorRole::Surface)).is_some(),
        "standard side sheet painted in surface"
    );
    assert!(scrim_fill(&shapes, driver.screen).is_none(), "no scrim");
    Ok(())
}

// ---------------------------------------------------------------------------
// Action sheet (on BottomSheet)

#[test]
fn an_action_sheet_button_runs_and_closes() -> TestResult {
    let open = Cell::new(true);
    let archived = Cell::new(false);
    let mut harness = support::harness_for(theme(), SIZE, |ui| {
        let mut o = open.get();
        let _response = MaterialActionSheet::new(egui::Id::unique("actions_sheet"), &mut o)
            .button("Archive", || archived.set(true))
            .button("Cancel", || {})
            .show(ui.ctx());
        open.set(o);
    });
    harness.run_steps(60);
    harness.get_by_label("Archive").click();
    harness.run_steps(60);
    assert!(archived.get(), "the callback ran");
    assert!(!open.get(), "the sheet closed");
    Ok(())
}
