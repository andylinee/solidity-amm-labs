// Lifetimes in Rust

// 1. Use Case
pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,
    pub lamports: Rc<RefCell<&'a mut u64>>,
    pub data: Rc<RefCell<&'a mut [u8]>>,
    pub owner: &'a Pubkey,
    pub rent_epoch: Epoch,
    ……
}


// 2. Documentation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// 3. Example
// After the structure name + angle brackets to mark the life cycle
struct MyEnum<'a> {
         // Attribute fields use the lifetime marked in the enumeration 'a
         // This means that the lifetime of the greet reference must be greater than the enumeration instance, otherwise an invalid reference will occur
     greet: &'a str,
}


fn main() {
     let hello = String::from("hello, hackquest");
         // The life cycle of the reference is greater than the structure instance, so it can be compiled normally
     let i = MyEnum { greet: &hello };
}


// 4. Quest
// Please improve the definition of the following function signature
// fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}
