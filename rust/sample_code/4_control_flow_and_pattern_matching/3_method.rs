// Method in Rust

// 1. Use Case
pub fn find_program_address(
    seeds: &[&[u8]],
    program_id: &Pubkey
) -> (Pubkey, u8)


// 2. Documentation
// Struct definition
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation for Rectangle
impl Rectangle {
    // The area method with &self as the first parameter, representing the instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        // Calling the area method on the struct instance
        rect1.area()
    );
}


// 3. Example
struct Rectangle {
     width: u32,
     height: u32,
}

impl Rectangle {
     /* Method with the same name: makes the code more consistent and concise. When you need to get or set the properties of a structure,
      * You can directly use the attribute name as the method name without additional memory or documentation, and it is more consistent.
      * Integrate intuitive reading and understanding methods to reduce the difficulty of code maintenance.
      */
     pub fn width(&self) -> u32 {
         return self.width;
     }

     //Associated function, no &self parameter
     pub fn new(width: u32, height: u32) -> Self {
         Rectangle { width, height }
     }
}

fn main() {
     // If there is no self parameter in the method, the method is an associated function.
     // Usually used to initialize instances, call the associated function new to create an instance corresponding to the structure
     let rect1 = Rectangle::new(30, 50);

     // Method 1, access the width field of Rectangle
     println!("{}", rect1.width);

     // Method 2: Call the width method of Rectangle, similar to getter()
     println!("{}", rect1.width());
}


// 4. Qeust
#[derive(Debug)]
struct Mario {
    is_small: bool,
    coins: i32,
}

impl Mario {
    // 1. Implement the following associated function `new`,
    // 2. This function returns a Mario instance, including is_small : false, coins : 100
    // Description: The return value `Self` is of type `Mario`
    fn new() -> Self {
        Mario {
            is_small : false ,
            coins: 100,
        }
    }
    
    pub fn get_coins(&self) -> i32 {
        self.coins
    }
}

fn main() {
    let mario = Mario::new();
    assert_eq!(mario.get_coins(), 100);
}
