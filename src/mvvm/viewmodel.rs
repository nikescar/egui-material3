//! ViewModel trait and helpers for MVVM pattern

/// ViewModel trait for managing View state and logic
///
/// Implement this trait for your ViewModels to get lifecycle hooks
/// and standard behavior for MVVM pattern.
///
/// # Examples
///
/// ```rust
/// use egui_material3::mvvm::{ViewModel, ValState};
///
/// struct UserViewModel {
///     name: ValState<String>,
///     email: ValState<String>,
/// }
///
/// impl UserViewModel {
///     fn new() -> Self {
///         Self {
///             name: ValState::new(String::new()),
///             email: ValState::new(String::new()),
///         }
///     }
///
///     fn update_name(&mut self, new_name: String) {
///         self.name.set(new_name);
///     }
/// }
///
/// impl ViewModel for UserViewModel {
///     fn init(&mut self) {
///         // Load initial data, setup subscriptions, etc.
///         println!("ViewModel initialized");
///     }
///
///     fn dispose(&mut self) {
///         // Cleanup resources, cancel subscriptions, etc.
///         println!("ViewModel disposed");
///     }
/// }
/// ```
pub trait ViewModel {
    /// Called when the ViewModel is first created
    ///
    /// Use this to initialize state, load data, setup subscriptions, etc.
    fn init(&mut self) {
        // Default: no-op
    }

    /// Called when the ViewModel is being destroyed
    ///
    /// Use this to cleanup resources, cancel async tasks, close connections, etc.
    fn dispose(&mut self) {
        // Default: no-op
    }
}

/// Helper macro for creating ViewModels with automatic init/dispose
///
/// # Examples
///
/// ```rust
/// use egui_material3::mvvm::{ViewModel, ValState};
///
/// struct CounterViewModel {
///     count: ValState<i32>,
/// }
///
/// impl CounterViewModel {
///     fn new() -> Self {
///         let mut vm = Self {
///             count: ValState::new(0),
///         };
///         vm.init(); // Call init explicitly
///         vm
///     }
/// }
///
/// impl ViewModel for CounterViewModel {}
/// ```
#[macro_export]
macro_rules! viewmodel {
    ($name:ident { $($field:ident: $ty:ty = $init:expr),* $(,)? }) => {
        struct $name {
            $($field: $ty,)*
        }

        impl $name {
            fn new() -> Self {
                let mut vm = Self {
                    $($field: $init,)*
                };
                <Self as $crate::mvvm::ViewModel>::init(&mut vm);
                vm
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                <Self as $crate::mvvm::ViewModel>::dispose(self);
            }
        }
    };
}
