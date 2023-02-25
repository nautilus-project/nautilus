pub fn nautilus_entrypoint_borsh(
    enum_name: &syn::Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let mut x: u8 = 0;
    let mut variants_with_values: Vec<(syn::Ident, u8)> = vec![];
    for v in variants.iter() {
        variants_with_values.push((v.ident.clone(), x));
        x += 1;
    }

    let tokens_ser_variants = variants_with_values.iter().map(|(v, val)| {
        quote::quote! { #enum_name::#v => #val }
    });
    let tokens_ser_match_arms = variants_with_values.iter().map(|(v, _)| {
        quote::quote! { #enum_name::#v => {}, }
    });
    let tokens_deser_variants = variants_with_values.iter().map(|(v, val)| {
        quote::quote! { #val => #enum_name::#v }
    });
    //
    quote::quote! {
        impl borsh::ser::BorshSerialize for #enum_name {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    #(#tokens_ser_variants,)*
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    #(#tokens_ser_match_arms)*
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for #enum_name {
            fn deserialize(
                buf: &mut &[u8],
            ) -> core::result::Result<Self, borsh::maybestd::io::Error> {
                let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
                let return_value = match variant_idx {
                    #(#tokens_deser_variants,)*
                    _ => {
                        let msg = {
                            let res = std::fmt::format(
                                std::format_args!("Unexpected variant index: {0:?}", variant_idx),
                            );
                            res
                        };
                        return Err(
                            borsh::maybestd::io::Error::new(
                                borsh::maybestd::io::ErrorKind::InvalidInput,
                                msg,
                            ),
                        );
                    }
                };
                Ok(return_value)
            }
        }
    }
}
