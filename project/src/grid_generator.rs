use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};

// Canvas configuration
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const MARGIN: i32 = 75;

// Grid configuration
const GRID: i32 = 5; // 5x5 blocks
const LINE_WIDTH: f64 = 40.0;

// Colors
const WHITE: (f64, f64, f64) = (0.8, 0.8, 0.8);
const BLACK: (f64, f64, f64) = (0.0, 0.0, 0.0);

pub fn generate_grid(simulation: &Application) {
    let area = DrawingArea::builder()
        .content_width(WIDTH)
        .content_height(HEIGHT)
        .build();

    area.set_draw_func(|_, canvas, width, height| {

        // paint background
        canvas.set_source_rgb(WHITE.0, WHITE.1, WHITE.2);
        canvas.paint().unwrap();

        // set grid specs
        canvas.set_line_width(LINE_WIDTH);
        canvas.set_source_rgb(BLACK.0, BLACK.1, BLACK.2);

        let cell_width  = (width - MARGIN * 2)  / GRID;
        let cell_height = (height - MARGIN * 2) / GRID;

        // x-axis
        for i in 0 ..= GRID {
            let x = (MARGIN + i * cell_width) as f64;
            canvas.move_to(x, MARGIN as f64);
            canvas.line_to(x, (height - MARGIN) as f64);
        }

        // y-axis
        for i in 0 ..= GRID {
            let y = (MARGIN + i * cell_height) as f64;
            canvas.move_to(MARGIN as f64, y);
            canvas.line_to((width - MARGIN) as f64, y);
        }

        canvas.stroke().unwrap();
    });

    let window = ApplicationWindow::builder()
        .application(simulation)
        .title("Grid")
        .default_width(WIDTH)
        .default_height(HEIGHT)
        .child(&area)
        .build();

    window.present();
}
