// Rust Project Structure

// 1. Documentation
// Define a module using mod, followed by the module name (my_module)
mod my_module {
    // Content of the module
    pub fn my_function() {
        // Function body
    }
}


// 2. Example
// src/main.rs

// Main module, equivalent to the hall of the house
mod living_room {
     // Submodule, equivalent to the bedroom of the house
     mod bedroom {
         // The functions in the module are equivalent to the furniture in the bedroom
         pub fn sleep() {
             println!("Sleeping in the bedroom");
         }
     }

     // Submodule, equivalent to the kitchen of the house
     mod kitchen {
         // The functions in the module are equivalent to the equipment in the kitchen
         pub fn cook() {
             println!("Cooking in the kitchen");
         }
     }

     // Functions in the main module are equivalent to activities in the lobby
     pub fn relax() {
         println!("Relaxing in the living room");
         bedroom::sleep(); // Call the function in the bedroom module
         kitchen::cook(); // Call the function in the kitchen module
     }
}

// Main function, equivalent to the entrance of the entire house
fn main() {
     // Call the function in the living_room module
     living_room::relax();
}
