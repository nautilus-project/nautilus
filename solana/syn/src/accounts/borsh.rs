pub fn nautilus_borsh_self(
    struct_name: &syn::Ident,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let borsh_ser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote::quote! { #field_ty: borsh::ser::BorshSerialize }
    });
    let borsh_ser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote::quote! { borsh::BorshSerialize::serialize(&self.#field_name, writer)? }
    });
    let borsh_deser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote::quote! { #field_ty: borsh::de::BorshDeserialize }
    });
    let borsh_deser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote::quote! { #field_name: borsh::BorshDeserialize::deserialize(buf)? }
    });
    borsh_impl(
        struct_name,
        borsh_ser_where,
        borsh_ser_impl,
        borsh_deser_where,
        borsh_deser_impl,
    )
}

pub fn nautilus_borsh_optionized(
    struct_name: &syn::Ident,
    field_names: &Vec<syn::Ident>,
    field_types: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let borsh_ser_where = field_types.iter().map(|f| {
        quote::quote! { #f: borsh::ser::BorshSerialize }
    });
    let borsh_ser_impl = field_names.iter().map(|f| {
        quote::quote! { borsh::BorshSerialize::serialize(&self.#f, writer)? }
    });
    let borsh_deser_where = field_types.iter().map(|f| {
        quote::quote! { #f: borsh::de::BorshDeserialize }
    });
    let borsh_deser_impl = field_names.iter().map(|f| {
        quote::quote! { #f: borsh::BorshDeserialize::deserialize(buf)? }
    });
    borsh_impl(
        struct_name,
        borsh_ser_where,
        borsh_ser_impl,
        borsh_deser_where,
        borsh_deser_impl,
    )
}

fn borsh_impl(
    struct_name: &syn::Ident,
    borsh_ser_where: impl Iterator<Item = proc_macro2::TokenStream>,
    borsh_ser_impl: impl Iterator<Item = proc_macro2::TokenStream>,
    borsh_deser_where: impl Iterator<Item = proc_macro2::TokenStream>,
    borsh_deser_impl: impl Iterator<Item = proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl borsh::ser::BorshSerialize for #struct_name
        where
            #(#borsh_ser_where,)*
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                #(#borsh_ser_impl;)*
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for #struct_name
        where
            #(#borsh_deser_where,)*
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    #(#borsh_deser_impl,)*
                })
            }
        }
    }
}
