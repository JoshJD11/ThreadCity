use crate::mythread::MyThread;
use crate::scheduler::Scheduler;
use std::collections::BinaryHeap;

pub struct RealTimeScheduler {
    ready_queue: BinaryHeap<MyThread>,
}

impl RealTimeScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: BinaryHeap::new(),
        }
    }
}

impl Scheduler for RealTimeScheduler {
    fn enqueue_process(&mut self, thread: MyThread) {
        self.ready_queue.push(thread);
    }

    fn run(&mut self) {
        const THE_NUMBER_OF_THE_BEAST: usize = 666;

        while let Some(mut t) = self.ready_queue.pop() {
            unsafe {
                if let Some(ctx) = t.ctx.take() { 
                    let new_ctx = ctx.context.resume(t.args);
                    t.ctx = Some(new_ctx);
                }
            }
            
            if let Some(ctx) = &t.ctx {
                if ctx.data != THE_NUMBER_OF_THE_BEAST {
                    self.ready_queue.push(t);
                }
            } else {
                self.ready_queue.push(t);
            }
        }
        println!("Too ezz");
    }
}
