use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio::sync::{broadcast, OnceCell};
use futures_util::{StreamExt, SinkExt};
use crate::provider::SimState;
use colored::*;

pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
        }
    }
}

static SERVER_INITIALIZED: OnceCell<()> = OnceCell::const_new();

pub async fn start_websocket_server(config: ServerConfig, mut rx: broadcast::Receiver<SimState>) {
    // Ensure this only runs once
    if SERVER_INITIALIZED.set(()).is_err() {
        eprintln!("{} Warning: Attempted to start WebSocket server more than once.", "BROKER >>".yellow());
        return;
    }

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind to port");
    
    println!("{} Server started on {}", "BROKER >>".bright_green(), addr);

    while let Ok((stream, _)) = listener.accept().await {
        let personal_rx = rx.resubscribe();
        tokio::spawn(handle_connection(stream, personal_rx));
    }
}

async fn handle_connection(stream: TcpStream, mut rx: broadcast::Receiver<SimState>) {
    let mut path = String::new();
    
    // Custom handshake to extract the path
    let callback = |req: &Request, response: Response| {
        path = req.uri().path().to_string();
        Ok(response)
    };

    let ws_stream = match accept_hdr_async(stream, callback).await {
        Ok(ws) => ws,
        Err(_) => return,
    };

    let (mut ws_sender, _) = ws_stream.split();

    while let Ok(state) = rx.recv().await {
        // Path-based Routing logic
        let message = match path.as_str() {
            "/ws/dashboard" => serde_json::to_string(&state.dashboard).unwrap(),
            "/ws/telemetry" => serde_json::to_string(&state.telemetry).unwrap(),
            _ => serde_json::to_string(&state).unwrap(), // Default to full state
        };

        if ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(message)).await.is_err() {
            break; // Client disconnected
        }
    }
}