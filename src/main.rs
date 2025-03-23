use std::{
    fmt::{Display, Formatter, Result},
    vec,
    thread,
    time::Duration,
    sync::atomic::{AtomicU64, Ordering},
};
use rand::Rng;

// Global variables
static GRID_HEIGHT: i32 = 5;
static GRID_WIDTH: i32 = 5;
static CAR_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

fn main() {
    // Generate height by width grid of cells
    let mut grid = Grid::generate_grid(Grid::new(), GRID_HEIGHT, GRID_WIDTH);
    print!("{}", grid);

    loop {
        // Clear the screen and put the cursor at first row & first col of the screen
        print!("\x1B[2J\x1B[1;1H");

        // Generate more vehicles
        grid.vehicles.push(Vehicle::generate_vehicle());
        grid.vehicles.push(Vehicle::generate_vehicle());
        grid.vehicles.push(Vehicle::generate_vehicle());
        grid.vehicles.push(Vehicle::generate_vehicle());
        grid.vehicles.push(Vehicle::generate_vehicle());

        // Update vehicle positions 
        grid.update_vehicles();
        
        // Then print the updated grid
        print!("{}", grid);
        
        thread::sleep(Duration::from_millis(200));
    }
}

// Generate grid >
struct Point {
    x: i32,
    y: i32,
    is_intersection: bool,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let symbol = if self.is_intersection { "X" } else { "O" };
        // Making sure that one and two digit values have same
        // String size.
        if self.x > 9 && self.y > 9 {
            write!(f, "{}({},{})", symbol, self.x, self.y)
        } else if self.x > 9 && self.y < 10 {
            write!(f, "{}({},0{})", symbol, self.x, self.y)
        } else if self.y > 9 && self.x < 10 {
            write!(f, "{}(0{},{})", symbol, self.x, self.y)
        } else {
            write!(f, "{}(0{},0{})", symbol, self.x, self.y)
        }
    }
}

struct Grid {
    points: Vec<Point>,
    vehicles: Vec<Vehicle>,
}

impl Grid {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            vehicles: Vec::new(),
        }  
    }

    fn generate_grid(mut self, height: i32, width: i32) -> Grid {

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
    
        // // Generate Vehicles
        // // let vehicle = Vehicle::generate();
        // let vehicle1 = Vehicle::new(
        //     0, 
        //     VehicleType::Car, 
        //     (0, 5), 
        //     1, 
        //     5, 
        //     (20, 20), 
        //     1
        // );
        // let vehicle2 = Vehicle::new(
        //     1, 
        //     VehicleType::Bus, 
        //     (5, 0), 
        //     1, 
        //     5, 
        //     (20, 20), 
        //     2
        // );
        // let vehicle3 = Vehicle::new(
        //     2, 
        //     VehicleType::Emergency, 
        //     (10, 5), 
        //     1, 
        //     5, 
        //     (20, 20), 
        //     3
        // );
        // self.vehicles.push(vehicle1);
        // self.vehicles.push(vehicle2);
        // self.vehicles.push(vehicle3);

        self.vehicles.push(Vehicle::generate_vehicle());
        self.vehicles.push(Vehicle::generate_vehicle());
        self.vehicles.push(Vehicle::generate_vehicle());
    
        return self; 
    }

    fn update_vehicles(&mut self) {
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

// Generate grid <


// Generate vechiles. >
#[derive(Clone, Debug)]
enum VehicleType {
    Car,
    Bus,
    Emergency,
}

#[derive(Clone)]
struct Vehicle {
    id: u64,
    // Use visual identifier (e.g., 'C', 'B', 'E')
    vehicle_type: VehicleType,
    current_position: (i32, i32),
    // Units per second
    current_speed: i32,
    max_speed: i32,
    destination: (i32, i32),
    priority: u8,
}

impl Vehicle {
    fn new(
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

    fn generate_vehicle() -> Vehicle {
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
            priority)
    }

    fn update(&mut self) {
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