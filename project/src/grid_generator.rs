use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, Button, Box as GtkBox, Orientation, Label};
use gtk::cairo::Context;
use crate::city::{City, StreetId};
use crate::architect::CityLayout;
use crate::vehicle::{Vehicle, VehicleType};
use crate::simulation::Simulation;

// Canvas configuration
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const MARGIN: f64 = 150.0;

// Grid configuration
const GRID: usize = 6; // n*n blocks
const LINE_WIDTH: f64 = 30.0;

// Colors
const BG: (f64, f64, f64) = (0.8, 0.8, 0.8);
const STREET_COLOR: (f64, f64, f64) = (0.0, 0.0, 0.0);
const CAR_COLOR: (f64, f64, f64) = (0.0, 0.7, 0.0);
const AMBULANCE_COLOR: (f64, f64, f64) = (1.0, 0.0, 0.0);
const TRUCK_COLOR: (f64, f64, f64) = (1.0, 1.0, 0.0);

pub fn generate_grid(app: &Application) {
    // Create the city and simulation
    let city = City::new(GRID);
    let simulation = Rc::new(RefCell::new(Simulation::new(city)));

    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("City Traffic Simulation")
        .default_width(WIDTH)
        .default_height(HEIGHT + 150) // Extra space for buttons and info
        .build();

    // Create a vertical box to hold the drawing area and the buttons
    let vbox = GtkBox::new(Orientation::Vertical, 5);
    window.set_child(Some(&vbox));

    // Create the drawing area
    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(WIDTH, HEIGHT);
    vbox.append(&drawing_area);

    // Create info label
    let info_label = Label::new(None);
    vbox.append(&info_label);

    // Create a horizontal box for buttons
    let button_box = GtkBox::new(Orientation::Horizontal, 5);
    vbox.append(&button_box);

    // Create buttons
    let spawn_button = Button::with_label("Spawn Vehicle");
    let car_button = Button::with_label("Generate Car");
    let ambulance_button = Button::with_label("Generate Ambulance");
    let truck_button = Button::with_label("Generate Truck");

    button_box.append(&spawn_button);
    button_box.append(&car_button);
    button_box.append(&ambulance_button);
    button_box.append(&truck_button);

    // Set up button click handlers
    let simulation_clone = simulation.clone();
    let drawing_area_clone = drawing_area.clone();
    let info_label_clone = info_label.clone();
    spawn_button.connect_clicked(move |_| {
        simulation_clone.borrow_mut().spawn_vehicle();
        drawing_area_clone.queue_draw();

        // Update info label
        let sim = simulation_clone.borrow();
        info_label_clone.set_text(&format!(
            "Active vehicles: {} | Queued vehicles: {}",
            sim.get_active_vehicle_count(),
            sim.get_queued_count()
        ));
    });

    let simulation_car = simulation.clone();
    let drawing_area_car = drawing_area.clone();
    let info_label_car = info_label.clone();
    car_button.connect_clicked(move |_| {
        simulation_car.borrow_mut().generate_vehicle();
        drawing_area_car.queue_draw();

        // Update info label
        let sim = simulation_car.borrow();
        info_label_car.set_text(&format!(
            "Active vehicles: {} | Queued vehicles: {}",
            sim.get_active_vehicle_count(),
            sim.get_queued_count()
        ));
    });

    let simulation_ambulance = simulation.clone();
    let drawing_area_ambulance = drawing_area.clone();
    let info_label_ambulance = info_label.clone();
    ambulance_button.connect_clicked(move |_| {
        // For now, we'll use Car type since we don't have ambulance route generation
        // In a real implementation, you'd have generate_ambulance() method
        simulation_ambulance.borrow_mut().generate_vehicle();
        drawing_area_ambulance.queue_draw();

        // Update info label
        let sim = simulation_ambulance.borrow();
        info_label_ambulance.set_text(&format!(
            "Active vehicles: {} | Queued vehicles: {}",
            sim.get_active_vehicle_count(),
            sim.get_queued_count()
        ));
    });

    let simulation_truck = simulation.clone();
    let drawing_area_truck = drawing_area.clone();
    let info_label_truck = info_label.clone();
    truck_button.connect_clicked(move |_| {
        // For now, we'll use Car type since we don't have truck route generation
        // In a real implementation, you'd have generate_truck() method
        simulation_truck.borrow_mut().generate_vehicle();
        drawing_area_truck.queue_draw();

        // Update info label
        let sim = simulation_truck.borrow();
        info_label_truck.set_text(&format!(
            "Active vehicles: {} | Queued vehicles: {}",
            sim.get_active_vehicle_count(),
            sim.get_queued_count()
        ));
    });

    // Set up the draw function
    let simulation_draw = simulation.clone();
    drawing_area.set_draw_func(move |_, cr, width, height| {
        // Clear the background
        cr.set_source_rgb(BG.0, BG.1, BG.2);
        cr.paint().expect("Paint failed");

        // Create a city layout helper
        let layout = CityLayout::new(GRID, width as f64, height as f64, MARGIN);

        // Draw streets
        cr.set_line_width(LINE_WIDTH);
        cr.set_source_rgb(STREET_COLOR.0, STREET_COLOR.1, STREET_COLOR.2);

        let sim = simulation_draw.borrow();
        for street in sim.city.get_all_streets() {
            let (start, end) = layout.street_to_screen(street);
            cr.move_to(start.0, start.1);
            cr.line_to(end.0, end.1);
            cr.stroke().expect("Stroke failed");
        }

        // Draw vehicles
        for vehicle in sim.active_vehicles.iter() {
            let (start, end) = layout.street_to_screen(&vehicle.current_street);
            // Draw vehicle as a circle in the middle of the street
            let x = (start.0 + end.0) / 2.0;
            let y = (start.1 + end.1) / 2.0;

            // Set color based on vehicle type
            match vehicle.vehicle_type {
                VehicleType::Car => cr.set_source_rgb(CAR_COLOR.0, CAR_COLOR.1, CAR_COLOR.2),
                VehicleType::Ambulance => cr.set_source_rgb(AMBULANCE_COLOR.0, AMBULANCE_COLOR.1, AMBULANCE_COLOR.2),
                VehicleType::Truck => cr.set_source_rgb(TRUCK_COLOR.0, TRUCK_COLOR.1, TRUCK_COLOR.2),
            }

            cr.arc(x, y, 8.0, 0.0, 2.0 * std::f64::consts::PI);
            cr.fill().expect("Fill failed");
        }
    });

    // Set up animation timer - update simulation every second
    let simulation_timer = simulation.clone();
    let drawing_area_timer = drawing_area.clone();
    let info_label_timer = info_label.clone();
    glib::timeout_add_seconds_local(1, move || {
        simulation_timer.borrow_mut().update();
        drawing_area_timer.queue_draw();

        // Update info label
        let sim = simulation_timer.borrow();
        info_label_timer.set_text(&format!(
            "Active vehicles: {} | Queued vehicles: {}",
            sim.get_active_vehicle_count(),
            sim.get_queued_count()
        ));

        glib::ControlFlow::Continue
    });

    // Initial info label update
    let sim = simulation.borrow();
    info_label.set_text(&format!(
        "Active vehicles: {} | Queued vehicles: {}",
        sim.get_active_vehicle_count(),
        sim.get_queued_count()
    ));

    window.present();
}
