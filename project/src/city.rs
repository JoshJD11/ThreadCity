// Topological model of the city as a grid graph
//   - N×N blocks -> (N+1)×(N+1) intersection (nodes)

pub type Node = (usize, usize);

#[derive(Clone, Copy)]
pub enum StreetId {
    Horizontal { row: usize, column: usize },
    Vertical { row: usize, column: usize },
}

impl StreetId {
    pub fn is_horizontal(&self) -> bool {
        matches!(self, StreetId::Horizontal { .. })
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, StreetId::Vertical { .. })
    }
}

// intersections are the nodes
// streets are the edges
pub struct CityGraph {
    size: usize,
}

impl CityGraph {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "CityGraph::new: n must be > 0");
        Self { size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn street_count(&self) -> usize {
        2 * self.size * (self.size + 1)
    }

    pub fn get_all_streets(&self) -> Vec<StreetId> {
        let mut streets = Vec::with_capacity(self.street_count());

        for row in 0 ..= self.size {
            for column in 0 .. self.size {
                streets.push(StreetId::Horizontal { row, column });
            }
        }
      
        for row in 0 .. self.size {
            for column in 0 ..= self.size {
                streets.push(StreetId::Vertical { row, column });
            }
        }
        streets
    }
}
