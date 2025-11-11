extern crate context;
use context::stack::ProtectedFixedSizeStack;
use context::{Context, Transfer};
use std::cmp::Ordering;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use crate::types::enums::SchedulingAlgorithm;


static NEXT_ID: AtomicUsize = AtomicUsize::new(0);


pub struct MyThread { 
    pub id: usize,
    pub ctx: Option<Transfer>,
    pub args: usize,
    pub sched_type: SchedulingAlgorithm,
    _stack: ProtectedFixedSizeStack,
    tickets: usize,
    deadline: usize,
}


impl MyThread {

    pub fn new(func: extern "C" fn(Transfer) -> !, arguments: usize, sched_algorithm: SchedulingAlgorithm) -> Self {
        let stack = ProtectedFixedSizeStack::new(1024 * 1024).unwrap();
        let thread_id = NEXT_ID.fetch_add(1, AtomicOrdering::Relaxed);
        Self {
            id: thread_id,
            ctx: Some(Transfer::new(unsafe { Context::new(&stack, func) }, arguments)), // arguments can be a default value like 0
            args: arguments,
            sched_type: sched_algorithm,
            _stack: stack,
            tickets: 1,
            deadline: 0,
        }
    }

    pub fn add_tickets(&mut self, tickets: usize) {
        self.tickets += tickets;
    }

    pub fn get_tickets(&mut self) -> usize {
        return self.tickets;
    }

    pub fn set_deadline(&mut self, deadline: usize) {
        self.deadline = deadline;
    }

    pub fn get_deadline(&mut self) -> usize {
        return self.deadline;
    }

}

impl PartialEq for MyThread {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline
    }
}

impl Eq for MyThread {}

impl PartialOrd for MyThread {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyThread {
    fn cmp(&self, other: &Self) -> Ordering {
        other.deadline.cmp(&self.deadline)
    }
}
