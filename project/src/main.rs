// mod mythread;
// mod types;
mod mymutex;
mod deadlocktest;

use std::thread;
use std::sync::Arc;
use mymutex::MyMutex;

fn main() {
    deadlocktest::test();
    // let lock = Arc::new(MyMutex::new());
    // let mut handles = vec![];

    // for i in 0..5 {
    //     let l = lock.clone();
    //     handles.push(thread::spawn(move || {
    //         l.lock();
    //         println!("Thread {i} inside");
    //         l.unlock();
    //     }));
    // }

    // for h in handles {
    //     h.join().unwrap();
    // }
}
