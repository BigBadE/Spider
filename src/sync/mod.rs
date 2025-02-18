use std::sync::atomic::Ordering;
#[cfg(not(feature = "spider"))]
pub use real_tokio::sync::{Mutex, MutexGuard};

use crate::spider::Spider;
use crate::ID;
use real_tokio::sync::{Mutex as TMutex, MutexGuard as TMutexGuard};

#[cfg(feature = "spider")]
pub struct Mutex<T> {
    inner: TMutex<T>,
    id: u64
}

#[cfg(feature = "spider")]
impl<T> Mutex<T> {
    pub fn new(inner: T) -> Self where T: Sized {
        Self {
            inner: TMutex::new(inner),
            id: ID.fetch_add(1, Ordering::Relaxed)
        }
    }

    pub async fn lock(&self) -> TMutexGuard<'_, T> {
        Spider::wait_for_resume().await;
        self.inner.lock().await
    }
}