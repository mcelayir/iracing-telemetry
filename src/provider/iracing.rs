use async_trait::async_trait;
use futures::StreamExt;
use pitwall::{Pitwall, PitwallFrame, UpdateRate};
use colored::Colorize;
use std::error::Error;

use super::{TelemetryFrame, TelemetryProvider};

/// Internal struct mapping exactly to iRacing's Memory Map fields.
#[derive(PitwallFrame, Debug)]
struct IRacingData {
    #[field_name = "Gear"]
    gear: i32,
    #[field_name = "RPM"]
    rpm: f32,
    #[field_name = "Speed"]
    speed: f32,
}

pub struct IRacingProvider {
    // Plan A: Persistent handle to the Windows Memory Map
    connection: Option<pitwall::LiveConnection>,
    // Transient stream that we can refresh without losing the connection
    stream: Option<Box<dyn futures::Stream<Item = IRacingData> + Send + Unpin>>,
}

impl IRacingProvider {
    pub fn new() -> Self {
        Self { connection: None, stream: None }
    }
}

#[async_trait]
impl TelemetryProvider for IRacingProvider {
    fn is_connected(&self) -> bool {
        // As long as the handle exists, we consider ourselves "Connected"
        self.connection.is_some()
    }

    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_connected() { return Ok(()); }

        // Establish the "Singleton" handle
        let conn = Pitwall::connect().await?;
        
        // Immediately try to open the stream
        let st = conn.subscribe::<IRacingData>(UpdateRate::Native);
        
        self.connection = Some(conn);
        self.stream = Some(Box::new(st));
        Ok(())
    }

    async fn disconnect(&mut self) {
        self.stream = None;
        self.connection = None;
    }

    async fn next_frame(&mut self) -> Option<TelemetryFrame> {
        // If the handle is gone, we are truly disconnected
        let conn = self.connection.as_ref()?;
        
        // If we have a handle but the stream is missing (e.g., after a hiccup), 
        // we try to re-subscribe silently here.
        if self.stream.is_none() {
            let st = conn.subscribe::<IRacingData>(UpdateRate::Native);
            self.stream = Some(Box::new(st));
        }

        let stream = self.stream.as_mut()?;
        match stream.next().await {
            Some(data) => Some(TelemetryFrame {
                gear: data.gear,
                rpm: data.rpm,
                speed: data.speed * 3.6,
            }),
            None => {
                // If the stream returns None, the game is still open (handle exists),
                // but no data is flowing. We clear the stream to trigger a 
                // re-subscribe next tick, but we DO NOT return None to main.rs.
                self.stream = None;
                // Return a dummy frame or wait briefly to keep the loop alive
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                Some(TelemetryFrame::default()) 
            }
        }
    }
}