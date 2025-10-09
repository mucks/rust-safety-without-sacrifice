# Ownership
1. Ownership Rules
   - Each value in Rust has a variable that’s called its owner.
   - There can only be one owner at a time.
   - When the owner goes out of scope, the value will be dropped.
- 2. References and Borrowing
   - References allow you to refer to some value without taking ownership of it.
   - You can have either one mutable reference or any number of immutable references, but not both at the same time.
   - References must always be valid.
   - Example:
   ```rust
   let s1 = String::from("hello");
   let len = calculate_length(&s1); // borrow s1
   println!("The length of '{}' is {}.", s1, len); // s1 is still valid
   fn calculate_length(s: &String) -> usize {
       s.len()
   }
   ```
- 3. Slices
   - A slice is a reference to a contiguous sequence of elements in a collection.
   - Slices do not have ownership.
   - Example:
   ```rust
   let s = String::from("hello world");
   let hello = &s[0..5]; // slice of the first 5 characters
   let world = &s[6..11]; // slice of the last 5 characters
   println!("{} {}", hello, world);
   ```
   - You can also use slices with arrays:
   ```rust
   let a = [1, 2, 3, 4, 5];
   let slice = &a[1..3]; // slice of elements at index 1 and 2
   println!("{:?}", slice); // prints [2, 3]
   ```
- 4. Dangling References
   - Rust prevents dangling references by ensuring that references are always valid.
   - Example of a dangling reference that Rust will not compile:
   ```rust
   fn dangle() -> &String {
       let s = String::from("hello");
       &s // s goes out of scope here, so this reference would be dangling
   }
   ```
   - To fix this, you can return the String itself instead of a reference:
   ```rust
   fn no_dangle() -> String {
       let s = String::from("hello");
       s // ownership of s is moved to the caller
   }
   ```
- 5. The `Copy` Trait
-  - Types that implement the `Copy` trait do not move ownership when assigned or passed to a function.
   - Examples of types that implement `Copy`: integers, booleans, floating-point numbers, and characters.
   - Example:
   ```rust
   let x = 5;
   let y = x; // x is copied, not moved
   println!("x: {}, y: {}", x, y); // both x and y are valid
   ```
- 6. The `Clone` Trait
-  - The `Clone` trait allows for deep copying of data.
   - You can explicitly call the `clone` method to create a copy of data that does not implement the `Copy` trait.
   - Example:
   ```rust
   let s1 = String::from("hello");
   let s2 = s1.clone(); // deep copy of s1
   println!("s1: {}, s2: {}", s1, s2); // both s1 and s2 are valid
   ```
- 7. Summary
    - Ownership is a key feature of Rust that ensures memory safety without a garbage collector.
    - Understanding ownership, borrowing, and slices is essential for writing efficient and safe Rust code.
    - Practice these concepts to become proficient in Rust programming!