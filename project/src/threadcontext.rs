#[repr(C)]
#[derive(Default)]
pub struct ThreadContext {
    pub rsp: usize,
    pub rbx: usize,
    pub rbp: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize
}

impl ThreadContext {
    pub fn new() -> Self {
        ThreadContext::default()
    }
}
