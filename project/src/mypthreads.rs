use crate::scheduler::Scheduler;
use crate::roundrobinscheduler::RoundRobinScheduler;
use crate::realtimescheduler::RealTimeScheduler;
use crate::ticketscheduler::TicketScheduler;
use crate::mymutex::MyMutex;
use crate::mythread::MyThread;
use crate::types::enums::SchedulingAlgorithm;
// use crate::timer;
use std::cell::UnsafeCell;


static mut MASTER: UnsafeCell<Option<MasterOfPuppets>> = UnsafeCell::new(None);
static MY_MUTEX: MyMutex = MyMutex::new();


pub struct MyPthreads {
    thread: MyThread,
    mutex: MyMutex,
    is_detached: bool,
} 

impl MyPthreads {

    pub fn my_thread_create(&mut self, func: extern "C" fn(Transfer) -> !, args: usize) { // Coming Soon!
        self.is_detached = false;
        self.thread = MyThread::new(func, args);
    }

    pub fn my_thread_yield(&mut self) { // Coming Soon!
        self.thread.ctx.context.resume(0);
    }

    pub fn my_thread_join(&mut self) { // Coming Soon!
        if !self.is_detached {

        }
        else {
            println!("The thread is detached, so you are not able to join this thread.");
        }
    }

    pub fn my_thread_detach(&mut self) {
        self.is_detached = true;
    }

    pub fn my_thread_chsched(&self, sched_type: SchedulingAlgorithm) { // Coming Soon!
        
    }

    pub fn my_thread_end(&mut self) {
        self.thread.ctx = None;
    }

    pub fn my_mutex_init(&mut self) {
        self.mutex = MyMutex::new();
    }

    pub fn my_mutex_trylock(&mut self) {
        self.mutex.try_lock();
    }

    pub fn my_mutex_lock(&mut self) {
        self.mutex.lock();
    }

    pub fn my_mutex_unlock(&mut self) {
        self.mutex.unlock();
    }

    pub fn my_mutex_destroy(&mut self) { // We have to talk about what should we do with this method.
        self.mutex.destroy();
    }
}