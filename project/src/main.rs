mod threadcontext;
mod mythread;
mod types;
mod scheduler;
mod threadmanager;

// use threadmanager::ThreadManager;
use mythread::MyThread;
use scheduler::Scheduler;

use crate::threadmanager::capture_main_as_thread_and_register;

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
    // Initialize main thread context for later return
    unsafe { capture_main_as_thread_and_register(); }
    println!("Main thread: initialized main context.");

    // Crear thread (sin parámetros)
    let mut thread1 = MyThread::new();
    let mut thread2 = MyThread::new();
    
    // Función simple para el thread
    fn simple_thread_func1() {
        println!("Thread 1 ejecutándose!");
    }

    fn simple_thread_func2() {
        println!("Thread 2 ejecutándose!");
    }
    println!("Main thread: created threads.");
    
    // Configurar el thread con su función
    MyThread::my_thread_create(simple_thread_func1, &mut thread1);
    MyThread::my_thread_create(simple_thread_func2, &mut thread2);
    // let mut sched = Scheduler::new();

    // let t1 = MyThread::new(hilo1);
    // let t2 = MyThread::new(hilo2);
    // let t3 = MyThread::new(hilo3);

    // sched.add_thread(t1);
    // sched.add_thread(t2);
    // sched.add_thread(t3);

    // unsafe {
    //     sched.run();
    // }
    
    println!("Main thread: starting scheduler to run threads."); // Thread main is not getting here
    unsafe { threadmanager::ThreadManager::schedule_next(); }
}

// // Ejemplo de uso
// fn example_usage() {
    
// }