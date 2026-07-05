# Product Requirements Document

## Roomba-Style Autonomous Robot Simulator

**Track:** Embedded / Robotics
**Difficulty:** Beginner
**Timeline:** 6 Weeks

---

## 1. Problem Statement

Developing and testing autonomous robot behavior on physical hardware is expensive, slow, and risky — especially early
in development. This project solves that by providing a fully simulated environment where a Roomba-style autonomous
cleaning robot can be built, tested, and validated before ever touching real hardware.

The primary user is the developer themselves, using the simulator to validate robot intelligence: coverage sweeping,
object handling, obstacle avoidance, and localization — all within a known map.

---

## 2. Goals & Success Criteria

The project is considered **done** when the simulated robot can:

1. **Sweep an entire map** using a structured boustrophedon (lawnmower) pattern, respecting user-defined no-go zones.
2. **Detect and "clean" small objects** (dirt) embedded in the map data — removing them from the simulation once the
   robot passes over them.
3. **Classify and handle large objects** — ignore them from cleaning, and instead plan a path to either drive around or
   over them depending on size.
4. **Localize itself** on a known, pre-loaded map using a hand-written Extended Kalman Filter (EKF) that fuses noisy odometry/IMU motion with periodic docking-station beacon corrections.
5. **Visualize all of the above** in a live 2D top-down view.

---

## 3. Core Features (MVP)

### 3.1 Pre-Loaded Map with Embedded Objects

The robot is given full knowledge of the environment at startup. The map includes:

- Room boundaries and no-go zones
- Small objects ("dirt") to be cleaned
- Large objects (obstacles) to navigate around or over

No runtime sensor detection in this phase — object awareness comes from the map data itself.

The simulator also provides noisy motion and reference measurements for localization, including odometry, IMU data, and
docking-station beacon observations.

### 3.2 EKF-Based Localization (Hand-Written)

The robot uses an Extended Kalman Filter, written from scratch by the developer, to estimate its 2D pose (`x`, `y`,
heading) within a known, pre-loaded map.

The EKF predicts the robot's motion using simulated wheel odometry and IMU data, both of which include noise and drift.
To prevent error from accumulating indefinitely, the robot periodically corrects its pose estimate using observations of
the docking station, which acts as a fixed reference beacon at a known world position.

For MVP, the docking station provides a simple simulated measurement such as range and bearing relative to the robot.
This keeps the localization problem focused on sensor fusion and state estimation rather than SLAM or scan matching.

This is the primary learning artifact of the project: implementing the EKF's motion model, measurement model,
Jacobians, predict/update steps, and covariance tuning by hand.

### 3.3 Boustrophedon Coverage Sweeping

The robot plans a structured lawnmower-pattern path across the navigable area of the map. No-go zones are excluded from
the sweep plan. This is the default operating mode.

### 3.4 Object Interaction Logic

When the robot encounters an object during its sweep:

- **Small objects (dirt):** The object is "deleted" from the map — considered cleaned.
- **Large objects (obstacles):** The robot does not attempt to clean them. Instead, it replans its local path to
  navigate around or over the obstacle and resume the sweep.

### 3.5 2D Visualizer

A real-time top-down 2D view rendered on the developer's machine showing:

- The room map, walls, and no-go zones
- Current robot position and heading
- Remaining dirt objects
- Large obstacles
- The robot's planned path

---

## 4. User Stories / Use Cases

### Scenario 1: Standard Cleaning Run

The user launches the simulator with a map file. The robot starts at a defined position, computes its boustrophedon
sweep path, and begins cleaning. The user watches in the 2D visualizer as dirt objects disappear and the robot
methodically covers the room. The run ends when the entire navigable area has been swept.

### Scenario 2: Obstacle Encountered Mid-Sweep

During a sweep, the robot reaches a large obstacle. It classifies the object as too large to clean, replans its local
path to navigate around it, and continues the sweep from where it left off. The obstacle remains visible in the
visualizer throughout.

### Scenario 3: No-Go Zone Enforcement

The user has defined a no-go zone (e.g., a pet's feeding area) in the map data. The robot's sweep planner excludes this
zone entirely — the robot never enters it, and the visualizer renders it clearly as an off-limits area.

---

## 5. Scope & Constraints

### In Scope (MVP)

- Simulation only — no physical hardware
- Pre-loaded, static map with embedded object data
- Hand-written EKF for 2D pose localization
- Simulated odometry and IMU prediction with docking-station beacon correction
- Boustrophedon sweep planning with local obstacle replanning
- Object classification (small = clean, large = navigate)
- 2D top-down visualizer running on the developer's machine
- Single Cargo workspace (monorepo) structure for future extensibility

### Out of Scope (MVP)

- SLAM or runtime map generation
- LiDAR scan matching
- Real sensor input (camera, LiDAR, hardware IMU)
- Runtime object detection — objects are map-defined for now
- Web dashboard or mobile UI (Leptos/Axum stack) — future phase
- Terminal UI (Ratatui) — future phase
- Persistent storage (SQLite) — future phase
- Remote access, WebSocket API, authentication
- Multi-robot support
- Scheduled cleaning or voice commands

### Stretch Goal (If Time Allows)

- Replace the 2D visualizer with a basic [Leptos](https://github.com/leptos-rs/leptos) web dashboard served locally

### Constraints & Dependencies

- Rust beginner level — all core algorithms (EKF, sweep planner) written from scratch for learning value; external
  crates used only for non-algorithmic concerns (rendering, serialization, etc.)
- Simulation must be portable and runnable on a developer laptop (Linux/macOS/Windows)
- Architecture must be designed with physical hardware deployment in mind — clean separation between robot logic and
  simulation layer so the transition to Raspberry Pi is straightforward

---

*Future Phases (post-MVP): physical hardware on Raspberry Pi Zero 2 W, real LiDAR + camera sensor fusion, full web
dashboard, persistent map storage, remote access.*
