extern crate context;
use context::stack::ProtectedFixedSizeStack;
use context::{Context, Transfer};
use std::cmp::Ordering;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use crate::types::enums::SchedulingAlgorithm;


static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct ThreadArgs {
    pub arguments: usize,
    pub preemptive: *mut bool,
}

impl ThreadArgs {
    pub fn new(mut args: usize, preemp: *mut bool) -> Self {
        Self {
            arguments: args,
            preemptive: preemp,
        }
    }
}

pub struct MyThread { 
    pub id: usize,
    pub ctx: Option<Transfer>,
    pub args: usize,
    pub sched_type: SchedulingAlgorithm,
    _stack: ProtectedFixedSizeStack,
    tickets: usize,
    deadline: usize,
    pub preemptive: bool,
}



impl MyThread {

    pub fn new(func: extern "C" fn(Transfer) -> !, arguments: usize, sched_algorithm: SchedulingAlgorithm) -> Self {
        let stack = ProtectedFixedSizeStack::new(1024 * 1024).unwrap();
        let thread_id = NEXT_ID.fetch_add(1, AtomicOrdering::Relaxed);        let mut thread = Self {
            id: thread_id,
            ctx: None,
            args: arguments,
            sched_type: sched_algorithm,
            _stack: stack,
            tickets: 1,
            deadline: 0,
            preemptive: true,
        };

        let preemptive_ptr: *mut bool = &mut thread.preemptive;

        let args = ThreadArgs::new(arguments, preemptive_ptr);
        let ptr = Box::into_raw(Box::new(args));

        thread.ctx = Some(Transfer::new(
            unsafe { Context::new(&thread._stack, func) },
            ptr as usize,
        ));

        thread }
   
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

    pub fn set_non_preemptive(&mut self) {
        self.preemptive = false;
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
