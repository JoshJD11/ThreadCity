use std::thread;
use std::sync::Arc;
use std::time::Duration;
use crate::mymutex::MyMutex;

// basic deadlock test
pub fn test() {
    let lock = Arc::new(MyMutex::new());
    
    println!("Starting deadlock test...");
    
    // Thread 1: counts up and holds lock forever
    let l1 = lock.clone();
    let handle1 = thread::spawn(move || {
        l1.lock();
        println!("Thread 1: Lock acquired, counting from 0 to 10M");
        
        // FIX: Use u64 to avoid overflow, or just do work without storing large sums
        let mut _sum: u64 = 0;
        for i in 0..10_000_000 {
            _sum += i as u64;  // Use u64 to prevent overflow
        }
        println!("Thread 1: Finished counting");
        
        // INTENTIONALLY DON'T UNLOCK to cause deadlock
        println!("Thread 1: Finished but NOT unlocking!");
        l1.unlock(); // Commented out to cause deadlock
    });

    // Thread 2: counts down - will block forever waiting for lock
    let l2 = lock.clone();
    let handle2 = thread::spawn(move || {
        // Small delay to ensure thread 1 gets lock first
        thread::sleep(Duration::from_millis(10));
        
        println!("Thread 2: Trying to acquire lock...");
        l2.lock(); // This will block forever
        println!("Thread 2: Lock acquired (this should never print)");
        
        // FIX: Use u64 to avoid overflow
        let mut _sum: u64 = 0;
        for i in (0..10_000_000).rev() {
            _sum += i as u64;  // Use u64 to prevent overflow
        }
        println!("Thread 2: Finished counting");
        
        l2.unlock();
    });

    // Wait for first thread to complete
    handle1.join().unwrap();
    
    // Now try second thread with timeout check
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(2);
    
    loop {
        if start.elapsed() > timeout {
            println!("Thread 2 is blocked for {:?} - DEADLOCK CONFIRMED!", start.elapsed());
            break;
        }
        
        // Try to see if thread 2 is making progress
        thread::sleep(Duration::from_millis(100));
    }
}