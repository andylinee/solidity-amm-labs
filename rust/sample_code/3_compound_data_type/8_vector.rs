// Vestor in Rust

// 1. Use Case
// solana_program::instruction::Instruction

pub struct Instruction {
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMeta>,
    // Data passed to the program, a dynamic array of type u8
    pub data: Vec<u8>,
}


// 2. Documentation
// 1. Explicitly declare the dynamic array type
let v1: Vec<i32> = Vec::new();

// 2. Compiler automatically infers type based on elements; v must be declared as mut to modify.
let mut v2 = Vec::new();
v2.push(1);

// 3. Use the vec! macro to create an array, supporting initialization values at creation
let v3 = vec![1, 2, 3];

// 4. Use [initial value; length] to create an array with a default value of 0 and an initial length of 3
let v4 = vec![0; 3];  // v4 = [0, 0, 0];

// 5. Use the from syntax to create an array
let v5 = Vec::from([0, 0, 0]);
assert_eq!(v4, v5);


// 3. Example
fn main() {
     let mut v1 = vec![1, 2, 3, 4, 5];

     // Directly access the element at the specified position through [index]
     let third: &i32 = &v1[2];
     println!("The third element is {}", third);

     //Access through the .get() method to prevent the subscript from going out of bounds
     // match belongs to pattern matching, which will be introduced in detail in subsequent chapters.
     match v1.get(2) {
         Some(third) => println!("The third element is {third}"),
         None => println!("The specified element does not exist"),
     }

     //Iterate through and modify elements
     for i in &mut v1 {
         // Here i is a mutable reference to the element in array v. The value is obtained through *i dereference and + 10
         *i += 10
     }
     println!("v1 = {:?}", v1); // v1 = [11, 12, 13, 14, 15]

     let mut v2: Vec<i32> = vec![1, 2];
     assert!(!v2.is_empty()); // Check if v2 is empty
     v2.insert(2, 3); //Insert data at the specified index. The index value cannot be greater than the length of v, v2: [1, 2, 3]
     assert_eq!(v2.remove(1), 2); // Remove the element at the specified position and return, v2: [1, 3]
     assert_eq!(v2.pop(), Some(3)); // Delete and return the element at the end of v, v2: [1]
     v2.clear(); // Clear v2, v2: []
}


// 4. Quest
use std::mem::size_of_val;
fn main() {
    let v1 = vec![1, 2, 3];
    // Implement the code here
    let v2 = Vec::from([1, 2, 3]);
    
    assert_eq!(v1, v2); 
    
    let mut v3 = Vec::from([1, 2, 3, 4, 5]);
    for i in &mut v3 {
        // Implement the code here...
        *i += 1
    }
    
    assert_eq!(v3, vec![2, 3, 4, 5, 6]);
}
