// Dangling References in Rust

// 1. Documentation
// Let's look at an example of a dangling reference.
{
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}


// 2. Example
// Get the longest of the strings referenced by variables x and y
fn longest(x: &str, y: &str) -> &str {
     if x.len() > y.len() {
         x
     } else {
         y
     }
}

// result points to a reference to string2, but the scope of this reference is limited to the inner {}, so a dangling reference will occur during println.
fn main1() {
     let string1 = String::from("123456789");
     let result;
     {
         let string2 = String::from("abcdefghijklmnopqrstuvwxyz");
         result = longest(string1.as_str(), string2.as_str());
     }
     println!("The longest string is {}", result);
}

// The values of string1 and string2 are exchanged here. At this time, result1 points to the reference of string1. Since the scope of the value of string1 is greater than the scope of the reference,
// In theory, this code can be compiled successfully, but in practice it will still fail to compile.
fn main2() {
     let string1 = String::from("abcdefghijklmnopqrstuvwxyz");
     let result;
     {
         let string2 = String::from("123456789");
         result = longest(string1.as_str(), string2.as_str());
     }
     println!("The longest string is {}", result);
}

// Summary: Rust's compiler is not as smart as humans. On the contrary, it is more conservative than humans. Therefore, for the above code, there must be a clear mechanism to identify the scope.
// Otherwise, Rust will fail to compile.
