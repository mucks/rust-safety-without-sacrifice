# Error Handling

In Rust, error handling is primarily done through the use of the `Result` and `Option` types, as well as the `panic!` macro for unrecoverable errors. This section will cover these three main aspects of error handling in Rust.

## panic! macro
The `panic!` macro is used to indicate that a program has encountered an unrecoverable error and needs to terminate. When `panic!` is called, the program will print an error message and unwind the stack, cleaning up resources as it goes. Here is an example of using the `panic!` macro:

```rust
fn main() {
    let v = vec![1, 2, 3];
    // This will cause a panic because we are trying to access an out-of-bounds index
    println!("{}", v[99]);
}
```
When you run this code, it will panic and display an error message indicating that you attempted to access an index that is out of bounds.
You can also use `panic!` with a custom message:

```rust
fn main() {
    let condition = false;
    if !condition {
        panic!("The condition was false, panicking!");
    }
}
```

## Result type
The `Result` type is used for functions that can return an error. It is an enum with two variants: `Ok(T)` for successful results and `Err(E)` for errors. Here is an example of using the `Result` type:   

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}   
fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```
In this example, the `read_file_contents` function attempts to open and read a file. If successful, it returns the file contents wrapped in `Ok`. If an error occurs, it returns the error wrapped in `Err`. The `?` operator is used to propagate errors.