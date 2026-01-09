pub mod provider;
pub mod network;
pub mod ui;

use tokio::sync::broadcast;
use crate::provider::{TelemetryProvider, IRacingProvider, MockProvider, SimState};
use crate::network::{ServerConfig, start_websocket_server};
use crate::ui::{WsGauge, WsTelemetry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    // 1. Setup Data Bus
    let (tx, _): (broadcast::Sender<SimState>, _) = broadcast::channel(16);
    let server_tx = tx.clone();
    let config = ServerConfig { port: 9000, ..Default::default() };

    // 2. Spawn WebSocket Server
    tokio::spawn(async move {
        start_websocket_server(config, tx.subscribe()).await;
    });

    // 3. Spawn Internal UI Consumer
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let gauge_consumer = WsGauge::new(9000);
        tokio::spawn(async move {
            gauge_consumer.run().await;
        });

        let gauge_telemetry = WsTelemetry::new(9000);
        tokio::spawn(async move {
            gauge_telemetry.run().await;
        });
    });

    // 4. Use the Interface (Trait Object)
    // This allows us to swap IRacingProvider with MockProvider easily
    let mut provider: Box<dyn TelemetryProvider> = Box::new(IRacingProvider::new());

    // 5. Orchestration Loop
    loop {
        if !provider.is_connected() {
            let _ = provider.connect().await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }

        if let Some(state) = provider.next_frame().await {
            let _ = server_tx.send(state);
        }
    }
}