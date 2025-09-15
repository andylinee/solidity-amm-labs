// Attribute-like Macros in Rust

// 1. Use Case
pub struct InitializeAccounts<'info> {

    #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
    pub pda_counter: Account<'info, Counter>,
    // ……
}

#[account]
struct Counter {
    count: i32,
}


// 2. Documentation
// Used for selectively including or excluding code based on conditions
#[cfg(feature = "some_feature")]
fn conditional_function() {
    // This function is compiled only when a specific feature is enabled
}

#[test]
fn my_test() {
    // Test function
}

#[allow(unused_variables)]
fn unused_variable() {
    // Allowing unused variables
}


// 3. Example
// vec! Macro for creating Vec.
let my_vector = vec![1, 2, 3];

// println! and format! macros for formatting strings.
let name = "World";
println!("Hello, {}!", name);

let formatted_string = format!("Hello, {}!", name);

// assert! and assert_eq! macros for writing assertions.
assert!(true);
assert_eq!(2 + 2, 4);


// panic! Macro used to cause Panic exceptions in the program.
panic!("Something went wrong!");

// env! Macro for obtaining environment variables at compile time.
let current_user = env!("USER");
println!("Current user: {}", current_user);


// declare_id! is a macro used in the anchor framework to declare program IDs
declare_id!("3Vg9yrVTKRjKL9QaBWsZq4w7UsePHAttuZDbrZK3G5pf");


// 4. Quest
use proc_macro::TokenStream;

#[proc_macro]

pub fn my_custom_fn_macro(input: TokenStream) -> TokenStream {
    // do something...
}
