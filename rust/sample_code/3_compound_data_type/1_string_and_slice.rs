// String and Slice in Rust

// 1. Documentation
// The string is allocated in memory
let s = String::from("hello world");

// "hello" does not reference the entire String s but only a part of it, specified by [0..5].
let hello: &str = &s[0..5];
let world: &str = &s[6..11];


// 2. Example
fn main() {
     let s: String = String::from("hello, hackquest.");

     // Starting from 0, .. represents one or more indexes
     let slice1: &str = &s[0..2];
     //The default also starts from 0
     let slice2: &str = &s[..2];

     let len: usize = s.len();
     // Contains the last byte. Since the index of the last byte is (len-1), the [4..len] method just contains the (len-1)th byte.
     let slice3: &str = &s[4..len];
     //Default to the last byte
     let slice4: &str = &s[4..];

     // Get a slice of the entire string
     let slice5: &str = &s[0..len];
     // Same as above
     let slice6: &str = &s[..];
}


// 3. Quest
use std::mem::size_of_val;
fn main() {
    let s = String::from("hello");
    
    let slice1 = &s[0..2];
    // Try another way
    let slice2 = &s[..2];
    
    assert_eq!(slice1, slice2);
    
    let s = "你好，世界";
	// Modify the following lines of code to make the code work     let slice = &s[..3];
    
    assert!(slice == "你");
}
