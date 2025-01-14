#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test() {
        // shared counter wrapped in a Mutex and Arc for multiple ownership
        let counter = Arc::new(Mutex::new(0));

        // two threads that attempt to increment the counter based on a condition
        let thread1 = thread::spawn({
            let counter = Arc::clone(&counter);
            move || {
                check_then_increment(&counter, 1);
            }
        });

        let thread2 = thread::spawn({
            let counter = Arc::clone(&counter);
            move || {
                check_then_increment(&counter, 10);
            }
        });

        // Wait for threads to finish
        thread1.join().unwrap();
        thread2.join().unwrap();

        // Printing the final state of the counter
        let final_counter = counter.lock().unwrap();
        assert_eq!(*final_counter, 10);
    }

    fn check_then_increment(counter: &Arc<Mutex<i32>>, increment_value: i32) {
        // Acquire a lock to the counter
        let mut data = counter.lock().unwrap();

        // Check a condition
        if *data % 2 == 0 {
            // Perform an action based on the condition
            *data += increment_value;
            println!("Incremented Counter by {}: {}", increment_value, *data);
        } else {
            println!("Condition not met. Counter value: {}", *data);
        }
    }
}