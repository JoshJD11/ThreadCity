use crate::mythread::MyThread;
use std::collections::VecDeque;

pub struct Scheduler {
    readyQueue: VecDeque<Box<MyThread>>,
    currentThread: Option<Box<MyThread>>
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            readyQueue: Vec::new(),
            currentThread: None
        }
    }

    pub fn add_thread(&mut self, thread: Box<MyThread>) {
        self.readyQueue.push_back(thread);
    }

    pub fn pop_thread(&mut self) -> Option<Box<MyThread>> {
        self.readyQueue.pop_front()
    }

    #[naked]
    unsafe extern "C" fn switch_context(old: *mut ThreadContext, new: *const ThreadContext) {
        asm!(
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
            "ret",
            options(noreturn)
        );
    }
}
