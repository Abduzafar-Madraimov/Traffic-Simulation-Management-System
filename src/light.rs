//light.rs
use crate::point::Point;
use rand::Rng;

pub enum LightState {
    Green,
    Yellow,
    Red,
}

pub struct TrafficLight {
    pub light_state: LightState,
    pub position: (i32, i32),
}

impl TrafficLight {
    pub fn new(
        light_state: LightState,
        position: (i32, i32),
    ) -> TrafficLight {
        TrafficLight { 
            light_state, 
            position, 
        }
    }

    pub fn generate_traffic_light(point: &Point) -> Result<TrafficLight, &'static str> {
        if point.is_intersection {
            Ok(TrafficLight {
                light_state: match rand::rng().random_range(1..=3) {
                    1 => LightState::Green,
                    2 => LightState::Yellow,
                    3 => LightState::Red,
                    // since num is always between 1 and 3
                    _ => unreachable!(), 
                },
                position: (point.x, point.y),
            })
        } else {
            Err("Given point is not an intersection")
        }
    }
}