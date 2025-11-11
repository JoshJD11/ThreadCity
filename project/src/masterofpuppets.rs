use crate::scheduler::Scheduler;
use crate::roundrobinscheduler::RoundRobinScheduler;
use crate::realtimescheduler::RealTimeScheduler;
use crate::ticketscheduler::TicketScheduler;
use crate::types::enums::SchedulingAlgorithm;
use crate::mythread::MyThread;
use crate::mythread::ThreadArgs;
use context::Transfer;

use crate::masterscheduler::MasterScheduler; // temporal

static THE_NUMBER_OF_THE_BEAST: usize = 666;


extern "C" fn rr_scheduler_init(mut t: Transfer) -> ! {

    let args = unsafe { &mut *(t.data as *mut ThreadArgs) };
    let sched = unsafe { &mut *(args.arguments as *mut RoundRobinScheduler) } as &mut dyn Scheduler;
    t = sched.run(t);
    unsafe { t.context.resume(THE_NUMBER_OF_THE_BEAST); }
    unreachable!();
}

extern "C" fn rt_scheduler_init(mut t: Transfer) -> ! {
    let args = unsafe { &mut *(t.data as *mut ThreadArgs) };
    let sched = unsafe { &mut *(args.arguments as *mut RealTimeScheduler) } as &mut dyn Scheduler;
    t = sched.run(t);
    unsafe { t.context.resume(THE_NUMBER_OF_THE_BEAST); }
    unreachable!();
}

extern "C" fn ticket_scheduler_init(mut t: Transfer) -> ! {
    let args = unsafe { &mut *(t.data as *mut ThreadArgs) };
    let sched = unsafe { &mut *(args.arguments as *mut TicketScheduler) } as &mut dyn Scheduler;
    t = sched.run(t);
    unsafe { t.context.resume(THE_NUMBER_OF_THE_BEAST); }
    unreachable!();
}


pub struct MasterOfPuppets {
    pub ready_queue: Box<MasterScheduler>,
    round_robin_scheduler: Box<RoundRobinScheduler>,
    ticket_scheduler: Box<TicketScheduler>,
    real_time_scheduler: Box<RealTimeScheduler>,
}

impl MasterOfPuppets {

    pub fn new() -> Self {
        let puppeteer: Box<MasterScheduler> = Box::new(MasterScheduler::new()); 
        let rr_sched: Box<RoundRobinScheduler> = Box::new(RoundRobinScheduler::new());
        let rt_sched: Box<RealTimeScheduler> = Box::new(RealTimeScheduler::new());
        let t_sched: Box<TicketScheduler> = Box::new(TicketScheduler::new());
        Self {
            ready_queue: puppeteer,
            round_robin_scheduler: rr_sched,
            real_time_scheduler: rt_sched,
            ticket_scheduler: t_sched,
        }
    }

    pub fn enqueue_round_robin(&mut self, thread: Box<MyThread>) {
        self.round_robin_scheduler.enqueue_process(thread);
        if self.round_robin_scheduler.get_cant_processes() == 1 {
            let ptr = &mut *self.round_robin_scheduler as *mut RoundRobinScheduler;
            let thread = Box::new(MyThread::new(rr_scheduler_init, ptr as usize, SchedulingAlgorithm::RoundRobin));
            self.ready_queue.enqueue_process(thread);
        }
    }  

    pub fn enqueue_real_time(&mut self, thread: Box<MyThread>) {
        self.real_time_scheduler.enqueue_process(thread);
        if self.real_time_scheduler.get_cant_processes() == 1 {
            let ptr = &mut *self.real_time_scheduler as *mut RealTimeScheduler;
            let thread = Box::new(MyThread::new(rt_scheduler_init, ptr as usize, SchedulingAlgorithm::RealTime));
            self.ready_queue.enqueue_process(thread);
        }
    }

    pub fn enqueue_lottery(&mut self, thread: Box<MyThread>) {
        self.ticket_scheduler.enqueue_process(thread);
        if self.ticket_scheduler.get_cant_processes() == 1 {
            let ptr = &mut *self.ticket_scheduler as *mut TicketScheduler;
            let thread = Box::new(MyThread::new(ticket_scheduler_init, ptr as usize, SchedulingAlgorithm::Lottery));
            self.ready_queue.enqueue_process(thread);
        }
    }

    pub fn enqueue_process(&mut self, thread: Box<MyThread>) {
        match thread.sched_type {
            SchedulingAlgorithm::Lottery => self.enqueue_lottery(thread),
            SchedulingAlgorithm::RealTime => self.enqueue_real_time(thread),
            SchedulingAlgorithm::RoundRobin => self.enqueue_round_robin(thread),
        }
    }

    pub fn get_cant_processes(&self) -> usize {
        self.ready_queue.get_cant_processes()
    }

    pub fn run(&mut self) {
        self.ready_queue.run();
    }

    pub fn switch_thread_sched(&mut self, id: usize, source_sched_type: SchedulingAlgorithm, target_sched_type: SchedulingAlgorithm) {
        let thread_opt: Option<Box<MyThread>> = match source_sched_type {
            SchedulingAlgorithm::Lottery => self.ticket_scheduler.pop_by_id(id),
            SchedulingAlgorithm::RealTime => self.real_time_scheduler.pop_by_id(id),
            SchedulingAlgorithm::RoundRobin => self.round_robin_scheduler.pop_by_id(id),
        };

        if let Some(mut thread) = thread_opt {
            thread.sched_type = target_sched_type;
            self.enqueue_process(thread);
        } else {
            println!("Thread with id {} not found in {:?} scheduler", id, source_sched_type);
        }
    }

    pub fn set_thread_non_preemtive(&mut self, id: usize, sched_type: SchedulingAlgorithm) {
        let thread_opt: Option<Box<MyThread>> = match sched_type {
            SchedulingAlgorithm::Lottery => self.ticket_scheduler.pop_by_id(id),
            SchedulingAlgorithm::RealTime => self.real_time_scheduler.pop_by_id(id),
            SchedulingAlgorithm::RoundRobin => self.round_robin_scheduler.pop_by_id(id),
        };

        if let Some(mut thread) = thread_opt {
            thread.set_non_preemptive();
            self.enqueue_process(thread);
        } else {
            println!("Thread with id {} not found in {:?} scheduler", id, sched_type);
        }
    }


}

