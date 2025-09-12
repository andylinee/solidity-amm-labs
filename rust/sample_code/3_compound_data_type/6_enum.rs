// Enum in Rust

// 1. Use Case
pub enum ProgramError {
    // Custom error code
    Custom(u32),
    // Invalid argument error
    InvalidArgument,
    // Invalid instruction data error
    InvalidInstructionData,
    ……
}


// 2. Documentation
// Defined using the enum keyword
enum TrafficLight {
  // Enumerating all possible values here
  Red,
  Yellow,
  Green,
}


// 3. Example
// Simple enumeration
enum TrafficLight {
     Red,
     Yellow,
     Green,
}

// An enumeration containing values, different members can hold different data types
enum TrafficLightWithTime {
   Red(u8),
   Yellow(char),
   Green(String),
}

fn main() {
     // Access members of TrafficLight through the :: operator
     let red = TrafficLight::Red;
     let yellow = TrafficLight::Yellow;

     // Traffic light containing time
     let red_with_time = TrafficLightWithTime::Red(10);
     let yellow_with_time = TrafficLightWithTime::Yellow('3');
     let green_with_time = TrafficLightWithTime::Green(String::from("Green light lasts for 30 seconds"));
}


// 4. Quest
enum Direction {
    North,
    South,
    East,
    West
}

fn main() {
    let direction = Direction::East;
    match direction {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East => println!("Moving East"),
        Direction::West => println!("Moving West"),
    }
}
