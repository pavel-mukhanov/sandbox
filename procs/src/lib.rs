extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input, AttributeArgs, };
use quote::quote;
use darling::FromMeta;

#[proc_macro]
pub fn say_hello(_input: TokenStream) -> TokenStream {
    // This macro will accept any input because it ignores it.
    // To enforce correctness in macros which don't take input,
    // you may want to add `assert!(_input.to_string().is_empty());`.
    "println!(\"Hello, world!\")".parse().unwrap()
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct MacroArgs {
    source: String,
}

#[proc_macro_attribute]
pub fn say_hello_attr(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attrs as AttributeArgs);
    let _input: DeriveInput = syn::parse(input).unwrap();

    let _args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            panic!(format!("{:?}", e))
        }
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        struct Hi {

        }
    };

    // Hand the output tokens back to the compiler
    expanded.into()
}