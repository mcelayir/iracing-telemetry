use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use crate::provider::TelemetryFrame; // Import the specific frame being sent
use colored::*;
use std::io::{self, Write};

pub struct WsTelemetry {
    url: String,
}

impl WsTelemetry {
    pub fn new(port: u16) -> Self {
        Self {
            url: format!("ws://127.0.0.1:{}/ws/telemetry", port),
        }
    }

    pub async fn run(&self) {
        // Connect to our own server
        let (ws_stream, _) = connect_async(&self.url)
            .await
            .expect("Failed to connect to internal WS");
            
        let (_, mut read) = ws_stream.split();

        println!("{} CLI Telemetry subscribed to WebSocket", "CONSUMER >>".bright_yellow());

        while let Some(Ok(message)) = read.next().await {
            if let Message::Text(text) = message {
                // Since /ws/dashboard sends only the DashboardFrame, 
                // we deserialize into that specific struct.
                if let Ok(frame) = serde_json::from_str::<TelemetryFrame>(&text) {
                    self.render_gauge(&frame);
                }
            }
        }
    }

    fn render_gauge(&self, frame: &TelemetryFrame) {
        let throttle_pct = (frame.throttle * 10.0) as usize;
        let brake_pct = (frame.brake * 10.0) as usize;

        print!(
            "\r{} T:{thr} B:{brk} ",
            "WS-TLM-CLIENT >>".bright_yellow(),
            thr = format!("{:_<10}", "I".repeat(throttle_pct)).green(),
            brk = format!("{:_<10}", "I".repeat(brake_pct)).red()
        );

        io::stdout().flush().unwrap();
    }

    

    
}