mod mythread;
mod roundrobinscheduler;
mod ticketscheduler;

use mythread::MyThread;
use roundrobinscheduler::RoundRobinScheduler;
// use ticketscheduler::TicketScheduler;
use context::Transfer;
use libc::{ITIMER_REAL, itimerval};


unsafe extern "C" {
    pub fn setitimer(
        which: ::std::os::raw::c_int,
        new_value: *const itimerval,
        old_value: *mut itimerval) -> ::std::os::raw::c_int;
}


unsafe fn start_preemption_timer() {
    let timer = itimerval {
        it_interval: libc::timeval { tv_sec: 0, tv_usec: 16 }, // 16us
        it_value:    libc::timeval { tv_sec: 0, tv_usec: 16 },
    };
    unsafe {setitimer(ITIMER_REAL, &timer, std::ptr::null_mut()); }
}

extern "C" fn alarm_handler(_sig: i32) {
    crate::roundrobinscheduler::preempt_flag();
}



fn main() {
    const THE_NUMBER_OF_THE_BEAST: usize = 666;

    unsafe { 
        libc::signal(libc::SIGALRM, alarm_handler as usize);
        start_preemption_timer(); 
    }

    extern "C" fn context_function1(mut t: Transfer) -> ! {
        for i in 0usize..20 {
            println!("thread 1 Currently at:  {}", i);
            if roundrobinscheduler::should_preempt() {
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
            if roundrobinscheduler::should_preempt() {
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
            if roundrobinscheduler::should_preempt() {
                println!("thread 3 Preempting at {}", i);
                t = unsafe { t.context.resume(0) };
            }
        }
        unsafe {
            t.context.resume(THE_NUMBER_OF_THE_BEAST);
        }
        unreachable!();
    }
    let mut sched = RoundRobinScheduler::new();
    let thread1 = MyThread::new(context_function1);
    let thread2 = MyThread::new(context_function2);
    let thread3 = MyThread::new(context_function3);

    sched.enqueue_process(thread1);
    sched.enqueue_process(thread2);
    // sched.enqueue_process(thread3);


    sched.run();
    
}
