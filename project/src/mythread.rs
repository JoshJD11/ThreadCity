extern crate context;
use context::stack::ProtectedFixedSizeStack;
use context::{Context, Transfer};


pub struct MyThread { 
    pub ctx: Option<Transfer>,
    _stack: ProtectedFixedSizeStack,
    pub tickets: usize
}


impl MyThread {

    pub fn new(func: extern "C" fn(Transfer) -> !) -> Self {
        let stack = ProtectedFixedSizeStack::default();
        Self {
            ctx: Some(Transfer::new(unsafe { Context::new(&stack, func) }, 0)),
            _stack: stack,
            tickets: 0
        }
    }

    pub fn add_tickets(&mut self, tickets: usize) {
        self.tickets += tickets;
    }

}
