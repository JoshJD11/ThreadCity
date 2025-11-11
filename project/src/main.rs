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
mod masterscheduler;

use mythread::MyThread;
use mythread::ThreadArgs;
use scheduler::Scheduler;
use mypthreads::MyPthreads;
use context::Transfer;
use crate::types::SchedulingAlgorithm;
use crate::mypthreads::run_master;

struct Test {
    message: String,
}

fn isPreemptive(mut t: Transfer) -> bool {
    let args = unsafe { &mut *(t.data as *mut ThreadArgs) };
    let preemptive_ref = unsafe { &mut *args.preemptive };
    return *preemptive_ref;
}

fn main() {
    const THE_NUMBER_OF_THE_BEAST: usize = 666;
    timer::start_timer();


    extern "C" fn context_function1(mut t: Transfer) -> ! {
        println!("thread 1 yield");
        t = MyPthreads::my_thread_yield(t, 0);
        unsafe {
            t = MyPthreads::my_thread_yield(t, THE_NUMBER_OF_THE_BEAST);
        }
        unreachable!();
    }

    extern "C" fn context_function2(mut t: Transfer) -> ! {
        let args = unsafe { &mut *(t.data as *mut ThreadArgs) };

        // Access arguments (usize)
        // println!("Arguments: {}", args.arguments);

        for i in 0usize..20 {
            println!("thread 2 Currently at:  {}", i);
            let preemptive = unsafe { &mut *args.preemptive };
            if timer::should_preempt() && *preemptive {
                println!("thread 2 Preempting at {}", i);
                unsafe {
                    t = unsafe { t.context.resume(0) };
                }
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
 
    let mut thread1 = MyPthreads::new();
    let mut thread2 = MyPthreads::new();
    let mut thread3 = MyPthreads::new();
    thread1.my_thread_create( context_function1, 0, 0, 0, SchedulingAlgorithm::RoundRobin);
    thread2.my_thread_create( context_function2, 0, 5, 20,  SchedulingAlgorithm::Lottery); 
    thread3.my_thread_create( context_function3, 0, 10, 3, SchedulingAlgorithm::RealTime);
    thread1.my_thread_chsched(SchedulingAlgorithm::Lottery);
    // thread2.my_thread_join();
    

    unsafe {
        run_master();
    }

}
