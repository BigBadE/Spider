use std::sync::atomic::Ordering;
#[cfg(not(feature = "spider"))]
pub use tokio::sync::{Mutex, MutexGuard};

use tokio::sync::{Mutex as TMutex, MutexGuard as TMutexGuard};
use crate::{ID, SPIDER};
use crate::spawn::TASK_ID;

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
        println!("Lock on id {:?} with mutex {}", TASK_ID.get(), self.id);
        SPIDER.lock().unwrap().dependencies.entry(self.id).or_default().insert(TASK_ID.get().0);
        self.inner.lock().await
    }
}