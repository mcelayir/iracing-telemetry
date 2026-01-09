use async_trait::async_trait;
use tokio::time::{sleep, Duration};
use std::error::Error;

use super::{SimState, DashboardFrame, TelemetryFrame, RaceFrame, TelemetryProvider};

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

    async fn next_frame(&mut self) -> Option<SimState> {
        if !self.is_active {
            return None;
        }

        // Simulate a 60Hz update rate (approx 16.6ms)
        sleep(Duration::from_millis(16)).await;

        // 1. Simulate RPM and Gear Logic
        if self.increasing {
            self.current_rpm += 150.0;
            if self.current_rpm >= 8000.0 { self.increasing = false; }
        } else {
            self.current_rpm -= 150.0;
            if self.current_rpm <= 1000.0 { self.increasing = true; }
        }

        // 2. Map into the unified SimState
        Some(SimState {
            dashboard: DashboardFrame {
                gear: 3,
                rpm: self.current_rpm,
                speed: (self.current_rpm / 100.0) * 2.5, // Fake speed scaling
            },
            telemetry: TelemetryFrame {
                // Simulate throttle being pressed when RPM increases, brake when decreasing
                throttle: if self.increasing { 0.8 } else { 0.0 },
                brake: if !self.increasing { 0.4 } else { 0.0 },
            },
            race: RaceFrame {
                sof: 2500,        // Static high-quality mock data
                track_temp: 32.5,  // Standard racing temp
                position: 5,      // Mock mid-field position
            },
        })
    }
}