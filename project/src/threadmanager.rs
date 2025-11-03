use crate::scheduler::Scheduler;
use crate::thread::MyThread;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::arch::asm;

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
    
    pub unsafe fn schedule_next() -> ! {
        // 1. Guardar contexto del thread actual (si existe)
        let current = Self::get_current_thread_ptr();
        if !current.is_null() {
            // Guardar registros antes de cambiar
            Self::save_current_context();
        }
        
        // 2. Obtener siguiente thread del scheduler
        let next_thread = SCHEDULER.get_next_thread();
        
        if let Some(next) = next_thread {
            // 3. Actualizar thread actual
            Self::set_current_thread(next);
            
            // 4. Cambiar al contexto del nuevo thread
            Self::context_switch(next);
        } else {
            // No hay más threads, volver al main thread
            Self::switch_to_main_thread();
        }
        
        // Nunca retorna desde aquí
        loop {
            core::hint::spin_loop();
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
    
    // #[naked]
    // unsafe extern "C" fn context_switch(new_thread: *mut MyThread) -> ! {
    //     // Función naked que hace el switch de contexto en assembly
    //     asm!(
    //         // Guardar registros del thread actual
    //         "push rbp",
    //         "push rbx",
    //         "push r12",
    //         "push r13",
    //         "push r14", 
    //         "push r15",
            
    //         // Guardar stack pointer actual
    //         "mov [rdi + 0], rsp",  // ctx.rsp
    //         "mov [rdi + 8], rbp",  // ctx.rbp
            
    //         // Cargar nuevo thread
    //         "mov rdi, rsi",
            
    //         // Cargar nuevo stack pointer
    //         "mov rsp, [rdi + 0]",  // nuevo ctx.rsp
    //         "mov rbp, [rdi + 8]",  // nuevo ctx.rbp
            
    //         // Restaurar registros
    //         "pop r15",
    //         "pop r14",
    //         "pop r13", 
    //         "pop r12",
    //         "pop rbx",
    //         "pop rbp",
            
    //         // Retornar al nuevo thread
    //         "ret",
    //         options(noreturn)
    //     );
    // }
    
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
        unsafe {
            asm!(
                "mov rsp, {main_stack}",
                "jmp {main_entry}",
                main_stack = in(reg) MAIN_STACK_TOP,
                main_entry = in(reg) MAIN_ENTRY_POINT,
                options(noreturn)
            );
        }
    }
}

// Manager global
static mut CURRENT_MANAGER: ThreadManager = ThreadManager::new(&ROUND_ROBIN_SCHEDULER);
static mut MAIN_STACK_TOP: usize = 0;
static mut MAIN_ENTRY_POINT: usize = 0;