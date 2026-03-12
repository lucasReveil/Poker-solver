# Poker Solver (Rust) — Postflop Heads-Up NLHE CFR MVP

This repository contains a **production-style foundation** for a deterministic, testable heads-up no-limit Texas Hold'em postflop solver.

## What this project is

- A Rust codebase with explicit architecture for cards, ranges, game state, tree compilation, terminal utility, and CFR solving.
- A deterministic CLI workflow driven by JSON config.
- A strict MVP focused on **correctness and maintainability** over speculative optimizations.

## What this project is not (yet)

- Not a full PioSolver/GTO Wizard equivalent.
- Not preflop solving.
- Not exact exploitability reporting.
- Not distributed/cloud solving.
- Not a GUI product.

## Current Scope

- Heads-up postflop only.
- Discrete action abstraction.
- CFR-based approximate equilibrium iteration.
- Range parsing (`AA`, `AKs`, `AKo`, comma-separated, optional `:weight`).
- Board card removal and combo-level filtering.
- Showdown and fold utility logic.
- JSON export of iteration statistics.

## Architecture

Workspace:
- `crates/solver`: library + CLI binary.

Main modules:
- `cards`: compact card representation, board parsing, 7-card evaluator.
- `ranges`: range syntax parsing, combo expansion, board filtering.
- `game`: player/street/state definitions.
- `tree`: action config and deterministic internal compiled tree.
- `solver`: payoff logic, solver tables, CFR loop, iteration stats.
- `io`: config loading and JSON export.

## Algorithm

Implemented variant:
- **Vanilla CFR** with regret-matching and cumulative average strategy tracking.

Notes:
- This MVP uses a simplified deterministic tree compiler and full-range terminal aggregation.
- Convergence metrics are limited to root EV and average absolute regret magnitude.
- No claim of exact Nash or exact exploitability.

## Build & Test

```bash
cargo build
cargo test
cargo bench
```

## CLI Usage

```bash
cargo run -p poker_solver -- \
  --config examples/river_spot.json \
  --out out/river_result.json
```

The solver prints final iteration metrics and optionally writes JSON export.

## Config Format

See `examples/river_spot.json`.

Required top-level keys:
- `board`: 5-card board string (current terminal evaluator path requires river board).
- `oop_range`, `ip_range`: range strings.
- `game`: pot/stack/rake config.
- `tree`: discrete action-tree configuration.
- `iterations`: number of CFR iterations.

## Honest Limitations / Tradeoffs

1. **Tree compiler scope**: currently compiles a small deterministic template (check/bet/call/fold branches).
   - Tradeoff: high confidence and deterministic behavior now vs broad abstraction coverage.
2. **Terminal handling**: currently evaluates fully specified 5-card boards only.
   - Tradeoff: correctness and explicitness over partial-board chance rollout complexity in v1.
3. **CFR granularity**: regret tables are node-action level, not information-set compressed.
   - Tradeoff: simpler and auditable MVP at the cost of scalability.
4. **Performance**: straightforward evaluator and nested range loops.
   - Tradeoff: clarity and determinism first; future optimization guided by profiling.

## Validation Strategy

Included tests cover:
- card encoding/parsing roundtrips,
- board duplicate rejection,
- range parsing/expansion,
- board card-removal filtering,
- tree compilation sanity,
- terminal utility correctness/determinism,
- deterministic solver smoke regression.

Benchmarks cover:
- range expansion,
- small CFR solve workload.

## Roadmap

Near-term extensions:
- Street-aware chance transitions (flop/turn rollout support).
- Richer compiled tree from config (multiple bet/raise branches by street).
- Information-set indexing with per-hand strategy output.
- Best-response/exploitability approximation.
- Profiling-driven optimizations (e.g., evaluator acceleration, traversal caching).

## Determinism Notes

- No RNG is required for the current algorithm path.
- Iteration order is stable via deterministic data structures and fixed traversal sequence.

