// String Operations in Rust

// 1. Example
fn main() {
     let mut s = String::from("Hello ");

     // Append a string, modify the original string, not generate a new string
     s.push_str("rust");
     println!("Append string push_str() -> {}", s);

     // Append characters
     s.push('!');
     println!("Append character push() -> {}", s);

     // Insert characters and modify the original string. You need to specify the index position. The index starts from 0.
     // If it goes out of bounds, an error will occur
     s.insert(5, ',');
     println!("Insert character insert() -> {}", s);

     // Insert string
     s.insert_str(6, "I like");
     println!("Insert string insert_str() -> {}", s);

     // replace The replacement operation generates a new string. Requires 2 parameters, the first parameter is
     // The string to be replaced, the second parameter is the new string
     let str_old = String::from("I like rust, rust, rust!");
     let str_new = str_old.replace("rust", "RUST");
     println!("The original string length is: {}, memory address: {:p}", str_old.len(), &str_old);
     println!("The new string length is: {}, memory address: {:p}", str_new.len(), &str_new);

     // pop delete operation, modify the original string, which is equivalent to popping the last character of the character array
     // The return value is the deleted character, Option type, if the string is empty, None is returned
     // Note: pop is performed according to the "character" dimension, not "byte"
     let mut string_pop = String::from("删除操作，rust 中文!");

     // What is deleted at this time is the exclamation mark "!" at the end.
     let p1 = string_pop.pop();
     println!("p1:{:?}", p1);
     // Delete the "文" at the end based on p1
     let p2 = string_pop.pop();
     println!("p2:{:?}", p2);
     // The remaining string at this time is “删除操作，rust 中”
     println!("string_pop:{:?}", string_pop);
}


// 2. Quest
use std::mem::size_of_val;
fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');
 
    assert_eq!(s, "hello, world!");
 
    let slice = &s[..5];
 
    assert_eq!(slice, "hello");
 
    println!("Success!");
}
