use std::sync::atomic::{AtomicBool, Ordering};
use libc::{ITIMER_REAL, itimerval};
static PREEMPT: AtomicBool = AtomicBool::new(false);

unsafe extern "C" {
    pub fn setitimer(
        which: ::std::os::raw::c_int,
        new_value: *const itimerval,
        old_value: *mut itimerval) 
        -> ::std::os::raw::c_int;
}

unsafe fn start_preemption_timer() {
    let timer = itimerval {
        it_interval: libc::timeval { tv_sec: 0, tv_usec: 16 }, // 16us
        it_value:    libc::timeval { tv_sec: 0, tv_usec: 16 },
    };
    unsafe { setitimer(ITIMER_REAL, &timer, std::ptr::null_mut()); }
}

extern "C" fn alarm_handler(_sig: i32) {
    crate::timer::preempt_flag();
}


pub fn preempt_flag() {
    PREEMPT.store(true, Ordering::SeqCst);
}

pub fn should_preempt() -> bool {
    PREEMPT.swap(false, Ordering::SeqCst)
}

pub fn start_timer() {
    unsafe { 
        libc::signal(libc::SIGALRM, alarm_handler as usize);
        start_preemption_timer(); 
     }
}