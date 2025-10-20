use std::ffi::c_void;

enum State {
    Start,
    Running,
    Stopped,
}

struct thread {
    id: i32,
    state: State,
    func_ptr: Option<extern "C" fn(arg: *mut c_void) -> *mut c_void> // hay que usar funciones con extern "C" para poder usarse.
}


extern "C" fn devuelve_valores(_arg: *mut c_void) -> *mut c_void {
    let mut datos = vec![10, 20, 30]; 
    let ptr = datos.as_mut_ptr();
    std::mem::forget(datos); 
    ptr as *mut c_void
}

fn main() {
    let func: Option<extern "C" fn(*mut c_void) -> *mut c_void> = Some(devuelve_valores);

    if let Some(f) = func {
        let result_ptr = f(std::ptr::null_mut()); 

        unsafe {
            let valores: *mut i32 = result_ptr as *mut i32;
            for i in 0..3 {
                println!("Valor {}: {}", i, *valores.add(i));
            }
        }
    }
}
