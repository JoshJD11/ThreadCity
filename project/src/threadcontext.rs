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

// impl ThreadContext {
//     pub fn new() -> Self {
//         Self {
//             rsp: 0,
//             rbp: 0,
//             rip: 0,
//             rbx: 0,
//             r12: 0,
//             r13: 0,
//             r14: 0,
//             r15: 0,
//         }
//     }
// }

impl ThreadContext {
    pub fn new() -> Self {
        ThreadContext::default()
    }
}
