# Implementation Plan — Roomba (std + Tokio, ROS/Gazebo-style)

> Roadmap, not a spec. Each slice lists a **goal**, the **decisions you resolve** while
> building it (you derive + write the code), and the **demo artifact** it produces.
> The 8 diagrams in `docs/` remain the design record; this is just the build order.

## Goal

A portfolio/CV project for a **robotics role**. What it has to _show_, in priority order:

1. A **visibly converging EKF** — estimated pose tracking ground truth, error (RMSE)
   shrinking on beacon updates, covariance visibly collapsing. This is the money shot.
2. A clean **sim ⇄ real hardware seam** (ROS/Gazebo pattern) — demonstrates systems thinking.
3. A **non-blocking concurrent runtime** (one task per sensor) — real robotics-middleware shape.
4. **Polish**: README with a GIF demo leaning on the existing diagrams, a short EKF-math
   writeup, brain unit tests, and metrics.

Depth + polish on a tight core beats a sprawling half-working superset. Core vertical:
**drive → localize (EKF) → cover → polish.** Modes, object-handling, and LiDAR are _future work_.

## Target architecture (the ROS/Gazebo mapping)

| ROS / Gazebo                  | Crate                       | Role                                                                                                                    | std? |
| ----------------------------- | --------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ---- |
| ROS control nodes             | `robot`                     | brain: `plan()`, EKF, coverage. Sync, pure, unit-testable.                                                              | yes  |
| ROS runtime + node wiring     | **`robotd`** _(new)_        | the daemon: owns concurrency, runs the brain, talks to hardware **through the `Hardware` trait**. Generic over the HAL. | yes  |
| Gazebo (headless physics)     | `sim`                       | implements `Hardware` with simulated physics + noise; advances `true_pose`; **logs** state to rerun.                    | yes  |
| RViz / Foxglove (viewer)      | **`rerun`** (external app)   | the visualization: `sim` logs poses/trail/covariance/scalars; the rerun Viewer renders + gives a scrubbable timeline.    | —    |
| ros_control / gazebo_ros seam | `Hardware` trait (`shared`) | the swap point. Sim impl now; real-driver impl later.                                                                   | —    |

**Dependency arrow:** `sim → robotd → robot + shared`. **`robotd` must never depend on `sim`** —
that is what guarantees it builds for the real target with no simulator attached. On the real
robot a different entry point hands `robotd` the real HAL; in sim, `sim`'s `main` hands it the fake one.
Same `robotd::run(hal)`, two callers.

**Bookkeeping this introduces** (do as you touch each area, not up front):

- MVP grows **3 → 4 crates**. Update the "MVP = 3 crates" line in `CLAUDE.md`.
- Amend **`03-concurrency`** and **`05-act-sim-seam`** to show the `robotd`/`sim` split and the
  dependency direction. (Ask me to redraw; you decide the content.)

## How to sequence — three moves, never one big bang

**Decided:** get every component working **single-threaded first**, then multithread. Multithreading
is the end goal (the concurrent `robotd` is the headline), but the **runtime flip is sequenced to
Slice 4**, after the robot + EKF are visibly correct — because until then you can't tell a _race_
from a _Jacobian bug_. The single-threaded version is your correctness oracle; the brain (`plan`, EKF)
is identical either way, so this is not throwaway work — the concurrency is _added_ around a proven core.

So: **make it visible → extract into `robotd` (behavior-preserving) → add concurrency.** Each is
independently verifiable.

---

## Slice 1 — Make it visible (single-threaded, via rerun)

Fixes today's state: the loop runs `step()` **once** and never logs anything;
`ekf_pose`/`trail` are dead code. (The macroquad `draw_*` stubs and `to_pixel` are gone —
we switched the viewer to **rerun**: `sim` logs data, the rerun Viewer renders it.)

- **Goal:** robot drives continuously and its ground-truth pose + trail appear in the rerun Viewer.
- **Decisions you resolve:**
  - **Loop shape (decided):** with rerun there is no in-process render loop, so this is a plain
    synchronous `loop { step(); log(); }` that **lives in `sim::main`** — keep `main` thin
    (construct · tick · log), all logic stays in the libraries. At Slice 4 the *control* loop moves
    into `robotd`'s threads and `main`'s loop demotes to the logging loop; it never leaves `main`.
  - **What to log, under what entity paths** (e.g. `world/robot`, `world/trail`), and on what
    timeline (log `set_time_*` from your `time_ms`) so the scrubber works.
  - Does `act()` apply a position delta or set a velocity the loop integrates? (This is what makes
    it _keep moving_ vs teleport-and-freeze.)
- **Demo artifact:** a moving robot + trail in the rerun Viewer, scrubbable. Save an `.rrd` — that's
  your shareable artifact, no screen-capture needed.
- **Done when:** the robot drives a visible path from the dock in rerun, no EKF, no concurrency.

## Slice 2 — Extract into `robotd` (behavior-preserving)

- **Goal:** the host loop moves **out of `sim`** into `robotd::run(hal)`; `sim` keeps only the
  `Hardware` impl, the physics that advances `true_pose`, and drawing. Behavior identical to Slice 1.
