
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let _path = &args[1];
        let _content = fs::read_to_string(_path).expect("failed to read input file");
        // TODO: implement processing logic
        return;
    }
}
