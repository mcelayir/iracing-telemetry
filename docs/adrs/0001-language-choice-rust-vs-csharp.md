# ADR 0001: Language Choice - Rust vs. C#

## Status
Approved

## Context
We need a programming language to build a high-performance telemetry engine for iRacing. The engine must read memory at 60Hz and eventually serve data to a UI overlay. The two primary candidates were C# (common in the sim racing community) and Rust.

## Decision
We have chosen **Rust** as the core language for this project.

## Reasoning
1. **Performance and Latency:** iRacing telemetry updates 60 times per second. Rust’s "zero-cost abstractions" and lack of a Garbage Collector (GC) ensure that we never experience "micro-stutters" caused by memory management pauses.
2. **Memory Safety:** Rust’s ownership model guarantees thread safety at compile time. This is crucial as we will be reading memory in one thread and sending it to a UI/Server in another.
3. **Resource Efficiency:** Rust produces a single, small binary with a very low RAM footprint. This is a key selling point compared to competitors like RaceLab or iOverlay, which can be resource-heavy.
4. **Learning Opportunity:** As a side hustle, building in Rust provides high market value for the developer's skill set.

## Consequences
* The learning curve for the developer is steeper than C#.
* We must manage cross-compilation (WSL to Windows) to access Windows Shared Memory.