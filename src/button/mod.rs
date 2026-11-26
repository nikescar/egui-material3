//! Material Design 3 common buttons and toggle buttons.
//!
//! [`MaterialButton`] draws the five M3 button styles (elevated, filled,
//! tonal, outlined, text) from the Compose tokens, in the five M3
//! Expressive sizes ([`ButtonSize`]) or the baseline 40 dp size, round or
//! square ([`ButtonShape`]).
//!
//! - **Colours** come from the active [`MaterialTheme`](crate::MaterialTheme):
//!   see [`MaterialButtonVariant::colors`] for the role each style uses,
//!   including the selected/unselected sets of toggle buttons and the
//!   disabled colours (`onSurface` at 10% for the container, 38% for the
//!   content).
//! - **States**: hover, focus and press draw the M3 state layer in the
//!   content colour; keyboard focus draws the focus ring; elevation follows
//!   the per-state tokens ([`MaterialButtonVariant::elevation`]).
//! - **Shape morph**: a pressed button springs to the size's pressed
//!   corner radius, and a selected toggle button to its selected shape,
//!   using the theme's fast spatial spring.
//! - **Toggle buttons**: [`MaterialButton::toggle`] binds a `&mut bool`; the
//!   button reports its toggled state to AccessKit.
//! - A disabled button cannot be clicked or focused.
//!
//! See also [`ButtonGroup`](crate::ButtonGroup) and
//! [`SplitButton`](crate::SplitButton).

mod builder;
mod rendering;
mod tokens;
mod types;

pub use tokens::{ButtonColors, ButtonMetrics};
pub use types::{ButtonShape, ButtonSize, MaterialButton, MaterialButtonVariant};
pub(crate) use types::{SegmentCorners, SideCorners};
