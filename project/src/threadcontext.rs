#[repr(C)]
#[derive(Default)]
struct ThreadContext {
    rsp: usize,
    rbx: usize,
    rbp: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize
}

impl ThreadContext {
    fn new() -> Self {
        ThreadContext::default()
    }
}
