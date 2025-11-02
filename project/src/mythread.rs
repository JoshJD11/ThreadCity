use crate::threadcontext::ThreadContext;
use libc::{mmap, munmap, MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::ptr;
use crate::types::enums::State;

const STACK_SIZE: usize = 1024 * 1024; // 1 MB stack size

pub struct MyThread { 
    pub stack: *mut u8,
    pub func: Option<fn()>,
    pub ctx: ThreadContext,
    pub state: State
    
}

pub fn thread_exit(thread: *mut MyThread) {
    unsafe {
        (*thread).state = State::Terminated;
        munmap((*thread).stack as *mut libc::c_void, STACK_SIZE);
    }
}

impl MyThread {

    pub fn new() -> Self {
        unsafe {
            let stack = mmap(
                ptr::null_mut(),
                STACK_SIZE,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANON,
                -1,
                0,
            ) as *mut u8;

            let mut ctx = ThreadContext::new();

            Self {
                stack,
                func: None,
                ctx,
                state: State::Ready
            }
        }
    }

    pub fn my_thread_create(func: fn(), thread: *mut MyThread) {
        unsafe {
            let stack_top = (*thread).stack.add(STACK_SIZE);

            let func_ptr = func as usize;
            let exit_ptr = thread_exit as usize;

            let stack_top_aligned = (stack_top as usize & !0xF) as *mut usize;
            let sp = stack_top_aligned.sub(3);

            *sp = func_ptr;
            *sp.add(2) = exit_ptr;
            *sp.add(1) = thread as usize;

            (*thread).ctx.rsp = sp as usize;
            (*thread).ctx.rbp = 0;
            (*thread).func = Some(func);
        }
    }

}
