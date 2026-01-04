// src/main.rs
use iracing_telemetry::provider::mock::MockProvider;
use iracing_telemetry::provider::TelemetryProvider;
use iracing_telemetry::ui::cli::render_gauge;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting modular iRacing Telemetry...");
    
    // Initialize our source
    let mut source = MockProvider { counter: 0 };

    loop {
        // 1. Get data from whatever provider we are using
        if let Some(frame) = source.next_frame().await {
            // 2. Send it to the UI
            render_gauge(&frame);
        }
        
        // 3. Wait for next frame (approx 60Hz)
        sleep(Duration::from_millis(16)).await;
    }
}