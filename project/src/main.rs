mod grid_generator;
mod architect;
mod city;
mod vehicle;
mod simulation;

use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{Application};
use crate::city::City;
use crate::simulation::Simulation;
use crate::vehicle::Vehicle;

// -----------------------------------------------
mod mythread;
mod ticketscheduler;
mod scheduler;
mod realtimescheduler;
mod roundrobinscheduler;
mod mypthreads;
mod masterofpuppets;
mod timer;
mod types;
mod mymutex;
mod masterscheduler;

use mythread::MyThread;
use scheduler::Scheduler;
use mypthreads::MyPthreads;
use context::Transfer;
use crate::types::SchedulingAlgorithm;
use crate::mypthreads::run_master;
// -----------------------------------------------
const APP_ID: &str = "org.gtk.city";

pub struct CitySimulation {
    pub sim: Rc<RefCell<Simulation>>
}

impl CitySimulation {
    pub fn new() -> Self {
        Self {
            sim: grid_generator::create_simulation()
        }
    }
}
fn main() {
    // Ejecutar un poco de lógica primero
    //let simulation: Rc<RefCell<Simulation>> = grid_generator::create_simulation();

    const THE_NUMBER_OF_THE_BEAST: usize = 666;

    let mut simulation = CitySimulation::new();
    let sim_ptr = &mut simulation as *mut CitySimulation; // Para acceso mutable
    let sim_usize = sim_ptr as usize;

    extern "C" fn context_function1(mut t: Transfer) -> ! {
        let city_sim_ptr = t.data as *const CitySimulation;

        unsafe {
            let city_simulation = &*city_sim_ptr;

            // Guardamos el Transfer original en un puntero raw
            // PASO 1: convertir &mut Transfer en *mut Transfer
            let t_ptr: *mut Transfer = &mut t;

            let simulation_app = Application::builder().application_id(APP_ID).build();

            simulation_app.connect_activate(move |app| {
                unsafe {
                    // PASO 2: Recuperar Transfer mut desde el puntero
                    let t_ref: &mut Transfer = &mut *t_ptr;

                    // PASO 3: usarlo
                    *t_ref = grid_generator::generate_grid(app, &city_simulation.sim, *t_ref);
                }
            });

            simulation_app.run();
        }

        unreachable!();
    }


    extern "C" fn context_function2(mut t: Transfer) -> ! {
        let city_sim_ptr = t.data as *const CitySimulation;

        unsafe {
            let city_simulation = &*city_sim_ptr;
            for i in 0..5 {
                println!("hilo 2");
                grid_generator::add_vehicle(&city_simulation.sim);
            }
        }

        unreachable!();
    }




    let mut thread1 = MyPthreads::new();
    let mut thread2 = MyPthreads::new();
    thread1.my_thread_create(context_function1, sim_usize, 0, 0, SchedulingAlgorithm::RoundRobin);
    thread2.my_thread_create(context_function2, 0, 0, 100, SchedulingAlgorithm::RoundRobin);

    unsafe {
        run_master();
    }

}
