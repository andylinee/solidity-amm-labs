// Generic Type and Trait in Rust

// 1. Use Case
// The From trait implements type conversion, and here it is generic over T, supporting conversion for various types
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}

// IntoAccountInfo is also a trait, here it is essentially implementing the From trait for the IntoAccountInfo type,
// ultimately obtaining data of the AccountInfo type
impl<T: IntoAccountInfo> From<T> for AccountInfo {
    fn from(src: T) -> Self {
        src.into_account_info()
    }
}


// 2. Documentation
// 2.1
// T can represent any type
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let arr1: [i32; 3] = [1, 2, 3];
    largest(&arr1);
    let arr2: [i64; 3] = [1, 2, 3];
    largest(&arr2);
}

// 2.2: monomorphized code
// Re-instantiated function for i32 type
fn largest_for_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Re-instantiated function for i64 type
fn largest_for_i64(list: &[i64]) -> i64 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


// 3. Example
// 1. Implement the feature constraints of PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> T {
     let mut largest = list[0];
     for &item in list.iter() {
//Only if the PartialOrd feature is implemented, the size can be compared
         if item > largest {
             largest = item;
         }
     }

     largest
}

// For the above code, compilation still fails because the list[0] operation attempts to move the element to the largest variable,
// Only types that implement the Copy attribute can do this, and for values of non-Copy types, ownership will result
// transfer

// 2. Implement the characteristic constraints of copy
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
     let mut largest = list[0];
     for &item in list.iter() {
         if item > largest {
             largest = item;
         }
     }

     largest
}

// 3. Implement feature constraints through where
fn largest<T>(list: &[T]) -> T
where T: PartialOrd + Copy,
{
     let mut largest = list[0];
     for &item in list.iter() {
         if item > largest {
             largest = item;
         }
     }

     largest
}
