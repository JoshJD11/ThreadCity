use crate::mythread::MyThread;

pub trait Scheduler {
    fn enqueue_process(&mut self, thread: MyThread);
    fn run(&mut self);
    fn get_cant_processes(&self) -> usize; 
}
