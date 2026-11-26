//! Material 3 Expressive split button.
//!
//! A [`SplitButton`] is a leading action button and a trailing menu button
//! joined 2 dp apart (`SplitButton*Tokens.BetweenSpace`). Their outer ends
//! are fully rounded; the inner corners follow the size's
//! `InnerCornerCornerSize`, widen on hover and press
//! (`InnerHoveredCornerCornerSize`, `InnerPressedCornerCornerSize`), and the
//! trailing button becomes fully round while its menu is open
//! (`TrailingInnerSelectedCornerCornerSizePercent` 50%), with its chevron
//! turned upside down. Both springs use the theme's fast spatial spec.
//!
//! The split button only tracks whether the menu is open; show the menu
//! yourself under [`SplitButtonResponse::trailing`].

use egui::{Pos2, Response, Role, Shape, Stroke, Ui, Vec2, Widget, WidgetText};

use crate::button::{
    ButtonSize, MaterialButton, MaterialButtonVariant, SegmentCorners, SideCorners,
};
use crate::tokens::{animate_value_with_spring, CornerSize, SpringSpeed};
use crate::MaterialTheme;

/// `SplitButton*Tokens.BetweenSpace`: 2 dp.
pub const SPLIT_GAP: f32 = 2.0;

/// The geometry of one split-button size (`SplitButton<Size>Tokens`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplitButtonMetrics {
    /// `ContainerHeight`.
    pub height: f32,
    /// `LeadingButtonLeadingSpace`.
    pub leading_button_leading_space: f32,
    /// `LeadingButtonTrailingSpace`.
    pub leading_button_trailing_space: f32,
    /// `TrailingButtonLeadingSpace`.
    pub trailing_button_leading_space: f32,
    /// `TrailingButtonTrailingSpace`.
    pub trailing_button_trailing_space: f32,
    /// `TrailingIconSize`: the chevron.
    pub trailing_icon_size: f32,
    /// `InnerCornerCornerSize`.
    pub inner_corner: CornerSize,
    /// `InnerHoveredCornerCornerSize`.
    pub inner_hovered_corner: CornerSize,
    /// `InnerPressedCornerCornerSize`.
    pub inner_pressed_corner: CornerSize,
}

impl SplitButtonMetrics {
    /// The token metrics for `size`.
    pub fn of(size: ButtonSize) -> Self {
        let m = |height,
                 leading: (f32, f32),
                 trailing: f32,
                 icon,
                 corners: (CornerSize, CornerSize)| SplitButtonMetrics {
            height,
            leading_button_leading_space: leading.0,
            leading_button_trailing_space: leading.1,
            trailing_button_leading_space: trailing,
            trailing_button_trailing_space: trailing,
            trailing_icon_size: icon,
            inner_corner: corners.0,
            inner_hovered_corner: corners.1,
            inner_pressed_corner: corners.1,
        };
        match size {
            ButtonSize::ExtraSmall => m(
                32.0,
                (12.0, 10.0),
                13.0,
                22.0,
                (CornerSize::ExtraSmall, CornerSize::Small),
            ),
            ButtonSize::Small => m(
                40.0,
                (16.0, 12.0),
                13.0,
                22.0,
                (CornerSize::ExtraSmall, CornerSize::Medium),
            ),
            ButtonSize::Medium => m(
                56.0,
                (24.0, 24.0),
                15.0,
                26.0,
                (CornerSize::ExtraSmall, CornerSize::Medium),
            ),
            ButtonSize::Large => m(
                96.0,
                (48.0, 48.0),
                29.0,
                38.0,
                (CornerSize::Small, CornerSize::LargeIncreased),
            ),
            ButtonSize::ExtraLarge => m(
                136.0,
                (64.0, 64.0),
                43.0,
                50.0,
                (CornerSize::Medium, CornerSize::LargeIncreased),
            ),
        }
    }
}

/// A leading action plus a trailing menu toggle.
///
/// ```no_run
/// # egui::__run_test_ui(|ui| {
/// use egui_material3::SplitButton;
/// let mut open = false;
/// let split = SplitButton::filled("Send", &mut open).show(ui);
/// if split.leading.clicked() { /* send */ }
/// if open {
///     // show a menu under `split.trailing.rect`
/// }
/// # });
/// ```
#[must_use = "You should call `.show(ui)` or `ui.add(split_button)`"]
pub struct SplitButton<'a> {
    variant: MaterialButtonVariant,
    text: WidgetText,
    open: &'a mut bool,
    size: ButtonSize,
    leading_icon: Option<String>,
    menu_label: String,
    enabled: bool,
}

/// What a [`SplitButton`] reports back.
pub struct SplitButtonResponse {
    /// Both halves together.
    pub response: Response,
    /// The leading action button: check `clicked()`.
    pub leading: Response,
    /// The trailing menu button; `clicked()` flips the open flag.
    pub trailing: Response,
}

