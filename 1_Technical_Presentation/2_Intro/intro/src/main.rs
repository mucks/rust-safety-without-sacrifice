use std::u64;

fn main() {
    // 1. mutability in Rust

    // javascript: const
    let x = 1;
    //x = 2;

    // javascript: let
    // python: y = 1
    // Int y = 1;
    let mut y = 1;
    y = x;

    // 2. data types

    let i: i32 = 1;
    // java: float
    let f: f32 = 1.0;
    // java: double
    let ff: f64 = 1.0;
    // java: bigint
    let mid_int: i64 = 1;
    let huge_int: i128 = 1;

    // unsigned integer
    let u_small: u8 = 1;
    let u_mid: u16 = 1;
    let u_large: u32 = 1;
    let u_huge: u64 = 1;
    let u_very_huge: u128 = 1;

    // char
    let c: char = 'a';
    let c_unicode: char = '我';
    let boolean: bool = true;

    // tuple
    let mut tup: (i32, f64, u8) = (1, 1.0, 1);
    let (mut a, b, c) = tup;

    tup.0 = 1;
    tup.1 = 1.0;

    a = 1;

    let array: [u64; 3] = [1, 2, 3];
    let quick_array = [3; 5]; // [3, 3, 3, 3, 3]
    let array_of_arrays = [[1, 2], [3, 4], [5, 6]];

    let s = "1234";

    let char_array = ['a', 'b', 'c'];

    let first_val = array[0];
    // 3. functions

    fn summarize() {}

    summarize();

    fn summarize_text(text: char) {
        println!("char: {text}");
    }
    summarize_text('c');

    fn summarize_multiple_texts(text1: char, text2: char, text3: char) {
        format!("{text1}{text3}{text3}");
        ()
    }
    let x = summarize_multiple_texts('i', 'i', 'i');
    println!("{x:?}");
    //fn summarize_multiple_texts(text1: char, text2: char, text3: char) {}

    // 4. Control Flow

    if true && x == () {
        println!("happens");
    } else if i == 2 {
    } else {
    }

    while x == () {
        println!("x is void");
        break;
    }
    let min = 10;
    let max = 100;

    for i in min..max {}

    for i in 0..10 {
        println!("i = {i}");
    }

    let mut counter = 1;

    loop {
        counter += 1;
        println!("test {counter}");
        if counter < 5 {
            continue;
        }

        break;
    }

    let i = 2;
    let i = 1;

    match i {
        j if j > 2 => println!("greater two"),
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("two"),
        _ => println!("not found"),
    };
}
