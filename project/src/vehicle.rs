use std::collections::VecDeque;
use crate::city::StreetId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VehicleType {
    Car,
    Ambulance,
    Truck
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vehicle {
    pub lane: usize,
    pub vehicle_type: VehicleType,
    pub current_street: StreetId,
    pub route: VecDeque<StreetId>
}

impl Vehicle {
    pub fn new(vehicle_type: VehicleType, mut route: VecDeque<StreetId>, lane: usize) -> Self {
        Self {
            lane,
            vehicle_type,
            current_street: route.pop_front().unwrap(),
            route
        }
    }

    pub fn set_street(&mut self, street: StreetId) {
        self.current_street = street;
    }

    pub fn next_street(&self) -> Option<&StreetId> {
        self.route.front()
    }

    pub fn move_forward(&mut self){
        let next_street = self.route.pop_front().unwrap();
        self.set_street(next_street);
    }
}
