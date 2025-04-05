// main.rs
mod grid;
mod point;
mod vehicle;
mod light;

use grid::Grid;
use std::sync::atomic::AtomicU64;
use tokio::sync::mpsc;
use std::time::{Instant, Duration};

// Global constants and variables
static GRID_HEIGHT: i32 = 3;
static GRID_WIDTH: i32 = 3;
static CAR_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    // Interval for more consistent scheduling
    // It calculates next tick based on the initial start time
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(120));

    // Track time passed
    let mut last_update = Instant::now();

    // Generate height by width grid of cells
    let mut grid = Grid::generate_grid(Grid::new(), GRID_HEIGHT, GRID_WIDTH);
    print!("{}", grid);

    loop {
        // Calculate how much time passed
        let now = Instant::now();
        let time_passed = (now - last_update).as_secs_f32();
        last_update = now;
        // Clear the screen and put the cursor at first row & first col of the screen
        print!("\x1B[2J\x1B[1;1H");

        // Update Traffic Lights
        grid.update_traffic_lights(time_passed);

        // Generate more vehicles
        for _ in 0..GRID_WIDTH {
            grid.vehicles.push(vehicle::Vehicle::generate_vehicle());
        }

        // Update vehicle positions 
        grid.update_vehicles().await;
        
        // Then print the updated grid
        print!("{}", grid);
        
        // Wait for the next tick (non-blocking)
        interval.tick().await;
    }
}