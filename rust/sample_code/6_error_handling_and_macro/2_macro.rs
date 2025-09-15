// Macro in Rust

// 1. Use Case
// Attribute-like macro: Used to set the Solana program ID
declare_id!("3Vg9yrVTKRjKL9QaBWsZq4w7UsePHAttuZDbrZK3G5pf");

// Derive macro: Used for (de)serialization
#[derive(BorshSerialize, BorshDeserialize)]
struct NoteState {
    title: String,
    body: String,
    id: u64
}


// 2. Documentation
// Logging macro: println!
println!("hello, micro");

// Dynamic array creation macro: vec!
let _dyn_arr = vec![1, 2, 3];

// Assertion macro: assert!, checks if a condition is satisfied
let x = 1;
let y = 2;
assert!(x + y == 3, "x + y should equal 3");

// String formatting macro: format!
let name = "world";
let _message = format!("Hello, {}!", name);


// 3. Example
// Macro rules
macro_rules! double {
     ($x:expr) => {
		 // Replace the code and double the expression
         $x*2
     };
}

fn main() {
     let result = double!(5);
     println!("Result: {}", result); // Output: Result: 10
}
