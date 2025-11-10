mod grid_generator;
mod architect;
mod city;
mod mymutex;
mod vehicle;
mod simulation;

use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{Application};

const APP_ID: &str = "org.gtk.city";

fn main() -> glib::ExitCode {
    let simulation = Application::builder().application_id(APP_ID).build();
    simulation.connect_activate(grid_generator::generate_grid);
    simulation.run()
}
