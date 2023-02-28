pub fn nautilus_processor(
    enum_name: &syn::Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let match_arms = variants.iter().map(|v| {
        let variant_name = v.ident.to_string();
        let variant_name_snake = case::CaseExt::to_snake(variant_name.as_str());
        let struct_name = syn::Ident::new(
            &(convert_case::Casing::to_case(
                &variant_name_snake.split("_").last().unwrap(),
                convert_case::Case::Title,
            )),
            proc_macro2::Span::call_site(),
        );
        // let call_function_name =
        //     syn::Ident::new(&(variant_name_snake), proc_macro2::Span::call_site());
        let call_function_name = syn::Ident::new("seeds", proc_macro2::Span::call_site());
        quote::quote! { #enum_name ::#v => #struct_name :: #call_function_name() }
    });
    quote::quote! {
        let instruction = #enum_name::try_from_slice(&input)?;
        match instruction {
            #(#match_arms,)*
        }
    }
}
