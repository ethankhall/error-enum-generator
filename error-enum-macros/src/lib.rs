#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;

mod gen;

#[proc_macro_error]
#[proc_macro_derive(ErrorEnum, attributes(error_enum))]
pub fn generate_error_code(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input as DeriveInput);

    gen::impl_generate_error_code(&ast)
}

#[proc_macro_error]
#[proc_macro_derive(ErrorContainer)]
pub fn generate_error_wrapper(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input as DeriveInput);

    gen::impl_generate_error_wrapper(&ast)
}
