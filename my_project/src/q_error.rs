use std::fs::File;

pub fn main() {
    // Unrecoverable Errors

    // Example 1: Rust Unrecoverable Errors with panic! Macro
    println!("Hello, World!");
    // panic!("Crash");

    // Example 2: Rust Unrecoverable Errors
    let numbers = [1, 2, 3];
    // println!("unknown index value = {}", numbers[3]);

    // Recoverable Errors
    // let data_result = File::open("data.txt");
    // let data_file = match data_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the data file: {:?}", error),
    // };
    // println!("Data file ={:?}", data_file);

    // Some and None
    let text = "Hello, World!";
    let character_option = text.chars().nth(5);
    let character = match character_option {
        None => "empty".to_string(),
        Some(c) => c.to_string(),
    };
    println!("Character at index 15 is {}", character);

    // Option is about Some or None (value or no value)
    // Result is about Ok or Err (result or error result)
}
