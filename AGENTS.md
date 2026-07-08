# AGENTS.md — Roomba

## First read

[`CLAUDE.md`](./CLAUDE.md) is the primary instruction file — all Socratic-mode rules, locked design decisions, and
diagram conventions live there. Honor it.

## Status

Design complete (8 Excalidraw diagrams in [`docs/`](./docs/)). Workspace skeleton done: 3 crates under `crates/`,
`shared` has module scaffold with domain types, `robot` has trait stubs, `sim` has entrypoint.
Next: slice 1 — driving loop + macroquad visualization.

## Crate layout

```
Cargo.toml  (workspace root, resolver = "3")
crates/
  shared/   — domain types (Pose, Map, Command, Measurement) with validators + tests
  robot/    — brain library: HardwareAbstraction trait, state machine, EKF, coverage. No `main`.
  sim/      — executable: implements HardwareAbstraction, macroquad viz, main loop
  robotd/   — (future) hardware impl for Raspberry Pi
```

## Linting

Clippy with `--all-targets -- -D clippy::pedantic -D clippy::nursery`.

## Design docs

`.excalidraw` files only (not Mermaid). Layout: generous spacing, no filled container boxes (render as black slabs in
dark theme), minimal arrow bends, labels clear of boxes.

## References

- `references/Roomba-Style Autonomous Robot — Project Blueprint.md` — full vision
- `references/Roomba Sim EKF Coverage Validator.md` — MVP PRD
