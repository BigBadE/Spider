use std::sync::Arc;
use std::time::Duration;
use rand::random;
use tokio::sync::Mutex;

pub use spider as tokio;

pub fn main() {
    // Unused
}

pub async fn race() {
    let mutex = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let mutex = mutex.clone();
        handles.push(tokio::spawn::spawn(async move {
            tokio::bypass::time::sleep(Duration::new(0, (random::<f64>() * 1000000000.0) as u32)).await;
            println!("Saw {} for {}", mutex.lock().await, random::<f64>() * 1000000000.0);
        }));
    }

    tokio::bypass::time::sleep(Duration::new(0, 1000000001)).await;
}

#[tokio::test]
pub async fn test() {
    tokio::run_sim(race()).await;
}