use crate::mythread::MyThread;
use context::Transfer;

pub trait Scheduler {
    fn enqueue_process(&mut self, thread: Box<MyThread>);
    fn get_cant_processes(&self) -> usize;
    fn run(&mut self, puppeteer_transfer: Transfer) -> Transfer;
    fn pop_by_id(&mut self, id: usize) -> Option<Box<MyThread>>;
}

