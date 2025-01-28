use crate::util::spawning::setup_spawn;
use std::future::Future;
use std::hash::Hash;
use tokio::task::JoinHandle;
use tokio::task_local;

task_local! {
    pub static TASK_ID: (u64, u64);
}

pub fn spawn<F: Future<Output=T> + Send + 'static, T: Send + 'static>(future: F) -> JoinHandle<T>
{
    tokio::spawn(setup_spawn(future))
}