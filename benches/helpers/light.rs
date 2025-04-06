//light.rs
use super::point::Point;
use rand::Rng;
use tokio::task::JoinSet;

#[derive(Clone)]
pub enum LightState {
    Green,
    Yellow,
    Red,
}

#[derive(Clone)]
pub struct TrafficLight {
    pub light_state: LightState,
    pub position: (i32, i32),
    // Track the time this light has spent in its current state
    pub time_in_state: f32,
    // Define durations for each state (in seconds)
    pub green_duration: f32,
    pub yellow_duration: f32,
    pub red_duration: f32,
}

impl TrafficLight {
    pub fn new(
        light_state: LightState,
        position: (i32, i32),
    ) -> TrafficLight {
        TrafficLight { 
            light_state, 
            position, 
            // In seconds:
            time_in_state: 0.0,
            green_duration: 3.0,
            yellow_duration: 1.0,  
            red_duration: 2.0, 
        }
    }

    pub fn generate_traffic_light(point: &Point) -> Result<TrafficLight, &'static str> {
        if point.is_intersection {
            // Generate random state for initial value
            let light_state = match rand::rng().random_range(1..=3) {
                1 => LightState::Green,
                2 => LightState::Yellow,
                3 => LightState::Red,
                _ => unreachable!(), 
            };
            
            // Create the TrafficLight with properties
            Ok(TrafficLight::new(light_state, (point.x, point.y)))
        } else {
            Err("Given point is not an intersection")
        }
    }

    // Get current state duration based on light state
    fn get_current_state_duration(&self) -> f32 {
        match self.light_state {
            LightState::Green => self.green_duration,
            LightState::Yellow => self.yellow_duration,
            LightState::Red => self.red_duration,
        }
    }

    // Get the next state in the traffic light cycle
    fn next_state(&self) -> LightState {
        match self.light_state {
            LightState::Green => LightState::Yellow,
            LightState::Yellow => LightState::Red,
            LightState::Red => LightState::Green,
        }
    }

    // Update a single traffic light given elapsed time
    pub async fn update(&mut self, time_passed: f32) {
        // Add the elapsed time to our time in current state
        self.time_in_state += time_passed;
        
        // Check if it's time to change state
        if self.time_in_state >= self.get_current_state_duration() {
            // Change to next state
            self.light_state = self.next_state();
            // Reset the timer
            self.time_in_state = 0.0;
        }
    }

    // Update all traffic lights in the grid's vector
    pub async fn update_traffic_lights(traffic_lights: &mut Vec<TrafficLight>, time_passed: f32) {
        // Create a collection of asynchronous tasks 
        let mut join_set = JoinSet::new();

        for (i, light) in traffic_lights.iter().enumerate() {
            // Clone the light so that the spawned task owns its data
            let light_clone = light.clone();
            // Create a future that updates the light
            join_set.spawn(async move {
                let mut updated = light_clone;
                updated.update(time_passed).await;
                (i, updated)
            });
        }

        // Collect updated traffic lights and update the vector
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok((i, updated_light)) => {
                    // Replace the original light with the updated one
                    traffic_lights[i] = updated_light;
                },
                Err(e) => {
                    eprintln!("Traffic light update task failed: {}", e);
                }
            }
        }
    
    }
}