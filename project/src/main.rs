mod mythread;
mod ticketscheduler;
mod scheduler;
mod realtimescheduler;
mod roundrobinscheduler;
mod mypthreads;
mod masterofpuppets;
mod timer;
mod types;
mod mymutex;

use mythread::MyThread;
use scheduler::Scheduler;
use mypthreads::MyPthreads;
use context::Transfer;
use crate::types::SchedulingAlgorithm;


struct Test {
    message: String,
}

fn main() {
    const THE_NUMBER_OF_THE_BEAST: usize = 666;
    timer::start_timer();


    extern "C" fn context_function1(mut t: Transfer) -> ! {
        for i in 0usize..20 {
            println!("thread 1 Currently at:  {}", i);
            if timer::should_preempt() {
                println!("thread 1 Preempting at {}", i);
                t = unsafe { t.context.resume(0) };
            }
        }
        unsafe {
            t.context.resume(THE_NUMBER_OF_THE_BEAST);
        }
        unreachable!();
    }
    extern "C" fn context_function2(mut t: Transfer) -> ! {
        for i in 0usize..20 {
            println!("thread 2 Currently at:  {}", i);
            if timer::should_preempt() {
                println!("thread 2 Preempting at {}", i);
                t = unsafe { t.context.resume(0) };
            }
        }
        unsafe {
            t.context.resume(THE_NUMBER_OF_THE_BEAST);
        }
        unreachable!();
    }

    extern "C" fn context_function3(mut t: Transfer) -> ! {
        for i in 0usize..20 {
            println!("thread 3 Currently at:  {}", i);
            if timer::should_preempt() {
                println!("thread 3 Preempting at {}", i);
                t = unsafe { t.context.resume(0) };
            }
        }
        unsafe {
            t.context.resume(THE_NUMBER_OF_THE_BEAST);
        }
        unreachable!();
    }
    let thread1 = MyThread::new(context_function1, 0, SchedulingAlgorithm::RoundRobin);
    let thread2 = MyThread::new(context_function2, 0, SchedulingAlgorithm::RoundRobin);
    let thread3 = MyThread::new(context_function3, 0, SchedulingAlgorithm::RoundRobin);

    let mut master = MASTER.lock().unwrap();
    master.run();

    // let mut mad_scientist_message = Test {
    //     message: "I am mad scientist, is so coool, son of a bitch!".to_string()
    // };


    // let ptr = &mut mad_scientist_message as *mut Test;

    // extern "C" fn context_function(t: Transfer) -> ! {
 
    //     let ptr = t.data as *mut Test;
    //     let info: &mut Test = unsafe { &mut *ptr };

    //     println!("{}", info.message);

    //     unsafe {
    //         t.context.resume(THE_NUMBER_OF_THE_BEAST);
    //     }

    //     unreachable!();
    // }


    // let mut sched: Box<dyn Scheduler> = Box::new(TicketScheduler::new());


    // let mut thread1 = MyThread::new(context_function, ptr as usize, SchedulingAlgorithm::RoundRobin);
    // thread1.add_tickets(10);

    // sched.enqueue_process(thread1);
    // sched.run();
}
