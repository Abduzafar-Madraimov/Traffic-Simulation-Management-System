// vehicle.rs
use std::sync::atomic::Ordering;
use rand::Rng;

// use crate::variables::{CAR_ID_COUNTER, GRID_HEIGHT, GRID_WIDTH};

use super::variables::{GRID_HEIGHT, GRID_WIDTH, CAR_ID_COUNTER};

#[derive(Clone, Debug)]
pub enum VehicleType {
    Car,
    Bus,
    Emergency,
}

#[derive(Clone)]
pub struct Vehicle {
    pub id: u64,
    // Use visual identifier (e.g., 'C', 'B', 'E')
    pub vehicle_type: VehicleType,
    pub current_position: (i32, i32),
    // Units per second
    pub current_speed: i32,
    pub max_speed: i32,
    pub destination: (i32, i32),
    pub priority: u8,
}

impl Vehicle {
    pub fn new(
        id: u64,
        vehicle_type: VehicleType,
        current_position: (i32, i32),
        // Units per second
        current_speed: i32,
        max_speed: i32,
        destination: (i32, i32),
        priority: u8,
    ) -> Vehicle {
        return Vehicle {
            id,
            vehicle_type,
            current_speed,
            max_speed,
            current_position,
            destination,
            priority,
        };
    }

    pub async fn generate_vehicle() -> Self {
        tokio::task::spawn_blocking(move || {
            // randomly generate the car type
            let mut rng = rand::rng();
            // The range 1..=3 generates numbers from 1 to 3 (inclusive)
            let num = rng.random_range(1..=3);

            // Match the random number to a vehicle type
            let vehicle_type = match num {
                1 => VehicleType::Car,
                2 => VehicleType::Bus,
                3 => VehicleType::Emergency,
                // since num is always between 1 and 3
                _ => unreachable!(), 
            };

            // Generate poisition
            let x_position = rng.random_range(0..=(GRID_WIDTH * 10));
            let y_position =
            // If x is not 0 and not divisible by 10, then generate y as a multiple of 10.
            if x_position != 0 || x_position % 10 != 0 {
                rng.random_range(0..=GRID_HEIGHT) * 10 
            } else {
                rng.random_range(0..=GRID_HEIGHT * 10)
            };

            // Generate speed based on the car type
            let speed = match num {
                1 => 2,
                2 => 1,
                3 => 3,
                _ => unreachable!(),
            };

            // Generate prioirty based on the car type (higher is better)
            let priority = match num {
                1 => 1,
                2 => 2,
                3 => 3,
                _ => unreachable!(),
            };

            // Generate destination (both should be divisible by 10 or 0)
            let x_final = rng.random_range(0..=GRID_WIDTH) * 10;
            let y_final = rng.random_range(0..=GRID_HEIGHT) * 10;

            // Create a vehicle
            Vehicle::new(
                CAR_ID_COUNTER.fetch_add(1, Ordering::Relaxed), 
                vehicle_type, 
                (x_position, y_position), 
                speed, 
                // When car just generated current speed is same as max speed.
                speed, 
                (x_final,y_final), 
                priority,
            )
        }).await.expect("Vehicle generation task failed")
    }

    pub async fn update(&mut self) {
        // Car at destination
        if self.current_position == self.destination {
            return 
        }

        // Car needs to move on y only
        if self.current_position.0 == self.destination.0{
            let distance = self.destination.1 - self.current_position.1;
            if self.current_speed <= distance {
                self.current_position.1 += self.current_speed;
            } else {
                // To make sure that car doesn't go beyond destination
                self.current_position.1 = self.destination.1;
            }
        } 

        // Car needs to move on x only
        else if self.current_position.1 == self.destination.1 {
            let distance = self.destination.0 - self.current_position.0;
            if self.current_speed <= distance {
                self.current_position.0 += self.current_speed;
            } else {
                self.current_position.0 = self.destination.0;
            }
        } 
        
        // Car needs to move on x and y
        else if self.current_position.1 == 0 || self.current_position.1 % 10 == 0 {
            if self.current_position.0 < self.destination.0 {
                // Car moves forward on x untill it same as destination
                let distance_x = self.destination.0 - self.current_position.0;

                if self.current_speed <= distance_x {
                    self.current_position.0 += self.current_speed;
                } else {
                    self.current_position.0 = self.destination.0;
                }
            } else {
                // Car moves back on x untill it same as destination
                let distance_x = self.destination.0 - self.current_position.0;

                if self.current_speed <= distance_x {
                    self.current_position.0 -= self.current_speed;
                } else {
                    self.current_position.0 = self.destination.0;
                }
            }
        }
        else if self.current_position.1 < self.destination.1 {
            // Car moves forward on y untill it same as destination
            let distance_y = self.destination.1 - self.current_position.1;

            if self.current_speed <= distance_y {
                self.current_position.1 += self.current_speed;
            } else {
                self.current_position.1 = self.destination.1;
            }
        } else {
            // Car moves back on y untill it same as destination
            let distance_1 = self.destination.1 - self.current_position.1;

            if self.current_speed <= distance_1 {
                self.current_position.1 -= self.current_speed;
            } else {
                self.current_position.1 = self.destination.1;
            }
        }
    }

}