# Roomba — Autonomous Vacuum Simulator (Rust)

## ⚠️ HOW TO WORK ON THIS PROJECT — read this first

The owner is learning **Rust + robotics through trial and error**. They reset this project once
because too much had been Claude-scaffolded — they consider a scaffolded project **"a loss."**
The whole point is that *they* do the thinking and the coding. So:

- **Socratic mode.** Ask questions that lead them to derive answers. Rarely give direct answers.
  **Do NOT write their implementation code.** Only write a snippet if they *explicitly* ask.
- **They write all code.** Review what they write; don't produce it for them.
- **Domain facts they can't derive** (how real hardware behaves, idiomatic library patterns, the
  Kalman-gain formula, etc.) — *do* teach those plainly; that's not scaffolding.
- **Diagrams: you draw, they derive.** They own every design decision (reached via your questions);
  you only transcribe it. **Never invent design content.**
    - Diagrams live in **`docs/` as `.excalidraw` files ONLY.** They tried Mermaid and rejected it;
      they find hand-drawing hard, so you generate the `.excalidraw` JSON.
    - Layout tips (you can't see the render): generous spacing, **no filled container boxes** (they
      render as black slabs in the owner's dark theme), minimal arrow bends, labels clear of boxes.
- **The map keeps them on-track.** When tempted to add something, the test is *"which box on the
  map is this?"* — build **to the map**, don't scope-creep off it.

## What this is

A Rust Cargo workspace for a Roomba-style autonomous cleaning robot, developed entirely in
simulation first. Full vision: `references/Roomba-Style Autonomous Robot — Project Blueprint.md`.
MVP spec (PRD): `references/Roomba Sim EKF Coverage Validator.md`.

**MVP = 3 crates:**

- `shared` — domain types crossing every boundary (Pose, Map, Cell, commands, measurements).
- `robot` — the brain **library**: state machine, hand-written **EKF** localization, boustrophedon
  coverage planner, object handling, all behind a **hardware-abstraction trait**. No `main`, unit-testable.
- `sim` — dev **host**: implements the trait with simulated physics + a 2D `macroquad` visualizer
  showing **true vs. EKF-estimated** pose.

Core algorithms (EKF, coverage planner) are **hand-written for learning**; external crates only for
plumbing (`macroquad` render, `serde`/`ron`, `nalgebra` matrices, `tokio` later).

## Current state

- **Workspace scaffolded** — 3 crates in `crates/`, `shared` has module structure with domain types (pose, command, measurement, geometry, map), `robot` has trait placeholder, `sim` has entrypoint.
- **Design map is COMPLETE** — 8 Excalidraw diagrams in `docs/`, fully cross-linked:
    - `01-overview` — one Sense→Plan→Act tick + shared state + supervisor
    - `02-modes` — state machine: Idle · Cleaning · Moving{target} · Charging · Fault · Off
    - `03-concurrency` — Tokio tasks (one per sensor) + channels + shared state
    - `04-plan-tick` — the pipeline inside one PLAN tick (links to 06/07/08)
    - `05-act-sim-seam` — ACT → hardware-abstraction trait → SimHost → noisy sensors (loop closes)
    - `06-ekf-localize` — the EKF predict/update loop
    - `07-coverage` — boustrophedon sweep decision flow
    - `08-object-handling` — dirt vs. obstacle handling

## Locked design decisions (don't relitigate — amend the diagram if changing)

- **Loop:** tick-based Sense→Plan→Act. Persistent state between ticks = EKF belief (μ, P) +
  coverage grid + map.
- **Sensors:** wheel odometry + IMU (fast, *integrated*), docking-station **beacon** (range/bearing —
  the MVP absolute correction), LiDAR (post-MVP: surprise-obstacle detection + scan-matching).
  Sim injects noise **and** latency jitter.
- **EKF:** predict from odometry/IMU (grows P), correct from the beacon (shrinks P). `f` (motion,
  uses cos/sin θ) and `h` (range/bearing) are nonlinear → linearize with Jacobians `F = ∂f/∂x`,
  `H = ∂h/∂x` each step. This is the project's primary learning artifact.
- **Map:** `outer_bound: Vec<Point>` (walls) + `inner_bound: Vec<Vec<Point>>` (no-go zones) =
  polygon-with-holes. Objects (dirt/obstacles) are **map-defined** in MVP (not sensed).
- **Coverage grid:** `cell = floor(pos / cell_size)` → O(1) "cleaned here?". `cell_size` trades
  accuracy vs. memory.
- **Coverage:** boustrophedon stripes; turn = U-turn (rotate 90° · forward one robot-width ·
  rotate 90°). Scoped to **"good enough,"** not optimal.
- **Objects:** small (dirt) → drive over, mark cleaned; large (obstacle) → replan **around**, rejoin lane.
- **Modes:** `Moving` = all non-cleaning travel (absorbed the old Docking); it carries a `target`
  that routes it to Charging (dock) or Cleaning (area). Start/resume goes Idle→Moving→Cleaning.
  Fault→auto-restart→Recovered?→Idle, else Alert→Off. Resume after charge if incomplete & <1h.
- **Concurrency:** Tokio, one task per sensor. `watch` channels for latest sensor values,
  `Arc<RwLock<..>>` for the shared world model, `mpsc` for commands. PLAN runs on
  `tokio::interval` (~10 Hz) reading the latest — never blocks on a slow sensor.
- **Sim seam:** `SimHost` implements the `HardwareAbstraction` trait; holds the **true** pose,
  advances it by commanded velocity + process noise, emits **noisy** sensor readings. Swap for a
  Raspberry-Pi impl (`robotd`) later without touching the brain.

## Next step — build slice 1 (do this Socratically; the owner writes it)

Get a robot driving in sim with **ground-truth pose** — no EKF, no coverage — just proving the
Sense→Plan→Act loop closes and the `macroquad` visualizer draws. Then vertical-slice upward:
swap in the EKF (Diagram 6), then coverage (7), then modes (2).
