//message.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SimulationMessage {
    GridUpdate { 
        tick: u64,
        vehicle_count: usize, 
        light_count: usize,
    },
}