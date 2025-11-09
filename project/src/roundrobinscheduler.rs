use std::collections::VecDeque;
use crate::mythread::MyThread;
use crate::scheduler::Scheduler;

pub struct RoundRobinScheduler {
    ready_queue: VecDeque<MyThread>,
}


impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

}

impl Scheduler for RoundRobinScheduler {

    fn enqueue_process(&mut self, thread: MyThread) {
        self.ready_queue.push_back(thread);
    }

    fn get_cant_processes(&self) -> usize {
        self.ready_queue.len()
    }

    fn run(&mut self) {
        const THE_NUMBER_OF_THE_BEAST: usize = 666;

        while let Some(mut t) = self.ready_queue.pop_front() {
            unsafe {
                if let Some(ctx) = t.ctx.take() { 
                    let new_ctx = ctx.context.resume(t.args);
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
