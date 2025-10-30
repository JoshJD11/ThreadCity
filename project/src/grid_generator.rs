use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};

use crate::city::{CityGraph, StreetId};
use crate::architect::CityLayout;

use pango::FontDescription;
use pangocairo::functions as surveyors; // easy to understand

// Canvas configuration
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const MARGIN: i32 = 75;

// Grid configuration
const GRID: i32 = 5; // n*n blocks
const LINE_WIDTH: f64 = 50.0;

// Colors
const BG: (f64, f64, f64) = (0.8, 0.8, 0.8);
const STREET_COLOR: (f64, f64, f64) = (0.0, 0.0, 0.0);
const LABEL_COLOR: (f64, f64, f64) = (1.0, 0.5, 0.3);

// Text configuration
const FONT: &str = "Monospace 10";

pub fn generate_grid(simulation: &Application) {
    let city_graph = CityGraph::new(GRID as usize);

    let window = DrawingArea::builder()
        .content_width(WIDTH)
        .content_height(HEIGHT)
        .build();

    window.set_draw_func(move |_, canvas, width, height| {

        // set background color
        canvas.set_source_rgb(BG.0, BG.1, BG.2);
        canvas.paint().unwrap();

        let city_layout = CityLayout::new(city_graph.size(), width as f64, height as f64, MARGIN as f64);

        // set streets specs
        canvas.set_line_width(LINE_WIDTH);
        canvas.set_source_rgb(STREET_COLOR.0, STREET_COLOR.1, STREET_COLOR.2);

        for street in city_graph.get_all_streets() {
            let ((x0, y0), (x1, y1)) = city_layout.street_to_screen(street);
            canvas.move_to(x0, y0);
            canvas.line_to(x1, y1);
        }
        canvas.stroke().unwrap();

        let text_layout = surveyors::create_layout(canvas);

        // set label text specs
        text_layout.set_font_description(Some(&FontDescription::from_string(FONT)));
        canvas.set_source_rgb(LABEL_COLOR.0, LABEL_COLOR.1, LABEL_COLOR.2);

        for street in city_graph.get_all_streets() {
            let ((x0, y0), (x1, y1)) = city_layout.street_to_screen(street);

            let midpoint_x = (x0 + x1) * 0.5;
            let midpoint_y = (y0 + y1) * 0.5;

            let label = match street {
                StreetId::Horizontal { row, column } => format!("H({row},{column})"),
                StreetId::Vertical   { row, column } => format!("V({row},{column})")
            };
            let (offset_x, offset_y) = (-23.0, -10.0);
            text_layout.set_text(&label);
            canvas.move_to(midpoint_x + offset_x, midpoint_y + offset_y);
            surveyors::show_layout(canvas, &text_layout);
        }
    });

    let app = ApplicationWindow::builder()
        .application(simulation)
        .title("Grid")
        .default_width(WIDTH)
        .default_height(HEIGHT)
        .child(&window)
        .build();

    app.present();
}
