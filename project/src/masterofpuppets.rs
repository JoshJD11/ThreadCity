use crate::scheduler::Scheduler;
use crate::roundrobinscheduler::RoundRobinScheduler;
use crate::realtimescheduler::RealTimeScheduler;
use crate::ticketscheduler::TicketScheduler;
use crate::types::enums::SchedulingAlgorithm;


static THE_NUMBER_OF_THE_BEAST: usize = 666;


extern "C" fn scheduler_init(mut t: Transfer) -> ! {

    let sched = unsafe { &mut *(t.data as *mut dyn Scheduler) };
    sched.run();  

    unsafe {
        t.context.resume(THE_NUMBER_OF_THE_BEAST);
    }
    unreachable!();
}


pub struct MasterOfPuppets { 
    ready_queue: Box<dyn Scheduler>,
    round_robin_scheduler: Box<dyn Scheduler>,
    ticket_scheduler: Box<dyn Scheduler>,
    real_time_scheduler: Box<dyn Scheduler>,
}

impl MasterOfPuppets {

    pub fn new() -> Self {
        let puppeteer: Box<dyn Scheduler> = Box::new(RoundRobinScheduler::new()); 
        let rr_sched: Box<dyn Scheduler> = Box::new(RoundRobinScheduler::new());
        let rt_sched: Box<dyn Scheduler> = Box::new(RealTimeScheduler::new());
        let t_sched: Box<dyn Scheduler> = Box::new(TicketScheduler::new());
        Self {
            ready_queue: puppeteer,
            round_robin_scheduler: rr_sched,
            real_time_scheduler: rt_sched,
            ticket_scheduler: t_sched,
        }
    }

    pub fn enqueue_round_robin(&mut self, thread: MyThread) {
        self.round_robin_scheduler.enqueue_process(thread);
        if self.round_robin_scheduler.get_cant_processes() == 1 {
            let ptr = &mut *self.round_robin_scheduler as *mut dyn Scheduler;
            let thread = MyThread::new(scheduler_init, ptr as usize);
            self.ready_queue.enqueue_process(thread);
        }
    }  

    pub fn enqueue_real_time(&mut self, thread: MyThread) {
        self.real_time_scheduler.enqueue_process(thread);
        if self.real_time_scheduler.get_cant_processes() == 1 {
            let ptr = &mut *self.real_time_scheduler as *mut dyn Scheduler;
            let thread = MyThread::new(scheduler_init, ptr as usize);
            self.ready_queue.enqueue_process(thread);
        }
    }

    pub fn enqueue_lottery(&mut self, thread: MyThread) {
        self.ticket_scheduler.enqueue_process(thread);
        if self.ticket_scheduler.get_cant_processes() == 1 {
            let ptr = &mut *self.ticket_scheduler as *mut dyn Scheduler;
            let thread = MyThread::new(scheduler_init, ptr as usize); 
            self.ready_queue.enqueue_process(thread);
        }
    }

}

impl Scheduler for MasterOfPuppets {

    fn enqueue_process(&mut self, thread: MyThread) {
        match thread.sched_type {
            SchedulingAlgorithm::Lottery => self.enqueue_lottery(thread),
            SchedulingAlgorithm::RealTime => self.enqueue_real_time(thread),
            SchedulingAlgorithm::RoundRobin => self.enqueue_round_robin(thread),
        }
    }

    fn get_cant_processes(&self) -> usize {
        self.ready_queue.get_cant_processes()
    }

    fn run(&mut self) {
        self.ready_queue.run();
    }
}
