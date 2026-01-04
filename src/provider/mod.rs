use async_trait::async_trait;

pub mod mock; // Link the mock.rs file

#[derive(Debug, Clone)]
pub struct TelemetryFrame {
    pub gear: i32,
    pub rpm: f32,
    pub speed: f32,
}

#[async_trait]
pub trait TelemetryProvider {
    async fn next_frame(&mut self) -> Option<TelemetryFrame>;
}