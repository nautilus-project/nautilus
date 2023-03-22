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

#[proc_macro_derive(Nautilus, attributes(default_instructions, primary_key, authority))]
pub fn nautilus_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote::ToTokens::to_token_stream(&syn::parse_macro_input!(
        input as nautilus_syn::NautilusObject
    ))
    .into()
}
