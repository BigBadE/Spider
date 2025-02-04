use crate::spawn::TASK_ID;
use crate::SPIDER;
use std::collections::{HashMap, HashSet};
use std::thread;
use tokio::sync::broadcast::{channel, Receiver, Sender};

pub(crate) struct Spider {
    // Maps a call location
    pub dependencies: HashMap<u64, HashSet<u64>>,
    pub execution_order: Vec<u64>,
    pub target_order: Vec<u64>,
    pub running: HashSet<u64>,
    pub sender: Sender<u64>,
    pub waiting: HashMap<u64, Sender<()>>,
}

impl Default for Spider {
    fn default() -> Self {
        let (sender, _) = channel(128);
        Self {
            dependencies: Default::default(),
            execution_order: Default::default(),
            target_order: Default::default(),
            running: Default::default(),
            sender,
            waiting: Default::default(),
        }
    }
}

impl Spider {
    pub async fn wait_for_resume() {
        let (call_location, task_id) = TASK_ID.get();
        let mut future = {
            let mut lock = SPIDER.lock().unwrap();
            lock.running.remove(&task_id);

            if lock.running.is_empty() {
                if lock.target_order.is_empty() {
                    lock.execution_order.push(task_id);
                    return;
                } else {
                    let target = lock.target_order[0];
                    lock.running.insert(target);
                    if target == task_id {
                        lock.target_order.remove(0);
                        return;
                    }
                    lock.sender.send(target);
                }
            }

            let target = lock.target_order[0];
            lock.sender.send(target);

            lock.sender.subscribe()
        };

        while let Ok(found) = future.recv().await {
            if found == task_id {
                SPIDER.lock().unwrap().target_order.remove(0);
                return;
            }
        }
    }

    pub fn task_end() {
        let (call_location, task_id) = TASK_ID.get();
        let mut lock = SPIDER.lock().unwrap();
        lock.running.remove(&task_id);
        if lock.target_order.len() != 0 {
            let target = lock.target_order[0];
            lock.sender.send(target);
        }
    }
}
