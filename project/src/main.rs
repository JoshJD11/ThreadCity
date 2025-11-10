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
use crate::mypthreads::runMaster;

struct Test {
    message: String,
}

fn main() {
    const THE_NUMBER_OF_THE_BEAST: usize = 666;
    timer::start_timer();


    extern "C" fn context_function1(mut t: Transfer) -> ! {
        t = MyPthreads::my_thread_yield(t);
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

    let mut thread1 = MyPthreads::new(); //Tiene que ser MyPthreads no MyThreads
    let mut thread2 = MyPthreads::new();
    let mut thread3 = MyPthreads::new();
    thread1.my_thread_create( context_function1, 0, SchedulingAlgorithm::RoundRobin);
    thread2.my_thread_create( context_function2, 0, SchedulingAlgorithm::RoundRobin);
    thread3.my_thread_create( context_function3, 0, SchedulingAlgorithm::RoundRobin);
    unsafe {
        runMaster();
    }

}
