# Exercise: Mini “Network State” Directory (Parts 1–5)

Design a tiny in-memory directory for a budding **network state**. Citizens join regions, can move between them, and some become validators. Keep it simple and stick strictly to concepts from **Setup, Intro, Ownership, Structs & Enums, Collections** (Parts 1–5).

---

## What you’ll practice
- Variables & mutability; functions & control flow  
- **Ownership**, **borrowing**, and **slices** (`&str`)  
- **Structs & Enums**  
- **Collections**: `Vec`, `String`, `HashMap`

> No traits/generics, no error handling beyond simple booleans, no lifetimes annotations (beyond elision), no async/threads.

---

## Task 1 — Model the world

- Create an enum `Role` with variants: `Citizen`, `Validator`, `Founder`.
- Create a struct `Member` with fields:  
  `id: u32`, `name: String`, `role: Role`.
- Create a struct `Network` that stores members by region:  
  `regions: HashMap<String, Vec<Member>>` (maps **region name → members**).

**Acceptance criteria**
- Types compile and can be instantiated in `main`.
- `println!("{:?}", ...)` works for quick debugging (you may derive `Debug`).

---

## Task 2 — Core operations (methods on `Network`)

Implement the following methods:

1. **`new() -> Self`**  
   Initialize the internal `HashMap`.

2. **`register_member(&mut self, region: &str, member: Member)`**  
   - Insert the `member` into the region’s vector (create it if missing).  
   - Demonstrates **ownership move** into the collection.

3. **`move_member(&mut self, from: &str, to: &str, id: u32) -> bool`**  
   - Remove the member with `id` from `from`, insert into `to`.  
   - Return `true` if moved, `false` if not found.  
   - Demonstrates moving ownership between vectors.

4. **`count_region(&self, region: &str) -> usize`**  
   - Read-only operation using borrowing.

5. **`members_with_prefix(&self, region: &str, prefix: &str) -> Vec<&Member>`**  
   - Return **borrowed** references to members whose `name` starts with `prefix`.  
   - Uses **slices** (`&str`) and borrowing.

6. **`role_stats(&self) -> (usize, usize, usize)`**  
   - Count `(citizens, validators, founders)` across all regions using `match`.

**Acceptance criteria**
- All functions compile and behave as specified on simple examples.
- No heap clones unless intentional (prefer borrowing for read-only queries).

---

## Task 3 — Driver program (`main`)

- Create a `Network`.
- Register at least **5 members** across **2–3 regions**.
- Make one member join as a `Validator`; make another a `Founder`.
- Call `move_member` to move one member between regions and print the result.
- Print counts per region using `count_region`.
- Demonstrate a **safe ASCII** slice with `&str`, e.g.:
  ```rust
  let name = "Alice";
  let prefix = &name[0..1]; // OK for simple ASCII examples
  ```
- Use `members_with_prefix("ForestCity", "F")` (or any region/prefix you choose) and print the names/ids.
- Print `role_stats()`.

**Acceptance criteria**
- Program runs and prints meaningful output showing each feature.

---

## Task 4 — Stretch (still ≤ Part 5)

If your students finish early or want an additional challenge, extend the network to include more structure and functionality — still staying within the limits of **Parts 1–5**.

### 🧱 Stretch A — Listing all members

Add a new method to `Network`:

```rust
fn list_all_members(&self) -> Vec<&Member>
```

- It should iterate through all regions and collect borrowed references (`&Member`) into a flat `Vec`.
- Use a simple `for` loop with nested iteration.

**Example:**
```rust
let all = network.list_all_members();
println!("Total members: {}", all.len());
for m in all {
    println!("{} ({:?})", m.name, m.role);
}
```

### 🌍 Stretch B — Region info and alliances

Create a `RegionInfo` struct:
```rust
struct RegionInfo {
    name: String,
    description: String,
}
```

Then, extend your `Network` struct with a new field:
```rust
alliances: Vec<String>,
```

Add a method:
```rust
fn add_alliance(&mut self, alliance: &str)
```

This should push a new alliance name to the `alliances` vector.  
Print the alliances when you display network information.

**Example:**
```rust
network.add_alliance("ForestCity ↔ Zuitzerland");
println!("Alliances: {:?}", network.alliances);
```

### ✅ Stretch Acceptance Criteria
- The new functions compile and are demonstrated with example output.
- You use only standard library collections (`Vec`, `HashMap`).
- All borrowing rules and ownership principles still apply.

---

## Hints

Here are some helpful reminders and Rust tricks relevant to this exercise:

### 🧭 Ownership & Borrowing
- Functions taking `&self` are **read-only** (shared borrow).
- Functions taking `&mut self` can **mutate internal data** (exclusive borrow).
- Move ownership only when necessary (e.g. transferring `Member` structs between regions).

### 📦 Working with HashMaps
- Use `entry` for inserting or updating in one line:
  ```rust
  let region_members = self.regions.entry(region.to_string()).or_insert_with(Vec::new);
  region_members.push(member);
  ```
- `or_insert_with(Vec::new)` creates a new empty `Vec` only when needed.

### 🧩 Iteration & Filtering
- To find something inside a vector:
  ```rust
  if let Some(idx) = vec.iter().position(|m| m.id == id) {
      vec.remove(idx);
  }
  ```
- To filter by prefix:
  ```rust
  if m.name.starts_with(prefix) { ... }
  ```

### 🧵 Debugging & Printing
- Use `#[derive(Debug)]` on structs/enums for quick output.
- Print with:
  ```rust
  println!("{:?}", my_struct);
  ```

### 💡 Miscellaneous
- Prefer slices (`&str`) over `String` in function parameters where possible.
- Use pattern matching (`match`) for clean role counting:
  ```rust
  match m.role {
      Role::Citizen => c += 1,
      Role::Validator => v += 1,
      Role::Founder => f += 1,
  }
  ```
- Iterate over the entire map:
  ```rust
  for (region, members) in &self.regions {
      println!("Region: {}", region);
      for m in members {
          println!("  - {}", m.name);
      }
  }
  ```
- Stick to ASCII for string slicing (avoid multibyte UTF-8 issues).

---

## Done when…

- You can:
  - Register and move members,
  - Query counts and prefixes,
  - Summarize role statistics,
  - And your code uses only concepts from **Parts 1–5**.

---

### Suggested structure

Place in your exercise 

```
1_Exercises/
  0_Simple_Exercise/
    Network_State_Directory.md
    network-state-directory/
      Cargo.toml
      src/
        main.rs
```

Then run:

```bash
cargo run
cargo test
```

---

### Example Output (for inspiration)

```
Moved Bob to ForestCity? true
ForestCity count: 3
Zuitzerland count: 1
Prospera count: 1
First letter of Alice is 'A'
ForestCity members with prefix 'A':
  - Alice (id 1)
  - Akira (id 2)
Role stats -> Citizens: 3, Validators: 1, Founders: 1
Alliances: ["ForestCity ↔ Zuitzerland"]
Total members: 5
```
