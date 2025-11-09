use crate::scheduler::Scheduler;
use crate::roundrobinscheduler::RoundRobinScheduler;
use crate::realtimescheduler::RealTimeScheduler;
use crate::ticketscheduler::TicketScheduler;
use crate::types::enums::SchedulingAlgorithm;


static THE_NUMBER_OF_THE_BEAST: usize = 666;


extern "C" fn lottery_scheduler_init(mut t: Transfer) -> ! {
    let mut sched: Box<dyn Scheduler> = Box::new(TicketScheduler::new());
    sched.run();
    unsafe {
        t.context.resume(THE_NUMBER_OF_THE_BEAST);
    }
    unreachable!();
}

extern "C" fn round_robin_scheduler_init(mut t: Transfer) -> ! {
    let mut sched: Box<dyn Scheduler> = Box::new(RoundRobinScheduler::new());
    sched.run();
    unsafe {
        t.context.resume(THE_NUMBER_OF_THE_BEAST);
    }
    unreachable!();
}

extern "C" fn real_time_scheduler_init(mut t: Transfer) -> ! {
    let mut sched: Box<dyn Scheduler> = Box::new(RealTimeScheduler::new());
    sched.run();
    unsafe {
        t.context.resume(THE_NUMBER_OF_THE_BEAST);
    }
    unreachable!();
}

pub struct MasterOfPuppets { // It isn't finished yet!!
    ready_queue: Box<dyn Scheduler>,
}

impl MasterOfPuppets {
    pub fn new(state: SchedulingAlgorithm) -> Self {
        let scheduler: Box<dyn Scheduler> = Box::new(RoundRobinScheduler::new()); 
        Self {
            ready_queue: scheduler,
        }
    }

    pub fn run(&mut self) {
        let round_robin_scheduler_thread: MyThread = MyThread::new(round_robin_scheduler_init);
        let real_time_scheduler_thread: MyThread = MyThread::new(real_time_scheduler_init);
        let lottery_scheduler_thread: MyThread = MyThread::new(lottery_scheduler_init);
        self.ready_queue.enqueue_process(round_robin_scheduler_thread);
        self.ready_queue.enqueue_process(real_time_scheduler_thread);
        self.ready_queue.enqueue_process(lottery_scheduler_thread);

        self.ready_queue.run();
        
    }
}
