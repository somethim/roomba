# Roomba-Style Autonomous Robot — Project Blueprint

---

## Overview

A Rust-based autonomous cleaning robot, built as a single Cargo workspace (monorepo). The system is split into **two
tiers** connected by shared types:

- **The robot** — a self-contained brain that handles its own movement, localization, decision-making, and controller
  I/O. It exposes an **internal API**: the control surface for commanding the robot and reading its state.
- **The user control plane** — a web API + dashboard, deployed to a **server**, that acts as the user's interface _for_
  the robot. Through it the user says what they want done: which rooms are fine to clean, which to ignore, when to dock.
  It is a **client of** the robot's internal API.

A simulation host (`sim`) runs the entire robot brain on a laptop with fake sensors and a 2D visualizer, so the robot's
intelligence can be built and validated before any hardware exists. Shared types ensure the robot, the server, and the
UI never drift apart.

---

## Goals

- Build a fully autonomous Roomba-style robot in Rust, with a clean seam between **robot logic** and its **host** (
  simulation today, hardware later)
- Develop and validate the entire robot brain in **simulation** before touching a Raspberry Pi
- Localize on a known, pre-loaded map using a **hand-written Extended Kalman Filter** (SLAM is a later phase)
- Give the user a **web dashboard** to configure and command the robot — rooms to clean, no-go zones, docking,
  scheduling
- Optional local diagnostics/control via a terminal UI
- Clean, maintainable monorepo architecture with shared types across both tiers

---

## Workspace Structure

```text
roomba/
├── Cargo.toml                  ← workspace root
├── crates/
│   ├── shared/                 ← common types crossing every boundary               [now]
│   ├── robot/                  ← robot brain (lib): movement, localization,
│   │                             decision-making, controller I/O — the internal API [now]
│   ├── sim/                    ← dev-only host: runs the brain with simulated
│   │                             physics + a 2D top-down visualizer                 [now]
│   ├── robotd/                 ← on-device host: runs the brain on real hardware,
│   │                             serves the internal API over the network        [future]
│   ├── api/                    ← user-facing server: client of the robot,
│   │                             serves the dashboard, persists user config       [future]
│   ├── dashboard/              ← Leptos WASM web app, served by api                [future]
│   └── tui/                    ← terminal client for local diagnostics/control     [future]
```

`shared`, `robot`, and `sim` are the MVP. Everything marked `[future]` snaps onto the same two libraries (`robot` +
`shared`) without changing them.

---

## The Two Control Planes

**Robot internal API (`robot` + `shared`)** — the low-level control surface: move, sense, localize, decide, drive the
controller. This _is_ the brain. It is defined as a Rust interface (traits) over a hardware abstraction, so the same
brain can be driven:

- **in-process** by `sim` during development (fake sensors + visualizer), and
- **over the network** by `robotd` on real hardware (real sensors + motors), exposed so the server tier can reach it.

**User control plane (`api` + `dashboard`)** — the human's interface. High-level intent: which rooms are fine, which to
ignore, when to dock, start/stop/pause. It does **not** contain the brain; it is a client that sends commands down to
the robot's internal API and renders state coming back up. `shared` carries the message types for both hops (
browser↔server and server↔robot).

---

## Crate Responsibilities

### `shared` _(lib — now)_

The single source of truth for all data types crossing crate boundaries and tier boundaries. Defines the domain model (
`Pose`, `Map`, `Cell`, `robot_state`, `RobotCommand`, sensor/measurement types) and the wire messages exchanged between
robot, server, and UI. Everything is serializable so it can travel over a network, be persisted, or be rendered —
without duplication or drift.

### `robot` _(lib — now)_

The brain of the robot, and the only place robot intelligence lives. Owns the state machine (
`Idle → Sweeping → Docking → Charging`), the hand-written EKF localization, the boustrophedon coverage planner, object
classification/handling, and local obstacle replanning. Talks to the world through a **hardware-abstraction trait** (
sensors in, actuator/controller commands out) so it never knows whether it is in a simulator or on a Pi. This
abstraction _is_ the robot's internal API. Pure library — no `main`, fully unit-testable.

### `sim` _(bin — now)_

The dev-only host for the brain and the MVP's runnable artifact. Implements the hardware-abstraction trait with
simulated physics: it holds the robot's **true** pose, advances it by commanded velocity plus injected noise, and feeds
the brain only **noisy** odometry and measurements. Renders a live 2D top-down view — the map, no-go zones, dirt,
obstacles, planned path, and both the **true** and **EKF-estimated** pose so the filter can be watched and debugged. The
dev twin of `robotd`.

### `robotd` _(bin — future)_

The on-device host. Implements the same hardware-abstraction trait against real Raspberry Pi hardware (GPIO, motor
controllers, LiDAR/camera), runs the `robot` brain, and exposes the internal API over the network so the user-facing
`api` can command it and stream its state. Because it shares the brain trait with `sim`, the hardware transition is a
new trait implementation, not a rewrite.

### `api` _(bin — future)_

An Axum server **deployed to a server** (not on the robot). The user's interface for the robot: accepts high-level
configuration and commands (rooms to clean/ignore, docking schedule, start/stop), forwards them to the robot's internal
API, and streams robot state back out over WebSocket. Serves the compiled `dashboard` bundle and persists user-facing
config, schedules, and session history.

### `dashboard` _(lib/WASM — future)_

A Leptos WASM web application served by `api`. Runs in any browser — phone, tablet, desktop. Connects over WebSocket to
render live robot state (position on the map, battery, mode, dirt remaining) and lets the user configure rooms/no-go
zones and send commands. No app install — just a browser.

