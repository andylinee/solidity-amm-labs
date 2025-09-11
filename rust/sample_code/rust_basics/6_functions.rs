// Functions in Rsut

// 1. Use Case
// Mark process_instruction function as the entry point for the program
entrypoint!(process_instruction);

// Function name follows snake_case naming convention
pub fn process_instruction(
    // Parameters must explicitly specify types
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
    // Function return type is the ProgramResult enum
) -> ProgramResult {
    msg!("Hello, world!");

    Ok(())
}

// Return type is an enum with variants of Result type
enum ProgramResult {
    // You can omit the Result:: prefix and directly use OK
    Ok(()),
    // Err value is still an enum type
    Err(ProgramError),
}

// Error enum
pub enum ProgramError {
    Custom(u32),
    InvalidArgument,
    InvalidInstructionData,
    // ...
}


// 2. Documentation
// 'fn' is the keyword for declaring a function
// 'unsafe_add()' is the function name, following the snake_case convention
// 'i' and 'j' are input parameters, and their types need to be explicitly specified
// --> 'i32' indicates the return type is also of type 'i32'
fn unsafe_add(i: i32, j: i32) -> i32 {
    // This is an expression, so the function will return this value after computing the sum
    i + j
}


// 3. Example
// Expression as a function that returns a value
fn max_plus_one(x: i32, y: i32) -> i32 {
     if x > y {
         // If this rule is hit, you can return directly through return
         return x + 1;
     }

     //The last line is an expression, which can be used as a function return value
     // Note that there cannot be a semicolon here, otherwise it will be a statement
     y+1
}

// Unit type () as a function that returns a value
// This function does not have an explicit return value type, and Rust returns the unit type () by default
fn print_hello() {
     // This is a statement, not an expression
     println!("hello");
}

// A divergent function that never returns, marked with !
fn diverging() -> ! {
     // Throw a panic exception and terminate the program.
     panic!("This function will never return!");
}


// 4. Quest
// fullfill the definition of the sum
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
}
