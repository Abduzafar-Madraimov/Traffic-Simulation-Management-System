// grid.rs
use crate::point::Point;
use crate::vehicle::{Vehicle, VehicleType};
use crate::light::{LightState, TrafficLight};
use std::fmt::{Display, Formatter, Result};
use crate::{GRID_HEIGHT, GRID_WIDTH};

use tokio::sync::{Mutex, mpsc};
use std::sync::Arc;
use tokio::task::JoinSet;

pub struct Grid {
    pub points: Vec<Point>,
    pub vehicles: Vec<Vehicle>,
    pub traffic_lights: Vec<TrafficLight>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            vehicles: Vec::new(),
            traffic_lights: Vec::new(),
        }  
    }

    pub fn generate_grid(mut self, height: i32, width: i32) -> Grid {
        // Generate points for a grid (height x width cells)
        for i in 0..= height - 1 {
            for j in 0..= width - 1 {
                let point = Point {
                    // x should be a column value 
                    x: j * 10,
                    // y should be a row value
                    y: i * 10,
                    // Intersection appears if either x or y middle values
                    // (Not upper bound or lower bound values)
                    is_intersection: (j > 0 && j < width - 1) || ((i > 0 && i < height - 1)),
                };
                
                // Generate trafic light for this point
                match TrafficLight::generate_traffic_light(&point) {
                    Ok(traffic_light) => {
                        self.traffic_lights.push(traffic_light);
                    },
                    Err(_) => {
                    }
                }

                // Add the point to grid
                self.points.push(point);
            }
        }
    
        return self; 
    }

    pub async fn update_vehicles(&mut self) {
        // Create a collection of asynchronous tasks 
        let mut join_set = JoinSet::new();

        // Create a vector to store updated vehicles
        let mut updated_vehicles = Vec::with_capacity(self.vehicles.len());
        for vehicle in &self.vehicles {
            updated_vehicles.push(vehicle.clone());
        }

        // Spawn each vehicle's update task
        for i in 0..updated_vehicles.len() {
            // Move ownership of the vehicle to the task
            let mut vehicle = updated_vehicles[i].clone();
            
            join_set.spawn(async move {
                vehicle.update().await;
                // Return the updated vehicle
                vehicle 
            });
        }

        // Collect updated vehicles
        let mut i = 0;
        // Await the completion of all tasks
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(updated_vehicle) => {
                    if i < self.vehicles.len() {
                        self.vehicles[i] = updated_vehicle;
                        i += 1;
                    }
                },
                Err(e) => {
                    eprintln!("Vehicle update task failed: {}", e);
                }
            }
        }

        // Remove vehicles that have reached their destination
        self.vehicles.retain(|vehicle| vehicle.current_position != vehicle.destination);
    }

    pub async fn update_traffic_lights(&mut self, time_passed: f32) {
        // Update all traffic lights with the elapsed time
        TrafficLight::update_traffic_lights(&mut self.traffic_lights, time_passed).await;
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Group points by rows (x-coordinate)
        let mut rows = std::collections::BTreeMap::new();
        for point in &self.points {
            // Group by y not x
            rows.entry(point.y).or_insert(Vec::new()).push(point);
        }

        // Iterate through rows (y-values)
        for (y, points) in &rows {
            // Sort points in a row by X-coordinate 
            // To print columns left-to-right
            // Clone because points are reference and can't be borrowed as mut
            let mut sorted_points = points.clone();
            sorted_points.sort_by_key(|p| p.x);
            // points.sort_by_key(|p| p.x);

            // Print points with horizontal connections
            for (i, point) in sorted_points.iter().enumerate() {
                // Check if there's a traffic light at this point
                let traffic_light = self.traffic_lights.iter()
                .find(|light| light.position.0 == point.x && light.position.1 == point.y);
                
                if let Some(light) = traffic_light {
                    // Display the point with traffic light information
                    let light_symbol = match light.light_state {
                        LightState::Green => "\x1B[32mG\x1B[0m",  // Green text
                        LightState::Yellow => "\x1B[33mY\x1B[0m", // Yellow text
                        LightState::Red => "\x1B[31mR\x1B[0m",    // Red text
                    };
                    
                    // Format coordinates the same way as in Point::fmt
                    let coords = if point.x > 9 && point.y > 9 {
                        format!("({},{})", point.x, point.y)
                    } else if point.x > 9 && point.y < 10 {
                        format!("({},0{})", point.x, point.y)
                    } else if point.y > 9 && point.x < 10 {
                        format!("(0{},{})", point.x, point.y)
                    } else {
                        format!("(0{},0{})", point.x, point.y)
                    };
                    
                    write!(f, "{}{}", light_symbol, coords)?;
                } else {
                    // Use the default Point display
                    write!(f, "{}", point)?;
                }
                
                // Write the Horizontal connections
                if i < sorted_points.len() - 1 {
                    // 9 dots between horizontal points
                    for j in 1..=9 {
                        // Intermediate x between consecutive points
                        let current_x = point.x + j;
                        let current_y = *y;
                        let mut vehicle_symbol = None;

                        for vehicle in &self.vehicles {
                            // Current_position is a tupple, so we acces by index
                            if vehicle.current_position.0 == current_x 
                            && vehicle.current_position.1 == current_y {
                                vehicle_symbol = Some(match vehicle.vehicle_type {
                                    VehicleType::Car => 'C',
                                    VehicleType::Bus => 'B',
                                    VehicleType::Emergency => 'E',
                                });
                                break;
                            }
                        }
                        write!(f, "{}", vehicle_symbol.unwrap_or('.'))?;
                    }
                }
            }
            writeln!(f)?;

            // Print vertical lines below (except last row)
            if *y < (rows.len() as i32 - 1) * 10 {
                for line in 1..=9 {
                    for point in &sorted_points {
                        let current_y = y + line;
                        let mut vehicle_symbol: Option<char> = None;

                        for vehicle in &self.vehicles {
                            // Current_position is a tupple, so we acces by index
                            if vehicle.current_position.0 == point.x 
                            && vehicle.current_position.1 == current_y {
                                vehicle_symbol = Some(match vehicle.vehicle_type {
                                    VehicleType::Car => 'C',
                                    VehicleType::Bus => 'B',
                                    VehicleType::Emergency => 'E',
                                });
                                break;
                            }
                        }
                        // Get the string representation of the point 
                        let point_string = format!("{}", point); 
                        // Adjust spacing based on point's width
                        // +10 because of the number of dots between horizontal points
                        // -2 to adjust spacing.
                        let spacing = " ".repeat(point_string.len() + 10 - 2); 
                        // Vertical line + spacing
                        write!(f, "{}{}", vehicle_symbol.unwrap_or('.'), spacing)?;
                    }
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}