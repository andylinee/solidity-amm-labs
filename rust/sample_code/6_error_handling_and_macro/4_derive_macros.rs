// Derive Macros in Rust

// 1. Use Case
// Derive macro
#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    // Fields in the struct
}


// 2. Documentation
struct Foo { x: i32, y: i32 }

// Method one
impl Copy for Foo { ... }
impl Clone for Foo { ... }
impl Ord for Foo { ... }
impl PartialOrd for Foo { ... }
impl Eq for Foo { ... }
impl PartialEq for Foo { ... }
impl Debug for Foo { ... }
impl Hash for Foo { ... }

// Method two
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
struct Foo { x: i32, y: i32 }


// 3. Example
//Definition of HelloMacro macro
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

     let ast:DeriveInput = syn::parse(input).unwrap();

     impl_hello_macro(&ast)

}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    
     let name = &ast.ident;
     let gen = quote! {
				// Characteristics implemented by derived macros
         impl HelloMacro for #name {
             fn hello_macro() {
                 println!("Hello, Macro! My name is {}!", stringify!(#name));
             }
         }
     };
     gen.into()
}


//Use of macros
#[derive(HelloMacro)]
struct MyStruct;

#[derive(HelloMacro)]
struct YourStruct;

fn main() {
     println!("Hello, world!");
     MyStruct::hello_macro();
     YourStruct::hello_macro();
}


// 4. Quest
#[derive(Copy, Clone)]

struct Bar {name: String, score: i32}
