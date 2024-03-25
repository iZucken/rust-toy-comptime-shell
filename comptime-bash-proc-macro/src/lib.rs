extern crate quote;

use proc_macro::{TokenStream};
use quote::quote;

#[proc_macro]
/// Executes a shell script at compile time and inlines the output
pub fn bash(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    // use: set -euo pipefail
    // find out how to execute shell script from rust
    // panic on non 0 code
    // on success just spew an output string
    let inline = "test, input: ".to_owned() + &input;
    (quote! {
        #inline
    }).into()
}
