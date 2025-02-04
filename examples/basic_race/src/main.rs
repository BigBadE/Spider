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

    {
        let mutex = mutex.clone();
        handles.push(tokio::spawn::spawn(async move {
            println!("Saw {} for 1", mutex.lock().await);
        }));
    }
    {
        let mutex = mutex.clone();
        handles.push(tokio::spawn::spawn(async move {
            println!("Saw {} for 2", mutex.lock().await);
        }));
    }
    {
        let mutex = mutex.clone();
        handles.push(tokio::spawn::spawn(async move {
            println!("Saw {} for 3", mutex.lock().await);
        }));
    }

    tokio::bypass::time::sleep(Duration::new(0, 1000001)).await;
}

#[tokio::test]
pub async fn test() {
    tokio::run_sim(|| race()).await;
}