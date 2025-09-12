// Array in Rust

// 1. Use Case
// solana_program::pubkey::Pubkey

// pubkey_array is a u8 array with a length of 32
pub const fn new_from_array(pubkey_array: [u8; 32]) -> Self {
    Self(pubkey_array)
}


// 2. Documentation
// Type inference by the compiler without specifying the element type
let a = [1, 2, 3, 4, 5];

// Explicitly specifying type and length with [type; length]
let b: [i32; 5] = [1, 2, 3, 4, 5];

// Initializing an array with a specific value repeated N times, c = [3,3,3,3,3]
let c = [3; 5];


// 3. Example
fn main() {
     // array = [type; length] This syntax is OK for basic types such as i32, f64, bool, etc.
     let a = [3u8; 5]; // a = [3, 3, 3, 3, 3]

     // But for non-basic types such as String, you need to use the following method, because the basic type data is in the stack memory and can be copied directly.
     // Data of non-basic types is in heap memory and requires deep copying.
     let b: [String; 3] = std::array::from_fn(|_i| String::from("rust")); // b = ["rust","rust","rust"]

     let c = [9, 8, 7, 6, 5];
     // Direct access via subscript
     let first = c[0]; // first = 9
     let second = c[1]; // second = 8

     // If you access a non-existent element, the compiler will directly recognize it and give an error message.
     // let none_element = c[100];

     // arrays is a two-dimensional array, each element is an array, and the element type is [u8; 5]
		 // arrays = [[3, 3, 3, 3, 3],[9, 8, 7, 6, 5]]
     let arrays: [[u8; 5]; 2] = [a, c];
}

// 4. Quest
use std::mem::size_of_val;
fn main() {
    // Please fill in the correct data type (use Rust's default numerical type, which is signed 32-bit)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    assert!(arr.len() == 5);
    
    // Please fill in the correct data type
    let arr: [char; 3] = ['中', 'b', 'c'];
    
    // All elements in the array can be initialized together to the same value
    let list: [i32; 100] = [1; 100];
    
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    
    println!("Success!")
}
