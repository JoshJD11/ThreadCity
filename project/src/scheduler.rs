use std::collections::VecDeque;
use crate::mythread::MyThread;


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
                t.ctx = t.ctx.context.resume(0);
            }
            if t.ctx.data != THE_NUMBER_OF_THE_BEAST {
                self.ready_queue.push_back(t);
            }
        }
        println!("Too ezz");
    }

}
