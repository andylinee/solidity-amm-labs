// Tuple in Rust

// 1. Use Case
// solana_program::pubkey::Pubkey

pub fn find_program_address(
    seeds: &[&[u8]],
    program_id: &Pubkey
		// Returns a tuple, with the first element being the PDA public key address,
		// and the second element being the corresponding bump seed
) -> (Pubkey, u8)


// 2. Documentation
// Create a tuple with length 4, containing elements of different types
let tup: (i32, f64, u8, &str) = (100, 1.1, 1, "This is a tuple");

// Tuple members can also be another tuple
let tup2: (u8, (i16, u32)) = (0, (-1, 1));


// 3. Example
fn main() {
     // Create a tuple of length 4 and multiple different element types
     let tup: (i32, f64, u8, &str) = (100, 1.1, 1, "这是一个元组");
    
     // Deconstruct the tup variable and bind the second element to the variable x
     let (_, x, ..) = tup;
     println!("The value of x is: {}", x); //1.1

     // Use . to access the element at the specified index
     let first = tup.0;
     let second = tup.1;
     let third = tup.2;
     let fourth = tup.3;
     println!("The value of first is: {}, second is: {}, third is: {}, fourth is: {}", first, second, third, fourth);

     let s = String::from("hello, hackquest.");
     //The function return value is a tuple type
     let (s1, len) = calculate_length(s);
     println!("The length of '{}' is {}.", s1, len);
}

// len() returns the length of the string
fn calculate_length(s: String) -> (String, usize) {
     let length = s.len();

     (s, length)
}


// 4. Quest
use std::mem::size_of_val;
fn main() {
    // Fill in the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    
    let t = ("i", "am", "learning", "rust");
    // Fill in the blanks to make the code work
    assert_eq!(t.3, "rust");
    
    println!("Success!")
}
