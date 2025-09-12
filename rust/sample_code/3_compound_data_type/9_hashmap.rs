// HashMap in Rust

// 1. Documentation
// Since HashMap is not included in Rust's prelude library, it needs to be manually imported.
use std::collections::HashMap;

fn main() {
    // Create a HashMap to store student grades
    let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 100);

    // Create a HashMap with a specified size to avoid frequent memory allocation and copying, improving performance.
    let mut student_grades2 = HashMap::with_capacity(3);
    student_grades2.insert("Alice", 100);
    student_grades2.insert("Bob", 99);
    student_grades2.insert("Eve", 59);
}


// 2. Example
use std::collections::HashMap;
fn main() {
     // Vector, type is tuple (user, balance)
     let user_list: Vec<(&str, i32)> = vec![
         ("Alice", 10000),
         ("Bob", 1000),
         ("Eve", 100),
         ("Mallory", 10),
     ];

     // Use iterator and collect method to convert array to HashMap
     let mut user_map: HashMap<&str, i32> = user_list.into_iter().collect();
     println!("{:?}", user_map);

     // Get the corresponding value through hashmap[key]
     let alice_balance = user_map["Alice"];
     println!("{:?}", alice_balance);

     // Get the corresponding value through hashmap.get(key), and the return value is the Option enumeration type
     let alice_balance: Option<&i32> = user_map.get("Alice");
     println!("{:?}", alice_balance);

     // If the key does not exist, the return value is None, but no error will be reported.
     let trent_balance: Option<&i32> = user_map.get("Trent");
     println!("{:?}", trent_balance);

     // Overwrite the existing value, insert operation returns the old value
     let old = user_map.insert("Alice", 20000);
     assert_eq!(old, Some(10000));

     // or_insert returns a reference to the old value if it exists; if it does not exist, inserts the default value and returns its reference
     let v = user_map.entry("Trent").or_insert(1);
     assert_eq!(*v, 1); // Does not exist, insert 1

     // Verify the value corresponding to Trent
     let v = user_map.entry("Trent").or_insert(2);
     assert_eq!(*v, 1); // Already exists, so 2 is not inserted
}


// 3. Quest
use std::collections::HashMap;
fn main() {
    let student_arr: [(&str, i32); 3] = [
        ("Alice", 100),
        ("Bob", 10),
        ("Eve", 50)
    ];
    let mut student_map1 = HashMap::new();
    for stu in &student_arr {
        student_map1 .insert(stu.0, stu.1);
    }
    
    // Create a HashMap using the `collect` method
    let mut student_map2 = student_arr.into_iter().collect();
    
    assert_eq!(student_map1, student_map2);
    
    let alice_grade = student_map1.entry("Alice").or_insert(10);
    // Fill in the correct value
    assert_eq!(*alice_grade, 100);
    
    println!("Success!")
}
