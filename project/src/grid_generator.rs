use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, Label, Box as GtkBox, Orientation, EventControllerKey};
use cairo::Context;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

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


pub fn create_simulation() -> Rc<RefCell<Simulation>> {
    let city = City::new(GRID);
    Rc::new(RefCell::new(Simulation::new(city)))
}

pub fn add_vehicle(simulation: &Rc<RefCell<Simulation>>) {
    simulation.borrow_mut().spawn_vehicle();
}

pub fn generate_grid(app: &Application, simulation: &Rc<RefCell<Simulation>>) {
    // Create the city and simulation
    let city = City::new(GRID);
    //let simulation = Rc::new(RefCell::new(Simulation::new(city)));

    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("City Traffic Simulation")
        .default_width(WIDTH)
        .default_height(HEIGHT + 150) // Extra space for buttons and info
        .build();

    // Create main vertical box
    let vbox = GtkBox::new(Orientation::Vertical, 5);
    window.set_child(Some(&vbox));

    // Create info label
    let info_label = Label::new(None);
    info_label.set_text("Active vehicles: 0 | Queued vehicles: 0");
    vbox.append(&info_label);

    // Create drawing area
    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(WIDTH, HEIGHT);
    vbox.append(&drawing_area);

    // Clone simulation for the draw callback
    let simulation_draw = simulation.clone();
    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        // Clear background
        cr.set_source_rgb(BG.0, BG.1, BG.2);
        cr.paint().unwrap();

        // Create city layout
        let layout = CityLayout::new(GRID, WIDTH as f64, HEIGHT as f64, MARGIN);

        // Draw streets
        cr.set_source_rgb(STREET_COLOR.0, STREET_COLOR.1, STREET_COLOR.2);
        cr.set_line_width(LINE_WIDTH);

        for street in simulation_draw.borrow().city.get_all_streets() {
            let ((x1, y1), (x2, y2)) = layout.street_to_screen(street);

            cr.move_to(x1, y1);
            cr.line_to(x2, y2);
            cr.stroke().unwrap();
        }

        // Draw vehicles
        for vehicle in &simulation_draw.borrow().active_vehicles {
            let color = match vehicle.vehicle_type {
                VehicleType::Car => CAR_COLOR,
                VehicleType::Ambulance => AMBULANCE_COLOR,
                VehicleType::Truck => TRUCK_COLOR,
            };

            cr.set_source_rgb(color.0, color.1, color.2);

            // Get vehicle position on current street
            let ((x1, y1), (x2, y2)) = layout.street_to_screen(&vehicle.current_street);

            // Calculate vehicle position (middle of the street segment)
            let vehicle_x = (x1 + x2) / 2.0;
            let vehicle_y = (y1 + y2) / 2.0;

            // Draw vehicle as a small rectangle
            let vehicle_size = LINE_WIDTH / 2.0;
            cr.rectangle(
                vehicle_x - vehicle_size / 2.0,
                vehicle_y - vehicle_size / 2.0,
                vehicle_size,
                vehicle_size
            );
            cr.fill().unwrap();

            // === Dibujar el número de lane encima ===
            cr.set_source_rgb(0.0, 0.0, 0.0); // texto negro
            cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
            cr.set_font_size(12.0);

            let lane_text = format!("{}", vehicle.lane);
            let text_extents = cr.text_extents(&lane_text).unwrap();
            let text_x = vehicle_x - text_extents.width() / 2.0 - text_extents.x_bearing();
            let text_y = vehicle_y - vehicle_size / 2.0 + 12.0; // un poco arriba del auto

            cr.move_to(text_x, text_y);
            cr.show_text(&lane_text).unwrap();
        }
    }); // Note: removed Inhibit completely for draw function

    //spawn_vehicle(simulation.clone());
    update(simulation.clone(), drawing_area.clone(), info_label.clone());
    traffic_officer(simulation.clone());

    window.present();
}

pub fn spawn_vehicle(simulation: Rc<RefCell<Simulation>>) {
    glib::timeout_add_local(Duration::from_millis(250), move || {
        simulation.borrow_mut().spawn_vehicle();
        glib::ControlFlow::Continue
    });
}

pub fn update(simulation: Rc<RefCell<Simulation>>, drawing_area: DrawingArea, info_label: Label) {
    glib::timeout_add_local(Duration::from_millis(125), move || {
        simulation.borrow_mut().update();

        // Update info label
        let active_count = simulation.borrow().get_active_vehicle_count();
        let queued_count = simulation.borrow().get_queued_count();
        info_label.set_text(&format!(
            "Active vehicles: {} | Queued vehicles: {}",
            active_count, queued_count
        ));

        // Request redraw
        drawing_area.queue_draw();

        glib::ControlFlow::Continue
    });
}

pub fn traffic_officer(simulation: Rc<RefCell<Simulation>>) {
    glib::timeout_add_local(Duration::from_millis(2500), move || {
        simulation.borrow_mut().clear_active_vehicles();
        glib::ControlFlow::Continue
    });
}
