
# Tiny CLI Lab (Live-Coding Friendly)

A tiny command interpreter you can live-code in class. Your program reads a **file path** as its first CLI argument, processes the file **line by line**, and writes **one line of output per command**.

## Commands to implement

Each non-empty, non-comment line starts with a keyword:

- `ADD a b` — print the sum of two integers `a + b`.
- `MUL a b` — print the product `a * b`.
- `UPPER text…` — print the rest of the line uppercased.
- `REV text…` — print the rest of the line with **word order reversed** (split on spaces).
- `COUNT text…` — print the number of characters in the rest of the line (no trimming).

Notes:
- Lines starting with `#` are comments and should be ignored.
- Inputs are simple; feel free to use `split_whitespace()` and `splitn()`.
- Output must match exactly; each command produces exactly one output line.

## Example (from the tests)

**Input**
```
ADD 2 3
MUL 4 5
UPPER hello world
REV rust is fun
COUNT banana
```

**Output**
```
5
20
HELLO WORLD
fun is rust
6
```

## How to run

```bash
cd tiny-cli-lab
cargo test                # runs all 5 tests (will FAIL until you implement main)
cargo run -- tests/data/case1.in
```

## Hints to live-code quickly

- Use `env::args()` to get the path; `fs::read_to_string(path)?` to read.
- Iterate `for line in content.lines()`; `if line.trim().is_empty() || line.starts_with('#') { continue; }`
- Parse with:
  - `split_whitespace()` to get the first token
  - `splitn(2, ' ')` to get the command + the rest of the line for text commands
- For `REV`, split `rest.split_whitespace().collect::<Vec<_>>()`, then `reverse()` and `join(" ")`.

Have fun!
