use async_trait::async_trait;
use super::{TelemetryFrame, TelemetryProvider};

pub struct MockProvider {
    pub counter: u32,
}

#[async_trait]
impl TelemetryProvider for MockProvider {
    async fn next_frame(&mut self) -> Option<TelemetryFrame> {
        self.counter += 1;
        Some(TelemetryFrame {
            gear: (self.counter / 100 % 6) as i32 + 1,
            rpm: (self.counter % 9000) as f32,
            speed: (self.counter % 300) as f32,
        })
    }
}