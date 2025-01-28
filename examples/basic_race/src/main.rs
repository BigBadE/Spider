use std::sync::Arc;
use std::time::Duration;
use spider::run_sim;
use spider::spawn::spawn;
use spider::sync::Mutex;

use spider::bypass as tokio;
use spider::bypass::time::sleep;
use rand::random;

pub fn main() {
    // Unused
}

pub async fn race() {
    let mutex = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    for i in 0..10 {
        let mutex = mutex.clone();
        handles.push(spawn(async move {
            sleep(Duration::new(0, (random::<f64>() * 1000000000.0) as u32)).await;
            println!("Saw {} for {}", mutex.lock().await, random::<f64>() * 1000000000.0);
        }));
    }

    sleep(Duration::new(0, 1000000001)).await;
}

#[spider::test]
pub async fn test() {
    run_sim(race()).await;
}