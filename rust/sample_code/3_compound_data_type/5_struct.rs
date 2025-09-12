// Struct in Rust

// 1. Use Case
pub struct AccountInfo {
    /// Account public key for unique identification
    pub key: &Pubkey,
    /// Lamports count for the account, a reference-counted smart pointer allowing mutable references to be shared across multiple places
    pub lamports: Rc<RefCell<&mut u64>>,
    /// Data held by the account
    pub data: Rc<RefCell<&mut [u8]>>,
    /// Owner of this account, also an account
    pub owner: &Pubkey,
    /// Epoch when rent for this account needs to be paid next
    pub rent_epoch: Epoch,
    /// Whether the transaction is signed by the account's public key
    pub is_signer: bool,
    /// Indicates whether the account is writable; if true, allows modification of account data
    pub is_writable: bool,
    /// Whether the account is an executable program
    pub executable: bool,
}


// 2. Documentation
struct Car {
    // Brand
    brand: String,
    // Color
    color: String,
    // Production Year
    year: String,
    // Is it a new energy vehicle?
    is_new_energy: bool,
    // Price
    price: f64
}


// 3. Example
//Standard creation method
fn build_car(color: String, year: String, price: f64) -> Car {
     Car {
         brand: String::from("Tesla"),
         color: color,
         year: year,
         is_new_energy: true,
         price: price,
     }
}

//Simplified creation method
fn build_car2(color: String, year: String, price: f64) -> Car {
     Car {
         brand: String::from("Tesla"),
         // When the function parameters and structure fields have the same name, they can be initialized directly using abbreviation.
         color,
         year,
         is_new_energy: true,
         price,
     }
}

fn main() {
     // Declare Car type variable (requires variable must be mut type)
     let mut car1 = build_car2(String::from("black"), String::from("2023-01-01"), 123.00);

     //Access and modify the structure (accessed through the . operator)
     car1.color = String::from("white");
     println!("car1 {:?}", car1);

     // Create a new structure instance based on the existing structure instance
     let mut car2 = Car {
         color: String::from("greey"),
         //Other fields are taken from car1, ..car1 must be used at the end of the structure
         ..car1
     };

     println!("car2 {:?}", car2);
}


// 4. QUest
struct Person {
    name: String,
    age: u8
}

fn main() {
    let age = 18;
    // Modify here
    let mut p = Person {
        name: String::from("web3"),
        age
    };
    
    p.age = 30;
    
    // Modify here
    p.name = String::from("rust");
}
