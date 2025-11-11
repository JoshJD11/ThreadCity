use crate::city::{City, StreetId};
use crate::vehicle::{Vehicle, VehicleType::Car};
use std::collections::VecDeque;
use rand::Rng;
use rand::seq::IndexedRandom;

pub struct Simulation {
    pub city: City,
    pub active_vehicles: VecDeque<Vehicle>,
    pub vehicles_queue: VecDeque<Vehicle>,
}

impl Simulation {
    pub fn new(city: City) -> Self {
        Self {
            city,
            active_vehicles: VecDeque::new(),
            vehicles_queue: VecDeque::new(),
        }
    }

    pub fn update(&mut self) {
        let mut vehicles_to_delete = Vec::new();
        for (i, vehicle) in self.active_vehicles.iter_mut().enumerate() {
            let current_street = vehicle.current_street;
            let lane = vehicle.lane;
            let current_street_mutexes = self.city.street_map.streets.get(&current_street).unwrap();
            let current_street_mutex = if lane == 0 {&current_street_mutexes.0} else {&current_street_mutexes.1};
            println!("Current street: {:?}", current_street); // debug
            if let Some(next_street) = vehicle.next_street() {
                println!("Next street: {:?}", next_street); // debug
                let next_street_mutexes =  self.city.street_map.streets.get(&next_street).unwrap();
                let next_street_mutex = if lane == 0 {&next_street_mutexes.0} else {&next_street_mutexes.1};
                next_street_mutex.lock();
                vehicle.move_forward();
            } else {
                vehicles_to_delete.push(i);
            }
            current_street_mutex.unlock();
        }
        for index in vehicles_to_delete {
            self.active_vehicles.remove(index);
        }
    }

    pub fn queue_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles_queue.push_back(vehicle);
    }

    pub fn generate_vehicle(&mut self) {
        let lane = rand::rng().random_range(0 ..= 1);
        self.queue_vehicle(Vehicle::new(Car, self.generate_route(), lane));
    }

    pub fn spawn_vehicle(&mut self) {
        // start by generating a new vehicle
        self.generate_vehicle();
        if !self.vehicles_queue.is_empty() {
            // select the next vehicle to spawn
            let new_vehicle = self.vehicles_queue.pop_front().unwrap();
            let new_vehicle_starting_street = new_vehicle.current_street;

            // lock the starting street
            let starting_street_mutexes = self.city.street_map.streets.get(&new_vehicle_starting_street).unwrap();
            let starting_street_mutex = if new_vehicle.lane == 0 {&starting_street_mutexes.0} else {&starting_street_mutexes.1};
            starting_street_mutex.lock();

            // transfer the vehicle to active_vehicles
            self.active_vehicles.push_back(new_vehicle);
        }
    }

    fn generate_route(&self) -> VecDeque<StreetId> {
        let mut randomizer = rand::rng();
        let mut route: VecDeque<StreetId> = VecDeque::new();

        // choose the starting and the arrival point
        let all_streets: Vec<StreetId> = self.city.get_all_streets().cloned().collect();
        let streets_sample: Vec<StreetId> = all_streets.sample(&mut randomizer, 2).cloned().collect();
        let starting_street = streets_sample[0];
        let arrival_street = streets_sample[1];

        route.push_back(starting_street);

        let mut current_street = self.get_adjacent_street(&starting_street, &arrival_street);
        route.push_back(current_street);

        while current_street != arrival_street {
            current_street = self.get_adjacent_street(&current_street, &arrival_street);
            route.push_back(current_street);
        }

        println!("S = {:?}", starting_street);
        println!("A = {:?}", arrival_street);

        route
    }

    fn get_adjacent_street(&self, current_street: &StreetId, arrival_street: &StreetId) -> StreetId {
        let (current_row, current_column) = current_street.get_position();
        let (arrival_row, arrival_column) = arrival_street.get_position();

        if arrival_street.is_horizontal() {
            if current_street.is_horizontal() {
                if current_row != arrival_row {
                    StreetId::Vertical {row: current_row.saturating_sub(1), column: current_column}
                } else {
                    StreetId::Horizontal {row: current_row,
                        column: if arrival_column > current_column {current_column + 1} else {current_column - 1}}
                }
            } else {
                if current_row != arrival_row && (current_row + 1) != arrival_row {
                    StreetId::Vertical {
                        row: if arrival_row > current_row {current_row + 1} else {current_row - 1},
                        column: current_column}
                } else {
                    StreetId::Horizontal {
                        row: if arrival_row > current_row {current_row + 1} else {current_row},
                        column: if arrival_column < current_column {current_column - 1} else {current_column}}
                }
            }
        } else { // arrival street is vertical
            if current_street.is_vertical() {
                if current_column != arrival_column {
                    StreetId::Horizontal {row: current_row, column: current_column.saturating_sub(1)}
                } else {
                    StreetId::Vertical {
                        row: if arrival_row > current_row {current_row + 1} else {current_row - 1},
                        column: current_column
                    }
                }
            } else {
                if current_column != arrival_column && (current_column + 1) != arrival_column {
                    StreetId::Horizontal {row: current_row,
                        column: if arrival_column > current_column {current_column + 1} else {current_column - 1}
                    }
                } else {
                    StreetId::Vertical {
                        row: current_row.saturating_sub(1),
                        column: if arrival_column > current_column {current_column + 1} else {current_column}
                    }
                }
            }
        }
    }

    pub fn get_active_vehicle_count(&self) -> usize {
        self.active_vehicles.len()
    }

    pub fn get_queued_count(&self) -> usize {
        self.vehicles_queue.len()
    }
}
