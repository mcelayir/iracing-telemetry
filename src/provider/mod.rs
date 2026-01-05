use async_trait::async_trait;

pub mod mock; // Link the mock.rs file
pub mod iracing;

#[derive(Debug, Clone, Default)]
pub struct TelemetryFrame {
    pub gear: i32,
    pub rpm: f32,
    pub speed: f32,
}

#[async_trait]
pub trait TelemetryProvider {
    // Lifecycle Management
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn disconnect(&mut self);
    fn is_connected(&self) -> bool;

    // Data Flow
    async fn next_frame(&mut self) -> Option<TelemetryFrame>;
}