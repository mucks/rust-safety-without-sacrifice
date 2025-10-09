# Exercise: The Network Alliance Protocol (Parts 1 – 8)

You are building the **Network Alliance Protocol** — a system that models multiple **network states** (like *Forest City*, *Prospera*, *Zuitzerland*, and *Zuzalu*) collaborating on trade, research, and governance.  

This exercise connects everything you’ve learned so far: **ownership, structs, enums, collections, modules, traits, and generics.**

---

## 🧠 What you’ll practice
- Structs, enums, and pattern matching  
- Ownership and borrowing  
- Vectors, HashMaps, and modules  
- Traits, generics, and trait bounds  
- Default trait implementations  
- Trait objects for polymorphism (`dyn Trait`)

---

## Task 1 — Model the Network States

Each **NetworkState** has:
- `name: String`
- `citizens: Vec<Citizen>`
- `gdp: f64`
- `allies: Vec<String>`

Each **Citizen** has:
- `id: u32`
- `name: String`
- `role: Role`

The **Role** enum should have:
- `Builder`
- `Researcher`
- `Validator`
- `Governor`

**Acceptance criteria**
- Use `#[derive(Debug)]` for easy printing.
- Create and instantiate at least three states (`Forest City`, `Prospera`, `Zuitzerland`).

---

## Task 2 — Define Shared Behavior with Traits

Define a trait `Governable` that provides:
```rust
trait Governable {
    fn population(&self) -> usize;
    fn add_citizen(&mut self, citizen: Citizen);
    fn economic_output(&self) -> f64;
}
```

Then, implement `Governable` for your `NetworkState` struct:
- `population()` → returns citizen count  
- `add_citizen()` → takes ownership of a `Citizen` and pushes to the vector  
- `economic_output()` → returns `gdp / population()` (use safe division)  

**Acceptance criteria**
- Each state implements `Governable`.
- You can call these methods polymorphically.

---

## Task 3 — Generics and Trait Bounds

Create a generic **function** that prints a report for any governable entity:

```rust
fn print_state_report<T: Governable + std::fmt::Debug>(entity: &T)
```

It should print:
- Name (if available, using pattern matching or a custom method)
- Population
- Economic output
- Optional extra info (like number of allies, if the type supports it)

Then call this function for each of your network states.

---

## Task 4 — Composite Behavior (Traits + Structs)

Create a struct `Alliance<T: Governable>` that models a trade or governance alliance between multiple network states.

```rust
struct Alliance<T: Governable> {
    name: String,
    members: Vec<T>,
}
```

Implement methods:
- `new(name: &str) -> Self`
- `add_member(&mut self, member: T)`
- `total_population(&self) -> usize`
- `average_gdp(&self) -> f64`
- `describe(&self)` → prints info about the alliance

**Acceptance criteria**
- Use trait bounds to make it generic.
- Demonstrate with an alliance between *Prospera* and *Zuitzerland*.

---

## Task 5 — Trait Objects and Dynamic Dispatch

Define a trait `Reportable`:
```rust
trait Reportable {
    fn summary(&self) -> String;
}
```

Implement it for both `NetworkState` **and** `Alliance<NetworkState>`.

Then, create a `Vec<Box<dyn Reportable>>` and store different objects in it:
- `Box::new(NetworkState { ... })`
- `Box::new(Alliance { ... })`

Iterate and print each summary dynamically using trait objects.

**Acceptance criteria**
- Demonstrate polymorphism via `Box<dyn Reportable>`.
- The summaries differ meaningfully for each type.

---

## Task 6 — Optional Stretch

- Add a `TechAlliance` struct that derives from `Alliance` but adds a field `research_budget: f64`.
- Implement a custom trait `Researchable` with:
  ```rust
  fn research_efficiency(&self) -> f64;
  ```
- Provide default implementation returning `budget / total_population()`.
- Override for `TechAlliance` to include an efficiency multiplier.

---

## Hints 💡

- Derive `#[derive(Debug, Clone)]` where helpful.  
- Use `impl Trait` syntax for function parameters when appropriate.  
- Pattern matching can be used to access inner values from enums.  
- Don’t overthink error handling — just keep it `f64` division-safe.  
- Use ownership carefully when adding citizens and states.  
- You can make `Role` implement `Display` to print nicely.

---

## ✅ Done When…

- You can:
  - Create and mutate multiple `NetworkState`s,
  - Use traits and generics to print reports,
  - Group states into alliances,
  - Use trait objects for dynamic summaries,
  - And demonstrate everything using only features from **Parts 1 – 8**.

---

### Example Output (for inspiration)

```
Forest City → 3 citizens, GDP = 320 000 000, Economic Output ≈ 106 666 666 per capita
Prospera → 5 citizens, GDP = 780 000 000, Economic Output ≈ 156 000 000 per capita
Alliance “Blue Network” → 2 members, total population = 8, avg GDP = 550 000 000

--- Polymorphic Report ---
[State] Zuitzerland: 4 citizens, 420 000 000 GDP
[Alliance] Zuzalu Collective: 3 members, total population = 12
```

---

### Suggested Structure

```
2_Exercises/
  1_Traits_Exercise/
    network-alliance-protocol/
      Cargo.toml
      src/
        main.rs
      Network_Alliance_Exercise.md
```

Run with:

```bash
cargo run
cargo test
```
