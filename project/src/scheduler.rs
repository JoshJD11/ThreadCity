use crate::mythread::MyThread;
use std::collections::VecDeque;
use crate::threadcontext::ThreadContext;
use std::arch::naked_asm;

pub struct Scheduler {
    pub ready_queue: VecDeque<MyThread>,
    pub current: Option<usize>
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
            current: None
        }
    }


    pub fn add_thread(&mut self, thread: MyThread) {
        self.ready_queue.push_back(thread);
    }

    pub unsafe fn yield_thread(&mut self) {
        if let Some(current_index) = self.current {
            let next_index = (current_index + 1) % self.ready_queue.len();
            let current_thread: *mut ThreadContext = &mut self.ready_queue[current_index].ctx;
            let next_thread: *const ThreadContext = &self.ready_queue[next_index].ctx;
            unsafe {
                switch_context(current_thread, next_thread);
            }
            self.current = Some(next_index);
        }
        else if !self.ready_queue.is_empty() {
            self.current = Some(0);
        }
    }

    pub unsafe fn run(&mut self) {
        if self.ready_queue.is_empty() {
            return;
        }
        else {
            self.current = Some(0);
            let first_thread = &self.ready_queue[0].ctx;
            let mut dummy_ctx = ThreadContext::new();
            unsafe {
                switch_context(&mut dummy_ctx as *mut ThreadContext, first_thread as *const ThreadContext);
            }
        }
    }
}
