// Trait Associated Type in Rust

// 1. Use Case
trait StorageType {
    // Define an associated type
    type Wraps<'a>: 'a
        where Self: 'a;

    // Define a method
    fn load<'s>(self) -> Self::Wraps<'s>
        where Self: 's;
}

// Define the state variables of the smart contract
struct Contract {
    owner: StorageAddress,
    active: StorageBool,
}

// Implement the StorageType trait for the smart contract
impl stylus_sdk::storage::StorageType for Contract {
    // Define the concrete associated type StorageGuard
    type Wraps<'a> = stylus_sdk::storage::StorageGuard<'a, Self> where Self: 'a;

    // Obtain an immutable reference
    fn load<'s>(self) -> Self::Wraps<'s> {
        stylus_sdk::storage::StorageGuard::new(self)
    }
}


// 2. Documentation
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}


// 3. Example
// Define a trait that contains an associated type
trait Summary {
     //Association type
     type Output;

     //Define a method to return the associated type
     fn summarize(&self) -> Self::Output;
}

// Concrete type that implements the Summary trait: NewsArticle
struct NewsArticle {
     headline: String,
     location: String,
     author: String,
}

// Implement Summary trait for NewsArticle
impl Summary for NewsArticle {
     //Specify the specific type of the associated type
     type Output = String;

     // Implement the methods in the trait
     fn summarize(&self) -> String {
         format!("{}, by {} ({})", self.headline, self.author, self.location)
     }
}

fn main() {
     let article = NewsArticle {
         headline: String::from("Penguins win the Stanley Cup Championship!"),
         location: String::from("Pittsburgh, PA, USA"),
         author: String::from("Iceburgh"),
     };

     println!("{}", article.summarize());
}


// 4. Quest
trait MyTrait {
    type AssociatedType;
    // define your method
}
struct MyStruct;

impl MyTrait for MyStruct {
    type AssociatedType = i32;
    // imply the method of a trait
}
