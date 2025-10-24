// mod thread;
// mod types;
mod mutex;

use std::thread;
use std::sync::Arc;

fn main() {
    let lock = Arc::new(mutex::Mutex::new());
    let mut handles = vec![];

    for i in 0..5 {
        let l = lock.clone();
        handles.push(thread::spawn(move || {
            l.lock();
            println!("Thread {i} inside");
            l.unlock();
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
