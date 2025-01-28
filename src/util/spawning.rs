use std::backtrace;
use std::future::Future;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::Ordering;
use crate::ID;
use crate::spawn::TASK_ID;

pub async fn setup_spawn<F: Future>(future: F) -> F::Output {
    let mut spawner_id = 0;
    let _ = TASK_ID.try_with(|(_, id)| spawner_id = *id);

    let mut hasher = DefaultHasher::new();
    backtrace::Backtrace::force_capture().to_string().hash(&mut hasher);
    let hash = hasher.finish();
    println!("Spawning from {spawner_id} at {hash}");
    async move {
        TASK_ID.scope((hash, ID.fetch_add(1, Ordering::Relaxed)), future).await
    }.await
}