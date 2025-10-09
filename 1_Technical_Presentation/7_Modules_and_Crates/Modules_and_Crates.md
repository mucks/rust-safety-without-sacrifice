# Modules and Crates

Modules and crates are fundamental concepts in Rust that help organize code into manageable and reusable components. They provide a way to encapsulate functionality, control visibility, and manage dependencies in a Rust project.

## Modules
Modules in Rust are used to organize code within a crate. They allow you to group related functions
, structs, enums, and other items together. Modules can be nested, creating a hierarchy that reflects the structure of your code.
To define a module, you use the `mod` keyword. Here is an example of how to create and use modules in Rust:
```rust
// main.rs
mod math; // Declare the math module
fn main() {
    let sum = math::add(5, 3);
    println!("The sum is: {}", sum);
}
```
```rust
// math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```
In this example, we have a `math` module defined in a separate file (`math.rs`). The `add` and `subtract` functions are made public using the `pub` keyword, allowing them to be accessed from outside the module.

You can also create nested modules by defining modules within other modules. For example:
```rust
// main.rs
mod geometry {
    pub mod shapes {
        pub fn area_of_circle(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }
}
fn main() {
    let area = geometry::shapes::area_of_circle(5.0);
    println!("Area of the circle: {}", area);
}
```

In this example, we have a `geometry` module that contains a nested `shapes` module. The `area_of_circle` function is public and can be accessed using the full path.

## Crates
A crate is the smallest unit of code that the Rust compiler considers. It can be a binary crate (an executable) or a library crate (a reusable library). Each Rust project is a crate, and you can create multiple crates within a workspace.
To create a new crate, you can use `cargo new` for a binary crate or `cargo new --lib` for a library crate. For example:
```sh
cargo new my_binary_crate
cargo new my_library_crate --lib
```