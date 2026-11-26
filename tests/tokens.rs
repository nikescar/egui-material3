//! Foundation token values, checked against the Compose material3 token
//! files they come from (named in each assertion).

use egui::Color32;
use egui_material3::tokens::{
    elevation::Elevation,
    motion::{self, CubicBezier, MotionScheme, SpringSpec, SpringSpeed},
    shape::{CornerSize, LayoutDirection, Shape},
    state::{self, InteractionState},
    typography::{TypeRole, WEIGHT_BOLD, WEIGHT_MEDIUM, WEIGHT_REGULAR},
    ColorRole, ColorScheme,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn baseline_colors_match_color_light_and_dark_tokens() {
    let light = ColorScheme::baseline_light();
    let dark = ColorScheme::baseline_dark();
    // ColorLightTokens.Primary = Primary40, ColorDarkTokens.Primary = Primary80.
    assert_eq!(
        light.get(ColorRole::Primary),
        Color32::from_rgb(0x67, 0x50, 0xA4),
        "light primary"
    );
    assert_eq!(
        dark.get(ColorRole::Primary),
        Color32::from_rgb(0xD0, 0xBC, 0xFF),
        "dark primary"
    );
    // ColorLightTokens.Surface = Neutral98, SurfaceContainerHighest = Neutral90.
    assert_eq!(
        light.get(ColorRole::Surface),
        Color32::from_rgb(0xFE, 0xF7, 0xFF),
        "light surface"
    );
    assert_eq!(
        light.get(ColorRole::SurfaceContainerHighest),
        Color32::from_rgb(0xE6, 0xE0, 0xE9),
        "light surfaceContainerHighest"
    );
    assert_eq!(
        light.get(ColorRole::Shadow),
        Color32::BLACK,
        "shadow is Neutral0"
    );
}

#[test]
fn every_role_name_round_trips() {
    assert_eq!(ColorRole::ALL.len(), 49, "49 roles");
    for role in ColorRole::ALL {
        assert_eq!(ColorRole::from_name(role.name()), Some(role), "{role:?}");
    }
    assert_eq!(ColorRole::from_name("onInverseSurface"), None, "not a role");
}

/// Every `get_global_color("...")` literal in `src/` names a real role: an
/// unknown name renders grey at run time, so catch it here (#21).
#[test]
fn every_color_literal_in_src_is_a_role() -> TestResult {
    let mut bad = Vec::new();
    let mut stack = vec![std::path::PathBuf::from("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path)?;
                for (_, rest) in text
                    .match_indices("get_global_color(\"")
                    .map(|(i, _)| text.split_at(i))
                {
                    let name = rest
                        .trim_start_matches("get_global_color(\"")
                        .split('"')
                        .next()
                        .unwrap_or_default();
                    if ColorRole::from_name(name).is_none() {
                        bad.push(format!("{}: {name}", path.display()));
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "unknown colour roles: {bad:?}");
    Ok(())
}

#[test]
fn type_scale_matches_type_scale_tokens() {
    let s = TypeRole::DisplayLarge.style();
    assert_eq!(
        (s.size, s.line_height, s.tracking, s.weight),
        (57.0, 64.0, -0.25, WEIGHT_REGULAR),
        "DisplayLarge"
    );
    let s = TypeRole::TitleMedium.style();
    assert_eq!(
        (s.size, s.line_height, s.tracking, s.weight),
        (16.0, 24.0, 0.15, WEIGHT_MEDIUM),
        "TitleMedium"
    );
    let s = TypeRole::LabelLarge.style();
    assert_eq!(
        (s.size, s.line_height, s.tracking, s.weight),
        (14.0, 20.0, 0.1, WEIGHT_MEDIUM),
        "LabelLarge"
    );
    let s = TypeRole::LabelSmall.style();
    assert_eq!(
        (s.size, s.line_height, s.tracking, s.weight),
        (11.0, 16.0, 0.5, WEIGHT_MEDIUM),
        "LabelSmall"
    );
    let s = TypeRole::BodySmall.style();
    assert_eq!(
        (s.size, s.line_height, s.tracking, s.weight),
        (12.0, 16.0, 0.4, WEIGHT_REGULAR),
        "BodySmall"
    );
    // *Emphasized: Medium for display/body, Bold for labels.
    assert_eq!(
        TypeRole::BodyLarge.emphasized().weight,
        WEIGHT_MEDIUM,
        "BodyLargeEmphasized"
    );
    assert_eq!(
        TypeRole::LabelLarge.emphasized().weight,
        WEIGHT_BOLD,
        "LabelLargeEmphasized"
    );
}

#[test]
fn shape_scale_matches_shape_tokens() {
    let values: Vec<Option<f32>> = [
        CornerSize::None,
        CornerSize::ExtraSmall,
        CornerSize::Small,
        CornerSize::Medium,
        CornerSize::Large,
        CornerSize::LargeIncreased,
        CornerSize::ExtraLarge,
        CornerSize::ExtraLargeIncreased,
        CornerSize::ExtraExtraLarge,
        CornerSize::Full,
    ]
    .iter()
    .map(|c| c.value())
    .collect();
    let expected = [0.0, 4.0, 8.0, 12.0, 16.0, 20.0, 28.0, 32.0, 48.0].map(Some);
    assert_eq!(values.get(..9), Some(&expected[..]), "CornerValue*");
    assert_eq!(values.get(9), Some(&None), "CornerFull is size-dependent");

    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 40.0));
    assert_eq!(
        Shape::FULL.corner_radius(rect, LayoutDirection::Ltr),
        egui::CornerRadius::same(20),
        "pill"
    );
    let end = Shape::LARGE_END.corner_radius(rect, LayoutDirection::Ltr);
    assert_eq!(
        (end.nw, end.ne, end.se, end.sw),
        (0, 16, 16, 0),
        "CornerLargeEnd"
    );
    let end_rtl = Shape::LARGE_END.corner_radius(rect, LayoutDirection::Rtl);
    assert_eq!(
        (end_rtl.nw, end_rtl.ne, end_rtl.se, end_rtl.sw),
        (16, 0, 0, 16),
        "end mirrors in RTL"
    );
}

#[test]
fn elevation_matches_elevation_tokens() {
    let dps: Vec<f32> = Elevation::ALL.iter().map(|e| e.dp()).collect();
    assert_eq!(dps, vec![0.0, 1.0, 3.0, 6.0, 8.0, 12.0], "Level0..Level5");
    let [ambient, key] = Elevation::Level0.shadows(Color32::BLACK);
    assert_eq!(
        (ambient.color, key.color),
        (Color32::TRANSPARENT, Color32::TRANSPARENT),
        "level 0 has no shadow"
    );
    let [_, low] = Elevation::Level1.shadows(Color32::BLACK);
    let [_, high] = Elevation::Level5.shadows(Color32::BLACK);
    assert!(
        high.blur > low.blur && high.offset[1] > low.offset[1],
        "higher surfaces cast softer, longer shadows"
    );
}

#[test]
fn state_layers_match_state_layer_tokens() {
    assert_eq!(
        (
            state::HOVER_OPACITY,
            state::FOCUS_OPACITY,
            state::PRESSED_OPACITY,
            state::DRAGGED_OPACITY
        ),
        (0.08, 0.10, 0.10, 0.16),
        "StateLayerTokens"
    );
    let s = |hovered, focused, pressed, dragged| {
        InteractionState {
            hovered,
            focused,
            pressed,
            dragged,
        }
        .opacity()
    };
    assert_eq!(s(false, false, false, false), 0.0, "idle");
    assert_eq!(s(true, false, false, false), 0.08, "hover");
    assert_eq!(s(true, true, false, false), 0.10, "focus beats hover");
    assert_eq!(s(true, true, true, false), 0.10, "pressed");
    assert_eq!(s(true, true, true, true), 0.16, "dragged wins");
}

#[test]
fn motion_matches_motion_tokens() {
    assert_eq!(motion::DURATION_SHORT1.as_millis(), 50, "DurationShort1");
    assert_eq!(motion::DURATION_MEDIUM2.as_millis(), 300, "DurationMedium2");
    assert_eq!(motion::DURATION_LONG4.as_millis(), 600, "DurationLong4");
    assert_eq!(
        motion::DURATION_EXTRA_LONG4.as_millis(),
        1000,
        "DurationExtraLong4"
    );
    assert_eq!(
        CubicBezier::EMPHASIZED_DECELERATE,
        CubicBezier::new(0.05, 0.7, 0.1, 1.0),
        "EmphasizedDecelerate"
    );
    assert_eq!(
        MotionScheme::Standard.spatial(SpringSpeed::Default),
        SpringSpec::new(0.9, 700.0),
        "StandardMotionTokens"
    );
    assert_eq!(
        MotionScheme::Expressive.spatial(SpringSpeed::Fast),
        SpringSpec::new(0.6, 800.0),
        "ExpressiveMotionTokens"
    );
    assert_eq!(
        MotionScheme::Expressive.effects(SpringSpeed::Slow),
        SpringSpec::new(1.0, 800.0),
        "effects springs"
    );
}

#[test]
fn easing_curves_sample_correctly() {
    let linear = CubicBezier::LINEAR;
    for x in [0.1_f32, 0.25, 0.5, 0.9] {
        assert!((linear.transform(x) - x).abs() < 1e-4, "linear at {x}");
    }
    let e = CubicBezier::STANDARD;
    assert_eq!(
        (e.transform(0.0), e.transform(1.0)),
        (0.0, 1.0),
        "endpoints"
    );
    // Standard (0.2, 0, 0, 1) is symmetric: y(0.5) = 0.5 is not, but it is
    // monotonic and ahead of linear past the start.
    let samples: Vec<f32> = (1..10).map(|i| e.transform(i as f32 / 10.0)).collect();
    assert!(
        samples.windows(2).all(|w| w.first() < w.get(1)),
        "monotonic: {samples:?}"
    );
    assert!(e.transform(0.5) > 0.5, "standard easing decelerates");
}

#[test]
fn springs_follow_the_closed_form() {
    // Critically damped: x(t) = (x0 + (v0 + w x0) t) e^(-w t), w = sqrt(k).
    let spring = SpringSpec::new(1.0, 1600.0);
    let w = 40.0_f32;
    for t in [0.01_f32, 0.05, 0.1] {
        let expected = (1.0 + w * t) * (-w * t).exp();
        let (x, _) = spring.displacement(1.0, 0.0, t);
        assert!(
            (x - expected).abs() < 1e-4,
            "critically damped at {t}: {x} vs {expected}"
        );
    }
    // Underdamped expressive spring overshoots; the standard one barely does.
    let bouncy = MotionScheme::Expressive.spatial(SpringSpeed::Fast);
    let peak = (1..200)
        .map(|i| bouncy.progress(i as f32 / 200.0))
        .fold(0.0_f32, f32::max);
    assert!(
        peak > 1.05,
        "expressive fast spring overshoots, peak {peak}"
    );
    // Overdamped settles without overshoot.
    let over = SpringSpec::new(2.0, 400.0);
    assert!(
        (1..100).all(|i| over.progress(i as f32 / 50.0) <= 1.0),
        "overdamped never overshoots"
    );
    assert!((spring.progress(1.0) - 1.0).abs() < 1e-3, "settles");
}
