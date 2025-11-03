use crate::scheduler::{ROUND_ROBIN_SCHEDULER, Scheduler};
use crate::mythread::MyThread;
use std::arch::naked_asm;
use crate::threadcontext::ThreadContext;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::arch::asm;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub struct ThreadManager {
    current_thread: AtomicPtr<MyThread>,
    scheduler: &'static dyn Scheduler,
}

impl ThreadManager {
    pub fn new(scheduler: &'static dyn Scheduler) -> Self {
        Self {
            current_thread: AtomicPtr::new(core::ptr::null_mut()),
            scheduler,
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
                Self::switch_context(old_ctx_ptr, new_ctx_ptr);
            }

            // We should never reach here because switch_context returns into the new thread.
        } else {
            // No hay más threads, volver al main thread
            Self::switch_to_main_thread();
        }
        
        // Nunca retorna desde aquí
        loop {
            core::hint::spin_loop();
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
    
    fn save_current_context() {
        // Guardar los registros del thread actual en su estructura
        // Esto se haría en assembly antes del context switch
    }
    
    fn switch_to_main_thread() -> ! {
        // Volver al thread principal del sistema
        println!("Volviendo al main thread");
        unsafe {
            asm!(
                "mov rsp, {0}",
                "jmp {1}",
                in(reg) MAIN_STACK_TOP,
                in(reg) MAIN_ENTRY_POINT,
                options(noreturn)
            );
        }
    }
}

// Manager global (initialized at runtime via lazy_static to avoid non-const initialization)
lazy_static! {
    pub static ref CURRENT_MANAGER: ThreadManager = ThreadManager::new(&*crate::scheduler::ROUND_ROBIN_SCHEDULER as &'static dyn Scheduler);
}

// Main thread context (captured on initialization)
pub static mut MAIN_STACK_TOP: usize = 0;
pub static mut MAIN_ENTRY_POINT: usize = 0;

// Initialize main thread context (call this at start of main)
pub unsafe fn init_main_context() {
    unsafe {
        let mut rsp: usize;
        let mut rip: usize;
        core::arch::asm!(
            // Store current stack and instruction pointers
            "mov {0}, rsp",           // Get current stack pointer
            "lea {1}, [rip + {label}]", // Get address of our label
            out(reg) rsp,             // Output: stack pointer
            out(reg) rip,             // Output: instruction pointer
            label = sym return_point,  // Define our symbol
            options(nomem, nostack)    // Don't modify memory or stack
        );
        MAIN_STACK_TOP = rsp;
        MAIN_ENTRY_POINT = rip;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn return_point() {
    // Just a marker - we return here from threads
    // No body needed as we just use the function address
}