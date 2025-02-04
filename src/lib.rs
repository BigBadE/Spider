
#[cfg(feature = "spider")]
use crate::spider::Spider;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::future::Future;
use itertools::Itertools;
use lazy_static::lazy_static;
#[cfg(not(feature = "spider"))]
pub use tokio::*;

#[cfg(feature = "spider")]
pub use tokio::{main, test};
#[cfg(feature = "spider")]
pub use tokio as bypass;

#[cfg(feature = "spider")]
pub mod sync;

#[cfg(feature = "spider")]
pub mod spawn;
#[cfg(feature = "spider")]
mod spider;
#[cfg(feature = "spider")]
mod util;

#[cfg(feature = "spider")]
pub mod runtime;

pub use tokio::pin;
use crate::spawn::TASK_ID;

#[cfg(feature = "spider")]
lazy_static! {
    static ref SPIDER: Mutex<Spider> = Mutex::new(Spider::default());
}

pub static ID: AtomicU64 = AtomicU64::new(1);

#[cfg(not(feature = "spider"))]
pub fn run_sim<F: Fn()>(_test: F) {
    panic!("Tried to run Spider sim without the spider feature. Enable that feature for this test.")
}

#[cfg(feature = "spider")]
pub async fn run_sim<C: Fn() -> F, F: Future + Send + 'static>(test: C) {
    test().await;
    println!("Found execution order {:?}", SPIDER.lock().unwrap().execution_order);
    let order = SPIDER.lock().unwrap().execution_order.clone();
    for possible in order.clone().into_iter().permutations(order.len()).filter(|possible| possible != &order) {
        ID.store(2, Ordering::SeqCst);
        println!("Trying execution order {:?}", possible);
        SPIDER.lock().unwrap().target_order = possible;
        test().await;
    }
}