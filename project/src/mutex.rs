use std::sync::atomic::{AtomicI32, Ordering};
use libc::{self, SYS_futex, FUTEX_WAIT, FUTEX_WAKE};

pub struct Mutex {
    state: AtomicI32
}

impl Mutex {
    pub fn new() -> Self {
        Self {
            state: AtomicI32::new(0)
        }
    }

    pub fn lock(&self) {
        while self.state.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
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
}
