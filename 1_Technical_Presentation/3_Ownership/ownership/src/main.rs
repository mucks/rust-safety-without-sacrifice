fn use_string(s: &String) {
    println!("{s}");
}

fn use_string_again(s: &str) {
    println!("second time: {s}");
}

fn use_string_mutable(s: &mut String) {
    *s = "".to_string();
}

fn change_string(mut s: String) -> String {
    s = "".to_string();
    s
}

fn main() {
    // 1. ownership in Rust

    let s = "hello world";

    {
        let owned_string = "hello world owned".to_string();

        let part_of_string = &[0..2];

        use_string(&owned_string);
        use_string_again(&owned_string);
        let new_string = change_string(owned_string);
        println!("new_string: {new_string}");
    }

    {
        let mut i = 1;

        fn use_i(i: i32) -> i32 {
            i
        }
        fn use_i_mutable(i: &mut i32) {
            *i = 2;
        }

        use_i(i);
        use_i(i);
        use_i(i);
        use_i_mutable(&mut i);
        use_i(i);
    }

    // 2. borrowing and references

    // 3. slices
    let s = "hello world";

    fn return_the_string(s: &str) -> &str {
        &s[1..5]
    }

    fn dangle() -> String {
        let s = String::from("hello");
        s // s goes out of scope here, so this reference would be dangling
    }

    // 4. dangling references

    // 5. the Copy trait

    // 6. the Clone trait
}
