// References and Borrowing in Rust

// 1. Use Case
entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {}


// 2. Documentation
// Variable s1 owns the string, similar to owning a cool car
let s1 = String::from("hello");

// Borrowing: Access the string using &s1, similar to a friend borrowing the car
// But the car still belongs to you
let s: &String = &s1;

// Dereferencing: Retrieve the value of the borrowed object using *s
// Similar to your friend showing off the borrowed car on the street
println!("s1 = {}, s = {}", s1, *s);


// 3. Example
// Immutable reference, get the length of the value
fn calculate_length(s: &String) -> usize {
     s.len()
}

// mutable reference
fn change(some_string: &mut String) {
     some_string.push_str(", hackquest.");
}

// Dangling reference (compilation fails)
fn dangle() -> &String {
     //Create variable s with string ownership
     let s = String::from("hello");

     // Return the borrowed object
     &s
    
} //After leaving the scope of the function body, the memory space of variable s will be automatically released. At this time &s becomes an invalid pointer (dangling reference), therefore,
   //Compilation will fail

fn main() {
     let s1 = String::from("hello");

     // &s1 is an immutable reference (default), that is, we can only read the object in the function, but cannot modify the object.
     let len = calculate_length(&s1);
     println!("The length of '{}' is {}.", s1, len);

     let mut s2 = String::from("hi");
     // &mut s2 is a mutable reference, so the change function can modify the value
     let r1 = &mut s2;
     change(r1);

     // Attempt to access object of dangling reference, compilation failed
     // let reference_to_nothing = dangle();
}


// 4. Quest
use std::mem::size_of_val;
fn main() {
    let x = String::from("hello, hackquest.");
    // Please borrow variable x here
    let y = &x;
    
    // {:p} is a way of formatting strings in Rust, which means printing the hexadecimal representation of a pointer.
    println!("The memory address of x is {:p}", y);

    let mut s = String::from("hello, ");
    // Please use borrowing instead of transferring ownership
    let p = &mut s;
     
    p.push_str(", hackquest.");
}
