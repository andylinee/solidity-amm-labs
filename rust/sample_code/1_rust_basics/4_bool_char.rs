// Boolean and Char in Rust

// 1. Use Case
// solana_program::account_info::AccountInfo

pub struct AccountInfo {
    // ……

    // Whether it is writable, if True, it is a data account responsible for storing data
    pub is_writable: bool,
    // Whether it is executable, if True, it is a program account responsible for executing logic
    pub executable: bool,
}


// 2. Documentation
// English character
let c = 'z';
// Mathematical symbol
let z = 'ℤ';
// Chinese character
let g = '国';
// Emoji
let ferris = '🦀';

// Boolean type
let m = true;


// 3. Example
use std::thread;
use std::time::Duration;

// This function takes 3 seconds
fn get_calculate_result() -> bool {
		// Simulating complex calculations takes 3 seconds
		thread::sleep(Duration::from_secs(3));
    println!("called this function");
    false
}

fn main() {
    // Print individual characters in various languages
    let thai_char  = 'ก';
    let korean_char = '한';
    let traditional_chinese_char = '繁';
    let indonesian_char = 'ä';
    // Note that str here is a string type, not a character, but the length is 1
    let str = "国";
    println!("thai_char : {}", thai_char );
    println!("Korean: {}", korean_char);
    println!("Traditional Chinese: {}", traditional_chinese_char);
    println!("Indonesian: {}", indonesian_char);
    
    //Test how many characters there are between '你' and '我'
    for i in '你'..='我' {
        print!("{}", i);//你佡佢佣……戏成我，total 4786 char
    }
    
    let f: bool = true;
    // The short-circuit principle is triggered and the get_calculate_result function will not be called for complex calculations.
    // If changed to get_calculate_result() | f, the function will be called first, which will have a performance impact
    if f || get_calculate_result() {
        println!("Success!");
    }    
} 


// 4. Quest
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 
    
    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 
}

