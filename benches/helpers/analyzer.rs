use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use super::message::SimulationMessage;


pub async fn run_analyzer(mut rx: mpsc::Receiver<SimulationMessage>) {
    while let Some(message) = rx.recv().await {
        println!("Analyzer received: {:?}", message);
        // Simulate some processing time (e.g., 50ms)
        sleep(Duration::from_millis(50)).await;
    }
}