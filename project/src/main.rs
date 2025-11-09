mod mythread;
mod ticketscheduler;
mod scheduler;


use mythread::MyThread;
use scheduler::Scheduler;
use ticketscheduler::TicketScheduler;
// use roundrobinscheduler::RoundRobinScheduler;
use context::Transfer;



struct Test {
    message: String,
}

fn main() {
    const THE_NUMBER_OF_THE_BEAST: usize = 666;

    let mut mad_scientist_message = Test {
        message: "I am mad scientist, is so coool, son of a bitch!".to_string()
    };


    let ptr = &mut mad_scientist_message as *mut Test;

    extern "C" fn context_function(t: Transfer) -> ! {
 
        let ptr = t.data as *mut Test;
        let info: &mut Test = unsafe { &mut *ptr };

        println!("{}", info.message);

        unsafe {
            t.context.resume(THE_NUMBER_OF_THE_BEAST);
        }

        unreachable!();
    }


    let mut sched: Box<dyn Scheduler> = Box::new(TicketScheduler::new());


    let mut thread1 = MyThread::new(context_function, ptr as usize);
    thread1.add_tickets(10);

    sched.enqueue_process(thread1);
    sched.run();
}