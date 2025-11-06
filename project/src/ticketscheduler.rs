use crate::mythread::MyThread;
use rand::Rng;

pub struct TicketScheduler {
    ready_queue: Vec<MyThread>,
    tickets: Vec<usize>,
    total_tickets: usize,
    current_index: usize,
} 

impl TicketScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
            tickets: Vec::new(),
            total_tickets: 0,
            current_index: 0,
        }
    }

    pub fn enqueue_process(&mut self, thread: MyThread, tickets: usize) {
        self.tickets.push(tickets);
        self.total_tickets += tickets;
        self.ready_queue.push(thread);
    }

    pub fn get_winner(&mut self) -> Option<&mut MyThread> {
        if self.ready_queue.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let win_condition = rng.gen_range(1..=self.total_tickets);
        let mut counter = 0;
        for i in 0..self.tickets.len() {
            counter += self.tickets[i];
            if counter >= win_condition {
                self.current_index = i;
                return self.ready_queue.get_mut(i);
            }
        }
        None
    }

    pub fn run(&mut self) {

        const THE_NUMBER_OF_THE_BEAST: usize = 666;
        while let Some(t) = self.get_winner() {
            unsafe {
                if let Some(ctx) = t.ctx.take() { 
                    let new_ctx = ctx.context.resume(0);
                    t.ctx = Some(new_ctx);
                }
            }
            if let Some(ctx) = &t.ctx {
                if ctx.data == THE_NUMBER_OF_THE_BEAST {
                    self.ready_queue.remove(self.current_index);
                    self.total_tickets -= self.tickets.remove(self.current_index);
                }
            }
    }
    println!("Too ezz");
    }

}
