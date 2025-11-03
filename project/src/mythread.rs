use crate::threadcontext::ThreadContext;
use libc::{mmap, munmap, MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::ptr;
use crate::types::enums::State;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::threadmanager::ThreadManager;
use crate::scheduler::{Scheduler, ROUND_ROBIN_SCHEDULER};

const STACK_SIZE: usize = 1024 * 1024; // 1 MB stack size
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct MyThread { 
    pub id: usize,
    pub stack: *mut u8,
    pub func: Option<fn()>,
    pub ctx: ThreadContext,
    pub state: State
    
}

pub unsafe fn thread_exit(thread: *mut MyThread) {
    unsafe {
        (*thread).state = State::Terminated;
        munmap((*thread).stack as *mut libc::c_void, STACK_SIZE);
        // 1. Obtener el thread actual
        let current_thread = ThreadManager::get_current_thread();
        
        // 2. Cambiar estado a Terminated
        (*current_thread).state = State::Terminated;
        
        // 3. Liberar recursos (pero no el stack todavía)
        (*current_thread).func = None;
        
        // 5. Remover el thread del scheduler
        // Call the scheduler instance (not the trait) to remove the thread
        ROUND_ROBIN_SCHEDULER.remove_thread((*current_thread).id);
        
        // 6. Schedule inmediatamente el siguiente thread
        ThreadManager::schedule_next();
        
        // Nunca retorna // debería ser imposible que llegue acá
        // loop {
        //     core::hint::spin_loop();
        // }
    }
}

// pub unsafe extern "C" fn thread_exit() -> ! {
//     }


impl MyThread {

    pub fn new() -> Self {
        unsafe {
            let stack = mmap(
                ptr::null_mut(),
                STACK_SIZE,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANON,
                -1,
                0,
            ) as *mut u8;

            let ctx = ThreadContext::new();
            let thread_id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);

            Self {
                id: thread_id,
                stack,
                func: None,
                ctx,
                state: State::Ready
            }
        }
    }

    pub fn my_thread_create(func: fn(), thread: *mut MyThread) {
        unsafe {
            let stack_top = (*thread).stack.add(STACK_SIZE);

            let stack_top_aligned = (stack_top as usize & !0xF) as *mut usize;
            let sp = stack_top_aligned.sub(3);

            *sp = func as usize;
            *sp.add(2) = thread as usize;
            *sp.add(1) = thread_exit as usize;

            (*thread).ctx.rsp = sp as usize;
            (*thread).ctx.rbp = 0;
            (*thread).func = Some(func);

            // recibe
    // fn add_thread(&self, thread: Arc<Mutex<MyThread>>) {
    //     self.ready_queue.lock().unwrap().push_back(thread);
    // }

        ROUND_ROBIN_SCHEDULER.add_thread(std::sync::Arc::new(std::sync::Mutex::new(*thread)));
        // Temporal
        ThreadManager::schedule_next();
        }
    }

}
