use crate::util::spawning::setup_spawn;
use std::future::Future;
use std::hash::Hash;
use real_tokio::task::JoinHandle;
use real_tokio::task_local;

task_local! {
    pub static TASK_ID: (u64, u64);
}

pub fn spawn<F: Future<Output=T> + Send + 'static, T: Send + 'static>(future: F) -> JoinHandle<T>
{
    real_tokio::spawn(setup_spawn(future))
}