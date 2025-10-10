use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let _path = &args[1];
        let _content = fs::read_to_string(_path).expect("failed to read input file");

        // TODO: Implement the simple command interpreter described in Tiny_CLI_Lab.md
        // For each line in the input file, parse the command and print exactly one output line.
        // You should support: ADD, MUL, REV, UPPER, COUNT (see the markdown for details).
        return;
    }
    // No default behavior. Run with: cargo run -- tests/data/case1.in
}
