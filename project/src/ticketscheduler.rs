use crate::mythread::MyThread;
use crate::scheduler::Scheduler;
use context::Transfer;
use rand::Rng;

pub struct TicketScheduler {
    ready_queue: Vec<Box<MyThread>>,
    total_tickets: usize,
}

impl TicketScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
            total_tickets: 0,
        }
    }

    fn get_winner_index(&mut self) -> Option<usize> {
        if self.ready_queue.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let target = rng.gen_range(1..=self.total_tickets);
        let mut sum = 0;

        for i in 0..self.ready_queue.len() {
            sum += self.ready_queue[i].get_tickets();
            if sum >= target {
                return Some(i);
            }
        }
        None
    }
}

impl Scheduler for TicketScheduler {

    fn enqueue_process(&mut self, mut thread: Box<MyThread>) {
        self.total_tickets += thread.get_tickets();
        self.ready_queue.push(thread);
    }

    fn get_cant_processes(&self) -> usize {
        self.ready_queue.len()
    }

    fn run(&mut self, mut puppeteer_transfer: Transfer) -> Transfer {
        const THE_NUMBER_OF_THE_BEAST: usize = 666;



        while let Some(idx) = self.get_winner_index() {

            let t = &mut self.ready_queue[idx];

            unsafe {
                if let Some(ctx) = t.ctx.take() {
                    let new_ctx = ctx.context.resume(t.args.arguments);
                    t.ctx = Some(new_ctx);
                }
            }

            if let Some(ctx) = &t.ctx {
                if ctx.data == THE_NUMBER_OF_THE_BEAST {
                    self.total_tickets -= t.get_tickets();
                    self.ready_queue.remove(idx);
                }
            }
            unsafe {
                puppeteer_transfer = puppeteer_transfer.context.resume(0);
            }
        }

        println!("Tickets done");
        return puppeteer_transfer;
    }

    fn pop_by_id(&mut self, thread_id: usize) -> Option<Box<MyThread>> {
        if let Some(pos) = self.ready_queue.iter().position(|x| x.id == thread_id) {
            let removed = self.ready_queue.remove(pos);
            println!("Removed");
            Some(removed)
        } else {
            None
        }
    }
}

