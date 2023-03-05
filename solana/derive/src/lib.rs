extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn nautilus(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let parameter_types = input_fn.sig.inputs.iter().map(|param| {
        quote! { #param }
    });

    let fn_body = &input_fn.block.stmts;

    quote! {
        fn #fn_name(#(#parameter_types, context: String),*) {
            println!("{}", context);
            #(#fn_body)*
        }
    }
    .into()
}
