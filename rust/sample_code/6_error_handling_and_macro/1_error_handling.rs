// Erro Handling in Rust

// 1. Use Case
enum ProgramResult {
    Ok(()),
    Err(ProgramError),
}


// 2. Documentation
/*
 * The definition of Result is as follows,
 *
 * enum Result<T, E> {
 *    Ok(T),
 *	  Err(E),
 * }
 */

// Division of two numbers
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero!"))
    } else {
        Ok(a / b)
    }
}

// Unrecoverable error (panic)
fn main1() {
    // Manually trigger a panic, and the program will terminate without further execution
    panic!("This is a panic situation!");
}

// Recoverable error using Result type for potential errors
fn main2() {
    // divide(1, 0) returns a Result type, and we use match to handle both success and failure cases
    match divide(1, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}


// 3. Example
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
     // Try to open the file,
		 // ? means that if successful, bind the file handle to the variable my_file, otherwise directly return Err(e)
     let mut my_file = File::open(file_path)?;

     //Try to read the file contents
     let mut contents = String::new();
	
		 // ? means that if successful, bind the file content to the variable contents, and the program will continue to execute, otherwise it will directly return Err(e)
     my_file.read_to_string(&mut contents)?;

		 //Return the file content on success
     Ok(contents)
}

fn main() {
     // Call the function with ?
     match read_file_contents("example.txt") {
         Ok(contents) => println!("File contents: {}", contents),
         Err(err) => eprintln!("Error reading file: {}", err),
     }
}


// 4. Error Propagation
use std::io;
use std::io::Read;
use std::fs::File;

// Open and read the contents of a file, returning a Result type. Whether it's successful or an error,
// it can be propagated to the upper-level caller, the main function.
fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    // Attempt to open the file
    let my_file: Result<File, io::Error> = File::open(file_path);

    // Using pattern matching, if the file is successfully opened, bind the file handle to the 'file' variable,
    // otherwise, explicitly return the error information, letting the upper-level caller (main function) handle it
    let mut file = match my_file {
        Ok(my_file) => my_file,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();

    // This is an expression, directly returning the corresponding value after pattern matching.
    // If reading is successful, return Ok(contents), otherwise return Err(e), which is handled by the
    // upper-level caller (main function).
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn main() {
    // Handle the result of the function call
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        // The upper-level caller decides to print the error message instead of interrupting the program execution
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}


// 5. Quest
fn must_positive(num: f64) -> Result<f64, &'static str> {
    if num < 0.0 {
        Err("Number is negative")
    } else {
        Ok(num)
    }
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b as i32 == 0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(a / b)
    }
}

fn test(num: f64) -> Result<f64, &'static str> {
    let a =  must_positive(num)?;
    
    // Use ? for simplified error propagation
    let b = divide(a, a + num)?;
    
    Ok(a + b)
}

fn main() {
    let b = test(1.1);
    match b {
        // Please fill in the correct matching pattern
        Ok(result) => println!("Result:{}", result),
        
        Err(e) => println!("Error: {}", e),
    }
}
