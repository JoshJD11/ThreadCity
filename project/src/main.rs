mod grid_generator;
mod architect;
mod city;
mod vehicle;
mod simulation;
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

use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{Application};
use crate::city::City;
use crate::simulation::Simulation;
use crate::vehicle::Vehicle;
use mythread::MyThread;
use scheduler::Scheduler;
use mypthreads::MyPthreads;
use context::Transfer;
use crate::types::SchedulingAlgorithm;
use crate::mypthreads::run_master;

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
    const THE_NUMBER_OF_THE_BEAST: usize = 666;

    loop {
        println!("Select Test:");
        println!("1) MyPthread Library");
        println!("2) ThreadCity Simulation");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        let test_option: i32 = input.trim().parse().unwrap_or(0);

        match test_option {
            1 => {
                extern "C" fn context_function1(mut t: Transfer) -> ! {
                    println!("thread 1 yield");
                    t = MyPthreads::my_thread_yield(t, 0);
                    unsafe {
                        t = MyPthreads::my_thread_yield(t, THE_NUMBER_OF_THE_BEAST);
                    }
                    unreachable!();
                }

                extern "C" fn context_function2(mut t: Transfer) -> ! {
                    for i in 0usize..20 {
                        println!("thread 2 Currently at:  {}", i);
                        if timer::should_preempt() {
                            println!("thread 2 Preempting at {}", i);
                            t = unsafe { t.context.resume(0) };
                        }
                    }
                    unsafe {
                        t.context.resume(THE_NUMBER_OF_THE_BEAST);
                    }
                    unreachable!();
                }

                extern "C" fn context_function3(mut t: Transfer) -> ! {
                    for i in 0usize..20 {
                        println!("thread 3 Currently at:  {}", i);
                        if timer::should_preempt() {
                            println!("thread 3 Preempting at {}", i);
                            t = unsafe { t.context.resume(0) };
                        }
                    }
                    unsafe {
                        t.context.resume(THE_NUMBER_OF_THE_BEAST);
                    }
                    unreachable!();
                }

                let mut thread1 = MyPthreads::new();
                let mut thread2 = MyPthreads::new();
                let mut thread3 = MyPthreads::new();
                thread1.my_thread_create( context_function1, 0, 0, 0, SchedulingAlgorithm::RoundRobin);
                thread2.my_thread_create( context_function2, 0, 5, 20,  SchedulingAlgorithm::Lottery);
                thread3.my_thread_create( context_function3, 0, 10, 3, SchedulingAlgorithm::RealTime);
                thread1.my_thread_chsched(SchedulingAlgorithm::Lottery);
                thread1.my_thread_end();

                unsafe {
                    run_master();
                }
            }
            2 => {
                let mut simulation = CitySimulation::new();
                let sim_ptr = &mut simulation as *mut CitySimulation; // for mutable access
                let sim_usize = sim_ptr as usize;

                extern "C" fn context_function1(mut t: Transfer) -> ! {
                    let city_sim_ptr = t.data as *const CitySimulation;
                    unsafe {
                        let city_simulation = &*city_sim_ptr;
                        //grid_generator::traffic_officer(&city_simulation.sim);
                        for _ in 0..10 {
                            println!("Thread #1");
                            grid_generator::add_vehicle(&city_simulation.sim);
                        }
                        t = MyPthreads::my_thread_yield(t, 0);
                    }
                    unsafe {
                        t.context.resume(THE_NUMBER_OF_THE_BEAST);
                    }
                    unreachable!();
                }

                extern "C" fn context_function2(mut t: Transfer) -> ! {
                    let city_sim_ptr = t.data as *const CitySimulation;
                    unsafe {
                        println!("Thread #2");
                        let city_simulation = &*city_sim_ptr;
                        let simulation_app = Application::builder().application_id(APP_ID).build();
                        simulation_app.connect_activate(move |app| {
                            grid_generator::generate_grid(app, &city_simulation.sim);
                        });
                        simulation_app.run();
                        t = MyPthreads::my_thread_yield(t, 0);
                    }
                    unsafe {
                        t.context.resume(THE_NUMBER_OF_THE_BEAST);
                    }
                    unreachable!();
                }

                loop {
                    // we instantiate the threads
                    let mut thread1 = MyPthreads::new();
                    let mut thread2 = MyPthreads::new();
                    // assign the threads
                    thread1.my_thread_create(context_function1, sim_usize, 0, 0, SchedulingAlgorithm::RoundRobin);
                    thread2.my_thread_create(context_function2, sim_usize, 0, 100, SchedulingAlgorithm::RoundRobin);
                    // run the threads
                    unsafe {
                        run_master();
                    }
                }
            }
            _ => println!("Invalid test option ¿are you dumb?")
        }
    }
}
