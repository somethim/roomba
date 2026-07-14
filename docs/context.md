# Roomba Project Context

## Overview

A Rust-based autonomous cleaning robot built as a single Cargo workspace (monorepo). The system is split into two tiers
connected by shared types:

- **The robot** (`robot` crate) — a self-contained brain: state machine, hand-written EKF localization, boustrophedon
  coverage planner, object handling. Talks to the world through a **hardware-abstraction trait** so it never knows
  whether it is in simulation or on a Pi.
- **The user control plane** (`api` + `dashboard`, future) — a web API + dashboard deployed to a server, acting as the
  user's interface for the robot.

## Workspace Layout

```
roomba/
├── Cargo.toml                  ← workspace root, resolver = "3"
├── crates/
│   ├── shared/                 ← domain types (Pose, Map, Command, Measurement) with validators + tests
│   ├── robot/                  ← brain library: HardwareAbstraction trait, state machine, EKF, coverage. No `main`.
│   ├── sim/                    ← executable: implements HardwareAbstraction, macroquad viz, main loop
│   ├── robotd/                 ← (future) hardware impl for Raspberry Pi
│   ├── api/                    ← (future) Axum server, user-facing API
│   ├── dashboard/              ← (future) Leptos WASM web app
│   └── tui/                    ← (future) Ratatui terminal client
```

## Current State (Jul 2026)

- **Design complete** — 8 Excalidraw diagrams in `docs/`
- **Workspace scaffolded** — 3 crates under `crates/`
- **`shared`** — module scaffold with domain types (pose, geometry, map, command, measurement)
- **`robot`** — trait stubs in `lib.rs`
- **`sim`** — entrypoint in `main.rs`
- **Next:** Slice 1 — driving loop + macroquad visualization

## TODO

- Fix EKF angle wrapping / radian normalization in the predict-update path. Current divergence is
  likely caused by unwrapped yaw and bearing residuals.

## Architecture & Locked Design Decisions

### Loop

Tick-based **Sense → Plan → Act**. Persistent state between ticks = EKF belief (μ, P) + coverage grid + map.

### Sensors

- **Wheel odometry + IMU** (fast, integrated) — predict step in EKF
- **Docking-station beacon** (range/bearing) — the MVP absolute correction, used in EKF update step
- **LiDAR** (post-MVP: surprise-obstacle detection + scan-matching)

Sim injects noise and latency jitter into all sensor readings.

### EKF

- Predict from odometry/IMU (grows covariance P)
- Correct from the beacon (shrinks P)
- `f` (motion, uses cos/sin θ) and `h` (range/bearing) are nonlinear → linearize with Jacobians `F = ∂f/∂x`, `H = ∂h/∂x`
  each step
- Primary learning artifact of the project

### Map

- `outer_bound: Vec<Point>` (walls) + `inner_bound: Vec<Vec<Point>>` (no-go zones) = polygon-with-holes
- Objects (dirt/obstacles) are **map-defined** in MVP (not sensed at runtime)

### Coverage

- **Coverage grid:** `cell = floor(pos / cell_size)` → O(1) "cleaned here?"
- **Pattern:** boustrophedon stripes; turn = U-turn (rotate 90° · forward one robot-width · rotate 90°)
- Scoped to "good enough," not optimal

### Object Handling

- **Small (dirt):** drive over, mark cleaned, remove from map
- **Large (obstacle):** replan **around**, rejoin lane

### State Machine (Modes)

`Idle` → `Moving{target}` → `Cleaning` | `Charging` | `Fault`

- `Moving` absorbs all non-cleaning travel; carries a `target` that routes to Charging (dock) or Cleaning (area)
- Start/resume: `Idle` → `Moving` → `Cleaning`
- Fault: auto-restart → Recovered? → `Idle`, else `Alert` → `Off`
- Resume after charge if incomplete & <1h

### Concurrency

- Tokio, one task per sensor
- `watch` channels for latest sensor values
- `Arc<RwLock<..>>` for the shared world model
- `mpsc` for commands
- PLAN runs on `tokio::interval` (~10 Hz) reading the latest — never blocks on a slow sensor

### Sim Seam (Hardware-Abstraction Trait)

`SimHost` implements the `HardwareAbstraction` trait; holds the **true** pose, advances it by commanded velocity +
process noise, emits **noisy** sensor readings. Swap for a Raspberry-Pi impl (`robotd`) later without touching the
brain.

## Design Diagrams (8 Excalidraw files in `docs/`)

| File                            | Topic                                                                    |
|---------------------------------|--------------------------------------------------------------------------|
| `01-overview.excalidraw`        | One Sense→Plan→Act tick + shared state + supervisor                      |
| `02-modes.excalidraw`           | State machine: Idle · Cleaning · Moving{target} · Charging · Fault · Off |
| `03-concurrency.excalidraw`     | Tokio tasks (one per sensor) + channels + shared state                   |
| `04-plan-tick.excalidraw`       | Pipeline inside one PLAN tick (links to 06/07/08)                        |
| `05-act-sim-seam.excalidraw`    | ACT → hardware-abstraction trait → SimHost → noisy sensors (loop closes) |
| `06-ekf-localize.excalidraw`    | EKF predict/update loop                                                  |
| `07-coverage.excalidraw`        | Boustrophedon sweep decision flow                                        |
| `08-object-handling.excalidraw` | Dirt vs. obstacle handling                                               |

## Tech Stack

| Concern          | Choice                               | Why                                              |
|------------------|--------------------------------------|--------------------------------------------------|
| Serialization    | `serde`                              | Universal — payloads, storage, files             |
| Map/config files | `ron`                                | Readable, serde-native                           |
| EKF matrix math  | `nalgebra`                           | Matrix plumbing; EKF equations stay hand-written |
| 2D visualizer    | `macroquad`                          | Minimal-boilerplate cross-platform 2D            |
| Simulation noise | `rand`                               | Seeded RNG for reproducible noise                |
| Async runtime    | `tokio` (future)                     | Industry standard, required by Axum              |
| Web server + WS  | `axum` (future)                      | Ergonomic, async-first                           |
| Database         | `sqlx` + SQLite (future)             | File-based, no server                            |
| Web dashboard    | `leptos` (future)                    | Compiles to WASM                                 |
| Terminal UI      | `ratatui` (future)                   | Feature-rich systems dashboards                  |
| Error handling   | `thiserror` (libs) + `anyhow` (bins) | Idiomatic per-layer                              |

## Build Order

**MVP (simulation):**

1. `shared` — domain types (done)
2. `robot` — brain behind the hardware-abstraction trait (stubs done)
3. `sim` — host with simulated physics + macroquad visualizer (entrypoint done)

**Post-MVP:**

1. `robotd` — hardware-abstraction on Raspberry Pi
2. `api` — user-facing Axum server
3. `dashboard` — Leptos UI
4. `tui` — Ratatui terminal client

## Linting

```sh
cargo clippy --all-targets -- -D clippy::pedantic -D clippy::nursery
```

## References

- [
  `references/Roomba-Style Autonomous Robot — Project Blueprint.md`](../references/Roomba-Style%20Autonomous%20Robot%20%E2%80%94%20Project%20Blueprint.md) —
  full project vision, crate responsibilities, data flow diagrams, connectivity design
- [`references/Roomba Sim EKF Coverage Validator.md`](../references/Roomba%20Sim%20EKF%20Coverage%20Validator.md) — MVP
  PRD: problem statement, success criteria, core features (EKF, boustrophedon, object handling, 2D visualizer),
  in-scope/out-of-scope lists
