use iracing_telemetry::provider::iracing::IRacingProvider;
use iracing_telemetry::provider::TelemetryProvider;
use iracing_telemetry::ui::cli::render_gauge;
use std::time::Duration;
use tokio::time::sleep;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    // Using the trait object approach (or direct struct if you prefer)
    let mut provider = IRacingProvider::new();

   loop {
        // 1. Connection Gate: Only run if we aren't connected
        if !provider.is_connected() {
            println!("🔍 {} Waiting for session...", "IDLE:".yellow());
            
            while let Err(_) = provider.connect().await {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
            if provider.is_connected() {
                println!("✅ {} Live!", "CONNECTED:".green().bold());
            }
        }

        // 2. Data Retrieval: Single poll per loop iteration
        match provider.next_frame().await {
            Some(frame) => {
                render_gauge(&frame);
            }
            None => {
                // If we get None, it doesn't necessarily mean the game closed.
                // We check the provider's connection status explicitly.
                if !provider.is_connected() {
                    println!("\n❌ {} Session lost.", "DISCONNECTED:".red().bold());
                    // Only now will the next iteration hit the "IDLE" print
                } else {
                    // The stream is still alive, but no data this tick. 
                    // We just sleep briefly and try again.
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
            }
        }
    }
}