# TODO — get the sim to a running position

A guided checklist **for you** (not an agent). Ordered so each group leaves the project
in a working state. "Running position" is defined at the bottom. Companion doc:
`docs/implementation-plan.md` (the why); this is the what-to-touch.

Convention: each item is an **outcome + where it lives + the decision you make**. The code is yours.

## 1 · Make it run and show up in rerun — plan Slice 1

Right now the loop runs **once** and logs nothing. There is no draw layer anymore — you **log** state
and the rerun Viewer renders it. That deletes the whole `to_pixel`/y-flip/draw-stub pile of work.

- [ ] **The loop runs once.** `run()` (`sim.rs`) calls `step()` a single time. With rerun there is
      **no async render loop** — make it a plain `loop { step(); log(); }` **in `main`** (decided).
      Keep `main` thin: construct · tick · log; all logic stays in the libraries. `SimHost::run()`
      stops owning a loop — it collapses into the pieces `main` calls.
- [ ] **Drop the macroquad `main`.** `main.rs` still carries `#[macroquad::main]` + `next_frame().await`.
      With rerun the entry point is an ordinary `fn main()`. **Decide** the entry shape.
- [ ] **Log the world each tick:** room outline (line-strips), robot pose, trail. **Decide** your entity
      paths (`world/robot`, `world/trail`, …) and set the timeline from `time_ms` so the scrubber works.
      Logging `trail` kills its dead-code warning.

## 2 · Make the motion continuous, not teleporting — plan Slice 1

- [ ] **`act()` applies a position delta.** `sim.rs:59-68` does `true_pose.translate(d, θ)` once per
      control tick — the robot jumps 100 ms at a time. **Decide:** store the commanded velocity as a
      **setpoint** and integrate `true_pose` on the physics clock (your loop tick), so it glides
      between control ticks. (Coupling control-dt = physics-dt is an acceptable _temporary_ shortcut —
      just know you made it.)

> ✅ **Checkpoint — you are now at "running position."** Stop here if you just want it running.
> Groups 3-4 are the refactor + making the EKF mean something.

## 3 · Extract `robotd` (the architecture refactor) — plan Slice 2

Behavior-preserving: the picture shouldn't change, only where the code lives.

- [ ] Create the `robotd` crate; add it to `members` in root `Cargo.toml:3-5` and a
      `robotd = { path = "crates/robotd" }` line under `[workspace.dependencies]`.
- [ ] Move the host loop (`run`/`step`, `sim.rs`) into `robotd::run(hal)` generic over
      `H: Hardware`. `sim` keeps only: the `Hardware` impl, the `true_pose` physics, and rerun logging.
- [ ] `sim`'s `main` calls `robotd::run(SimHost::new())`.
- [ ] **Guardrail:** `cargo tree -p robotd` must **not** list `sim`. If it does, the dependency
      arrow is backwards and the "runs on the real robot" story breaks.
- [ ] **`step()` returns a discarded 5-tuple** (`sim.rs`). Decide what the logger actually
      needs (true pose, ekf pose, trail) and how it reads it — return value vs shared state.

## 4 · Make the EKF do real work — plan Slice 3

- [ ] **Sensors are noise-free**, so the EKF just mirrors truth (`plan` runs predict+update but has
      nothing to correct — `lib.rs:47-61`). In `sim/src/sensors.rs`, ticks come straight from the
      command and the beacon straight from `true_pose`. Add **measurement noise** there, and
      **process noise** in `act()`'s physics update.
- [ ] Log `ekf_pose` alongside `true_pose` (kills the last warning) and an uncertainty ellipse —
      compute its points from `P`'s eigenvectors and log them as a line-strip (rerun has no
      2×2-covariance primitive). Log the true-vs-estimate error as a **scalar** each tick → rerun
      draws the RMSE plot for free. This is the headline demo — see the plan.

## 5 · Bookkeeping (do when you touch the area)

- [ ] `CLAUDE.md` says "MVP = 3 crates" — update to 4 once `robotd` exists.
- [ ] Ask me to amend diagrams **03-concurrency** and **05-act-sim-seam** for the `robotd`/`sim`
      split (you decide the content, I transcribe).

---

## Definition of "running position"

`cargo run -p sim` launches the rerun Viewer; a robot drives a **visible, continuous path** from the
dock with a trail; the loop ticks steadily; no panics; `cargo check` is clean; you can save/scrub an
`.rrd`. EKF convergence, concurrency, and coverage come **after** this — they're groups 3-4 here and
the later slices in the plan.
