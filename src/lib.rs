use crate::spider::Spider;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;
use std::future::Future;
use lazy_static::lazy_static;
pub use tokio::macros;
pub use tokio::{main, test};

pub use tokio as bypass;

pub mod sync;
pub mod spawn;
mod spider;
mod util;

lazy_static! {
    static ref SPIDER: Mutex<Spider> = Mutex::new(Spider::default());
}

pub static ID: AtomicU64 = AtomicU64::new(0);

#[cfg(not(feature = "spider"))]
pub fn run_sim<F: Fn()>(_test: F) {
    panic!("Tried to run Spider sim without the spider feature. Enable that feature for this test.")
}

#[cfg(feature = "spider")]
pub async fn run_sim<F: Future + Send + 'static>(test: F) {
    test.await;
    println!("Found dependencies {:?}", SPIDER.lock().unwrap().dependencies);
}