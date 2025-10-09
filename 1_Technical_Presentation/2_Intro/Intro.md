# Introduction to Rust
Welcome to the world of Rust programming! Rust is a systems programming language that focuses on safety, speed, and concurrency. It is designed to be a "safe, concurrent, and practical language" that supports functional and imperative-procedural paradigms.

In this section, we will cover the basics of Rust, including variables and mutability, data types, functions, and control flow. By the end of this section, you will have a solid understanding of the fundamental concepts of Rust programming.

## Variables and Mutability
In Rust, variables are immutable by default, meaning their values cannot be changed once assigned. To create a mutable variable, you need to use the `mut` keyword. For example:

```rust
let x = 5; // immutable variable
let mut y = 10; // mutable variable
y += 5; // y can be changed
```

## Data Types
Rust has several built-in data types, including:
- Scalar types: integers, floating-point numbers, booleans, and characters.
- Compound types: tuples and arrays.
- User-defined types: structs and enums.
- The most commonly used scalar types are `i32` (32-bit integer), `f64` (64-bit floating-point), `bool` (boolean), and `char` (character).
- Tuples are fixed-size collections of values of different types, while arrays are fixed-size collections of values of the same type.
- Here are some examples of data types in Rust:

```rust
let a: i32 = 10; // integer
let b: f64 = 3.14; // floating-point
let c: bool = true; // boolean
let d: char = 'R'; // character
let e: (i32, f64, char) = (10, 3.14, 'R'); // tuple
let f: [i32; 3] = [1, 2, 3]; // array
```

## Functions
Functions in Rust are defined using the `fn` keyword, followed by the function name and parameters. Functions can return values using the `->` syntax. Here is an example of a simple function:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

You can call the function like this:

```rust
let result = add(5, 10);
println!("The sum is: {}", result);
```

## Control Flow
Rust provides several control flow constructs, including `if`, `else`, `loop`, `while`, and `for`. Here are some examples:
```rust
let number = 5;
if number < 10 {
    println!("The number is less than 10");
} else {
    println!("The number is 10 or greater");
}
let mut count = 0;
while count < 5 {
    println!("Count is: {}", count);
    count += 1;
}
for i in 0..5 {
    println!("i is: {}", i);
}
```
