//! Reactive state primitives for MVVM pattern
//!
//! Provides reactive state containers that notify observers when values change.

use std::sync::{Arc, Mutex};

/// Reactive value state for owned data
///
/// `ValState<T>` holds an owned value and provides reactive updates.
/// Use this for simple types that can be cloned efficiently.
///
/// # Examples
///
/// ```rust
/// use egui_material3::mvvm::ValState;
///
/// let mut state = ValState::new(42);
/// assert_eq!(state.get(), 42);
///
/// state.set(100);
/// assert_eq!(state.get(), 100);
///
/// state.update(|v| *v += 10);
/// assert_eq!(state.get(), 110);
/// ```
#[derive(Clone, Debug)]
pub struct ValState<T: Clone> {
    value: T,
}

impl<T: Clone> ValState<T> {
    /// Create a new ValState with an initial value
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Get the current value (cloned)
    pub fn get(&self) -> T {
        self.value.clone()
    }

    /// Set a new value
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Update the value using a closure
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_material3::mvvm::ValState;
    /// let mut state = ValState::new(5);
    /// state.update(|v| *v *= 2);
    /// assert_eq!(state.get(), 10);
    /// ```
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.value);
    }

    /// Map the current value to a new type
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_material3::mvvm::ValState;
    /// let state = ValState::new(42);
    /// let doubled = state.map(|v| v * 2);
    /// assert_eq!(doubled, 84);
    /// ```
    pub fn map<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        f(&self.value)
    }
}

impl<T: Clone + Default> Default for ValState<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// Reactive reference state for shared data
///
/// `RefState<T>` uses `Arc<Mutex<T>>` for thread-safe shared access.
/// Use this for larger types or when multiple ViewModels need to share state.
///
/// # Examples
///
/// ```rust
/// use egui_material3::mvvm::RefState;
///
/// #[derive(Clone, Debug)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let state = RefState::new(User {
///     name: "Alice".to_string(),
///     age: 30,
/// });
///
/// // Read the value
/// let name = smol::block_on(async {
///     state.with(|user| user.name.clone()).await
/// });
/// assert_eq!(name, "Alice");
///
/// // Update the value
/// smol::block_on(async {
///     state.update(|user| user.age += 1).await;
/// });
/// ```
#[derive(Clone, Debug)]
pub struct RefState<T> {
    value: Arc<Mutex<T>>,
}

impl<T> RefState<T> {
    /// Create a new RefState with an initial value
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }

    /// Read the value using a closure
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_material3::mvvm::RefState;
    /// let state = RefState::new(vec![1, 2, 3]);
    /// let len = state.with(|v| v.len());
    /// assert_eq!(len, 3);
    /// ```
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.value.lock().unwrap();
        f(&guard)
    }

    /// Update the value using a closure
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_material3::mvvm::RefState;
    /// let state = RefState::new(vec![1, 2, 3]);
    /// state.update(|v| v.push(4));
    /// let len = state.with(|v| v.len());
    /// assert_eq!(len, 4);
    /// ```
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut guard = self.value.lock().unwrap();
        f(&mut guard);
    }

    /// Try to read the value without blocking
    ///
    /// Returns `None` if the lock is currently held.
    pub fn try_with<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.value.try_lock().ok().map(|guard| f(&guard))
    }

    /// Try to update the value without blocking
    ///
    /// Returns `false` if the lock is currently held.
    pub fn try_update<F>(&self, f: F) -> bool
    where
        F: FnOnce(&mut T),
    {
        if let Ok(mut guard) = self.value.try_lock() {
            f(&mut guard);
            true
        } else {
            false
        }
    }
}

impl<T: Clone> RefState<T> {
    /// Get a clone of the current value
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_material3::mvvm::RefState;
    /// let state = RefState::new(42);
    /// let value = state.get();
    /// assert_eq!(value, 42);
    /// ```
    pub fn get(&self) -> T {
        self.with(|v| v.clone())
    }

    /// Set a new value
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use egui_material3::mvvm::RefState;
    /// let state = RefState::new(42);
    /// state.set(100);
    /// assert_eq!(state.get(), 100);
    /// ```
    pub fn set(&self, value: T) {
        self.update(|v| *v = value);
    }
}

impl<T: Default> Default for RefState<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_val_state_basic() {
        let mut state = ValState::new(10);
        assert_eq!(state.get(), 10);

        state.set(20);
        assert_eq!(state.get(), 20);

        state.update(|v| *v += 5);
        assert_eq!(state.get(), 25);
    }

    #[test]
    fn test_val_state_map() {
        let state = ValState::new(5);
        let doubled = state.map(|v| v * 2);
        assert_eq!(doubled, 10);
    }

    #[test]
    fn test_ref_state_basic() {
        let state = RefState::new(10);
        assert_eq!(state.get(), 10);

        state.set(20);
        assert_eq!(state.get(), 20);

        state.update(|v| *v += 5);
        assert_eq!(state.get(), 25);
    }

    #[test]
    fn test_ref_state_try_operations() {
        let state = RefState::new(10);

        // try_with should succeed when not locked
        let value = state.try_with(|v| *v);
        assert_eq!(value, Some(10));

        // try_update should succeed when not locked
        let success = state.try_update(|v| *v = 20);
        assert!(success);
        assert_eq!(state.try_with(|v| *v), Some(20));
    }
}
