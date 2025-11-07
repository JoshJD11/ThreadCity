mod mythread;
mod roundrobinscheduler;
mod ticketscheduler;
mod timer;

use mythread::MyThread;
use roundrobinscheduler::RoundRobinScheduler;
use ticketscheduler::TicketScheduler;
use context::Transfer;



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
    let mut sched = TicketScheduler::new();
    let thread1 = MyThread::new(context_function1);
    let thread2 = MyThread::new(context_function2);
    let thread3 = MyThread::new(context_function3);

    sched.enqueue_process(thread1, 999);
    sched.enqueue_process(thread2, 1);
    sched.enqueue_process(thread3, 499);


    sched.run();
    
}
