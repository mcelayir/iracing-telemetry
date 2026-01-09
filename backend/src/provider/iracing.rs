use async_trait::async_trait;
use futures::StreamExt;
use pitwall::{Pitwall, PitwallFrame, UpdateRate};

use super::{SimState, DashboardFrame, TelemetryFrame, RaceFrame, TelemetryProvider};

/// Internal struct mapping exactly to iRacing's Memory Map fields.
#[derive(PitwallFrame, Debug)]
struct IRacingData {
    #[field_name = "Gear"]
    gear: i32,
    #[field_name = "RPM"]
    rpm: f32,
    #[field_name = "Speed"]
    speed: f32,

    // Telemetry Data (Inputs)
    #[field_name = "Throttle"]
    throttle: f32,
    #[field_name = "Brake"]
    brake: f32,

    // Race Context Data
    #[field_name = "TrackTemp"]
    track_temp: f32,
    #[field_name = "PlayerCarPosition"]
    position: i32,
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

    async fn next_frame(&mut self) -> Option<SimState> {
        // 1. Check for connection handle
        let conn = self.connection.as_ref()?;
        
        // 2. Resilience: Re-subscribe if stream is missing
        if self.stream.is_none() {
            let st = conn.subscribe::<IRacingData>(UpdateRate::Native);
            self.stream = Some(Box::new(st));
        }

        let stream = self.stream.as_mut()?;

        // 3. Poll the next telemetry frame
        match stream.next().await {
            Some(data) => {
                // Mapping raw SDK data to our modular Public Interface
                Some(SimState {
                    dashboard: DashboardFrame {
                        gear: data.gear,
                        rpm: data.rpm,
                        speed: data.speed * 3.6, // Convert m/s to KPH
                    },
                    telemetry: TelemetryFrame {
                        throttle: data.throttle,
                        brake: data.brake,
                    },
                    race: RaceFrame {
                        sof: 0, // Placeholder for YAML parsing
                        track_temp: data.track_temp,
                        position: data.position,
                    },
                })
            },
            None => {
                // Stream hiccup: Clear stream for next tick re-subscription
                self.stream = None;
                
                // Wait briefly to prevent high-frequency spinning during downtime
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                self.disconnect().await;
                None
            }
        }
    }
}