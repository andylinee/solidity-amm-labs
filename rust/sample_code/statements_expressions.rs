// Statements and Expressions in Rust

// 1. Use Case
// Entry point for on-chain program execution
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    // This is a statement
    msg!("Hello, world!");

    // ...specific logic omitted

    // This is an expression, returning a Result::Ok type with a value of (), indicating no return value is needed
    Ok(())
}


// 2. Documentation
fn main() {
    // Statement: use the 'let' keyword to create a variable and bind it to a value
    let a = 1;

    // Statement: since statements do not return a value, attempting to bind a statement (let a = 1) to variable b will result in a compilation error
    let b = (let a = 1);

    // Expression: the return value is x + 1
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); // y = 4
}


// 3. Example
fn main() {
     //The following 4 are statements
     let a = 1;
     let b: Vec<f64> = Vec::new(); // vec means creating a dynamic array of type f64
     let (a, c) = ("hi", false); // Tuple type
     let x: i32 = 5;

     // This is a code block expression
     let y = {
         let x_squared = x * x;
         let x_cube = x_squared * x;

         //The value of the following expression will be assigned to `y`
         x_cube + x_squared + x
     };
     println!("y is {:?}", y); // y = 155

     let z = {
         // This is an expression that calculates the value of x+1 and returns
         x+1

         // If you add a semicolon (;), it becomes a statement with no return value.
         // The default in Rust is "unit type ()", at this time z = ()
         // x + 1;
     };
     println!("z = {:?}", z);
    
     // The if statement block is also an expression, so it can be used for assignment or return directly
     // Similar to the ternary operator, in Rust we can write it like this
     let p = if x % 2 == 1 {
         "odd"
     } else {
         "even"
     };
}


// 4. Quest
// caculate x add y, then return
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let v = {
        let mut x = 1;
        x + 2
    };
    assert_eq!(v, 3);

   let s = sum(1 , 2);
   assert_eq!(s, 3);
}
