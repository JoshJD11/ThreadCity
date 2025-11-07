use std::collections::VecDeque;
use crate::mythread::MyThread;

use std::sync::atomic::{AtomicBool, Ordering};

static PREEMPT: AtomicBool = AtomicBool::new(false);

pub fn preempt_flag() {
    PREEMPT.store(true, Ordering::SeqCst);
}

pub fn should_preempt() -> bool {
    PREEMPT.swap(false, Ordering::SeqCst)
}

pub struct RoundRobinScheduler {
    ready_queue: VecDeque<MyThread>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    pub fn enqueue_process(&mut self, thread: MyThread) {
        self.ready_queue.push_back(thread);
    }

    pub fn run(&mut self) {
        const THE_NUMBER_OF_THE_BEAST: usize = 666;

        while let Some(mut t) = self.ready_queue.pop_front() {
            unsafe {
                if let Some(ctx) = t.ctx.take() { 
                    let new_ctx = ctx.context.resume(0);
                    t.ctx = Some(new_ctx);
                }
            }
            
            if let Some(ctx) = &t.ctx {
                if ctx.data != THE_NUMBER_OF_THE_BEAST {
                    self.ready_queue.push_back(t);
                }
            } else {
                self.ready_queue.push_back(t);
            }
        }
        println!("Too ezz");
    }
}
