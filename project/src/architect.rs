// Transform logical position (row, column) to screen coordinates (x, y)

use crate::city::{StreetId, Node};

pub struct CityLayout {
    size: usize,
    origin_x: f64,
    origin_y: f64,
    step_x: f64,
    step_y: f64,
}

impl CityLayout {
    pub fn new(size: usize, width: f64, height: f64, margin: f64) -> Self {
        assert!(size > 0, "CityLayout::new: size must be > 0");
        let origin_x = margin;
        let origin_y = margin;
        let inner_width = width - margin * 2.0;
        let inner_height = height - margin * 2.0;
        let step_x = inner_width / size as f64;
        let step_y = inner_height / size as f64;

        Self {
            size,
            origin_x,
            origin_y,
            step_x,
            step_y,
        }
    }

    fn node_to_coordinate(&self, node: Node) -> (f64, f64) {
        let (row, column) = node;
        debug_assert!(row <= self.size && column <= self.size, "node out of range");
        let x = self.origin_x + self.step_x * column as f64;
        let y = self.origin_y + self.step_y * row as f64;
        (x, y)
    }

    pub fn street_to_screen(&self, street: StreetId) -> ((f64, f64), (f64, f64)) {
        match street {
            StreetId::Horizontal { row, column } => {
                let a = self.node_to_coordinate((row, column));
                let b = self.node_to_coordinate((row, column + 1));
                (a, b)
            }
            StreetId::Vertical { row, column } => {
                let a = self.node_to_coordinate((row, column));
                let b = self.node_to_coordinate((row + 1, column));
                (a, b)
            }
        }
    }
}
