// use crate::types::enums::State;
use crate::threadcontext::ThreadContext;
// use std::collections::HashMap;
// use std::any::Any;
// use std::sync::atomic::{AtomicI32, Ordering};
use libc::{mmap, MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::ptr;

// static ID_COUNTER: AtomicI32 = AtomicI32::new(1);
const STACK_SIZE: usize = 1024 * 1024; // 1 MB stack size

pub struct MyThread { 
    pub stack: *mut u8,
    pub func: Option<fn()>,
    pub ctx: ThreadContext
}

extern "C" fn thread_start(func: fn()) {
    func();
    // Here we can notify the scheduler that the thread has finished
}

impl MyThread {
    pub fn new(func: fn()) -> Self {
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
            let stack_top = stack.add(STACK_SIZE);

            let func_ptr = func as usize;

            let stack_top_aligned = (stack_top as usize & !0xF) as *mut usize;
            let sp = stack_top_aligned.sub(2);
            *sp.add(1) = func_ptr;

            ctx.rsp = sp as usize;
            ctx.rbp = 0;

            Self {
                stack,
                func: Some(func),
                ctx
            }
        }
    }

}
