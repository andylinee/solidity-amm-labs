// String Literals in Rust

// 1. Use Case
use solana_program::msg;

msg!("Hello World Rust program entrypoint");


// 2. Documentation
// String literal, known at compile time
let x: &str = "hello world";

// Dynamic string
let hello: String = String::from("hello world");
// String slice, references the entire string
let y: &str = &hello[..];
// String slice, references a part of the string
let z: &str = &hello[0..3];


// 3. Example
// string literal
let a: &str = "hello, hackquest.";
println!("{} go go go!", a);
// Rust will hard-code the variable a into the program when compiling, so the 5th line of code after compilation should look like this.
// println!("hello, hackquest. go go go!");

// Get hello by index
let b = &a[..5];
println!("{}", b);

// Get dynamic string slice
let s1: String = String::from("hello,world!");
let s2: &str = &s1[..5];
println!("{}", s2);

//Convert dynamic string to string literal
let s3: &str = s1.as_str();

//Convert string literal to dynamic string type
let s4: String = "hello,world".to_string();
let s5: String = String::from("hello,world");
