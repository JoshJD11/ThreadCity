mod threadcontext;
mod mythread;
mod types;
mod scheduler;

use mythread::MyThread;
use scheduler::Scheduler;

fn hilo1() {
    println!("Hilo 1: comenzando ejecución");
    for i in 0..3 {
        println!("Hilo 1: iteración {i}");
    }
    println!("Hilo 1: terminando");
}

fn hilo2() {
    println!("Hilo 2: comenzando ejecución");
    for i in 0..3 {
        println!("Hilo 2: iteración {i}");
    }
    println!("Hilo 2: terminando");
}

fn hilo3() {
    println!("Hilo 3: comenzando ejecución");
    for i in 0..3 {
        println!("Hilo 3: iteración {i}");
    }
    println!("Hilo 3: terminando");
}

fn main() {
    let mut sched = Scheduler::new();

    let t1 = MyThread::new(hilo1);
    let t2 = MyThread::new(hilo2);
    let t3 = MyThread::new(hilo3);

    sched.add_thread(t1);
    sched.add_thread(t2);
    sched.add_thread(t3);

    unsafe {
        sched.run();
    }
}


