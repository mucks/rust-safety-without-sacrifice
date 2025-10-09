# Collections
In this section, we will explore three primary collection types in Rust: Vectors, Strings, and HashMaps. Collections are used to store multiple values in a single data structure.

## Vectors
Vectors are a dynamic array type that can grow and shrink in size. They are defined using the `Vec<T>` type, where `T` is the type of elements stored in the vector.
Here is an example of how to create and use a vector in Rust:

```rust
fn main() {
    // Create a new vector
    let mut numbers: Vec<i32> = Vec::new();
    // Add elements to the vector
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    // Access elements in the vector
    for number in &numbers {
        println!("{}", number);
    }
    // Remove the last element
    numbers.pop();
    println!("After pop: {:?}", numbers);
}
```

You can also create a vector with initial values using the `vec!` macro:

```rust
let fruits = vec!["Apple", "Banana", "Cherry"];
println!("{:?}", fruits);
```

## Strings
Strings in Rust are UTF-8 encoded and are represented by the `String` type. They are growable and mutable. Here is an example of how to create and manipulate strings:
```rust
fn main() {
    // Create a new String
    let mut greeting = String::from("Hello");
    // Append to the String
    greeting.push_str(", world!");
    println!("{}", greeting);
    // Concatenate Strings
    let name = String::from("Alice");
    let welcome_message = format!("{}, {}!", greeting, name);
    println!("{}", welcome_message);
}
```

You can also convert a string slice (`&str`) to a `String`:

```rust
let slice = "Hello, Rust!";
let string = slice.to_string();
println!("{}", string);
```
## HashMaps
HashMaps are collections of key-value pairs. They are defined using the `HashMap<K, V>` type, where `K` is the type of keys and `V` is the type of values. Here is an example of how to create and use a HashMap in Rust:
```rust
use std::collections::HashMap;

fn main() {
    // Create a new HashMap
    let mut scores = HashMap::new();
    // Insert key-value pairs
    scores.insert("Alice", 50);
    scores.insert("Bob", 30);
    scores.insert("Charlie", 40);
    // Access values by key
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }
    // Iterate over key-value pairs
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
```
