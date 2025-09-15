// Declarative Macros in Rust

// 1. Use Case
// Declarative macro: Used to declare the Solana program entry point function
entrypoint!(process_instruction);

// Logic for handling instructions
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
        Ok(())
}


// 2. Documentation
// Macro definition
macro_rules! add {
		// Match 2 parameters, e.g., add!(1,2), add!(2,3)
    ($a:expr,$b:expr) => {
        // Expanded code of the macro
        {
            // Expression addition
            $a + $b
        }
    };

		// Match 1 parameter, e.g., add!(1), add!(2)
    ($a:expr) => {{
        $a
    }};
}

fn main() {
		let x = 0;
    // Using the macro
    add!(1, 2);
    add!(x);
}

// Expanded code of the macro
fn main() {
	{
		1 + 2
	}
}


// 3. Example
macro_rules! vec {
		 // $x:expr, that is, a variable of expr type $x
		 // ($(type of mark),*), that is, the type of mark is repeated multiple times
     ( $( $x:expr ),* ) => {
         {
             let mut temp_vec = Vec::new();
						 //Execute multiple push commands here
             $(
                 temp_vec.push($x);
             )*
             temp_vec
         }
     };
}

fn main() {
let v = vec!([1,2,3]);
}

//The expanded code is as follows
let v = {
     let mut temp_vec = Vec::new();
     temp_vec.push(1);
     temp_vec.push(2);
     temp_vec.push(3);
     temp_vec
}


// 4. Quest
macro_rules! subtract {
    ($a:expr, $b: expr) => {
        {
            $a - $b
        }
    };
}
