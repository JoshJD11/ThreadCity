use crate::types;
use types::enums::State;
use std::collections::HashMap;
use std::any::Any;
use std::sync::atomic::{AtomicI32, Ordering};

static ID_COUNTER: AtomicI32 = AtomicI32::new(1);

pub struct MyThread { // Add more attributes as needed, this file is just a brainstorm.
    id: i32,
    state: State,
    args: HashMap<String, Box<dyn Any>>,
    function: Option<fn(HashMap<String, Box<dyn Any>>) -> Box<dyn Any>>,
    stack: Vec<Box<dyn Any>>
}

impl MyThread {
    pub fn new() -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            state: State::Ready,
            args: HashMap::new(),
            function: None,
            stack: Vec::new()
        }
    }

    pub fn test_function(&self) -> i32 {
        self.id
    }

}
