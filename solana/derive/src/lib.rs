extern crate proc_macro;

#[proc_macro_attribute]
pub fn nautilus(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    quote::ToTokens::to_token_stream(&syn::parse_macro_input!(
        input as nautilus_syn::NautilusEntrypoint
    ))
    .into()
}

#[proc_macro_derive(Nautilus, attributes(primary_key, authority))]
pub fn nautilus_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote::ToTokens::to_token_stream(&syn::parse_macro_input!(
        input as nautilus_syn::NautilusObject
    ))
    .into()
}

// #[proc_macro_attribute]
// pub fn nautilus(_: TokenStream, input: TokenStream) -> TokenStream {
//     let item_mod = parse_macro_input!(input as ItemMod);

//     println!("{:#?}", item_mod.content);

//     // collect all the function names defined in the module
//     let mut functions = vec![];
//     for item in &item_mod.content.unwrap().1 {
//         if let syn::Item::Fn(item_fn) = item {
//             println!("FN: {}", item_fn.sig.ident.to_string());
//             functions.push(item_fn.sig.ident.clone());
//         }
//     }

//     let output = quote! {
//         fn print_something_else() {
//             println!("Something else");
//         }
//     };

//     output.into()
// }
