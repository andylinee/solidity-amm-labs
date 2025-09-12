// Pattern Matching in Rust

// 1. Use Case
// Instruction type for user operations, represented as an enum
pub enum MovieInstruction {
    // Add a movie review
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
    // Update a movie review
    UpdateMovieReview {
        title: String,
        rating: u8,
        description: String
    }
}

pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    // Actually assign values based on the input
    let variant = 1;
    let payload;

    // Pattern matching
    Ok(match variant {
        0 => MovieInstruction::AddMovieReview {
            title: payload.title,
            rating: payload.rating,
            description: payload.description,
        },
        1 => MovieInstruction::UpdateMovieReview {
            title: payload.title,
            rating: payload.rating,
            description: payload.description
        },
        // Cover other cases here
        _ => return Err(ProgramError::InvalidInstructionData),
    })
}


// 2. Documentation
enum BlockChain {
    BitCoin,
    Ethereum,
    Starknet,
    Solana,
}

fn main() {
    let block_chain = BlockChain::Solana;
    match block_chain {
        BlockChain::BitCoin => println!("BitCoin"),
        // X | Y, similar to a logical OR, means this branch can match either X or Y
        BlockChain::Ethereum | BlockChain::Starknet => {
            println!("Ethereum or Starknet");
        },
        // Use _ to represent all unlisted possibilities
        _ => println!("Solana"),
    };
}


// 3. Example
enum Shape {
     Circle(f64),
     Rectangle(f64, f64),
     Square(f64),
}

fn calculate_area(shape: &Shape) -> f64 {
     match shape {
         // Get the bound value from the matching pattern, such as radiux, width, side
         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
         Shape::Rectangle(width, height) => width * height,
         Shape::Square(side) => side * side,
     }
}
struct Point {
     x: i32,
     y:i32,
}

fn process_point(point: Point) {
     match point {
         Point { x: 0, y: 0 } => println!("Coordinates are at the origin"),
         Point { x, y } => println!("Coordinates are at ({}, {})", x, y),
     }
}

fn main() {
     let circle = Shape::Circle(3.0);
     let rectangle = Shape::Rectangle(4.0, 5.0);
     let square = Shape::Square(2.0);

     // 1. Call the function to output the area of each shape
     println!("The area of a circle: {}", calculate_area(&circle));
     println!("The area of the rectangle: {}", calculate_area(&rectangle));
     println!("Area of square: {}", calculate_area(&square));

     // 2. Match pattern matching for assignment
     let area = match circle {
         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
         Shape::Rectangle(width, height) => width * height,
         Shape::Square(side) => side * side,
     };
     println!("The area of the circle: {}", area);

     // 3. Deconstruct the structure
     let point1 = Point { x: 0, y: 0 };
     let point2 = Point { x: 3, y: 7 };
     process_point(point1);
     process_point(point2);

		 // 4. if let simple matching
     let some_u8_value = Some(3u8);
     match some_u8_value {
         Some(3) => println!("three"),
         // Other values besides 3 and None values should also be considered here
         _ => (),
     }
    
     //Only match the value 3
     if let Some(3) = some_u8_value {
         println!("three");
     }
}


// 4. Quest
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255, 255, 0)
    ];
    
    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    match msg {
        // 1、match the enum of Message::Move
        Message::Move{x:a, y:b} => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            // 2、please input the correct content
            assert_eq!(g, 255);
            // 3、please input the correct content
            assert_eq!(b, 0);
        }
        __ => println!("no data in these variants")
    }
}
