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
const CAR_COLOR: (f64, f64, f64) = (0.0, 1.0, 0.0);
const AMBULANCE_COLOR: (f64, f64, f64) = (1.0, 0.0, 0.0);
const TRUCK_COLOR: (f64, f64, f64) = (1.0, 1.0, 0.0);

// Colors - después de los colores existentes
const BOAT_COLOR: (f64, f64, f64) = (0.0, 0.0, 1.0); // Azul

// Estructura para barcos decorativos - agrega esto después de los imports
#[derive(Clone)]
struct DecorativeBoat {
    position: f64,
    speed: f64,
    direction: bool,
    offset: f64
}

impl DecorativeBoat {
    fn new(start_position: f64, speed: f64, direction: bool, offset: f64) -> Self {
        Self {
            position: start_position,
            speed,
            direction,
            offset
        }
    }

    fn update(&mut self) {
        if self.direction {
            self.position += self.speed;
        } else {
            self.position -= self.speed;
        }

        // Wrap around if position goes beyond 0.0-1.0 range
        if self.position > 1.0 {
            self.position -= 1.0;
        } else if self.position < 0.0 {
            self.position += 1.0;
        }
    }

    fn position(&self) -> (f64, f64) {
        let perimeter = 2.0 * ((WIDTH as f64 - 2.0 * MARGIN) + (HEIGHT as f64 - 2.0 * MARGIN));
        let current_distance = self.position * perimeter;

        let top_length = WIDTH as f64 - 2.0 * MARGIN;
        let right_length = HEIGHT as f64 - 2.0 * MARGIN;
        let bottom_length = top_length;
        let left_length = right_length;

        // Aplicar el offset en todas las posiciones
        let offset = self.offset;

        // Calculate position along the margin perimeter with offset
        if current_distance < top_length {
            // Top edge - aplicar offset en Y
            (MARGIN + current_distance, MARGIN - offset)
        } else if current_distance < top_length + right_length {
            // Right edge - aplicar offset en X
            (WIDTH as f64 - MARGIN + offset, MARGIN + (current_distance - top_length))
        } else if current_distance < top_length + right_length + bottom_length {
            // Bottom edge - aplicar offset en Y
            let dist = current_distance - (top_length + right_length);
            (WIDTH as f64 - MARGIN - dist, HEIGHT as f64 - MARGIN + offset)
        } else {
            // Left edge - aplicar offset en X
            let dist = current_distance - (top_length + right_length + bottom_length);
            (MARGIN - offset, HEIGHT as f64 - MARGIN - dist)
        }
    }
}

// Color para la planta nuclear
const NUCLEAR_PLANT_COLOR: (f64, f64, f64) = (0.5, 0.0, 0.5); // Morado

// Estructura para la planta nuclear
struct NuclearPlant {
    position: (f64, f64), // Posición fija (x, y)
    size: f64, // Tamaño de la planta
}

impl NuclearPlant {
    fn new(position: (f64, f64), size: f64) -> Self {
        Self {
            position,
            size
        }
    }

    fn draw(&self, cr: &Context) {
        let (x, y) = self.position;

        // Dibujar el edificio principal (rectángulo)
        cr.set_source_rgb(NUCLEAR_PLANT_COLOR.0, NUCLEAR_PLANT_COLOR.1, NUCLEAR_PLANT_COLOR.2);
        cr.rectangle(
            x - self.size / 2.0,
            y - self.size / 2.0,
            self.size,
            self.size
        );
        cr.fill().unwrap();

        // Dibujar las torres de refrigeración (círculos)
        cr.set_source_rgb(0.3, 0.3, 0.3); // Color gris para las torres
        let tower_radius = self.size / 6.0;

        // Torre izquierda
        cr.arc(x - self.size / 3.0, y, tower_radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().unwrap();

        // Torre derecha
        cr.arc(x + self.size / 3.0, y, tower_radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().unwrap();

        // Dibujar humo (círculos más suaves)
        cr.set_source_rgba(0.7, 0.7, 0.7, 0.6); // Humo semi-transparente

        // Humo de la torre izquierda
        cr.arc(x - self.size / 3.0, y - self.size / 3.0, tower_radius * 1.2, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().unwrap();

        // Humo de la torre derecha
        cr.arc(x + self.size / 3.0, y - self.size / 3.0, tower_radius * 1.2, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().unwrap();

        // Etiqueta de texto "NUCLEAR"
        cr.set_source_rgb(1.0, 1.0, 1.0); // Texto blanco
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(14.0);

        let text = "NUCLEAR";
        let text_extents = cr.text_extents(text).unwrap();
        let text_x = x - text_extents.width() / 2.0 - text_extents.x_bearing();
        let text_y = y + self.size / 2.0 + 20.0;

        cr.move_to(text_x, text_y);
        cr.show_text(text).unwrap();
    }
}

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
    // Crear barcos decorativos que navegan por el margen
    // Crear barcos decorativos que navegan por el margen con diferentes offsets
    let decorative_boats = Rc::new(RefCell::new(vec![
        DecorativeBoat::new(0.0, 0.0005, true, 90.0),
        DecorativeBoat::new(0.25, 0.001, false, 70.0),
        DecorativeBoat::new(0.5, 0.0003, true, 100.0),
        DecorativeBoat::new(0.75, 0.001, false, 125.0)
    ]));
    let decorative_boats_clone = decorative_boats.clone();
    // Crear planta nuclear
    let nuclear_plant_1 = Rc::new(NuclearPlant::new(
        (WIDTH as f64 / 3.075, HEIGHT as f64 / 3.075), // Centro del canvas
        80.0
    ));
    let nuclear_plant_2 = Rc::new(NuclearPlant::new(
        (WIDTH as f64 / 1.484, HEIGHT as f64 / 1.484), // Centro del canvas
        80.0
    ));
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
        // Dibujar planta nuclear (en el fondo)
        nuclear_plant_1.draw(&cr);
        nuclear_plant_2.draw(&cr);
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
        // Dibujar barcos decorativos
        let boats = decorative_boats.borrow();

        cr.set_source_rgb(BOAT_COLOR.0, BOAT_COLOR.1, BOAT_COLOR.2);
        for boat in boats.iter() {
            let (x, y) = boat.position();

            // Dibujar barco más elaborado
            let boat_size = LINE_WIDTH * 1.2;
            cr.move_to(x, y);
            cr.line_to(x, y - boat_size);
            cr.stroke().unwrap();
        }
    });

    spawn_vehicle(simulation.clone());
    update(simulation.clone(), drawing_area.clone(), info_label.clone());
    traffic_officer(simulation.clone());
    // Actualizar barcos decorativos
    let drawing_area_clone = drawing_area.clone();
    glib::timeout_add_local(Duration::from_millis(50), move || {
        for boat in decorative_boats_clone.borrow_mut().iter_mut() {
            boat.update();
        }
        drawing_area_clone.queue_draw();
        glib::ControlFlow::Continue
    });

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
