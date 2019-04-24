extern crate proc_macro;

// This is always necessary to get the `TokenStream` typedef.

use proc_macro::TokenStream;

#[proc_macro]
pub fn say_hello(_input: TokenStream) -> TokenStream {
    // This macro will accept any input because it ignores it.
    // To enforce correctness in macros which don't take input,
    // you may want to add `assert!(_input.to_string().is_empty());`.
    "println!(\"Hello, world!\")".parse().unwrap()
}
