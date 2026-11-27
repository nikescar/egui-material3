//! [`MaterialTheme`]: the active theme, stored per egui [`egui::Context`].
//!
//! The theme lives in the context's data (`ctx.data`), so two contexts (two
//! viewports, or two tests running in parallel) can hold different themes.
//! A context that has none set falls back to the process-wide theme that
//! [`crate::theme`] manages, so code written against the old global API
//! keeps working unchanged.

use std::sync::Arc;

use egui::{Color32, Context, Id};

use crate::theme::{ContrastLevel, MaterialThemeContext, MaterialThemeFile, ThemeMode};
use crate::tokens::{ColorRole, ColorScheme, LayoutDirection, MotionScheme};

/// The resolved Material 3 theme a widget draws with.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialTheme {
    /// One colour per role.
    pub colors: ColorScheme,
    /// Whether `colors` is a dark scheme.
    pub is_dark: bool,
    /// Standard or Expressive springs.
    pub motion: MotionScheme,
    /// Left-to-right or right-to-left.
    pub layout_direction: LayoutDirection,
}

impl Default for MaterialTheme {
    fn default() -> Self {
        Self::baseline(false)
    }
}

fn theme_id() -> Id {
    Id::unique("egui_material3::MaterialTheme")
}

impl MaterialTheme {
    /// Compose's baseline scheme, light or dark.
    pub fn baseline(dark: bool) -> Self {
        Self {
            colors: ColorScheme::baseline(dark),
            is_dark: dark,
            motion: MotionScheme::default(),
            layout_direction: LayoutDirection::default(),
        }
    }

    /// A theme from a Theme Builder export, picking the light or dark scheme
    /// at `contrast`. Roles missing from the file keep their baseline value.
    pub fn from_theme_file(file: &MaterialThemeFile, dark: bool, contrast: ContrastLevel) -> Self {
        let context = MaterialThemeContext {
            theme_mode: if dark {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
            contrast_level: contrast,
            material_theme: Some(file.clone()),
            dark_mode: dark,
            ..Default::default()
        };
        Self::from_context(&context)
    }

    /// The theme a [`MaterialThemeContext`] describes, at its current
    /// light/dark state.
    pub fn from_context(context: &MaterialThemeContext) -> Self {
        let dark = match context.theme_mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::Auto => context.dark_mode,
        };
        Self {
            colors: context.color_scheme_for_mode(dark),
            ..Self::baseline(dark)
        }
    }

    /// The theme for `ctx`: the one set with [`MaterialTheme::set`], or else
    /// the process-wide theme from [`crate::theme`].
    pub fn of(ctx: &Context) -> Arc<MaterialTheme> {
        if let Some(theme) = ctx.data(|d| d.get_temp::<Arc<MaterialTheme>>(theme_id())) {
            return theme;
        }
        let global = crate::theme::get_global_theme();
        let theme = match global.lock() {
            Ok(context) => Self::from_context(&context),
            Err(poisoned) => Self::from_context(&poisoned.into_inner()),
        };
        Arc::new(theme)
    }

    /// Sets the theme for `ctx`. Widgets drawn in that context use it from
    /// then on; other contexts are not affected.
    pub fn set(ctx: &Context, theme: MaterialTheme) {
        ctx.data_mut(|d| d.insert_temp(theme_id(), Arc::new(theme)));
    }

    /// Removes the theme set for `ctx`, going back to the process-wide one.
    pub fn clear(ctx: &Context) {
        ctx.data_mut(|d| d.remove::<Arc<MaterialTheme>>(theme_id()));
    }

    /// The colour of `role`.
    #[inline]
    pub fn color(&self, role: ColorRole) -> Color32 {
        self.colors.get(role)
    }
}
