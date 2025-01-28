use std::backtrace;
use crate::ID;
use std::future::Future;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::Ordering;
#[cfg(not(feature = "spider"))]
pub use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::task_local;

task_local! {
    pub static TASK_ID: (u64, u64);
}

#[cfg(feature = "spider")]
pub fn spawn<F: Future<Output=T> + Send + 'static, T: Send + 'static>(future: F) -> JoinHandle<T>
{
    let mut hasher = DefaultHasher::new();
    backtrace::Backtrace::force_capture().to_string().hash(&mut hasher);
    let hash = hasher.finish();
    tokio::spawn(async move {
        TASK_ID.scope((hash, ID.fetch_add(1, Ordering::Relaxed)), future).await
    })
}