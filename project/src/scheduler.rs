use std::collections::VecDeque;
use std::sync::{Mutex, Arc};
use crate::mythread::MyThread;
// #[macro_use]
// extern crate lazy_static;

pub trait Scheduler: Send + Sync {
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
        let mut queue = self.ready_queue.lock().unwrap();
        if let Some(pos) = queue.iter().position(|t| t.lock().unwrap().id == thread_id) {
            queue.remove(pos);
        }
    }
    
    fn yield_current(&self, current: Arc<Mutex<MyThread>>) {
        self.ready_queue.lock().unwrap().push_back(current);
    }
}

// // Scheduler global
// RoundRobinScheduler contains interior synchronization (Mutex) but some inner
// data of MyThread uses raw pointers that are not automatically Send/Sync.
// Assert Sync for the scheduler here when you know it's safe in your program.
unsafe impl Sync for RoundRobinScheduler {}
// We assert Send as well: the scheduler internally synchronizes access to thread
// ids and doesn't transfer ownership of non-Send data across threads. Marking
// this `Send` is a manual guarantee similar to the `Sync` assertion above.
unsafe impl Send for RoundRobinScheduler {}

lazy_static::lazy_static! {
    pub static ref ROUND_ROBIN_SCHEDULER: RoundRobinScheduler = RoundRobinScheduler::new();
    // Public scheduler alias so other modules can refer to the currently chosen
    // scheduler as a global `&'static dyn Scheduler` (keeps code modular).
    // You can later change this to point to a different scheduler instance.
    pub static ref SCHEDULER: &'static dyn Scheduler = &*ROUND_ROBIN_SCHEDULER as &'static dyn Scheduler;
}