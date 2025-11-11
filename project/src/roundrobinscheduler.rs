use std::collections::VecDeque;
use crate::mythread::MyThread;
use crate::scheduler::Scheduler;
use context::Transfer;

pub struct RoundRobinScheduler {
    ready_queue: VecDeque<Box<MyThread>>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
}

impl Scheduler for RoundRobinScheduler {

    fn enqueue_process(&mut self, thread: Box<MyThread>) {
        self.ready_queue.push_back(thread);
    }

    fn get_cant_processes(&self) -> usize {
        self.ready_queue.len()
    }

    fn run(&mut self, mut puppeteer_transfer: Transfer) -> Transfer {
        const THE_NUMBER_OF_THE_BEAST: usize = 666;

        while let Some(mut t) = self.ready_queue.pop_front() {

            unsafe {
                if let Some(ctx) = t.ctx.take() {
                    let new_ctx = ctx.context.resume(t.args.arguments);
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
            unsafe {
                puppeteer_transfer = puppeteer_transfer.context.resume(0);
            }
        }
        println!("RR done");
        return puppeteer_transfer;
    }

    fn pop_by_id(&mut self, thread_id: usize) -> Option<Box<MyThread>> {
        if let Some(pos) = self.ready_queue.iter().position(|x| x.id == thread_id) {
            let removed = self.ready_queue.remove(pos);
            println!("Removed");
            Some(removed?)
        } else {
            None
        }
    }
}
