pub mod mock;
pub mod iracing;

pub use self::iracing::IRacingProvider;
pub use self::mock::MockProvider;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimState {
    pub dashboard: DashboardFrame,
    pub race: RaceFrame,
    pub telemetry: TelemetryFrame,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardFrame {
    pub gear: i32,
    pub rpm: f32,
    pub speed: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RaceFrame {
    pub sof: i32,
    pub track_temp: f32,
    pub position: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryFrame {
    pub throttle: f32, // 0.0 to 1.0
    pub brake: f32,    // 0.0 to 1.0
}

#[async_trait]
pub trait TelemetryProvider {
    // Lifecycle Management
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn disconnect(&mut self);
    fn is_connected(&self) -> bool;

    // Data Flow
    async fn next_frame(&mut self) -> Option<SimState>;
}