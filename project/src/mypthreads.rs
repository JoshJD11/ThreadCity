use crate::scheduler::Scheduler;
use crate::roundrobinscheduler::RoundRobinScheduler;
use crate::realtimescheduler::RealTimeScheduler;
use crate::ticketscheduler::TicketScheduler;
use crate::mythread::MyThread;
use crate::types::enums::SchedulingAlgorithm;
use crate::masterofpuppets::MasterOfPuppets;
use std::sync::Mutex;
use crate::mymutex::MyMutex;
use context::Transfer;
use lazy_static::lazy_static;
use std::sync::OnceLock;


static MASTER: OnceLock<Mutex<MasterOfPuppets>> = OnceLock::new();

pub fn runMaster() {
    if let Some(master_mutex) = MASTER.get() {
        let mut master = master_mutex.lock().unwrap();
        master.run(); // sale que recibe un parámetro
    }
}

pub struct MyPthreads {
    mutex: MyMutex,
    is_detached: bool,
} 

impl MyPthreads {

    pub fn new() -> Self {
        Self {
            mutex: MyMutex::new(),
            is_detached: false,
        }
    }

    pub fn my_thread_create(&mut self, func: extern "C" fn(Transfer) -> !, args: usize, tickets: usize, deadline: usize, sched_type: SchedulingAlgorithm) {
        let mut thread = Box::new(MyThread::new(func, args, sched_type));
        thread.set_deadline(deadline);
        thread.add_tickets(tickets);

        MASTER.get_or_init(|| Mutex::new(MasterOfPuppets::new()));

        let master = MASTER.get().unwrap();
        master.lock().unwrap().enqueue_process(thread);
    }

    pub fn my_thread_yield(t: Transfer, args: usize) -> Transfer { 
        println!("Yielding...");
        let back = unsafe { t.context.resume(args) };
        return back;
    }

    pub fn my_thread_detach(&mut self) {
        self.is_detached = true;
    }

    // pub fn my_thread_chsched(&self, sched_type: SchedulingAlgorithm) { 

    // }

    // pub fn my_thread_end(&mut self) {
        
    // }

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

    pub fn my_mutex_destroy(&mut self) { 
        self.mutex.destroy();
    }
}
