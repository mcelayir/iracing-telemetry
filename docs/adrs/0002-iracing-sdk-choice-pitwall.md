# ADR 0002: iRacing SDK Choice - Pitwall vs. iracing-telem

## Status
Proposed

## Context
iRacing provides telemetry via Windows Shared Memory. We need a Rust "crate" (library) to interface with this memory map safely and efficiently.

## Decision
We will use the **Pitwall** crate.

## Reasoning
1. **Developer Experience:** Pitwall uses Rust "Derive Macros." This allows us to define our data structures simply (e.g., `#[derive(PitwallFrame)]`) without manually calculating memory offsets or addresses.
2. **Efficiency:** It supports subscribing to data at different `UpdateRates` (Native 60Hz vs. 1Hz), which allows us to optimize CPU usage by only checking slow data (like fuel) once per second.
3. **Async Support:** Pitwall is built from the ground up to work with the `futures` and `tokio` ecosystems, which aligns with our concurrency goals.

## Consequences
* We become dependent on the maintenance of the Pitwall crate.
* We must ensure our Windows target (`x86_64-pc-windows-gnu`) is correctly configured to allow Pitwall to access the Windows API.