// Stack and Heap in Rust

// 1. Use Case
use std::io;

fn main() {
    // Create a mutable string variable to store user input
    let mut input: String = String::new();
    println!("Please enter your name:");
    // Read user input and store it in the input variable
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    // Print the user-input string
    println!("Your name is: {}", input);
}


// 2. Example
fn main() {
     let s1 = String::from("hello");
    
     // Clone, copy the data of variable s1 in the heap memory, and store it in the newly opened memory space
     //Variable s3 records the position, length, and size information of the new space in the stack memory.
     let s3 = s1.clone();
     println!("s1 = {}, s3 = {}", s1, s3);
}
