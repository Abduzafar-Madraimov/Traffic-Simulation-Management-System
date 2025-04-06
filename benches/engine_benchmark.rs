use criterion::{criterion_group, criterion_main, Criterion};
use helpers::{grid::Grid, light::{LightState, TrafficLight}, variables::CAR_ID_COUNTER, vehicle::Vehicle};
use tokio::runtime::Runtime;
use std::sync::atomic::{Ordering};

mod helpers;

// Reset global counters to ensure consistent benchmarks
fn reset_globals() {
    CAR_ID_COUNTER.store(0, Ordering::SeqCst);
}

// Benchmark vehicle generation throughput
fn benchmark_vehicle_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vehicle Generation");
    reset_globals();

    group.bench_function("Generate 100 Vehicles", |b| {
        let rt = Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let mut handles = vec![];
                for _ in 0..100 {
                    handles.push(tokio::spawn(Vehicle::generate_vehicle()));
                }
                for handle in handles {
                    handle.await.unwrap();
                }
            });
        });
    });
    group.finish();
}

// Benchmark traffic light update latency
fn benchmark_traffic_light_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("Traffic Light Updates");
    
    group.bench_function("Update 9 Traffic Lights", |b| {
        let rt = Runtime::new().unwrap();
        let mut lights = vec![];
        for _ in 0..9 {
            lights.push(TrafficLight::new(
                LightState::Green,
                (0, 0),
            ));
        }

        b.iter(|| {
            rt.block_on(async {
                TrafficLight::update_traffic_lights(&mut lights, 1.0).await;
            });
        });
    });
    group.finish();
}

// Benchmark grid update cycle (vehicles + lights)
fn benchmark_grid_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grid Update Cycle");
    reset_globals();

    group.bench_function("Full Grid Update (120ms Interval)", |b| {
        let rt = Runtime::new().unwrap();
        let mut grid = Grid::generate_grid(Grid::new(), 3, 3);
        
        b.iter(|| {
            rt.block_on(async {
                grid.update_vehicles().await;
                grid.update_traffic_lights(0.3).await; // Simulate 300ms interval
            });
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_vehicle_generation,
    benchmark_traffic_light_updates,
    benchmark_grid_updates
);
criterion_main!(benches);