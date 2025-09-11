// Ownership and Functions in Rust

// 1. Use Case
// borrow_mut returns the type RefCell, commonly used when borrowing rules are not determinable at compile time
// [..] gets the complete slice of data
let data: &mut [u8] = &mut account.data.borrow_mut()[..];
data[0] = instruction_data[0];


// 2. Documentation
// Ownership can be transfer to functions
struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped (released) here
}

fn main() {
    let foo = Foo { x: 42 };
    // foo is transferred to do_something
    do_something(foo);
    // After this point, foo can no longer be used
}

// Ownership can also be acquired from functions
fn do_something() -> Foo {
    Foo { x: 42 }
    // Ownership is moved out
}

fn main() {
    let foo = do_something();
    // foo becomes the owner
    // foo is dropped at the end of the function scope
}


// 3. Example
struct Foo {
     x: i32,
}

fn do_something(f: Foo) {
     println!("{}", f.x);
     // f is dropped here and released
}

fn main() {
     let mut foo = Foo { x: 42 };
     let f = &mut foo;

     // An error will be reported: do_something(foo);
     // Unable to take ownership of foo because it has been mutably borrowed

     // An error will be reported: foo.x = 13;
     // Because foo has been mutably borrowed and cannot be modified

     f.x = 13;
     // f will be dropped because it will no longer be used.
    
     println!("{}", foo.x);
    
     // Now modifications can proceed normally because all mutable references have been dropped
     foo.x = 7;
    
     //Move ownership of foo into a function
     do_something(foo);
}
