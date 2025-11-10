use crate::scheduler::Scheduler;
use crate::roundrobinscheduler::RoundRobinScheduler;
use crate::realtimescheduler::RealTimeScheduler;
use crate::ticketscheduler::TicketScheduler;
use crate::mythread::MyThread;
use crate::types::enums::SchedulingAlgorithm;
use crate::masterofpuppets::MasterOfPuppets;
use crate::mymutex::MyMutex;
use context::Transfer;

lazy_static! {
    pub static ref MASTER: MyMutex<MasterOfPuppets> = MyMutex::new(MasterOfPuppets::new());
}
// static mut MASTER: Option<MasterOfPuppets> = None;


// extern "C" fn puppeteer_init(mut t: Transfer) -> ! {
//     const THE_NUMBER_OF_THE_BEAST: usize = 666;
//     let master = unsafe { &mut *(t.data as *mut MasterOfPuppets) };
//     master.run();

//     unsafe {
//         t.context.resume(THE_NUMBER_OF_THE_BEAST);
//     }
//     unreachable!();
// }


pub struct MyPthreads {
    thread: Option<MyThread>,
    mutex: MyMutex,
    is_detached: bool,
} 

impl MyPthreads {

    pub fn my_thread_create(&mut self, func: extern "C" fn(Transfer) -> !, args: usize, sched_type: SchedulingAlgorithm) {
        self.is_detached = false;
        self.thread = MyThread::new(func, args, sched_type);
        // let mut need_to_run = false;

        unsafe {
      
            let mut master = MASTER.lock().unwrap();
            if master.is_none() {
                master = Some(MasterOfPuppets::new());
            }

            let master = MASTER.as_mut().unwrap();
            // if master.get_cant_processes() == 0 {
            //     need_to_run = true;
            // }
            let thread = self.thread.take().unwrap();
            master.enqueue_process(thread);


            // if need_to_run {

            //     let master_ptr = master as *mut MasterOfPuppets as usize;

            //     let puppeteer_thread = MyThread::new(
            //         puppeteer_init,
            //         master_ptr,
            //         SchedulingAlgorithm::RoundRobin
            //     );

            //     master.actual_thread = puppeteer_thread;
            //     master.actual_thread.ctx.context.resume(master_ptr);
            // }
        }
    }


    pub fn my_thread_yield(mut t: Transfer) { 
        println!("Yielding...");
        unsafe { t.context.resume(0); }
    }

    // pub fn my_thread_join(&mut self) { // Coming Soon! 
    //     if !self.is_detached {
  
    //     }
    //     else {
    //         println!("The thread is detached, so you are not able to join this thread.");
    //     }
    // }

    pub fn my_thread_detach(&mut self) {
        self.is_detached = true;
    }

    pub fn my_thread_chsched(&self, sched_type: SchedulingAlgorithm) { // Coming Soon!
        
    }

    pub fn my_thread_end(&mut self) {
        self.thread = None;
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

    pub fn my_mutex_destroy(&mut self) { 
        self.mutex.destroy();
    }
}