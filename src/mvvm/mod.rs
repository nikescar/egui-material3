//! MVVM (Model-View-ViewModel) support for egui-material3
//!
//! This module provides reactive state management and ViewModel patterns
//! for building egui applications following the MVVM architecture.
//!
//! ## Architecture
//!
//! - **Model**: Your data structures and business logic
//! - **View**: egui-material3 components (MaterialButton, MaterialCheckbox, etc.)
//! - **ViewModel**: Reactive state and commands connecting Model and View
//!
//! ## Core Components
//!
//! - [`ValState`] - Reactive value state (owned data)
//! - [`RefState`] - Reactive reference state (Arc-based shared data)
//! - [`ViewModel`] - Trait for ViewModels with lifecycle hooks
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use egui_material3::mvvm::{ValState, ViewModel};
//! use egui_material3::MaterialButton;
//!
//! struct CounterViewModel {
//!     count: ValState<i32>,
//! }
//!
//! impl CounterViewModel {
//!     fn new() -> Self {
//!         Self {
//!             count: ValState::new(0),
//!         }
//!     }
//!
//!     fn increment(&mut self) {
//!         self.count.update(|v| *v += 1);
//!     }
//! }
//!
//! impl ViewModel for CounterViewModel {
//!     fn init(&mut self) {
//!         // Initialize ViewModel
//!     }
//!
//!     fn dispose(&mut self) {
//!         // Cleanup resources
//!     }
//! }
//!
//! // In your egui update loop:
//! fn update(vm: &mut CounterViewModel, ctx: &egui::Context, ui: &mut egui::Ui) {
//!     ui.label(format!("Count: {}", vm.count.get()));
//!
//!     if ui.add(MaterialButton::filled("Increment")).clicked() {
//!         vm.increment();
//!         ctx.request_repaint(); // Trigger UI update
//!     }
//! }
//! ```

pub mod state;
pub mod viewmodel;

pub use state::{RefState, ValState};
pub use viewmodel::ViewModel;
