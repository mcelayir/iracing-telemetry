use async_trait::async_trait;
use tokio::time::{sleep, Duration};
use std::error::Error;

use super::{TelemetryFrame, TelemetryProvider};

pub struct MockProvider {
    is_active: bool,
    current_rpm: f32,
    increasing: bool,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            is_active: false,
            current_rpm: 1000.0,
            increasing: true,
        }
    }
}

#[async_trait]
impl TelemetryProvider for MockProvider {
    fn is_connected(&self) -> bool {
        self.is_active
    }

    async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        // Simulate a slight handshake delay
        sleep(Duration::from_millis(500)).await;
        self.is_active = true;
        Ok(())
    }

    async fn disconnect(&mut self) {
        self.is_active = false;
    }

    async fn next_frame(&mut self) -> Option<TelemetryFrame> {
        if !self.is_active {
            return None;
        }

        // Simulate a 60Hz update rate
        sleep(Duration::from_millis(16)).await;

        // Simple RPM logic: bounce between 1000 and 8000
        if self.increasing {
            self.current_rpm += 150.0;
            if self.current_rpm >= 8000.0 { self.increasing = false; }
        } else {
            self.current_rpm -= 150.0;
            if self.current_rpm <= 1000.0 { self.increasing = true; }
        }

        Some(TelemetryFrame {
            gear: 3,
            rpm: self.current_rpm,
            speed: (self.current_rpm / 100.0) * 2.5, // Fake speed scaling
        })
    }
}