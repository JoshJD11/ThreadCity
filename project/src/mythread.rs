use crate::threadcontext::ThreadContext;
use libc::{mmap, MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::ptr;
use crate::types::enums::State;

const STACK_SIZE: usize = 1024 * 1024; // 1 MB stack size

pub struct MyThread { 
    pub stack: *mut u8,
    pub func: Option<fn()>,
    pub ctx: ThreadContext,
    pub state: State
    
}


impl MyThread {

    pub fn thread_exit(&mut self) {
        self.state = State::Terminated;
        // unsafe { libc::exit(0) }; // Colocar extern "C" en caso de usarse
    }

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
            let exit_ptr = MyThread::thread_exit as usize;

            let stack_top_aligned = (stack_top as usize & !0xF) as *mut usize;
            let sp = stack_top_aligned.sub(2);

            *sp = func_ptr;
            *sp.add(1) = exit_ptr;

            ctx.rsp = sp as usize;
            ctx.rbp = 0;

            Self {
                stack,
                func: Some(func),
                ctx,
                state: State::Ready
            }
        }
    }

}