### `tui` _(bin — future)_

A Ratatui terminal client for local diagnostics and control, run from an operator's machine. Talks to the `api` server
and/or the robot directly. Useful for on-the-fly config, inspecting live readings, and manual commands during
development or maintenance.

---

## Data Flow

```text
                 USER (browser / phone)
                          │
                          ▼
                   dashboard (WASM)
                          │  WebSocket
                          ▼
   ┌─────────────── USER CONTROL PLANE (server) ───────────────┐
   │   api  ◄────────────────────────────►  SQLite             │
   │   "clean room A, ignore B, dock 9pm"   (user config,      │
   │                                         schedules, history)│
   └──────────────────────────┬────────────────────────────────┘
                              │  robot internal API (network)
                              │  commands ▼      ▲ state
   ┌──────────────────────────┴────────────────────────────────┐
   │  ROBOT — hosted by robotd (hardware)  or  sim (dev)        │
   │                                                            │
   │     robot (brain) + shared                                │
   │     state machine · EKF localization · coverage planner   │
   │     object handling · controller I/O                      │
   │                       │                                    │
   │            hardware-abstraction trait                      │
   │              │                      │                      │
   │   real sensors / motors   ── or ──   simulated physics     │
   │        (robotd)                        (sim + visualizer)  │
   └────────────────────────────────────────────────────────────┘
```

---

## Key Technology Choices

| Concern                | Crate / Tier             | Choice                               | Reason                                                 |
|:-----------------------|:-------------------------|:-------------------------------------|:-------------------------------------------------------|
| Serialization          | `shared` (all)           | `serde`                              | Universal — network payloads, storage, files           |
| Map / config files     | `sim`, `shared`          | `ron`                                | Readable, serde-native scenario + map files            |
| EKF matrix math        | `robot`                  | `nalgebra`                           | Matrix plumbing; the EKF _equations_ stay hand-written |
| 2D visualizer          | `sim`                    | `macroquad`                          | Minimal-boilerplate cross-platform 2D                  |
| Simulation noise       | `sim`                    | `rand`                               | Seeded RNG for reproducible odometry/measurement noise |
| Async runtime          | `api`, `robotd` (future) | `tokio`                              | Industry standard, required by Axum                    |
| Web server + WebSocket | `api` (future)           | `axum`                               | Ergonomic, async-first                                 |
| Database (server)      | `api` (future)           | `sqlx` + SQLite                      | File-based, no server, same API as Postgres            |
| Web dashboard          | `dashboard` (future)     | `leptos`                             | Compiles to WASM, modern Rust web                      |
| Terminal UI            | `tui` (future)           | `ratatui`                            | Feature-rich systems dashboards                        |
| Error handling         | all                      | `thiserror` (libs) + `anyhow` (bins) | Idiomatic per-layer errors                             |

Per the beginner constraint: the core algorithms (EKF, coverage planner) are written from scratch. External crates cover
only non-algorithmic plumbing (rendering, serialization, networking, matrix arithmetic).

---

## Storage Design

Persistence is split across the two tiers:

**Server-side (`api` → SQLite).** The user's intent and history: room definitions, no-go zones, docking schedules,
obstacle thresholds, and session history (timestamp, area covered, duration) for the dashboard to display. Editable live
from the dashboard.

**Robot-side (future).** Whatever the robot needs to resume after a reboot: its known map and last-known pose/state.
Kept local to the device for low-latency access, independent of server connectivity.

---

## Connectivity (Future)

- **User → server:** standard web hosting. The dashboard and `tui` reach the `api` server over HTTPS/WebSocket.
- **Robot → server:** the robot connects _out_ to the server, so it needs no inbound networking and works behind home
  NAT or on 4G. A token check on the handshake is enough for a personal project.
- **Local-only fallback:** `tui` can talk to the robot directly over SSH/USB-serial for diagnostics without the server.

---

## Suggested Build Order

**MVP (simulation):**

1. **`shared`** — define the domain types first. Unblocks every other crate.
2. **`robot`** — build the brain behind the hardware-abstraction trait: EKF localization, boustrophedon coverage, object
   handling, local replanning. Unit-tested with no host.
3. **`sim`** — host the brain with simulated physics + the 2D visualizer (true vs. estimated pose). This completes the
   MVP — a fully validatable robot with no hardware.

**Post-MVP (hardware + server tiers):**

1. **`robotd`** — implement the hardware-abstraction trait on the Pi; expose the internal API over the network.
2. **`api`** — the user-facing server: command the robot, persist config, serve the dashboard.
3. **`dashboard`** — the Leptos UI, once the API contract is stable.
4. **`tui`** — terminal client for local control/diagnostics.

Each step produces a working, testable system. The brain (`robot` + `shared`) never changes shape as the outer tiers are
added.

---

## Future Ideas

- **SLAM** — build/update the map at runtime instead of relying on a pre-loaded one
- **Real sensor fusion** — LiDAR + camera feeding the EKF on hardware
- **Scheduled sweeping** stored server-side, triggered by a timer
- **No-go zones drawn on the dashboard map** and pushed to the robot
- **Multi-robot support** by parameterizing the server with a robot ID
- **OTA firmware updates** triggered from the dashboard
- **Voice commands** via the dashboard (Web Speech API → command)
- **Companion Postgres** on a VPS for long-term analytics, while the robot keeps a local store

---

_Stack: Rust · Cargo Workspace · Serde · nalgebra · macroquad · rand · RON · (future) Tokio · Axum · SQLx · Leptos ·
Ratatui · SQLite_
