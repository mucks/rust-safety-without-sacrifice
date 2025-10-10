
# Advanced Exercise: Network Logic Simulator (Hard Mode)

Design and implement a **deterministic simulator** for a network state that stresses **advanced Rust concepts**:
- Zero-copy parsing with string slices (`&str`), custom error types (`thiserror`-style without deps), and robust input validation.
- Graph modeling with **weighted edges**, multi-criteria pathfinding, and **DAG scheduling**.
- Trait objects + generics (e.g., `trait Objective` for routing), iterator combinators, and pattern matching.
- **Concurrency** with threads (channels, `Arc<RwLock<_>>`) and **async** (executors or a minimal home-grown future-poll loop).
- Clean separation of modules: parser/IR, planner, simulator, and presenters.

## What you’ll build

A single binary that reads a plain-text DSL and prints deterministic reports. Implement **only standard library** unless you wish to add more; the skeleton compiles without extra crates.

### DSL Overview

- `NODE <name> [weight=<u32>] [role=Validator|Observer]`
- `LINK <a> <b> latency=<ms> capacity=<mbps>` (undirected)
- `SUMMARY` → prints:
  - `NODES: <n>, LINKS: <m>`
  - `ROLES: validators=<v> observers=<o>`
  - `LATENCY_MIN_MAX_MS: <min> <max>`
  - `CAPACITY_SUM_MBPS: <sum>`
  - `CONNECTED: <true|false>`

- `ROUTE <src> <dst> objective=<latency|min_hops|max_bandwidth>` → shortest path by objective.
  - Output: `ROUTE (<objective>) <src>-><dst>: <path> latency=<L> hops=<H> min_capacity=<C>`

- `JOB <id> dur=<ms> req=<cpu:mem> [DEPENDS <id1,id2,...>]`
- `SCHEDULE policy=<fifo|shortest>` → produce a **deterministic plan** (no resource contention logic required; focus on DAG + policy).
  - Output lines:
    - `t=<start>..<end>  <job>  cpu=<c> mem=<m> deps_satisfied=<bool>`
  - Then `ORDER: <comma-separated ids>` and `TOTAL_DURATION_MS: <ms>`

- `VOTE <validator> for=<candidate> round=<r>`
- `CONSENSUS round=<r> quorum=<pct>` → elect the candidate with **most weight** that reaches quorum.
  - Output: `CONSENSUS r<r>: leader=<name> quorum_met=<bool> votes={Alice:2, Bob:1}`

- `FLOW <name> from=<src> to=<dst> size=<MB> chunk=<MB>`
- `RUN_THREADS` → spawn one thread per flow; compute path and per-chunk hop-latency; print:
  - `FLOW <name>: chunks=<k> path=<..> ETA=<ms>` (ETA = 2 * max path latency; deterministic)
  - Finally: `COMPLETE (threads): <name1>, <name2>, ...` in **completion order**
  - Must finish under **500 ms** for provided inputs.

- `ASYNC_SYNC <name> delay=<ms>`
- `RUN_ASYNC` → run async tasks concurrently and print completion lines:
  - `ASYNC done: <name>@<delay>ms` (in **completion order**)
  - Must finish under **500 ms** for provided inputs.

### Expectations

- Prefer **zero-copy parsing** with `&str` slices.
- Model graph with adjacency lists; implement Dijkstra (latency), widest path (max-bandwidth), and BFS (min_hops).
- DAG scheduling via **topological sort**; tie-breakers by policy.
- Use `Arc<RwLock<_>>` for shared state in threaded flows; use channels to collect completion order.
- For async, you may implement a simple timer-based executor or use a single-thread event loop (no external crates required).

### Tests & Timing


## 📚 Allowed Libraries

External crates are **allowed**. You may (optionally) use, for example:
- `tokio` for async runtimes and timers
- `crossbeam` or `flume` for channels
- `petgraph` for graph utilities (if you prefer over a hand-rolled graph)
- `thiserror` for custom error types

> The provided skeleton already includes an optional `tokio` dependency with multi-thread runtime and macros.


Two test cases enforce a **time ceiling of 500 ms** to encourage real concurrency/async. Your implementation should pass on modern multi-core machines.

## 📦 Test Inputs & Expected Outputs

The repository includes 5 cases in `tests/data/*.in` with expected outputs `*.out`. Your program must exactly match them (line-by-line). See the repository for the full content.
