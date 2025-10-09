# Exercise: Network State Simulation Engine (Advanced Topics)

Welcome to the **Network State Simulation Engine**, a continuation of the *Network Alliance Protocol*.  
This project introduces **advanced Rust concepts** — lifetimes, closures & iterators, smart pointers, concurrency, and async/await — all in the context of simulating collaboration between modern network states like **Forest City**, **Prospera**, **Zuitzerland**, and **Zuzalu**.

---

## 🧠 What you’ll practice
- Lifetimes and reference management  
- Closures and iterators for simulation logic  
- Smart pointers (`Box`, `Rc`, `RefCell`) for flexible ownership  
- Concurrency with threads and channels  
- Asynchronous programming with `async`/`await`  

---

## Task 1 — Shared Knowledge Pool (Lifetimes & References)

Each **NetworkState** maintains a set of `knowledge_entries`, which are `&str` references to static ideas shared between states.

Create:
```rust
struct KnowledgeBase<'a> {
    entries: Vec<&'a str>,
}
```

Then define:
```rust
fn longest_entry<'a>(a: &'a str, b: &'a str) -> &'a str
```
Return the longer entry — this function demonstrates **lifetimes**.

Finally, attach a `KnowledgeBase` to each `NetworkState`:
```rust
struct NetworkState<'a> {
    name: String,
    knowledge: KnowledgeBase<'a>,
}
```

**Acceptance criteria**
- Use lifetime annotations correctly so the program compiles.
- Print the longest entry between two ideas shared by Forest City and Zuitzerland.

---

## Task 2 — Iterators & Closures (Simulated Data)

Use closures and iterators to simulate GDP growth.

Start with:
```rust
let gdps = vec![320_000_000.0, 780_000_000.0, 420_000_000.0];
```

- Use `.iter().map(|g| g * 1.05).collect::<Vec<_>>()` to simulate 5% growth.
- Use a closure to filter only states with GDP > 500,000,000.
- Chain iterator calls elegantly (no explicit `for` loops).

**Acceptance criteria**
- You must use **map**, **filter**, and **collect**.
- Output before/after GDP values using formatted printing.

---

## Task 3 — Smart Pointers for Shared State

Introduce a shared **InnovationHub** managed by multiple network states.

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct InnovationHub {
    patents: RefCell<Vec<String>>,
}
```

Each state holds an `Rc<InnovationHub>`.  
They can add new patents via:
```rust
fn submit_patent(&self, name: &str)
```
This should `borrow_mut()` from the `RefCell` and push the patent name.

**Acceptance criteria**
- Demonstrate two different states sharing the same hub.
- After both add patents, print all registered patents.
- Explain why `Rc<RefCell<T>>` is used instead of `Box<T>` or raw references.

---

## Task 4 — Concurrency (Threads & Channels)

Now simulate concurrent events:  
Each state produces a message describing its daily report and sends it through a channel.

- Use `std::sync::mpsc::channel()`
- Spawn one thread per state (`thread::spawn`)
- Each thread should send a `String` message like:
  ```
  "[Forest City] Energy production increased by 3%"
  ```
- In the main thread, receive all messages and print them as they arrive.

**Acceptance criteria**
- Use at least 3 spawned threads.
- Collect messages and print them in order received.
- Join all threads safely at the end.

---

## Task 5 — Async Network Synchronization

Now simulate asynchronous collaboration between network states.

Use **Tokio** runtime for simplicity (add `tokio` to `Cargo.toml`):
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Write an async function:
```rust
async fn sync_state(name: &str, delay_secs: u64)
```

It should:
- Await a `tokio::time::sleep(Duration::from_secs(delay_secs))`
- Then print:
  ```
  "{name} synced successfully after {delay_secs}s"
  ```

Run several tasks concurrently:
```rust
tokio::join!(
    sync_state("Forest City", 3),
    sync_state("Prospera", 1),
    sync_state("Zuitzerland", 2),
);
```

**Acceptance criteria**
- All tasks run concurrently.
- Output order reflects completion times (not function order).

---

## Task 6 — Stretch: Global Simulation Engine

Combine everything into a `Simulation` struct:
```rust
struct Simulation<'a> {
    states: Vec<NetworkState<'a>>,
    hub: Rc<InnovationHub>,
}
```

Add methods:
- `run_concurrent_reports(&self)` — spawns threads for each state report.  
- `sync_all(&self)` — async version of state synchronization.  
- `analyze(&self)` — uses iterators to find top GDP states.  

**Bonus**
- Add a closure inside `analyze()` that classifies states as *developed* or *emerging*.
- Store classification results in a `HashMap<String, String>`.

---

## ✅ Done When…

- You’ve demonstrated **lifetimes**, **closures**, **Rc/RefCell**, **threads**, and **async** in a cohesive network simulation.
- Each advanced concept is represented by one part of the system.
- You can explain why and where each advanced feature is used.

---

### Example Output (for inspiration)

```
--- Lifetimes ---
Longest entry: “Distributed Identity Protocol”

--- Iterators & Closures ---
Before: [320000000.0, 780000000.0, 420000000.0]
After:  [336000000.0, 819000000.0, 441000000.0]

--- Smart Pointers ---
Patents in shared hub: ["Rust-based DAO", "Sovereign Compute Network"]

--- Concurrency ---
[Prospera] Energy production increased by 3%
[Zuitzerland] Launched new validator program
[Forest City] Upgraded decentralized grid

--- Async Synchronization ---
Prospera synced successfully after 1s
Zuitzerland synced successfully after 2s
Forest City synced successfully after 3s
```

---

### Suggested Structure

```
2_Exercises/
  2_Advanced_Exercise/
    Network_Simulation_Exercise.md
    network-simulation-engine/
      Cargo.toml
      src/
        main.rs
```

Run with:
```bash
cargo run
cargo test
```