- **Decisions you resolve:**
  - `robotd` API shape: does it own its clock (a `run()` looper) or get stepped by the caller?
    (Coupling control-rate and render-rate is a fine simplification _here_; decoupling is the
    Slice-4 payoff — see open decisions.)
  - Crate wiring: `robotd` generic over `H: Hardware`; who constructs the `Robot`?
- **Demo artifact:** same visual as Slice 1, new architecture — `git diff` shows the seam.
- **Done when:** `robotd` builds without `sim` in its dependency tree; sim still drives the robot.

## Slice 3 — EKF converging + noise (the money shot)

The EKF predict/update is already wired but currently a **no-op**: sim produces perfect ticks from
the command and a perfect beacon from `true_pose`, so belief and truth move in lockstep with nothing
to correct.

- **Goal:** inject noise so belief _drifts_ and the beacon _re-locks_ it; draw estimated pose next to
  true pose; show error shrinking.
- **Decisions you resolve:**
  - Where does each noise enter? (process noise on the physics pose update vs measurement noise on
    each sensor reading — which sensor maps to which of `Q`/`R`?)
  - Covariance `P` → an uncertainty ellipse: compute the ellipse points from `P`'s eigenvectors
    yourself and log them as a line-strip (rerun has no "ellipse from a 2×2" primitive — the math is
    yours, which is good EKF-understanding to show). Log **RMSE** as a scalar each tick → rerun draws
    the time-series plot for free.
  - Should EKF predict use IMU yaw-rate as well as wheel-differential `theta`? (Diagram 6 says
    odometry+IMU; today only wheels.)
- **Demo artifact:** true + estimated pose in the viewer, an uncertainty ellipse that swells between
  beacon fixes and snaps tight on update, and a live RMSE plot beside it. **This is the headline for
  the README** — and it's a scrubbable `.rrd`, not a screen-grab.
- **Done when:** estimate visibly tracks truth and error stays bounded under injected noise.

## Slice 4 — Concurrency (the `robotd` payoff)

- **Goal:** flip `robotd`'s internals to Tokio — one task per sensor publishing **latest-only**, plan
  on a `tokio::interval` reading the latest without blocking, act consuming commands; physics on its
  own clock. Robot never stalls on a slow sensor.
- **The macroquad/Tokio spike is gone.** Because rerun is a **separate process** and logging is a
  plain synchronous call, there's no main-thread-owning render loop to fight Tokio for. This is the
  hidden payoff of the rerun switch: async now enters **only because you chose it** for the sensor
  tasks, not because a renderer forced it. `RecordingStream` is cheap to `Clone` and share across
  tasks, so each task can log directly.
- **Decisions you resolve:**
  - Channel choice per edge: `watch` for latest sensor values, `mpsc` for commands, `Arc<RwLock<…>>`
    for the shared world model (per Diagram 3).
  - **Trait granularity:** the monolithic `sense() -> SensorFrame` splits here — each sensor task
    needs its _own_ read. What replaces the atomic frame, and what does `None`/staleness mean once
    the streams are independent?
  - Control clock (~10 Hz) vs physics/log clock — now genuinely separate.
- **Demo artifact:** add latency jitter to one sensor and show the robot keeps moving / plan keeps
  ticking — the thing single-threaded couldn't do. Amend diagrams 03 & 05 here.
- **Done when:** sensors, plan, act, and physics run concurrently; no stalls; EKF still converges
  (your Slice-3 oracle confirms the runtime didn't break the math).

## Slice 5 — Coverage

- **Goal:** boustrophedon sweep in the brain; O(1) coverage grid; draw cleaned cells.
- **Decisions you resolve:** U-turn decomposition (rotate 90° · forward one robot-width · rotate 90°);
  `cell = floor(pos / cell_size)`; how coverage % is computed and drawn. (Diagram 7.)
- **Demo artifact:** stripes filling the room; a live **coverage %** metric.
- **Done when:** robot covers a room "good enough" and the grid shows it.

## Future work (label as such in the README — do not scope-creep the core)

- State machine / modes (Diagram 2): Idle → Moving → Cleaning → Charging → Fault.
- Object handling (Diagram 8): drive-over dirt vs replan-around obstacle.
- LiDAR: surprise-obstacle detection + scan-matching.
- **Real HAL / `robotd` on a Raspberry Pi** — the seam already exists; this is the payoff story.
  (A future no-OS motor-firmware tier could go `no_std`/Embassy — mention as design foresight.)

## Cross-cutting deliverables (CV polish — start early, finish alongside the core)

- **README** with a demo (rerun `.rrd` recording and/or a GIF exported from the Viewer), embedding
  the 8 diagrams as the architecture story. Logging to rerun **reads as robotics tooling** to a
  reviewer — that recognition is part of the deliverable.
- **Short EKF-math writeup** (motion model `f`, measurement `h`, Jacobians `F`/`H`, gain) — proves
  you understand it, not just wired a crate.
- **Unit tests on the brain** — `plan()` is sync and pure precisely so this is easy; it's also your
  race-vs-math oracle.
- **Metrics**: localization RMSE over time, coverage %.
