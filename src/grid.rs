use crate::point::Point;
use crate::vehicle::{Vehicle, VehicleType};
use std::fmt::{Display, Formatter, Result};
use crate::{GRID_HEIGHT, GRID_WIDTH};

pub struct Grid {
    pub points: Vec<Point>,
    pub vehicles: Vec<Vehicle>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            vehicles: Vec::new(),
        }  
    }

    pub fn generate_grid(mut self, height: i32, width: i32) -> Grid {

        // Generate points for a grid (height x width cells)
        for i in 0..= height - 1 {
            for j in 0..= width - 1 {
                self.points.push(Point {
                    // x should be a column value 
                    x: j * 10,
                    // y should be a row value
                    y: i * 10,
                    // Intersection appears if either x or y middle values
                    // (Not upper bound or lower bound values)
                    is_intersection: (j > 0 && j < width - 1) || ((i > 0 && i < height - 1)),
                });
            }
        }
    
        // Populate starting grid with vehicles 
        for _i in 0..GRID_WIDTH {
            self.vehicles.push(Vehicle::generate_vehicle());
        }
    
        return self; 
    }

    pub fn update_vehicles(&mut self) {
        for vehicle in self.vehicles.iter_mut() {
            vehicle.update();
        }
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
                // Get the string representation of the point
                let point_string = format!("{}", point); 
                // Write the point
                write!(f, "{}", point_string)?;
                
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