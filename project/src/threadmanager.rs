use crate::scheduler::{ROUND_ROBIN_SCHEDULER, Scheduler};
use crate::mythread::MyThread;
use std::arch::naked_asm;
use crate::threadcontext::ThreadContext;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::arch::asm;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::mythread::ID_COUNTER; // or provide a function to allocate ids
use libc;

pub struct ThreadManager {
    current_thread: AtomicPtr<MyThread>,
}

impl ThreadManager {
    pub fn new(scheduler: &'static dyn Scheduler) -> Self {
        Self {
            current_thread: AtomicPtr::new(core::ptr::null_mut()),
        }
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

    pub unsafe fn schedule_next() -> ! {
        // println!("ThreadManager: scheduling next thread...");
        // 1. Guardar contexto del thread actual (si existe)
        let current = Self::get_current_thread_ptr();

        // Prepare an old context pointer. If there's no current thread, use a stack-local
        // dummy context so switch_context always receives a valid pointer to write into.
        let mut dummy_ctx = ThreadContext::new();
        let old_ctx_ptr: *mut ThreadContext = if current.is_null() {
            &mut dummy_ctx as *mut ThreadContext
        } else {
            // SAFETY: current is non-null so deref is valid
            unsafe { &mut (*current).ctx as *mut ThreadContext }
        };
        
        // 2. Obtener siguiente thread del scheduler
        let next_thread_opt: Option<Arc<Mutex<MyThread>>> = ROUND_ROBIN_SCHEDULER.get_next_thread(); 
        
        if let Some(next_arc) = next_thread_opt {
            // Lock the next thread to get access to its context and obtain a raw pointer.
            // Keep the guard in scope until after switch_context so the memory remains valid.
            let mut guard = next_arc.lock().unwrap();
            let new_ctx_ptr: *const ThreadContext = &guard.ctx as *const ThreadContext;
            let next_ptr: *mut MyThread = &mut *guard as *mut MyThread;
 
            // 3. Actualizar thread actual (store raw pointer)
            Self::set_current_thread(next_ptr);

            unsafe {
                // 4. Cambiar al contexto del nuevo thread
                // println!("ThreadManager: switching to thread ID {}", guard.id);
                Self::switch_context(old_ctx_ptr, new_ctx_ptr);
            }

            // We should never reach here because switch_context returns into the new thread.
        } else {
            // No hay más threads, volver al main thread
            Self::switch_to_main_thread();
        }
        
        // Nunca retorna desde aquí
        println!("ThreadManager: no more threads to schedule, exiting.");
        // SAFETY: libc::exit is unsafe because it terminates the process immediately
        unsafe {
            libc::exit(0);
        }
    }
    
    
    pub unsafe fn get_current_thread() -> *mut MyThread {
        Self::get_current_thread_ptr()
    }
    
    fn get_current_thread_ptr() -> *mut MyThread {
        CURRENT_MANAGER.current_thread.load(Ordering::Acquire)
    }
    
    fn set_current_thread(thread: *mut MyThread) {
        CURRENT_MANAGER.current_thread.store(thread, Ordering::Release);
    }
    
    // fn save_current_context is removed as it's not used
    fn switch_to_main_thread() -> ! {
        println!("Volviendo al main thread");
        
        // Try to get main thread from our storage first
        if let Some(main_arc) = MAIN_THREAD_STORE.lock().unwrap().as_ref() {
            println!("Found main thread in storage, switching to it.");
            let guard = main_arc.lock().unwrap();
            let ctx_ptr = &guard.ctx as *const ThreadContext;
            
            // Create a dummy context to save current state (won't be used)
            let mut dummy_ctx = ThreadContext::new();
            let dummy_ptr = &mut dummy_ctx as *mut ThreadContext;
            
            unsafe {
                // Switch to main's saved context
                Self::switch_context(dummy_ptr, ctx_ptr);
            }
        }

        // As fallback, find main thread in scheduler (it should be the thread with ID 0)
        else if let Some(main_arc) = ROUND_ROBIN_SCHEDULER.get_next_thread() {
            println!("Found main thread in scheduler, switching to it.");
            let guard = main_arc.lock().unwrap();
            let ctx_ptr = &guard.ctx as *const ThreadContext;
            
            // Create a dummy context to save current state (won't be used)
            let mut dummy_ctx = ThreadContext::new();
            let dummy_ptr = &mut dummy_ctx as *mut ThreadContext;
            
            unsafe {
                // Switch to main's saved context
                Self::switch_context(dummy_ptr, ctx_ptr);
            }
        }
        
        // If we couldn't find main thread, exit cleanly
        println!("Error: couldn't find main thread in storage or scheduler!");
        unsafe {
            libc::exit(1);
        }
    }
}

// Manager global (initialized at runtime via lazy_static to avoid non-const initialization)
lazy_static! {
    pub static ref CURRENT_MANAGER: ThreadManager = ThreadManager::new(&*crate::scheduler::ROUND_ROBIN_SCHEDULER as &'static dyn Scheduler);
}

// Single module-level store for the main thread so capture and switch both use the same storage
lazy_static! {
    static ref MAIN_THREAD_STORE: Mutex<Option<Arc<Mutex<MyThread>>>> = Mutex::new(None);
}


// Capture the current registers into `ctx` using the same layout used by switch_context.
// SAFETY: this writes CPU registers into memory at ctx; call only when you intend to capture
// the current execution state (e.g., at program startup to snapshot main).
pub unsafe fn capture_current_context(ctx: &mut ThreadContext) {
    let ptr = ctx as *mut ThreadContext;
    // Inline assembly is considered an unsafe operation even inside an unsafe fn under Rust 2024,
    // so wrap the asm! invocation in an explicit inner unsafe block.
    unsafe {
        core::arch::asm!(
            // write current registers into the ThreadContext fields:
            "mov [rdi + 0x00], rsp",
            "mov [rdi + 0x08], rbx",
            "mov [rdi + 0x10], rbp",
            "mov [rdi + 0x18], r12",
            "mov [rdi + 0x20], r13",
            "mov [rdi + 0x28], r14",
            "mov [rdi + 0x30], r15",
            in("rdi") ptr,
            options(nostack)
        );
    }
}

pub fn capture_main_as_thread_and_register() {
    // capture registers
    let mut main_ctx = ThreadContext::new();
    unsafe { capture_current_context(&mut main_ctx) };

    // create a MyThread that wraps that context
    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let main_thread = MyThread::from_existing_ctx(id, main_ctx);

    // wrap and push to scheduler
    let arc = Arc::new(Mutex::new(main_thread));
    // Lock briefly to get a stable pointer to the heap allocation
    let guard = arc.lock().unwrap();
    let ptr: *mut MyThread = &*guard as *const MyThread as *mut MyThread;
    
    // Add to scheduler first
    // ROUND_ROBIN_SCHEDULER.add_thread(arc.clone());
    
    // Set current thread so first schedule call saves live context
    CURRENT_MANAGER.current_thread.store(ptr, Ordering::Release);
    drop(guard);
    
    // Store main thread reference in ThreadManager
    // Note: CURRENT_MANAGER is not mutable directly, but we can work around this
    // by defining a new static that uses interior mutability with a Mutex.
    *MAIN_THREAD_STORE.lock().unwrap() = Some(arc);
}


