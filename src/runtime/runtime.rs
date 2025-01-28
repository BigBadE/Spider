use std::future::Future;
use tokio::runtime::Runtime as TRuntime;
use crate::util::spawning::setup_spawn;

pub struct Runtime {
    pub(crate) runtime: TRuntime
}

impl Runtime {
    pub fn block_on<F: Future>(&self, future: F) -> F::Output
    {
        self.runtime.block_on(setup_spawn(future))
    }
}