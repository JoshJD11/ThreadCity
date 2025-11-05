mod mythread;
mod scheduler;

use mythread::MyThread;
use scheduler::RoundRobinScheduler;
use context::Transfer;


fn main() {
    const THE_NUMBER_OF_THE_BEAST: usize = 666;

    extern "C" fn context_function(mut t: Transfer) -> ! {
        for i in 0usize..3 {
            println!("Yielding {}", i);
            t = unsafe { t.context.resume(i) };
        }
        unsafe {
            t.context.resume(THE_NUMBER_OF_THE_BEAST); // THE NUMBER OF THE BEAST!!!
        }
        unreachable!();
    }

    let mut sched = RoundRobinScheduler::new();
    let thread1 = MyThread::new(context_function);
    let thread2 = MyThread::new(context_function);

    sched.enqueue_process(thread1);
    sched.enqueue_process(thread2);

    sched.run();
    
}
