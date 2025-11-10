use std::sync::atomic::{AtomicI32, Ordering};
use libc::{self, SYS_futex, FUTEX_WAIT, FUTEX_WAKE};

pub struct MyMutex {
    state: AtomicI32
}

impl MyMutex {
    pub fn new() -> Self {
        Self {
            state: AtomicI32::new(0)
        }
    }

    pub fn try_lock(&self) -> bool {
        self.state.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).is_ok()
    }

    pub fn lock(&self) {
        while !self.try_lock() {
            unsafe {
                libc::syscall(SYS_futex, &self.state as *const _, FUTEX_WAIT, 1, std::ptr::null::<libc::timespec>());
            }
        }
    }

    pub fn unlock(&self) {
        self.state.store(0, Ordering::Release);
        unsafe {
            libc::syscall(SYS_futex, &self.state as *const _, FUTEX_WAKE, 1);
        }
    }

    pub fn destroy(&self) {
        println!("Destroying thread... :D");
    }
}
