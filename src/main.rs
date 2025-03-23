mod grid;
mod point;
mod vehicle;

use grid::Grid;
use std::{
    fmt::{Display, Formatter, Result},
    vec,
    thread,
    time::Duration,
    sync::atomic::{AtomicU64, Ordering},
};
use rand::Rng;

// Global constants and variables
static GRID_HEIGHT: i32 = 3;
static GRID_WIDTH: i32 = 3;
static CAR_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

fn main() {
    // Generate height by width grid of cells
    let mut grid = Grid::generate_grid(Grid::new(), GRID_HEIGHT, GRID_WIDTH);
    print!("{}", grid);

    loop {
        // Clear the screen and put the cursor at first row & first col of the screen
        print!("\x1B[2J\x1B[1;1H");

        // Generate more vehicles
        for _ in 0..GRID_WIDTH {
            grid.vehicles.push(vehicle::Vehicle::generate_vehicle());
        }

        // Update vehicle positions 
        grid.update_vehicles();
        
        // Then print the updated grid
        print!("{}", grid);
        
        thread::sleep(Duration::from_millis(100));
    }
}