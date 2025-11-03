use std::collections::VecDeque;
use std::sync::{Mutex, Arc};
use crate::mythread::MyThread;

pub trait Scheduler {
    fn add_thread(&self, thread: Arc<Mutex<MyThread>>);
    fn get_next_thread(&self) -> Option<Arc<Mutex<MyThread>>>;
    fn remove_thread(&self, thread_id: usize);
    fn yield_current(&self, current: Arc<Mutex<MyThread>>);
}

pub struct RoundRobinScheduler {
    ready_queue: Mutex<VecDeque<Arc<Mutex<MyThread>>>>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl Scheduler for RoundRobinScheduler {
    fn add_thread(&self, thread: Arc<Mutex<MyThread>>) {
        self.ready_queue.lock().unwrap().push_back(thread);
    }
    
    fn get_next_thread(&self) -> Option<Arc<Mutex<MyThread>>> {
        self.ready_queue.lock().unwrap().pop_front()
    }
    
    fn remove_thread(&self, thread_id: usize) {
        // TODO : put id in thread
        // let mut queue = self.ready_queue.lock().unwrap();
        // if let Some(pos) = queue.iter().position(|t| t.lock().unwrap().id == thread_id) {
        //     queue.remove(pos);
        // }
    }
    
    fn yield_current(&self, current: Arc<Mutex<MyThread>>) {
        self.ready_queue.lock().unwrap().push_back(current);
    }
}

// Scheduler global
// lazy_static::lazy_static! {
//     pub static ref SCHEDULER: RoundRobinScheduler = RoundRobinScheduler::new();
// }