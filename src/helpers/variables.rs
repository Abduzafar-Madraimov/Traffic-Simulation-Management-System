use std::sync::atomic::AtomicU64;

// Global constants and variables
pub static GRID_HEIGHT: i32 = 3;
pub static GRID_WIDTH: i32 = 3;
pub static CAR_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
