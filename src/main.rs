// main.rs
mod grid;
mod point;
mod vehicle;
mod light;
mod message;
mod analyzer;

use grid::Grid;
use std::sync::atomic::AtomicU64;
use tokio::sync::mpsc;
use std::time::Instant;
use message::SimulationMessage;

// Global constants and variables
static GRID_HEIGHT: i32 = 3;
static GRID_WIDTH: i32 = 3;
static CAR_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    // Set up a channel for inter-component communication
    let (tx, rx) = mpsc::channel::<SimulationMessage>(100);

    // Spawn the dummy analyzer task
    tokio::spawn(async move {
        analyzer::run_analyzer(rx).await;
    });

    // Interval for more consistent scheduling
    // It calculates next tick based on the initial start time
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(120));

    // Track time passed
    let mut last_update = Instant::now();

    // Generate height by width grid of cells
    let mut grid = Grid::generate_grid(Grid::new(), GRID_HEIGHT, GRID_WIDTH);
    print!("{}", grid);

    let mut tick: u64 = 0;
    loop {
        // Calculate how much time passed
        let now = Instant::now();
        let time_passed = (now - last_update).as_secs_f32();
        last_update = now;
        tick += 1;

        // Clear the screen and put the cursor at first row & first col of the screen
        print!("\x1B[2J\x1B[1;1H");

        // Update Traffic Lights asynchronously
        grid.update_traffic_lights(time_passed).await;

        // Generate more vehicles asynchronously
        let mut handles = vec![];
        for _ in 0..GRID_WIDTH {
            let handle = tokio::spawn(
                vehicle::Vehicle::generate_vehicle()
            );
            handles.push(handle);
        }
        // Collect generated vehicles
        for handle in handles {
            match handle.await {
                Ok(vehicle) => grid.vehicles.push(vehicle),
                Err(e) => {
                    eprintln!("Error generating vehicle: {}", e);
                }
            }
        }

        // Send a simulation update message to the analyzer as a placeholder
        let update_message = SimulationMessage::GridUpdate {
            tick,
            vehicle_count: grid.vehicles.len(),
            light_count: grid.traffic_lights.len(),
        };
        if let Err(e) = tx.send(update_message).await {
            eprintln!("Failed to send simulation update: {}", e);
        }

        // Update vehicle positions 
        grid.update_vehicles().await;
        
        // Then print the updated grid
        print!("{}", grid);
        
        // Wait for the next tick (non-blocking)
        interval.tick().await;
    }
}