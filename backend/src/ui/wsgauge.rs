use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use crate::provider::DashboardFrame; // Import the specific frame being sent
use colored::*;
use std::io::{self, Write};

pub struct WsGauge {
    url: String,
}

impl WsGauge {
    pub fn new(port: u16) -> Self {
        Self {
            url: format!("ws://127.0.0.1:{}/ws/dashboard", port),
        }
    }

    pub async fn run(&self) {
        // Connect to our own server
        let (ws_stream, _) = connect_async(&self.url)
            .await
            .expect("Failed to connect to internal WS");
            
        let (_, mut read) = ws_stream.split();

        println!("{} CLI Gauge subscribed to WebSocket", "CONSUMER >>".bright_blue());

        while let Some(Ok(message)) = read.next().await {
            if let Message::Text(text) = message {
                // Since /ws/dashboard sends only the DashboardFrame, 
                // we deserialize into that specific struct.
                if let Ok(frame) = serde_json::from_str::<DashboardFrame>(&text) {
                    self.render_gauge(&frame);
                }
            }
        }
    }

    fn render_gauge(&self, frame: &DashboardFrame) {
        let gear_display = match frame.gear {
            0 => "N".yellow().bold(),
            -1 => "R".red().bold(),
            g => g.to_string().green().bold(),
        };

        let rpm_color = if frame.rpm > 8000.0 {
            format!("{:<5.0}", frame.rpm).on_red().white().bold()
        } else {
            format!("{:<5.0}", frame.rpm).white()
        };

        print!(
            "\r{} Gear: [{}] | RPM: {} | Speed: {:<5.1} km/h          ",
            "WS-GAG-CLIENT >>".bright_blue(),
            gear_display,
            rpm_color,
            frame.speed
        );

        io::stdout().flush().unwrap();
    }

    

    
}