impl<'a> SplitButton<'a> {
    /// A split button in `variant`, tracking its menu in `open`.
    pub fn new(
        variant: MaterialButtonVariant,
        text: impl Into<WidgetText>,
        open: &'a mut bool,
    ) -> Self {
        Self {
            variant,
            text: text.into(),
            open,
            size: ButtonSize::Small,
            leading_icon: None,
            menu_label: "More options".to_owned(),
            enabled: true,
        }
    }

    /// A filled split button.
    pub fn filled(text: impl Into<WidgetText>, open: &'a mut bool) -> Self {
        Self::new(MaterialButtonVariant::Filled, text, open)
    }

    /// A tonal split button.
    pub fn filled_tonal(text: impl Into<WidgetText>, open: &'a mut bool) -> Self {
        Self::new(MaterialButtonVariant::FilledTonal, text, open)
    }

    /// An elevated split button.
    pub fn elevated(text: impl Into<WidgetText>, open: &'a mut bool) -> Self {
        Self::new(MaterialButtonVariant::Elevated, text, open)
    }

    /// An outlined split button.
    pub fn outlined(text: impl Into<WidgetText>, open: &'a mut bool) -> Self {
        Self::new(MaterialButtonVariant::Outlined, text, open)
    }

    /// Size XS to XL (default Small).
    #[inline]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// An icon before the leading label (a Material Symbols name).
    #[inline]
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// The trailing button's accessible label (default "More options").
    #[inline]
    pub fn menu_label(mut self, label: impl Into<String>) -> Self {
        self.menu_label = label.into();
        self
    }

    /// Enable or disable both halves.
    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Lays out and paints the split button.
    pub fn show(self, ui: &mut Ui) -> SplitButtonResponse {
        let SplitButton {
            variant,
            text,
            open,
            size,
            leading_icon,
            menu_label,
            enabled,
        } = self;
        let metrics = SplitButtonMetrics::of(size);
        let height = metrics.height;
        let full = Vec2::splat(height);
        let radius = |c: CornerSize| c.radius(egui::Rect::from_min_size(Pos2::ZERO, full));
        let inner = |selected_full| SideCorners::Inner {
            idle: radius(metrics.inner_corner),
            hovered: radius(metrics.inner_hovered_corner),
            pressed: radius(metrics.inner_pressed_corner),
            selected_full,
        };

        let inner_response = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = SPLIT_GAP;
            let mut leading =
                MaterialButton::opt_image_and_text_with_variant(variant, None, Some(text))
                    .size(size)
                    .enabled(enabled)
                    .padding(
                        metrics.leading_button_leading_space,
                        metrics.leading_button_trailing_space,
                    )
                    .segment(SegmentCorners {
                        start: SideCorners::Outer,
                        end: inner(false),
                        selected: false,
                    });
            if let Some(icon) = leading_icon {
                leading = leading.leading_icon(icon);
            }
            let leading = ui.add(leading);

            let trailing_width = metrics.trailing_button_leading_space
                + metrics.trailing_icon_size
                + metrics.trailing_button_trailing_space;
            let trailing = MaterialButton::opt_image_and_text_with_variant(variant, None, None)
                .size(size)
                .enabled(enabled)
                .label(menu_label.clone())
                .padding(0.0, 0.0)
                .exact_width(trailing_width)
                .segment(SegmentCorners {
                    start: inner(true),
                    end: SideCorners::Outer,
                    selected: *open,
                });
            let mut trailing = ui.add(trailing);
            if trailing.clicked() {
                *open = !*open;
                trailing.mark_changed();
            }
            crate::a11y::describe(
                &trailing,
                Role::Button,
                enabled && ui.is_enabled(),
                &menu_label,
                Some(*open),
            );

            let theme = MaterialTheme::of(ui.ctx());
            let angle = animate_value_with_spring(
                ui.ctx(),
                trailing.id.with("m3_chevron"),
                if *open { std::f32::consts::PI } else { 0.0 },
                theme.motion.spatial(SpringSpeed::Fast),
            );
            let color = variant
                .colors(&theme, None, enabled && ui.is_enabled())
                .content;
            paint_chevron(
                ui,
                trailing.rect.center(),
                metrics.trailing_icon_size,
                angle,
                color,
            );
            (leading, trailing)
        });
        let (leading, trailing) = inner_response.inner;
        let response = inner_response.response;
        SplitButtonResponse {
            response,
            leading,
            trailing,
        }
    }
}

/// Paints a `keyboard_arrow_down` chevron of `size`, turned by `angle`.
fn paint_chevron(ui: &Ui, center: Pos2, size: f32, angle: f32, color: egui::Color32) {
    // The glyph's stroke runs (7, 9.5) -> (12, 14.5) -> (17, 9.5) in a
    // 24-unit box, 2 units wide.
    let unit = size / 24.0;
    let rot = egui::emath::Rot2::from_angle(angle);
    let point = |x: f32, y: f32| center + rot * (Vec2::new(x - 12.0, y - 12.0) * unit);
    ui.painter().add(Shape::line(
        vec![point(7.0, 9.5), point(12.0, 14.5), point(17.0, 9.5)],
        Stroke::new(2.0 * unit, color),
    ));
}

impl Widget for SplitButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let split = self.show(ui);
        split.response.union(split.leading).union(split.trailing)
    }
}
