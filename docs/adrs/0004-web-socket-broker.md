# ADR 0004: Modular Telemetry Dispatching via Single-Port WebSocket Broker

## Status
Accepted

## Context
The application must serve real-time telemetry data to multiple independent UI components (Electron overlays). We require a modular system where specific data sets (e.g., Dashboard vs. Physics Telemetry) can be enabled or disabled based on user selection.

Our previous technical discovery confirmed that the `iRacingProvider` must remain a **Singleton** to maintain a persistent handle to the Windows Memory Map and avoid resource contention. We also want to avoid the "port fatigue" of managing multiple network ports (8081, 8082, etc.), which complicates firewall configuration and client-side discovery.

## Decision
We will implement a **Centralized Dispatcher (Broker) Pattern** using a single WebSocket server on a fixed port.

1.  **Singleton Source**: A single `IRacingProvider` instance maintains the persistent connection handle to the iRacing SDK.
2.  **Modular Extractors**: We will implement specialized "Data Extractor" modules. Each extractor is responsible for transforming raw SDK data into a specific, typed JSON schema (e.g., `DashboardData`, `FuelData`, `StintAnalytics`).
3.  **Single-Port Multi-Channel**: The WebSocket server will utilize **URL Path Routing** (e.g., `/ws/dashboard`, `/ws/physics`) to allow UI components to subscribe to specific data streams.
4.  **Tevhid-Compliant Implementation**: Logic and naming conventions will remain strictly technical and professional, ensuring the code remains clean and free of non-technical metaphors.



## Consequences
* **Positive**: Optimized Resource Usage. The memory-mapped file is only read once by the Singleton, regardless of how many overlays are active.
* **Positive**: Simplified Frontend Integration. Electron components only need a standard WebSocket URL to access specific data.
* **Positive**: Strategic Scalability. New Business KPIs can be implemented as independent "Extractors" without modifying the core connection logic.
* **Negative**: Centralized Complexity. The WebSocket routing logic becomes a critical component that requires robust unit testing.
* **Negative**: Serialization Overhead. Multiple extractors require efficient JSON serialization to maintain low latency for high-frequency data (e.g., RPM).

## Compliance & KPIs
This decision supports the **Business Case (2025-12-23)** by providing a modular architecture that can be packaged into different product tiers (Basic vs. Pro) by simply restricting access to specific WebSocket paths.