// Generic Type in Rust

// 1. Documentation
// 1.1
// Finding the largest element in an array of type i32
fn largest_for_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Finding the largest element in an array of type i64
// Other than the difference in parameter type and return value type,
// everything else is exactly the same as the previous function
fn largest_for_i64(list: &[i64]) -> i64 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 1.2: Abstracting the specific type as a generic T
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // This is just for illustrative purposes, actual code would need to add generic constraints
        if item > largest {
            largest = item;
        }
    }

    largest
}


// 2. Example
// 1. Generics are used in the structure, and the types of all members are T
struct Point1<T> {
     x: T,
     y: T,
}

// 2. Use generics in the structure, and members can have different types
struct Point2<T,U> {
     x: T,
     y: U,
}

// 3. Use generics in enumerations, and the Option enumeration returns a value of any type Some(T), or no value None
enum Option<T> {
     Some(T),
     None,
}

// 4. Using generics in the method, we implemented the method get_x for the structure Point1<T>, which is used to return the value of the x member
impl<T> Point1<T> {
     fn get_x(&self) -> &T {
         &self.x
     }
}

fn main() {
     // 1. Use generics in structures
     let int_point = Point1 { x: 5, y: 10 };
     let float_point = Point1 { x: 1.0, y: 4.0 };

     // 2. Use generics in structures
     let p = Point2{x: 1, y:1.1};

     // 3. Use generics in enumerations
     let option1 = Option::Some(1_i32);
     let option2 = Option::Some(1.00_f64);

     // 4. Use generics in methods
     let x = int_point.get_x();
}


// 3. Quest
struct Point<T> {
    // Modify the following structure
    x: T,
    y: String,
}

fn main() {
    let p = Point{x: 5, y: "hello".to_string()};
}
