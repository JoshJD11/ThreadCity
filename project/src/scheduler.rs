use crate::mythread::MyThread;

pub trait Scheduler {
    fn enqueue_process(&mut self, thread: Box<MyThread>);
    fn get_cant_processes(&self) -> usize;
    fn run(&mut self);
}

