# Structs and Enums
In this section, we will explore how to define and use structs and enums in Rust. Structs allow us to create custom data types that group related data together, while enums enable us to define a type that can be one of several variants.

## Defining and Instantiating Structs
A struct is a custom data type that lets you group related values together. You can define a struct using the `struct` keyword. Here is an example of defining and instantiating a struct:
```rust 
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
}
```

You can access the fields of a struct using dot notation:
```rust
println!("Name: {}, Age: {}", person.name, person.age);
```

You can also create mutable instances of structs and modify their fields:
```rustlet mut person = Person {
    name: String::from("Bob"),
    age: 25,
};
person.age += 1; // Increment age by 1
```
## Defining and Using Enums
An enum (short for "enumeration") is a type that can be one of several variants. You can define an enum using the `enum` keyword. Here is an example of defining and using an enum:
```rust
enum Direction {
    North,
    South,
    East,
    West,
}
fn main() {
    let dir = Direction::North;
}
```
You can use a `match` expression to handle different variants of an enum:
```rust
match dir {
    Direction::North => println!("Heading North"),
    Direction::South => println!("Heading South"),
    Direction::East => println!("Heading East"),
    Direction::West => println!("Heading West"),
}
```
## Pattern Matching with Enums
Pattern matching is a powerful feature in Rust that allows you to destructure enums and extract their values. Here is an example of an enum with associated data and how to use pattern matching with it:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
```
