# Traits and Generics

- Defining and implementing traits
- Using generics in functions and structs
- Trait bounds and constraints
- Associated types
- Default implementations
- Generic data structures
- Dynamic dispatch with trait objects
  

## Defining and implementing traits

Traits in Rust are similar to interfaces in other programming languages. They define a set of methods that a type must implement. You can define your own traits and implement them for different types. Here is an example of defining and implementing a trait:

```rust
// Define a trait
trait Summary {
    fn summarize(&self) -> String;
}
// Implement the trait for a struct
struct NewsArticle {
    headline: String,
    location: String,
    content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.location, self.content)
    }
}
// Implement the trait for another struct
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        location: String::from("Internet"),
        content: String::from("Rust is a systems programming language that runs blazingly fast
        and prevents segfaults."),
    };
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("Learning Rust is fun!"),
        reply: false,
        retweet: false,
    };
    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());
}
```

