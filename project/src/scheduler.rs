use crate::mythread::MyThread;
use std::collections::VecDeque;
use crate::threadcontext::ThreadContext;
use std::arch::naked_asm;
use crate::types::enums::State;

pub struct Scheduler {
    pub ready_queue: VecDeque<MyThread>,
    pub current_thread: Option<MyThread>,
}

#[unsafe(naked)]
unsafe extern "C" fn switch_context(old: *mut ThreadContext, new: *const ThreadContext) {
    naked_asm!(
        "mov [rdi + 0x00], rsp",
        "mov [rdi + 0x08], rbx",
        "mov [rdi + 0x10], rbp",
        "mov [rdi + 0x18], r12",
        "mov [rdi + 0x20], r13",
        "mov [rdi + 0x28], r14",
        "mov [rdi + 0x30], r15",
        "mov rsp, [rsi + 0x00]",
        "mov rbx, [rsi + 0x08]",
        "mov rbp, [rsi + 0x10]",
        "mov r12, [rsi + 0x18]",
        "mov r13, [rsi + 0x20]",
        "mov r14, [rsi + 0x28]",
        "mov r15, [rsi + 0x30]",
        "ret"
    );
}

impl Scheduler {
    pub fn new() -> Self {
        Self { 
            ready_queue: VecDeque::new(),
            current_thread: None,
        }
    }


    pub fn add_thread(&mut self, thread: MyThread) {
        self.ready_queue.push_back(thread);
    }

    pub unsafe fn yield_thread(&mut self) {
        if self.current_thread.is_none() && self.ready_queue.is_empty() {
            return;
        }

        let old_ctx_ptr: *mut ThreadContext;
        let current_thread;

        if self.current_thread.is_none() {
            if let Some(mut next_thread) = self.ready_queue.pop_front() {
                next_thread.state = State::Running;
                self.current_thread = Some(next_thread);
                // println!("{} state", self.current_thread.as_mut().unwrap().state as i32);
            }
            current_thread = self.current_thread.as_mut().unwrap();
            old_ctx_ptr = &mut current_thread.ctx;
            // println!("gets here");
            let mut dummy = ThreadContext::new();
            let dummy_ctx_ptr: *mut ThreadContext = &mut dummy;

            unsafe {
                println!("context switch 1");
                switch_context(dummy_ctx_ptr, old_ctx_ptr);
            }
            return;
        }


        current_thread = self.current_thread.as_mut().unwrap();
        // let old_ctx_ptr: *mut ThreadContext = &mut current_thread.ctx;
        old_ctx_ptr = &mut current_thread.ctx;
        let mut should_requeue = current_thread.state != State::Terminated;

        let mut old_thread = self.current_thread.take().unwrap();
        

        if let Some(mut next_thread) = self.ready_queue.pop_front() {
            next_thread.state = State::Running;
            let new_ctx_ptr: *const ThreadContext = &next_thread.ctx;
            self.current_thread = Some(next_thread);
            unsafe {
                println!("context switch 2");
                switch_context(old_ctx_ptr, new_ctx_ptr);
            }
        } else {
            should_requeue = false;
            println!("Else");
            if old_thread.state == State::Terminated {
                self.current_thread = None;
            }
        }
        if should_requeue {
            old_thread.state = State::Ready;
            self.ready_queue.push_back(old_thread);
        }
    }
    
    
    pub unsafe fn run(&mut self) {
        while !self.ready_queue.is_empty() || self.current_thread.is_some() {
           
            self.yield_thread();

        }
    }
}