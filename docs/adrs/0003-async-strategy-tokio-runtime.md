# ADR 0003: Async Strategy - Tokio Runtime

## Status
Proposed

## Context
The application needs to handle multiple tasks simultaneously:
1. Reading the iRacing memory stream (60 times/sec).
2. Processing/formatting the data for display.
3. (Future) Sending data to a local WebSocket for the UI.
4. (Future) Uploading data to a remote server.

## Decision
We will use the **Tokio** asynchronous runtime.

## Reasoning
1. **Non-blocking Execution:** Tokio allows our "Data Collector" to keep running at 60Hz even if the "Cloud Uploader" is waiting for a slow internet response.
2. **Industry Standard:** Tokio is the most widely used and stable async runtime in the Rust ecosystem. This ensures long-term support and easy integration with other libraries (like WebSockets or API clients).
3. **The "Pit Crew" Analogy:** Tokio acts as the crew chief, managing different tasks (mechanics) without them tripping over each other, ensuring the car (data flow) never stops.

## Consequences
* The code becomes slightly more complex as it requires `async/await` syntax.
* We must use the `tokio` versions of libraries (e.g., `tokio::time::sleep` instead of `std::thread::sleep`) to avoid blocking the executor.