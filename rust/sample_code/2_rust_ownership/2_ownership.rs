// Ownership in Rust

// 1. Use Case
// Here we are still passing a reference of instruction_data to the internal function
pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {}


// 2. Documentation
// Variable s1 owns the string "hello"
let mut s1: String = String::from("hello");

// The variable s1 can modify this string
s1.push_str(", hackquest."); // push_str() appends a literal to the string


// 3. Example 
//Move ownership
fn main() {
     let s1 = String::from("hello");
    
     //Here s1 moved ownership to variable s2. According to the second of the three principles of ownership,
     // At this time, only s2 has ownership, which means that s1 has expired.
     let s2 = s1;
    
     // Printing s1 here will fail because it is invalid
     // println!("{}, world!", s1);

} //The third principle of ownership is triggered here, that is, drop the s2 variable and release the memory
