// Variables and Mutability

// 1. Use Case: GreetingAccount is Solana
pub struct GreetingAccount {
    pub counter: u32,
}

// This is the entry function for the program (smart contract)
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {

    // Use mut to make greeting_account mutable when deserializing
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;

    Ok(())
}


// 2. Documentation
fn main() {
    // x is a mutable variable; mut stands for mutable, allowing changes
    let mut x = 1;
    println!("x = {}", x);
    x = 2;
    println!("x = {}", x);

    // y is an immutable variable; Rust defaults to immutable if not specified
    let y = 3;
    println!("y = {}", y);
    // Attempting to reassign a value to an immutable variable y results in a compile-time error
    y = 4;
    println!("y = {}", y);
}


// 3. Example
fn main() {
    // Immutable，default
    let ferris_id = 1234567890;
    println!("ferris_id_card = {}", ferris_id);

    // Mutable
    let mut ferris_address: &str = "Sunshine Beach No. 01";
    println!("ferris lived in, {}!", ferris_address);

    // reassign
    ferris_address = "Sunshine Beach No. 02";
    println!("now, ferris lived in, {}!", ferris_address);
}