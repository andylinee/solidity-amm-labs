// Destructing in Rust

// 1. Use Case
// Calculate PDA account and bump seed based on an array of signer keys and program ID
let (pda_account, bump_seed) = Pubkey::find_program_address(&[signer.key.as_ref()], program_id);

// If the second element is not needed, it can be replaced with _
let (pda_account, _) = Pubkey::find_program_address(&[signer.key.as_ref()], program_id);


// 2. Documentation
// First approach
let (a, b, c) = ("Banana", "pineapple", "durian");

// Second approach
let (e, d, _) = ("Banana", "pineapple", "durian");


// 3. Example
struct Ferris {
    e: i32,
    f: String
}

fn main() {
    let (a, b, c, d, e, f);

    // tuple destructure
    (a, b) = (1, 2);
    // Array destructuring, .. means ignoring multiple elements, _ means ignoring the element at the corresponding index position (1)
    [c, .., d, _] = [1, 2, 3, 4, 5];
    // struct destructure
    Ferris { e, f } = Ferris { e:5, f:"rust".to_string() };

    assert_eq!((1, 2, 1, 4, 5, "rust".to_string()), (a, b, c, d, e, f));
}
