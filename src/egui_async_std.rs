//! Simple async binding for egui using async-std runtime
//!
//! Provides async state management for egui UI, replacing egui-async with async-std-based implementation.

use std::future::Future;
use async_std::sync::{Arc, Mutex};

/// State of an async operation
#[derive(Clone, Debug)]
pub enum StateWithData<T, E> {
    /// Operation hasn't started or has been reset
    Idle,
    /// Operation is in progress
    Pending,
    /// Operation completed successfully
    Finished(T),
    /// Operation failed with an error
    Failed(E),
}

impl<T, E> Default for StateWithData<T, E> {
    fn default() -> Self {
        Self::Idle
    }
}

/// Bind an async future to UI state, automatically updating when the future completes
pub struct Bind<T: Clone, E: Clone> {
    state: Arc<Mutex<StateWithData<T, E>>>,
    allow_multiple: bool,
}

impl<T: Clone + Send + 'static, E: Clone + Send + 'static> Bind<T, E> {
    /// Create a new bind
    pub fn new(allow_multiple: bool) -> Self {
        Self {
            state: Arc::new(Mutex::new(StateWithData::Idle)),
            allow_multiple,
        }
    }

    /// Start executing an async operation
    pub fn refresh<F>(&mut self, future: F)
    where
        F: Future<Output = Result<T, E>> + Send + 'static,
    {
        let state = Arc::clone(&self.state);

        // Set to pending state
        if let Some(mut s) = state.try_lock() {
            // If not allowing multiple and already pending, skip
            if !self.allow_multiple && matches!(*s, StateWithData::Pending) {
                return;
            }
            *s = StateWithData::Pending;
        }

        // Spawn the async task using async-std
        async_std::task::spawn(async move {
            let result = future.await;
            let mut s = state.lock().await;
            *s = match result {
                Ok(data) => StateWithData::Finished(data),
                Err(err) => StateWithData::Failed(err),
            };
        });
    }

    /// Get the current state (blocking)
    pub fn state(&self) -> StateWithData<T, E> {
        // Use try_lock to avoid blocking in UI context
        if let Some(s) = self.state.try_lock() {
            s.clone()
        } else {
            StateWithData::Pending
        }
    }

    /// Reset to idle state
    pub fn reset(&mut self) {
        if let Some(mut s) = self.state.try_lock() {
            *s = StateWithData::Idle;
        }
    }
}

/// Initialize async-std executor for egui integration
/// async-std uses a global executor that starts automatically,
/// so this function is a no-op but kept for API compatibility
pub fn init_executor() {
    // async-std's global executor starts automatically when tasks are spawned
    // No explicit initialization needed
}
