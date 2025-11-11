use std::collections::{HashMap};
use crate::mymutex::MyMutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    pub fn get_position(&self) -> (usize, usize) {
        match self {
            StreetId::Horizontal { row, column } => (*row, *column),
            StreetId::Vertical { row, column } => (*row, *column),
        }
    }
}

pub struct StreetMap {
    pub streets: HashMap<StreetId, (MyMutex, MyMutex)>
}

impl StreetMap {
    pub fn new(size: usize) -> Self {
        let mut streets = HashMap::with_capacity(2 * size * (size + 1));
        for row in 0 ..= size {
            for column in 0 .. size {
                streets.insert(StreetId::Horizontal { row, column }, (MyMutex::new(), MyMutex::new()));
            }
        }
        for row in 0 .. size {
            for column in 0 ..= size {
                streets.insert(StreetId::Vertical { row, column }, (MyMutex::new(), MyMutex::new()));
            }
        }
        Self { streets }
    }
}

pub struct City {
    size: usize,
    pub street_map: StreetMap
}

impl City {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "City::new: n must be > 0");
        Self {
            size,
            street_map: StreetMap::new(size)
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn street_count(&self) -> usize {
        2 * self.size * (self.size + 1)
    }

    pub fn get_all_streets(&self) -> impl Iterator<Item=&StreetId> {
        self.street_map.streets.keys()
    }
}
