
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let _path = &args[1];
        // Students should: read input, build states/alliances, and print output.
        let _content = fs::read_to_string(_path).expect("failed to read input file");
        // TODO: Implement parsing and behavior per the exercise.
        return;
    }
}
